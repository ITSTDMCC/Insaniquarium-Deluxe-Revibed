//! `Sexy::ButtonWidget` (framework, 0x00475090..0x00475c40) and `Sexy::HyperlinkWidget`
//! (0x004728e0..0x00472b40). `ButtonWidget_data` starts at object offset 0x88.

use crate::sexy::graphics::{FUN_00455890, FUN_004558e0, FUN_00455880, FUN_00455920, FUN_00455cf0, FUN_00455d20, FUN_00455e40};
use crate::sexy::object::{WidgetContainer_data, Widget_data};
use crate::sexy::prelude::*;

/// `ButtonWidget_data` (object offset 0x88, 152 bytes).
#[derive(Debug, Clone, Default)]
pub struct ButtonWidget_data {
    /// +0x88 `mId`.
    pub offset_0x0: i32,
    /// +0x8c `mLabel` (std::string 0x8c..0xa8).
    pub field_0x4: Vec<u8>,
    /// +0xa8 `mLabelJustify` (0 center, 1 right, else left).
    pub offset_0x20: i32,
    /// +0xac `mFont` (owned copy).
    pub offset_0x24: Ptr,
    /// +0xb0 `mButtonImage`.
    pub offset_0x28: Ptr,
    /// +0xb4 `mOverImage`.
    pub offset_0x2c: Ptr,
    /// +0xb8 `mDownImage`.
    pub offset_0x30: Ptr,
    /// +0xbc `mDisabledImage`.
    pub offset_0x34: Ptr,
    /// +0xc0 `mNormalRect`.
    pub offset_0x38: Rect,
    /// +0xd0 `mOverRect`.
    pub offset_0x48: Rect,
    /// +0xe0 `mDownRect`.
    pub offset_0x58: Rect,
    /// +0xf0 `mDisabledRect`.
    pub offset_0x68: Rect,
    /// +0x100 `mInverted`.
    pub offset_0x78: bool,
    /// +0x101 `mBtnNoDraw`.
    pub offset_0x79: bool,
    /// +0x102 `mFrameNoDraw`.
    pub offset_0x7a: bool,
    /// +0x104 `mButtonListener` (the object whose ButtonListener vftable receives events).
    pub offset_0x7c: Ptr,
    /// +0x108 `mOverAlpha`.
    pub offset_0x80: f64,
    /// +0x110 `mOverAlphaSpeed`.
    pub offset_0x88: f64,
    /// +0x118 `mOverAlphaFadeInSpeed`.
    pub offset_0x90: f64,
}

/// `HyperlinkWidget_data` (object offset 0x120).
#[derive(Debug, Clone, Default)]
pub struct HyperlinkWidget_data {
    /// +0x120 `mColor`.
    pub offset_0x0: Color,
    /// +0x130 `mOverColor`.
    pub field_0x10: Color,
    /// +0x140 `mUnderlineSize`.
    pub offset_0x20: i32,
    /// +0x144 `mUnderlineOffset`.
    pub offset_0x24: i32,
}

/// Parts below ButtonWidget.
#[derive(Debug, Clone)]
pub enum BtnSub {
    None,
    Hyperlink(HyperlinkWidget_data),
    DialogButton(crate::sexy::dialog_button::DialogButton_data),
    MenuButton(crate::game::menu_button::MenuButtonWidget_data),
    StoreButton(crate::game::store::StoreButtonWidget_data),
    FishButton(crate::game::sim_fish::FishButtonWidget_data),
    PetButton(crate::game::pets_screen::PetButtonWidget_data),
    Scrollbutton(crate::sexy::scrollbar::ScrollbuttonWidget_data),
}

#[derive(Debug, Clone)]
pub struct ButtonExt {
    pub b: ButtonWidget_data,
    pub sub: BtnSub,
}

impl G {
    pub fn btn(&mut self, p: Ptr) -> &mut ButtonWidget_data {
        match &mut self.widget(p).ext {
            WExt::Button(e) => &mut e.b,
            e => panic!("{p} is not a ButtonWidget: {e:?}"),
        }
    }
    pub fn hyperlink(&mut self, p: Ptr) -> &mut HyperlinkWidget_data {
        match &mut self.widget(p).ext {
            WExt::Button(e) => match &mut e.sub {
                BtnSub::Hyperlink(h) => h,
                s => panic!("{p} is not a HyperlinkWidget: {s:?}"),
            },
            e => panic!("{p} is not a ButtonWidget: {e:?}"),
        }
    }
}

/// `gButtonWidgetColors` (@ 005e7318): label, label hilite, dark outline, light outline,
/// medium outline, background.
const BUTTON_COLORS: [[i32; 3]; 6] = [[0, 0, 0], [0, 0, 0], [0, 0, 0], [255, 255, 255], [132, 132, 132], [212, 212, 212]];

/// port: 00475090 Sexy::ButtonWidget::ButtonWidget
/// `ButtonWidget(int theId, ButtonListener*)`: the parts (the caller allocates). Ends with
/// `SetColors(gButtonWidgetColors, 6)`, which on a widget without a parent stores the colors
/// and sets `mDirty`.
pub fn ButtonWidget(param_1: i32, param_2: Ptr) -> (WidgetContainer_data, Widget_data, ButtonWidget_data) {
    let (mut wc, mut w) = crate::sexy::widget::Widget();
    wc.offset_0x3c = true;
    let b = ButtonWidget_data { offset_0x0: param_1, offset_0x7c: param_2, ..Default::default() };
    w.offset_0xc = BUTTON_COLORS.iter().map(|c| FUN_00433360(c[0], c[1], c[2])).collect();
    wc.offset_0x28 = true;
    (wc, w, b)
}

/// `new ButtonWidget(theId, theListener)`.
pub fn new_button_widget(g: &mut G, id: i32, listener: Ptr) -> Ptr {
    let (wc, w, b) = ButtonWidget(id, listener);
    g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__ButtonWidget_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::Button(Box::new(ButtonExt { b, sub: BtnSub::None })) }),
    })
}

/// port: 004751d0 Sexy::ButtonWidget::~ButtonWidget
pub fn dtor_ButtonWidget(g: &mut G, this: Ptr) {
    let font = g.btn(this).offset_0x24;
    if font != NULL {
        g.free(font);
    }
    g.btn(this).field_0x4.clear();
    crate::sexy::widget::dtor_Widget(g, this);
}

/// port: 004752c0 FUN_004752c0
/// `HaveButtonImage(Image*, const Rect&)`.
pub fn FUN_004752c0(param_1: Ptr, rect: &Rect) -> bool {
    !(param_1 == NULL && rect.mWidth == 0)
}

/// port: 004752e0 Sexy::ButtonWidget::vfunction71
/// `DrawButtonImage(Graphics*, Image*, const Rect&, int x, int y)`: a rect selects a cel of
/// `mButtonImage`, otherwise the image is drawn whole.
pub fn vfunction71(g: &mut G, this: Ptr, gfx: &mut Graphics, image: Ptr, rect: Rect, x: i32, y: i32) {
    if rect.mWidth != 0 {
        let img = g.btn(this).offset_0x28;
        FUN_00455e40(gfx, g, img, x, y, &rect);
    } else {
        FUN_00455d20(gfx, g, image, x, y);
    }
}

/// port: 00475270 Sexy::ButtonWidget::vfunction72
/// `SetFont(Font*)`: keeps its own duplicate.
pub fn vfunction72(g: &mut G, this: Ptr, font: Ptr) {
    let old = g.btn(this).offset_0x24;
    if old != NULL {
        g.free(old);
    }
    let dup = crate::sexy::image_font::duplicate(g, font);
    g.btn(this).offset_0x24 = dup;
}

/// port: 004752a0 Sexy::ButtonWidget::vfunction73
/// `IsButtonDown()`.
pub fn vfunction73(g: &mut G, this: Ptr) -> bool {
    let w = g.w(this);
    w.offset_0x4 && w.offset_0x5 && !w.offset_0x2
}

fn color(g: &mut G, this: Ptr, idx: i32) -> Color {
    crate::sexy::widget::vfunction37(g, this, idx)
}

/// port: 00475330 Sexy::ButtonWidget::vfunction27
/// `Draw(Graphics*)`.
pub fn vfunction27(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    if g.btn(this).offset_0x79 {
        return;
    }
    // (A SysFont "Arial Unicode MS" 10 is created when a labelled button has no font; every
    // button of the game gets a font before drawing.)
    let w = g.w(this).clone();
    let is_down = (w.offset_0x4 && w.offset_0x5 && !w.offset_0x2) ^ g.btn(this).offset_0x78;
    let (mut fx, mut fy) = (0, 0);
    let font = g.btn(this).offset_0x24;
    let (bw, bh) = (g.wc(this).offset_0x34, g.wc(this).offset_0x38);
    if font != NULL {
        let label = g.btn(this).field_0x4.clone();
        match g.btn(this).offset_0x20 {
            0 => fx = (bw - crate::sexy::image_font::string_width(g, font, &label)) / 2,
            1 => fx = bw - crate::sexy::image_font::string_width(g, font, &label),
            _ => {}
        }
        let a = crate::sexy::image_font::get_ascent(g, font);
        fy = (crate::sexy::image_font::get_ascent(g, font) + (bh - a / 6 - 1)) / 2;
    }
    FUN_00455880(gfx, font);
    let b = g.btn(this).clone();
    let label = b.field_0x4.clone();
    if b.offset_0x28 == NULL && b.offset_0x30 == NULL {
        if !b.offset_0x7a {
            let c = color(g, this, 5);
            FUN_00455890(gfx, c);
            FUN_00455920(gfx, 0, 0, bw, bh);
        }
        if is_down {
            if !b.offset_0x7a {
                let c = color(g, this, 2);
                FUN_00455890(gfx, c);
                FUN_00455920(gfx, 0, 0, bw - 1, 1);
                FUN_00455920(gfx, 0, 0, 1, bh - 1);
                let c = color(g, this, 3);
                FUN_00455890(gfx, c);
                FUN_00455920(gfx, 0, bh - 1, bw, 1);
                FUN_00455920(gfx, bw - 1, 0, 1, bh);
                let c = color(g, this, 4);
                FUN_00455890(gfx, c);
                FUN_00455920(gfx, 1, 1, bw - 3, 1);
                FUN_00455920(gfx, 1, 1, 1, bh - 3);
            }
            let c = color(g, this, w.offset_0x5 as i32);
            FUN_00455890(gfx, c);
            FUN_00455cf0(gfx, g, &label, fx + 1, fy + 1);
            return;
        }
        if !b.offset_0x7a {
            let c = color(g, this, 3);
            FUN_00455890(gfx, c);
            FUN_00455920(gfx, 0, 0, bw - 1, 1);
            FUN_00455920(gfx, 0, 0, 1, bh - 1);
            let c = color(g, this, 2);
            FUN_00455890(gfx, c);
            FUN_00455920(gfx, 0, bh - 1, bw, 1);
            FUN_00455920(gfx, bw - 1, 0, 1, bh);
            let c = color(g, this, 4);
            FUN_00455890(gfx, c);
            FUN_00455920(gfx, 1, bh - 2, bw - 2, 1);
            FUN_00455920(gfx, bw - 2, 1, 1, bh - 2);
        }
    } else if is_down {
        if FUN_004752c0(b.offset_0x30, &b.offset_0x58) {
            vcall!(g, this, btn.vfunction71, gfx, b.offset_0x30, b.offset_0x58, 0, 0);
        } else if FUN_004752c0(b.offset_0x2c, &b.offset_0x48) {
            vcall!(g, this, btn.vfunction71, gfx, b.offset_0x2c, b.offset_0x48, 1, 1);
        } else {
            vcall!(g, this, btn.vfunction71, gfx, b.offset_0x28, b.offset_0x38, 1, 1);
        }
        let c = color(g, this, 1);
        FUN_00455890(gfx, c);
        FUN_00455cf0(gfx, g, &label, fx + 1, fy + 1);
        return;
    } else if w.offset_0x2 && FUN_004752c0(b.offset_0x34, &b.offset_0x68) {
        vcall!(g, this, btn.vfunction71, gfx, b.offset_0x34, b.offset_0x68, 0, 0);
    } else if 0.0 < b.offset_0x80 && FUN_004752c0(b.offset_0x2c, &b.offset_0x48) {
        if FUN_004752c0(b.offset_0x28, &b.offset_0x38) && b.offset_0x80 < 1.0 {
            vcall!(g, this, btn.vfunction71, gfx, b.offset_0x28, b.offset_0x38, 0, 0);
        }
        FUN_004558e0(gfx, true);
        let a = crate::sexy::crt::ftol(b.offset_0x80 * 255.0) as i32;
        FUN_00455890(gfx, CRect(0xff, 0xff, 0xff, a));
        vcall!(g, this, btn.vfunction71, gfx, b.offset_0x2c, b.offset_0x48, 0, 0);
        FUN_004558e0(gfx, false);
    } else if (w.offset_0x5 || w.offset_0x4) && FUN_004752c0(b.offset_0x2c, &b.offset_0x48) {
        vcall!(g, this, btn.vfunction71, gfx, b.offset_0x2c, b.offset_0x48, 0, 0);
    } else if FUN_004752c0(b.offset_0x28, &b.offset_0x38) {
        vcall!(g, this, btn.vfunction71, gfx, b.offset_0x28, b.offset_0x38, 0, 0);
    }
    let c = color(g, this, w.offset_0x5 as i32);
    FUN_00455890(gfx, c);
    FUN_00455cf0(gfx, g, &label, fx, fy);
}

/// port: 00475900 Sexy::ButtonWidget::vfunction38
/// `SetDisabled(bool)`.
pub fn vfunction38(g: &mut G, this: Ptr, disabled: bool) {
    crate::sexy::widget::vfunction38(g, this, disabled);
    let b = g.btn(this);
    if FUN_004752c0(b.offset_0x34, &b.offset_0x68.clone()) {
        vcall!(g, this, w.vfunction18);
    }
}

fn hilite_differs(g: &mut G, this: Ptr) -> bool {
    let c = &g.w(this).offset_0xc;
    c.get(1) != c.get(0)
}

/// port: 00475930 Sexy::ButtonWidget::vfunction51
/// `MouseEnter()`.
pub fn vfunction51(g: &mut G, this: Ptr) {
    let b = g.btn(this);
    if b.offset_0x90 == 0.0 && 0.0 < b.offset_0x80 {
        b.offset_0x80 = 0.0;
    }
    let (over, rect) = (g.btn(this).offset_0x2c, g.btn(this).offset_0x48);
    if g.w(this).offset_0x4 || FUN_004752c0(over, &rect) || hilite_differs(g, this) {
        vcall!(g, this, w.vfunction18);
    }
    let (listener, id) = (g.btn(this).offset_0x7c, g.btn(this).offset_0x0);
    vcall!(g, listener, bl.vfunction5, id);
}

/// port: 004759e0 Sexy::ButtonWidget::vfunction52
/// `MouseLeave()`.
pub fn vfunction52(g: &mut G, this: Ptr) {
    let b = g.btn(this);
    if b.offset_0x88 == 0.0 && 0.0 < b.offset_0x80 {
        b.offset_0x80 = 0.0;
    } else if 0.0 < b.offset_0x88 && b.offset_0x80 == 0.0 {
        b.offset_0x80 = 1.0;
    }
    let (over, rect) = (g.btn(this).offset_0x2c, g.btn(this).offset_0x48);
    if g.w(this).offset_0x4 || FUN_004752c0(over, &rect) || hilite_differs(g, this) {
        vcall!(g, this, w.vfunction18);
    }
    let (listener, id) = (g.btn(this).offset_0x7c, g.btn(this).offset_0x0);
    vcall!(g, listener, bl.vfunction6, id);
}

/// port: 00475ac0 Sexy::ButtonWidget::vfunction53
/// `MouseMove(int x, int y)`.
pub fn vfunction53(g: &mut G, this: Ptr, x: i32, y: i32) {
    let (listener, id) = (g.btn(this).offset_0x7c, g.btn(this).offset_0x0);
    vcall!(g, listener, bl.vfunction7, id, x, y);
}

/// port: 00475af0 Sexy::ButtonWidget::vfunction54
/// `MouseDown(int x, int y, int btn, int clicks)`.
pub fn vfunction54(g: &mut G, this: Ptr, _x: i32, _y: i32, _btn: i32, clicks: i32) {
    let (listener, id) = (g.btn(this).offset_0x7c, g.btn(this).offset_0x0);
    vcall!(g, listener, bl.vfunction1, id, clicks);
    vcall!(g, this, w.vfunction18);
}

/// port: 00475b20 Sexy::ButtonWidget::vfunction56
/// `MouseUp(int x, int y, int btn, int clicks)`: a release over the button is a click.
pub fn vfunction56(g: &mut G, this: Ptr, _x: i32, _y: i32, _btn: i32, _clicks: i32) {
    let wm = g.wc(this).offset_0xc;
    if g.w(this).offset_0x5 && crate::sexy::widget_manager::wm(g, wm).offset_0x30 {
        let (listener, id) = (g.btn(this).offset_0x7c, g.btn(this).offset_0x0);
        vcall!(g, listener, bl.vfunction3, id);
    }
    vcall!(g, this, w.vfunction18);
}

/// port: 00475b60 Sexy::ButtonWidget::vfunction23
/// `Update()`: down-tick while held; fades `mOverAlpha` in/out.
pub fn vfunction23(g: &mut G, this: Ptr) {
    crate::sexy::widget_container::vfunction23(g, this);
    let w = g.w(this).clone();
    if w.offset_0x4 && w.offset_0x5 {
        let (listener, id) = (g.btn(this).offset_0x7c, g.btn(this).offset_0x0);
        vcall!(g, listener, bl.vfunction4, id);
    }
    let w = g.w(this).clone();
    if !w.offset_0x4 && !w.offset_0x5 {
        let b = g.btn(this);
        if 0.0 < b.offset_0x80 {
            if b.offset_0x88 <= 0.0 {
                b.offset_0x80 = 0.0;
            } else {
                b.offset_0x80 -= b.offset_0x88;
                if b.offset_0x80 < 0.0 {
                    b.offset_0x80 = 0.0;
                }
            }
            vcall!(g, this, w.vfunction18);
        }
        return;
    }
    if !w.offset_0x5 {
        return;
    }
    let b = g.btn(this);
    if 0.0 < b.offset_0x90 && b.offset_0x80 < 1.0 {
        b.offset_0x80 += b.offset_0x90;
        if 1.0 < b.offset_0x80 {
            b.offset_0x80 = 1.0;
        }
        vcall!(g, this, w.vfunction18);
    }
}

/// port: 004728e0 Sexy::HyperlinkWidget::HyperlinkWidget
/// `HyperlinkWidget(int theId, ButtonListener*)`: white text and hover color, finger
/// cursor, a one-pixel underline 3 pixels below the baseline.
pub fn HyperlinkWidget(g: &mut G, param_1: i32, param_2: Ptr) -> Ptr {
    let (wc, mut w, b) = ButtonWidget(param_1, param_2);
    w.offset_0x28 = true;
    let h = HyperlinkWidget_data {
        offset_0x0: FUN_00433360(0xff, 0xff, 0xff),
        field_0x10: FUN_00433360(0xff, 0xff, 0xff),
        offset_0x20: 1,
        offset_0x24: 3,
    };
    g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__HyperlinkWidget_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::Button(Box::new(ButtonExt { b, sub: BtnSub::Hyperlink(h) })) }),
    })
}

/// port: 00472950 Sexy::HyperlinkWidget::vfunction27
/// `Draw(Graphics*)`: centered label in `mColor`/`mOverColor`, underlined.
pub fn vfunction27__00472950(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    let font = g.btn(this).offset_0x24;
    let label = g.btn(this).field_0x4.clone();
    let (bw, bh) = (g.wc(this).offset_0x34, g.wc(this).offset_0x38);
    let sw = crate::sexy::image_font::string_width(g, font, &label);
    let x = (bw - sw) / 2;
    let a = crate::sexy::image_font::get_ascent(g, font);
    let y = (a + bh) / 2 - 1;
    let h = g.hyperlink(this).clone();
    FUN_00455890(gfx, if g.w(this).offset_0x5 { h.field_0x10 } else { h.offset_0x0 });
    FUN_00455880(gfx, font);
    FUN_00455cf0(gfx, g, &label, x, y);
    for i in 0..h.offset_0x20 {
        let w = crate::sexy::image_font::string_width(g, font, &label);
        FUN_00455920(gfx, x, h.offset_0x24 + i + y, w, 1);
    }
}

/// port: 00472b00 Sexy::HyperlinkWidget::vfunction51
/// `MouseEnter()`: also repaints fully (the underline color changes).
pub fn vfunction51__00472b00(g: &mut G, this: Ptr) {
    vfunction51(g, this);
    vcall!(g, this, w.vfunction20);
}

/// port: 00472b20 Sexy::HyperlinkWidget::vfunction52
/// `MouseLeave()`.
pub fn vfunction52__00472b20(g: &mut G, this: Ptr) {
    vfunction52(g, this);
    vcall!(g, this, w.vfunction20);
}

/// port: 0046ca30 Sexy::ButtonListener::vfunction1
/// `ButtonPress(int theId, int theClickCount)`: forwards to `ButtonPress(theId)` (slot 2).
pub fn vfunction1__0046ca30(g: &mut G, this: Ptr, id: i32, _clicks: i32) {
    vcall!(g, this, bl.vfunction2, id);
}
