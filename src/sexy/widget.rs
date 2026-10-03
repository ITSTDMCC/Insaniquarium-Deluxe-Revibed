//! `Sexy::Widget` (framework, 0x0046dca0..0x0046e880).

use crate::sexy::graphics::{FUN_00455800, FUN_00455d20, FUN_00456340, FUN_00457480, FUN_00457490};
use crate::sexy::object::{WidgetContainer_data, Widget_data};
use crate::sexy::prelude::*;
use crate::sexy::widget_manager as wmgr;

/// port: 0046dca0 Sexy::Widget::Widget
/// The base parts of a freshly constructed widget: visible, mouse-visible, everything else
/// cleared (`FUN_004728d0` zeroes `mMouseInsets`).
pub fn Widget() -> (WidgetContainer_data, Widget_data) {
    let wc = crate::sexy::widget_container::WidgetContainer();
    let w = Widget_data { offset_0x0: true, offset_0x1: true, ..Default::default() };
    (wc, w)
}

/// port: 0046dd50 Sexy::Widget::~Widget
/// Frees `mColors`, then the container part. (`FUN_004776d0` empties the color vector.)
pub fn dtor_Widget(g: &mut G, this: Ptr) {
    g.w(this).offset_0xc = Vec::new();
    crate::sexy::widget_container::dtor_WidgetContainer(g, this);
}

/// port: 0046dd90 FUN_0046dd90
/// `Widget::WidgetRemovedHelper()`: detaches this widget and its children from the manager.
pub fn FUN_0046dd90(g: &mut G, this: Ptr) {
    let wm = g.wc(this).offset_0xc;
    if wm == NULL {
        return;
    }
    let mut it = g.wc(this).offset_0x4.begin();
    while it != 0 {
        let child = g.wc(this).offset_0x4.get(it);
        FUN_0046dd90(g, child);
        it = g.wc(this).offset_0x4.next(it);
    }
    vcall!(g, wm, w.vfunction7, this, true);
    let wm = g.wc(this).offset_0xc;
    for info in wmgr::wm(g, wm).offset_0x5c.iter_mut() {
        if info.mPrevBaseModalWidget == this {
            info.mPrevBaseModalWidget = NULL;
        }
        if info.mPrevFocusWidget == this {
            info.mPrevFocusWidget = NULL;
        }
    }
    vcall!(g, this, w.vfunction22, wm);
    vcall!(g, this, w.vfunction19, this);
    g.wc(this).offset_0xc = NULL;
}

/// port: 0046de70 Sexy::Widget::vfunction61
/// `IsPointVisible(int, int)`: always true for plain widgets.
pub fn vfunction61(_g: &mut G, _this: Ptr, _x: i32, _y: i32) -> bool {
    true
}

/// port: 0046de80 Sexy::Widget::vfunction32
/// `SetVisible(bool)`.
pub fn vfunction32(g: &mut G, this: Ptr, visible: bool) {
    if g.w(this).offset_0x0 != visible {
        g.w(this).offset_0x0 = visible;
        if visible {
            vcall!(g, this, w.vfunction18);
        } else {
            vcall!(g, this, w.vfunction20);
        }
        let wm = g.wc(this).offset_0xc;
        if wm != NULL {
            wmgr::FUN_0046d820(g, wm);
        }
    }
}

/// port: 0046deb0 Sexy::Widget::vfunction44
/// `DrawOverlay(Graphics*, int thePriority)`: forwards to `DrawOverlay(g)` (slot 45).
pub fn vfunction44(g: &mut G, this: Ptr, gfx: &mut Graphics, _priority: i32) {
    vcall!(g, this, w.vfunction45, gfx);
}

/// port: 0046ded0 Sexy::Widget::vfunction34
/// `SetColors(int theColors[][3], int n)`.
pub fn vfunction34(g: &mut G, this: Ptr, colors: &[[i32; 3]]) {
    g.w(this).offset_0xc.clear();
    for (i, c) in colors.iter().enumerate() {
        let col = FUN_00433360(c[0], c[1], c[2]);
        vcall!(g, this, w.vfunction35, i as i32, col);
    }
    vcall!(g, this, w.vfunction18);
}

/// port: 0046df70 Sexy::Widget::vfunction33
/// `SetColors(int theColors[][4], int n)`.
pub fn vfunction33(g: &mut G, this: Ptr, colors: &[[i32; 4]]) {
    g.w(this).offset_0xc.clear();
    for (i, c) in colors.iter().enumerate() {
        let col = CRect(c[0], c[1], c[2], c[3]);
        vcall!(g, this, w.vfunction35, i as i32, col);
    }
    vcall!(g, this, w.vfunction18);
}

/// port: 0046e010 Sexy::Widget::vfunction35
/// `SetColor(int idx, const Color&)`: grows `mColors` with opaque black as needed.
pub fn vfunction35(g: &mut G, this: Ptr, idx: i32, color: Color) {
    let colors = &mut g.w(this).offset_0xc;
    if colors.len() as i32 <= idx {
        colors.resize(idx as usize + 1, FUN_00433300());
    }
    colors[idx as usize] = color;
    vcall!(g, this, w.vfunction18);
}

/// port: 0046e0c0 Sexy::Widget::vfunction37
/// `GetColor(int idx)`: opaque black (a function-local static) when out of range.
pub fn vfunction37(g: &mut G, this: Ptr, idx: i32) -> Color {
    let colors = &g.w(this).offset_0xc;
    if (idx as u32 as usize) < colors.len() && idx < colors.len() as i32 { colors[idx as usize] } else { FUN_00433300() }
}

/// port: 0046e130 Sexy::Widget::vfunction36
/// `GetColor(int idx, const Color& theDefault)`.
pub fn vfunction36(g: &mut G, this: Ptr, idx: i32, default: Color) -> Color {
    let colors = &g.w(this).offset_0xc;
    if idx < colors.len() as i32 { colors[idx as usize] } else { default }
}

/// port: 0046e1b0 Sexy::Widget::vfunction41
/// `Resize(int x, int y, int w, int h)`.
pub fn vfunction41(g: &mut G, this: Ptr, x: i32, y: i32, w: i32, h: i32) {
    let wc = g.wc(this);
    if wc.offset_0x2c != x || wc.offset_0x30 != y || wc.offset_0x34 != w || wc.offset_0x38 != h {
        vcall!(g, this, w.vfunction20);
        let wc = g.wc(this);
        wc.offset_0x30 = y;
        wc.offset_0x2c = x;
        wc.offset_0x34 = w;
        wc.offset_0x38 = h;
        vcall!(g, this, w.vfunction18);
        let wm = g.wc(this).offset_0xc;
        if wm != NULL {
            wmgr::FUN_0046d820(g, wm);
        }
    }
}

/// port: 0046e210 Sexy::Widget::vfunction40
/// `Resize(const Rect&)`.
pub fn vfunction40(g: &mut G, this: Ptr, rect: Rect) {
    vcall!(g, this, w.vfunction41, rect.mX, rect.mY, rect.mWidth, rect.mHeight);
}

/// port: 0046e240 Sexy::Widget::vfunction42
/// `Move(int x, int y)`.
pub fn vfunction42(g: &mut G, this: Ptr, x: i32, y: i32) {
    let (w, h) = (g.wc(this).offset_0x34, g.wc(this).offset_0x38);
    vcall!(g, this, w.vfunction41, x, y, w, h);
}

/// port: 0046e260 Sexy::Widget::vfunction43
/// `WantsFocus()`.
pub fn vfunction43(g: &mut G, this: Ptr) -> bool {
    g.w(this).offset_0x29
}

/// port: 0046e270 Sexy::Widget::vfunction38
/// `SetDisabled(bool)`.
pub fn vfunction38(g: &mut G, this: Ptr, disabled: bool) {
    if g.w(this).offset_0x2 != disabled {
        g.w(this).offset_0x2 = disabled;
        let wm = g.wc(this).offset_0xc;
        if disabled && wm != NULL {
            vcall!(g, wm, w.vfunction7, this, true);
        }
        vcall!(g, this, w.vfunction18);
        let wm = g.wc(this).offset_0xc;
        if !disabled && wm != NULL {
            let (mx, my) = (wmgr::wm(g, wm).offset_0x8c, wmgr::wm(g, wm).offset_0x90);
            if vcall!(g, this, w.vfunction69, mx, my) {
                wmgr::FUN_0046d770(g, wm, mx, my);
            }
        }
    }
}

/// port: 0046e2f0 Sexy::Widget::vfunction46
/// `GotFocus()`.
pub fn vfunction46(g: &mut G, this: Ptr) {
    g.w(this).offset_0x3 = true;
}

/// port: 0046e300 Sexy::Widget::vfunction47
/// `LostFocus()`.
pub fn vfunction47(g: &mut G, this: Ptr) {
    g.w(this).offset_0x3 = false;
}

/// port: 0046e320 Sexy::Widget::vfunction49
/// `KeyDown(KeyCode)`: Tab moves focus to `mTabPrev` with Shift held, else `mTabNext`.
pub fn vfunction49(g: &mut G, this: Ptr, key: i32) {
    if key == 9 {
        let wm = g.wc(this).offset_0xc;
        let target = if wmgr::wm(g, wm).offset_0xa0[0x10] { g.w(this).offset_0x2c } else { g.w(this).offset_0x30 };
        if target != NULL {
            vcall!(g, wm, w.vfunction9, target);
        }
    }
}

/// port: 0046e370 Sexy::Widget::vfunction39
/// `ShowFinger(bool)`: `mApp->SetCursor(on ? CURSOR_HAND : CURSOR_POINTER)`.
pub fn vfunction39(g: &mut G, this: Ptr, on: bool) {
    let wm = g.wc(this).offset_0xc;
    if wm != NULL {
        let app = wmgr::wm(g, wm).offset_0x8;
        crate::sexy::sexy_app_base::FUN_004891c0(g, app, on as i32);
    }
}

/// port: 0046e3a0 Sexy::Widget::vfunction55
/// `MouseDown(int x, int y, int clickCount)`: 3 = middle button, negative = right button.
pub fn vfunction55(g: &mut G, this: Ptr, x: i32, y: i32, clicks: i32) {
    if clicks == 3 {
        vcall!(g, this, w.vfunction54, x, y, 2, 1);
    } else if clicks >= 0 {
        vcall!(g, this, w.vfunction54, x, y, 0, clicks);
    } else {
        vcall!(g, this, w.vfunction54, x, y, 1, clicks.wrapping_neg());
    }
}

/// port: 0046e410 Sexy::Widget::vfunction57
/// `MouseUp(int x, int y, int clickCount)`.
pub fn vfunction57(g: &mut G, this: Ptr, x: i32, y: i32, clicks: i32) {
    vcall!(g, this, w.vfunction58, x, y);
    if clicks == 3 {
        vcall!(g, this, w.vfunction56, x, y, 2, 1);
    } else if clicks >= 0 {
        vcall!(g, this, w.vfunction56, x, y, 0, clicks);
    } else {
        vcall!(g, this, w.vfunction56, x, y, 1, clicks.wrapping_neg());
    }
}

/// port: 0046e6d0 Sexy::Widget::vfunction67
/// `GetNumDigits(int)`.
pub fn vfunction67(_g: &mut G, _this: Ptr, n: i32) -> i32 {
    let mut d = 10i32;
    let mut count = 1;
    if 9 < n {
        loop {
            d = d.wrapping_mul(10);
            count += 1;
            if !(d <= n) {
                break;
            }
        }
    }
    count
}

/// port: 0046e700 Sexy::Widget::vfunction68
/// `WriteNumberFromStrip(Graphics*, int n, int x, int y, Image* strip, int spacing)`: draws
/// each digit as a tenth of the strip, clipped through a temporary Graphics.
pub fn vfunction68(g: &mut G, this: Ptr, gfx: &mut Graphics, n: i32, x: i32, y: i32, image: Ptr, spacing: i32) {
    let mut d = 10i32;
    let mut count = 1;
    if 9 < n {
        loop {
            d = d.wrapping_mul(10);
            count += 1;
            if !(d <= n) {
                break;
            }
        }
    }
    if n == 0 {
        d = 10;
    }
    let digit_w = FUN_00457480(g, image) / 10;
    let mut offset = 0;
    let _ = this;
    while count != 0 {
        d /= 10;
        let mut tmp = FUN_00455800(gfx);
        let h = FUN_00457490(g, image);
        FUN_00456340(&mut tmp, offset + x, y, digit_w, h);
        FUN_00455d20(&mut tmp, g, image, offset - ((n / d) % 10) * digit_w + x, y);
        offset += digit_w + spacing;
        count -= 1;
    }
}

/// port: 0046e800 Sexy::Widget::vfunction69
/// `Contains(int x, int y)` in parent coordinates.
pub fn vfunction69(g: &mut G, this: Ptr, x: i32, y: i32) -> bool {
    let wc = g.wc(this);
    wc.offset_0x2c <= x && x < wc.offset_0x34 + wc.offset_0x2c && wc.offset_0x30 <= y && y < wc.offset_0x38 + wc.offset_0x30
}

/// port: 0046e840 Sexy::Widget::vfunction70
/// `GetInsetRect()`: the rect shrunk by `mMouseInsets`.
pub fn vfunction70(g: &mut G, this: Ptr) -> Rect {
    let ins = g.w(this).field_0x18;
    let wc = g.wc(this);
    Rect::new(wc.offset_0x2c + ins[0], wc.offset_0x30 + ins[1], wc.offset_0x34 - ins[2] - ins[0], wc.offset_0x38 - ins[3] - ins[1])
}

/// port: 0046e620 Sexy::Widget::vfunction64
/// `WriteString(Graphics*, const string&, int x, int y, int theWidth, int theJustification,
/// bool drawString, int theOffset, int theLength)`: with colored-string parsing on.
pub fn vfunction64(_g: &mut G, _this: Ptr, gfx: &mut Graphics, s: &[u8], x: i32, y: i32, width: i32, justify: i32, draw: bool, offset: i32, length: i32) -> i32 {
    let old = gfx.s.mWriteColoredString;
    gfx.s.mWriteColoredString = true;
    let r = crate::sexy::graphics::FUN_00456a30(gfx, _g, s, x, y, width, justify, draw, offset, length, 0xffff_ffff);
    gfx.s.mWriteColoredString = old;
    r
}

/// port: 0046e670 Sexy::Widget::vfunction65
/// `WriteWordWrapped(Graphics*, const Rect&, const string&, int theLineSpacing, int
/// theJustification)`: with colored-string parsing on.
pub fn vfunction65(g: &mut G, _this: Ptr, gfx: &mut Graphics, rect: Rect, s: &[u8], spacing: i32, justify: i32) -> i32 {
    let old = gfx.s.mWriteColoredString;
    gfx.s.mWriteColoredString = true;
    let r = crate::sexy::graphics::FUN_00456e50(gfx, g, &rect, s, spacing, justify);
    gfx.s.mWriteColoredString = old;
    r
}

/// port: 0046e6b0 Sexy::Widget::vfunction66
/// `GetWordWrappedHeight(Graphics*, int theWidth, const string&, int theLineSpacing)`.
pub fn vfunction66(g: &mut G, _this: Ptr, gfx: &mut Graphics, width: i32, s: &[u8], spacing: i32) -> i32 {
    crate::sexy::graphics::FUN_004571f0(gfx, g, width, s, spacing)
}

/// port: 0046e480 Sexy::Widget::vfunction63
/// `Rect WriteCenteredLine(Graphics*, int anOffset, const string&)`: draws the line centered
/// in the widget's width with its baseline at `anOffset`; returns the text's rect.
pub fn vfunction63(g: &mut G, this: Ptr, gfx: &mut Graphics, offset: i32, line: &[u8]) -> Rect {
    let f = crate::sexy::graphics::FUN_00455870(gfx);
    let w = crate::sexy::image_font::string_width(g, f, line);
    let x = (g.wc(this).offset_0x34 - w) / 2;
    crate::sexy::graphics::FUN_00455cf0(gfx, g, line, x, offset);
    let h = crate::sexy::image_font::get_height(g, f);
    let asc = crate::sexy::image_font::get_ascent(g, f);
    Rect::new(x, offset - asc, w, h)
}

/// port: 0046e880 FUN_0046e880
/// `Widget::Layout(int theLayoutFlags, Widget* theRelativeWidget, int theLeftPad,
/// int theTopPad, int theWidthPad, int theHeightPad)`: positions and sizes this widget
/// relative to another (its parent counts as the origin), applying the flag bits from low
/// to high, then `Resize`.
pub fn FUN_0046e880(g: &mut G, this: Ptr, param_1: u32, param_2: Ptr, param_3: i32, param_4: i32, param_5: i32, param_6: i32) {
    let rel = g.wc(param_2).clone();
    let (mut rx, mut ry) = (rel.offset_0x2c, rel.offset_0x30);
    if param_2 == g.wc(this).offset_0x10 {
        rx = 0;
        ry = 0;
    }
    let (rw, rh) = (rel.offset_0x34, rel.offset_0x38);
    let me = g.wc(this);
    let (mut x, mut y, mut w, mut h) = (me.offset_0x2c, me.offset_0x30, me.offset_0x34, me.offset_0x38);
    let (rr, rb) = (rw + rx, rh + ry);
    let mut bit: u32 = 1;
    loop {
        if param_1 & bit != 0 {
            match bit {
                0x1 => w = param_5 + rw,
                0x2 => h = rh + param_6,
                0x10 => x = param_3,
                0x20 => y = param_4,
                0x40 => w = param_5,
                0x100 => y = ry - h + param_4,
                0x200 => y = rb + param_4,
                0x400 => x = rr + param_3,
                0x800 => x = (rx - w) + param_3,
                0x1000 => x = rx + param_3,
                0x2000 => x = (rr - w) + param_3,
                0x4000 => y = ry + param_4,
                0x8000 => y = rb - h + param_4,
                0x10000 => w = (rr - x) + param_5,
                0x20000 => w = (rx - x) + param_5,
                0x40000 => h = (ry - y) + param_6,
                0x80000 => h = (rb - y) + param_6,
                0x100000 => x = (rw - w) / 2 + rx + param_3,
                0x200000 => y = (rh - h) / 2 + ry + param_4,
                _ => {}
            }
        }
        bit = bit.wrapping_mul(2);
        if 0x3fffff < bit as i32 {
            vcall!(g, this, w.vfunction41, x, y, w, h);
            return;
        }
    }
}
