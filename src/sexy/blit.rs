//! Software blitters for an opaque 32-bit destination, following `MemoryImage`'s 32-bit
//! routines (`NormalBlt` = `FUN_00461730`, `AdditiveBlt` = `FUN_00460c60`): same channel
//! math, same special cases. The original's screen was a DirectDraw surface whose blitters
//! (DDImage, replaced) did the equivalent per pixel format; this is the 32-bit reference.
//! `MemoryImage`'s run-length tables only skip work, so a per-pixel loop gives the same pixels.

use crate::sexy::graphics::ImageCmd;
use crate::sexy::prelude::*;

/// A destination: 0xAARRGGBB pixels, row-major.
pub struct Target<'a> {
    pub bits: &'a mut [u32],
    pub width: i32,
    pub height: i32,
    /// `mHasAlpha` of the source images drawn so far (sources do not change while a batch of
    /// recorded calls is replayed, so each is scanned once per batch, not once per draw).
    alpha: std::collections::HashMap<Ptr, bool>,
}

#[inline]
pub(crate) fn blend(d: u32, s: u32, a: u32) -> u32 {
    let ia = 0x100 - a;
    (((d & 0xff00ff) * ia >> 8) + ((s & 0xff00ff) * a >> 8)) & 0xffff00ff
        | ((((d & 0xff00) * ia >> 8) + ((s & 0xff00) * a >> 8)) & 0xff00)
        | 0xff00_0000
}

/// `NormalBlt` onto an opaque destination (`mHasAlpha`/`mHasTrans` clear).
pub(crate) fn normal_px(d: u32, s: u32, c: Color) -> u32 {
    let white = c == Color::WHITE;
    if white {
        let a = s >> 24;
        return match a {
            0xff => s,
            0 => d,
            _ => blend(d, s, a),
        };
    }
    // Colorized: u = srcAlpha * colorAlpha / 255; with an opaque destination the new alpha
    // is 255 and the weight is u.
    let u = ((s >> 24) * c.mAlpha as u32) / 0xff;
    if u == 0 {
        return d;
    }
    let da = d >> 24;
    let na = ((0xff - da) * u) / 0xff + da;
    let w = (u * 0xff) / na;
    let iw = 0x100 - w;
    let (r, gg, b) = (c.mRed as u32, c.mGreen as u32, c.mBlue as u32);
    if r == gg && gg == b {
        ((((s & 0xff00ff) * r >> 8 & 0xff00ff) * w >> 8) + ((d & 0xff00ff) * iw >> 8)) & 0xff00ff
            | ((((s & 0xff00) * w * r) >> 0x10) + ((d & 0xff00) * iw >> 8)) & 0xff00
            | (na << 24)
    } else {
        ((((s & 0xff0000) * w >> 8) * r >> 8) + ((d & 0xff0000) * iw >> 8)) & 0xff0000
            | (((s & 0xff) * w * b >> 0x10) + ((d & 0xff) * iw >> 8)) & 0xff
            | (((s & 0xff00) * w * gg >> 0x10) + ((d & 0xff00) * iw >> 8)) & 0xff00
            | (na << 24)
    }
}

/// `AdditiveBlt`: with a white color the source is added as is, otherwise each channel is
/// first scaled by the color pre-multiplied by its alpha; a source with `mHasAlpha` is also
/// weighted by its pixel alpha. Each channel saturates at 255 (the 0x11d max table).
pub(crate) fn additive_px(d: u32, s: u32, c: Color, has_alpha: bool) -> u32 {
    let sat = |v: u32| v.min(0xff);
    let (mut sr, mut sg, mut sb) = (s & 0xff0000, s & 0xff00, s & 0xff);
    if c != Color::WHITE {
        let r = (c.mRed * c.mAlpha / 0xff) as u32;
        let gg = (c.mGreen * c.mAlpha / 0xff) as u32;
        let b = (c.mBlue * c.mAlpha / 0xff) as u32;
        sr = sr * r >> 8;
        sg = sg * gg >> 8;
        sb = sb * b >> 8;
    }
    if has_alpha {
        let a = s >> 24;
        sr = sr * a >> 8;
        sg = sg * a >> 8;
        sb = sb * a >> 8;
    }
    let nr = sat((sr + (d & 0xff0000)) >> 16);
    let ng = sat((sg + (d & 0xff00)) >> 8);
    let nb = sat((d & 0xff) + sb);
    (d & 0xff00_0000) | (nr << 16) | (ng << 8) | nb
}

/// `MemoryImage::mHasAlpha` as `CommitBits` derives it: some pixel is neither fully opaque
/// nor fully transparent.
pub(crate) fn has_alpha(img: &crate::sexy::image::Image) -> bool {
    img.mBits.iter().any(|p| {
        let a = p >> 24;
        a != 0 && a != 0xff
    })
}

impl<'a> Target<'a> {
    pub fn new(bits: &'a mut [u32], width: i32, height: i32) -> Target<'a> {
        Target { bits, width, height, alpha: std::collections::HashMap::new() }
    }

    fn has_alpha(&mut self, image: Ptr, img: &crate::sexy::image::Image) -> bool {
        *self.alpha.entry(image).or_insert_with(|| has_alpha(img))
    }

    /// Pixel for pixel what a loop over the whole source rect does (skipping source pixels
    /// outside the image and destination pixels outside the target), with the skipped rows
    /// and columns cut off up front and the blend picked once per call.
    #[allow(clippy::too_many_arguments)]
    fn blt(&mut self, image: Ptr, src: &crate::sexy::image::Image, x: i32, y: i32, sr: Rect, c: Color, mode: i32, mirror: bool) {
        let (sw, sh) = (src.offset_0x20, src.offset_0x24);
        // Columns whose source x is inside the image.
        let (c0, c1) = if mirror { (sr.mX + sr.mWidth - sw, sr.mX + sr.mWidth) } else { (-sr.mX, sw - sr.mX) };
        let col0 = 0.max(c0).max(-x);
        let col1 = sr.mWidth.min(c1).min(self.width - x);
        let row0 = 0.max(-sr.mY).max(-y);
        let row1 = sr.mHeight.min(sh - sr.mY).min(self.height - y);
        if col0 >= col1 || row0 >= row1 {
            return;
        }
        let ha = mode == 1 && self.has_alpha(image, src);
        let width = self.width;
        let rows = |f: &mut dyn FnMut(&mut [u32], &[u32])| {
            for row in row0..row1 {
                let d0 = ((y + row) * width + x + col0) as usize;
                let n = (col1 - col0) as usize;
                let srow = ((sr.mY + row) * sw) as usize;
                let (s0, s1) = if mirror {
                    ((sr.mX + sr.mWidth - col1) as usize, (sr.mX + sr.mWidth - col0) as usize)
                } else {
                    ((sr.mX + col0) as usize, (sr.mX + col1) as usize)
                };
                f(&mut self.bits[d0..d0 + n], &src.mBits[srow + s0..srow + s1]);
            }
        };
        let mut rows = rows;
        macro_rules! run {
            ($px:expr) => {
                rows(&mut |d: &mut [u32], s: &[u32]| {
                    if mirror {
                        for (d, &s) in d.iter_mut().zip(s.iter().rev()) {
                            *d = $px(*d, s);
                        }
                    } else {
                        for (d, &s) in d.iter_mut().zip(s) {
                            *d = $px(*d, s);
                        }
                    }
                })
            };
        }
        if mode == 1 {
            run!(|d, s| additive_px(d, s, c, ha));
        } else if c == Color::WHITE {
            run!(|d, s| normal_px(d, s, Color::WHITE));
        } else {
            run!(|d, s| normal_px(d, s, c));
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn stretched(&mut self, g: &G, image: Ptr, dest: Rect, src: Rect, clip: Rect, color: Color, draw_mode: i32, mirror: bool) {
        let Some(img) = image_of(g, image) else { return };
        if dest.mWidth <= 0 || dest.mHeight <= 0 {
            return;
        }
        let area = crate::sexy::graphics::FUN_00468430(&dest, &clip);
        let ha = draw_mode == 1 && self.has_alpha(image, img);
        // (Pixels outside the target were skipped one by one; they are cut off here.)
        let (x0, x1) = (area.mX.max(0), (area.mX + area.mWidth).min(self.width));
        let (y0, y1) = (area.mY.max(0), (area.mY + area.mHeight).min(self.height));
        if x0 >= x1 || y0 >= y1 {
            return;
        }
        let cols: Vec<i32> = (x0..x1)
            .map(|x| {
                let col = (x - dest.mX) * src.mWidth / dest.mWidth;
                if mirror { src.mX + src.mWidth - 1 - col } else { src.mX + col }
            })
            .collect();
        for y in y0..y1 {
            let sy = src.mY + (y - dest.mY) * src.mHeight / dest.mHeight;
            if sy < 0 || sy >= img.offset_0x24 {
                continue;
            }
            let srow = (sy * img.offset_0x20) as usize;
            let drow = (y * self.width) as usize;
            for (x, &sx) in (x0..x1).zip(&cols) {
                if sx < 0 || sx >= img.offset_0x20 {
                    continue;
                }
                let s = img.mBits[srow + sx as usize];
                let d = &mut self.bits[drow + x as usize];
                *d = if draw_mode == 1 { additive_px(*d, s, color, ha) } else { normal_px(*d, s, color) };
            }
        }
    }

    /// Replays one recorded blitter call.
    pub fn apply(&mut self, g: &G, cmd: &ImageCmd) {
        match *cmd {
            ImageCmd::Blt { image, x, y, src, color, draw_mode } => {
                if let Some(img) = image_of(g, image) {
                    self.blt(image, img, x, y, src, color, draw_mode, false);
                }
            }
            ImageCmd::BltF { image, x, y, src, clip, color, draw_mode } => {
                // The 3D path's sub-pixel placement (the D3D blitter filters it); the
                // software target snaps to the pixel grid, then clips like `Blt`.
                if let Some(img) = image_of(g, image) {
                    let (ix, iy) = (x.floor() as i32, y.floor() as i32);
                    let x0 = ix.max(clip.mX);
                    let y0 = iy.max(clip.mY);
                    let x1 = (ix + src.mWidth).min(clip.mX + clip.mWidth);
                    let y1 = (iy + src.mHeight).min(clip.mY + clip.mHeight);
                    if x0 < x1 && y0 < y1 {
                        let s = Rect::new(src.mX + x0 - ix, src.mY + y0 - iy, x1 - x0, y1 - y0);
                        self.blt(image, img, x0, y0, s, color, draw_mode, false);
                    }
                }
            }
            ImageCmd::BltMirror { image, x, y, src, color, draw_mode } => {
                if let Some(img) = image_of(g, image) {
                    self.blt(image, img, x, y, src, color, draw_mode, true);
                }
            }
            ImageCmd::BltStretched { image, dest, src, clip, color, draw_mode, .. } => self.stretched(g, image, dest, src, clip, color, draw_mode, false),
            ImageCmd::BltStretchedMirror { image, dest, src, clip, color, draw_mode, .. } => self.stretched(g, image, dest, src, clip, color, draw_mode, true),
            ImageCmd::FillRect { rect, color, draw_mode } => {
                let s = ((color.mAlpha as u32) << 24) | ((color.mRed as u32) << 16) | ((color.mGreen as u32) << 8) | color.mBlue as u32;
                let (x0, x1) = (rect.mX.max(0), (rect.mX + rect.mWidth).min(self.width));
                let (y0, y1) = (rect.mY.max(0), (rect.mY + rect.mHeight).min(self.height));
                if x0 >= x1 || y0 >= y1 {
                    return;
                }
                for y in y0..y1 {
                    let row = (y * self.width) as usize;
                    for d in &mut self.bits[row + x0 as usize..row + x1 as usize] {
                        *d = if draw_mode == 1 {
                            additive_px(*d, 0xffff_ffff, color, false)
                        } else {
                            let a = s >> 24;
                            if a == 0xff { s | 0xff00_0000 } else { blend(*d, s, a) }
                        };
                    }
                }
            }
            ImageCmd::DrawRect { rect, color, draw_mode } => {
                let r = rect;
                for (x, y, w, h) in [
                    (r.mX, r.mY, r.mWidth + 1, 1),
                    (r.mX, r.mY + r.mHeight, r.mWidth + 1, 1),
                    (r.mX, r.mY + 1, 1, r.mHeight - 1),
                    (r.mX + r.mWidth, r.mY + 1, 1, r.mHeight - 1),
                ] {
                    self.apply(g, &ImageCmd::FillRect { rect: Rect::new(x, y, w, h), color, draw_mode });
                }
            }
            ImageCmd::DrawLine { x1, y1, x2, y2, color, draw_mode } => {
                let steps = ((x2 - x1).abs().max((y2 - y1).abs())).ceil().max(1.0) as i32;
                for i in 0..=steps {
                    let t = i as f64 / steps as f64;
                    let x = (x1 + (x2 - x1) * t) as i32;
                    let y = (y1 + (y2 - y1) * t) as i32;
                    self.apply(g, &ImageCmd::FillRect { rect: Rect::new(x, y, 1, 1), color, draw_mode });
                }
            }
        }
    }
}

fn image_of(g: &G, p: Ptr) -> Option<&crate::sexy::image::Image> {
    match &g.obj_ref(p).node {
        Node::Image(i) => Some(i),
        _ => None,
    }
}

/// `new MemoryImage(gApp)` + `Create(w, h)`: a transparent-black offscreen image.
pub fn new_memory_image(g: &mut G, w: i32, h: i32) -> Ptr {
    let mut img = crate::sexy::image::Image::new();
    img.offset_0x20 = w;
    img.offset_0x24 = h;
    img.mBits = vec![0; (w * h) as usize];
    g.alloc(Obj { vt: None, node: Node::Image(Box::new(img)) })
}

/// Draws into an offscreen image: `Graphics g(image); ...;` the recorded blits are applied
/// to the image's pixels when drawing is done (the original blits immediately; nothing
/// reads the pixels in between).
pub fn render_offscreen(g: &mut G, image: Ptr, draw: impl FnOnce(&mut G, &mut Graphics)) {
    let (w, h) = (g.image(image).offset_0x20, g.image(image).offset_0x24);
    let out = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let mut gfx = Graphics::new(image, w, h, out.clone());
    draw(g, &mut gfx);
    let cmds = std::mem::take(&mut *out.lock().unwrap());
    // (Port addition) a file image drawn into no longer matches its HD art.
    let mut hd = std::mem::take(&mut g.hd);
    hd.changed(g, image);
    g.hd = hd;
    let mut bits = std::mem::take(&mut g.image(image).mBits);
    {
        let mut t = Target::new(&mut bits, w, h);
        for c in &cmds {
            t.apply(g, c);
        }
    }
    g.image(image).mBits = bits;
}

#[cfg(test)]
mod tests {
    //! The clipped blitters against the straightforward per-pixel loops they replaced.
    use super::*;
    use crate::sexy::image::Image;

    fn ref_put(t: &mut [u32], w: i32, h: i32, x: i32, y: i32, f: impl FnOnce(u32) -> u32) {
        if x >= 0 && y >= 0 && x < w && y < h {
            let i = (y * w + x) as usize;
            t[i] = f(t[i]);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn ref_blt(t: &mut [u32], w: i32, h: i32, src: &Image, x: i32, y: i32, sr: Rect, c: Color, mode: i32, mirror: bool) {
        let ha = mode == 1 && has_alpha(src);
        for row in 0..sr.mHeight {
            for col in 0..sr.mWidth {
                let sx = if mirror { sr.mX + sr.mWidth - 1 - col } else { sr.mX + col };
                let sy = sr.mY + row;
                if sx < 0 || sy < 0 || sx >= src.offset_0x20 || sy >= src.offset_0x24 {
                    continue;
                }
                let s = src.mBits[(sy * src.offset_0x20 + sx) as usize];
                ref_put(t, w, h, x + col, y + row, |d| if mode == 1 { additive_px(d, s, c, ha) } else { normal_px(d, s, c) });
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn ref_stretched(t: &mut [u32], w: i32, h: i32, img: &Image, dest: Rect, src: Rect, clip: Rect, color: Color, draw_mode: i32, mirror: bool) {
        if dest.mWidth <= 0 || dest.mHeight <= 0 {
            return;
        }
        let area = crate::sexy::graphics::FUN_00468430(&dest, &clip);
        let ha = draw_mode == 1 && has_alpha(img);
        for y in area.mY..area.mY + area.mHeight {
            for x in area.mX..area.mX + area.mWidth {
                let col = (x - dest.mX) * src.mWidth / dest.mWidth;
                let sx = if mirror { src.mX + src.mWidth - 1 - col } else { src.mX + col };
                let sy = src.mY + (y - dest.mY) * src.mHeight / dest.mHeight;
                if sx < 0 || sy < 0 || sx >= img.offset_0x20 || sy >= img.offset_0x24 {
                    continue;
                }
                let s = img.mBits[(sy * img.offset_0x20 + sx) as usize];
                ref_put(t, w, h, x, y, |d| if draw_mode == 1 { additive_px(d, s, color, ha) } else { normal_px(d, s, color) });
            }
        }
    }

    fn ref_fill(t: &mut [u32], w: i32, h: i32, rect: Rect, color: Color, draw_mode: i32) {
        let s = ((color.mAlpha as u32) << 24) | ((color.mRed as u32) << 16) | ((color.mGreen as u32) << 8) | color.mBlue as u32;
        for y in rect.mY..rect.mY + rect.mHeight {
            for x in rect.mX..rect.mX + rect.mWidth {
                ref_put(t, w, h, x, y, |d| {
                    if draw_mode == 1 {
                        additive_px(d, 0xffff_ffff, color, false)
                    } else {
                        let a = s >> 24;
                        if a == 0xff { s | 0xff00_0000 } else { blend(d, s, a) }
                    }
                });
            }
        }
    }

    struct Rng(u64);
    impl Rng {
        fn next(&mut self) -> u32 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            (self.0 >> 16) as u32
        }
        fn range(&mut self, lo: i32, hi: i32) -> i32 {
            lo + (self.next() % (hi - lo) as u32) as i32
        }
    }

    #[test]
    fn clipped_blitters_match_per_pixel_loops() {
        let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
        let mut g = G::default();
        let mut images = Vec::new();
        for i in 0..6 {
            let (w, h) = (rng.range(1, 90), rng.range(1, 70));
            let mut img = Image::new();
            img.offset_0x20 = w;
            img.offset_0x24 = h;
            img.mBits = (0..w * h)
                .map(|_| {
                    let a = match (i, rng.next() % 4) {
                        (0, _) => 0xff,
                        (_, 0) => 0,
                        (_, 1) => 0xff,
                        _ => rng.next() & 0xff,
                    };
                    (a << 24) | (rng.next() & 0xff_ffff)
                })
                .collect();
            let p = g.alloc(Obj { vt: None, node: Node::Image(Box::new(img.clone())) });
            images.push((p, img));
        }
        let (w, h) = (160, 120);
        let colors = [Color::WHITE, Color { mRed: 255, mGreen: 255, mBlue: 255, mAlpha: 128 }, Color { mRed: 200, mGreen: 200, mBlue: 200, mAlpha: 255 }, Color { mRed: 255, mGreen: 40, mBlue: 90, mAlpha: 200 }];
        for round in 0..400 {
            let init: Vec<u32> = (0..w * h).map(|_| if round % 3 == 0 { rng.next() } else { 0xff00_0000 | rng.next() }).collect();
            let mut expect = init.clone();
            let mut got = init;
            let mut cmds = Vec::new();
            for _ in 0..12 {
                let (p, img) = &images[rng.range(0, images.len() as i32) as usize];
                let src = Rect::new(rng.range(-10, img.offset_0x20), rng.range(-10, img.offset_0x24), rng.range(0, img.offset_0x20 + 15), rng.range(0, img.offset_0x24 + 15));
                let color = colors[rng.range(0, 4) as usize];
                let mode = rng.range(0, 2);
                let mirror = rng.next() % 2 == 0;
                let (x, y) = (rng.range(-100, w + 20), rng.range(-80, h + 20));
                match rng.next() % 4 {
                    0 | 1 => {
                        ref_blt(&mut expect, w, h, img, x, y, src, color, mode, mirror);
                        cmds.push(if mirror { ImageCmd::BltMirror { image: *p, x, y, src, color, draw_mode: mode } } else { ImageCmd::Blt { image: *p, x, y, src, color, draw_mode: mode } });
                    }
                    2 => {
                        let dest = Rect::new(x, y, rng.range(-2, 200), rng.range(-2, 150));
                        let clip = Rect::new(rng.range(-20, 60), rng.range(-20, 60), rng.range(0, 200), rng.range(0, 160));
                        ref_stretched(&mut expect, w, h, img, dest, src, clip, color, mode, mirror);
                        cmds.push(if mirror {
                            ImageCmd::BltStretchedMirror { image: *p, dest, src, clip, color, draw_mode: mode, fast_stretch: false }
                        } else {
                            ImageCmd::BltStretched { image: *p, dest, src, clip, color, draw_mode: mode, fast_stretch: false }
                        });
                    }
                    _ => {
                        let rect = Rect::new(x, y, rng.range(-3, 120), rng.range(-3, 90));
                        ref_fill(&mut expect, w, h, rect, color, mode);
                        cmds.push(ImageCmd::FillRect { rect, color, draw_mode: mode });
                    }
                }
            }
            let mut t = Target::new(&mut got, w, h);
            for c in &cmds {
                t.apply(&g, c);
            }
            assert!(expect == got, "round {round} differs");
        }
    }
}
