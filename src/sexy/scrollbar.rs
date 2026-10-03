//! `Sexy::ScrollbuttonWidget` and `Sexy::ScrollbarWidget` (framework, 0x0046faa0..
//! 0x00470b40): a gray scrollbar with an arrow button at each end and a thumb sized to the
//! page; reports `ScrollPosition(id, value)` to its ScrollListener.
//!
//! `ScrollbuttonWidget_data` starts at object offset 0x120 (after ButtonWidget);
//! `ScrollbarWidget_data` at 0x8c (after the ButtonListener vftable at 0x88). The scrollbar
//! object is 0xe0 bytes, past the database's 0x94 (fields `ext_0xNN`).

use crate::sexy::button_widget::{BtnSub, ButtonExt};
use crate::sexy::graphics::{FUN_00455890, FUN_00455920, FUN_004559b0};
use crate::sexy::prelude::*;
use crate::sexy::types::FUN_00433360;

/// `ScrollbuttonWidget_data` (object offset 0x120).
#[derive(Debug, Clone, Default)]
pub struct ScrollbuttonWidget_data {
    /// +0x120 `mHorizontal` (arrows point sideways).
    pub offset_0x0: bool,
    /// +0x124 `mType` (0: by id; 1 up, 2 down, 3 left, 4 right).
    pub offset_0x4: i32,
}

/// `ScrollbarWidget_data` (object offset 0x8c) plus the fields past the database size.
#[derive(Debug, Clone, Default)]
pub struct ScrollbarWidget_data {
    /// +0x8c `mUpButton`.
    pub offset_0x0: Ptr,
    /// +0x90 `mDownButton`.
    pub offset_0x4: Ptr,
    /// +0x94 `mInvisIfNoScroll`.
    pub ext_0x94: bool,
    /// +0x98 `mId`.
    pub ext_0x98: i32,
    /// +0xa0 `mValue`.
    pub ext_0xa0: f64,
    /// +0xa8 `mMaxValue`.
    pub ext_0xa8: f64,
    /// +0xb0 `mPageSize`.
    pub ext_0xb0: f64,
    /// +0xb8 `mHorizontal`.
    pub ext_0xb8: bool,
    /// +0xb9 `mPressedOnThumb`.
    pub ext_0xb9: bool,
    /// +0xbc `mMouseDownThumbPos`.
    pub ext_0xbc: i32,
    /// +0xc0 `mMouseDownX`.
    pub ext_0xc0: i32,
    /// +0xc4 `mMouseDownY`.
    pub ext_0xc4: i32,
    /// +0xc8 `mUpdateMode` (0 none, 1 paging up, 2 paging down).
    pub ext_0xc8: i32,
    /// +0xcc `mUpdateAcc`.
    pub ext_0xcc: i32,
    /// +0xd0 `mButtonAcc`.
    pub ext_0xd0: i32,
    /// +0xd4 `mLastMouseX`.
    pub ext_0xd4: i32,
    /// +0xd8 `mLastMouseY`.
    pub ext_0xd8: i32,
    /// +0xdc `mScrollListener` (the object whose ScrollListener vftable is told).
    pub ext_0xdc: Ptr,
}

impl G {
    pub fn scrollbar(&mut self, p: Ptr) -> &mut ScrollbarWidget_data {
        match &mut self.widget(p).ext {
            WExt::Scrollbar(d) => d,
            e => panic!("{p} is not a ScrollbarWidget: {e:?}"),
        }
    }
    pub fn scrollbutton(&mut self, p: Ptr) -> &mut ScrollbuttonWidget_data {
        match &mut self.widget(p).ext {
            WExt::Button(e) => match &mut e.sub {
                BtnSub::Scrollbutton(d) => d,
                s => panic!("{p} is not a ScrollbuttonWidget: {s:?}"),
            },
            e => panic!("{p} is not a ButtonWidget: {e:?}"),
        }
    }
}

/// port: 0046faa0 Sexy::ScrollbuttonWidget::ScrollbuttonWidget
/// `ScrollbuttonWidget(int theId, ButtonListener*)` (id in ECX, listener in EAX).
pub fn ScrollbuttonWidget(g: &mut G, param_1: i32, param_2: Ptr) -> Ptr {
    let (wc, w, b) = crate::sexy::button_widget::ButtonWidget(param_1, param_2);
    let d = ScrollbuttonWidget_data { offset_0x0: false, offset_0x4: 0 };
    g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__ScrollbuttonWidget_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::Button(Box::new(ButtonExt { b, sub: BtnSub::Scrollbutton(d) })) }),
    })
}

/// port: 0046fae0 Sexy::ScrollbuttonWidget::~ScrollbuttonWidget
pub fn dtor_ScrollbuttonWidget(g: &mut G, this: Ptr) {
    crate::sexy::button_widget::dtor_ButtonWidget(g, this);
}

/// port: 0046fac0 Sexy::ScrollbuttonWidget::deleting_destructor
pub fn deleting_destructor(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_ScrollbuttonWidget(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 0046faf0 Sexy::ScrollbuttonWidget::vfunction27
/// `Draw(Graphics*)`: a gray bevelled box (sunken and shifted a pixel while pressed) with a
/// black arrow (gray when disabled): up/down by id or type, or left/right when horizontal.
pub fn vfunction27(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    let (w, h) = (g.wc(this).offset_0x34, g.wc(this).offset_0x38);
    let wd = g.w(this).clone();
    let mut inset = 0;
    FUN_00455890(gfx, FUN_00433360(0xd4, 0xd4, 0xd4));
    FUN_00455920(gfx, 0, 0, w, h);
    if !wd.offset_0x4 || !wd.offset_0x5 || wd.offset_0x2 {
        FUN_00455890(gfx, FUN_00433360(0xff, 0xff, 0xff));
        FUN_00455920(gfx, 1, 1, w - 2, 1);
        FUN_00455920(gfx, 1, 1, 1, h - 2);
        FUN_00455890(gfx, Color::BLACK);
        FUN_00455920(gfx, 0, h - 1, w, 1);
        FUN_00455920(gfx, w - 1, 0, 1, h);
        FUN_00455890(gfx, FUN_00433360(0x84, 0x84, 0x84));
        FUN_00455920(gfx, 1, h - 2, w - 2, 1);
        FUN_00455920(gfx, w - 2, 1, 1, h - 2);
    } else {
        inset = 1;
        FUN_00455890(gfx, FUN_00433360(0x84, 0x84, 0x84));
        FUN_004559b0(gfx, 0, 0, w - 1, h - 1);
    }
    FUN_00455890(gfx, if !wd.offset_0x2 { Color::BLACK } else { FUN_00433360(0x84, 0x84, 0x84) });
    let d = g.scrollbutton(this).clone();
    let id = g.btn(this).offset_0x0;
    if !d.offset_0x0 && d.offset_0x4 != 3 && d.offset_0x4 != 4 {
        let mut i = 0;
        let mut len = 1;
        while len < 9 {
            let (x, y) = if id == 0 || d.offset_0x4 == 1 {
                (w / 2 - i, (h - 4) / 2 + i)
            } else {
                (w / 2 - i, ((h - 4) / 2 - i) + 3)
            };
            FUN_00455920(gfx, x - 1 + inset, y + inset, len, 1);
            i += 1;
            len += 2;
        }
        return;
    }
    let mut i = 0;
    let mut len = 1;
    while len < 9 {
        let (x, y) = if id == 0 || d.offset_0x4 == 3 {
            ((w - 4) / 2 + i, h / 2 - i)
        } else {
            (((w - 4) / 2 - i) + 3, h / 2 - i)
        };
        FUN_00455920(gfx, x + inset, y - 1 + inset, 1, len);
        i += 1;
        len += 2;
    }
}

/// port: 0046fdf0 Sexy::ScrollbarWidget::ScrollbarWidget
/// `ScrollbarWidget(int theId, ScrollListener*)`: disabled, both arrow buttons (disabled)
/// added, nothing to scroll yet.
pub fn ScrollbarWidget(g: &mut G, param_1: i32, param_2: Ptr) -> Ptr {
    let (wc, w) = crate::sexy::widget::Widget();
    let d = ScrollbarWidget_data { ext_0xdc: param_2, ext_0x98: param_1, ..Default::default() };
    let this = g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__ScrollbarWidget_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::Scrollbar(Box::new(d)) }),
    });
    crate::sexy::widget::vfunction38(g, this, true);
    let up = ScrollbuttonWidget(g, 0, this);
    g.scrollbar(this).offset_0x0 = up;
    vcall!(g, up, w.vfunction38, true);
    let down = ScrollbuttonWidget(g, 1, this);
    g.scrollbar(this).offset_0x4 = down;
    vcall!(g, down, w.vfunction38, true);
    crate::sexy::widget_container::vfunction4(g, this, up);
    crate::sexy::widget_container::vfunction4(g, this, down);
    this
}

/// port: 0046ff80 Sexy::ScrollbarWidget::~ScrollbarWidget
/// Removes and deletes the arrow buttons.
pub fn dtor_ScrollbarWidget(g: &mut G, this: Ptr) {
    for b in [g.scrollbar(this).offset_0x0, g.scrollbar(this).offset_0x4] {
        if b != NULL {
            crate::sexy::widget_container::vfunction5(g, this, b);
            vcall!(g, b, w.vfunction1, 1);
        }
    }
    crate::sexy::widget::dtor_Widget(g, this);
}

/// port: 0046ff60 Sexy::ScrollbarWidget::deleting_destructor_for_Widget
pub fn deleting_destructor_for_Widget(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_ScrollbarWidget(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 00470020 Sexy::ScrollbarWidget::vfunction71_for_Widget
/// `SetInvisIfNoScroll(bool)`: when set, hides the bar and its buttons (until there is
/// something to scroll).
pub fn vfunction71_for_Widget(g: &mut G, this: Ptr, param_1: bool) {
    g.scrollbar(this).ext_0x94 = param_1;
    if param_1 {
        vcall!(g, this, w.vfunction32, false);
        let d = g.scrollbar(this).clone();
        vcall!(g, d.offset_0x4, w.vfunction32, false);
        vcall!(g, d.offset_0x0, w.vfunction32, false);
    }
}

/// port: 00470070 Sexy::ScrollbarWidget::vfunction75_for_Widget
/// `SetHorizontal(bool)`, for the buttons too.
pub fn vfunction75_for_Widget(g: &mut G, this: Ptr, param_1: bool) {
    let d = g.scrollbar(this).clone();
    g.scrollbar(this).ext_0xb8 = param_1;
    g.scrollbutton(d.offset_0x4).offset_0x0 = param_1;
    g.scrollbutton(d.offset_0x0).offset_0x0 = param_1;
}

/// port: 004700a0 Sexy::ScrollbarWidget::vfunction76_for_Widget
/// `ResizeScrollbar(x, y, w, h)`: square buttons at the two ends.
pub fn vfunction76_for_Widget(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32, param_4: i32) {
    vcall!(g, this, w.vfunction41, param_1, param_2, param_3, param_4);
    let d = g.scrollbar(this).clone();
    if !d.ext_0xb8 {
        vcall!(g, d.offset_0x0, w.vfunction41, 0, 0, param_3, param_3);
        vcall!(g, d.offset_0x4, w.vfunction41, 0, param_4 - param_3, param_3, param_3);
    } else {
        vcall!(g, d.offset_0x0, w.vfunction41, 0, 0, param_4, param_4);
        vcall!(g, d.offset_0x4, w.vfunction41, param_3 - param_4, 0, param_4, param_4);
    }
}

/// port: 00470120 Sexy::ScrollbarWidget::vfunction72_for_Widget
/// `SetMaxValue(double)`.
pub fn vfunction72_for_Widget(g: &mut G, this: Ptr, param_1: f64) {
    g.scrollbar(this).ext_0xa8 = param_1;
    vcall!(g, this, scroll.vfunction83);
    vcall!(g, this, w.vfunction18);
}

/// port: 00470150 Sexy::ScrollbarWidget::vfunction73_for_Widget
/// `SetPageSize(double)`.
pub fn vfunction73_for_Widget(g: &mut G, this: Ptr, param_1: f64) {
    g.scrollbar(this).ext_0xb0 = param_1;
    vcall!(g, this, scroll.vfunction83);
    vcall!(g, this, w.vfunction18);
}

/// port: 00470180 Sexy::ScrollbarWidget::vfunction74_for_Widget
/// `SetValue(double)`: clamped, then always reported.
pub fn vfunction74_for_Widget(g: &mut G, this: Ptr, param_1: f64) {
    g.scrollbar(this).ext_0xa0 = param_1;
    vcall!(g, this, scroll.vfunction83);
    let d = g.scrollbar(this).clone();
    vcall!(g, d.ext_0xdc, scl.vfunction1, d.ext_0x98, d.ext_0xa0);
    vcall!(g, this, w.vfunction18);
}

/// port: 004701d0 Sexy::ScrollbarWidget::vfunction77_for_Widget
/// `AtBottom()`: within one unit of the end.
pub fn vfunction77_for_Widget(g: &mut G, this: Ptr) -> bool {
    let d = g.scrollbar(this);
    (d.ext_0xa8 - d.ext_0xb0) - d.ext_0xa0 <= 1.0
}

/// port: 00470200 Sexy::ScrollbarWidget::vfunction78_for_Widget
/// `GoToBottom()`.
pub fn vfunction78_for_Widget(g: &mut G, this: Ptr) {
    let d = g.scrollbar(this);
    d.ext_0xa0 = d.ext_0xa8 - d.ext_0xb0;
    vcall!(g, this, scroll.vfunction83);
    let v = g.scrollbar(this).ext_0xa0;
    vcall!(g, this, scroll.vfunction74, v);
}

/// port: 00470240 Sexy::ScrollbarWidget::vfunction79_for_Widget
/// `DrawThumb(g, x, y, w, h)`: a raised gray box.
pub fn vfunction79_for_Widget(_g: &mut G, _this: Ptr, gfx: &mut Graphics, x: i32, y: i32, w: i32, h: i32) {
    FUN_00455890(gfx, FUN_00433360(0xd4, 0xd4, 0xd4));
    FUN_00455920(gfx, x, y, w, h);
    FUN_00455890(gfx, FUN_00433360(0xff, 0xff, 0xff));
    FUN_00455920(gfx, x + 1, y + 1, w - 2, 1);
    FUN_00455920(gfx, x + 1, y + 1, 1, h - 2);
    FUN_00455890(gfx, Color::BLACK);
    FUN_00455920(gfx, x, y - 1 + h, w, 1);
    FUN_00455920(gfx, x - 1 + w, y, 1, h);
    FUN_00455890(gfx, FUN_00433360(0x84, 0x84, 0x84));
    FUN_00455920(gfx, x + 1, y - 2 + h, w - 2, 1);
    FUN_00455920(gfx, x - 2 + w, y + 1, 1, h - 2);
}

/// port: 00470380 Sexy::ScrollbarWidget::vfunction80_for_Widget
/// `GetTrackSize()`: the length between the buttons.
pub fn vfunction80_for_Widget(g: &mut G, this: Ptr) -> i32 {
    let d = g.scrollbar(this).clone();
    let bw = g.wc(d.offset_0x0).offset_0x34;
    if d.ext_0xb8 {
        return g.wc(this).offset_0x34 + bw * -2;
    }
    g.wc(this).offset_0x38 + bw * -2
}

/// port: 004703a0 Sexy::ScrollbarWidget::vfunction81_for_Widget
/// `GetThumbSize()`: the track share of the page, at least 8 (0 when it all fits).
pub fn vfunction81_for_Widget(g: &mut G, this: Ptr) -> i32 {
    let d = g.scrollbar(this).clone();
    if d.ext_0xa8 < d.ext_0xb0 {
        return 0;
    }
    let track = vcall!(g, this, scroll.vfunction80);
    let n = crate::sexy::crt::ftol(track as f64 * d.ext_0xb0 / d.ext_0xa8 + 0.5) as i32;
    if n < 8 {
        8
    } else {
        n
    }
}

/// port: 00470400 Sexy::ScrollbarWidget::vfunction82_for_Widget
/// `GetThumbPosition()`.
pub fn vfunction82_for_Widget(g: &mut G, this: Ptr) -> i32 {
    let d = g.scrollbar(this).clone();
    let bw = g.wc(d.offset_0x0).offset_0x34;
    if d.ext_0xa8 < d.ext_0xb0 {
        return bw;
    }
    let thumb = vcall!(g, this, scroll.vfunction81);
    let track = vcall!(g, this, scroll.vfunction80);
    let n = crate::sexy::crt::ftol(((track - thumb) as f64 * d.ext_0xa0) / (d.ext_0xa8 - d.ext_0xb0) + 0.5) as i32;
    let bw = g.wc(d.offset_0x0).offset_0x34;
    n + bw
}

/// port: 00470480 Sexy::ScrollbarWidget::vfunction27_for_Widget
/// `Draw(Graphics*)`: the track (darker on the side being paged) and the thumb.
pub fn vfunction27_for_Widget(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    let size = vcall!(g, this, scroll.vfunction81);
    let pos = vcall!(g, this, scroll.vfunction82);
    let d = g.scrollbar(this).clone();
    let (w, h) = (g.wc(this).offset_0x34, g.wc(this).offset_0x38);
    let shade = |on: bool| if on { FUN_00433360(0x30, 0x30, 0x30) } else { FUN_00433360(0xe8, 0xe8, 0xe8) };
    if d.ext_0xb8 {
        FUN_00455890(gfx, shade(d.ext_0xc8 == 1));
        FUN_00455920(gfx, 0, 0, pos, h);
        if 0 < size {
            vcall!(g, this, scroll.vfunction79, gfx, pos, 0, size, h);
        }
        FUN_00455890(gfx, shade(d.ext_0xc8 == 2));
        FUN_00455920(gfx, pos + size, 0, (w - pos) - size, h);
        return;
    }
    FUN_00455890(gfx, shade(d.ext_0xc8 == 1));
    FUN_00455920(gfx, 0, 0, w, pos);
    if 0 < size {
        vcall!(g, this, scroll.vfunction79, gfx, 0, pos, w, size);
    }
    FUN_00455890(gfx, shade(d.ext_0xc8 == 2));
    FUN_00455920(gfx, 0, pos + size, w, (h - pos) - size);
}

/// port: 00470620 Sexy::ScrollbarWidget::vfunction83_for_Widget
/// `ClampValue()`: keeps the value within the scrollable range; enabled (and, when so set,
/// visible) only with something to scroll; reports a changed value.
pub fn vfunction83_for_Widget(g: &mut G, this: Ptr) {
    let old = g.scrollbar(this).ext_0xa0;
    {
        let d = g.scrollbar(this);
        let top = d.ext_0xa8 - d.ext_0xb0;
        if top < d.ext_0xa0 {
            d.ext_0xa0 = top;
        }
        if 0.0 > d.ext_0xa0 {
            d.ext_0xa0 = 0.0;
        }
    }
    let d = g.scrollbar(this).clone();
    let can_scroll = d.ext_0xa8 > d.ext_0xb0;
    vcall!(g, this, w.vfunction38, !can_scroll);
    vcall!(g, d.offset_0x0, w.vfunction38, !can_scroll);
    vcall!(g, d.offset_0x4, w.vfunction38, !can_scroll);
    if d.ext_0x94 {
        vcall!(g, this, w.vfunction32, can_scroll);
        vcall!(g, d.offset_0x4, w.vfunction32, can_scroll);
        vcall!(g, d.offset_0x0, w.vfunction32, can_scroll);
    }
    let v = g.scrollbar(this).ext_0xa0;
    if old != v {
        vcall!(g, d.ext_0xdc, scl.vfunction1, d.ext_0x98, v);
    }
}

/// port: 00470730 Sexy::ScrollbarWidget::vfunction84_for_Widget
/// `SetThumbPosition(int)`: the value for a thumb at that pixel.
pub fn vfunction84_for_Widget(g: &mut G, this: Ptr, param_1: i32) {
    let d = g.scrollbar(this).clone();
    let bw = g.wc(d.offset_0x0).offset_0x34;
    let num = (param_1 - bw) as f64 * (d.ext_0xa8 - d.ext_0xb0);
    let thumb = vcall!(g, this, scroll.vfunction81);
    let track = vcall!(g, this, scroll.vfunction80);
    vcall!(g, this, scroll.vfunction74, num / (track - thumb) as f64);
}

/// port: 004707a0 Sexy::ScrollbarWidget::vfunction2_for_ButtonListener
/// `ButtonPress(int id)`: an arrow steps one unit.
pub fn vfunction2_for_ButtonListener(g: &mut G, this: Ptr, param_1: i32) {
    let v = g.scrollbar(this).ext_0xa0;
    g.scrollbar(this).ext_0xd0 = 0;
    if param_1 == 0 {
        vcall!(g, this, scroll.vfunction74, v - 1.0);
        return;
    }
    vcall!(g, this, scroll.vfunction74, v + 1.0);
}

/// port: 004707f0 Sexy::ScrollbarWidget::vfunction4_for_ButtonListener
/// `ButtonDownTick(int id)`: a held arrow repeats every update after 25.
pub fn vfunction4_for_ButtonListener(g: &mut G, this: Ptr, param_1: i32) {
    g.scrollbar(this).ext_0xd0 += 1;
    let d = g.scrollbar(this).clone();
    if d.ext_0xd0 < 0x19 {
        return;
    }
    let v = if param_1 == 0 { d.ext_0xa0 - 1.0 } else { d.ext_0xa0 + 1.0 };
    vcall!(g, this, scroll.vfunction74, v);
    g.scrollbar(this).ext_0xd0 = 0x18;
}

/// port: 00470850 Sexy::ScrollbarWidget::vfunction23_for_Widget
/// `Update()`: while the track is held off the thumb, pages toward the mouse every 5
/// updates (after 25), stopping once the thumb reaches it.
pub fn vfunction23_for_Widget(g: &mut G, this: Ptr) {
    crate::sexy::widget_container::vfunction23(g, this);
    let d = g.scrollbar(this).clone();
    let (want, dir) = match d.ext_0xc8 {
        1 => (-1, -1.0),
        2 => (1, 1.0),
        _ => return,
    };
    let c = vcall!(g, this, scroll.vfunction85, d.ext_0xd4, d.ext_0xd8);
    if c != want {
        g.scrollbar(this).ext_0xc8 = 0;
        vcall!(g, this, w.vfunction18);
        return;
    }
    g.scrollbar(this).ext_0xcc += 1;
    let d = g.scrollbar(this).clone();
    if 0x18 < d.ext_0xcc {
        vcall!(g, this, scroll.vfunction74, d.ext_0xa0 + dir * d.ext_0xb0);
        g.scrollbar(this).ext_0xcc = 0x14;
    }
}

/// port: 00470950 Sexy::ScrollbarWidget::vfunction85_for_Widget
/// `ThumbCompare(x, y)`: -1 before the thumb, 0 on it, 1 after it.
pub fn vfunction85_for_Widget(g: &mut G, this: Ptr, param_1: i32, param_2: i32) -> i32 {
    let p = if g.scrollbar(this).ext_0xb8 { param_1 } else { param_2 };
    let pos = vcall!(g, this, scroll.vfunction82);
    if p < pos {
        return -1;
    }
    let pos = vcall!(g, this, scroll.vfunction82);
    let size = vcall!(g, this, scroll.vfunction81);
    (pos + size <= p) as i32
}

/// port: 004709b0 Sexy::ScrollbarWidget::vfunction54_for_Widget
/// `MouseDown(x, y, btn, clicks)`: off the thumb pages once and keeps paging (Update); on
/// it starts a drag.
pub fn vfunction54_for_Widget(g: &mut G, this: Ptr, param_1: i32, param_2: i32, _btn: i32, _clicks: i32) {
    if !g.w(this).offset_0x2 {
        let c = vcall!(g, this, scroll.vfunction85, param_1, param_2);
        if c == -1 {
            let d = g.scrollbar(this).clone();
            vcall!(g, this, scroll.vfunction74, d.ext_0xa0 - d.ext_0xb0);
            g.scrollbar(this).ext_0xc8 = 1;
            g.scrollbar(this).ext_0xcc = 0;
        } else if c == 0 {
            g.scrollbar(this).ext_0xb9 = true;
            let pos = vcall!(g, this, scroll.vfunction82);
            let d = g.scrollbar(this);
            d.ext_0xc4 = param_2;
            d.ext_0xd8 = param_2;
            d.ext_0xc0 = param_1;
            d.ext_0xd4 = param_1;
            d.ext_0xbc = pos;
            return;
        } else if c == 1 {
            let d = g.scrollbar(this).clone();
            vcall!(g, this, scroll.vfunction74, d.ext_0xb0 + d.ext_0xa0);
            g.scrollbar(this).ext_0xc8 = 2;
            g.scrollbar(this).ext_0xcc = 0;
        }
    }
    let d = g.scrollbar(this);
    d.ext_0xd8 = param_2;
    d.ext_0xd4 = param_1;
}

/// port: 00470a90 Sexy::ScrollbarWidget::vfunction56_for_Widget
/// `MouseUp(...)`: stops paging and dragging.
pub fn vfunction56_for_Widget(g: &mut G, this: Ptr, _x: i32, _y: i32, _btn: i32, _clicks: i32) {
    g.scrollbar(this).ext_0xc8 = 0;
    g.scrollbar(this).ext_0xb9 = false;
    vcall!(g, this, w.vfunction18);
}

/// port: 00470ab0 Sexy::ScrollbarWidget::vfunction59_for_Widget
/// `MouseDrag(x, y)`: drags the thumb.
pub fn vfunction59_for_Widget(g: &mut G, this: Ptr, param_1: i32, param_2: i32) {
    let d = g.scrollbar(this).clone();
    if d.ext_0xb9 {
        if d.ext_0xb8 {
            vcall!(g, this, scroll.vfunction84, (d.ext_0xbc - d.ext_0xc0) + param_1);
        } else {
            vcall!(g, this, scroll.vfunction84, (d.ext_0xbc - d.ext_0xc4) + param_2);
        }
    }
    let d = g.scrollbar(this);
    d.ext_0xd4 = param_1;
    d.ext_0xd8 = param_2;
}
