//! `Sexy::ResourceManager` (framework, 0x00415190..0x0041a260), replaced.
//!
//! Reads `properties\resources.xml` (groups of `<Image>`, `<Sound>`, `<Font>` with
//! `<SetDefaults path idprefix>`), loads a group's resources on request, and hands out the
//! loaded objects by id. It follows the framework's rules for composing images:
//! `ImageLib::GetImage(path, true)` for the colors, then `alphaimage` (whole-image alpha) or
//! `alphagrid` (one cel's alpha copied into every cel) from the low byte of that image,
//! `rows`/`cols` for the cel grid, and the animation attributes.
//!
//! The game reaches it through `mResourceManager` vtable calls; those call sites name the
//! method they reach here (e.g. vtable +0x3c = `LoadResources(group)`).

use crate::sexy::image::Image;
use crate::sexy::image_lib::FUN_0049fed0;
use crate::sexy::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct ImageRes {
    pub id: String,
    pub path: String,
    pub alpha_image: String,
    pub alpha_grid: String,
    pub rows: i32,
    pub cols: i32,
    /// `mAnimInfo`, computed at parse time; `None` for `anim="none"` or no `anim`.
    pub anim: Option<crate::sexy::image::AnimInfo>,
    /// Loaded image (null until loaded).
    pub image: Ptr,
}

#[derive(Clone, Debug, Default)]
pub struct SoundRes {
    pub id: String,
    pub path: String,
    pub volume: f64,
    /// Sound id given out at load time (-1 until loaded).
    pub sound_id: i32,
}

#[derive(Clone, Debug, Default)]
pub struct FontRes {
    pub id: String,
    pub path: String,
    /// Loaded font object (null until loaded).
    pub font: Ptr,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Image,
    Sound,
    Font,
}

#[derive(Debug, Default)]
pub struct ResourceManager {
    groups: HashMap<String, Vec<(Kind, String)>>,
    pub images: HashMap<String, ImageRes>,
    pub sounds: HashMap<String, SoundRes>,
    pub fonts: HashMap<String, FontRes>,
    /// `mCurResGroupList` / `mCurResGroupListItr`.
    cur: Vec<(Kind, String)>,
    cur_index: usize,
    /// `mHadError` / `mError`.
    pub error: Option<String>,
    next_sound_id: i32,
    /// Sound id -> file path, for the host's audio playback.
    pub sound_files: HashMap<i32, (String, f64)>,
}

/// One `<Tag a=b ...>` of the manifest.
struct Tag {
    name: String,
    attrs: Vec<(String, String)>,
    closing: bool,
}

fn parse_tags(text: &str) -> Vec<Tag> {
    let b = text.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        if b[i] != b'<' {
            i += 1;
            continue;
        }
        if text[i..].starts_with("<!--") {
            i = text[i..].find("-->").map_or(b.len(), |e| i + e + 3);
            continue;
        }
        if text[i..].starts_with("<?") {
            i = text[i..].find("?>").map_or(b.len(), |e| i + e + 2);
            continue;
        }
        let end = text[i..].find('>').map_or(b.len(), |e| i + e);
        let inner = text[i + 1..end].trim().trim_end_matches('/').trim();
        let closing = inner.starts_with('/');
        let inner = inner.trim_start_matches('/');
        let mut chars = inner.char_indices().peekable();
        let mut name = String::new();
        while let Some(&(_, c)) = chars.peek() {
            if c.is_whitespace() {
                break;
            }
            name.push(c);
            chars.next();
        }
        let rest: String = chars.map(|(_, c)| c).collect();
        let mut attrs = Vec::new();
        let r = rest.as_bytes();
        let mut j = 0;
        while j < r.len() {
            while j < r.len() && (r[j] as char).is_whitespace() {
                j += 1;
            }
            let ks = j;
            while j < r.len() && r[j] != b'=' && !(r[j] as char).is_whitespace() {
                j += 1;
            }
            let key = rest[ks..j].to_string();
            while j < r.len() && (r[j] as char).is_whitespace() {
                j += 1;
            }
            let mut val = String::new();
            if j < r.len() && r[j] == b'=' {
                j += 1;
                while j < r.len() && (r[j] as char).is_whitespace() {
                    j += 1;
                }
                if j < r.len() && r[j] == b'"' {
                    let vs = j + 1;
                    j = vs + rest[vs..].find('"').unwrap_or(rest.len() - vs);
                    val = rest[vs..j].to_string();
                    j += 1;
                } else {
                    let vs = j;
                    while j < r.len() && !(r[j] as char).is_whitespace() {
                        j += 1;
                    }
                    val = rest[vs..j].to_string();
                }
            }
            if !key.is_empty() {
                attrs.push((key, val));
            }
        }
        out.push(Tag { name, attrs, closing });
        i = end + 1;
    }
    out
}

impl Tag {
    fn get(&self, k: &str) -> Option<&str> {
        self.attrs.iter().find(|(a, _)| a.eq_ignore_ascii_case(k)).map(|(_, v)| v.as_str())
    }
}

/// CRT `atol`: leading whitespace, optional sign, digits; 0 when there are none.
fn atol(s: &str) -> i32 {
    let t = s.trim_start();
    let (neg, digits) = match t.as_bytes().first() {
        Some(b'-') => (true, &t[1..]),
        Some(b'+') => (false, &t[1..]),
        _ => (false, t),
    };
    let mut v: i32 = 0;
    for b in digits.bytes().take_while(|b| b.is_ascii_digit()) {
        v = v.wrapping_mul(10).wrapping_add((b - b'0') as i32);
    }
    if neg { v.wrapping_neg() } else { v }
}

fn ints(s: &str) -> Vec<i32> {
    s.split(|c: char| c == ',' || c.is_whitespace()).filter_map(|t| t.trim().parse().ok()).collect()
}

impl ResourceManager {
    /// `ParseResourcesFile(const std::string&)`: false (with `error` set) on failure.
    pub fn parse_resources_file(&mut self, g_vfs: &crate::sexy::vfs::Vfs, path: &str) -> bool {
        let Some(bytes) = g_vfs.read(path) else {
            self.error = Some(format!("Resource file not found: {path}"));
            return false;
        };
        let text = String::from_utf8_lossy(bytes).to_string();
        let mut group: Option<String> = None;
        let (mut def_path, mut def_prefix) = (String::new(), String::new());
        for t in parse_tags(&text) {
            match (t.name.as_str(), t.closing) {
                ("Resources", false) => {
                    let id = t.get("id").unwrap_or("").to_string();
                    self.groups.entry(id.clone()).or_default();
                    group = Some(id);
                    def_path.clear();
                    def_prefix.clear();
                }
                ("Resources", true) => group = None,
                ("SetDefaults", _) => {
                    if let Some(p) = t.get("path") {
                        def_path = p.to_string();
                    }
                    if let Some(p) = t.get("idprefix") {
                        def_prefix = p.to_string();
                    }
                }
                ("Image" | "Sound" | "Font", false) => {
                    let Some(gid) = group.clone() else { continue };
                    let id = format!("{def_prefix}{}", t.get("id").unwrap_or(""));
                    let rel = t.get("path").unwrap_or("");
                    let path = if def_path.is_empty() { rel.to_string() } else { format!("{def_path}\\{rel}") };
                    let kind = match t.name.as_str() {
                        "Image" => {
                            let dir = |a: Option<&str>| a.map(|a| if def_path.is_empty() { a.to_string() } else { format!("{def_path}\\{a}") }).unwrap_or_default();
                            // ParseImageResource: anim type by name, then framedelay, begindelay,
                            // enddelay, perframedelay, framemap, then Compute(max(rows, cols)).
                            let rows: i32 = t.get("rows").and_then(|v| v.parse().ok()).unwrap_or(1);
                            let cols: i32 = t.get("cols").and_then(|v| v.parse().ok()).unwrap_or(1);
                            let anim_type = match t.get("anim").map(|a| a.to_ascii_lowercase()) {
                                None => 0,
                                Some(a) => match a.as_str() {
                                    "none" => 0,
                                    "once" => 1,
                                    "loop" => 3,
                                    "pingpong" => 2,
                                    _ => {
                                        self.error = Some("Invalid animation type.".to_string());
                                        return false;
                                    }
                                },
                            };
                            let anim = if anim_type != 0 {
                                let mut a = crate::sexy::image::FUN_00457530();
                                a.offset_0x0 = anim_type;
                                if let Some(v) = t.get("framedelay") {
                                    a.offset_0x4 = atol(v);
                                }
                                let begin = t.get("begindelay").map(atol).unwrap_or(0);
                                let end = t.get("enddelay").map(atol).unwrap_or(0);
                                if let Some(v) = t.get("perframedelay") {
                                    a.offset_0x10 = ints(v);
                                }
                                if let Some(v) = t.get("framemap") {
                                    a.offset_0x20 = ints(v);
                                }
                                crate::sexy::image::FUN_004575e0(&mut a, rows.max(cols), begin, end);
                                Some(a)
                            } else {
                                None
                            };
                            self.images.insert(
                                id.clone(),
                                ImageRes {
                                    id: id.clone(),
                                    path,
                                    alpha_image: dir(t.get("alphaimage")),
                                    alpha_grid: dir(t.get("alphagrid")),
                                    rows,
                                    cols,
                                    anim,
                                    image: NULL,
                                },
                            );
                            Kind::Image
                        }
                        "Sound" => {
                            let volume = t.get("volume").and_then(|v| v.parse().ok()).unwrap_or(-1.0);
                            self.sounds.insert(id.clone(), SoundRes { id: id.clone(), path, volume, sound_id: -1 });
                            Kind::Sound
                        }
                        _ => {
                            self.fonts.insert(id.clone(), FontRes { id: id.clone(), path, font: NULL });
                            Kind::Font
                        }
                    };
                    self.groups.get_mut(&gid).unwrap().push((kind, id.clone()));
                    if let Some(alias) = t.get("alias") {
                        let alias = format!("{def_prefix}{alias}");
                        if kind == Kind::Image {
                            let r = self.images[&id].clone();
                            self.images.insert(alias, r);
                        }
                    }
                }
                _ => {}
            }
        }
        true
    }

    /// `StartLoadResources(group)`.
    pub fn start_load_resources(&mut self, group: &str) {
        self.cur = self.groups.get(group).cloned().unwrap_or_default();
        self.cur_index = 0;
    }

    /// `GetNumResources(group)`.
    pub fn get_num_resources(&self, group: &str) -> i32 {
        self.groups.get(group).map_or(0, |v| v.len() as i32)
    }

    pub fn has_group(&self, group: &str) -> bool {
        self.groups.contains_key(group)
    }
}

/// Loads the image resource `id` into a new `Image` object (`DoLoadImage`).
fn do_load_image(g: &mut G, id: &str) -> bool {
    let res = g.resource_manager.images[id].clone();
    let Some(mut li) = FUN_0049fed0(&g.vfs, &res.path, true) else {
        g.resource_manager.error = Some(format!("Failed to load image: {}", res.path));
        return false;
    };
    if !res.alpha_image.is_empty() {
        match FUN_0049fed0(&g.vfs, &res.alpha_image, true) {
            Some(a) if a.mWidth == li.mWidth && a.mHeight == li.mHeight => {
                for (p, q) in li.mBits.iter_mut().zip(a.mBits.iter()) {
                    *p = (*p & 0x00ff_ffff) | ((q & 0xff) << 24);
                }
            }
            Some(_) => {
                g.resource_manager.error = Some(format!("AlphaImage size mismatch between {} and {}", res.path, res.alpha_image));
                return false;
            }
            None => {
                g.resource_manager.error = Some(format!("Failed to load image: {}", res.alpha_image));
                return false;
            }
        }
    }
    if !res.alpha_grid.is_empty() {
        let (cw, ch) = (li.mWidth / res.cols, li.mHeight / res.rows);
        match FUN_0049fed0(&g.vfs, &res.alpha_grid, true) {
            Some(a) if a.mWidth == cw && a.mHeight == ch => {
                for r in 0..res.rows {
                    for c in 0..res.cols {
                        for y in 0..ch {
                            for x in 0..cw {
                                let di = ((r * ch + y) * li.mWidth + c * cw + x) as usize;
                                let si = (y * cw + x) as usize;
                                li.mBits[di] = (li.mBits[di] & 0x00ff_ffff) | ((a.mBits[si] & 0xff) << 24);
                            }
                        }
                    }
                }
            }
            Some(_) => {
                g.resource_manager.error = Some(format!("GridAlphaImage size mismatch between {} and {}", res.path, res.alpha_grid));
                return false;
            }
            None => {
                g.resource_manager.error = Some(format!("Failed to load image: {}", res.alpha_grid));
                return false;
            }
        }
    }
    let mut img = Image::new();
    img.field_0x4 = res.path.clone();
    img.offset_0x20 = li.mWidth;
    img.offset_0x24 = li.mHeight;
    img.offset_0x28 = res.rows;
    img.offset_0x2c = res.cols;
    img.mBits = li.mBits;
    // DoLoadImage: `if (mAnimInfo.mAnimType != AnimType_None) aImage->mAnimInfo = new AnimInfo(mAnimInfo)`.
    img.offset_0x30 = res.anim.clone().map(Box::new);
    let p = g.alloc(Obj { vt: None, node: Node::Image(Box::new(img)) });
    g.resource_manager.images.get_mut(id).unwrap().image = p;
    true
}

/// `DoLoadSound`: gives the sound the next free id and records its file for the host.
fn do_load_sound(g: &mut G, id: &str) -> bool {
    let rm = &mut g.resource_manager;
    let sid = rm.next_sound_id;
    rm.next_sound_id += 1;
    let res = rm.sounds.get_mut(id).unwrap();
    res.sound_id = sid;
    let entry = (res.path.clone(), res.volume);
    rm.sound_files.insert(sid, entry);
    true
}

/// `DoLoadFont`: builds the `ImageFont` from its descriptor.
fn do_load_font(g: &mut G, id: &str) -> bool {
    let path = g.resource_manager.fonts[id].path.clone();
    match crate::sexy::image_font::load_image_font(g, &path) {
        Some(f) => {
            g.resource_manager.fonts.get_mut(id).unwrap().font = f;
            true
        }
        None => {
            g.resource_manager.error = Some(format!("Failed to load font: {path}"));
            false
        }
    }
}

/// `LoadNextResource()`: loads one resource of the current group; false when done or on error.
pub fn load_next_resource(g: &mut G) -> bool {
    if g.resource_manager.error.is_some() {
        return false;
    }
    while g.resource_manager.cur_index < g.resource_manager.cur.len() {
        let (kind, id) = g.resource_manager.cur[g.resource_manager.cur_index].clone();
        g.resource_manager.cur_index += 1;
        let loaded = match kind {
            Kind::Image => g.resource_manager.images[&id].image != NULL,
            Kind::Sound => g.resource_manager.sounds[&id].sound_id != -1,
            Kind::Font => g.resource_manager.fonts[&id].font != NULL,
        };
        if loaded {
            continue;
        }
        return match kind {
            Kind::Image => do_load_image(g, &id),
            Kind::Sound => do_load_sound(g, &id),
            Kind::Font => do_load_font(g, &id),
        };
    }
    false
}

/// `LoadResources(group)` (vtable +0x3c): loads the whole group; false on error.
pub fn load_resources(g: &mut G, group: &str) -> bool {
    g.resource_manager.start_load_resources(group);
    while load_next_resource(g) {}
    g.resource_manager.error.is_none()
}

/// `GetImage(id)` (vtable +0x40 path via `SharedImageRef`): null if not loaded.
pub fn get_image(g: &G, id: &str) -> Ptr {
    g.resource_manager.images.get(id).map_or(NULL, |r| r.image)
}

/// `GetSoundThrow(id)` (vtable +0x44).
pub fn get_sound(g: &G, id: &str) -> i32 {
    g.resource_manager.sounds.get(id).map_or(-1, |r| r.sound_id)
}

/// `GetFontThrow(id)` (vtable +0x48).
pub fn get_font(g: &G, id: &str) -> Ptr {
    g.resource_manager.fonts.get(id).map_or(NULL, |r| r.font)
}

/// `LoadImage(id)` outside a group load (used by `LoadImageById`).
pub fn load_image_now(g: &mut G, id: &str) -> Ptr {
    if g.resource_manager.images.get(id).is_some_and(|r| r.image == NULL) {
        do_load_image(g, id);
    }
    get_image(g, id)
}

/// `DeleteImage(id)`: forgets the loaded image (the object itself is freed when unused).
pub fn delete_image(g: &mut G, id: &str) {
    if let Some(r) = g.resource_manager.images.get_mut(id) {
        r.image = NULL;
    }
}
