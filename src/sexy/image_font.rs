//! `Sexy::ImageFont` and its `FontData` (framework), replaced.
//!
//! The descriptor interpreter (`FontData::Load`, @ 004586f0, 10 KB) is replaced by a parser
//! of the subset of the font language the game's `data\*.txt` files use (`Define`,
//! `CreateLayer`, `LayerSetImage`, `LayerSetImageMap`, `LayerSetCharWidths`,
//! `LayerSetCharOffsets`, `LayerSetKerningPairs`, `LayerSetAscent`, `LayerSetAscentPadding`,
//! `LayerSetLineSpacingOffset`, `LayerSetPointSize`, `SetDefaultPointSize`). Layout and
//! drawing follow the binary's `ImageFont::CharWidthKern` (@ 0045bfd0), `StringWidth`
//! (@ 0045bf50) and `DrawStringEx` (@ 0045c170) at scale 1.0: glyph position
//! `x + charOffset.x + layerOffset.x`, `y + charOffset.y - ascent + layerOffset.y`, advance
//! `width + spacing + kerning[c][next]`, colour `min(mult*c/255 + add, 255)`, and glyphs
//! drawn grouped by `order + 0x80` (stable).

use crate::sexy::graphics::{FUN_00455890, FUN_004558c0, FUN_004558d0, FUN_004558e0, FUN_004558f0, FUN_004558b0, FUN_00455e40};
use crate::sexy::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct CharData {
    pub image_rect: Rect,
    pub offset: Point,
    /// `mKerningOffsets[next]` (signed byte in the original).
    pub kerning: HashMap<u8, i32>,
    pub width: i32,
    pub order: i32,
}

#[derive(Clone, Debug)]
pub struct FontLayer {
    pub name: String,
    pub image: Ptr,
    pub chars: Vec<CharData>,
    pub ascent: i32,
    pub ascent_padding: i32,
    pub height: i32,
    pub line_spacing_offset: i32,
    pub point_size: i32,
    pub spacing: i32,
    pub offset: Point,
    pub color_mult: Color,
    pub color_add: Color,
    pub draw_mode: i32,
    pub base_order: i32,
}

impl FontLayer {
    fn new(name: &str) -> FontLayer {
        FontLayer {
            name: name.to_string(),
            image: NULL,
            chars: vec![CharData::default(); 256],
            ascent: 0,
            ascent_padding: 0,
            height: 0,
            line_spacing_offset: 0,
            point_size: 0,
            spacing: 0,
            offset: Point::default(),
            color_mult: Color::WHITE,
            color_add: Color { mRed: 0, mGreen: 0, mBlue: 0, mAlpha: 0 },
            draw_mode: -1,
            base_order: 0,
        }
    }
}

/// `Sexy::ImageFont` with its `Font_data` (+0x4: ascent, ascent padding, height, line
/// spacing offset) and `FontData` (layers, char map, default point size).
#[derive(Clone, Debug)]
pub struct ImageFont {
    /// `Font_data.offset_0x0` `mAscent`.
    pub mAscent: i32,
    /// `Font_data.offset_0x4` `mAscentPadding`.
    pub mAscentPadding: i32,
    /// `Font_data.offset_0x8` `mHeight`.
    pub mHeight: i32,
    /// `Font_data.offset_0xc` `mLineSpacingOffset`.
    pub mLineSpacingOffset: i32,
    pub layers: Vec<FontLayer>,
    /// `FontData::mCharMap` (+0x60).
    pub char_map: [u8; 256],
    pub default_point_size: i32,
    /// `ImageFont::mScale` (+0x40), 1.0.
    pub scale: f64,
}

fn tokens(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let b: Vec<char> = text.chars().collect();
    let mut i = 0;
    while i < b.len() {
        let c = b[i];
        if c.is_whitespace() {
            i += 1;
        } else if c == '/' && i + 1 < b.len() && b[i + 1] == '/' {
            while i < b.len() && b[i] != '\n' {
                i += 1;
            }
        } else if c == '\'' || c == '"' {
            let q = c;
            let mut s = String::new();
            i += 1;
            while i < b.len() && b[i] != q {
                if b[i] == '\\' && i + 1 < b.len() {
                    i += 1;
                }
                s.push(b[i]);
                i += 1;
            }
            i += 1;
            out.push(format!("'{s}"));
        } else if "(),;".contains(c) {
            out.push(c.to_string());
            i += 1;
        } else {
            let mut s = String::new();
            while i < b.len() && !b[i].is_whitespace() && !"(),;'\"".contains(b[i]) {
                s.push(b[i]);
                i += 1;
            }
            out.push(s);
        }
    }
    out
}

/// A parsed value: a scalar or a parenthesized list.
#[derive(Clone, Debug)]
enum Val {
    S(String),
    L(Vec<Val>),
}

fn parse_val(t: &[String], i: &mut usize) -> Val {
    if t[*i] == "(" {
        *i += 1;
        let mut items = Vec::new();
        while *i < t.len() && t[*i] != ")" {
            if t[*i] == "," {
                *i += 1;
                continue;
            }
            items.push(parse_val(t, i));
        }
        *i += 1;
        Val::L(items)
    } else {
        let s = t[*i].clone();
        *i += 1;
        Val::S(s)
    }
}

impl Val {
    fn resolve<'a>(&'a self, defs: &'a HashMap<String, Val>) -> &'a Val {
        match self {
            Val::S(s) if defs.contains_key(s) => defs[s].resolve(defs),
            v => v,
        }
    }
    fn list<'a>(&'a self, defs: &'a HashMap<String, Val>) -> Vec<&'a Val> {
        match self.resolve(defs) {
            Val::L(v) => v.iter().map(|x| x.resolve(defs)).collect(),
            v => vec![v],
        }
    }
    fn int(&self, defs: &HashMap<String, Val>) -> i32 {
        match self.resolve(defs) {
            Val::S(s) => s.parse().unwrap_or(0),
            _ => 0,
        }
    }
    fn ch(&self, defs: &HashMap<String, Val>) -> u8 {
        match self.resolve(defs) {
            Val::S(s) => s.strip_prefix('\'').and_then(|c| c.bytes().next()).unwrap_or(0),
            _ => 0,
        }
    }
    fn string(&self, defs: &HashMap<String, Val>) -> String {
        match self.resolve(defs) {
            Val::S(s) => s.trim_start_matches('\'').to_string(),
            _ => String::new(),
        }
    }
}

/// Loads `path` (e.g. `data\JungleFever17outline.txt`) and its layer images; registers the
/// font object and returns it.
pub fn load_image_font(g: &mut G, path: &str) -> Option<Ptr> {
    let text = String::from_utf8_lossy(g.vfs.read(path)?).to_string();
    let dir = match path.rfind(['\\', '/']) {
        Some(i) => &path[..=i],
        None => "",
    };
    let t = tokens(&text);
    let mut defs: HashMap<String, Val> = HashMap::new();
    let mut font = ImageFont {
        mAscent: 0,
        mAscentPadding: 0,
        mHeight: 0,
        mLineSpacingOffset: 0,
        layers: Vec::new(),
        char_map: std::array::from_fn(|i| i as u8),
        default_point_size: 0,
        scale: 1.0,
    };
    let mut i = 0;
    while i < t.len() {
        let cmd = t[i].clone();
        i += 1;
        let mut args = Vec::new();
        while i < t.len() && t[i] != ";" {
            args.push(parse_val(&t, &mut i));
        }
        i += 1;
        let layer = |font: &mut ImageFont, v: &Val, defs: &HashMap<String, Val>| -> Option<usize> {
            let n = v.string(defs);
            font.layers.iter().position(|l| l.name == n)
        };
        match cmd.as_str() {
            "Define" if args.len() == 2 => {
                if let Val::S(n) = &args[0] {
                    defs.insert(n.clone(), args[1].clone());
                }
            }
            "SetDefaultPointSize" => font.default_point_size = args[0].int(&defs),
            "CreateLayer" => font.layers.push(FontLayer::new(&args[0].string(&defs))),
            "LayerSetImage" => {
                let l = layer(&mut font, &args[0], &defs)?;
                let name = format!("{dir}{}", args[1].string(&defs));
                let li = crate::sexy::image_lib::FUN_0049fed0(&g.vfs, &name, true)?;
                let mut img = crate::sexy::image::Image::new();
                img.field_0x4 = name;
                img.offset_0x20 = li.mWidth;
                img.offset_0x24 = li.mHeight;
                img.mBits = li.mBits;
                font.layers[l].image = g.alloc(Obj { vt: None, node: Node::Image(Box::new(img)) });
            }
            "LayerSetImageMap" => {
                let l = layer(&mut font, &args[0], &defs)?;
                let chars = args[1].list(&defs);
                let rects = args[2].list(&defs);
                for (c, r) in chars.iter().zip(rects.iter()) {
                    let r = r.list(&defs);
                    let rect = Rect::new(r[0].int(&defs), r[1].int(&defs), r[2].int(&defs), r[3].int(&defs));
                    let ly = &mut font.layers[l];
                    ly.chars[c.ch(&defs) as usize].image_rect = rect;
                    if rect.mHeight > ly.height {
                        ly.height = rect.mHeight;
                    }
                }
            }
            "LayerSetCharWidths" => {
                let l = layer(&mut font, &args[0], &defs)?;
                for (c, w) in args[1].list(&defs).iter().zip(args[2].list(&defs).iter()) {
                    font.layers[l].chars[c.ch(&defs) as usize].width = w.int(&defs);
                }
            }
            "LayerSetCharOffsets" => {
                let l = layer(&mut font, &args[0], &defs)?;
                for (c, o) in args[1].list(&defs).iter().zip(args[2].list(&defs).iter()) {
                    let o = o.list(&defs);
                    font.layers[l].chars[c.ch(&defs) as usize].offset = Point { mX: o[0].int(&defs), mY: o[1].int(&defs) };
                }
            }
            "LayerSetKerningPairs" => {
                let l = layer(&mut font, &args[0], &defs)?;
                for (p, v) in args[1].list(&defs).iter().zip(args[2].list(&defs).iter()) {
                    let s = p.string(&defs);
                    let b = s.as_bytes();
                    if b.len() == 2 {
                        font.layers[l].chars[b[0] as usize].kerning.insert(b[1], v.int(&defs) as i8 as i32);
                    }
                }
            }
            "LayerSetAscent" => {
                let l = layer(&mut font, &args[0], &defs)?;
                font.layers[l].ascent = args[1].int(&defs);
            }
            "LayerSetAscentPadding" => {
                let l = layer(&mut font, &args[0], &defs)?;
                font.layers[l].ascent_padding = args[1].int(&defs);
            }
            "LayerSetLineSpacingOffset" => {
                let l = layer(&mut font, &args[0], &defs)?;
                font.layers[l].line_spacing_offset = args[1].int(&defs);
            }
            "LayerSetPointSize" => {
                let l = layer(&mut font, &args[0], &defs)?;
                font.layers[l].point_size = args[1].int(&defs);
            }
            _ => {}
        }
    }
    // ImageFont::Prepare: the font's metrics are the maxima over its layers.
    for l in &font.layers {
        font.mAscent = font.mAscent.max(l.ascent);
        font.mAscentPadding = font.mAscentPadding.max(l.ascent_padding);
        font.mHeight = font.mHeight.max(l.height);
        font.mLineSpacingOffset = font.mLineSpacingOffset.max(l.line_spacing_offset);
    }
    Some(g.alloc(Obj { vt: None, node: Node::Font(Box::new(font)) }))
}

/// `Font::Duplicate()` (vtable +0x2c): a new font object with the same data.
pub fn duplicate(g: &mut G, f: Ptr) -> Ptr {
    let copy = g.font(f).clone();
    g.alloc(Obj { vt: None, node: Node::Font(Box::new(copy)) })
}

impl G {
    pub fn font(&mut self, p: Ptr) -> &mut ImageFont {
        match &mut self.obj(p).node {
            Node::Font(f) => f,
            n => panic!("{p} is not a font: {n:?}"),
        }
    }
}

/// `Font::GetAscent()` (vtable +0x4).
pub fn get_ascent(g: &mut G, f: Ptr) -> i32 {
    g.font(f).mAscent
}
/// `Font::GetAscentPadding()` (vtable +0x8).
pub fn get_ascent_padding(g: &mut G, f: Ptr) -> i32 {
    g.font(f).mAscentPadding
}
/// `Font::GetDescent()` (vtable +0xc).
pub fn get_descent(g: &mut G, f: Ptr) -> i32 {
    let ft = g.font(f);
    ft.mHeight - ft.mAscent
}
/// `Font::GetHeight()` (vtable +0x10).
pub fn get_height(g: &mut G, f: Ptr) -> i32 {
    g.font(f).mHeight
}
/// `Font::GetLineSpacingOffset()` (vtable +0x14).
pub fn get_line_spacing_offset(g: &mut G, f: Ptr) -> i32 {
    g.font(f).mLineSpacingOffset
}
/// `Font::GetLineSpacing()` (vtable +0x18).
pub fn get_line_spacing(g: &mut G, f: Ptr) -> i32 {
    let ft = g.font(f);
    ft.mHeight + ft.mLineSpacingOffset
}

/// `ImageFont::CharWidthKern(char, char prev)` (vtable +0x24) at scale 1.
pub fn char_width_kern(g: &mut G, f: Ptr, c: u8, prev: u8) -> i32 {
    let ft = g.font(f);
    let c = ft.char_map[c as usize];
    let prev = if prev != 0 { ft.char_map[prev as usize] } else { 0 };
    let mut max = 0;
    for l in &ft.layers {
        let w = crate::sexy::crt::ftol(l.chars[c as usize].width as f64 * ft.scale) as i32;
        let k = if prev != 0 {
            crate::sexy::crt::ftol((l.chars[prev as usize].kerning.get(&c).copied().unwrap_or(0) + l.spacing) as f64 * ft.scale) as i32
        } else {
            0
        };
        max = max.max(w + k);
    }
    max
}

/// `ImageFont::StringWidth(const std::string&)` (vtable +0x1c).
pub fn string_width(g: &mut G, f: Ptr, s: &[u8]) -> i32 {
    let mut w = 0;
    let mut prev = 0u8;
    for &c in s {
        w += char_width_kern(g, f, c, prev);
        prev = c;
    }
    w
}

/// `ImageFont::DrawString(Graphics*, int x, int y, const std::string&, const Color&, const Rect& clip)`
/// (vtable +0x28) = `DrawStringEx(..., NULL, NULL)`.
pub fn draw_string(g: &mut G, f: Ptr, gfx: &mut Graphics, x: i32, y: i32, s: &[u8], color: Color, _clip: Rect) {
    struct Cmd {
        image: Ptr,
        x: i32,
        y: i32,
        src: Rect,
        color: Color,
        mode: i32,
    }
    let mut buckets: Vec<Vec<Cmd>> = (0..256).map(|_| Vec::new()).collect();
    {
        let ft = g.font(f).clone();
        let mut cur_x = x;
        for (n, &raw) in s.iter().enumerate() {
            let c = ft.char_map[raw as usize] as usize;
            let next = if n + 1 < s.len() { ft.char_map[s[n + 1] as usize] } else { 0 };
            let mut max_x = cur_x;
            for l in &ft.layers {
                let cd = &l.chars[c];
                let ix = cd.offset.mX + l.offset.mX + cur_x;
                let iy = (cd.offset.mY - l.ascent) + l.offset.mY + y;
                let spacing = if next != 0 { cd.kerning.get(&next).copied().unwrap_or(0) + l.spacing } else { 0 };
                let ch = |m: i32, v: i32, a: i32| ((m * v) / 0xff + a).min(0xff);
                let col = Color {
                    mRed: ch(l.color_mult.mRed, color.mRed, l.color_add.mRed),
                    mGreen: ch(l.color_mult.mGreen, color.mGreen, l.color_add.mGreen),
                    mBlue: ch(l.color_mult.mBlue, color.mBlue, l.color_add.mBlue),
                    mAlpha: ch(l.color_mult.mAlpha, color.mAlpha, l.color_add.mAlpha),
                };
                let order = (cd.order + l.base_order + 0x80).clamp(0, 0xff) as usize;
                buckets[order].push(Cmd { image: l.image, x: ix, y: iy, src: cd.image_rect, color: col, mode: l.draw_mode });
                let lx = cur_x + cd.width + spacing;
                max_x = max_x.max(lx);
            }
            cur_x = max_x;
        }
    }
    let old_colorize = FUN_004558f0(gfx);
    FUN_004558e0(gfx, true);
    let old_color = FUN_004558b0(gfx);
    let old_mode = FUN_004558d0(gfx);
    for cmd in buckets.into_iter().flatten() {
        if cmd.image == NULL {
            continue;
        }
        FUN_00455890(gfx, cmd.color);
        if cmd.mode != -1 {
            FUN_004558c0(gfx, cmd.mode);
        }
        FUN_00455e40(gfx, g, cmd.image, cmd.x, cmd.y, &cmd.src);
        FUN_004558c0(gfx, old_mode);
    }
    FUN_00455890(gfx, old_color);
    FUN_004558e0(gfx, old_colorize);
}
