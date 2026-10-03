//! `Sexy::UserDialog`: "WHO ARE YOU?" (dialog 0x18: the player roster with "(Create a New
//! User)", Rename and Delete) or "TRANSFER SHELLS" (dialog 0x28: the other players and an
//! amount field). Plus the app functions behind its buttons and the roster changes.
//!
//! The object is 0x178 bytes: MoneyDialog (0x158), the ListListener and EditListener
//! vftables (+0x158, +0x15c), `UserDialog_data` at +0x160 and the mode flag at +0x174.

use crate::game::money_dialog::FUN_00500ad0;
use crate::game::money_dialog::MoneyDialog_data;
use crate::sexy::dialog::{alloc_dialog, DlgSub, MoneySub};
use crate::sexy::graphics::{FUN_00455880, FUN_00455890, FUN_00455cf0, FUN_00455d20};
use crate::sexy::prelude::*;
use crate::sexy::types::FUN_00433320;

/// `UserDialog_data` (object offset 0x160) plus the mode flag past it.
#[derive(Debug, Clone, Default)]
pub struct UserDialog_data {
    /// +0x160 the roster (ListWidget).
    pub offset_0x0: Ptr,
    /// +0x164 the transfer amount (EditWidget).
    pub offset_0x4: Ptr,
    /// +0x168 the roster's scrollbar.
    pub offset_0x8: Ptr,
    /// +0x16c "Rename" (id 0).
    pub offset_0xc: Ptr,
    /// +0x170 "Delete" (id 1).
    pub offset_0x10: Ptr,
    /// +0x174 transfer mode.
    pub ext_0x174: bool,
}

/// `gUserListColors` (@ 005df8ac): background, outline, text, hilite, select.
pub const DAT_005df8ac: [[i32; 3]; 5] = [[53, 43, 29], [0, 0, 0], [255, 225, 73], [255, 255, 255], [188, 22, 22]];

impl G {
    pub fn user_dialog(&mut self, p: Ptr) -> &mut UserDialog_data {
        match &mut self.widget(p).ext {
            WExt::Dialog(e) => match &mut e.sub {
                DlgSub::Money(_, MoneySub::User(u)) => u,
                s => panic!("{p} is not a UserDialog: {s:?}"),
            },
            e => panic!("{p} is not a Dialog: {e:?}"),
        }
    }
}

/// port: 005309e0 FUN_005309e0
/// A dialog's content left edge: its x plus the left border and inset.
pub fn FUN_005309e0(g: &mut G, this: Ptr) -> i32 {
    let x = g.wc(this).offset_0x2c;
    let d = g.dialog(this);
    d.ext_0x11c + d.ext_0x10c + x
}

/// port: 00530a10 FUN_00530a10
/// A dialog's content top: its y plus the top border, inset and 54 pixels of header.
pub fn FUN_00530a10(g: &mut G, this: Ptr) -> i32 {
    let y = g.wc(this).offset_0x30;
    let d = g.dialog(this);
    d.ext_0x120 + d.ext_0x110 + 0x36 + y
}

/// port: 005309f0 FUN_005309f0
/// A dialog's content width.
pub fn FUN_005309f0(g: &mut G, this: Ptr) -> i32 {
    let w = g.wc(this).offset_0x34;
    let d = g.dialog(this);
    (((w - d.ext_0x124) - d.ext_0x11c) - d.ext_0x114) - d.ext_0x10c
}

/// port: 00536450 FUN_00536450
/// `UserDialog(WinFishApp*, bool transfer)`: OK/Cancel; the roster (centered lines in the
/// game's colors, 15-pixel rows) with its scrollbar (hidden below 8 profiles), "Rename",
/// "Delete" and the amount field (starting at "0", digits only). Choosing: "(Create a New
/// User)", the current player (selected), then the others in roster order. Transferring:
/// the other players only, no Rename/Delete.
pub fn FUN_00536450(g: &mut G, param_1: Ptr, param_2: bool) -> Ptr {
    let this = alloc_dialog(
        g,
        &crate::sexy::vtables_gen::Sexy__UserDialog_vftable,
        DlgSub::Money(MoneyDialog_data::default(), MoneySub::User(UserDialog_data::default())),
    );
    let header: &[u8] = if param_2 { b"TRANSFER SHELLS" } else { b"WHO ARE YOU?" };
    let (comp, btn) = (g.res.DAT_005e8cc0, g.res.DAT_005e8c54);
    let id = if param_2 { 0x28 } else { 0x18 };
    crate::game::money_dialog::MoneyDialog(g, this, param_1, comp, btn, id, true, header, b"", b"", 2);
    g.user_dialog(this).ext_0x174 = param_2;
    let f = g.res.DAT_005e8dac;
    let list = crate::sexy::list_widget::ListWidget(g, 0, f, this);
    g.user_dialog(this).offset_0x0 = list;
    crate::sexy::widget::vfunction34(g, list, &DAT_005df8ac);
    {
        let l = g.list(list);
        l.ext_0xe5 = true;
        l.field_0xc = 1;
        l.ext_0xec = 0xf;
    }
    let sb = crate::sexy::scrollbar::ScrollbarWidget(g, 0, list);
    g.user_dialog(this).offset_0x8 = sb;
    let f = g.res.DAT_005e8a5c;
    let rename = crate::game::game_selector::FUN_00506760(g, 0, this, b"Rename", f);
    g.user_dialog(this).offset_0xc = rename;
    let delete = crate::game::game_selector::FUN_00506760(g, 1, this, b"Delete", f);
    g.user_dialog(this).offset_0x10 = delete;
    let edit = FUN_00500ad0(g, 0, this);
    g.user_dialog(this).offset_0x4 = edit;
    vcall!(g, edit, edit.vfunction74, b"0", true);
    let len = g.edit(edit).field_0x4.len() as i32;
    g.edit(edit).offset_0x54 = len;
    g.list(list).field_0x8 = sb;
    if !param_2 {
        vcall!(g, list, list.vfunction74, b"(Create a New User)", false);
    }
    let profile = g.wfa(param_1).offset_0x18c;
    if profile != NULL && !param_2 {
        let name = g.profile(profile).field_0x24.clone();
        let i = vcall!(g, list, list.vfunction74, &name, false);
        vcall!(g, list, list.vfunction84, i);
    }
    let mgr = g.wfa(param_1).offset_0x184;
    let entries = g.profile_mgr(mgr).offset_0x4.clone();
    for (_, p) in &entries {
        let name = g.profile(*p).field_0x24.clone();
        if profile == NULL || name != g.profile(profile).field_0x24 {
            vcall!(g, list, list.vfunction74, &name, false);
        }
    }
    if (entries.len() as u32) < 8 {
        vcall!(g, sb, w.vfunction32, false);
    }
    let d = g.user_dialog(this).clone();
    if !d.ext_0x174 {
        vcall!(g, d.offset_0x4, w.vfunction32, false);
    } else {
        vcall!(g, d.offset_0xc, w.vfunction32, false);
        vcall!(g, d.offset_0x10, w.vfunction32, false);
    }
    this
}

/// port: 00531e80 Sexy::UserDialog::~UserDialog
pub fn dtor_UserDialog(g: &mut G, this: Ptr) {
    let d = g.user_dialog(this).clone();
    for p in [d.offset_0x0, d.offset_0x8, d.offset_0xc, d.offset_0x10, d.offset_0x4] {
        if p != NULL {
            vcall!(g, p, w.vfunction1, 1);
        }
    }
    crate::game::money_dialog::dtor_MoneyDialog(g, this);
}

/// port: 00532b90 Sexy::UserDialog::deleting_destructor
pub fn deleting_destructor(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_UserDialog(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 00531f50 Sexy::UserDialog::vfunction41
/// `Resize(x, y, w, h)`: the roster across the content (130 high) with its scrollbar at the
/// right when shown; Rename and Delete over OK and Cancel; the amount field beside OK.
pub fn vfunction41(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32, param_4: i32) {
    crate::sexy::dialog::vfunction41_for_Widget(g, this, param_1, param_2, param_3, param_4);
    let x = FUN_005309e0(g, this) + 10;
    let y = FUN_00530a10(g, this);
    let w = FUN_005309f0(g, this) - 0x14;
    let d = g.user_dialog(this).clone();
    let sbw = if g.w(d.offset_0x8).offset_0x0 { 0x10 } else { 0 };
    let lw = w - sbw;
    vcall!(g, d.offset_0x0, w.vfunction41, x, y, lw, 0x82);
    vcall!(g, d.offset_0x8, scroll.vfunction76, lw + x, y, sbw, 0x82);
    let (yes, no) = (g.dialog(this).offset_0x8, g.dialog(this).offset_0xc);
    crate::sexy::widget::FUN_0046e880(g, d.offset_0xc, 0x1103, yes, 0, 0, 0, 0);
    crate::sexy::widget::FUN_0046e880(g, d.offset_0x10, 0x1103, no, 0, 0, 0, 0);
    crate::sexy::widget::FUN_0046e880(g, d.offset_0x4, 0x1d0, yes, x + 0xaa, -10, w - 0xaa, 0x18);
}

/// port: 00532060 Sexy::UserDialog::vfunction21
/// `AddedToManager(WidgetManager*)`: the roster, scrollbar, buttons and field; the field
/// takes the focus when transferring.
pub fn vfunction21(g: &mut G, this: Ptr, param_1: Ptr) {
    crate::sexy::dialog::vfunction21_for_Widget(g, this, param_1);
    let d = g.user_dialog(this).clone();
    for p in [d.offset_0x0, d.offset_0x8, d.offset_0x10, d.offset_0xc, d.offset_0x4] {
        vcall!(g, param_1, w.vfunction4, p);
    }
    if d.ext_0x174 {
        vcall!(g, param_1, w.vfunction9, d.offset_0x4);
    }
}

/// port: 005320e0 Sexy::UserDialog::vfunction22
/// `RemovedFromManager(WidgetManager*)`.
pub fn vfunction22(g: &mut G, this: Ptr, param_1: Ptr) {
    crate::sexy::dialog::vfunction22_for_Widget(g, this, param_1);
    let d = g.user_dialog(this).clone();
    for p in [d.offset_0x0, d.offset_0x8, d.offset_0x10, d.offset_0xc, d.offset_0x4] {
        vcall!(g, param_1, w.vfunction5, p);
    }
}

/// port: 005321e0 Sexy::UserDialog::vfunction1
/// `ListClicked(int id, int idx, int clickCount)`: transferring selects; else "(Create a
/// New User)" opens the new-user dialog, a name is selected (a double click chooses it).
pub fn vfunction1(g: &mut G, this: Ptr, _id: i32, param_2: i32, param_3: i32) {
    let d = g.user_dialog(this).clone();
    if d.ext_0x174 {
        vcall!(g, d.offset_0x0, list.vfunction84, param_2);
        return;
    }
    let app = g.money(this).offset_0x50;
    if param_2 == 0 {
        crate::game::win_fish_app::FUN_0054b6a0(g, app);
        return;
    }
    vcall!(g, d.offset_0x0, list.vfunction84, param_2);
    if param_3 == 2 {
        FUN_0054ce30(g, app, true);
    }
}

/// port: 00532240 Sexy::UserDialog::vfunction1
/// `EditWidgetText(int, const string&)` (Enter in the amount field): the app's
/// `ButtonDepress(mId + 2000)`, as if OK were pressed.
pub fn vfunction1__00532240(g: &mut G, this: Ptr, _id: i32, _text: &[u8]) {
    let app = g.money(this).offset_0x50;
    let did = g.dialog(this).ext_0x13c;
    crate::game::win_fish_app::vfunction3(g, app, did + 2000);
}

/// port: 00532260 Sexy::UserDialog::vfunction3
/// `AllowChar(int, char)`: digits only.
pub fn vfunction3(_g: &mut G, _this: Ptr, _id: i32, param_2: u8) -> bool {
    param_2.is_ascii_digit()
}

/// port: 00536a40 Sexy::UserDialog::vfunction27
/// `Draw(Graphics*)`: the dialog; transferring, the captions, the shell icon and the
/// field's frame.
pub fn vfunction27(g: &mut G, this: Ptr, param_1: &mut Graphics) {
    crate::sexy::dialog::vfunction27_for_Widget(g, this, param_1);
    let d = g.user_dialog(this).clone();
    if !d.ext_0x174 {
        return;
    }
    let f = g.res.DAT_005e8cf0;
    FUN_00455880(param_1, f);
    FUN_00455890(param_1, FUN_00433320(0xffffff));
    let (mx, my) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
    let y = FUN_00530a10(g, this) + (-7 - my);
    let x = FUN_005309e0(g, this) + (0x46 - mx);
    FUN_00455cf0(param_1, g, b"Transfer Shells to User", x, y);
    let (ex, ey) = (g.wc(d.offset_0x4).offset_0x2c, g.wc(d.offset_0x4).offset_0x30);
    let y = (ey - my) + 0x11;
    let x = FUN_005309e0(g, this) - mx;
    FUN_00455cf0(param_1, g, b"Transfer Amount", x, y);
    let icon = g.res.DAT_005e8b74;
    FUN_00455d20(param_1, g, icon, (ex - mx) + -0x23, (ey - my) + 3);
    crate::game::money_dialog::FUN_00503470(g, param_1, d.offset_0x4);
}

/// port: 00532f40 FUN_00532f40
/// The transfer amount typed (`atol`; negative counts as 0).
pub fn FUN_00532f40(g: &mut G, this: Ptr) -> i32 {
    let e = g.user_dialog(this).offset_0x4;
    let text = g.edit(e).field_0x4.clone();
    let v = crate::sexy::crt::atol(&text);
    if v < 0 { 0 } else { v }
}

/// port: 0054f0e0 FUN_0054f0e0
/// The "give" cheat: the shell transfer dialog (0x28, 400 wide at y 100) once the first
/// tank is beaten, else "Not Allowed".
pub fn FUN_0054f0e0(g: &mut G, this: Ptr) {
    crate::game::win_fish_app::vfunction78(g, this, 0x28);
    let profile = g.wfa(this).offset_0x18c;
    if profile != NULL && (g.profile(profile).field_0x59 || g.profile(profile).field_0x1c != 1) {
        let d = FUN_00536450(g, this, true);
        let h = vcall!(g, d, dlg.vfunction74, 400);
        vcall!(g, d, w.vfunction41, 0, 100, 400, h);
        crate::sexy::sexy_app_base::vfunction76(g, this, 0x28, d);
        return;
    }
    crate::game::win_fish_app::vfunction73(g, this, 0xe, true, b"Not Allowed", b"You need to beat the first tank before you can transfer shells.", b"OK", 3);
}

/// port: 0054f280 FUN_0054f280
/// The transfer dialog closed: on OK the amount moves from the current player to the one
/// chosen (both saved), unless it is more than the player has ("Not Enough Shells") or no
/// one is chosen ("Choose User"), which leave the dialog up; then it closes.
pub fn FUN_0054f280(g: &mut G, this: Ptr, param_1: bool) {
    let d = crate::sexy::sexy_app_base::vfunction74(g, this, 0x28);
    if d == NULL {
        return;
    }
    let profile = g.wfa(this).offset_0x18c;
    if param_1 && profile != NULL {
        let n = FUN_00532f40(g, d);
        if g.profile(profile).field_0x48 < n {
            crate::game::win_fish_app::vfunction73(g, this, 0xe, true, b"Not Enough Shells", b"You don't have that many shells to transfer.", b"OK", 3);
            return;
        }
        let name = FUN_00536990(g, d);
        let mgr = g.wfa(this).offset_0x184;
        let to = crate::game::profile_mgr::FUN_00514eb0(g, mgr, &name);
        if to == NULL {
            crate::game::win_fish_app::vfunction73(g, this, 0xe, true, b"Choose User", b"Please choose a user from the list.", b"OK", 3);
            return;
        }
        if to != g.wfa(this).offset_0x18c {
            crate::game::profile::FUN_005145a0(g, to);
            crate::game::profile::FUN_00501200(g, to, n);
            let cur = g.wfa(this).offset_0x18c;
            crate::game::profile::FUN_00501200(g, cur, -n);
            crate::game::profile::FUN_00514730(g, to);
            let cur = g.wfa(this).offset_0x18c;
            crate::game::profile::FUN_00514730(g, cur);
        }
    }
    crate::game::win_fish_app::vfunction78(g, this, 0x28);
}

/// port: 00536990 FUN_00536990
/// The selected player's name ("" for "(Create a New User)" or nothing selected).
pub fn FUN_00536990(g: &mut G, this: Ptr) -> Vec<u8> {
    let d = g.user_dialog(this).clone();
    let sel = g.list(d.offset_0x0).ext_0xd4;
    if !d.ext_0x174 && sel < 1 {
        return Vec::new();
    }
    if -1 < sel {
        let n = vcall!(g, d.offset_0x0, list.vfunction76);
        if sel < n {
            return vcall!(g, d.offset_0x0, list.vfunction73, sel);
        }
    }
    Vec::new()
}

/// port: 00536bd0 Sexy::UserDialog::vfunction3
/// `ButtonDepress(int id)`: with a player selected, "Rename" (0) asks for the new name,
/// "Delete" (1) asks to confirm.
pub fn vfunction3__00536bd0(g: &mut G, this: Ptr, param_1: i32) {
    crate::game::money_dialog::vfunction3_for_ButtonListener(g, this, param_1);
    let name = FUN_00536990(g, this);
    if !name.is_empty() {
        let app = g.money(this).offset_0x50;
        if param_1 == 0 {
            FUN_0054b770(g, app, &name);
        } else if param_1 == 1 {
            FUN_0054f790(g, app, &name);
        }
    }
}

/// port: 00532150 FUN_00532150
/// Removes the selected name; the one before it (at least the first player) is selected.
pub fn FUN_00532150(g: &mut G, this: Ptr) {
    let list = g.user_dialog(this).offset_0x0;
    let sel = g.list(list).ext_0xd4;
    vcall!(g, list, list.vfunction80, sel);
    let mut s = sel - 1;
    if s < 1 {
        s = 1;
    }
    let n = vcall!(g, list, list.vfunction76);
    if 1 < n {
        vcall!(g, list, list.vfunction84, s);
    }
}

/// port: 005321b0 FUN_005321b0
/// Renames the selected line.
pub fn FUN_005321b0(g: &mut G, this: Ptr, param_1: &[u8]) {
    let list = g.user_dialog(this).offset_0x0;
    let sel = g.list(list).ext_0xd4;
    if 0 < sel {
        vcall!(g, list, list.vfunction75, sel, param_1);
    }
}

/// port: 00532820 FUN_00532820
/// `NewUserDialog::SetName(const string&)`: the field shows it with the cursor at its end.
pub fn FUN_00532820(g: &mut G, this: Ptr, param_1: &[u8]) {
    let edit = g.new_user(this).offset_0x4;
    vcall!(g, edit, edit.vfunction74, param_1, true);
    g.edit(edit).offset_0x54 = param_1.len() as i32;
    g.edit(edit).offset_0x58 = 0;
}

/// port: 0054b5f0 FUN_0054b5f0
/// `ShowUserDialog()`: dialog 0x18, 400 pixels wide.
pub fn FUN_0054b5f0(g: &mut G, this: Ptr) {
    crate::game::win_fish_app::vfunction78(g, this, 0x18);
    let d = FUN_00536450(g, this, false);
    let h = vcall!(g, d, dlg.vfunction74, 400);
    vcall!(g, d, w.vfunction41, 0, 0x11, 400, h);
    crate::sexy::sexy_app_base::vfunction76(g, this, 0x18, d);
}

/// port: 0054ce30 FUN_0054ce30
/// "WHO ARE YOU?" closed: on OK the selected player becomes the current one (the screens
/// redrawn); the dialog goes.
pub fn FUN_0054ce30(g: &mut G, this: Ptr, param_1: bool) {
    let d = crate::sexy::sexy_app_base::vfunction74(g, this, 0x18);
    if d == NULL {
        return;
    }
    if param_1 {
        let name = FUN_00536990(g, d);
        let mgr = g.wfa(this).offset_0x184;
        let p = crate::game::profile_mgr::FUN_00514eb0(g, mgr, &name);
        if p != NULL {
            g.wfa(this).offset_0x18c = p;
            crate::game::profile::FUN_005013f0(g, p);
            let wm = g.sab(this).offset_0x318;
            vcall!(g, wm, w.vfunction11);
            let sel = g.wfa(this).offset_0xc;
            if sel != NULL {
                crate::game::game_object::vfunction75(g, sel);
            }
        }
    }
    crate::game::win_fish_app::vfunction78(g, this, 0x18);
}

/// port: 0054b770 FUN_0054b770
/// `ShowRenameUserDialog(const string& name)`: dialog 0x1b, 400 pixels wide, centered,
/// with the name filled in.
pub fn FUN_0054b770(g: &mut G, this: Ptr, param_1: &[u8]) {
    crate::game::win_fish_app::vfunction78(g, this, 0x1b);
    let d = crate::game::money_dialog::FUN_00534740(g, this, true);
    let h = vcall!(g, d, dlg.vfunction74, 400);
    let (aw, ah) = (g.sab(this).field_0xb8, g.sab(this).field_0xbc);
    vcall!(g, d, w.vfunction41, (aw + -400) / 2, (ah - h) / 2, 400, h);
    FUN_00532820(g, d, param_1);
    crate::sexy::sexy_app_base::vfunction76(g, this, 0x1b, d);
}

/// port: 0054f790 FUN_0054f790
/// `ConfirmDeleteUser(const string& name)`: dialog 0x1a, yes/no.
pub fn FUN_0054f790(g: &mut G, this: Ptr, param_1: &[u8]) {
    crate::game::win_fish_app::vfunction78(g, this, 0x1a);
    let mut msg = b"This will permanently remove '".to_vec();
    msg.extend_from_slice(param_1);
    msg.extend_from_slice(b"' from the player roster!");
    crate::game::win_fish_app::vfunction73(g, this, 0x1a, true, b"Are You Sure?", &msg, b"", 1);
}

/// port: 0054f8b0 FUN_0054f8b0
/// "Are You Sure?" closed: on yes the selected player is deleted (with their files) and
/// leaves the roster; deleting the current player switches to the newly selected one, or
/// the first, or (none left) asks for a new user. The roster is saved.
pub fn FUN_0054f8b0(g: &mut G, this: Ptr, param_1: bool) {
    crate::game::win_fish_app::vfunction78(g, this, 0x1a);
    if !param_1 {
        return;
    }
    let d = crate::sexy::sexy_app_base::vfunction74(g, this, 0x18);
    if d == NULL {
        return;
    }
    let cur = g.wfa(this).offset_0x18c;
    let cur_name = if cur == NULL { Vec::new() } else { g.profile(cur).field_0x24.clone() };
    let sel = FUN_00536990(g, d);
    if sel == cur_name {
        g.wfa(this).offset_0x18c = NULL;
    }
    let mgr = g.wfa(this).offset_0x184;
    crate::game::profile_mgr::FUN_00511340(g, mgr, &sel);
    FUN_00532150(g, d);
    if g.wfa(this).offset_0x18c == NULL {
        let name = FUN_00536990(g, d);
        let p = crate::game::profile_mgr::FUN_00514eb0(g, mgr, &name);
        g.wfa(this).offset_0x18c = p;
        if p == NULL {
            let p = crate::game::profile_mgr::FUN_00514bc0(g, mgr);
            g.wfa(this).offset_0x18c = p;
        }
    }
    crate::game::profile_mgr::FUN_00514d50(g, mgr);
    if g.wfa(this).offset_0x18c == NULL {
        crate::game::win_fish_app::FUN_0054b6a0(g, this);
    }
    let wm = g.sab(this).offset_0x318;
    vcall!(g, wm, w.vfunction11);
    let selr = g.wfa(this).offset_0xc;
    if selr != NULL {
        crate::game::game_object::vfunction75(g, selr);
    }
}

/// port: 0054fad0 FUN_0054fad0
/// "RENAME USER" closed: on OK with a name, renames the selected player ("Name Conflict"
/// when taken), saves the roster, keeps the current player current, updates the line.
pub fn FUN_0054fad0(g: &mut G, this: Ptr, param_1: bool) {
    if !param_1 {
        crate::game::win_fish_app::vfunction78(g, this, 0x1b);
        return;
    }
    let d18 = crate::sexy::sexy_app_base::vfunction74(g, this, 0x18);
    let d1b = crate::sexy::sexy_app_base::vfunction74(g, this, 0x1b);
    if d18 == NULL || d1b == NULL {
        return;
    }
    let old = FUN_00536990(g, d18);
    let new = crate::game::money_dialog::FUN_005333c0(g, d1b);
    if new.is_empty() {
        return;
    }
    let mgr = g.wfa(this).offset_0x184;
    let renamed = crate::game::profile_mgr::FUN_00514eb0(g, mgr, &old);
    let cur = g.wfa(this).offset_0x18c;
    if !crate::game::profile_mgr::FUN_00512630(g, mgr, &old, &new) {
        crate::game::win_fish_app::vfunction73(
            g,
            this,
            0x1d,
            true,
            b"Name Conflict",
            b"The name you entered is already being used.  Please enter a unique player name.",
            b"OK",
            3,
        );
        return;
    }
    crate::game::profile_mgr::FUN_00514d50(g, mgr);
    if renamed == cur {
        let p = crate::game::profile_mgr::FUN_00514eb0(g, mgr, &new);
        g.wfa(this).offset_0x18c = p;
    }
    FUN_005321b0(g, d18, &new);
    let wm = g.sab(this).offset_0x318;
    vcall!(g, wm, w.vfunction11);
    crate::game::win_fish_app::vfunction78(g, this, 0x1b);
}
