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

impl Target<'_> {
    fn put(&mut self, x: i32, y: i32, f: impl FnOnce(u32) -> u32) {
        if x >= 0 && y >= 0 && x < self.width && y < self.height {
            let i = (y * self.width + x) as usize;
            self.bits[i] = f(self.bits[i]);
        }
    }

    fn blt(&mut self, src: &crate::sexy::image::Image, x: i32, y: i32, sr: Rect, c: Color, mode: i32, mirror: bool) {
        let ha = mode == 1 && has_alpha(src);
        for row in 0..sr.mHeight {
            for col in 0..sr.mWidth {
                let sx = if mirror { sr.mX + sr.mWidth - 1 - col } else { sr.mX + col };
                let sy = sr.mY + row;
                if sx < 0 || sy < 0 || sx >= src.offset_0x20 || sy >= src.offset_0x24 {
                    continue;
                }
                let s = src.mBits[(sy * src.offset_0x20 + sx) as usize];
                self.put(x + col, y + row, |d| if mode == 1 { additive_px(d, s, c, ha) } else { normal_px(d, s, c) });
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn stretched(&mut self, g: &G, image: Ptr, dest: Rect, src: Rect, clip: Rect, color: Color, draw_mode: i32, mirror: bool) {
        let Some(img) = image_of(g, image) else { return };
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
                self.put(x, y, |d| if draw_mode == 1 { additive_px(d, s, color, ha) } else { normal_px(d, s, color) });
            }
        }
    }

    /// Replays one recorded blitter call.
    pub fn apply(&mut self, g: &G, cmd: &ImageCmd) {
        match *cmd {
            ImageCmd::Blt { image, x, y, src, color, draw_mode } => {
                if let Some(img) = image_of(g, image) {
                    self.blt(img, x, y, src, color, draw_mode, false);
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
                        self.blt(img, x0, y0, s, color, draw_mode, false);
                    }
                }
            }
            ImageCmd::BltMirror { image, x, y, src, color, draw_mode } => {
                if let Some(img) = image_of(g, image) {
                    self.blt(img, x, y, src, color, draw_mode, true);
                }
            }
            ImageCmd::BltStretched { image, dest, src, clip, color, draw_mode, .. } => self.stretched(g, image, dest, src, clip, color, draw_mode, false),
            ImageCmd::BltStretchedMirror { image, dest, src, clip, color, draw_mode, .. } => self.stretched(g, image, dest, src, clip, color, draw_mode, true),
            ImageCmd::FillRect { rect, color, draw_mode } => {
                let s = ((color.mAlpha as u32) << 24) | ((color.mRed as u32) << 16) | ((color.mGreen as u32) << 8) | color.mBlue as u32;
                for y in rect.mY..rect.mY + rect.mHeight {
                    for x in rect.mX..rect.mX + rect.mWidth {
                        self.put(x, y, |d| {
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
        let mut t = Target { bits: &mut bits, width: w, height: h };
        for c in &cmds {
            t.apply(g, c);
        }
    }
    g.image(image).mBits = bits;
}
