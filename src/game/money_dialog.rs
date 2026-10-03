//! `Sexy::MoneyDialog`, the game's Dialog base (fonts, colors and three-cel buttons of the
//! game's look, a click sound, optional delayed button enabling), and `Sexy::NewUserDialog`,
//! the "NEW USER / Please enter your name." dialog with its name field.
//!
//! The database places `MoneyDialog_data` at 0x100 (over Dialog's own fields past its
//! 0x100-byte type); MoneyDialog's own fields are its `offset_0x50` (+0x150) and
//! `offset_0x54` (+0x154). NewUserDialog adds an EditListener part at +0x158 and
//! `NewUserDialog_data` at +0x15c.

use crate::sexy::dialog::{alloc_dialog, DlgSub, MoneySub};
use crate::sexy::prelude::*;

/// MoneyDialog's own fields (+0x150, database `MoneyDialog_data.offset_0x50/0x54`).
#[derive(Debug, Clone, Default)]
pub struct MoneyDialog_data {
    /// +0x150: the app.
    pub offset_0x50: Ptr,
    /// +0x154: the buttons stay disabled until `mUpdateCnt` reaches this (-1 = enabled).
    pub offset_0x54: i32,
}

/// `NewUserDialog_data` (object offset 0x15c).
#[derive(Debug, Clone, Default)]
pub struct NewUserDialog_data {
    /// +0x15c: the app (the EditListener part's owner pointer).
    pub field_0x0: Ptr,
    /// +0x160: the name field (EditWidget).
    pub offset_0x4: Ptr,
}

/// `FishNamingDialog`'s bytes past the database's class size (object offset 0x164; its
/// `FishNamingDialog_data` at 0x15c is the app and the name field, as NewUserDialog's).
#[derive(Debug, Clone, Default)]
pub struct FishNamingDialog_ext {
    /// +0x164 (0x3039).
    pub ext_0x164: i32,
    /// +0x168 show the money-back note.
    pub ext_0x168: bool,
}

impl G {
    pub fn fish_naming(&mut self, p: Ptr) -> &mut FishNamingDialog_ext {
        match &mut self.widget(p).ext {
            WExt::Dialog(e) => match &mut e.sub {
                DlgSub::Money(_, MoneySub::FishNaming(_, f)) => f,
                s => panic!("{p} is not a FishNamingDialog: {s:?}"),
            },
            e => panic!("{p} is not a Dialog: {e:?}"),
        }
    }
    pub fn money(&mut self, p: Ptr) -> &mut MoneyDialog_data {
        match &mut self.widget(p).ext {
            WExt::Dialog(e) => match &mut e.sub {
                DlgSub::Money(m, _) => m,
                s => panic!("{p} is not a MoneyDialog: {s:?}"),
            },
            e => panic!("{p} is not a Dialog: {e:?}"),
        }
    }
    pub fn new_user(&mut self, p: Ptr) -> &mut NewUserDialog_data {
        match &mut self.widget(p).ext {
            WExt::Dialog(e) => match &mut e.sub {
                DlgSub::Money(_, MoneySub::NewUser(n)) => n,
                DlgSub::Money(_, MoneySub::FishNaming(n, _)) => n,
                s => panic!("{p} is not a NewUserDialog: {s:?}"),
            },
            e => panic!("{p} is not a Dialog: {e:?}"),
        }
    }
}

/// port: 00530920 Sexy::MoneyDialog::MoneyDialog
/// `MoneyDialog(WinFishApp*, Image* theComponentImage, Image* theButtonImage, int theId,
/// bool isModal, header, lines, footer, int theButtonMode)` on an allocated object.
pub fn MoneyDialog(
    g: &mut G,
    this: Ptr,
    param_1: Ptr,
    param_2: Ptr,
    param_3: Ptr,
    param_4: i32,
    param_5: bool,
    param_6: &[u8],
    param_7: &[u8],
    param_8: &[u8],
    param_9: i32,
) {
    crate::sexy::dialog::Dialog(g, this, param_2, param_3, param_4, param_5, param_6, param_7, param_8, param_9);
    let m = g.money(this);
    m.offset_0x50 = param_1;
    m.offset_0x54 = -1;
    FUN_00504060(g, this);
}

/// port: 00504060 FUN_00504060
/// The game's dialog look: fonts, content insets, yellow header / white lines / pale yellow
/// button text, 15 pixels after the header, three-cel buttons.
pub fn FUN_00504060(g: &mut G, param_1: Ptr) {
    let (bf, hf, lf) = (g.res.DAT_005e8a5c, g.res.DAT_005e8e18, g.res.DAT_005e8cf0);
    vcall!(g, param_1, dlg.vfunction71, bf);
    vcall!(g, param_1, dlg.vfunction72, hf);
    vcall!(g, param_1, dlg.vfunction73, lf);
    let d = g.dialog(param_1);
    d.ext_0x11c = 0x24;
    d.ext_0x120 = 0xf;
    d.ext_0x124 = 0x24;
    d.ext_0x128 = 0x24;
    let c = FUN_00433360(0xff, 200, 0);
    vcall!(g, param_1, w.vfunction35, 0, c);
    let c = FUN_00433360(0xff, 0xff, 0xff);
    vcall!(g, param_1, w.vfunction35, 1, c);
    let c = FUN_00433360(0xff, 0xff, 100);
    vcall!(g, param_1, w.vfunction35, 3, c);
    g.dialog(param_1).ext_0x12c = 0xf;
    let (yes, no) = (g.dialog(param_1).offset_0x8, g.dialog(param_1).offset_0xc);
    if yes != NULL {
        crate::game::game_selector::FUN_005034e0(g, yes);
    }
    if no != NULL {
        crate::game::game_selector::FUN_005034e0(g, no);
    }
}

/// port: 005309c0 Sexy::MoneyDialog::~MoneyDialog
pub fn dtor_MoneyDialog(g: &mut G, this: Ptr) {
    crate::sexy::dialog::dtor_Dialog(g, this);
}

/// port: 005327c0 Sexy::MoneyDialog::deleting_destructor_for_Widget
pub fn deleting_destructor_for_Widget(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_MoneyDialog(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 00530a30 Sexy::MoneyDialog::vfunction77_for_Widget
/// Disables both buttons until `mUpdateCnt` reaches `param_1`.
pub fn vfunction77_for_Widget(g: &mut G, this: Ptr, param_1: i32) {
    g.money(this).offset_0x54 = param_1;
    let (yes, no) = (g.dialog(this).offset_0x8, g.dialog(this).offset_0xc);
    if yes != NULL {
        vcall!(g, yes, w.vfunction38, true);
    }
    if no != NULL {
        vcall!(g, no, w.vfunction38, true);
    }
}

/// port: 00530a70 Sexy::MoneyDialog::vfunction23_for_Widget
/// `Update()`: re-enables the buttons on the update count set by #77.
pub fn vfunction23_for_Widget(g: &mut G, this: Ptr) {
    crate::sexy::widget_container::vfunction23(g, this);
    if g.wc(this).offset_0x24 == g.money(this).offset_0x54 {
        let (yes, no) = (g.dialog(this).offset_0x8, g.dialog(this).offset_0xc);
        if yes != NULL {
            vcall!(g, yes, w.vfunction38, false);
        }
        if no != NULL {
            vcall!(g, no, w.vfunction38, false);
        }
    }
}

/// port: 00530ac0 Sexy::MoneyDialog::vfunction2_for_ButtonListener
/// `ButtonPress(int)`: the click sound (app vtable +0xd8 `PlaySample`).
pub fn vfunction2_for_ButtonListener(g: &mut G, this: Ptr, _id: i32) {
    let app = g.money(this).offset_0x50;
    let s = g.res.DAT_005e8c40;
    crate::sexy::sexy_app_base::vfunction55(g, app, s);
}

/// port: 00530ae0 Sexy::MoneyDialog::vfunction3_for_ButtonListener
/// `ButtonDepress(int)`: ignored until the buttons have been enabled.
pub fn vfunction3_for_ButtonListener(g: &mut G, this: Ptr, id: i32) {
    if g.money(this).offset_0x54 < g.wc(this).offset_0x24 {
        crate::sexy::dialog::vfunction3_for_ButtonListener(g, this, id);
    }
}

/// port: 00530b00 Sexy::MoneyDialog::vfunction78_for_Widget
/// `CheckboxChecked(int id, bool checked)` of the game's dialogs (`RET 8`: both
/// arguments ignored): the click sound.
pub fn vfunction78_for_Widget(g: &mut G, this: Ptr, _param_1: i32, _param_2: bool) {
    let app = g.money(this).offset_0x50;
    let s = g.res.DAT_005e8c40;
    crate::sexy::sexy_app_base::vfunction55(g, app, s);
}

/// port: 00534740 FUN_00534740
/// `NewUserDialog(WinFishApp*, bool isRename)`: dialog 0x19 ("NEW USER") or 0x1b ("RENAME
/// USER"), OK/Cancel, with a 12-character name field that must also fit 150 pixels of
/// FONT_... (`DAT_005e8cf0`) and 220 of `DAT_005e8a28`.
pub fn FUN_00534740(g: &mut G, param_1: Ptr, param_2: bool) -> Ptr {
    let this = alloc_dialog(
        g,
        &crate::sexy::vtables_gen::Sexy__NewUserDialog_vftable,
        DlgSub::Money(MoneyDialog_data::default(), MoneySub::NewUser(NewUserDialog_data::default())),
    );
    let header: &[u8] = if param_2 { b"RENAME USER" } else { b"NEW USER" };
    let (comp, btn) = (g.res.DAT_005e8cc0, g.res.DAT_005e8c54);
    let id = (param_2 as i32) * 2 + 0x19;
    MoneyDialog(g, this, param_1, comp, btn, id, true, header, b"\nPlease enter your name.", b"", 2);
    g.new_user(this).field_0x0 = param_1;
    let (bf, hf, lf) = (g.res.DAT_005e8a5c, g.res.DAT_005e8e18, g.res.DAT_005e8cf0);
    crate::sexy::dialog::vfunction71_for_Widget(g, this, bf);
    crate::sexy::dialog::vfunction72_for_Widget(g, this, hf);
    crate::sexy::dialog::vfunction73_for_Widget(g, this, lf);
    let d = g.dialog(this);
    d.ext_0x11c = 0x24;
    d.ext_0x120 = 0xf;
    d.ext_0x124 = 0x24;
    d.ext_0x128 = 0x24;
    crate::sexy::dialog::vfunction35_for_Widget(g, this, 0, FUN_00433360(0xff, 200, 0));
    crate::sexy::dialog::vfunction35_for_Widget(g, this, 1, FUN_00433360(0xff, 0xff, 0xff));
    crate::sexy::dialog::vfunction35_for_Widget(g, this, 3, FUN_00433360(0xff, 0xff, 0xff));
    let edit = FUN_00500ad0(g, 0, this);
    g.new_user(this).offset_0x4 = edit;
    g.edit(edit).offset_0x68 = 0xc;
    let f1 = g.res.DAT_005e8cf0;
    crate::sexy::edit_widget::FUN_00472e30(g, edit, f1, 0x96);
    let f2 = g.res.DAT_005e8a28;
    crate::sexy::edit_widget::FUN_00472e30(g, edit, f2, 0xdc);
    this
}

/// port: 00500ad0 FUN_00500ad0
/// `MakeEditWidget(int theId, EditListener*)`: the game's edit field look (font
/// `DAT_005e8dac`, transparent background, faster blink).
pub fn FUN_00500ad0(g: &mut G, param_1: i32, param_2: Ptr) -> Ptr {
    let e = crate::sexy::edit_widget::EditWidget(g, param_1, param_2);
    let f = g.res.DAT_005e8dac;
    vcall!(g, e, edit.vfunction73, f, NULL);
    // `gGameEditColors` (@ 005dfe68, 5 x RGBA).
    let colors: [[i32; 4]; 5] = [[0, 0, 0, 0], [0, 0, 0, 0], [240, 240, 255, 255], [255, 255, 255, 255], [0, 0, 0, 255]];
    vcall!(g, e, w.vfunction33, &colors);
    g.edit(e).offset_0x60 = 0xe;
    e
}

/// port: 00530be0 Sexy::NewUserDialog::~NewUserDialog
pub fn dtor_NewUserDialog(g: &mut G, this: Ptr) {
    let edit = g.new_user(this).offset_0x4;
    if edit != NULL {
        vcall!(g, edit, w.vfunction1, 1);
    }
    dtor_MoneyDialog(g, this);
}

/// port: 005327f0 Sexy::NewUserDialog::deleting_destructor
pub fn deleting_destructor(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_NewUserDialog(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 00530010 Sexy::NewUserDialog::vfunction74
/// `GetPreferredHeight(int)`: room for the name field.
pub fn vfunction74(g: &mut G, this: Ptr, width: i32) -> i32 {
    crate::sexy::dialog::vfunction74_for_Widget(g, this, width) + 0x28
}

/// port: 00530220 Sexy::NewUserDialog::vfunction21
/// `AddedToManager(WidgetManager*)`: adds the name field and focuses it.
pub fn vfunction21(g: &mut G, this: Ptr, manager: Ptr) {
    crate::sexy::dialog::vfunction21_for_Widget(g, this, manager);
    let edit = g.new_user(this).offset_0x4;
    vcall!(g, manager, w.vfunction4, edit);
    vcall!(g, manager, w.vfunction9, edit);
}

/// port: 00530260 Sexy::NewUserDialog::vfunction22
/// `RemovedFromManager(WidgetManager*)`.
pub fn vfunction22(g: &mut G, this: Ptr, manager: Ptr) {
    crate::sexy::dialog::vfunction22_for_Widget(g, this, manager);
    let edit = g.new_user(this).offset_0x4;
    vcall!(g, manager, w.vfunction5, edit);
}

/// port: 00530c60 Sexy::NewUserDialog::vfunction41
/// `Resize(int x, int y, int w, int h)`: the name field 48 pixels in, 110 above the bottom.
pub fn vfunction41(g: &mut G, this: Ptr, x: i32, y: i32, w: i32, h: i32) {
    crate::sexy::dialog::vfunction41_for_Widget(g, this, x, y, w, h);
    let edit = g.new_user(this).offset_0x4;
    let (mx, my, mw, mh) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30, g.wc(this).offset_0x34, g.wc(this).offset_0x38);
    vcall!(g, edit, w.vfunction41, mx + 0x30, mh + -0x6e + my, mw + -0x60, 0x18);
}

/// port: 00530cb0 Sexy::NewUserDialog::vfunction27
/// `Draw(Graphics*)`: the dialog, then the field's frame.
pub fn vfunction27(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    crate::sexy::dialog::vfunction27_for_Widget(g, this, gfx);
    let edit = g.new_user(this).offset_0x4;
    FUN_00503470(g, gfx, edit);
}

/// port: 00503470 FUN_00503470
/// Draws the edit-field frame (`DAT_005e8dbc` as a 9-slice) 8/4 pixels around a widget, in
/// the coordinates of a Graphics translated to the widget's parent.
pub fn FUN_00503470(g: &mut G, gfx: &mut Graphics, param_2: Ptr) {
    let (x, y, w, h) = (g.wc(param_2).offset_0x2c, g.wc(param_2).offset_0x30, g.wc(param_2).offset_0x34, g.wc(param_2).offset_0x38);
    let rx = crate::sexy::crt::ftol((x - 8) as f64 - gfx.s.mTransX as f64) as i32;
    let ry = crate::sexy::crt::ftol((y - 4) as f64 - gfx.s.mTransY as f64) as i32;
    let img = g.res.DAT_005e8dbc;
    crate::sexy::graphics::FUN_004563f0(gfx, g, &Rect::new(rx, ry, w + 0x10, h + 8), img);
}

/// port: 00530ce0 Sexy::NewUserDialog::vfunction1
/// `EditWidgetText(int, const string&)` (Enter in the field): the app's `ButtonDepress(mId +
/// 2000)`, as if OK were pressed (app vtable +0x8).
pub fn vfunction1(g: &mut G, this: Ptr, _id: i32, _text: &[u8]) {
    let app = g.new_user(this).field_0x0;
    let did = g.dialog(this).ext_0x13c;
    crate::game::win_fish_app::vfunction3(g, app, did + 2000);
}

/// port: 00530d00 Sexy::NewUserDialog::vfunction3
/// `AllowChar(int, char)`: letters, digits and space (`_isalnum` on the sign-extended char;
/// bytes from 0x80 are outside the C locale's table and the edit widget never offers them).
pub fn vfunction3(_g: &mut G, _this: Ptr, _id: i32, c: u8) -> bool {
    ((c as i8) >= 0 && c.is_ascii_alphanumeric()) || c == b' '
}

/// port: 005333c0 FUN_005333c0
/// `NewUserDialog::GetName()`: the field's text with runs of spaces collapsed to one and a
/// trailing space removed.
pub fn FUN_005333c0(g: &mut G, this: Ptr) -> Vec<u8> {
    let edit = g.new_user(this).offset_0x4;
    let s = g.edit(edit).field_0x4.clone();
    let mut out: Vec<u8> = Vec::new();
    let mut prev = b' ';
    for &c in &s {
        if c != b' ' || prev != b' ' {
            out.push(c);
        }
        prev = c;
    }
    if !out.is_empty() && out[out.len() - 1] == b' ' {
        out.truncate(out.len() - 1);
    }
    out
}

/// port: 00533a40 FUN_00533a40
/// `FishNamingDialog(WinFishApp*, const string& theLines)`: dialog 0xf "Name Your Fish",
/// OK/Cancel, with a 12-character name field at most 170 pixels wide (and fitting
/// `DAT_005e8e18`).
pub fn FUN_00533a40(g: &mut G, param_1: Ptr, param_2: &[u8]) -> Ptr {
    let this = alloc_dialog(
        g,
        &crate::sexy::vtables_gen::Sexy__FishNamingDialog_vftable,
        DlgSub::Money(MoneyDialog_data::default(), MoneySub::FishNaming(NewUserDialog_data::default(), FishNamingDialog_ext::default())),
    );
    let (comp, btn) = (g.res.DAT_005e8cc0, g.res.DAT_005e8c54);
    MoneyDialog(g, this, param_1, comp, btn, 0xf, true, b"Name Your Fish", param_2, b"", 2);
    g.new_user(this).field_0x0 = param_1;
    g.fish_naming(this).ext_0x164 = 0x3039;
    let (bf, hf, lf) = (g.res.DAT_005e8a5c, g.res.DAT_005e8e18, g.res.DAT_005e8cf0);
    crate::sexy::dialog::vfunction71_for_Widget(g, this, bf);
    crate::sexy::dialog::vfunction72_for_Widget(g, this, hf);
    crate::sexy::dialog::vfunction73_for_Widget(g, this, lf);
    let d = g.dialog(this);
    d.ext_0x11c = 0x24;
    d.ext_0x120 = 0xf;
    d.ext_0x124 = 0x24;
    d.ext_0x128 = 0x24;
    crate::sexy::dialog::vfunction35_for_Widget(g, this, 0, FUN_00433360(0xff, 200, 0));
    crate::sexy::dialog::vfunction35_for_Widget(g, this, 1, FUN_00433360(0xff, 0xff, 0xff));
    crate::sexy::dialog::vfunction35_for_Widget(g, this, 3, FUN_00433360(0xff, 0xff, 0xff));
    let edit = FUN_00500ad0(g, 0, this);
    g.new_user(this).offset_0x4 = edit;
    g.edit(edit).offset_0x68 = 0xc;
    g.edit(edit).offset_0x6c = 0xaa;
    let f = g.res.DAT_005e8e18;
    crate::sexy::edit_widget::FUN_00472e30(g, edit, f, -1);
    this
}

/// port: 00532580 Sexy::FishNamingDialog::~FishNamingDialog
/// Takes the name field off the widget manager, deletes it, then the MoneyDialog.
pub fn dtor_FishNamingDialog(g: &mut G, this: Ptr) {
    let edit = g.new_user(this).offset_0x4;
    if edit != NULL {
        let app = g.new_user(this).field_0x0;
        let wm = g.sab(app).offset_0x318;
        vcall!(g, wm, w.vfunction5, edit);
    }
    let edit = g.new_user(this).offset_0x4;
    if edit != NULL {
        vcall!(g, edit, w.vfunction1, 1);
    }
    dtor_MoneyDialog(g, this);
}

/// port: 00532d70 Sexy::FishNamingDialog::deleting_destructor
pub fn deleting_destructor__00532d70(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_FishNamingDialog(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 005301f0 Sexy::FishNamingDialog::vfunction74
/// `GetPreferredHeight(int)`: room for the name field, and the note when shown.
pub fn vfunction74__005301f0(g: &mut G, this: Ptr, param_1: i32) -> i32 {
    let h = crate::sexy::dialog::vfunction74_for_Widget(g, this, param_1);
    if g.fish_naming(this).ext_0x168 { h + 0x4c } else { h + 0x38 }
}

/// port: 00530290 Sexy::FishNamingDialog::vfunction41
/// `Resize(int x, int y, int w, int h)`: the name field 48 pixels in, 80 down.
pub fn vfunction41__00530290(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32, param_4: i32) {
    crate::sexy::dialog::vfunction41_for_Widget(g, this, param_1, param_2, param_3, param_4);
    let edit = g.new_user(this).offset_0x4;
    let (mx, my, mw) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30, g.wc(this).offset_0x34);
    vcall!(g, edit, w.vfunction41, mx + 0x30, my + 0x50, mw - 0x60, 0x18);
}

/// port: 00533c60 Sexy::FishNamingDialog::vfunction27
/// `Draw(Graphics*)`: the dialog, the field's frame, and the money-back note under it.
pub fn vfunction27__00533c60(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    crate::sexy::dialog::vfunction27_for_Widget(g, this, gfx);
    let edit = g.new_user(this).offset_0x4;
    FUN_00503470(g, gfx, edit);
    if !g.fish_naming(this).ext_0x168 {
        return;
    }
    let d = g.dialog(this).clone();
    let w = g.wc(this).offset_0x34;
    let r = Rect::new(d.ext_0x10c + 2 + d.ext_0x11c, 0x78, w - d.ext_0x124 - d.ext_0x114 - d.ext_0x10c - d.ext_0x11c - 4, 0);
    let spacing = crate::sexy::image_font::get_line_spacing(g, d.offset_0x70) + d.ext_0x104;
    vcall!(g, this, w.vfunction65, gfx, r, b"Note: All fish come with a one day\nmoney back guarantee.", spacing, d.ext_0x100);
}

/// port: 005302e0 Sexy::FishNamingDialog::vfunction3
/// `AllowChar(int, char)`: printable characters and space (`_isprint` on the zero-extended
/// byte; the C locale's table marks only 0x20..0x7e).
pub fn vfunction3__005302e0(_g: &mut G, _this: Ptr, _id: i32, c: u8) -> bool {
    (0x20..0x7f).contains(&c) || c == b' '
}
