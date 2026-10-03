//! `Sexy::Image` and its subclasses as far as game code reads them. Pixels live in the
//! renderer (`crate::render`); the game only ever reads the size and cel grid.

use crate::sexy::prelude::*;

/// `Sexy::Image` (`Image_data` at object offset 0x4).
#[derive(Clone, Debug, Default)]
pub struct Image {
    /// +0x04 `mDrawn`.
    pub offset_0x0: bool,
    /// +0x08 `mFilePath` (std::string, 0x08..0x24).
    pub field_0x4: String,
    /// +0x24 `mWidth`.
    pub offset_0x20: i32,
    /// +0x28 `mHeight`.
    pub offset_0x24: i32,
    /// +0x2c `mNumRows`.
    pub offset_0x28: i32,
    /// +0x30 `mNumCols`.
    pub offset_0x2c: i32,
    /// +0x34 `mAnimInfo` (owned; null = not animated).
    pub offset_0x30: Option<Box<AnimInfo>>,
    /// `MemoryImage::mBits` (0xAARRGGBB, `mWidth * mHeight`); empty for images without pixels.
    pub mBits: Vec<u32>,
}

impl Image {
    /// `Image::Image()` (@ 004572f0): empty, one cel.
    pub fn new() -> Image {
        Image { offset_0x28: 1, offset_0x2c: 1, ..Default::default() }
    }
}

/// `Sexy::AnimInfo` (0x30 bytes): an image's cel animation.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AnimInfo {
    /// +0x00 `mAnimType`: 0 none, 1 once, 2 ping-pong, 3 loop.
    pub offset_0x0: i32,
    /// +0x04 `mFrameDelay` (time units per cel).
    pub offset_0x4: i32,
    /// +0x08 `mNumCels`.
    pub offset_0x8: i32,
    /// +0x10 `mPerFrameDelay` (std::vector<int>).
    pub offset_0x10: Vec<i32>,
    /// +0x20 `mFrameMap` (std::vector<int>).
    pub offset_0x20: Vec<i32>,
    /// +0x2c `mTotalAnimTime`.
    pub offset_0x2c: i32,
}

/// port: 00457530 FUN_00457530
/// `AnimInfo::AnimInfo()`.
pub fn FUN_00457530() -> AnimInfo {
    AnimInfo { offset_0x0: 0, offset_0x4: 1, offset_0x8: 1, ..Default::default() }
}

/// port: 00457590 FUN_00457590
/// `AnimInfo::SetPerFrameDelay(int theFrame, int theTime)` (this in EAX, frame in EDI, time
/// in EBX): grows the vector to `theFrame + 1` when needed.
pub fn FUN_00457590(this: &mut AnimInfo, frame: usize, time: i32) {
    if this.offset_0x10.len() <= frame {
        this.offset_0x10.resize(frame + 1, 0);
    }
    this.offset_0x10[frame] = time;
}

/// port: 004575e0 FUN_004575e0
/// `AnimInfo::Compute(int theNumCels (EAX), int theBeginFrameTime, int theEndFrameTime)`.
pub fn FUN_004575e0(this: &mut AnimInfo, num_cels: i32, param_2: i32, param_3: i32) {
    this.offset_0x8 = num_cels;
    if num_cels < 1 {
        this.offset_0x8 = 1;
    }
    if this.offset_0x4 < 1 {
        this.offset_0x4 = 1;
    }
    if this.offset_0x0 == 2 && 1 < this.offset_0x8 {
        this.offset_0x20.resize((num_cels * 2 - 2) as usize, 0);
        let mut idx = 0usize;
        for i in 0..num_cels {
            this.offset_0x20[idx] = i;
            idx += 1;
        }
        let mut i = num_cels - 2;
        while 0 < i {
            this.offset_0x20[idx] = i;
            idx += 1;
            i -= 1;
        }
    }
    if !this.offset_0x20.is_empty() {
        this.offset_0x8 = this.offset_0x20.len() as i32;
    }
    if 0 < param_2 {
        FUN_00457590(this, 0, param_2);
    }
    if 0 < param_3 {
        let last = (this.offset_0x8 - 1) as usize;
        FUN_00457590(this, last, param_3);
    }
    if this.offset_0x10.is_empty() {
        this.offset_0x2c = this.offset_0x8 * this.offset_0x4;
    } else {
        this.offset_0x2c = 0;
        this.offset_0x10.resize(this.offset_0x8 as usize, 0);
        for i in 0..this.offset_0x8 as usize {
            if this.offset_0x10[i] < 1 {
                this.offset_0x10[i] = this.offset_0x4;
            }
            this.offset_0x2c += this.offset_0x10[i];
        }
    }
    if !this.offset_0x20.is_empty() {
        let n = this.offset_0x8 as usize;
        this.offset_0x20.resize(n, 0);
    }
}

/// port: 00457790 FUN_00457790
/// `AnimInfo::GetPerFrameCel(int theTime)` (time in EAX, this in ESI).
pub fn FUN_00457790(this: &AnimInfo, mut time: i32) -> i32 {
    let mut i = 0;
    while i < this.offset_0x8 {
        time -= this.offset_0x10[i as usize];
        if time < 0 {
            return i;
        }
        i += 1;
    }
    this.offset_0x8 - 1
}

/// port: 004577e0 FUN_004577e0
/// `AnimInfo::GetCel(int theTime)` (time in EAX). The `%`/`/` are signed, as compiled.
pub fn FUN_004577e0(this: &AnimInfo, time: i32) -> i32 {
    if this.offset_0x0 == 1 && this.offset_0x2c <= time {
        if !this.offset_0x20.is_empty() {
            return this.offset_0x20[this.offset_0x20.len() - 1];
        }
        return this.offset_0x8 - 1;
    }
    let frame = if this.offset_0x10.is_empty() {
        ((time % this.offset_0x2c) / this.offset_0x4) % this.offset_0x8
    } else {
        FUN_00457790(this, time % this.offset_0x2c)
    };
    if !this.offset_0x20.is_empty() {
        return this.offset_0x20[frame as usize];
    }
    frame
}

/// port: 004578a0 FUN_004578a0
/// `Image::GetAnimCel(int theTime)`: 0 for an image without animation.
pub fn FUN_004578a0(g: &mut G, this: Ptr, time: i32) -> i32 {
    match &g.image(this).offset_0x30 {
        None => 0,
        Some(a) => FUN_004577e0(a, time),
    }
}

/// port: 00457500 FUN_00457500
/// `Image::GetCelRect(int theCol, int theRow)`.
pub fn FUN_00457500(g: &mut G, this: Ptr, param_2: i32, param_3: i32) -> Rect {
    let h = crate::sexy::graphics::FUN_004574a0(g, this);
    let w = crate::sexy::graphics::FUN_004574b0(g, this);
    Rect::new(w * param_2, h * param_3, w, h)
}

/// port: 004578c0 FUN_004578c0
/// `Image::GetAnimCelRect(int theTime)`: cels run along the row when there are several
/// columns, else down the column.
pub fn FUN_004578c0(g: &mut G, this: Ptr, time: i32) -> Rect {
    let cel = FUN_004578a0(g, this, time);
    let w = crate::sexy::graphics::FUN_004574b0(g, this);
    let h = crate::sexy::graphics::FUN_004574a0(g, this);
    let img = g.image(this);
    if 1 < img.offset_0x2c {
        return Rect::new(w * cel, 0, w, img.offset_0x24);
    }
    Rect::new(0, h * cel, img.offset_0x20, h)
}

/// port: 004574c0 FUN_004574c0
/// `Image::GetCelRect(int theCel)`: cels row-major.
pub fn FUN_004574c0(g: &mut G, this: Ptr, param_1: i32) -> Rect {
    let h = crate::sexy::graphics::FUN_004574a0(g, this);
    let w = crate::sexy::graphics::FUN_004574b0(g, this);
    let cols = g.image(this).offset_0x2c;
    Rect::new((param_1 % cols) * w, (param_1 / cols) * h, w, h)
}
