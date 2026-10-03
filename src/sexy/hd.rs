//! HD screen (a port addition with no counterpart in the original).
//!
//! When the game folder holds upscaled art (`hd\images\<name>.png`, exactly `K` times the
//! size of `images\<name>`, made from the player's own files by `tools/upscale_art.py`),
//! the frame's recorded blitter calls are also painted onto a screen `K` times larger:
//! images with HD art are copied from it pixel for pixel, the others are enlarged with
//! bilinear filtering. The 640x480 screen is still painted exactly as before and stays
//! what the game and the test hooks see; the HD screen only replaces what is displayed.
//!
//! For now the HD screen is used while the main menu (the game selector) is up; elsewhere
//! nothing changes. It assumes an image with HD art is not redrawn at runtime (true of the
//! loaded resources the main menu uses).

use crate::sexy::blit::{additive_px, has_alpha, normal_px};
use crate::sexy::graphics::ImageCmd;
use crate::sexy::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

/// The HD scale.
pub const K: i32 = 4;

/// Upscaled art for one image.
pub struct HdImage {
    pub width: i32,
    pub height: i32,
    pub bits: Vec<u32>,
}

#[derive(Default)]
pub struct HdScreen {
    /// The HD screen is being painted and displayed.
    pub active: bool,
    pub width: i32,
    pub height: i32,
    pub bits: Vec<u32>,
    /// The game folder has HD art (checked once).
    available: Option<bool>,
    /// HD art by image (keyed with its file path, since heap slots are reused).
    cache: HashMap<Ptr, (String, Option<Arc<HdImage>>)>,
}

impl std::fmt::Debug for HdScreen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HdScreen").field("active", &self.active).field("width", &self.width).field("height", &self.height).finish()
    }
}

impl Clone for HdScreen {
    fn clone(&self) -> Self {
        HdScreen::default()
    }
}

fn image_of(g: &G, p: Ptr) -> Option<&crate::sexy::image::Image> {
    match &g.obj_ref(p).node {
        Node::Image(i) => Some(i),
        _ => None,
    }
}

impl HdScreen {
    pub fn available(&mut self, g: &G) -> bool {
        *self.available.get_or_insert_with(|| g.vfs.has_prefix("hd/images/"))
    }

    fn art(&mut self, g: &G, p: Ptr) -> Option<Arc<HdImage>> {
        let img = image_of(g, p)?;
        let path = &img.field_0x4;
        if let Some((cached_path, art)) = self.cache.get(&p)
            && cached_path == path
        {
            return art.clone();
        }
        let art = if path.is_empty() {
            None
        } else {
            g.vfs
                .read(&format!("hd/{path}.png"))
                .and_then(|bytes| image::load_from_memory_with_format(bytes, image::ImageFormat::Png).ok())
                .map(|i| i.to_rgba8())
                .filter(|i| i.width() as i32 == img.offset_0x20 * K && i.height() as i32 == img.offset_0x24 * K)
                .map(|i| {
                    let (w, h) = (i.width() as i32, i.height() as i32);
                    let bits = i.pixels().map(|p| ((p[3] as u32) << 24) | ((p[0] as u32) << 16) | ((p[1] as u32) << 8) | p[2] as u32).collect();
                    Arc::new(HdImage { width: w, height: h, bits })
                })
        };
        self.cache.insert(p, (path.clone(), art.clone()));
        art
    }

    /// Starts painting the HD screen from the current 640x480 one (enlarged), so the parts
    /// not redrawn yet are already in place.
    pub fn activate(&mut self, screen: &[u32], w: i32, h: i32) {
        self.width = w * K;
        self.height = h * K;
        self.bits = vec![0xff00_0000; (self.width * self.height) as usize];
        for y in 0..self.height {
            for x in 0..self.width {
                self.bits[(y * self.width + x) as usize] = screen[((y / K) * w + x / K) as usize] | 0xff00_0000;
            }
        }
        self.active = true;
    }

    /// Paints a frame's recorded blitter calls (in 640x480 coordinates) onto the HD screen.
    /// The screen is split into bands painted in parallel; each band applies every call in
    /// order, so the result is the same as painting them one after another.
    pub fn paint(&mut self, g: &G, cmds: &[ImageCmd]) {
        let mut sources: HashMap<Ptr, Source> = HashMap::new();
        for c in cmds {
            let (image, additive) = match *c {
                ImageCmd::Blt { image, draw_mode, .. }
                | ImageCmd::BltMirror { image, draw_mode, .. }
                | ImageCmd::BltF { image, draw_mode, .. }
                | ImageCmd::BltStretched { image, draw_mode, .. }
                | ImageCmd::BltStretchedMirror { image, draw_mode, .. } => (image, draw_mode == 1),
                _ => continue,
            };
            if !sources.contains_key(&image) {
                let art = self.art(g, image);
                sources.insert(image, Source { art, has_alpha: None });
            }
            if additive {
                let src = sources.get_mut(&image).unwrap();
                if src.has_alpha.is_none() {
                    src.has_alpha = Some(image_of(g, image).is_some_and(has_alpha));
                }
            }
        }
        let (w, h) = (self.width, self.height);
        let threads = std::thread::available_parallelism().map_or(4, |n| n.get()).clamp(1, 16) as i32;
        let rows = (h + threads - 1) / threads;
        let sources = &sources;
        std::thread::scope(|scope| {
            for (i, band) in self.bits.chunks_mut((rows * w) as usize).enumerate() {
                let y0 = i as i32 * rows;
                scope.spawn(move || {
                    let mut p = Painter { bits: band, width: w, y0, y1: (y0 + rows).min(h) };
                    for c in cmds {
                        p.apply(g, sources, c);
                    }
                });
            }
        });
    }
}

/// An image's HD art (if any) and, for additive draws, `mHasAlpha`.
struct Source {
    art: Option<Arc<HdImage>>,
    has_alpha: Option<bool>,
}

/// Paints into rows `y0..y1` of the HD screen (`bits` holds just those rows).
struct Painter<'a> {
    bits: &'a mut [u32],
    width: i32,
    y0: i32,
    y1: i32,
}

impl Painter<'_> {
    fn bounds(&self) -> Rect {
        Rect::new(0, self.y0, self.width, self.y1 - self.y0)
    }

    #[inline]
    fn px(&mut self, x: i32, y: i32) -> &mut u32 {
        &mut self.bits[((y - self.y0) * self.width + x) as usize]
    }

    fn apply(&mut self, g: &G, sources: &HashMap<Ptr, Source>, cmd: &ImageCmd) {
        let k = K as f32;
        match *cmd {
            ImageCmd::Blt { image, x, y, src, color, draw_mode } => {
                let d = Rect::new(x * K, y * K, src.mWidth * K, src.mHeight * K);
                self.quad(g, sources, image, d, src, d, color, draw_mode, false);
            }
            ImageCmd::BltMirror { image, x, y, src, color, draw_mode } => {
                let d = Rect::new(x * K, y * K, src.mWidth * K, src.mHeight * K);
                self.quad(g, sources, image, d, src, d, color, draw_mode, true);
            }
            ImageCmd::BltF { image, x, y, src, clip, color, draw_mode } => {
                let d = Rect::new((x * k).round() as i32, (y * k).round() as i32, src.mWidth * K, src.mHeight * K);
                self.quad(g, sources, image, d, src, scale(clip), color, draw_mode, false);
            }
            ImageCmd::BltStretched { image, dest, src, clip, color, draw_mode, .. } => {
                self.quad(g, sources, image, scale(dest), src, scale(clip), color, draw_mode, false);
            }
            ImageCmd::BltStretchedMirror { image, dest, src, clip, color, draw_mode, .. } => {
                self.quad(g, sources, image, scale(dest), src, scale(clip), color, draw_mode, true);
            }
            ImageCmd::FillRect { rect, color, draw_mode } => {
                let r = crate::sexy::graphics::FUN_00468430(&scale(rect), &self.bounds());
                let s = ((color.mAlpha as u32) << 24) | ((color.mRed as u32) << 16) | ((color.mGreen as u32) << 8) | color.mBlue as u32;
                for y in r.mY..r.mY + r.mHeight {
                    for x in r.mX..r.mX + r.mWidth {
                        let d = self.px(x, y);
                        *d = if draw_mode == 1 { additive_px(*d, 0xffff_ffff, color, false) } else { normal_px(*d, s, Color::WHITE) };
                    }
                }
            }
            ImageCmd::DrawRect { rect: r, color, draw_mode } => {
                for (x, y, w, h) in [
                    (r.mX, r.mY, r.mWidth + 1, 1),
                    (r.mX, r.mY + r.mHeight, r.mWidth + 1, 1),
                    (r.mX, r.mY + 1, 1, r.mHeight - 1),
                    (r.mX + r.mWidth, r.mY + 1, 1, r.mHeight - 1),
                ] {
                    self.apply(g, sources, &ImageCmd::FillRect { rect: Rect::new(x, y, w, h), color, draw_mode });
                }
            }
            ImageCmd::DrawLine { x1, y1, x2, y2, color, draw_mode } => {
                let steps = ((x2 - x1).abs().max((y2 - y1).abs())).ceil().max(1.0) as i32;
                for i in 0..=steps {
                    let t = i as f64 / steps as f64;
                    let x = (x1 + (x2 - x1) * t) as i32;
                    let y = (y1 + (y2 - y1) * t) as i32;
                    self.apply(g, sources, &ImageCmd::FillRect { rect: Rect::new(x, y, 1, 1), color, draw_mode });
                }
            }
        }
    }

    /// Draws `src` (640x480-scale source coordinates) into the HD rect `dest`, clipped.
    #[allow(clippy::too_many_arguments)]
    fn quad(&mut self, g: &G, sources: &HashMap<Ptr, Source>, image: Ptr, dest: Rect, src: Rect, clip: Rect, color: Color, mode: i32, mirror: bool) {
        let Some(img) = image_of(g, image) else { return };
        if dest.mWidth <= 0 || dest.mHeight <= 0 || src.mWidth <= 0 || src.mHeight <= 0 {
            return;
        }
        let area = crate::sexy::graphics::FUN_00468430(&crate::sexy::graphics::FUN_00468430(&dest, &clip), &self.bounds());
        if area.mWidth <= 0 || area.mHeight <= 0 {
            return;
        }
        // The source rect must lie inside the image (cels never read their neighbours).
        let s = crate::sexy::graphics::FUN_00468430(&src, &Rect::new(0, 0, img.offset_0x20, img.offset_0x24));
        if s.mWidth <= 0 || s.mHeight <= 0 {
            return;
        }
        let source = &sources[&image];
        let ha = source.has_alpha.unwrap_or(false);
        let blend = |d: u32, p: u32| if mode == 1 { additive_px(d, p, color, ha) } else { normal_px(d, p, color) };
        // Unstretched with HD art: HD pixels map one to one.
        if let Some(a) = &source.art
            && dest.mWidth == src.mWidth * K
            && dest.mHeight == src.mHeight * K
        {
            let (ax0, ax1) = (s.mX * K, (s.mX + s.mWidth) * K - 1);
            let (ay0, ay1) = (s.mY * K, (s.mY + s.mHeight) * K - 1);
            for y in area.mY..area.mY + area.mHeight {
                let ay = (src.mY * K + y - dest.mY).clamp(ay0, ay1);
                let row = (ay * a.width) as usize;
                for x in area.mX..area.mX + area.mWidth {
                    let off = x - dest.mX;
                    let ax = if mirror { (src.mX + src.mWidth) * K - 1 - off } else { src.mX * K + off }.clamp(ax0, ax1);
                    let p = a.bits[row + ax as usize];
                    let d = self.px(x, y);
                    *d = blend(*d, p);
                }
            }
            return;
        }
        let (sx_per, sy_per) = (src.mWidth as f32 / dest.mWidth as f32, src.mHeight as f32 / dest.mHeight as f32);
        for y in area.mY..area.mY + area.mHeight {
            let v = src.mY as f32 + (y - dest.mY) as f32 * sy_per + 0.5 * sy_per;
            for x in area.mX..area.mX + area.mWidth {
                let mut u = (x - dest.mX) as f32 * sx_per + 0.5 * sx_per;
                u = if mirror { src.mX as f32 + src.mWidth as f32 - u } else { src.mX as f32 + u };
                let p = match &source.art {
                    Some(a) => {
                        let ax = ((u * K as f32) as i32).clamp(s.mX * K, (s.mX + s.mWidth) * K - 1);
                        let ay = ((v * K as f32) as i32).clamp(s.mY * K, (s.mY + s.mHeight) * K - 1);
                        a.bits[(ay * a.width + ax) as usize]
                    }
                    None => bilinear(&img.mBits, img.offset_0x20, img.offset_0x24, s, u - 0.5, v - 0.5),
                };
                let d = self.px(x, y);
                *d = blend(*d, p);
            }
        }
    }
}

fn scale(r: Rect) -> Rect {
    Rect::new(r.mX * K, r.mY * K, r.mWidth * K, r.mHeight * K)
}

/// Bilinear sample at (`u`, `v`) (pixel centres at integers), clamped to `rect`; colours are
/// weighted by alpha so transparent pixels leave no dark fringe.
fn bilinear(bits: &[u32], w: i32, _h: i32, rect: Rect, u: f32, v: f32) -> u32 {
    let u = u.clamp(rect.mX as f32, (rect.mX + rect.mWidth - 1) as f32);
    let v = v.clamp(rect.mY as f32, (rect.mY + rect.mHeight - 1) as f32);
    let (x0, y0) = (u.floor() as i32, v.floor() as i32);
    let x1 = (x0 + 1).min(rect.mX + rect.mWidth - 1);
    let y1 = (y0 + 1).min(rect.mY + rect.mHeight - 1);
    let (fx, fy) = (u - x0 as f32, v - y0 as f32);
    let mut acc = [0f32; 4];
    for (x, y, wt) in [(x0, y0, (1.0 - fx) * (1.0 - fy)), (x1, y0, fx * (1.0 - fy)), (x0, y1, (1.0 - fx) * fy), (x1, y1, fx * fy)] {
        let p = bits[(y * w + x) as usize];
        let a = (p >> 24) as f32 * wt;
        acc[0] += a;
        acc[1] += ((p >> 16) & 0xff) as f32 * a;
        acc[2] += ((p >> 8) & 0xff) as f32 * a;
        acc[3] += (p & 0xff) as f32 * a;
    }
    if acc[0] <= 0.0 {
        return 0;
    }
    let c = |v: f32| ((v / acc[0]) + 0.5).min(255.0) as u32;
    (((acc[0] + 0.5).min(255.0) as u32) << 24) | (c(acc[1]) << 16) | (c(acc[2]) << 8) | c(acc[3])
}
