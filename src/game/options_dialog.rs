//! `Sexy::OptionsDialog`: the Options dialog (dialog id 1): music and sound-effect sliders,
//! the Fullscreen / Custom Cursors / Hardware Acceleration checkboxes, and the Register,
//! Help, web link, Check Updates and Back to Main Menu buttons. A MoneyDialog with a
//! SliderListener part at +0x158 and a CheckboxListener part at +0x15c; `OptionsDialog_data`
//! at +0x160 (the object is 400 bytes; the database's 44-byte type stops before the
//! in-game flag at +0x18c).

use crate::sexy::dialog::{alloc_dialog, DlgSub, MoneySub};
use crate::sexy::prelude::*;

/// `OptionsDialog_data` (object offset 0x160).
#[derive(Debug, Clone, Default)]
pub struct OptionsDialog_data {
    /// +0x160 the app.
    pub offset_0x0: Ptr,
    /// +0x164 the music slider (id 5).
    pub offset_0x4: Ptr,
    /// +0x168 the sound-effects slider (id 6).
    pub offset_0x8: Ptr,
    /// +0x16c the Fullscreen checkbox (id 7).
    pub offset_0xc: Ptr,
    /// +0x170 the Custom Cursors checkbox (id 8).
    pub offset_0x10: Ptr,
    /// +0x174 the Hardware Acceleration checkbox (id 9).
    pub offset_0x14: Ptr,
    /// +0x178 the Register button (id 0).
    pub offset_0x18: Ptr,
    /// +0x17c the Help button (id 1).
    pub offset_0x1c: Ptr,
    /// +0x180 the Check Updates button (id 4).
    pub offset_0x20: Ptr,
    /// +0x184 the web link button (id 2).
    pub offset_0x24: Ptr,
    /// +0x188 the Back to Main Menu button (id 3).
    pub offset_0x28: Ptr,
    /// +0x18c opened from the main menu (no Back to Main Menu).
    pub field_0x2c: bool,
}

impl G {
    pub fn options_dialog(&mut self, p: Ptr) -> &mut OptionsDialog_data {
        match &mut self.widget(p).ext {
            WExt::Dialog(e) => match &mut e.sub {
                DlgSub::Money(_, MoneySub::Options(o)) => o,
                s => panic!("{p} is not an OptionsDialog: {s:?}"),
            },
            e => panic!("{p} is not a Dialog: {e:?}"),
        }
    }
}

/// port: 00500b70 FUN_00500b70
/// `MakeCheckbox(int theId, CheckboxListener*, bool checked)`: the game's checkbox images
/// (`DAT_005e8d5c` / `DAT_005e8c64`), with transparencies, in front.
pub fn FUN_00500b70(g: &mut G, param_1: i32, param_2: Ptr, param_3: bool) -> Ptr {
    let (off, on) = (g.res.DAT_005e8d5c, g.res.DAT_005e8c64);
    let c = crate::sexy::checkbox::Checkbox(g, off, on, param_1, param_2);
    g.checkbox(c).offset_0x8 = param_3;
    g.w(c).offset_0x6 = true;
    g.wc(c).offset_0x3c = true;
    c
}

/// port: 005349d0 FUN_005349d0
/// `OptionsDialog(WinFishApp*, bool fromMainMenu)`: "Options" with one OK button; the
/// buttons (Register hidden when registered, Check Updates when the app says so, the web
/// link without its text, Back to Main Menu from the main menu); the sliders at the app's
/// volumes (music clamped to 0..1); the checkboxes at the current screen mode, cursors and
/// 3D.
pub fn FUN_005349d0(g: &mut G, param_1: Ptr, param_2: bool) -> Ptr {
    let this = alloc_dialog(
        g,
        &crate::sexy::vtables_gen::Sexy__OptionsDialog_vftable,
        DlgSub::Money(crate::game::money_dialog::MoneyDialog_data::default(), MoneySub::Options(OptionsDialog_data::default())),
    );
    let (comp, btn) = (g.res.DAT_005e8cc0, g.res.DAT_005e8c54);
    crate::game::money_dialog::MoneyDialog(g, this, param_1, comp, btn, 1, true, b"Options", b"", b"OK", 3);
    g.options_dialog(this).field_0x2c = param_2;
    g.options_dialog(this).offset_0x0 = param_1;
    crate::sexy::dialog::vfunction35_for_Widget(g, this, 3, FUN_00433360(0xff, 0xff, 0x64));
    use crate::game::game_selector::FUN_00506760;
    let b = FUN_00506760(g, 0, this, b"Register", NULL);
    g.options_dialog(this).offset_0x18 = b;
    let b = FUN_00506760(g, 1, this, b"Help", NULL);
    g.options_dialog(this).offset_0x1c = b;
    let label = crate::sexy::sexy_app_base::FUN_004872f0(g, param_1, "weblinktext", b"");
    let b = FUN_00506760(g, 2, this, &label, NULL);
    g.options_dialog(this).offset_0x24 = b;
    let b = FUN_00506760(g, 4, this, b"Check Updates", NULL);
    g.options_dialog(this).offset_0x20 = b;
    let b = FUN_00506760(g, 3, this, b"Back to Main Menu", NULL);
    g.options_dialog(this).offset_0x28 = b;
    let (track, thumb) = (g.res.DAT_005e8cc8, g.res.DAT_005e8be4);
    let s = crate::sexy::slider::Slider(g, track, thumb, 5, this);
    g.options_dialog(this).offset_0x4 = s;
    let v = crate::sexy::sexy_app_base::vfunction57(g, param_1) as f32;
    let mut c = if 1.0 < v { 1.0 } else { v };
    if c < 0.0 {
        c = 0.0;
    }
    vcall!(g, s, slider.vfunction71, c as f64);
    let s = crate::sexy::slider::Slider(g, track, thumb, 6, this);
    g.options_dialog(this).offset_0x8 = s;
    let v = crate::sexy::sexy_app_base::vfunction58(g, param_1);
    vcall!(g, s, slider.vfunction71, v);
    let windowed = g.sab(param_1).field_0x33b;
    let c = FUN_00500b70(g, 7, this, !windowed);
    g.options_dialog(this).offset_0xc = c;
    let cursors = g.sab(param_1).field_0x4f8;
    let c = FUN_00500b70(g, 8, this, cursors);
    g.options_dialog(this).offset_0x10 = c;
    let is3d = crate::sexy::sexy_app_base::dtor_MemoryImage__00489a20(g, param_1);
    let c = FUN_00500b70(g, 9, this, is3d);
    g.options_dialog(this).offset_0x14 = c;
    let d = g.options_dialog(this).clone();
    if d.field_0x2c {
        vcall!(g, d.offset_0x28, w.vfunction32, false);
    }
    if g.app_obj(param_1).sa.field_0x6d {
        vcall!(g, d.offset_0x20, w.vfunction32, false);
    }
    if g.app_obj(param_1).sa.offset_0xe5 {
        vcall!(g, d.offset_0x18, w.vfunction32, false);
    }
    let has_link = !g.btn(d.offset_0x24).field_0x4.is_empty();
    vcall!(g, d.offset_0x24, w.vfunction32, has_link);
    this
}

/// port: 00530d30 Sexy::OptionsDialog::~OptionsDialog
/// Deletes the buttons, sliders and checkboxes.
pub fn dtor_OptionsDialog(g: &mut G, this: Ptr) {
    let d = g.options_dialog(this).clone();
    for p in [
        d.offset_0x18, d.offset_0x20, d.offset_0x24, d.offset_0x4, d.offset_0x8, d.offset_0xc, d.offset_0x10, d.offset_0x14, d.offset_0x1c,
        d.offset_0x28,
    ] {
        if p != NULL {
            vcall!(g, p, w.vfunction1, 1);
        }
    }
    crate::game::money_dialog::dtor_MoneyDialog(g, this);
}

/// port: 00532870 Sexy::OptionsDialog::deleting_destructor
pub fn deleting_destructor(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_OptionsDialog(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

fn visible(g: &mut G, p: Ptr) -> bool {
    g.w(p).offset_0x0
}

/// port: 00530e60 Sexy::OptionsDialog::vfunction74
/// `GetPreferredHeight(int)`: 0x148 (0x16a with Back to Main Menu), plus a row for
/// Register + Check Updates and one for the web link.
pub fn vfunction74(g: &mut G, this: Ptr, _width: i32) -> i32 {
    let d = g.options_dialog(this).clone();
    let mut h = 0x16a;
    if !visible(g, d.offset_0x28) {
        h = 0x148;
    }
    if visible(g, d.offset_0x18) && visible(g, d.offset_0x20) {
        h += 0x22;
    }
    if visible(g, d.offset_0x24) {
        h += 0x22;
    }
    h
}

/// port: 00530eb0 Sexy::OptionsDialog::vfunction21
/// `AddedToManager(WidgetManager*)`: adds its widgets.
pub fn vfunction21(g: &mut G, this: Ptr, param_1: Ptr) {
    crate::sexy::dialog::vfunction21_for_Widget(g, this, param_1);
    let d = g.options_dialog(this).clone();
    for p in [
        d.offset_0x18, d.offset_0x20, d.offset_0x1c, d.offset_0x4, d.offset_0x8, d.offset_0x10, d.offset_0x14, d.offset_0xc, d.offset_0x24,
        d.offset_0x28,
    ] {
        vcall!(g, param_1, w.vfunction4, p);
    }
}

/// port: 00530f70 Sexy::OptionsDialog::vfunction22
/// `RemovedFromManager(WidgetManager*)`: takes its widgets out.
pub fn vfunction22(g: &mut G, this: Ptr, param_1: Ptr) {
    crate::sexy::dialog::vfunction22_for_Widget(g, this, param_1);
    let d = g.options_dialog(this).clone();
    for p in [
        d.offset_0x18, d.offset_0x20, d.offset_0x1c, d.offset_0x4, d.offset_0x8, d.offset_0xc, d.offset_0x10, d.offset_0x14, d.offset_0x24,
        d.offset_0x28,
    ] {
        vcall!(g, param_1, w.vfunction5, p);
    }
}

/// port: 00531030 Sexy::OptionsDialog::vfunction41
/// `Resize(int x, int y, int w, int h)`: stacks the visible buttons upward from the OK
/// button (Back to Main Menu, the web link, then Help / Register / Check Updates: one alone
/// full width, two side by side, three as one plus a pair), and places the sliders and
/// checkboxes.
pub fn vfunction41(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32, param_4: i32) {
    crate::sexy::dialog::vfunction41_for_Widget(g, this, param_1, param_2, param_3, param_4);
    let yes = g.dialog(this).offset_0x8;
    let yes_y = g.wc(yes).offset_0x30;
    let d = g.options_dialog(this).clone();
    let (left, right) = (g.dialog(this).ext_0x11c, g.dialog(this).ext_0x124);
    let (mx, my, mw) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30, g.wc(this).offset_0x34);
    let mut y = yes_y - 0x22;
    if visible(g, d.offset_0x28) {
        vcall!(g, d.offset_0x28, w.vfunction41, mx + left, y, mw - right - left, 0x21);
        y = yes_y - 0x44;
    }
    let inner = mw - right;
    let half = ((inner - left) - 6) / 2;
    if visible(g, d.offset_0x24) {
        vcall!(g, d.offset_0x24, w.vfunction41, mx + left, y, inner - left, 0x21);
        y -= 0x22;
    }
    let mut row: Vec<Ptr> = Vec::new();
    for p in [d.offset_0x1c, d.offset_0x18, d.offset_0x20] {
        if visible(g, p) {
            row.push(p);
        }
    }
    let n = row.len();
    if n == 1 || n == 3 {
        vcall!(g, row[0], w.vfunction41, mx + left, y, mw - right - left, 0x21);
        y -= 0x22;
    }
    if 1 < n {
        vcall!(g, row[n - 2], w.vfunction41, left + mx, y, half, 0x21);
        vcall!(g, row[n - 1], w.vfunction41, left + mx + 3 + half, y, half, 0x21);
    }
    vcall!(g, d.offset_0x4, w.vfunction41, mx + 0x98, my + 0x3a, mw - 0xc0, 0x21);
    vcall!(g, d.offset_0x8, w.vfunction41, mx + 0x98, my + 0x5a, mw - 0xc0, 0x21);
    vcall!(g, d.offset_0xc, w.vfunction41, mx + 0x21, my + 0x80, 0x2e, 0x2d);
    vcall!(g, d.offset_0x10, w.vfunction41, mx + 0xa1, my + 0x80, 0x2e, 0x2d);
    vcall!(g, d.offset_0x14, w.vfunction41, mx + 0x21, my + 0xac, 0x2e, 0x2d);
}

/// port: 00531270 Sexy::OptionsDialog::vfunction1
/// `SliderVal(int id, double val)` (the SliderListener part): the music or sound-effects
/// volume; a sound-effects change not being dragged plays the click as a sample.
pub fn vfunction1__00531270(g: &mut G, this: Ptr, param_2: i32, param_3: f64) {
    let app = g.options_dialog(this).offset_0x0;
    if param_2 == 5 {
        crate::game::win_fish_app::vfunction61(g, app, param_3);
    } else if param_2 == 6 {
        crate::sexy::sexy_app_base::vfunction62(g, app, param_3);
        let s = g.options_dialog(this).offset_0x8;
        if !g.slider(s).offset_0x1c {
            let snd = g.res.DAT_005e8c40;
            crate::sexy::sexy_app_base::vfunction55(g, app, snd);
        }
    }
}

/// port: 00537220 Sexy::OptionsDialog::vfunction1
/// `CheckboxChecked(int id, bool checked)` (the CheckboxListener part): `vfunction78`.
pub fn vfunction1__00537220(g: &mut G, this: Ptr, param_1: i32, param_2: bool) {
    vfunction78(g, this, param_1, param_2);
}

/// port: 00534f60 Sexy::OptionsDialog::vfunction27
/// `Draw(Graphics*)`: the dialog, then the labels in its second color: Music, Sound Fx, and
/// beside each checkbox its name.
pub fn vfunction27(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    use crate::sexy::graphics::{FUN_00455880, FUN_00455890, FUN_004558e0, FUN_00455cf0};
    crate::sexy::dialog::vfunction27_for_Widget(g, this, gfx);
    let f = g.res.DAT_005e8cf0;
    FUN_00455880(gfx, f);
    FUN_004558e0(gfx, true);
    let c = g.w(this).offset_0xc[1];
    FUN_00455890(gfx, c);
    FUN_00455cf0(gfx, g, b"Music", 0x30, 0x4e);
    FUN_00455cf0(gfx, g, b"Sound Fx", 0x30, 0x6e);
    let d = g.options_dialog(this).clone();
    let (mx, my) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
    for (p, s) in [(d.offset_0xc, &b"Fullscreen"[..]), (d.offset_0x10, b"Custom Cursors"), (d.offset_0x14, b"Hardware Acceleration")] {
        let (x, y) = (g.wc(p).offset_0x2c - mx + 0x2b, g.wc(p).offset_0x30 - my + 0x18);
        FUN_00455cf0(gfx, g, s, x, y);
    }
    FUN_004558e0(gfx, false);
}

/// port: 005351b0 Sexy::OptionsDialog::vfunction78
/// `CheckboxChecked(int id, bool checked)`: the click; unchecking Fullscreen where the
/// desktop allows no window explains and re-checks it; checking Hardware Acceleration
/// where it is unsupported unchecks it and says so, where only not recommended it warns.
pub fn vfunction78(g: &mut G, this: Ptr, param_1: i32, param_2: bool) {
    crate::game::money_dialog::vfunction78_for_Widget(g, this, param_1, param_2);
    let app = g.options_dialog(this).offset_0x0;
    if param_1 == 7 {
        if !param_2 && g.sab(app).field_0x33e {
            crate::game::win_fish_app::vfunction73(
                g,
                app,
                8,
                true,
                b"No Windowed Mode",
                b"Windowed mode is only available if your desktop was running in either\n16 bit or 32 bit color mode when you started the game.\n\nIf you'd like to run in Windowed mode then you need to quit the game and switch your desktop to 16 or 32 bit color mode.",
                b"OK",
                3,
            );
            let c = g.options_dialog(this).offset_0xc;
            vcall!(g, c, checkbox.vfunction71, true, false);
        }
    } else if param_1 == 9 && param_2 {
        if !crate::sexy::sexy_app_base::FUN_00489a30(g, app) {
            let c = g.options_dialog(this).offset_0x14;
            vcall!(g, c, checkbox.vfunction71, false, false);
            crate::game::win_fish_app::vfunction73(
                g,
                app,
                0xe,
                true,
                b"Not Supported",
                b"Hardware Acceleration cannot be enabled on this computer.\n\nYour video card does not\nmeet the minimum requirements\nfor this game.",
                b"OK",
                3,
            );
            return;
        }
        if !crate::sexy::sexy_app_base::FUN_00489a50(g, app) {
            crate::game::win_fish_app::vfunction73(
                g,
                app,
                0xe,
                true,
                b"Warning",
                b"Your video card may not fully support this feature.\n\nIf you experience slower performance, please disable Hardware Acceleration.",
                b"OK",
                3,
            );
        }
    }
}

/// port: 00535430 Sexy::OptionsDialog::vfunction3
/// `ButtonDepress(int id)`: the dialog's own buttons first; Register opens the registration
/// dialog, Help the help screen, the web link the `weblink` page (popcap.com by default),
/// Check Updates the updater; Back to Main Menu asks first when a game in progress would be
/// lost (outside the virtual tank), else returns to the main menu.
pub fn vfunction3(g: &mut G, this: Ptr, param_1: i32) {
    crate::game::money_dialog::vfunction3_for_ButtonListener(g, this, param_1);
    let app = g.options_dialog(this).offset_0x0;
    match param_1 {
        0 => crate::game::register_dialog::FUN_0054e8a0(g, app),
        1 => crate::game::win_fish_app::FUN_0054c220(g, app, false),
        2 => {
            let url = crate::sexy::sexy_app_base::FUN_004872f0(g, app, "weblink", b"http://www.popcap.com");
            crate::game::win_fish_app::vfunction38__005516c0(g, app, &url, false);
        }
        3 => {
            let board = g.wfa(app).offset_0x4;
            if g.wfa(app).offset_0x150 != 5 && board != NULL && crate::game::board::FUN_00537bf0(g, board) {
                crate::game::win_fish_app::FUN_0054e760(g, app);
                return;
            }
            crate::game::win_fish_app::FUN_00552230(g, app);
        }
        4 => crate::game::win_fish_app::FUN_00551a70(g, app),
        _ => {}
    }
}
