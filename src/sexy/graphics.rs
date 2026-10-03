//! `Sexy::Graphics`: the drawing context every `Draw(Graphics*)` receives.
//!
//! The state and every method the game calls are ported as written. Where the original
//! hands a finished, clipped primitive to the destination image's virtual blitter
//! (`DDImage::Blt`, `FillRect`, ...), the port records an [`ImageCmd`] with exactly those
//! arguments instead; the Bevy renderer (`crate::render`) replays them. That keeps all of
//! the game-visible arithmetic (translation, clipping, cel math, colorize) identical.

use crate::sexy::prelude::*;
use std::sync::{Arc, Mutex};

/// One call into the destination image's blitter, with the arguments the original passed.
#[derive(Clone, Debug, PartialEq)]
pub enum ImageCmd {
    /// `Image` vslot 10 (`Blt(Image*, int x, int y, const Rect& src, const Color&, int drawMode)`).
    Blt { image: Ptr, x: i32, y: i32, src: Rect, color: Color, draw_mode: i32 },
    /// `Image` vslot 13 (`BltStretched(Image*, const Rect& dest, const Rect& src, const Rect& clip, const Color&, int drawMode, bool fastStretch)`).
    BltStretched { image: Ptr, dest: Rect, src: Rect, clip: Rect, color: Color, draw_mode: i32, fast_stretch: bool },
    /// `Image` vslot 17 (`BltStretchedMirror(Image*, const Rect& dest, const Rect& src, const Rect& clip, const Color&, int drawMode, bool fastStretch)`).
    BltStretchedMirror { image: Ptr, dest: Rect, src: Rect, clip: Rect, color: Color, draw_mode: i32, fast_stretch: bool },
    /// `Image` vslot 16 (`BltMirror(Image*, int x, int y, const Rect& src, const Color&, int drawMode)`).
    BltMirror { image: Ptr, x: i32, y: i32, src: Rect, color: Color, draw_mode: i32 },
    /// `Image` vslot 11 (`BltF(Image*, float x, float y, const Rect& src, const Rect& clip, const Color&, int drawMode)`).
    BltF { image: Ptr, x: f32, y: f32, src: Rect, clip: Rect, color: Color, draw_mode: i32 },
    /// `Image` vslot 3 (`FillRect(const Rect&, const Color&, int drawMode)`).
    FillRect { rect: Rect, color: Color, draw_mode: i32 },
    /// `Image` vslot 4 (`DrawRect(const Rect&, const Color&, int drawMode)`).
    DrawRect { rect: Rect, color: Color, draw_mode: i32 },
    /// `Image` vslot 6 (`DrawLine(double x1, double y1, double x2, double y2, const Color&, int drawMode)`).
    DrawLine { x1: f64, y1: f64, x2: f64, y2: f64, color: Color, draw_mode: i32 },
}

/// Commands recorded into one destination image during a frame.
pub type ImageCmds = Arc<Mutex<Vec<ImageCmd>>>;

/// `Sexy::GraphicsState` (object offsets 0x4..0x4c).
#[derive(Clone, Debug)]
pub struct GraphicsState {
    /// +0x04 `mDestImage`.
    pub mDestImage: Ptr,
    /// +0x08 `mTransX`.
    pub mTransX: f32,
    /// +0x0c `mTransY`.
    pub mTransY: f32,
    /// +0x10 `mScaleX`.
    pub mScaleX: f32,
    /// +0x14 `mScaleY`.
    pub mScaleY: f32,
    /// +0x18 `mScaleOrigX`.
    pub mScaleOrigX: f32,
    /// +0x1c `mScaleOrigY`.
    pub mScaleOrigY: f32,
    /// +0x20 `mClipRect`.
    pub mClipRect: Rect,
    /// +0x30 `mColor`.
    pub mColor: Color,
    /// +0x40 `mFont`.
    pub mFont: Ptr,
    /// +0x44 `mDrawMode`.
    pub mDrawMode: i32,
    /// +0x48 `mColorizeImages`.
    pub mColorizeImages: bool,
    /// +0x49 `mFastStretch`.
    pub mFastStretch: bool,
    /// +0x4a `mWriteColoredString`.
    pub mWriteColoredString: bool,
    /// +0x4b `mLinearBlend`.
    pub mLinearBlend: bool,
    /// +0x4c `mIs3D`.
    pub mIs3D: bool,
}

/// `Sexy::Graphics` (0x68 bytes).
#[derive(Clone, Debug)]
pub struct Graphics {
    pub s: GraphicsState,
    /// +0x5c `mStateStack` (std::list<GraphicsState>).
    pub mStateStack: Vec<GraphicsState>,
    /// Where blits into `mDestImage` are recorded (shared by copies, like the image itself).
    pub out: ImageCmds,
}

/// `Graphics::DRAWMODE_NORMAL` / `DRAWMODE_ADDITIVE`.
pub const DRAWMODE_NORMAL: i32 = 0;
pub const DRAWMODE_ADDITIVE: i32 = 1;

/// x87 `FILD int; FADD float; call _ftol2`: the original's int + float translation.
#[inline]
pub fn trans_i(v: i32, t: f32) -> i32 {
    crate::sexy::crt::ftol(v as f64 + t as f64) as i32
}

impl Graphics {
    /// `Graphics::Graphics(Image* theDestImage)` (@ 00455610) for an image of `width`x`height`.
    pub fn new(dest_image: Ptr, width: i32, height: i32, out: ImageCmds) -> Graphics {
        Graphics {
            s: GraphicsState {
                mDestImage: dest_image,
                mTransX: 0.0,
                mTransY: 0.0,
                mScaleX: 1.0,
                mScaleY: 1.0,
                mScaleOrigX: 0.0,
                mScaleOrigY: 0.0,
                mClipRect: Rect::new(0, 0, width, height),
                mColor: FUN_00433300(),
                mFont: NULL,
                mDrawMode: DRAWMODE_NORMAL,
                mColorizeImages: false,
                mFastStretch: false,
                mWriteColoredString: true,
                mLinearBlend: false,
                mIs3D: false,
            },
            mStateStack: Vec::new(),
            out,
        }
    }

    fn blt_color(&self) -> Color {
        if self.s.mColorizeImages { self.s.mColor } else { Color::WHITE }
    }
}

/// port: 00455800 FUN_00455800
/// `Graphics::Create()`: a new Graphics with a copy of this one's state (`new Graphics(*this)`).
pub fn FUN_00455800(this: &Graphics) -> Graphics {
    Graphics { s: this.s.clone(), mStateStack: Vec::new(), out: this.out.clone() }
}

/// port: 00455710 FUN_00455710
/// `Graphics::PushState()`.
pub fn FUN_00455710(this: &mut Graphics) {
    this.mStateStack.push(this.s.clone());
}

/// port: 004557a0 FUN_004557a0
/// `Graphics::PopState()`: restores the last pushed state; a no-op on an empty stack.
pub fn FUN_004557a0(this: &mut Graphics) {
    if let Some(s) = this.mStateStack.pop() {
        this.s = s;
    }
}

/// port: 00455870 FUN_00455870
/// `Graphics::GetFont()`.
pub fn FUN_00455870(this: &Graphics) -> Ptr {
    this.s.mFont
}

/// port: 00455880 FUN_00455880
/// `Graphics::SetFont(Font*)`.
pub fn FUN_00455880(this: &mut Graphics, param_1: Ptr) {
    this.s.mFont = param_1;
}

/// port: 00455890 FUN_00455890
/// `Graphics::SetColor(const Color&)`.
pub fn FUN_00455890(this: &mut Graphics, param_1: Color) {
    this.s.mColor = param_1;
}

/// port: 004558b0 FUN_004558b0
/// `Graphics::GetColor()`.
pub fn FUN_004558b0(this: &Graphics) -> Color {
    this.s.mColor
}

/// port: 004558c0 FUN_004558c0
/// `Graphics::SetDrawMode(int)`.
pub fn FUN_004558c0(this: &mut Graphics, param_1: i32) {
    this.s.mDrawMode = param_1;
}

/// port: 004558d0 FUN_004558d0
/// `Graphics::GetDrawMode()`.
pub fn FUN_004558d0(this: &Graphics) -> i32 {
    this.s.mDrawMode
}

/// port: 004558e0 FUN_004558e0
/// `Graphics::SetColorizeImages(bool)`.
pub fn FUN_004558e0(this: &mut Graphics, param_1: bool) {
    this.s.mColorizeImages = param_1;
}

/// port: 004558f0 FUN_004558f0
/// `Graphics::GetColorizeImages()`.
pub fn FUN_004558f0(this: &Graphics) -> bool {
    this.s.mColorizeImages
}

/// port: 00455900 FUN_00455900
/// `Graphics::SetFastStretch(bool)`.
pub fn FUN_00455900(this: &mut Graphics, param_1: bool) {
    this.s.mFastStretch = param_1;
}

/// port: 00455910 FUN_00455910
/// `Graphics::SetLinearBlend(bool)`.
pub fn FUN_00455910(this: &mut Graphics, param_1: bool) {
    this.s.mLinearBlend = param_1;
}

/// port: 00468430 FUN_00468430
/// `TRect<int>::Intersection(const TRect&)`; empty results become (0,0,0,0).
pub fn FUN_00468430(this: &Rect, param_2: &Rect) -> Rect {
    let x1 = if this.mX <= param_2.mX { param_2.mX } else { this.mX };
    let x2 = if param_2.mX + param_2.mWidth <= this.mX + this.mWidth { param_2.mX + param_2.mWidth } else { this.mX + this.mWidth };
    let y1 = if this.mY <= param_2.mY { param_2.mY } else { this.mY };
    let y2 = if param_2.mY + param_2.mHeight <= this.mY + this.mHeight { param_2.mY + param_2.mHeight } else { this.mY + this.mHeight };
    if -1 < x2 - x1 && -1 < y2 - y1 {
        Rect::new(x1, y1, x2 - x1, y2 - y1)
    } else {
        Rect::new(0, 0, 0, 0)
    }
}

/// port: 00455920 FUN_00455920
/// `Graphics::FillRect(int x, int y, int w, int h)`: nothing when the color is transparent.
pub fn FUN_00455920(this: &mut Graphics, param_1: i32, param_2: i32, param_3: i32, param_4: i32) {
    if this.s.mColor.mAlpha != 0 {
        let r = Rect::new(trans_i(param_1, this.s.mTransX), trans_i(param_2, this.s.mTransY), param_3, param_4);
        let rect = FUN_00468430(&r, &this.s.mClipRect);
        this.out.lock().unwrap().push(ImageCmd::FillRect { rect, color: this.s.mColor, draw_mode: this.s.mDrawMode });
    }
}

/// port: 00455990 FUN_00455990
/// `Graphics::FillRect(const Rect&)`.
pub fn FUN_00455990(this: &mut Graphics, param_1: &Rect) {
    FUN_00455920(this, param_1.mX, param_1.mY, param_1.mWidth, param_1.mHeight);
}

/// port: 004559b0 FUN_004559b0
/// `Graphics::DrawRect(int x, int y, int w, int h)`: one primitive when unclipped,
/// otherwise four 1-pixel fills (the outline is w+1 by h+1 pixels).
pub fn FUN_004559b0(this: &mut Graphics, param_1: i32, param_2: i32, param_3: i32, param_4: i32) {
    if this.s.mColor.mAlpha != 0 {
        let x = trans_i(param_1, this.s.mTransX);
        let y = trans_i(param_2, this.s.mTransY);
        let full = Rect::new(x, y, param_3, param_4);
        let r = Rect::new(x, y, param_3 + 1, param_4 + 1);
        let c = FUN_00468430(&r, &this.s.mClipRect);
        if x == c.mX && y == c.mY && param_3 + 1 == c.mWidth && param_4 + 1 == c.mHeight {
            this.out.lock().unwrap().push(ImageCmd::DrawRect { rect: full, color: this.s.mColor, draw_mode: this.s.mDrawMode });
            return;
        }
        FUN_00455920(this, param_1, param_2, param_3 + 1, 1);
        FUN_00455920(this, param_1, param_2 + param_4, param_3 + 1, 1);
        FUN_00455920(this, param_1, param_2 + 1, 1, param_4 - 1);
        FUN_00455920(this, param_1 + param_3, param_2 + 1, 1, param_4 - 1);
    }
}

/// port: 00455ac0 FUN_00455ac0
/// `Graphics::DrawRect(const Rect&)`.
pub fn FUN_00455ac0(this: &mut Graphics, param_1: &Rect) {
    FUN_004559b0(this, param_1.mX, param_1.mY, param_1.mWidth, param_1.mHeight);
}

/// port: 00455cf0 FUN_00455cf0
/// `Graphics::DrawString(const std::string&, int x, int y)`: the font draws with this
/// Graphics' color and clip rect; no font, no text.
pub fn FUN_00455cf0(this: &mut Graphics, g: &mut G, param_1: &[u8], param_2: i32, param_3: i32) {
    if this.s.mFont != NULL {
        let (f, c, clip) = (this.s.mFont, this.s.mColor, this.s.mClipRect);
        crate::sexy::image_font::draw_string(g, f, this, param_2, param_3, param_1, c, clip);
    }
}

/// port: 00455d20 FUN_00455d20
/// `Graphics::DrawImage(Image*, int x, int y)`.
pub fn FUN_00455d20(this: &mut Graphics, g: &mut G, param_1: Ptr, param_2: i32, param_3: i32) {
    if this.s.mScaleX == 1.0 && this.s.mScaleY == 1.0 {
        let x = trans_i(param_2, this.s.mTransX);
        let y = trans_i(param_3, this.s.mTransY);
        let (w, h) = (FUN_00457480(g, param_1), FUN_00457490(g, param_1));
        let d = FUN_00468430(&Rect::new(x, y, w, h), &this.s.mClipRect);
        let src = Rect::new(d.mX - x, d.mY - y, d.mWidth, d.mHeight);
        if 0 < d.mWidth && 0 < d.mHeight {
            let color = this.blt_color();
            this.out.lock().unwrap().push(ImageCmd::Blt { image: param_1, x: d.mX, y: d.mY, src, color, draw_mode: this.s.mDrawMode });
        }
    } else {
        let src = Rect::new(0, 0, g.image(param_1).offset_0x20, g.image(param_1).offset_0x24);
        FUN_00455e40(this, g, param_1, param_2, param_3, &src);
    }
}

/// port: 00455e40 FUN_00455e40
/// `Graphics::DrawImage(Image*, int x, int y, const Rect& src)`: ignores source rects that
/// run past the image.
pub fn FUN_00455e40(this: &mut Graphics, g: &mut G, param_1: Ptr, param_2: i32, param_3: i32, param_4: &Rect) {
    if param_4.mX + param_4.mWidth > FUN_00457480(g, param_1) {
        return;
    }
    if param_4.mY + param_4.mHeight > FUN_00457490(g, param_1) {
        return;
    }
    let x = trans_i(param_2, this.s.mTransX);
    let y = trans_i(param_3, this.s.mTransY);
    if this.s.mScaleX == 1.0 && this.s.mScaleY == 1.0 {
        let d = FUN_00468430(&Rect::new(x, y, param_4.mWidth, param_4.mHeight), &this.s.mClipRect);
        let src = Rect::new(param_4.mX - x + d.mX, param_4.mY - y + d.mY, d.mWidth, d.mHeight);
        if 0 < d.mWidth && 0 < d.mHeight {
            let color = this.blt_color();
            this.out.lock().unwrap().push(ImageCmd::Blt { image: param_1, x: d.mX, y: d.mY, src, color, draw_mode: this.s.mDrawMode });
        }
    } else {
        // x87 sequence: each product is stored to a float, widened to double for
        // _ceil (FUN_0056a430) / _floor (FUN_00581fe0), stored to a float again, then _ftol2.
        let s = &this.s;
        let ftol = crate::sexy::crt::ftol;
        let h = ((param_4.mHeight as f64 * s.mScaleY as f64) as f32 as f64).ceil() as f32;
        let w = ((param_4.mWidth as f64 * s.mScaleX as f64) as f32 as f64).ceil() as f32;
        let fx = (((x as f64 - s.mScaleOrigX as f64) * s.mScaleX as f64) as f32 as f64).floor() as f32;
        let fy = (((y as f64 - s.mScaleOrigY as f64) * s.mScaleY as f64) as f32 as f64).floor() as f32;
        let dest = Rect::new(
            ftol(fx as f64 + s.mScaleOrigX as f64) as i32,
            ftol(fy as f64 + s.mScaleOrigY as f64) as i32,
            ftol(w as f64) as i32,
            ftol(h as f64) as i32,
        );
        let color = this.blt_color();
        this.out.lock().unwrap().push(ImageCmd::BltStretched {
            image: param_1,
            dest,
            src: *param_4,
            clip: s.mClipRect,
            color,
            draw_mode: s.mDrawMode,
            fast_stretch: s.mFastStretch,
        });
    }
}

/// port: 004560a0 FUN_004560a0
/// `Graphics::DrawImageMirror(Image*, int x, int y, const Rect& src, bool mirror)`.
pub fn FUN_004560a0(this: &mut Graphics, g: &mut G, param_1: Ptr, param_2: i32, param_3: i32, param_4: &Rect, param_5: bool) {
    if !param_5 {
        FUN_00455e40(this, g, param_1, param_2, param_3, param_4);
        return;
    }
    let x = trans_i(param_2, this.s.mTransX);
    let y = trans_i(param_3, this.s.mTransY);
    if param_4.mX + param_4.mWidth <= FUN_00457480(g, param_1) && param_4.mHeight + param_4.mY <= FUN_00457490(g, param_1) {
        let d = FUN_00468430(&Rect::new(x, y, param_4.mWidth, param_4.mHeight), &this.s.mClipRect);
        let src = Rect::new(param_4.mX + param_4.mWidth - d.mX - d.mWidth + x, param_4.mY - y + d.mY, d.mWidth, d.mHeight);
        if 0 < d.mWidth && 0 < d.mHeight {
            let color = this.blt_color();
            this.out.lock().unwrap().push(ImageCmd::BltMirror { image: param_1, x: d.mX, y: d.mY, src, color, draw_mode: this.s.mDrawMode });
        }
    }
}

/// port: 004563b0 FUN_004563b0
/// `Graphics::ClipRect(const Rect&)`.
pub fn FUN_004563b0(this: &mut Graphics, param_1: &Rect) {
    FUN_00456340(this, param_1.mX, param_1.mY, param_1.mWidth, param_1.mHeight);
}

/// port: 00455ae0 FUN_00455ae0
/// `Graphics::DrawLineClipHelper(double* x1, double* y1, double* x2, double* y2)`: clips
/// the segment to the clip rect in x, then in y; false when nothing is left. The far edges
/// clip to `right - 1` / `bottom - 1`.
pub fn FUN_00455ae0(this: &Graphics, x1: &mut f64, y1: &mut f64, x2: &mut f64, y2: &mut f64) -> bool {
    let c = this.s.mClipRect;
    // Left-to-right: (xa, ya) is the left end, (xb, yb) the right end.
    let (mut xa, mut ya, mut xb, mut yb) = (*x1, *y1, *x2, *y2);
    if *x2 < *x1 {
        xa = *x2;
        xb = *x1;
        yb = *y1;
        ya = *y2;
    }
    let cx = c.mX as f64;
    if xa < cx {
        if cx > xb {
            return false;
        }
        ya = ((yb - ya) / (xb - xa)) * (c.mX as f64 - xa) + ya;
        xa = c.mX as f64;
    }
    let right = (c.mWidth + c.mX) as f64;
    if right <= xb {
        if right <= xa {
            return false;
        }
        let x = (c.mX - 1 + c.mWidth) as f64;
        yb = ((yb - ya) / (xb - xa)) * (x - xb) + yb;
        xb = x;
    }
    // Top-to-bottom: (xt, yt) is the upper end, (xu, yu) the lower end.
    let (mut xt, mut yt, mut xu, mut yu) = (xa, ya, xb, yb);
    if yb < ya {
        yu = ya;
        yt = yb;
        xt = xb;
        xu = xa;
    }
    let cy = c.mY as f64;
    if yt < cy {
        if cy > yu {
            return false;
        }
        xt = ((xu - xt) / (yu - yt)) * (c.mY as f64 - yt) + xt;
        yt = c.mY as f64;
    }
    let bottom = (c.mHeight + c.mY) as f64;
    if bottom > yu {
        *x1 = xt;
        *y1 = yt;
        *x2 = xu;
        *y2 = yu;
        return true;
    }
    if bottom > yt {
        let y = (c.mY - 1 + c.mHeight) as f64;
        *x1 = xt;
        *y1 = yt;
        *x2 = ((xu - xt) / (yu - yt)) * (y - yu) + xu;
        *y2 = y;
        return true;
    }
    false
}

/// port: 00455c50 FUN_00455c50
/// `Graphics::DrawLine(int x1, int y1, int x2, int y2)`: translated, clipped, then the
/// destination image's `DrawLine` (vslot 6) with this color and draw mode.
pub fn FUN_00455c50(this: &mut Graphics, param_1: i32, param_2: i32, param_3: i32, param_4: i32) {
    let mut x1 = param_1 as f64 + this.s.mTransX as f64;
    let mut y1 = param_2 as f64 + this.s.mTransY as f64;
    let mut x2 = param_3 as f64 + this.s.mTransX as f64;
    let mut y2 = param_4 as f64 + this.s.mTransY as f64;
    if FUN_00455ae0(this, &mut x1, &mut y1, &mut x2, &mut y2) {
        this.out.lock().unwrap().push(ImageCmd::DrawLine { x1, y1, x2, y2, color: this.s.mColor, draw_mode: this.s.mDrawMode });
    }
}

/// port: 00456260 FUN_00456260
/// `Graphics::DrawImage(Image*, const Rect& dest, const Rect& src)`: stretched (image vslot
/// 13) with the destination translated.
pub fn FUN_00456260(this: &mut Graphics, param_1: Ptr, param_2: &Rect, param_3: &Rect) {
    let dest = Rect::new(trans_i(param_2.mX, this.s.mTransX), trans_i(param_2.mY, this.s.mTransY), param_2.mWidth, param_2.mHeight);
    let color = this.blt_color();
    let s = &this.s;
    this.out.lock().unwrap().push(ImageCmd::BltStretched {
        image: param_1,
        dest,
        src: *param_3,
        clip: s.mClipRect,
        color,
        draw_mode: s.mDrawMode,
        fast_stretch: s.mFastStretch,
    });
}

/// port: 004561c0 FUN_004561c0
/// `Graphics::DrawImageMirror(Image*, const Rect& dest, const Rect& src, bool mirror)`:
/// stretched, mirrored through image vslot 17.
pub fn FUN_004561c0(this: &mut Graphics, param_1: Ptr, param_2: &Rect, param_3: &Rect, param_4: bool) {
    if !param_4 {
        FUN_00456260(this, param_1, param_2, param_3);
        return;
    }
    let dest = Rect::new(trans_i(param_2.mX, this.s.mTransX), trans_i(param_2.mY, this.s.mTransY), param_2.mWidth, param_2.mHeight);
    let color = this.blt_color();
    let s = &this.s;
    this.out.lock().unwrap().push(ImageCmd::BltStretchedMirror {
        image: param_1,
        dest,
        src: *param_3,
        clip: s.mClipRect,
        color,
        draw_mode: s.mDrawMode,
        fast_stretch: s.mFastStretch,
    });
}

/// port: 00456060 FUN_00456060
/// `Graphics::DrawImageMirror(Image*, int x, int y, bool mirror)`: the whole image.
pub fn FUN_00456060(this: &mut Graphics, g: &mut G, param_1: Ptr, param_2: i32, param_3: i32, param_4: bool) {
    let src = Rect::new(0, 0, g.image(param_1).offset_0x20, g.image(param_1).offset_0x24);
    FUN_004560a0(this, g, param_1, param_2, param_3, &src, param_4);
}

/// port: 004563d0 FUN_004563d0
/// `Graphics::Translate(int x, int y)`.
pub fn FUN_004563d0(this: &mut Graphics, param_1: i32, param_2: i32) {
    this.s.mTransX = param_1 as f32 + this.s.mTransX;
    this.s.mTransY = param_2 as f32 + this.s.mTransY;
}

/// port: 00456340 FUN_00456340
/// `Graphics::ClipRect(int x, int y, int w, int h)`: intersects the clip rect.
pub fn FUN_00456340(this: &mut Graphics, param_1: i32, param_2: i32, param_3: i32, param_4: i32) {
    let r = Rect::new(trans_i(param_1, this.s.mTransX), trans_i(param_2, this.s.mTransY), param_3, param_4);
    this.s.mClipRect = FUN_00468430(&this.s.mClipRect, &r);
}

/// port: 00456980 FUN_00456980
/// `Graphics::DrawImageCel(Image*, int x, int y, int col, int row)`.
pub fn FUN_00456980(this: &mut Graphics, g: &mut G, param_1: Ptr, param_2: i32, param_3: i32, param_4: i32, param_5: i32) {
    let (rows, cols, w, h) = {
        let i = g.image(param_1);
        (i.offset_0x28, i.offset_0x2c, i.offset_0x20, i.offset_0x24)
    };
    if -1 < param_5 && -1 < param_4 && param_5 < rows && param_4 < cols {
        let cw = w / cols;
        let ch = h / rows;
        let src = Rect::new(cw * param_4, ch * param_5, cw, ch);
        FUN_00455e40(this, g, param_1, param_2, param_3, &src);
    }
}

/// port: 00456950 FUN_00456950
/// `Graphics::DrawImageCel(Image*, int x, int y, int cel)`: cel index row-major.
pub fn FUN_00456950(this: &mut Graphics, g: &mut G, param_1: Ptr, param_2: i32, param_3: i32, param_4: i32) {
    let cols = g.image(param_1).offset_0x2c;
    FUN_00456980(this, g, param_1, param_2, param_3, param_4 % cols, param_4 / cols);
}

/// port: 004562e0 FUN_004562e0
/// `Graphics::DrawImageF(Image*, float x, float y, const Rect& src)`: the destination's
/// `BltF` at the translated position, clipped, in the current colour when colourizing
/// (else white, `DAT_005ec478`) and draw mode.
pub fn FUN_004562e0(this: &mut Graphics, _g: &mut G, param_1: Ptr, param_2: f32, param_3: f32, param_4: &Rect) {
    let color = this.blt_color();
    this.out.lock().unwrap().push(ImageCmd::BltF {
        image: param_1,
        x: this.s.mTransX + param_2,
        y: this.s.mTransY + param_3,
        src: *param_4,
        clip: this.s.mClipRect,
        color,
        draw_mode: this.s.mDrawMode,
    });
}

/// port: 00457480 FUN_00457480
/// `Image::GetWidth()`.
pub fn FUN_00457480(g: &mut G, this: Ptr) -> i32 {
    g.image(this).offset_0x20
}

/// port: 00457490 FUN_00457490
/// `Image::GetHeight()`.
pub fn FUN_00457490(g: &mut G, this: Ptr) -> i32 {
    g.image(this).offset_0x24
}

/// port: 004574a0 FUN_004574a0
/// `Image::GetCelHeight()`.
pub fn FUN_004574a0(g: &mut G, this: Ptr) -> i32 {
    let i = g.image(this);
    i.offset_0x24 / i.offset_0x28
}

/// port: 004574b0 FUN_004574b0
/// `Image::GetCelWidth()`.
pub fn FUN_004574b0(g: &mut G, this: Ptr) -> i32 {
    let i = g.image(this);
    i.offset_0x20 / i.offset_0x2c
}

// Font metrics go through the font's vtable (+0x04 `GetAscent`, +0x08 `GetAscentPadding`,
// +0x0c `GetDescent`, +0x18 `GetLineSpacing`, +0x1c `StringWidth`, +0x24 `CharWidthKern`);
// every font the game creates is an `ImageFont`.
use crate::sexy::image_font as font;

/// port: 00456a30 FUN_00456a30
/// `Graphics::WriteString(const string&, int x, int y, int theWidth, int theJustification,
/// bool drawString, int theOffset, int theLength, int theOldColor)`: draws `^RRGGBB^`-colored
/// text (`^oldclr^` = `theOldColor`, `^^` = a literal caret) when `mWriteColoredString`; returns
/// the width written. Justification 0 centers in `theWidth`, 1 right-aligns.
pub fn FUN_00456a30(
    this: &mut Graphics,
    g: &mut G,
    param_1: &[u8],
    mut param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: bool,
    param_7: i32,
    param_8: i32,
    param_9: u32,
) -> i32 {
    let param_9 = if param_9 == 0xffff_ffff { FUN_004333e0(&FUN_004558b0(this)) } else { param_9 };
    if param_6 {
        if param_5 == 0 {
            let w = FUN_00456a30(this, g, param_1, param_2, param_3, param_4, -1, false, param_7, param_8, param_9);
            param_2 += (param_4 - w) / 2;
        } else if param_5 == 1 {
            let w = FUN_00456a30(this, g, param_1, param_2, param_3, param_4, -1, false, param_7, param_8, param_9);
            param_2 += param_4 - w;
        }
    }
    let len = param_1.len() as i32;
    let end = if param_8 < 0 || len < param_8 + param_7 { len } else { param_8 + param_7 };
    let mut cur: Vec<u8> = Vec::new();
    let mut x_off = 0;
    let mut i = param_7;
    while i < end {
        let c = param_1[i as usize];
        if c == b'^' && this.s.mWriteColoredString {
            if i + 1 < end && param_1[(i + 1) as usize] == b'^' {
                cur.push(b'^');
                i += 2;
                continue;
            }
            if end - 8 < i {
                break;
            }
            let mut color: u32 = 0;
            if param_1[(i + 1) as usize] == b'o' {
                if param_1[(i + 1) as usize..].starts_with(b"oldclr") {
                    color = param_9;
                }
            } else {
                for k in 0..6 {
                    let d = param_1[(i + 1 + k) as usize];
                    let v = if d.wrapping_sub(0x30) < 10 {
                        (d - 0x30) as u32
                    } else if d.wrapping_add(0xbf) < 6 {
                        (d - 0x37) as u32
                    } else if d.wrapping_add(0x9f) < 6 {
                        (d - 0x57) as u32
                    } else {
                        0
                    };
                    color = color.wrapping_add(v << (20 - 4 * k));
                }
            }
            if param_6 {
                FUN_00455cf0(this, g, &cur, x_off + param_2, param_3);
                let a = FUN_004558b0(this).mAlpha;
                FUN_00455890(this, CRect((color >> 16 & 0xff) as i32, (color >> 8 & 0xff) as i32, (color & 0xff) as i32, a));
            }
            i += 7;
            let f = FUN_00455870(this);
            x_off += font::string_width(g, f, &cur);
            cur.clear();
        } else {
            cur.push(c);
        }
        i += 1;
    }
    if param_6 {
        FUN_00455cf0(this, g, &cur, x_off + param_2, param_3);
    }
    let f = FUN_00455870(this);
    font::string_width(g, f, &cur) + x_off
}

/// port: 00456e00 FUN_00456e00
/// `WriteString(theLine.substr(start, count), ...)` as inlined in `WriteWordWrapped` (start in
/// ECX, count in EAX, length in EDX): the count is clipped to the string; -1 when nothing is
/// left past `start`.
pub fn FUN_00456e00(
    this: &mut Graphics,
    g: &mut G,
    line: &[u8],
    start: i32,
    mut count: i32,
    x: i32,
    y: i32,
    width: i32,
    justify: i32,
    old_color: u32,
) -> i32 {
    let len = line.len() as i32;
    if len < start + count {
        count = len - start;
        if count < 1 {
            return -1;
        }
    }
    FUN_00456a30(this, g, line, x, y, width, justify, true, start, count, old_color)
}

/// port: 00456e50 FUN_00456e50
/// `Graphics::WriteWordWrapped(const Rect&, const string&, int theLineSpacing (-1 = the
/// font's), int theJustification)`: breaks at spaces (or mid-word when a word is wider than
/// the rect) and at '\n'; returns the text height. Lines broken at a space are drawn only when
/// their baseline is inside the clip rect.
pub fn FUN_00456e50(this: &mut Graphics, g: &mut G, rect: &Rect, line: &[u8], mut spacing: i32, justify: i32) -> i32 {
    let orig_color = FUN_004558b0(this);
    let mut orig_int = FUN_004333e0(&orig_color);
    if orig_int & 0xff00_0000 == 0xff00_0000 {
        orig_int &= 0x00ff_ffff;
    }
    let len = line.len();
    let f = FUN_00455870(this);
    let pad = font::get_ascent_padding(g, f);
    let mut y_off = font::get_ascent(g, f) - pad;
    if spacing == -1 {
        spacing = font::get_line_spacing(g, f);
    }
    let mut cur_pos: usize = 0;
    let mut line_start: usize = 0;
    let mut cur_width: i32 = 0;
    let mut cur_char: u8 = 0;
    let mut prev_char: u8 = 0;
    let mut space_pos: i32 = -1;
    let mut max_width: i32 = 0;
    while cur_pos < len {
        cur_char = line[cur_pos];
        if cur_char == b'^' && this.s.mWriteColoredString && cur_pos + 1 < len {
            if line[cur_pos + 1] == b'^' {
                cur_pos += 1;
            } else {
                cur_pos += 8;
                continue;
            }
        } else if cur_char == b' ' {
            space_pos = cur_pos as i32;
        } else if cur_char == b'\n' {
            cur_width = rect.mWidth + 1;
            space_pos = cur_pos as i32;
            cur_pos += 1;
        }
        cur_width += font::char_width_kern(g, f, cur_char, prev_char);
        prev_char = cur_char;
        if cur_width <= rect.mWidth {
            cur_pos += 1;
            continue;
        }
        let written;
        if space_pos != -1 {
            let phys = crate::sexy::crt::ftol(this.s.mTransY as f64 + (rect.mY + y_off) as f64) as i32;
            let clip = this.s.mClipRect;
            if clip.mY <= phys && phys < clip.mY + clip.mHeight + spacing {
                let count = space_pos - line_start as i32;
                FUN_00456e00(this, g, line, line_start as i32, count, rect.mX, rect.mY + y_off, rect.mWidth, justify, orig_int);
            }
            written = cur_width;
            if written < 0 {
                break;
            }
            cur_pos = (space_pos + 1) as usize;
            if cur_char != b'\n' {
                while cur_pos < len && line[cur_pos] == b' ' {
                    cur_pos += 1;
                }
            }
        } else {
            if (cur_pos as i32) < line_start as i32 + 1 {
                cur_pos += 1;
            }
            let count = cur_pos as i32 - line_start as i32;
            written = FUN_00456e00(this, g, line, line_start as i32, count, rect.mX, rect.mY + y_off, rect.mWidth, justify, orig_int);
            if written < 0 {
                break;
            }
        }
        if max_width < written {
            max_width = written;
        }
        y_off += spacing;
        line_start = cur_pos;
        space_pos = -1;
        cur_width = 0;
        prev_char = 0;
    }
    let bottom;
    if line_start < len {
        let r = FUN_00456e00(this, g, line, line_start as i32, (len - line_start) as i32, rect.mX, rect.mY + y_off, rect.mWidth, justify, orig_int);
        bottom = if r < 0 { y_off } else { y_off + spacing };
    } else {
        if cur_char == b'\n' {
            y_off += spacing;
        }
        bottom = y_off;
    }
    // aMaxWidth is computed but this build's WriteWordWrapped does not return it.
    let _ = max_width;
    FUN_00455890(this, orig_color);
    font::get_descent(g, f) + (bottom - spacing)
}

/// port: 004571f0 FUN_004571f0
/// `Graphics::GetWordWrappedHeight(int theWidth, const string& (EBX), int theLineSpacing)`:
/// runs `WriteWordWrapped(Rect(0, 0, theWidth, 0), ..., theLineSpacing, -1)` on a Graphics
/// without a destination image (a 0x0 clip rect, so nothing is drawn) using this one's font.
pub fn FUN_004571f0(this: &Graphics, g: &mut G, param_2: i32, line: &[u8], param_3: i32) -> i32 {
    let mut gfx = Graphics::new(NULL, 0, 0, std::sync::Arc::new(std::sync::Mutex::new(Vec::new())));
    FUN_00455880(&mut gfx, this.s.mFont);
    FUN_00456e50(&mut gfx, g, &Rect::new(0, 0, param_2, 0), line, param_3, -1)
}

/// port: 004563f0 FUN_004563f0
/// `Graphics::DrawImageBox(const Rect& theDest, Image*)`: the whole image as the box source.
pub fn FUN_004563f0(this: &mut Graphics, g: &mut G, dest: &Rect, image: Ptr) {
    let src = Rect::new(0, 0, g.image(image).offset_0x20, g.image(image).offset_0x24);
    FUN_00456430(this, g, &src, image, dest);
}

/// port: 00456430 FUN_00456430
/// `Graphics::DrawImageBox(const Rect& theSrc, const Rect& theDest (ESI), Image*)`: a
/// 9-slice draw; the source is split into thirds, corners drawn once, edges and center tiled
/// inside clipped copies of this Graphics.
pub fn FUN_00456430(this: &mut Graphics, g: &mut G, src: &Rect, image: Ptr, dest: &Rect) {
    if !(0 < src.mWidth && 0 < src.mHeight) {
        return;
    }
    let cw = src.mWidth / 3;
    let ch = src.mHeight / 3;
    let sx = src.mX;
    let sy = src.mY;
    let mw = src.mWidth + cw * -2;
    let mh = src.mHeight + ch * -2;
    let right_sx = mw + sx + cw;
    let bottom_sy = sy + mh + ch;
    FUN_00455e40(this, g, image, dest.mX, dest.mY, &Rect::new(sx, sy, cw, ch));
    FUN_00455e40(this, g, image, (dest.mWidth - cw) + dest.mX, dest.mY, &Rect::new(right_sx, sy, cw, ch));
    FUN_00455e40(this, g, image, dest.mX, (dest.mY - ch) + dest.mHeight, &Rect::new(sx, bottom_sy, cw, ch));
    FUN_00455e40(this, g, image, (dest.mWidth - cw) + dest.mX, (dest.mY - ch) + dest.mHeight, &Rect::new(right_sx, bottom_sy, cw, ch));
    // Top and bottom edges.
    let mut g1 = FUN_00455800(this);
    let span = dest.mWidth - cw * 2;
    FUN_00456340(&mut g1, dest.mX + cw, dest.mY, span, dest.mHeight);
    let mut n = 0;
    let mut off = 0;
    while n < (span - 1 + mw) / mw {
        FUN_00455e40(&mut g1, g, image, dest.mX + off + cw, dest.mY, &Rect::new(sx + cw, sy, mw, ch));
        FUN_00455e40(&mut g1, g, image, dest.mX + off + cw, (dest.mY - ch) + dest.mHeight, &Rect::new(sx + cw, bottom_sy, mw, ch));
        n += 1;
        off += mw;
    }
    // Left and right edges.
    let mut g2 = FUN_00455800(this);
    let vspan = dest.mHeight + ch * -2;
    FUN_00456340(&mut g2, dest.mX, dest.mY + ch, dest.mWidth, vspan);
    let mut n = 0;
    let mut off = 0;
    while n < (vspan - 1 + mh) / mh {
        FUN_00455e40(&mut g2, g, image, dest.mX, dest.mY + off + ch, &Rect::new(sx, sy + ch, cw, mh));
        FUN_00455e40(&mut g2, g, image, (dest.mWidth - cw) + dest.mX, dest.mY + off + ch, &Rect::new(right_sx, sy + ch, cw, mh));
        off += mh;
        n += 1;
    }
    // Center.
    let mut g3 = FUN_00455800(this);
    FUN_00456340(&mut g3, dest.mX + cw, dest.mY + ch, span, vspan);
    let rows = (vspan - 1 + mh) / mh;
    let mut col = 0;
    let mut xo = 0;
    while col < (span - 1 + mw) / mw {
        let mut row = 0;
        let mut yo = 0;
        while row < rows {
            FUN_00455e40(&mut g3, g, image, dest.mX + xo + cw, dest.mY + yo + ch, &Rect::new(sx + cw, sy + ch, mw, mh));
            yo += mh;
            row += 1;
        }
        col += 1;
        xo += mw;
    }
}

/// port: 00456a00 FUN_00456a00
/// `Graphics::DrawImageAnim(Image*, int x, int y, int theTime)`.
pub fn FUN_00456a00(this: &mut Graphics, g: &mut G, image: Ptr, x: i32, y: i32, time: i32) {
    let cel = crate::sexy::image::FUN_004578a0(g, image, time);
    FUN_00456950(this, g, image, x, y, cel);
}
