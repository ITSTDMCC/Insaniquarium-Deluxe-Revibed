//! `Sexy::MenuButtonWidget` (the shop buttons along the top of the tank) and its
//! `Sexy::ToolTipWidget`. A MenuButtonWidget is a `ButtonWidget` with an item icon (a cel
//! of an image, animated when the cel is negative), a small caption over it (+0x15c, e.g.
//! the food quantity), a price line (+0x140, "$n" or "MAX"), a flash countdown and a tooltip
//! that the widget manager shows while the mouse is over the button.
//! `MenuButtonWidget_data` starts at object offset 0x120; ToolTipWidget is a
//! `MyLabelWidget` (same data, own vtable).

use crate::game::board_parts::MyLabelWidget_data;
use crate::sexy::button_widget::{BtnSub, ButtonExt};
use crate::sexy::graphics::{FUN_00455800, FUN_00455880, FUN_00455890, FUN_00455920, FUN_004559b0, FUN_00455cf0, FUN_00455d20, FUN_00456340, FUN_00456950, FUN_00456980};
use crate::sexy::image_font as font;
use crate::sexy::prelude::*;
use crate::sexy::types::{CRect, FUN_00433300, FUN_00433360};

/// `MenuButtonWidget_data` (object offset 0x120, 116 bytes).
#[derive(Debug, Clone, Default)]
pub struct MenuButtonWidget_data {
    /// +0x120 the `ToolTipWidget` (owned).
    pub offset_0x0: Ptr,
    /// +0x124 the icon image (none: no icon).
    pub offset_0x4: Ptr,
    /// +0x128 icon x.
    pub field_0x8: i32,
    /// +0x12c icon y.
    pub field_0xc: i32,
    /// +0x130 icon column (negative: animated by the board's update count).
    pub field_0x10: i32,
    /// +0x134 icon row.
    pub field_0x14: i32,
    /// +0x138 update counter.
    pub offset_0x18: i32,
    /// +0x13c flash countdown (drawn as the flash cels while 1..6).
    pub offset_0x1c: i32,
    /// +0x140 the price line.
    pub field_0x20: Vec<u8>,
    /// +0x15c the caption over the icon.
    pub field_0x3c: Vec<u8>,
    /// +0x178 the price color.
    pub offset_0x58: Color,
    /// +0x188 the widget manager (shows / hides the tooltip).
    pub offset_0x68: Ptr,
    /// +0x18c tooltip x offset from the button.
    pub offset_0x6c: i32,
    /// +0x190 tooltip y offset from the button.
    pub offset_0x70: i32,
}

impl G {
    pub fn menu_button(&mut self, p: Ptr) -> &mut MenuButtonWidget_data {
        match &mut self.widget(p).ext {
            WExt::Button(e) => match &mut e.sub {
                BtnSub::MenuButton(d) => d,
                s => panic!("{p} is not a MenuButtonWidget: {s:?}"),
            },
            e => panic!("{p} is not a ButtonWidget: {e:?}"),
        }
    }
}

/// port: 005373c0 Sexy::MenuButtonWidget::MenuButtonWidget
/// `MenuButtonWidget(WidgetManager*, int theId, ButtonListener*, std::string theToolTip)`.
pub fn MenuButtonWidget(g: &mut G, param_1: Ptr, param_2: i32, param_3: Ptr, param_4: &[u8]) -> Ptr {
    let (wc, w, b) = crate::sexy::button_widget::ButtonWidget(param_2, param_3);
    let d = MenuButtonWidget_data { offset_0x58: FUN_00433300(), ..Default::default() };
    let this = g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__MenuButtonWidget_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::Button(Box::new(ButtonExt { b, sub: BtnSub::MenuButton(d) })) }),
    });
    g.w(this).offset_0x28 = true;
    g.wc(this).offset_0x3d = false;
    let tip = ToolTipWidget(g, param_4);
    let d = g.menu_button(this);
    d.offset_0x68 = param_1;
    d.offset_0x6c = 0xf;
    d.offset_0x70 = 0x3c;
    d.offset_0x4 = NULL;
    d.offset_0x1c = 0;
    d.offset_0x18 = 0;
    d.offset_0x0 = tip;
    d.offset_0x58 = FUN_00433360(0x6e, 0xfa, 0x6e);
    this
}

/// port: 00533030 Sexy::MenuButtonWidget::~MenuButtonWidget
/// Deletes the tooltip.
pub fn dtor_MenuButtonWidget(g: &mut G, this: Ptr) {
    let tip = g.menu_button(this).offset_0x0;
    if tip != NULL {
        vcall!(g, tip, w.vfunction1, 1);
    }
    let d = g.menu_button(this);
    d.field_0x3c.clear();
    d.field_0x20.clear();
    crate::sexy::button_widget::dtor_ButtonWidget(g, this);
}

/// port: 00533330 Sexy::MenuButtonWidget::deleting_destructor
pub fn deleting_destructor(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_MenuButtonWidget(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 005303a0 Sexy::MenuButtonWidget::vfunction55
/// `MouseDown(x, y, clicks)`: `Widget::MouseDown`, then hides the tooltip.
pub fn vfunction55(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32) {
    crate::sexy::widget::vfunction55(g, this, param_1, param_2, param_3);
    let (mgr, tip) = (g.menu_button(this).offset_0x68, g.menu_button(this).offset_0x0);
    vcall!(g, mgr, w.vfunction5, tip);
}

/// port: 00530460 Sexy::MenuButtonWidget::vfunction51
/// `MouseEnter()`: moves the tooltip below the button and shows it.
pub fn vfunction51(g: &mut G, this: Ptr) {
    crate::sexy::button_widget::vfunction51(g, this);
    let d = g.menu_button(this).clone();
    let (x, y) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
    let (tw, th) = (g.wc(d.offset_0x0).offset_0x34, g.wc(d.offset_0x0).offset_0x38);
    vcall!(g, d.offset_0x0, w.vfunction41, d.offset_0x6c + x, d.offset_0x70 + y, tw, th);
    vcall!(g, d.offset_0x68, w.vfunction4, d.offset_0x0);
}

/// port: 005304b0 Sexy::MenuButtonWidget::vfunction52
/// `MouseLeave()`: hides the tooltip.
pub fn vfunction52(g: &mut G, this: Ptr) {
    crate::sexy::button_widget::vfunction52(g, this);
    let (mgr, tip) = (g.menu_button(this).offset_0x68, g.menu_button(this).offset_0x0);
    vcall!(g, mgr, w.vfunction5, tip);
}

/// port: 005304d0 Sexy::MenuButtonWidget::vfunction22
/// `RemovedFromManager(WidgetManager*)`: also takes the tooltip down.
pub fn vfunction22(g: &mut G, this: Ptr, param_1: Ptr) {
    crate::sexy::widget_container::vfunction22(g, this, param_1);
    let (mgr, tip) = (g.menu_button(this).offset_0x68, g.menu_button(this).offset_0x0);
    if tip != NULL {
        vcall!(g, mgr, w.vfunction5, tip);
    }
}

/// port: 00530500 Sexy::MenuButtonWidget::vfunction23
/// `Update()`: counts and runs the flash down unless the board is paused.
pub fn vfunction23(g: &mut G, this: Ptr) {
    crate::sexy::widget_container::vfunction23(g, this);
    let app = g.globals.DAT_005eb6a4;
    let board = g.wfa(app).offset_0x4;
    if board == NULL || !g.board(board).field_0x8 {
        let d = g.menu_button(this);
        let t = d.offset_0x1c;
        d.offset_0x18 += 1;
        if 0 < t {
            d.offset_0x1c = t - 1;
        }
    }
}

/// port: 00530540 Sexy::MenuButtonWidget::vfunction27
/// `Draw(Graphics*)`: while mouse-visible (and not early in a flash) the button, the icon
/// and the green caption; always the price, the flash cel, and the frame (clipped below
/// y 0x28 when the button itself was not drawn).
pub fn vfunction27(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    let mut drawn = false;
    let width = g.wc(this).offset_0x34;
    if g.w(this).offset_0x1 && g.menu_button(this).offset_0x1c < 7 {
        crate::sexy::button_widget::vfunction27(g, this, gfx);
        let app = g.globals.DAT_005eb6a4;
        let board = g.wfa(app).offset_0x4;
        let t = if board == NULL { 0 } else { g.board(board).ext_0x444 };
        let d = g.menu_button(this).clone();
        if d.offset_0x4 != NULL {
            let col = if d.field_0x10 < 0 { (t / 2) % g.image(d.offset_0x4).offset_0x2c } else { d.field_0x10 };
            FUN_00456980(gfx, g, d.offset_0x4, d.field_0x8, d.field_0xc, col, d.field_0x14);
        }
        let f = g.res.DAT_005e8cd0;
        FUN_00455880(gfx, f);
        FUN_00455890(gfx, FUN_00433360(0x6e, 0xfa, 0x6e));
        let y = font::get_ascent(g, f) + 9;
        let sw = font::string_width(g, f, &d.field_0x3c);
        FUN_00455cf0(gfx, g, &d.field_0x3c, (width - sw) / 2, y);
        drawn = true;
    }
    let d = g.menu_button(this).clone();
    let f = g.res.DAT_005e8c20;
    FUN_00455880(gfx, f);
    FUN_00455890(gfx, d.offset_0x58);
    let y = font::get_ascent(g, f) + 0x2d;
    let sw = font::string_width(g, f, &d.field_0x20);
    FUN_00455cf0(gfx, g, &d.field_0x20, (width - sw) / 2, y);
    let n = d.offset_0x1c;
    if n != 0 && n < 7 {
        let img = g.res.DAT_005e8a78;
        FUN_00456950(gfx, g, img, 0, 0, (6 - n) / 2);
    }
    let frame = g.res.DAT_005e8e2c;
    if drawn {
        FUN_00455d20(gfx, g, frame, 0, 0);
        return;
    }
    let mut g2 = FUN_00455800(gfx);
    let h = g.wc(this).offset_0x38;
    FUN_00456340(&mut g2, 0, 0x28, width, h - 0x28);
    FUN_00455d20(&mut g2, g, frame, 0, 0);
}

/// port: 005303e0 FUN_005303e0
/// `SetPriceColor(const Color&)` (+0x178).
pub fn FUN_005303e0(g: &mut G, this: Ptr, param_1: Color) {
    g.menu_button(this).offset_0x58 = param_1;
}

/// port: 00530410 FUN_00530410
/// `SetIcon(Image*, int x, int y, int row, int col)`.
pub fn FUN_00530410(g: &mut G, this: Ptr, param_1: Ptr, param_2: i32, param_3: i32, param_4: i32, param_5: i32) {
    let d = g.menu_button(this);
    d.offset_0x4 = param_1;
    d.field_0x8 = param_2;
    d.field_0xc = param_3;
    d.field_0x14 = param_4;
    d.field_0x10 = param_5;
}

/// port: 00530450 FUN_00530450
/// `Flash()`: eight updates of flash.
pub fn FUN_00530450(g: &mut G, this: Ptr) {
    g.menu_button(this).offset_0x1c = 8;
}

/// port: 00534360 FUN_00534360
/// `SetCaption(const std::string&)` (+0x15c).
pub fn FUN_00534360(g: &mut G, this: Ptr, param_1: &[u8]) {
    g.menu_button(this).field_0x3c = param_1.to_vec();
}

/// port: 00534450 FUN_00534450
/// `SetMaxed()`: no finger, no mouse, no caption, price "MAX".
pub fn FUN_00534450(g: &mut G, this: Ptr) {
    g.w(this).offset_0x28 = false;
    g.w(this).offset_0x1 = false;
    vcall!(g, this, w.vfunction39, false);
    let d = g.menu_button(this);
    d.field_0x3c = b"".to_vec();
    d.field_0x20 = b"MAX".to_vec();
}

/// port: 00537320 Sexy::ToolTipWidget::ToolTipWidget
/// `ToolTipWidget(const std::string&)`: a hidden-from-mouse label in font `DAT_005e8aa4`,
/// sized to the text.
pub fn ToolTipWidget(g: &mut G, param_1: &[u8]) -> Ptr {
    let this = crate::game::board_parts::MyLabelWidget(g);
    g.obj(this).vt = Some(&crate::sexy::vtables_gen::Sexy__ToolTipWidget_vftable);
    let f = g.res.DAT_005e8aa4;
    g.my_label(this).offset_0x30 = f;
    g.w(this).offset_0x1 = false;
    FUN_005363f0(g, this, param_1);
    this
}

/// port: 005363f0 FUN_005363f0
/// `ToolTipWidget::SetText(const std::string&)`: the text, and a size of its width + 10 by
/// the font height + 12 (keeping the position).
pub fn FUN_005363f0(g: &mut G, this: Ptr, param_1: &[u8]) {
    g.my_label(this).field_0x0 = param_1.to_vec();
    let MyLabelWidget_data { offset_0x30: f, field_0x0: ref s, .. } = g.my_label(this).clone();
    let h = font::get_height(g, f) + 0xc;
    let w = font::string_width(g, f, s) + 10;
    let (x, y) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
    vcall!(g, this, w.vfunction41, x, y, w, h);
}

/// port: 00531d80 Sexy::ToolTipWidget::vfunction27
/// `Draw(Graphics*)`: a pale yellow box with a black border, the text 5 px in, vertically
/// centered.
pub fn vfunction27__00531d80(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    let (w, h) = (g.wc(this).offset_0x34, g.wc(this).offset_0x38);
    FUN_00455890(gfx, CRect(0xff, 0xff, 200, 0xff));
    FUN_00455920(gfx, 0, 0, w, h);
    FUN_00455890(gfx, CRect(0, 0, 0, 0xff));
    FUN_004559b0(gfx, 0, 0, w - 1, h - 1);
    let d = g.my_label(this).clone();
    FUN_00455880(gfx, d.offset_0x30);
    let fh = font::get_height(g, d.offset_0x30);
    let asc = font::get_ascent(g, d.offset_0x30);
    FUN_00455cf0(gfx, g, &d.field_0x0, 5, asc + ((h - fh) >> 1));
}

/// port: 00531e50 Sexy::ToolTipWidget::vfunction23
/// `Update()`: keeps the tooltip on top.
pub fn vfunction23__00531e50(g: &mut G, this: Ptr) {
    crate::sexy::widget_container::vfunction23(g, this);
    let mgr = g.wc(this).offset_0xc;
    vcall!(g, mgr, w.vfunction12, this);
}

/// port: 00534430 FUN_00534430
/// `SetPriceText(const std::string&)` (+0x140).
pub fn FUN_00534430(g: &mut G, this: Ptr, param_1: &[u8]) {
    g.menu_button(this).field_0x20 = param_1.to_vec();
}
