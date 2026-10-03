//! `Sexy::SimSetupScreen`: the virtual tank's "Tank Setup" screen (the TANK button): the
//! current backdrop with Prev / Next / Sell, five checkboxes for the tank's options, the
//! "Screensaver..." button with the screensaver state, and "Return to Tank". Plus the app
//! functions that open and close it and read the screensaver settings.
//!
//! `SimSetupScreen_data` starts at object offset 0x90 (after the ButtonListener vftable at
//! 0x88 and the CheckboxListener vftable at 0x8c).
//!
//! The screensaver state comes from the operating system in the original (the module path,
//! `GetShortPathNameA`, and the `SCRNSAVE.EXE` value under HKCU "Control Panel\Desktop" or
//! SYSTEM.INI). Here those are host boundaries: the value is read from the replaced
//! registry (`g.registry`, key `Control Panel\Desktop\SCRNSAVE.EXE`), the OS is taken to be
//! NT 6+ (`FUN_0040fab0` true), and a path is never shortened (no file is looked up).

use crate::sexy::graphics::{FUN_00455880, FUN_00455890, FUN_00455cf0, FUN_00455d20, FUN_00455870, FUN_004563f0};
use crate::sexy::image_font as font;
use crate::sexy::prelude::*;
use crate::sexy::types::{CRect, FUN_00433320, FUN_00433360};

/// `SimSetupScreen_data` (object offset 0x90).
#[derive(Debug, Clone, Default)]
pub struct SimSetupScreen_data {
    /// +0x90 the app.
    pub field_0x0: Ptr,
    /// +0x94 backdrop frame x (0x3c).
    pub field_0x4: i32,
    /// +0x98 backdrop frame y (0x8c).
    pub field_0x8: i32,
    /// +0x9c something changed (leaving re-enters the virtual tank).
    pub field_0xc: bool,
    /// +0xa0 "Return to Tank" (id 9).
    pub offset_0x10: Ptr,
    /// +0xa4 "Prev" (id 0).
    pub offset_0x14: Ptr,
    /// +0xa8 "Next" (id 1).
    pub offset_0x18: Ptr,
    /// +0xac "Sell" (id 2).
    pub offset_0x1c: Ptr,
    /// +0xb0 "Screensaver..." (id 8).
    pub offset_0x20: Ptr,
    /// +0xb4 "Show Fish Names" (id 3; board +0x4fc).
    pub offset_0x24: Ptr,
    /// +0xb8 "Show Bubbulator" (id 4; board +0x4fd).
    pub offset_0x28: Ptr,
    /// +0xbc "Enable Alien Attractor" (id 5; board +0x4fe).
    pub offset_0x2c: Ptr,
    /// +0xc0 "Allow Fish To Drop Shells" (id 6; board +0x500).
    pub offset_0x30: Ptr,
    /// +0xc4 "Always Show When Fish Are Hungry" (id 7; board +0x4ff).
    pub offset_0x34: Ptr,
}

impl G {
    pub fn sim_setup(&mut self, p: Ptr) -> &mut SimSetupScreen_data {
        match &mut self.widget(p).ext {
            WExt::SimSetup(d) => d,
            e => panic!("{p} is not a SimSetupScreen: {e:?}"),
        }
    }
}

/// port: 0052a190 Sexy::SimSetupScreen::SimSetupScreen
/// `SimSetupScreen(WinFishApp*)`: full screen; the store music; the buttons (Prev at the
/// backdrop frame, Next beside it, Sell centered under both); the checkboxes at the board's
/// options, stacked under the first.
pub fn SimSetupScreen(g: &mut G, param_1: Ptr) -> Ptr {
    use crate::game::game_selector::{FUN_00506760, FUN_00506820};
    use crate::sexy::widget::FUN_0046e880;
    let (wc, w) = crate::sexy::widget::Widget();
    let d = SimSetupScreen_data { field_0x0: param_1, ..Default::default() };
    let this = g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__SimSetupScreen_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::SimSetup(Box::new(d)) }),
    });
    let (aw, ah) = (g.sab(param_1).field_0xb8, g.sab(param_1).field_0xbc);
    let wc = g.wc(this);
    wc.offset_0x2c = 0;
    wc.offset_0x30 = 0;
    wc.offset_0x34 = aw;
    wc.offset_0x38 = ah;
    crate::game::win_fish_app::FUN_0054b020(g, param_1);
    crate::game::win_fish_app::FUN_0054b1a0(g, param_1, 2, 0, false);
    g.sim_setup(this).field_0x8 = 0x8c;
    g.sim_setup(this).field_0x4 = 0x3c;

    let f = g.res.DAT_005e8cf0;
    let b = FUN_00506760(g, 9, this, b"Return to Tank", f);
    g.sim_setup(this).offset_0x10 = b;
    let h = g.wc(b).offset_0x38;
    vcall!(g, b, w.vfunction41, 0xe1, 0x1a4, 0xba, h);

    let b = FUN_00506760(g, 8, this, b"Screensaver...", f);
    g.sim_setup(this).offset_0x20 = b;
    let h = g.wc(b).offset_0x38;
    vcall!(g, b, w.vfunction41, 0x50, 0x140, 0xba, h);

    let img = g.res.DAT_005e8ad8;
    let prev = FUN_00506820(g, 0, this, b"Prev", img);
    g.sim_setup(this).offset_0x14 = prev;
    let (fx, fy) = (g.sim_setup(this).field_0x4, g.sim_setup(this).field_0x8);
    let h = g.wc(prev).offset_0x38;
    vcall!(g, prev, w.vfunction41, fx + 0xe, fy + 0x86, 0x41, h);

    let img = g.res.DAT_005e8dec;
    let next = FUN_00506820(g, 1, this, b"Next", img);
    g.sim_setup(this).offset_0x18 = next;
    FUN_0046e880(g, next, 0x4403, prev, 0x41, 0, 0, 0);

    let img = g.res.DAT_005e8d28;
    let sell = FUN_00506820(g, 2, this, b"Sell", img);
    g.sim_setup(this).offset_0x1c = sell;
    FUN_0046e880(g, sell, 0x4402, prev, -2, 0, 0, 0);
    FUN_0046e880(g, sell, 0x20000, next, 0, 0, 3, 0);

    for b in [prev, next, sell] {
        vcall!(g, b, w.vfunction35, 0, FUN_00433320(0xffffff));
    }
    let ret = g.sim_setup(this).offset_0x10;
    vcall!(g, ret, w.vfunction35, 0, FUN_00433360(0xff, 0xf0, 0));
    let ss = g.sim_setup(this).offset_0x20;
    vcall!(g, ss, w.vfunction35, 0, FUN_00433320(0xffffff));

    let board = g.wfa(param_1).offset_0x4;
    let bd = g.board(board).clone();
    let mk = crate::game::options_dialog::FUN_00500b70;
    let c3 = mk(g, 3, this, bd.ext_0x4fc);
    let c4 = mk(g, 4, this, bd.ext_0x4fd);
    let c5 = mk(g, 5, this, bd.ext_0x4fe);
    let c7 = mk(g, 7, this, bd.ext_0x4ff);
    let c6 = mk(g, 6, this, bd.ext_0x500);
    let d = g.sim_setup(this);
    d.offset_0x24 = c3;
    d.offset_0x28 = c4;
    d.offset_0x2c = c5;
    d.offset_0x34 = c7;
    d.offset_0x30 = c6;
    vcall!(g, c3, w.vfunction41, 0x154, 0x8c, 0x2e, 0x2d);
    FUN_0046e880(g, c4, 0x1203, c3, 0, 0, 0, 0);
    FUN_0046e880(g, c5, 0x1203, c4, 0, 0, 0, 0);
    FUN_0046e880(g, c6, 0x1203, c5, 0, 0, 0, 0);
    FUN_0046e880(g, c7, 0x1203, c6, 0, 0, 0, 0);
    this
}

/// port: 00519130 Sexy::SimSetupScreen::~SimSetupScreen
pub fn dtor_SimSetupScreen(g: &mut G, this: Ptr) {
    let d = g.sim_setup(this).clone();
    for p in [d.offset_0x10, d.offset_0x20, d.offset_0x14, d.offset_0x18, d.offset_0x1c, d.offset_0x24, d.offset_0x28, d.offset_0x2c, d.offset_0x34, d.offset_0x30] {
        if p != NULL {
            vcall!(g, p, w.vfunction1, 1);
        }
    }
    crate::sexy::widget::dtor_Widget(g, this);
}

/// port: 0051b200 Sexy::SimSetupScreen::deleting_destructor_for_Widget
pub fn deleting_destructor_for_Widget(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_SimSetupScreen(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

fn children(d: &SimSetupScreen_data) -> [Ptr; 10] {
    [d.offset_0x10, d.offset_0x20, d.offset_0x14, d.offset_0x18, d.offset_0x1c, d.offset_0x24, d.offset_0x28, d.offset_0x2c, d.offset_0x34, d.offset_0x30]
}

/// port: 00519250 Sexy::SimSetupScreen::vfunction21_for_Widget
/// `AddedToManager(WidgetManager*)`: adds the buttons and checkboxes.
pub fn vfunction21_for_Widget(g: &mut G, this: Ptr, param_1: Ptr) {
    crate::sexy::widget_container::vfunction21(g, this, param_1);
    let d = g.sim_setup(this).clone();
    for p in children(&d) {
        vcall!(g, param_1, w.vfunction4, p);
    }
}

/// port: 00519310 Sexy::SimSetupScreen::vfunction22_for_Widget
/// `RemovedFromManager(WidgetManager*)`.
pub fn vfunction22_for_Widget(g: &mut G, this: Ptr, param_1: Ptr) {
    crate::sexy::widget_container::vfunction22(g, this, param_1);
    let d = g.sim_setup(this).clone();
    for p in children(&d) {
        vcall!(g, param_1, w.vfunction5, p);
    }
}

/// port: 00519430 Sexy::SimSetupScreen::vfunction23_for_Widget
/// `Update()`: redraws every update.
pub fn vfunction23_for_Widget(g: &mut G, this: Ptr) {
    crate::sexy::widget_container::vfunction23(g, this);
    vcall!(g, this, w.vfunction18);
}

/// port: 00519450 Sexy::SimSetupScreen::vfunction2_for_ButtonListener
/// `ButtonPress(int)`: the click sound.
pub fn vfunction2_for_ButtonListener(g: &mut G, this: Ptr, _id: i32) {
    let app = g.sim_setup(this).field_0x0;
    let s = g.res.DAT_005e8c40;
    crate::sexy::sexy_app_base::vfunction55(g, app, s);
}

/// port: 005193d0 FUN_005193d0
/// Two pairs of rivets (`DAT_005e8b18`) at x (EBX) and x + 30, at y (EAX) and y + `param_1`.
pub fn FUN_005193d0(g: &mut G, gfx: &mut Graphics, x: i32, y: i32, param_1: i32) {
    let img = g.res.DAT_005e8b18;
    FUN_00455d20(gfx, g, img, x, y);
    FUN_00455d20(gfx, g, img, x + 0x1e, y);
    FUN_00455d20(gfx, g, img, x, y + param_1);
    FUN_00455d20(gfx, g, img, x + 0x1e, y + param_1);
}

/// port: 00506920 FUN_00506920
/// A checkbox's label: right of the box (43 pixels in), one line 24 down, or two lines (15
/// and 30 down) when `param_4` is given; in the coordinates of a Graphics translated to the
/// box's parent.
pub fn FUN_00506920(g: &mut G, param_1: &mut Graphics, param_2: &[u8], param_3: Ptr, param_4: Option<&[u8]>) {
    let ftol = crate::sexy::crt::ftol;
    let (bx, by) = (g.wc(param_3).offset_0x2c, g.wc(param_3).offset_0x30);
    let (tx, ty) = (param_1.s.mTransX, param_1.s.mTransY);
    let x = |g: &mut G| -> i32 {
        let _ = g;
        ftol((bx as f32 - tx) as f64 + 43.0) as i32
    };
    match param_4 {
        None => {
            let y = ftol((by as f32 - ty) as f64 + 24.0) as i32;
            let xx = x(g);
            FUN_00455cf0(param_1, g, param_2, xx, y);
        }
        Some(l2) => {
            let y = ftol((by as f32 - ty) as f64 + 15.0) as i32;
            let xx = x(g);
            FUN_00455cf0(param_1, g, param_2, xx, y);
            let y = ftol((by as f32 - ty) as f64 + 30.0) as i32;
            let xx = x(g);
            FUN_00455cf0(param_1, g, l2, xx, y);
        }
    }
}

/// port: 0052a6d0 Sexy::SimSetupScreen::vfunction27_for_Widget
/// `Draw(Graphics*)`: the frame, title bar and panels with their rivets, "Tank Setup", the
/// current backdrop in its frame, the checkbox labels, the help line for the hovered
/// control, and the screensaver state (and its user when enabled) under its button.
pub fn vfunction27_for_Widget(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    use crate::game::help_screen::{FUN_00503390, FUN_00503400};
    let frame = g.res.DAT_005e8c70;
    FUN_004563f0(gfx, g, &Rect::new(-5, -5, 0x28a, 0x1ea), frame);
    let bar = g.res.DAT_005e8d3c;
    let h = g.image(bar).offset_0x24;
    FUN_004563f0(gfx, g, &Rect::new(0x14, 0, 600, h), bar);
    let panel = g.res.DAT_005e8a58;
    FUN_00503390(g, gfx, panel, 0xd, 400, 0x266);
    let rivet = g.res.DAT_005e8b18;
    FUN_00455d20(gfx, g, rivet, 0x21, 0x181);
    FUN_00455d20(gfx, g, rivet, 0x24b, 0x181);
    let vbar = g.res.DAT_005e8cbc;
    FUN_00503400(g, gfx, vbar, 0x136, 0x69, 0x12a);
    FUN_00503390(g, gfx, panel, 0xd, 100, 0x266);
    FUN_00455d20(gfx, g, rivet, 0x21, 0x73);
    FUN_00455d20(gfx, g, rivet, 0x24b, 0x73);
    FUN_005193d0(g, gfx, 0x127, 0x73, 0x10f);
    let f = g.res.DAT_005e8e40;
    FUN_00455880(gfx, f);
    FUN_00455890(gfx, CRect(0xff, 200, 0, 0xff));
    vcall!(g, this, w.vfunction63, gfx, 0x19, b"Tank Setup");

    let d = g.sim_setup(this).clone();
    let bframe = g.res.DAT_005e8ba4;
    FUN_00455d20(gfx, g, bframe, d.field_0x4, d.field_0x8);
    let (ix, iy) = (d.field_0x4 + 0x10, d.field_0x8 + 10);
    let board = g.wfa(d.field_0x0).offset_0x4;
    let id = g.board(board).field_0x35c + 0x61;
    let thumb = crate::sexy::res::FUN_005016a0(g, id) as Ptr;
    FUN_00455d20(gfx, g, thumb, ix, iy);
    let f = g.res.DAT_005e8a5c;
    FUN_00455880(gfx, f);
    FUN_00455890(gfx, FUN_00433320(0xffffff));
    let fnt = FUN_00455870(gfx);
    let s: &[u8] = b"Current Backdrop";
    let sw = font::string_width(g, fnt, s);
    let fw = g.image(bframe).offset_0x20;
    FUN_00455cf0(gfx, g, s, (fw - sw) / 2 + d.field_0x4, d.field_0x8 - 5);

    let f = g.res.DAT_005e8cf0;
    FUN_00455880(gfx, f);
    FUN_00455890(gfx, FUN_00433320(0xffffff));
    FUN_00506920(g, gfx, b"Show Fish Names", d.offset_0x24, None);
    FUN_00506920(g, gfx, b"Show Bubbulator", d.offset_0x28, None);
    FUN_00506920(g, gfx, b"Enable Alien Attractor", d.offset_0x2c, None);
    FUN_00506920(g, gfx, b"Allow Fish To Drop Shells", d.offset_0x30, None);
    FUN_00506920(g, gfx, b"Always Show When", d.offset_0x34, Some(b"Fish Are Hungry"));

    let over = |g: &mut G, p: Ptr| g.w(p).offset_0x5;
    let over_down = |g: &mut G, p: Ptr| g.w(p).offset_0x5 || g.w(p).offset_0x4;
    let help: &[u8] = if over(g, d.offset_0x24) {
        b"This checkbox controls whether or not to display the names of your fish in your Virtual Tank."
    } else if over(g, d.offset_0x28) {
        b"This checkbox controls whether or not to display the Bubbulator in your Virtual Tank."
    } else if over(g, d.offset_0x2c) {
        b"This checkbox controls whether or not to display the Alien Attractor in your Virtual Tank."
    } else if over(g, d.offset_0x30) {
        b"This checkbox controls whether fish will drop shells in your Virtual Tank."
    } else if over(g, d.offset_0x34) {
        b"Virtual Fish only need to be fed three times per day.  After that, they will not look hungry unless you check this checkbox."
    } else if over_down(g, d.offset_0x20) {
        b"Click to set your Virtual Tank as your computer's screensaver.\nThe screensaver will display the Virtual Tank belonging to the user who last enabled it."
    } else if over_down(g, d.offset_0x1c) {
        b"You can sell your tank backdrops if you're in need of more Shells."
    } else if over_down(g, d.offset_0x14) || over_down(g, d.offset_0x18) {
        b"You can choose which tank backdrop to display in Virtual Tank.  You can buy additional backdrops from the Virtual Tank Store."
    } else {
        b"Welcome to the Virtual Tank setup screen!"
    };
    if !help.is_empty() {
        let f = g.res.DAT_005e8cf0;
        FUN_00455880(gfx, f);
        FUN_00455890(gfx, FUN_00433320(0xffffaa));
        let h = vcall!(g, this, w.vfunction66, gfx, 0x208, help, -1);
        let r = Rect::new(0x3c, 0x46 - h / 2, 0x208, 0);
        vcall!(g, this, w.vfunction65, gfx, r, help, -1, 0);
    }
    FUN_00455890(gfx, FUN_00433320(0xffffff));
    let ss = g.wc(d.offset_0x20).clone();
    let cx = ss.offset_0x34 / 2 + ss.offset_0x2c;
    let mut y = ss.offset_0x38 + 0x14 + ss.offset_0x30;
    let on = g.wfa(d.field_0x0).offset_0x10c;
    let s = format!("The Screensaver is {}", if on { "enabled" } else { "disabled" }).into_bytes();
    let fnt = FUN_00455870(gfx);
    let sw = font::string_width(g, fnt, &s);
    FUN_00455cf0(gfx, g, &s, cx - sw / 2, y);
    y += 0xf;
    if on {
        let user = String::from_utf8_lossy(&g.wfa(d.field_0x0).field_0x130).into_owned();
        let s = format!("User: {user}").into_bytes();
        let fnt = FUN_00455870(gfx);
        let sw = font::string_width(g, fnt, &s);
        FUN_00455cf0(gfx, g, &s, cx - sw / 2, y);
    }
}

/// port: 0052ae40 Sexy::SimSetupScreen::vfunction3_for_ButtonListener
/// `ButtonDepress(int id)`: "Return to Tank" (9) closes the screen, applies the checkboxes
/// to the board (no Alien Attractor resets its timer to 3000), saves the tank, re-enters the
/// virtual tank when something changed, and unpauses. Prev / Next (0, 1) step to the
/// previous / next owned backdrop (the current one counts as owned). Sell (2) asks to
/// confirm (refund 10000), unless it is the only backdrop. "Screensaver..." (8) opens the
/// screensaver dialog.
pub fn vfunction3_for_ButtonListener(g: &mut G, this: Ptr, param_1: i32) {
    let app = g.sim_setup(this).field_0x0;
    match param_1 {
        9 => {
            FUN_0054af50(g, app);
            let d = g.sim_setup(this).clone();
            let board = g.wfa(app).offset_0x4;
            let c = |g: &mut G, p: Ptr| g.checkbox(p).offset_0x8;
            let v = (c(g, d.offset_0x24), c(g, d.offset_0x28), c(g, d.offset_0x2c), c(g, d.offset_0x30), c(g, d.offset_0x34));
            let b = g.board(board);
            b.ext_0x4fc = v.0;
            b.ext_0x4fd = v.1;
            b.ext_0x4fe = v.2;
            b.ext_0x500 = v.3;
            b.ext_0x4ff = v.4;
            if !b.ext_0x4fe {
                b.field_0x234 = 3000;
            }
            crate::game::board_save::FUN_00538940(g, board);
            if g.sim_setup(this).field_0xc {
                crate::game::store::FUN_0054aff0(g, app);
            }
            let board = g.wfa(app).offset_0x4;
            crate::game::board_update::FUN_0053db80(g, board, false);
        }
        0 | 1 => {
            g.sim_setup(this).field_0xc = true;
            let board = g.wfa(app).offset_0x4;
            let profile = g.wfa(app).offset_0x18c;
            let mut i = g.board(board).field_0x35c - 1;
            g.profile(profile).field_0x72[i as usize] = true;
            loop {
                i += (param_1 != 0) as i32 * 2 - 1;
                if i < 0 {
                    i = 5;
                } else if 5 < i {
                    i = 0;
                }
                if g.profile(profile).field_0x72[i as usize] {
                    break;
                }
            }
            crate::game::board_level::FUN_00538a10(g, board, i + 1);
        }
        2 => {
            let profile = g.wfa(app).offset_0x18c;
            let n = g.profile(profile).field_0x72.iter().filter(|&&b| b).count();
            if n == 1 {
                crate::game::win_fish_app::vfunction73(g, app, 0xe, true, b"Not Allowed", b"You are not allowed to sell your only backdrop!", b"OK", 3);
            } else {
                let msg = format!("Are you sure that you want to sell this backdrop?\n\nRefund Value: {} Shells", 10000);
                FUN_0054fc90(g, app, msg.as_bytes());
            }
        }
        8 => FUN_0054fef0(g, app),
        _ => {}
    }
}

/// port: 0052b110 Sexy::SimSetupScreen::vfunction1_for_CheckboxListener
/// `CheckboxChecked(int id, bool checked)`: the click sound, marks a change; turning on the
/// Bubbulator (4) or Alien Attractor (5) without owning it says so and unchecks it.
pub fn vfunction1_for_CheckboxListener(g: &mut G, this: Ptr, param_1: i32, param_2: bool) {
    let app = g.sim_setup(this).field_0x0;
    let s = g.res.DAT_005e8c40;
    crate::sexy::sexy_app_base::vfunction55(g, app, s);
    g.sim_setup(this).field_0xc = true;
    let profile = g.wfa(app).offset_0x18c;
    if param_1 == 4 {
        if param_2 && g.profile(profile).field_0x90 == 0 {
            crate::game::win_fish_app::vfunction73(g, app, 0xe, true, b"Unavailable", b"You must buy the bubbulator in order to use this option.", b"OK", 3);
            let c = g.sim_setup(this).offset_0x28;
            g.checkbox(c).offset_0x8 = false;
        }
    } else if param_1 == 5 && param_2 && g.profile(profile).field_0x94 == 0 {
        crate::game::win_fish_app::vfunction73(g, app, 0xe, true, b"Unavailable", b"You must buy the alien attractor in order to use this option.", b"OK", 3);
        let c = g.sim_setup(this).offset_0x2c;
        g.checkbox(c).offset_0x8 = false;
    }
}

/// port: 00551c90 FUN_00551c90
/// `ShowSimSetupScreen()`: re-reads the screensaver settings, closes the dialogs and any
/// setup screen, pauses the board, and opens the setup screen full-screen.
pub fn FUN_00551c90(g: &mut G, this: Ptr) {
    FUN_0054e2d0(g, this);
    crate::game::win_fish_app::FUN_0054b4c0(g, this);
    FUN_0054af50(g, this);
    let board = g.wfa(this).offset_0x4;
    if board != NULL {
        crate::game::board_update::FUN_0053db80(g, board, true);
    }
    let s = SimSetupScreen(g, this);
    g.wfa(this).offset_0xf8 = s;
    let (w, h) = (g.sab(this).field_0xb8, g.sab(this).field_0xbc);
    vcall!(g, s, w.vfunction41, 0, 0, w, h);
    let wm = g.sab(this).offset_0x318;
    vcall!(g, wm, w.vfunction4, s);
}

/// port: 0054af50 FUN_0054af50
/// `RemoveSimSetupScreen()` (+0x824).
pub fn FUN_0054af50(g: &mut G, this: Ptr) {
    let s = g.wfa(this).offset_0xf8;
    if s != NULL {
        let wm = g.sab(this).offset_0x318;
        vcall!(g, wm, w.vfunction5, s);
        crate::sexy::sexy_app_base::vfunction35(g, this, s);
        g.wfa(this).offset_0xf8 = NULL;
    }
}

/// port: 0054e2d0 FUN_0054e2d0
/// `ReadScreenSaverSettings()`: the screensaver's registry booleans (Sound, RotateBackDrops,
/// PeriodicDim, ShowMoney, Powersave under "ScreenSaver\"), then whether the system
/// screensaver is this game's (+0x838): it becomes true when they match and false when they
/// differ.
pub fn FUN_0054e2d0(g: &mut G, this: Ptr) {
    use crate::sexy::app_host::registry_read_boolean;
    let base = String::from_utf8_lossy(&g.wfa(this).field_0x5c).into_owned();
    if let Some(v) = registry_read_boolean(g, &format!("{base}Sound")) {
        g.wfa(this).offset_0x10d = v;
    }
    if let Some(v) = registry_read_boolean(g, &format!("{base}RotateBackDrops")) {
        g.wfa(this).offset_0x10e = v;
    }
    if let Some(v) = registry_read_boolean(g, &format!("{base}PeriodicDim")) {
        g.wfa(this).offset_0x10f = v;
    }
    if let Some(v) = registry_read_boolean(g, &format!("{base}ShowMoney")) {
        g.wfa(this).offset_0x110 = v;
    }
    if let Some(v) = registry_read_boolean(g, &format!("{base}Powersave")) {
        g.wfa(this).offset_0x112 = v;
    }
    let ours = FUN_0054d230(g, this).0;
    let system = FUN_0054d0c0(g);
    let same = ours == system;
    if !g.wfa(this).offset_0x10c {
        if same {
            g.wfa(this).offset_0x10c = true;
        }
    } else if !same {
        g.wfa(this).offset_0x10c = false;
    }
}

/// port: 0054d230 FUN_0054d230
/// `GetScreenSaverPaths()`: the screensaver file, "Insaniquarium.scr" in the app-data folder
/// (`gApp` +0x98, its trailing slash removed) on NT 6+, else beside the executable, and
/// shortened when the file exists (ECX); and the copy source (EDI): the executable's name
/// in the app-data folder on NT 6+, else the executable's path. (Host boundary: the OS is
/// NT 6+, the executable is the running one, and a path is never shortened.)
pub fn FUN_0054d230(g: &mut G, this: Ptr) -> (Vec<u8>, Vec<u8>) {
    let _ = this;
    let gapp = g.globals.DAT_005eb6a4;
    let mut dir = g.sab(gapp).field_0x90.clone();
    if matches!(dir.last(), Some(b'\\') | Some(b'/')) {
        dir.pop();
    }
    let exe = std::env::current_exe().ok().and_then(|p| p.file_name().map(|n| n.to_string_lossy().into_owned())).unwrap_or_default();
    let mut copy = dir.clone();
    copy.push(b'\\');
    copy.extend_from_slice(exe.as_bytes());
    dir.push(b'\\');
    dir.extend_from_slice(b"Insaniquarium.scr");
    (dir, copy)
}

/// port: 0054d0c0 FUN_0054d0c0
/// `GetSystemScreenSaver()`: HKCU "Control Panel\Desktop" `SCRNSAVE.EXE` (a string value;
/// anything else gives ""). (Host boundary: read from the replaced registry.)
pub fn FUN_0054d0c0(g: &mut G) -> Vec<u8> {
    crate::sexy::app_host::registry_read_string(g, "Control Panel\\Desktop\\SCRNSAVE.EXE").map(|s| s.into_bytes()).unwrap_or_default()
}

/// port: 0054fc90 FUN_0054fc90
/// `ConfirmSellBackdrop(const string& theLines)`: dialog 0x24 "ARE YOU SURE?" (yes/no) with
/// a "Sell" button, at y 60 (250 over the tank screen +0x828).
pub fn FUN_0054fc90(g: &mut G, this: Ptr, param_1: &[u8]) {
    let d = crate::game::win_fish_app::vfunction73(g, this, 0x24, true, b"ARE YOU SURE?", param_1, b"", 2);
    let yes = g.dialog(d).offset_0x8;
    g.btn(yes).field_0x4 = b"Sell".to_vec();
    let y = if g.wfa(this).offset_0xfc == NULL { 0x3c } else { 0xfa };
    let wc = g.wc(d).clone();
    vcall!(g, d, w.vfunction41, wc.offset_0x2c, y, wc.offset_0x34, wc.offset_0x38);
}

/// port: 0054fef0 FUN_0054fef0
/// `ShowScreenSaverDialog()`: re-reads the screensaver settings and shows dialog 0x25, 420
/// wide at (110, 40).
pub fn FUN_0054fef0(g: &mut G, this: Ptr) {
    FUN_0054e2d0(g, this);
    let d = FUN_00535e10(g, this);
    let h = vcall!(g, d, dlg.vfunction74, 0x1a4);
    vcall!(g, d, w.vfunction41, 0x6e, 0x28, 0x1a4, h);
    crate::sexy::sexy_app_base::vfunction76(g, this, 0x25, d);
}

/// port: 00519470 FUN_00519470
/// The backdrop sale answered: on yes (and when another backdrop is owned), marks a change,
/// gives up the current backdrop for 10000 shells and shows the next owned one.
pub fn FUN_00519470(g: &mut G, this: Ptr, param_1: bool) {
    if !param_1 {
        return;
    }
    g.sim_setup(this).field_0xc = true;
    let app = g.sim_setup(this).field_0x0;
    let profile = g.wfa(app).offset_0x18c;
    let board = g.wfa(app).offset_0x4;
    let mut cur = g.board(board).field_0x35c - 1;
    let owned = g.profile(profile).field_0x72;
    let other = (0..6).any(|i| owned[i as usize] && cur != i);
    if !other {
        return;
    }
    g.profile(profile).field_0x72[cur as usize] = false;
    crate::game::profile::FUN_00501200(g, profile, 10000);
    loop {
        cur += 1;
        if cur < 0 {
            cur = 5;
        } else if 5 < cur {
            cur = 0;
        }
        if g.profile(profile).field_0x72[cur as usize] {
            break;
        }
    }
    crate::game::board_level::FUN_00538a10(g, board, cur + 1);
}

/// port: 0054b8a0 FUN_0054b8a0
/// "ARE YOU SURE?" (selling) answered: passed to the tank screen (+0x828) or the setup
/// screen (+0x824), then the dialog closes.
pub fn FUN_0054b8a0(g: &mut G, this: Ptr, param_1: bool) {
    let tank = g.wfa(this).offset_0xfc;
    if tank != NULL {
        crate::game::sim_fish::FUN_0052a090(g, tank, param_1);
        crate::game::win_fish_app::vfunction78(g, this, 0x24);
        return;
    }
    let setup = g.wfa(this).offset_0xf8;
    if setup != NULL {
        FUN_00519470(g, setup, param_1);
    }
    crate::game::win_fish_app::vfunction78(g, this, 0x24);
}

/// `ScreenSaverDialog_data` (object offset 0x15c): its six checkboxes.
#[derive(Debug, Clone, Default)]
pub struct ScreenSaverDialog_data {
    /// +0x15c "Enable Screensaver" (id 0; app +0x838).
    pub offset_0x0: Ptr,
    /// +0x160 "Enable Sound" (id 1; app +0x839).
    pub offset_0x4: Ptr,
    /// +0x164 "Rotate Backdrops" (id 2; app +0x83a).
    pub offset_0x8: Ptr,
    /// +0x168 "Periodically Darken Screen" (id 3; app +0x83b).
    pub offset_0xc: Ptr,
    /// +0x16c "Show Shells Collected" (id 4; app +0x83c).
    pub offset_0x10: Ptr,
    /// +0x170 "Allow Monitor Powersave Mode" (id 5; app +0x83e).
    pub offset_0x14: Ptr,
}

impl G {
    pub fn screensaver_dialog(&mut self, p: Ptr) -> &mut ScreenSaverDialog_data {
        use crate::sexy::dialog::{DlgSub, MoneySub};
        match &mut self.widget(p).ext {
            WExt::Dialog(e) => match &mut e.sub {
                DlgSub::Money(_, MoneySub::ScreenSaver(d)) => d,
                s => panic!("{p} is not a ScreenSaverDialog: {s:?}"),
            },
            e => panic!("{p} is not a Dialog: {e:?}"),
        }
    }
}

fn ssd_boxes(d: &ScreenSaverDialog_data) -> [Ptr; 6] {
    [d.offset_0x0, d.offset_0x4, d.offset_0x8, d.offset_0xc, d.offset_0x10, d.offset_0x14]
}

/// port: 00535e10 FUN_00535e10
/// `ScreenSaverDialog(WinFishApp*)`: dialog 0x25 "Screensaver", OK/Cancel, with checkboxes
/// at the app's screensaver settings.
pub fn FUN_00535e10(g: &mut G, param_1: Ptr) -> Ptr {
    use crate::sexy::dialog::{alloc_dialog, DlgSub, MoneySub};
    let this = alloc_dialog(
        g,
        &crate::sexy::vtables_gen::Sexy__ScreenSaverDialog_vftable,
        DlgSub::Money(crate::game::money_dialog::MoneyDialog_data::default(), MoneySub::ScreenSaver(ScreenSaverDialog_data::default())),
    );
    let (comp, btn) = (g.res.DAT_005e8cc0, g.res.DAT_005e8c54);
    crate::game::money_dialog::MoneyDialog(g, this, param_1, comp, btn, 0x25, true, b"Screensaver", b"", b"", 2);
    let app = g.money(this).offset_0x50;
    let mk = crate::game::options_dialog::FUN_00500b70;
    let v = g.wfa(app).offset_0x10c;
    let c0 = mk(g, 0, this, v);
    let w = g.wfa(param_1);
    let (v1, v2, v3, v4, v5) = (w.offset_0x10d, w.offset_0x10e, w.offset_0x10f, w.offset_0x110, w.offset_0x112);
    let c1 = mk(g, 1, this, v1);
    let c2 = mk(g, 2, this, v2);
    let c3 = mk(g, 3, this, v3);
    let c4 = mk(g, 4, this, v4);
    let c5 = mk(g, 5, this, v5);
    *g.screensaver_dialog(this) = ScreenSaverDialog_data { offset_0x0: c0, offset_0x4: c1, offset_0x8: c2, offset_0xc: c3, offset_0x10: c4, offset_0x14: c5 };
    this
}

/// port: 005316b0 Sexy::ScreenSaverDialog::~ScreenSaverDialog
pub fn dtor_ScreenSaverDialog(g: &mut G, this: Ptr) {
    let d = g.screensaver_dialog(this).clone();
    for p in ssd_boxes(&d) {
        if p != NULL {
            vcall!(g, p, w.vfunction1, 1);
        }
    }
    crate::game::money_dialog::dtor_MoneyDialog(g, this);
}

/// port: 00532b60 Sexy::ScreenSaverDialog::deleting_destructor
pub fn deleting_destructor__00532b60(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_ScreenSaverDialog(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 00531790 Sexy::ScreenSaverDialog::vfunction74
/// `GetPreferredHeight(int)`: 440.
pub fn vfunction74__00531790(_g: &mut G, _this: Ptr, _param_1: i32) -> i32 {
    0x1b8
}

/// port: 005317a0 Sexy::ScreenSaverDialog::vfunction21
/// `AddedToManager(WidgetManager*)`: adds the checkboxes.
pub fn vfunction21__005317a0(g: &mut G, this: Ptr, param_1: Ptr) {
    crate::sexy::dialog::vfunction21_for_Widget(g, this, param_1);
    let d = g.screensaver_dialog(this).clone();
    for p in ssd_boxes(&d) {
        vcall!(g, param_1, w.vfunction4, p);
    }
}

/// port: 00531820 Sexy::ScreenSaverDialog::vfunction22
/// `RemovedFromManager(WidgetManager*)`.
pub fn vfunction22__00531820(g: &mut G, this: Ptr, param_1: Ptr) {
    crate::sexy::dialog::vfunction22_for_Widget(g, this, param_1);
    let d = g.screensaver_dialog(this).clone();
    for p in ssd_boxes(&d) {
        vcall!(g, param_1, w.vfunction5, p);
    }
}

/// port: 005318a0 Sexy::ScreenSaverDialog::vfunction41
/// `Resize(int x, int y, int w, int h)`: the checkboxes stacked from (33, 100): enable,
/// powersave, sound, rotate, darken, shells.
pub fn vfunction41__005318a0(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32, param_4: i32) {
    use crate::sexy::widget::FUN_0046e880;
    crate::sexy::dialog::vfunction41_for_Widget(g, this, param_1, param_2, param_3, param_4);
    let d = g.screensaver_dialog(this).clone();
    let (x, y) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
    vcall!(g, d.offset_0x0, w.vfunction41, x + 0x21, y + 100, 0x2e, 0x2d);
    FUN_0046e880(g, d.offset_0x14, 0x1203, d.offset_0x0, 0, 0, 0, 0);
    FUN_0046e880(g, d.offset_0x4, 0x1203, d.offset_0x14, 0, 0, 0, 0);
    FUN_0046e880(g, d.offset_0x8, 0x1203, d.offset_0x4, 0, 0, 0, 0);
    FUN_0046e880(g, d.offset_0xc, 0x1203, d.offset_0x8, 0, 0, 0, 0);
    FUN_0046e880(g, d.offset_0x10, 0x1203, d.offset_0xc, 0, 0, 0, 0);
}

/// port: 00531980 Sexy::ScreenSaverDialog::vfunction78
/// `CheckboxChecked(int, bool)`: the MoneyDialog's (the click sound).
pub fn vfunction78__00531980(g: &mut G, this: Ptr, param_1: i32, param_2: bool) {
    crate::game::money_dialog::vfunction78_for_Widget(g, this, param_1, param_2);
}

/// port: 00532370 Sexy::ScreenSaverDialog::vfunction1
/// The CheckboxListener part's `CheckboxChecked(int, bool)`: forwards to the dialog's.
pub fn vfunction1__00532370(g: &mut G, this: Ptr, param_1: i32, param_2: bool) {
    vcall!(g, this, money.vfunction78, param_1, param_2);
}

/// port: 00535fe0 Sexy::ScreenSaverDialog::vfunction27
/// `Draw(Graphics*)`: the dialog, the checkbox labels, and the help line for the hovered
/// checkbox (or the title line).
pub fn vfunction27__00535fe0(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    crate::sexy::dialog::vfunction27_for_Widget(g, this, gfx);
    let d = g.screensaver_dialog(this).clone();
    FUN_00506920(g, gfx, b"Enable Screensaver", d.offset_0x0, None);
    FUN_00506920(g, gfx, b"Allow Monitor Powersave Mode", d.offset_0x14, None);
    FUN_00506920(g, gfx, b"Enable Sound", d.offset_0x4, None);
    FUN_00506920(g, gfx, b"Rotate Backdrops", d.offset_0x8, None);
    FUN_00506920(g, gfx, b"Periodically Darken Screen", d.offset_0xc, None);
    FUN_00506920(g, gfx, b"Show Shells Collected", d.offset_0x10, None);
    let w = g.wc(this).offset_0x34 - 0x50;
    let over = |g: &mut G, p: Ptr| g.w(p).offset_0x5;
    let help: &[u8] = if over(g, d.offset_0x0) {
        b"Check here to set your Virtual Tank as your Screensaver."
    } else if over(g, d.offset_0x14) {
        b"Check here to allow your monitor to go into powersave mode while running the Insaniquarium Screensaver."
    } else if over(g, d.offset_0x4) {
        b"This option controls whether or not to play sounds in the Screensaver."
    } else if over(g, d.offset_0x8) {
        b"Setting this option will cause the Screensaver to periodically rotate among your purchased tank backdrops."
    } else if over(g, d.offset_0xc) {
        b"This option will make the Screensaver periodically darken the screen."
    } else if over(g, d.offset_0x10) {
        b"This option will have the Screensaver show you how many Shells you have collected while it has been running."
    } else {
        b"Insaniquarium Screensaver Settings"
    };
    if !help.is_empty() {
        let f = g.res.DAT_005e8cf0;
        FUN_00455880(gfx, f);
        FUN_00455890(gfx, FUN_00433320(0xffffaa));
        let h = vcall!(g, this, w.vfunction66, gfx, w, help, -1);
        let r = Rect::new(0x28, 0x46 - h / 2, w, 0);
        vcall!(g, this, w.vfunction65, gfx, r, help, -1, 0);
    }
}

/// `SexyAppBase::FileExists` (@ 0047fb10, framework): here, whether the game's files hold
/// it (the port installs no screensaver file, so the .scr is never found).
fn file_exists(g: &mut G, path: &[u8]) -> bool {
    let p = String::from_utf8_lossy(path).into_owned();
    g.vfs.read(&p).is_some()
}

/// port: 0054ab40 FUN_0054ab40
/// `GetFileTime(path, &lastWrite)` (path in EAX). Host boundary: the in-memory file system
/// keeps no times, so it never succeeds.
pub fn FUN_0054ab40(_g: &mut G, _path: &[u8]) -> Option<u64> {
    None
}

/// port: 0054ab90 FUN_0054ab90
/// `SetFileTime(path, &lastWrite)` (path in EAX); see `FUN_0054ab40`.
pub fn FUN_0054ab90(_g: &mut G, _path: &[u8], _time: u64) -> bool {
    false
}

/// port: 0054d580 FUN_0054d580
/// `MakeScreensaverDat(const string& scr)`: `screensaver.dat` (in the application data
/// folder on NT 6+, else beside the .scr) is made a copy of the .scr unless both exist with
/// the same time; the copy's first byte becomes 'N' and it gets the source's time. False
/// when the copy or the patch failed.
pub fn FUN_0054d580(g: &mut G, _this: Ptr, param_1: &[u8]) -> bool {
    let src = param_1.to_vec();
    let dir: Vec<u8> = if crate::sexy::app_host::is_nt6(g) {
        crate::sexy::app_host::app_data_folder(g)
    } else {
        let s = &src;
        let cut = s.iter().rposition(|&c| c == 0x5c || c == b'/').map_or(0, |i| i);
        s[..cut].to_vec()
    };
    let mut dest = dir;
    dest.extend_from_slice(b"\\screensaver.dat");
    let t_src = FUN_0054ab40(g, &src);
    let t_dst = FUN_0054ab40(g, &dest);
    let differ = match (t_src, t_dst) {
        (Some(a), Some(b)) => a != b,
        _ => true,
    };
    if !differ {
        return true;
    }
    let (sp, dp) = (String::from_utf8_lossy(&src).into_owned(), String::from_utf8_lossy(&dest).into_owned());
    let Some(data) = g.vfs.read(&sp).map(|d| d.to_vec()) else { return false };
    let mut data = data;
    if data.is_empty() {
        data.push(0);
    }
    data[0] = b'N';
    g.vfs.write(&dp, data);
    if let Some(t) = t_src {
        FUN_0054ab90(g, &dest, t);
    }
    true
}

/// port: 0054ff90 FUN_0054ff90
/// OK in "Screensaver": enabling remembers the previous system screensaver (when it is not
/// ours) and, when our .scr exists (copying it first under the "ScrCopy" property), turns
/// the system screensaver on and makes it ours, else leaves the screensaver disabled;
/// disabling restores the previous one. Then the other settings, the screensaver's user
/// (the current player when enabled), the registry, and the dialog closes.
/// (Host boundary: the system calls are not made; see the module note.)
pub fn FUN_0054ff90(g: &mut G, this: Ptr) {
    use crate::sexy::app_host::registry_write;
    let d = crate::sexy::sexy_app_base::vfunction74(g, this, 0x25);
    if d == NULL {
        return;
    }
    let boxes = g.screensaver_dialog(d).clone();
    let enable = g.checkbox(boxes.offset_0x0).offset_0x8;
    let (ours, copy_from) = FUN_0054d230(g, this);
    if !enable {
        FUN_0054ca60(g, this);
    } else {
        let system = FUN_0054d0c0(g);
        if system != ours {
            g.wfa(this).field_0x114 = system;
        }
        if !file_exists(g, &ours) {
            g.wfa(this).offset_0x10c = false;
        } else {
            let copy = crate::sexy::app_host::FUN_00487170(g, "ScrCopy", false);
            let failed = copy && !FUN_0054d580(g, this, &copy_from);
            if failed {
                crate::sexy::app_host::popup(g, b"Failed to make screensaver.dat");
                g.wfa(this).offset_0x10c = false;
            } else {
                // SystemParametersInfoA(SPI_GETSCREENSAVEACTIVE / SPI_SETSCREENSAVEACTIVE)
                // is not made.
                FUN_0054aa40(g, &ours);
                g.wfa(this).offset_0x10c = true;
            }
        }
    }
    let c = |g: &mut G, p: Ptr| g.checkbox(p).offset_0x8;
    let v = [c(g, boxes.offset_0x4), c(g, boxes.offset_0x8), c(g, boxes.offset_0xc), c(g, boxes.offset_0x10), c(g, boxes.offset_0x14)];
    let w = g.wfa(this);
    w.offset_0x10d = v[0];
    w.offset_0x10e = v[1];
    w.offset_0x10f = v[2];
    w.offset_0x110 = v[3];
    w.offset_0x112 = v[4];
    let profile = g.wfa(this).offset_0x18c;
    g.wfa(this).field_0x130 = if enable && profile != NULL { g.profile(profile).field_0x24.clone() } else { Vec::new() };
    let key = format!("{}User", String::from_utf8_lossy(&g.wfa(this).field_0x5c));
    let user = String::from_utf8_lossy(&g.wfa(this).field_0x130).into_owned();
    registry_write(g, &key, user);
    FUN_0054e550(g, this);
    crate::game::win_fish_app::vfunction78(g, this, 0x25);
}

/// port: 0054ca60 FUN_0054ca60
/// `DisableScreenSaver()`: not ours any more (`DAT_005e9634` set), and the previous system
/// screensaver (+0x840) back.
pub fn FUN_0054ca60(g: &mut G, this: Ptr) {
    g.wfa(this).offset_0x10c = false;
    g.globals.DAT_005e9634 = true;
    let old = g.wfa(this).field_0x114.clone();
    FUN_0054aa40(g, &old);
}

/// port: 0054aa40 FUN_0054aa40
/// `SetSystemScreenSaver(const char*)` (ESI): HKCU "Control Panel\Desktop" `SCRNSAVE.EXE`.
/// (Host boundary: written to the replaced registry.)
pub fn FUN_0054aa40(g: &mut G, param_1: &[u8]) {
    let v = String::from_utf8_lossy(param_1).into_owned();
    crate::sexy::app_host::registry_write(g, "Control Panel\\Desktop\\SCRNSAVE.EXE", v);
}

/// port: 0054e550 FUN_0054e550
/// `WriteScreenSaverSettings()`: OldPath, Enabled, Sound, RotateBackDrops, PeriodicDim,
/// ShowMoney and Powersave under "ScreenSaver\".
pub fn FUN_0054e550(g: &mut G, this: Ptr) {
    use crate::sexy::app_host::registry_write;
    let w = g.wfa(this);
    let base = String::from_utf8_lossy(&w.field_0x5c).into_owned();
    let old = String::from_utf8_lossy(&w.field_0x114).into_owned();
    let v = [w.offset_0x10c, w.offset_0x10d, w.offset_0x10e, w.offset_0x10f, w.offset_0x110, w.offset_0x112];
    registry_write(g, &format!("{base}OldPath"), old);
    // `RegistryWriteBoolean` stores 1 / 0.
    let b = |v: bool| if v { "1".to_string() } else { "0".to_string() };
    registry_write(g, &format!("{base}Enabled"), b(v[0]));
    registry_write(g, &format!("{base}Sound"), b(v[1]));
    registry_write(g, &format!("{base}RotateBackDrops"), b(v[2]));
    registry_write(g, &format!("{base}PeriodicDim"), b(v[3]));
    registry_write(g, &format!("{base}ShowMoney"), b(v[4]));
    registry_write(g, &format!("{base}Powersave"), b(v[5]));
}
