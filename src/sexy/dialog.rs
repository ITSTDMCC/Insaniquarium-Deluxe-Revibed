//! `Sexy::Dialog` (framework, 0x00475c40..0x00476eb0): a box with a header, word-wrapped
//! lines, an optional footer and up to two buttons (ids 1000 and 1001), registered with the
//! app by id (`mDialogMap`); it reports button presses to the app's DialogListener part. The
//! dialog's own ButtonListener part receives its buttons' events. `Dialog_data` starts at
//! object offset 0x8c; the class is 0x150 bytes, larger than the database's 0x100 (fields
//! `ext_0xNNN`; the database attributes those bytes to `MoneyDialog_data`, which begins at
//! 0x100 in its layout).

use crate::sexy::graphics::{FUN_00455880, FUN_00455890, FUN_00455920, FUN_004559b0, FUN_004563f0};
use crate::sexy::image_font as font;
use crate::sexy::prelude::*;

/// `Dialog_data` (object offset 0x8c) plus the bytes past the database's class size.
#[derive(Debug, Clone, Default)]
pub struct Dialog_data {
    /// +0x8c `mDialogListener` (the app's DialogListener part; the app pointer here).
    pub field_0x0: Ptr,
    /// +0x90 `mComponentImage` (null = drawn with rectangles).
    pub field_0x4: Ptr,
    /// +0x94 `mYesButton` (id 1000).
    pub offset_0x8: Ptr,
    /// +0x98 `mNoButton` (id 1001).
    pub offset_0xc: Ptr,
    /// +0x9c `mNumButtons` (only cleared, for dialogs without buttons).
    pub field_0x10: i32,
    /// +0xa0 `mDialogHeader`.
    pub field_0x14: Vec<u8>,
    /// +0xbc `mDialogFooter`.
    pub field_0x30: Vec<u8>,
    /// +0xd8 `mDialogLines`.
    pub field_0x4c: Vec<u8>,
    /// +0xf4 `mButtonMode`: 0 none, 1 yes/no, 2 ok/cancel, 3 one button labelled with the footer.
    pub field_0x68: i32,
    /// +0xf8 `mHeaderFont` (owned copy).
    pub offset_0x6c: Ptr,
    /// +0xfc `mLinesFont` (owned copy).
    pub offset_0x70: Ptr,
    /// +0x100 `mTextAlign`.
    pub ext_0x100: i32,
    /// +0x104 `mLineSpacingOffset`.
    pub ext_0x104: i32,
    /// +0x108 `mButtonHeight`.
    pub ext_0x108: i32,
    /// +0x10c `mBackgroundInsets.mLeft`.
    pub ext_0x10c: i32,
    /// +0x110 `mBackgroundInsets.mTop`.
    pub ext_0x110: i32,
    /// +0x114 `mBackgroundInsets.mRight`.
    pub ext_0x114: i32,
    /// +0x118 `mBackgroundInsets.mBottom`.
    pub ext_0x118: i32,
    /// +0x11c `mContentInsets.mLeft`.
    pub ext_0x11c: i32,
    /// +0x120 `mContentInsets.mTop`.
    pub ext_0x120: i32,
    /// +0x124 `mContentInsets.mRight`.
    pub ext_0x124: i32,
    /// +0x128 `mContentInsets.mBottom`.
    pub ext_0x128: i32,
    /// +0x12c `mSpaceAfterHeader`.
    pub ext_0x12c: i32,
    /// +0x130 `mDragging`.
    pub ext_0x130: bool,
    /// +0x134 `mDragMouseX`.
    pub ext_0x134: i32,
    /// +0x138 `mDragMouseY`.
    pub ext_0x138: i32,
    /// +0x13c `mId`.
    pub ext_0x13c: i32,
    /// +0x140 `mIsModal`.
    pub ext_0x140: bool,
    /// +0x144 `mResult` (0x7fffffff while open, -1 = closed without a result).
    pub ext_0x144: i32,
    /// +0x148 `mButtonHorzSpacing`.
    pub ext_0x148: i32,
    /// +0x14c `mButtonSidePadding`.
    pub ext_0x14c: i32,
}

/// Parts below `Sexy::Dialog`.
#[derive(Debug, Clone)]
pub enum DlgSub {
    None,
    /// `Sexy::MoneyDialog` (+0x150..0x158) and what derives from it.
    Money(crate::game::money_dialog::MoneyDialog_data, MoneySub),
    /// `Sexy::UpdateCheckDialog` (+0x150..0x160).
    UpdateCheck(crate::sexy::update_check_dialog::UpdateCheckDialog_data),
}

/// Parts below `Sexy::MoneyDialog`.
#[derive(Debug, Clone)]
pub enum MoneySub {
    None,
    NewUser(crate::game::money_dialog::NewUserDialog_data),
    Options(crate::game::options_dialog::OptionsDialog_data),
    Virtual(crate::game::virtual_tank::VirtualDialog_data),
    /// `FishNamingDialog`: its two fields are laid out as NewUserDialog's, then its own.
    ScreenSaver(crate::game::sim_setup::ScreenSaverDialog_data),
    FishNaming(crate::game::money_dialog::NewUserDialog_data, crate::game::money_dialog::FishNamingDialog_ext),
    User(crate::game::user_dialog::UserDialog_data),
    Pet(Box<crate::game::pet_dialog::PetDialog_data>),
    Food(crate::game::food_dialog::FoodDialog_data),
    Register(crate::game::register_dialog::RegisterDialog_data),
    Continue(crate::game::continue_dialog::ContinueDialog_data),
}

#[derive(Debug, Clone)]
pub struct DialogExt {
    pub d: Dialog_data,
    pub sub: DlgSub,
}

impl G {
    pub fn dialog(&mut self, p: Ptr) -> &mut Dialog_data {
        match &mut self.widget(p).ext {
            WExt::Dialog(e) => &mut e.d,
            e => panic!("{p} is not a Dialog: {e:?}"),
        }
    }
}

/// `gDialogColors` (@ 005e7360, 7 x RGB): header, lines, footer, button text, button text
/// hilite, background, outline. Mutable: a Dialog without a button image zeroes entries 3
/// and 4 for every later dialog.
#[derive(Debug, Clone)]
pub struct DialogColors(pub [[i32; 3]; 7]);

impl Default for DialogColors {
    fn default() -> Self {
        DialogColors([[255, 255, 255], [255, 255, 0], [255, 255, 255], [255, 255, 255], [255, 255, 255], [80, 80, 80], [255, 255, 255]])
    }
}

/// Allocates a Dialog-derived object (`operator new` + the Widget part); the class's
/// constructor then initializes it in place.
pub fn alloc_dialog(g: &mut G, vt: &'static VTable, sub: DlgSub) -> Ptr {
    let (wc, w) = crate::sexy::widget::Widget();
    g.alloc(Obj { vt: Some(vt), node: Node::Widget(WidgetObj { wc, w, ext: WExt::Dialog(Box::new(DialogExt { d: Dialog_data::default(), sub })) }) })
}

/// port: 00475c40 Sexy::Dialog::Dialog
/// `Dialog(Image* theComponentImage, Image* theButtonComponentImage, int theId, bool isModal,
/// const string& header, const string& lines, const string& footer, int theButtonMode)` on an
/// object from [`alloc_dialog`].
pub fn Dialog(
    g: &mut G,
    this: Ptr,
    param_1: Ptr,
    param_2: Ptr,
    param_3: i32,
    param_4: bool,
    param_5: &[u8],
    param_6: &[u8],
    param_7: &[u8],
    param_8: i32,
) {
    let app = g.globals.DAT_005eb6a4;
    {
        let d = g.dialog(this);
        d.field_0x4 = param_1;
        d.ext_0x13c = param_3;
        d.ext_0x144 = 0x7fffffff;
        d.ext_0x140 = param_4;
        d.ext_0x11c = 0x18;
        d.ext_0x120 = 0x18;
        d.ext_0x124 = 0x18;
        d.ext_0x128 = 0x18;
        d.ext_0x100 = 0;
        d.ext_0x104 = 0;
        d.ext_0x12c = 10;
        d.ext_0x14c = 0;
        d.ext_0x148 = 8;
        d.field_0x0 = app;
        d.field_0x14 = param_5.to_vec();
        d.field_0x30 = param_7.to_vec();
        d.field_0x68 = param_8;
    }
    if param_8 == 1 || param_8 == 2 {
        let yes = crate::sexy::dialog_button::DialogButton(g, param_2, 1000, this);
        g.dialog(this).offset_0x8 = yes;
        let no = crate::sexy::dialog_button::DialogButton(g, param_2, 0x3e9, this);
        g.dialog(this).offset_0xc = no;
        let (a, b) = if param_8 == 1 {
            (
                crate::sexy::sexy_app_base::FUN_004872f0(g, app, "DIALOG_BUTTON_YES", b"YES"),
                crate::sexy::sexy_app_base::FUN_004872f0(g, app, "DIALOG_BUTTON_NO", b"NO"),
            )
        } else {
            (
                crate::sexy::sexy_app_base::FUN_004872f0(g, app, "DIALOG_BUTTON_OK", b"OK"),
                crate::sexy::sexy_app_base::FUN_004872f0(g, app, "DIALOG_BUTTON_CANCEL", b"CANCEL"),
            )
        };
        g.btn(yes).field_0x4 = a;
        g.btn(no).field_0x4 = b;
    } else if param_8 == 3 {
        let yes = crate::sexy::dialog_button::DialogButton(g, param_2, 1000, this);
        g.dialog(this).offset_0x8 = yes;
        g.btn(yes).field_0x4 = param_7.to_vec();
        g.dialog(this).offset_0xc = NULL;
    } else {
        let d = g.dialog(this);
        d.offset_0x8 = NULL;
        d.offset_0xc = NULL;
        d.field_0x10 = 0;
    }
    g.dialog(this).field_0x4c = param_6.to_vec();
    let bh = if param_2 != NULL { g.image(param_2).offset_0x24 } else { 0x18 };
    g.dialog(this).ext_0x108 = bh;
    g.w(this).offset_0x6 = true;
    g.wc(this).offset_0x3c = true;
    let d = g.dialog(this);
    d.offset_0x6c = NULL;
    d.offset_0x70 = NULL;
    d.ext_0x130 = false;
    g.wc(this).offset_0x48 = 1;
    if param_2 == NULL {
        g.globals.DAT_005e7360.0[3] = [0, 0, 0];
        g.globals.DAT_005e7360.0[4] = [0, 0, 0];
    }
    let colors = g.globals.DAT_005e7360.0;
    crate::sexy::widget::vfunction34(g, this, &colors);
}

/// port: 004761a0 Sexy::Dialog::~Dialog
pub fn dtor_Dialog(g: &mut G, this: Ptr) {
    let d = g.dialog(this).clone();
    for b in [d.offset_0x8, d.offset_0xc] {
        if b != NULL {
            vcall!(g, b, w.vfunction1, 1);
        }
    }
    for f in [d.offset_0x6c, d.offset_0x70] {
        if f != NULL {
            g.free(f);
        }
    }
    let d = g.dialog(this);
    d.field_0x4c.clear();
    d.field_0x30.clear();
    d.field_0x14.clear();
    crate::sexy::widget::dtor_Widget(g, this);
}

/// port: 00476180 Sexy::Dialog::deleting_destructor_for_Widget
pub fn deleting_destructor_for_Widget(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_Dialog(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 004762d0 Sexy::Dialog::vfunction35_for_Widget
/// `SetColor(int, const Color&)`: colors 3/4 (button text / hilite) go to both buttons'
/// label colors 0/1 too.
pub fn vfunction35_for_Widget(g: &mut G, this: Ptr, idx: i32, color: Color) {
    crate::sexy::widget::vfunction35(g, this, idx, color);
    let (yes, no) = (g.dialog(this).offset_0x8, g.dialog(this).offset_0xc);
    let i = match idx {
        3 => 0,
        4 => 1,
        _ => return,
    };
    if yes != NULL {
        vcall!(g, yes, w.vfunction35, i, color);
    }
    if no == NULL {
        return;
    }
    vcall!(g, no, w.vfunction35, i, color);
}

/// port: 00476360 Sexy::Dialog::vfunction71_for_Widget
/// `SetButtonFont(Font*)`.
pub fn vfunction71_for_Widget(g: &mut G, this: Ptr, font: Ptr) {
    let (yes, no) = (g.dialog(this).offset_0x8, g.dialog(this).offset_0xc);
    if yes != NULL {
        vcall!(g, yes, btn.vfunction72, font);
    }
    if no != NULL {
        vcall!(g, no, btn.vfunction72, font);
    }
}

/// port: 004763b0 Sexy::Dialog::vfunction72_for_Widget
/// `SetHeaderFont(Font*)`: keeps a copy (`Font::Duplicate`, vtable +0x2c).
pub fn vfunction72_for_Widget(g: &mut G, this: Ptr, font: Ptr) {
    let old = g.dialog(this).offset_0x6c;
    if old != NULL {
        g.free(old);
    }
    let copy = font::duplicate(g, font);
    g.dialog(this).offset_0x6c = copy;
}

/// port: 004763e0 Sexy::Dialog::vfunction73_for_Widget
/// `SetLinesFont(Font*)`.
pub fn vfunction73_for_Widget(g: &mut G, this: Ptr, font: Ptr) {
    let old = g.dialog(this).offset_0x70;
    if old != NULL {
        g.free(old);
    }
    let copy = font::duplicate(g, font);
    g.dialog(this).offset_0x70 = copy;
}

/// port: 00476410 FUN_00476410
/// `EnsureFonts()`: Arial (Windows GDI fonts) for a dialog that was given no header or lines
/// font. Every dialog the game makes sets both first, so the GDI path is never reached.
pub fn FUN_00476410(g: &mut G, param_1: Ptr) {
    let d = g.dialog(param_1);
    if d.offset_0x6c == NULL || d.offset_0x70 == NULL {
        crate::sexy::pending(0x00476410, "Dialog::EnsureFonts SysFont (Windows GDI font)");
    }
}

/// port: 00476570 Sexy::Dialog::vfunction74_for_Widget
/// `GetPreferredHeight(int theWidth)`: insets, header, wrapped lines, footer, buttons.
pub fn vfunction74_for_Widget(g: &mut G, this: Ptr, width: i32) -> i32 {
    FUN_00476410(g, this);
    let d = g.dialog(this).clone();
    let mut h = d.ext_0x128 + d.ext_0x120 + d.ext_0x118 + d.ext_0x110;
    let mut need_space = !d.field_0x14.is_empty();
    if need_space {
        let pad = font::get_ascent_padding(g, d.offset_0x6c);
        let fh = font::get_height(g, d.offset_0x6c);
        h = (h - pad) + fh;
    }
    if !d.field_0x4c.is_empty() {
        if need_space {
            h += d.ext_0x12c;
        }
        let mut gfx = Graphics::new(NULL, 0, 0, std::sync::Arc::new(std::sync::Mutex::new(Vec::new())));
        FUN_00455880(&mut gfx, d.offset_0x70);
        let spacing = font::get_line_spacing(g, d.offset_0x70);
        let w = ((((width - d.ext_0x124) - d.ext_0x11c) - d.ext_0x114) - d.ext_0x10c) + -4;
        let lines = d.field_0x4c.clone();
        h += vcall!(g, this, w.vfunction66, &mut gfx, w, &lines, spacing + d.ext_0x104);
        need_space = true;
    }
    if !d.field_0x30.is_empty() && d.field_0x68 != 3 {
        if need_space {
            h += 8;
        }
        h += font::get_line_spacing(g, d.offset_0x6c);
        need_space = true;
    }
    if d.offset_0x8 != NULL {
        if need_space {
            h += 8;
        }
        return h + 8 + d.ext_0x108;
    }
    h
}

/// port: 00476710 Sexy::Dialog::vfunction27_for_Widget
/// `Draw(Graphics*)`: the box (component image as a 9-slice, or a framed rectangle with a
/// shadow), centered header, word-wrapped lines, centered footer.
pub fn vfunction27_for_Widget(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    FUN_00476410(g, this);
    let d = g.dialog(this).clone();
    let (w, h) = (g.wc(this).offset_0x34, g.wc(this).offset_0x38);
    let box_rect = Rect::new(d.ext_0x10c, d.ext_0x110, (w - d.ext_0x114) - d.ext_0x10c, (h - d.ext_0x118) - d.ext_0x110);
    if d.field_0x4 == NULL {
        let c = vcall!(g, this, w.vfunction36, 6, FUN_004333a0());
        FUN_00455890(gfx, c);
        FUN_004559b0(gfx, 0xc, 0xc, w + -0x19, h + -0x19);
        let c = vcall!(g, this, w.vfunction36, 5, FUN_004333a0());
        FUN_00455890(gfx, c);
        FUN_00455920(gfx, 0xd, 0xd, w + -0x1a, h + -0x1a);
        FUN_00455890(gfx, CRect(0, 0, 0, 0x80));
        FUN_00455920(gfx, w + -0xc, 0x18, 0xc, h + -0x24);
        FUN_00455920(gfx, 0x18, h + -0xc, w + -0x18, 0xc);
    } else {
        FUN_004563f0(gfx, g, &box_rect, d.field_0x4);
    }
    let mut cur_y = d.ext_0x120 + d.ext_0x110;
    if !d.field_0x14.is_empty() {
        let pad = font::get_ascent_padding(g, d.offset_0x6c);
        let asc = font::get_ascent(g, d.offset_0x6c);
        let y = asc + (cur_y - pad);
        FUN_00455880(gfx, d.offset_0x6c);
        let c = g.w(this).offset_0xc[0];
        FUN_00455890(gfx, c);
        vcall!(g, this, w.vfunction63, gfx, y, &d.field_0x14);
        let asc = font::get_ascent(g, d.offset_0x6c);
        let fh = font::get_height(g, d.offset_0x6c);
        cur_y = fh + (y - asc) + d.ext_0x12c;
    }
    FUN_00455880(gfx, d.offset_0x70);
    let c = g.w(this).offset_0xc[1];
    FUN_00455890(gfx, c);
    let rect = Rect::new(d.ext_0x10c + 2 + d.ext_0x11c, cur_y, ((((w - d.ext_0x124) - d.ext_0x10c) - d.ext_0x11c) - d.ext_0x114) + -4, 0);
    let spacing = font::get_line_spacing(g, d.offset_0x70);
    let lines_h = vcall!(g, this, w.vfunction65, gfx, rect, &d.field_0x4c, spacing + d.ext_0x104, d.ext_0x100);
    if !d.field_0x30.is_empty() && d.field_0x68 != 3 {
        let ls = font::get_line_spacing(g, d.offset_0x6c);
        FUN_00455880(gfx, d.offset_0x6c);
        let c = g.w(this).offset_0xc[2];
        FUN_00455890(gfx, c);
        vcall!(g, this, w.vfunction63, gfx, cur_y + lines_h + 8 + ls, &d.field_0x30);
    }
}

/// `Color::Color()` for the default arguments above (@ 004333a0): transparent black.
fn FUN_004333a0() -> Color {
    Color { mRed: 0, mGreen: 0, mBlue: 0, mAlpha: 0 }
}

/// port: 00476a30 Sexy::Dialog::vfunction21_for_Widget
/// `AddedToManager(WidgetManager*)`: adds the buttons.
pub fn vfunction21_for_Widget(g: &mut G, this: Ptr, manager: Ptr) {
    crate::sexy::widget_container::vfunction21(g, this, manager);
    let (yes, no) = (g.dialog(this).offset_0x8, g.dialog(this).offset_0xc);
    if yes != NULL {
        vcall!(g, manager, w.vfunction4, yes);
    }
    if no != NULL {
        vcall!(g, manager, w.vfunction4, no);
    }
}

/// port: 00476a70 Sexy::Dialog::vfunction22_for_Widget
/// `RemovedFromManager(WidgetManager*)`.
pub fn vfunction22_for_Widget(g: &mut G, this: Ptr, manager: Ptr) {
    crate::sexy::widget_container::vfunction22(g, this, manager);
    let (yes, no) = (g.dialog(this).offset_0x8, g.dialog(this).offset_0xc);
    if yes != NULL {
        vcall!(g, manager, w.vfunction5, yes);
    }
    if no != NULL {
        vcall!(g, manager, w.vfunction5, no);
    }
}

/// port: 00476ab0 Sexy::Dialog::vfunction31_for_Widget
/// `OrderInManagerChanged()`: keeps the buttons in front of the dialog.
pub fn vfunction31_for_Widget(g: &mut G, this: Ptr) {
    let wm = g.wc(this).offset_0xc;
    let (yes, no) = (g.dialog(this).offset_0x8, g.dialog(this).offset_0xc);
    if yes != NULL {
        vcall!(g, wm, w.vfunction15, yes, this);
    }
    if no != NULL {
        vcall!(g, wm, w.vfunction15, no, this);
    }
}

/// port: 00476af0 Sexy::Dialog::vfunction41_for_Widget
/// `Resize(int x, int y, int w, int h)`: lays the buttons out along the bottom (two side by
/// side with `mButtonHorzSpacing` between, or one full width).
pub fn vfunction41_for_Widget(g: &mut G, this: Ptr, x: i32, y: i32, w: i32, h: i32) {
    crate::sexy::widget::vfunction41(g, this, x, y, w, h);
    let d = g.dialog(this).clone();
    let (mx, my, mw, mh) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30, g.wc(this).offset_0x34, g.wc(this).offset_0x38);
    if d.offset_0x8 == NULL {
        return;
    }
    let by = (((mh - d.ext_0x128) - d.ext_0x118) + my) - d.ext_0x108;
    if d.offset_0xc != NULL {
        let bw = ((((((d.ext_0x14c * -2 - d.ext_0x124) - d.ext_0x114) + mw) - d.ext_0x10c) - d.ext_0x11c) - d.ext_0x148) / 2;
        vcall!(g, d.offset_0x8, w.vfunction41, d.ext_0x11c + d.ext_0x14c + mx + d.ext_0x10c, by, bw, d.ext_0x108);
        let (yx, yy) = (g.wc(d.offset_0x8).offset_0x2c, g.wc(d.offset_0x8).offset_0x30);
        vcall!(g, d.offset_0xc, w.vfunction41, yx + bw + d.ext_0x148, yy, bw, d.ext_0x108);
        return;
    }
    vcall!(g, d.offset_0x8, w.vfunction41, mx + d.ext_0x10c + d.ext_0x11c, by, (((mw - d.ext_0x124) - d.ext_0x114) - d.ext_0x10c) - d.ext_0x11c, d.ext_0x108);
}

/// port: 00476c40 Sexy::Dialog::vfunction54_for_Widget
/// `MouseDown(int x, int y, int theBtnNum, int theClickCount)`: a single click starts
/// dragging the dialog (the drag cursor, `mWidgetManager->mApp->SetCursor(2)`).
pub fn vfunction54_for_Widget(g: &mut G, this: Ptr, x: i32, y: i32, _btn: i32, clicks: i32) {
    if clicks == 1 {
        let wm = g.wc(this).offset_0xc;
        let app = crate::sexy::widget_manager::wm(g, wm).offset_0x8;
        crate::sexy::sexy_app_base::FUN_004891c0(g, app, 2);
        let d = g.dialog(this);
        d.ext_0x130 = true;
        d.ext_0x134 = x;
        d.ext_0x138 = y;
    }
}

/// port: 00476c80 Sexy::Dialog::vfunction59_for_Widget
/// `MouseDrag(int x, int y)`: moves the dialog, keeping it within 8 pixels of the screen.
pub fn vfunction59_for_Widget(g: &mut G, this: Ptr, x: i32, y: i32) {
    if !g.dialog(this).ext_0x130 {
        return;
    }
    let (mx, my, mw, mh) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30, g.wc(this).offset_0x34, g.wc(this).offset_0x38);
    let (dx, dy) = (g.dialog(this).ext_0x134, g.dialog(this).ext_0x138);
    let wm = g.wc(this).offset_0xc;
    let (sw, sh) = (g.wc(wm).offset_0x34, g.wc(wm).offset_0x38);
    let mut nx = (mx - dx) + x;
    let mut ny = (my - dy) + y;
    if nx < -8 {
        nx = -8;
    } else if sw + 8 < mw + nx {
        nx = (sw - mw) + 8;
    }
    if ny < -8 {
        ny = -8;
    } else if sh + 8 < mh + ny {
        ny = (sh - mh) + 8;
    }
    let ax = (mx - nx) + x;
    let ay = (my - ny) + y;
    let d = g.dialog(this);
    d.ext_0x134 = ax;
    d.ext_0x138 = ay;
    if ax < 8 {
        d.ext_0x134 = 8;
    } else if mw + -9 < ax {
        d.ext_0x134 = mw + -9;
    }
    if ay < 8 {
        d.ext_0x138 = 8;
    } else if mh + -9 < ay {
        d.ext_0x138 = mh + -9;
    }
    vcall!(g, this, w.vfunction42, nx, ny);
}

/// port: 00476d90 Sexy::Dialog::vfunction56_for_Widget
/// `MouseUp(int x, int y, int theBtnNum, int theClickCount)`: ends dragging (arrow cursor).
pub fn vfunction56_for_Widget(g: &mut G, this: Ptr, _x: i32, _y: i32, _btn: i32, _clicks: i32) {
    if g.dialog(this).ext_0x130 {
        let wm = g.wc(this).offset_0xc;
        let app = crate::sexy::widget_manager::wm(g, wm).offset_0x8;
        crate::sexy::sexy_app_base::FUN_004891c0(g, app, 0);
        g.dialog(this).ext_0x130 = false;
    }
}

/// port: 00476dd0 Sexy::Dialog::vfunction75_for_Widget
/// `IsModal()`.
pub fn vfunction75_for_Widget(g: &mut G, this: Ptr) -> bool {
    g.dialog(this).ext_0x140
}

/// port: 00476de0 Sexy::Dialog::vfunction76_for_Widget
/// `WaitForResult(bool autoKill)`: runs the app's message loop until the dialog has a
/// result. A blocking loop inside game code cannot run inside a Bevy system; no dialog the
/// game shows on its path so far waits like this.
pub fn vfunction76_for_Widget(_g: &mut G, _this: Ptr, _auto_kill: bool) -> i32 {
    crate::sexy::pending(0x00476de0, "Dialog::WaitForResult (blocking modal loop)")
}

/// port: 00476e40 Sexy::Dialog::vfunction2_for_ButtonListener
/// `ButtonPress(int theId)`: `mDialogListener->DialogButtonPress(mId, theId)` for its buttons.
pub fn vfunction2_for_ButtonListener(g: &mut G, this: Ptr, id: i32) {
    if id == 1000 || id == 0x3e9 {
        let (l, did) = (g.dialog(this).field_0x0, g.dialog(this).ext_0x13c);
        crate::sexy::sexy_app_base::vfunction1(g, l, did, id);
    }
}

/// port: 00476e70 Sexy::Dialog::vfunction3_for_ButtonListener
/// `ButtonDepress(int theId)`: the result, then `DialogButtonDepress(mId, theId)`.
pub fn vfunction3_for_ButtonListener(g: &mut G, this: Ptr, id: i32) {
    if id == 1000 || id == 0x3e9 {
        g.dialog(this).ext_0x144 = id;
        let (l, did) = (g.dialog(this).field_0x0, g.dialog(this).ext_0x13c);
        crate::sexy::sexy_app_base::vfunction2(g, l, did, id);
    }
}
