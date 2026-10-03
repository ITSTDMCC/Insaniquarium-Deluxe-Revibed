//! `Sexy::RegisterDialog`: "Register Insaniquarium" (dialog 2), the trial version's
//! registration form: a "CLICK HERE TO GET REGISTRATION CODE" button, the name and code
//! fields (Tab between them, Enter moves on / submits), "Register" / "Cancel". Plus the app
//! functions that open it and the other trial-version dialogs.
//!
//! The object is 0x168 bytes: MoneyDialog (0x158), the EditListener vftable at +0x158, then
//! `RegisterDialog_data` at +0x15c.

use crate::game::money_dialog::MoneyDialog_data;
use crate::sexy::dialog::{alloc_dialog, DlgSub, MoneySub};
use crate::sexy::graphics::{FUN_00455880, FUN_00455890, FUN_00455cf0};
use crate::sexy::prelude::*;
use crate::sexy::types::FUN_00433320;

/// `RegisterDialog_data` (object offset 0x15c).
#[derive(Debug, Clone, Default)]
pub struct RegisterDialog_data {
    /// +0x15c the name field (EditWidget, id 0).
    pub offset_0x0: Ptr,
    /// +0x160 the code field (EditWidget, id 1).
    pub offset_0x4: Ptr,
    /// +0x164 "CLICK HERE TO GET REGISTRATION CODE" (id 0).
    pub offset_0x8: Ptr,
}

impl G {
    pub fn register_dialog(&mut self, p: Ptr) -> &mut RegisterDialog_data {
        match &mut self.widget(p).ext {
            WExt::Dialog(e) => match &mut e.sub {
                DlgSub::Money(_, MoneySub::Register(d)) => d,
                s => panic!("{p} is not a RegisterDialog: {s:?}"),
            },
            e => panic!("{p} is not a Dialog: {e:?}"),
        }
    }
}

/// port: 00535a40 FUN_00535a40
/// `RegisterDialog(WinFishApp*)`: modal, "Register" / "Cancel"; the link-style button and
/// the two fields (white text), each other's tab neighbours.
pub fn FUN_00535a40(g: &mut G, param_1: Ptr) -> Ptr {
    let this = alloc_dialog(
        g,
        &crate::sexy::vtables_gen::Sexy__RegisterDialog_vftable,
        DlgSub::Money(MoneyDialog_data::default(), MoneySub::Register(RegisterDialog_data::default())),
    );
    let (comp, btn) = (g.res.DAT_005e8cc0, g.res.DAT_005e8c54);
    crate::game::money_dialog::MoneyDialog(
        g,
        this,
        param_1,
        comp,
        btn,
        2,
        true,
        b"Register Insaniquarium",
        b"You must obtain a registration code online and enter it below.",
        b"",
        2,
    );
    g.money(this).offset_0x50 = param_1;
    let (yes, no) = (g.dialog(this).offset_0x8, g.dialog(this).offset_0xc);
    g.btn(yes).field_0x4 = b"Register".to_vec();
    g.btn(no).field_0x4 = b"Cancel".to_vec();
    let link = crate::game::game_selector::FUN_00506760(g, 0, this, b"CLICK HERE TO GET REGISTRATION CODE", NULL);
    g.register_dialog(this).offset_0x8 = link;
    let name = crate::game::money_dialog::FUN_00500ad0(g, 0, this);
    g.register_dialog(this).offset_0x0 = name;
    vcall!(g, name, w.vfunction35, 2, FUN_00433320(0xffffff));
    let code = crate::game::money_dialog::FUN_00500ad0(g, 1, this);
    g.register_dialog(this).offset_0x4 = code;
    vcall!(g, code, w.vfunction35, 2, FUN_00433320(0xffffff));
    g.w(name).offset_0x2c = code;
    g.w(name).offset_0x30 = code;
    g.w(code).offset_0x2c = name;
    g.w(code).offset_0x30 = name;
    this
}

/// port: 00531410 Sexy::RegisterDialog::~RegisterDialog
pub fn dtor_RegisterDialog(g: &mut G, this: Ptr) {
    let d = g.register_dialog(this).clone();
    for p in [d.offset_0x0, d.offset_0x4, d.offset_0x8] {
        if p != NULL {
            vcall!(g, p, w.vfunction1, 1);
        }
    }
    crate::game::money_dialog::dtor_MoneyDialog(g, this);
}

/// port: 00532b30 Sexy::RegisterDialog::deleting_destructor
pub fn deleting_destructor(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_RegisterDialog(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 005314b0 Sexy::RegisterDialog::vfunction41
/// `Resize(x, y, w, h)`: the button under the text, the name field below it, the code
/// field 40 pixels under that.
pub fn vfunction41(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32, param_4: i32) {
    crate::sexy::dialog::vfunction41_for_Widget(g, this, param_1, param_2, param_3, param_4);
    let d = g.register_dialog(this).clone();
    vcall!(g, d.offset_0x8, w.vfunction41, param_1 + 0x24, param_2 + 0x5a, param_3 - 0x4b, 0x21);
    let y = param_2 + 0x87 + g.wc(d.offset_0x8).offset_0x38;
    vcall!(g, d.offset_0x0, w.vfunction41, param_1 + 0x34, y, param_3 - 0x6b, 0x18);
    let y2 = g.wc(d.offset_0x0).offset_0x38 + 0x28 + y;
    vcall!(g, d.offset_0x4, w.vfunction41, param_1 + 0x34, y2, param_3 - 0x6b, 0x18);
}

/// port: 00531550 Sexy::RegisterDialog::vfunction74
/// `GetPreferredHeight(int theWidth)`: the dialog's, plus 190 pixels for the fields.
pub fn vfunction74(g: &mut G, this: Ptr, param_1: i32) -> i32 {
    crate::sexy::dialog::vfunction74_for_Widget(g, this, param_1) + 0xbe
}

/// port: 00531570 Sexy::RegisterDialog::vfunction21
/// `AddedToManager(WidgetManager*)`: the button and fields; the name field has the focus.
pub fn vfunction21(g: &mut G, this: Ptr, param_1: Ptr) {
    crate::sexy::dialog::vfunction21_for_Widget(g, this, param_1);
    let d = g.register_dialog(this).clone();
    vcall!(g, param_1, w.vfunction4, d.offset_0x8);
    vcall!(g, param_1, w.vfunction4, d.offset_0x0);
    vcall!(g, param_1, w.vfunction4, d.offset_0x4);
    vcall!(g, param_1, w.vfunction9, d.offset_0x0);
}

/// port: 005315d0 Sexy::RegisterDialog::vfunction22
/// `RemovedFromManager(WidgetManager*)`.
pub fn vfunction22(g: &mut G, this: Ptr, param_1: Ptr) {
    crate::sexy::dialog::vfunction22_for_Widget(g, this, param_1);
    let d = g.register_dialog(this).clone();
    vcall!(g, param_1, w.vfunction5, d.offset_0x8);
    vcall!(g, param_1, w.vfunction5, d.offset_0x4);
    vcall!(g, param_1, w.vfunction5, d.offset_0x0);
}

/// port: 00535cc0 Sexy::RegisterDialog::vfunction27
/// `Draw(Graphics*)`: the dialog, "YOUR NAME" and "YOUR REGISTRATION CODE" in yellow over
/// the fields, and the fields' frames.
pub fn vfunction27(g: &mut G, this: Ptr, param_1: &mut Graphics) {
    crate::sexy::dialog::vfunction27_for_Widget(g, this, param_1);
    let f = g.res.DAT_005e8a5c;
    FUN_00455880(param_1, f);
    FUN_00455890(param_1, FUN_00433320(0xffff00));
    let d = g.register_dialog(this).clone();
    let (mx, my) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
    let (nx, ny) = (g.wc(d.offset_0x0).offset_0x2c, g.wc(d.offset_0x0).offset_0x30);
    FUN_00455cf0(param_1, g, b"YOUR NAME", nx - mx, (ny - my) - 10);
    let (cx, cy) = (g.wc(d.offset_0x4).offset_0x2c, g.wc(d.offset_0x4).offset_0x30);
    FUN_00455cf0(param_1, g, b"YOUR REGISTRATION CODE", cx - mx, (cy - my) - 10);
    crate::game::money_dialog::FUN_00503470(g, param_1, d.offset_0x0);
    crate::game::money_dialog::FUN_00503470(g, param_1, d.offset_0x4);
}

/// port: 00531640 Sexy::RegisterDialog::vfunction1
/// `EditWidgetText(int id, const string&)` (Enter): from the name field the focus moves to
/// the code field; from the code field it is "Register" (`ButtonDepress(1000)`).
pub fn vfunction1(g: &mut G, this: Ptr, param_1: i32, _param_2: &[u8]) {
    if param_1 == 0 {
        let wm = g.wc(this).offset_0xc;
        let code = g.register_dialog(this).offset_0x4;
        vcall!(g, wm, w.vfunction9, code);
        return;
    }
    if param_1 == 1 {
        vcall!(g, this, bl.vfunction3, 1000);
    }
}

/// port: 00531680 Sexy::RegisterDialog::vfunction3
/// `ButtonDepress(int id)`: the link button (0) opens the registration page (the app's
/// `vfunction101`); the rest as any game dialog.
pub fn vfunction3(g: &mut G, this: Ptr, param_1: i32) {
    if param_1 == 0 {
        let app = g.money(this).offset_0x50;
        crate::sexy::app_host::open_registration_page(g, app);
        return;
    }
    crate::game::money_dialog::vfunction3_for_ButtonListener(g, this, param_1);
}

/// port: 00531620 FUN_00531620
/// The name typed.
pub fn FUN_00531620(g: &mut G, this: Ptr) -> Vec<u8> {
    let e = g.register_dialog(this).offset_0x0;
    g.edit(e).field_0x4.clone()
}

/// port: 00531630 FUN_00531630
/// The registration code typed.
pub fn FUN_00531630(g: &mut G, this: Ptr) -> Vec<u8> {
    let e = g.register_dialog(this).offset_0x4;
    g.edit(e).field_0x4.clone()
}

/// port: 0054e8a0 FUN_0054e8a0
/// `ShowRegisterDialog()`: once loading is far enough along, the registration form (dialog
/// 2, 420 wide at y 32), or "Already Registered".
pub fn FUN_0054e8a0(g: &mut G, this: Ptr) {
    if !crate::game::win_fish_app::FUN_0054b590(g, this) {
        return;
    }
    if !g.app_obj(this).sa.offset_0xe4 {
        let d = FUN_00535a40(g, this);
        let h = vcall!(g, d, dlg.vfunction74, 0x1a4);
        let aw = g.sab(this).field_0xb8;
        vcall!(g, d, w.vfunction41, (aw - 0x1a4) / 2, 0x20, 0x1a4, h);
        crate::sexy::sexy_app_base::vfunction76(g, this, 2, d);
    } else {
        crate::game::win_fish_app::vfunction73(g, this, 0xe, true, b"Already Registered", b"You have already registered Insaniquarium.", b"OK", 3);
    }
}

/// port: 0054ea40 FUN_0054ea40
/// `ShowTrialExpired()`: "PLEASE REGISTER!" (dialog 3) with "Register" / "Quit".
pub fn FUN_0054ea40(g: &mut G, this: Ptr) {
    let d = crate::game::win_fish_app::vfunction73(
        g,
        this,
        3,
        true,
        b"PLEASE REGISTER!",
        b"Your trial version of Insaniquarium has expired!\n\nYou must register your copy\nto continue playing.",
        b"",
        2,
    );
    let (yes, no) = (g.dialog(d).offset_0x8, g.dialog(d).offset_0xc);
    g.btn(yes).field_0x4 = b"Register".to_vec();
    g.btn(no).field_0x4 = b"Quit".to_vec();
}

/// port: 0054edc0 FUN_0054edc0
/// "Invalid Code" (dialog 7).
pub fn FUN_0054edc0(g: &mut G, this: Ptr) {
    crate::game::win_fish_app::vfunction73(
        g,
        this,
        7,
        true,
        b"Invalid Code",
        b"The license code you entered is not valid for that name.\n\nMake sure the name and registration number are entered correctly.",
        b"OK",
        3,
    );
}
