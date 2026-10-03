//! Small value types of the PopCap framework (`Sexy::Color`, `Sexy::Rect`, `Sexy::Point`).

/// A pointer of the original program: the id of an object in [`crate::sexy::G`]'s arena.
/// `0` is the null pointer. Ids are never reused, so a stale pointer fails loudly instead
/// of aliasing a newer object.
pub type Ptr = u32;
pub const NULL: Ptr = 0;

/// `Sexy::Color`: four ints, red/green/blue/alpha in 0..=255.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Color {
    pub mRed: i32,
    pub mGreen: i32,
    pub mBlue: i32,
    pub mAlpha: i32,
}

impl Color {
    pub const WHITE: Color = Color { mRed: 255, mGreen: 255, mBlue: 255, mAlpha: 255 };
    /// `Color::Black` (`DAT_005ec488`).
    pub const BLACK: Color = Color { mRed: 0, mGreen: 0, mBlue: 0, mAlpha: 255 };
}

/// port: 00433300 FUN_00433300
/// `Color::Color()`: opaque black.
pub fn FUN_00433300() -> Color {
    Color { mRed: 0, mGreen: 0, mBlue: 0, mAlpha: 0xff }
}

/// port: 00433320 FUN_00433320
/// `Color::Color(int theColor)` from 0xAARRGGBB; an alpha byte of 0 means opaque.
pub fn FUN_00433320(param_1: u32) -> Color {
    let a = ((param_1 as i32) >> 0x18) & 0xff;
    Color {
        mRed: ((param_1 as i32) >> 0x10) & 0xff,
        mGreen: ((param_1 >> 8) & 0xff) as i32,
        mBlue: (param_1 & 0xff) as i32,
        mAlpha: if a == 0 { 0xff } else { a },
    }
}

/// port: 00433360 FUN_00433360
/// `Color::Color(int r, int g, int b)`, opaque.
pub fn FUN_00433360(param_1: i32, param_2: i32, param_3: i32) -> Color {
    Color { mRed: param_1, mGreen: param_2, mBlue: param_3, mAlpha: 0xff }
}

/// port: 00433380 CRect::CRect
/// `Color::Color(int r, int g, int b, int a)`. The decompiler matched this body to MFC's
/// `CRect::CRect(int,int,int,int)` (identical code, folded by the linker); every caller
/// in the game builds a `Sexy::Color` with it.
pub fn CRect(param_1: i32, param_2: i32, param_3: i32, param_4: i32) -> Color {
    Color { mRed: param_1, mGreen: param_2, mBlue: param_3, mAlpha: param_4 }
}

/// `Sexy::TRect<int>`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Rect {
    pub mX: i32,
    pub mY: i32,
    pub mWidth: i32,
    pub mHeight: i32,
}

impl Rect {
    pub const fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect { mX: x, mY: y, mWidth: w, mHeight: h }
    }

    /// `TRect::Intersection`, as the framework's header defines it.
    pub fn intersection(&self, other: &Rect) -> Rect {
        let x1 = self.mX.max(other.mX);
        let x2 = (self.mX + self.mWidth).min(other.mX + other.mWidth);
        let y1 = self.mY.max(other.mY);
        let y2 = (self.mY + self.mHeight).min(other.mY + other.mHeight);
        if x2 - x1 < 0 || y2 - y1 < 0 {
            Rect::new(0, 0, 0, 0)
        } else {
            Rect::new(x1, y1, x2 - x1, y2 - y1)
        }
    }

    /// `TRect::Contains(x, y)`.
    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.mX && x < self.mX + self.mWidth && y >= self.mY && y < self.mY + self.mHeight
    }
}

/// `Sexy::TPoint<int>`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Point {
    pub mX: i32,
    pub mY: i32,
}

/// `Sexy::ModalFlags`: the widget-flag masks threaded through `UpdateAll`/`DrawAll`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ModalFlags {
    pub mOverFlags: u32,
    pub mUnderFlags: u32,
    pub mIsOver: bool,
}

impl ModalFlags {
    /// `ModalFlags::GetFlags`.
    pub fn get_flags(&self) -> u32 {
        if self.mIsOver { self.mOverFlags } else { self.mUnderFlags }
    }
}

/// Widget flag bits (`WIDGETFLAGS_*`).
pub const WIDGETFLAGS_UPDATE: u32 = 1;
pub const WIDGETFLAGS_MARK_DIRTY: u32 = 2;
pub const WIDGETFLAGS_DRAW: u32 = 4;
pub const WIDGETFLAGS_CLIP: u32 = 8;
pub const WIDGETFLAGS_ALLOW_MOUSE: u32 = 16;
pub const WIDGETFLAGS_ALLOW_FOCUS: u32 = 32;

/// port: 004333e0 FUN_004333e0
/// `Color::ToInt()`: 0xAARRGGBB.
pub fn FUN_004333e0(this: &Color) -> u32 {
    (((((this.mAlpha as u32) << 8 | this.mRed as u32) << 8) | this.mGreen as u32) << 8) | this.mBlue as u32
}
