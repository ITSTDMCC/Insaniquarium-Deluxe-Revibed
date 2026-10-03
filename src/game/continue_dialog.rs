//! `Sexy::ContinueDialog`: "CONTINUE GAME?" (dialog 0x1e) when a saved game was resumed:
//! "Continue" (the save is used up), "Restart Level" / "New Game" (after a confirmation),
//! or CANCEL back to the menu. Plus the app functions that open it and act on the
//! confirmation.
//!
//! The object is 0x160 bytes: MoneyDialog (0x158), then `ContinueDialog_data` (the two
//! buttons).

use crate::game::money_dialog::MoneyDialog_data;
use crate::sexy::dialog::{alloc_dialog, DlgSub, MoneySub};
use crate::sexy::prelude::*;

/// `ContinueDialog_data` (object offset 0x158).
#[derive(Debug, Clone, Default)]
pub struct ContinueDialog_data {
    /// +0x158 "Continue" (id 0).
    pub offset_0x0: Ptr,
    /// +0x15c "Restart Level" / "New Game" (id 1).
    pub offset_0x4: Ptr,
}

impl G {
    pub fn continue_dialog(&mut self, p: Ptr) -> &mut ContinueDialog_data {
        match &mut self.widget(p).ext {
            WExt::Dialog(e) => match &mut e.sub {
                DlgSub::Money(_, MoneySub::Continue(d)) => d,
                s => panic!("{p} is not a ContinueDialog: {s:?}"),
            },
            e => panic!("{p} is not a Dialog: {e:?}"),
        }
    }
}

/// port: 005334d0 FUN_005334d0
/// `ContinueDialog(WinFishApp*)`: modal, CANCEL; in Adventure "continue your current game
/// or restart the level?" with Continue / Restart Level, elsewhere "... or start a new
/// game?" with Continue / New Game.
pub fn FUN_005334d0(g: &mut G, param_1: Ptr) -> Ptr {
    let this = alloc_dialog(
        g,
        &crate::sexy::vtables_gen::Sexy__ContinueDialog_vftable,
        DlgSub::Money(MoneyDialog_data::default(), MoneySub::Continue(ContinueDialog_data::default())),
    );
    let (comp, btn) = (g.res.DAT_005e8cc0, g.res.DAT_005e8c54);
    crate::game::money_dialog::MoneyDialog(g, this, param_1, comp, btn, 0x1e, true, b"CONTINUE GAME?", b"", b"CANCEL", 3);
    let (lines, second): (&[u8], &[u8]) = if g.wfa(param_1).offset_0x150 == 0 {
        (b"Do you want to continue your current game or restart the level?", b"Restart Level")
    } else {
        (b"Do you want to continue your current game or start a new game?", b"New Game")
    };
    g.dialog(this).field_0x4c = lines.to_vec();
    let b0 = crate::game::game_selector::FUN_00506760(g, 0, this, b"Continue", NULL);
    g.continue_dialog(this).offset_0x0 = b0;
    let b1 = crate::game::game_selector::FUN_00506760(g, 1, this, second, NULL);
    g.continue_dialog(this).offset_0x4 = b1;
    this
}

/// port: 00532380 Sexy::ContinueDialog::~ContinueDialog
pub fn dtor_ContinueDialog(g: &mut G, this: Ptr) {
    let d = g.continue_dialog(this).clone();
    for b in [d.offset_0x0, d.offset_0x4] {
        if b != NULL {
            vcall!(g, b, w.vfunction1, 1);
        }
    }
    crate::game::money_dialog::dtor_MoneyDialog(g, this);
}

/// port: 00532bf0 Sexy::ContinueDialog::deleting_destructor
pub fn deleting_destructor(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_ContinueDialog(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 00530030 Sexy::ContinueDialog::vfunction41
/// `Resize(...)`: the two buttons side by side above CANCEL, splitting the button row.
pub fn vfunction41(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32, param_4: i32) {
    crate::sexy::dialog::vfunction41_for_Widget(g, this, param_1, param_2, param_3, param_4);
    let (mx, mw) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x34);
    let d = g.dialog(this).clone();
    let bw = ((((((mw - d.ext_0x124) - d.ext_0x114) + d.ext_0x14c * -2) - d.ext_0x148) - d.ext_0x11c) - d.ext_0x10c) / 2;
    let c = g.continue_dialog(this).clone();
    let img = g.dialog_button(c.offset_0x0).offset_0x0;
    let bh = crate::sexy::graphics::FUN_00457490(g, img);
    let (yx, yy) = (g.wc(d.offset_0x8).offset_0x2c, g.wc(d.offset_0x8).offset_0x30);
    vcall!(g, c.offset_0x0, w.vfunction41, mx + d.ext_0x14c + d.ext_0x11c + d.ext_0x10c, (yy - bh) + -4, bw, bh);
    let y0 = g.wc(c.offset_0x0).offset_0x30;
    vcall!(g, c.offset_0x4, w.vfunction41, yx + bw + d.ext_0x148, y0, bw, bh);
}

/// port: 00530100 Sexy::ContinueDialog::vfunction21
/// `AddedToManager(WidgetManager*)`.
pub fn vfunction21(g: &mut G, this: Ptr, param_1: Ptr) {
    crate::sexy::dialog::vfunction21_for_Widget(g, this, param_1);
    let c = g.continue_dialog(this).clone();
    vcall!(g, param_1, w.vfunction4, c.offset_0x0);
    vcall!(g, param_1, w.vfunction4, c.offset_0x4);
}

/// port: 00530140 Sexy::ContinueDialog::vfunction22
/// `RemovedFromManager(WidgetManager*)`.
pub fn vfunction22(g: &mut G, this: Ptr, param_1: Ptr) {
    crate::sexy::dialog::vfunction22_for_Widget(g, this, param_1);
    let c = g.continue_dialog(this).clone();
    vcall!(g, param_1, w.vfunction5, c.offset_0x0);
    vcall!(g, param_1, w.vfunction5, c.offset_0x4);
}

/// port: 00533760 Sexy::ContinueDialog::vfunction3
/// `ButtonDepress(int id)`: Continue (0) deletes the save just loaded and closes; the
/// second button (1) asks to confirm (dialog 0x22, its OK relabelled); CANCEL closes and
/// goes back to the menu.
pub fn vfunction3(g: &mut G, this: Ptr, param_1: i32) {
    let app = g.money(this).offset_0x50;
    let id = g.dialog(this).ext_0x13c;
    if param_1 == 0 {
        let mode = g.wfa(app).offset_0x150;
        let profile = g.wfa(app).offset_0x18c;
        let pid = g.profile(profile).field_0x44;
        let path = crate::game::win_fish_app::FUN_00505880(mode, pid);
        crate::sexy::app_host::FUN_0047fbc0(g, &path);
        crate::game::win_fish_app::vfunction78(g, app, id);
    } else if param_1 == 1 {
        let (header, lines, ok): (&[u8], &[u8], &[u8]) = if g.wfa(app).offset_0x150 == 0 {
            (b"Restart Level?", b"Are you sure that you want to restart the level?", b"Restart")
        } else {
            (b"New Game?", b"Are you sure that you want to start a new game?", b"New Game")
        };
        let d = crate::game::win_fish_app::vfunction73(g, app, 0x22, true, header, lines, b"", 2);
        let yes = g.dialog(d).offset_0x8;
        g.btn(yes).field_0x4 = ok.to_vec();
    } else {
        crate::game::win_fish_app::vfunction78(g, app, id);
        crate::game::win_fish_app::FUN_00552100(g, app);
    }
}

/// port: 0054b3a0 FUN_0054b3a0
/// `ShowContinueDialog()`: dialog 0x1e, 380 pixels wide, centered at y 70.
pub fn FUN_0054b3a0(g: &mut G, this: Ptr) {
    let d = FUN_005334d0(g, this);
    let h = vcall!(g, d, dlg.vfunction74, 0x17c);
    let aw = g.sab(this).field_0xb8;
    vcall!(g, d, w.vfunction41, (aw + -0x17c) / 2, 0x46, 0x17c, h);
    crate::sexy::sexy_app_base::vfunction76(g, this, 0x1e, d);
}

/// port: 00552480 FUN_00552480
/// "Restart Level?" / "New Game?" confirmed: both dialogs close, the save is deleted, the
/// level music restarts, and Adventure begins the level again (others pick a tank).
pub fn FUN_00552480(g: &mut G, this: Ptr) {
    crate::game::win_fish_app::vfunction78(g, this, 0x1e);
    crate::game::win_fish_app::vfunction78(g, this, 0x22);
    crate::game::win_fish_app::FUN_0054bc30(g, this);
    let mode = g.wfa(this).offset_0x150;
    let profile = g.wfa(this).offset_0x18c;
    let pid = g.profile(profile).field_0x44;
    let path = crate::game::win_fish_app::FUN_00505880(mode, pid);
    crate::sexy::app_host::FUN_0047fbc0(g, &path);
    crate::game::win_fish_app::FUN_0054b020(g, this);
    crate::game::win_fish_app::FUN_0054b1a0(g, this, 2, 0, false);
    if g.wfa(this).offset_0x150 == 0 {
        crate::game::win_fish_app::FUN_00552380(g, this, false, true);
        return;
    }
    crate::game::tank_screen::FUN_0054c130(g, this);
}
