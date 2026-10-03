//! `Sexy::HighScoreScreen`: the Hall of Fame (main menu): tabs for Adventure, Time Trial,
//! Challenge and the player's Personal Records, over a blue gradient with bubbles. Plus the
//! app functions that open and close it, and the high-score table lookups it uses.
//!
//! `HighScoreScreen_data` starts at object offset 0x8c (after the ButtonListener vftable at
//! 0x88). With 3D acceleration every line bobs (a D3D transform pushed per line); here that
//! is the Graphics translation, the same offset for the blits.

use crate::game::high_score::HighScore;
use crate::sexy::graphics::{FUN_00455870, FUN_00455880, FUN_00455890, FUN_00455920, FUN_00455cf0, FUN_004563f0};
use crate::sexy::image_font as font;
use crate::sexy::prelude::*;
use crate::sexy::types::{CRect, FUN_00433320, FUN_00433360};

/// `HighScoreScreen_data` (object offset 0x8c).
#[derive(Debug, Clone, Default)]
pub struct HighScoreScreen_data {
    /// +0x8c the app.
    pub field_0x0: Ptr,
    /// +0x90 the bubbles (`BubbleMgr`, owned).
    pub offset_0x4: Ptr,
    /// +0x94 "Menu" (id 4).
    pub offset_0x8: Ptr,
    /// +0x98 "Adventure" (id 0).
    pub offset_0xc: Ptr,
    /// +0x9c "Time Trial" (id 1).
    pub offset_0x10: Ptr,
    /// +0xa0 "Challenge" (id 2).
    pub offset_0x14: Ptr,
    /// +0xa4 "Personal" (id 3).
    pub offset_0x18: Ptr,
    /// +0xa8 the tab shown (0..3; remembered in `DAT_005e15fc`).
    pub offset_0x1c: i32,
}

impl G {
    pub fn high_score_screen(&mut self, p: Ptr) -> &mut HighScoreScreen_data {
        match &mut self.widget(p).ext {
            WExt::HighScoreScreen(d) => d,
            e => panic!("{p} is not a HighScoreScreen: {e:?}"),
        }
    }
}

/// port: 005241d0 Sexy::HighScoreScreen::HighScoreScreen
/// `HighScoreScreen(WinFishApp*)`: bubbles rising across the screen; the remembered tab;
/// "Menu" at the top right; the four tabs in a row along the bottom.
pub fn HighScoreScreen(g: &mut G, param_1: Ptr) -> Ptr {
    use crate::game::board_parts::{FUN_00500120, FUN_00500150, FUN_00500180, FUN_005001a0, FUN_00512080};
    use crate::game::game_selector::FUN_00506820;
    use crate::sexy::widget::FUN_0046e880;
    let (wc, w) = crate::sexy::widget::Widget();
    let d = HighScoreScreen_data { field_0x0: param_1, ..Default::default() };
    let this = g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__HighScoreScreen_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::HighScoreScreen(Box::new(d)) }),
    });
    let b = crate::game::board_parts::BubbleMgr(g);
    g.high_score_screen(this).offset_0x4 = b;
    FUN_00500120(g, b, &Rect::new(0, 0x3c, 0x280, 0x1a4));
    FUN_00500150(g, b, &Rect::new(0, 0x50, 0x280, 0x15e));
    FUN_00500180(g, b, 0x14, 3);
    FUN_005001a0(g, b, 3, 3);
    FUN_00512080(g, b);
    g.high_score_screen(this).offset_0x1c = g.globals.DAT_005e15fc;
    let img = g.res.DAT_005e8c1c;
    let menu = FUN_00506820(g, 4, this, b"Menu", img);
    g.high_score_screen(this).offset_0x8 = menu;
    let h = g.wc(menu).offset_0x38;
    vcall!(g, menu, w.vfunction41, 0x20d, 4, 0x50, h);
    let adv = FUN_00506820(g, 0, this, b"Adventure", img);
    let tt = FUN_00506820(g, 1, this, b"Time Trial", img);
    let ch = FUN_00506820(g, 2, this, b"Challenge", img);
    let pers = FUN_00506820(g, 3, this, b"Personal", img);
    let d = g.high_score_screen(this);
    d.offset_0xc = adv;
    d.offset_0x10 = tt;
    d.offset_0x14 = ch;
    d.offset_0x18 = pers;
    let h = g.wc(adv).offset_0x38;
    vcall!(g, adv, w.vfunction41, 0x14, 0x1d6 - h, 0x78, h);
    FUN_0046e880(g, tt, 0x8403, adv, 0x28, 0, 0, 0);
    FUN_0046e880(g, ch, 0x8403, tt, 0x28, 0, 0, 0);
    FUN_0046e880(g, pers, 0x8403, ch, 0x28, 0, 0, 0);
    this
}

/// port: 00518320 Sexy::HighScoreScreen::~HighScoreScreen
/// Remembers the tab, then deletes the bubbles and buttons.
pub fn dtor_HighScoreScreen(g: &mut G, this: Ptr) {
    let d = g.high_score_screen(this).clone();
    g.globals.DAT_005e15fc = d.offset_0x1c;
    if d.offset_0x4 != NULL {
        crate::game::board_parts::deleting_destructor__00505420(g, d.offset_0x4, 1);
    }
    for p in [d.offset_0x8, d.offset_0xc, d.offset_0x10, d.offset_0x14, d.offset_0x18] {
        if p != NULL {
            vcall!(g, p, w.vfunction1, 1);
        }
    }
    crate::sexy::widget::dtor_Widget(g, this);
}

/// port: 0051aeb0 Sexy::HighScoreScreen::deleting_destructor_for_Widget
pub fn deleting_destructor_for_Widget(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_HighScoreScreen(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

fn buttons(d: &HighScoreScreen_data) -> [Ptr; 5] {
    [d.offset_0x8, d.offset_0xc, d.offset_0x10, d.offset_0x14, d.offset_0x18]
}

/// port: 00518400 Sexy::HighScoreScreen::vfunction21_for_Widget
/// `AddedToManager(WidgetManager*)`: adds the buttons.
pub fn vfunction21_for_Widget(g: &mut G, this: Ptr, param_1: Ptr) {
    crate::sexy::widget_container::vfunction21(g, this, param_1);
    let d = g.high_score_screen(this).clone();
    for p in buttons(&d) {
        vcall!(g, param_1, w.vfunction4, p);
    }
}

/// port: 00518470 Sexy::HighScoreScreen::vfunction22_for_Widget
/// `RemovedFromManager(WidgetManager*)`.
pub fn vfunction22_for_Widget(g: &mut G, this: Ptr, param_1: Ptr) {
    crate::sexy::widget_container::vfunction22(g, this, param_1);
    let d = g.high_score_screen(this).clone();
    for p in buttons(&d) {
        vcall!(g, param_1, w.vfunction5, p);
    }
}

/// port: 00518550 Sexy::HighScoreScreen::vfunction23_for_Widget
/// `Update()`: the bubbles, and redraw.
pub fn vfunction23_for_Widget(g: &mut G, this: Ptr) {
    crate::sexy::widget_container::vfunction23(g, this);
    let b = g.high_score_screen(this).offset_0x4;
    crate::game::board_parts::FUN_005110e0(g, b);
    vcall!(g, this, w.vfunction18);
}

/// port: 00518570 Sexy::HighScoreScreen::vfunction3_for_ButtonListener
/// `ButtonDepress(int id)`: "Menu" (4) back to the main menu; a tab (0..3) shows it.
pub fn vfunction3_for_ButtonListener(g: &mut G, this: Ptr, param_1: i32) {
    if param_1 == 4 {
        let app = g.high_score_screen(this).field_0x0;
        FUN_00552280(g, app);
        return;
    }
    if (param_1 as u32) < 4 {
        g.high_score_screen(this).offset_0x1c = param_1;
    }
}

/// The bob of a line at `y` with 3D acceleration: `3 * sin(3t° + 2y°)` (degrees through
/// 3.14159 / 180, rounded to floats as the original does); pushed on the Graphics.
fn push_bob(g: &mut G, this: Ptr, gfx: &mut Graphics, y: i32) -> f32 {
    let pi = f64::from_bits(0x400921fa_00000000);
    let t = g.wc(this).offset_0x24;
    let a = ((t as f64 * 3.0 * pi / 180.0) + ((y + y) as f64 * pi / 180.0)) as f32;
    let d = ((a as f64).sin() as f32 as f64 * 3.0) as f32;
    gfx.s.mTransY += d;
    d
}

/// The same bob at a fixed phase: `3 * sin(3t° + k)`.
fn push_bob_k(g: &mut G, this: Ptr, gfx: &mut Graphics, k: f64) -> f32 {
    let pi = f64::from_bits(0x400921fa_00000000);
    let t = g.wc(this).offset_0x24;
    let a = ((t as f64 * 3.0 * pi / 180.0) + k) as f32;
    let d = ((a as f64).sin() as f32 as f64 * 3.0) as f32;
    gfx.s.mTransY += d;
    d
}

/// port: 00513730 FUN_00513730
/// The high-score table of adventure level `param_1`-`param_2` (an empty one, kept in a
/// function static, for other levels).
pub fn FUN_00513730(g: &mut G, this: Ptr, param_1: i32, param_2: i32) -> Vec<HighScore> {
    let i = crate::game::profile::FUN_00500dd0(param_1, param_2);
    if i < 0 {
        return Vec::new();
    }
    g.high_score_mgr(this).field_0x60[i as usize].clone()
}

/// port: 005137e0 FUN_005137e0
/// The best entry's name of a level's table ("" when it has none).
pub fn FUN_005137e0(g: &mut G, this: Ptr, param_1: i32, param_2: i32) -> Vec<u8> {
    let t = FUN_00513730(g, this, param_1, param_2);
    t.first().map(|e| e.field_0x0.clone()).unwrap_or_default()
}

/// port: 00504c90 FUN_00504c90
/// A time in seconds as "m:ss", or "h:mm:ss" from an hour (hours padded to two places).
pub fn FUN_00504c90(param_2: i32) -> Vec<u8> {
    let m = param_2 / 0x3c;
    if m / 0x3c < 1 {
        return format!("{}:{:02}", m, param_2 % 0x3c).into_bytes();
    }
    format!("{:2}:{:02}:{:02}", m / 0x3c, m % 0x3c, param_2 % 0x3c).into_bytes()
}

/// port: 00524510 FUN_00524510
/// The Adventure / Time Trial / Challenge tables: tanks 1-4 in two columns ("Tank n" and up
/// to five rows: adventure shows each level's best player beside "n-m"; Time Trial money,
/// the others times), then for a player who finished the adventure the tank-5 table of
/// surviving pets (up to three, "n / 18").
pub fn FUN_00524510(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    let d = g.high_score_screen(this).clone();
    let app = d.field_0x0;
    let tt = d.offset_0x1c == 1;
    let adv = d.offset_0x1c == 0;
    let mut low_y = 0x118;
    let mut final_tank = false;
    let profile = g.wfa(app).offset_0x18c;
    if adv && g.profile(profile).field_0x59 {
        final_tank = true;
        low_y = 0xf0;
    }
    let is3d = crate::sexy::sexy_app_base::dtor_MemoryImage__00489a20(g, app);
    let mgr = g.wfa(app).offset_0x180;
    for tank in 1..5i32 {
        // `mgr + 0x34 + (tank - 1) * 0xc`, or 0x30 before it for Time Trial.
        let list = if tt { g.high_score_mgr(mgr).field_0x0[(tank - 1) as usize].clone() } else { g.high_score_mgr(mgr).field_0x30[(tank - 1) as usize].clone() };
        let x = if tank & 1 != 0 { 0x3c } else { 0x17c };
        let mut y = if tank < 3 { 100 } else { low_y };
        let f = g.res.DAT_005e8a5c;
        FUN_00455880(gfx, f);
        FUN_00455890(gfx, FUN_00433360(0xff, 0xff, 100));
        let title = format!("Tank {tank}").into_bytes();
        let fnt = FUN_00455870(gfx);
        let tw = font::string_width(g, fnt, &title);
        let bob = if is3d { push_bob(g, this, gfx, y) } else { 0.0 };
        FUN_00455cf0(gfx, g, &title, (200 - tw) / 2 + x, y);
        if is3d {
            gfx.s.mTransY -= bob;
        }
        let fnt = FUN_00455870(gfx);
        y += font::get_height(g, fnt);
        let f = g.res.DAT_005e8cf0;
        FUN_00455880(gfx, f);
        FUN_00455890(gfx, FUN_00433360(0xff, 0xff, 0xff));
        for (n, e) in list.iter().enumerate() {
            let row = n as i32 + 1;
            if 5 <= row - 1 {
                break;
            }
            let bob = if is3d { push_bob(g, this, gfx, y) } else { 0.0 };
            crate::game::help_screen::FUN_005184e0(g, this, gfx, y);
            let name = if adv {
                let nm = FUN_005137e0(g, mgr, tank, row);
                let lv = format!("{tank}-{row}").into_bytes();
                FUN_00455cf0(gfx, g, &lv, x - 0x1e, y);
                nm
            } else {
                e.field_0x0.clone()
            };
            FUN_00455cf0(gfx, g, &name, x, y);
            let score = if tt { format!("{}", e.offset_0x1c).into_bytes() } else { FUN_00504c90(e.offset_0x1c) };
            let fnt = FUN_00455870(gfx);
            let sw = font::string_width(g, fnt, &score);
            FUN_00455cf0(gfx, g, &score, x + (200 - sw), y);
            y += 0xf;
            if is3d {
                gfx.s.mTransY -= bob;
            }
        }
    }
    if !final_tank {
        return;
    }
    let bob = if is3d { push_bob_k(g, this, gfx, 12.740893) } else { 0.0 };
    let f = g.res.DAT_005e8a5c;
    FUN_00455880(gfx, f);
    FUN_00455890(gfx, FUN_00433360(0xff, 0xff, 100));
    let title: &[u8] = b"Tank 5 (Surviving Pets)";
    let fnt = FUN_00455870(gfx);
    let tw = font::string_width(g, fnt, title);
    FUN_00455cf0(gfx, g, title, 0x140 - tw / 2, 0x16d);
    if is3d {
        gfx.s.mTransY -= bob;
    }
    let fnt = FUN_00455870(gfx);
    let mut y = font::get_height(g, fnt) + 0x16d;
    let f = g.res.DAT_005e8cf0;
    FUN_00455880(gfx, f);
    FUN_00455890(gfx, FUN_00433360(0xff, 0xff, 0xff));
    let list = FUN_00513730(g, mgr, 5, 1);
    for e in list.iter().take(3) {
        let bob = if is3d { push_bob(g, this, gfx, y) } else { 0.0 };
        crate::game::help_screen::FUN_005184e0(g, this, gfx, y);
        FUN_00455cf0(gfx, g, &e.field_0x0, 0xdc, y);
        let s = format!("{} / 18", e.offset_0x1c).into_bytes();
        let fnt = FUN_00455870(gfx);
        let sw = font::string_width(g, fnt, &s);
        FUN_00455cf0(gfx, g, &s, 0x1a4 - sw, y);
        y += 0xf;
        if is3d {
            gfx.s.mTransY -= bob;
        }
    }
}

/// port: 00524c20 FUN_00524c20
/// Personal Records: the player's best time for every adventure level (tanks 1-4 in four
/// columns), the final boss's pets saved, and per tank the best Time Trial money and
/// Challenge time ("-----" where there is none).
pub fn FUN_00524c20(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    let app = g.high_score_screen(this).field_0x0;
    let profile = g.wfa(app).offset_0x18c;
    let is3d = crate::sexy::sexy_app_base::dtor_MemoryImage__00489a20(g, app);
    let f = g.res.DAT_005e8a5c;
    FUN_00455880(gfx, f);
    FUN_00455890(gfx, FUN_00433360(0xff, 0xff, 100));
    let bob = if is3d { push_bob_k(g, this, gfx, 3.8397212) } else { 0.0 };
    let fnt = FUN_00455870(gfx);
    let w = font::string_width(g, fnt, b"Adventure");
    FUN_00455cf0(gfx, g, b"Adventure", 0x140 - w / 2, 0x6e);
    let mut bob2 = 0.0;
    if is3d {
        gfx.s.mTransY -= bob;
        bob2 = push_bob_k(g, this, gfx, 10.122902);
    }
    let fnt = FUN_00455870(gfx);
    let w = font::string_width(g, fnt, b"Time Trial");
    FUN_00455cf0(gfx, g, b"Time Trial", 0xad - w / 2, 0x122);
    let fnt = FUN_00455870(gfx);
    let w = font::string_width(g, fnt, b"Challenge");
    FUN_00455cf0(gfx, g, b"Challenge", 0x1d2 - w / 2, 0x122);
    if is3d {
        gfx.s.mTransY -= bob2;
    }
    let f = g.res.DAT_005e8cf0;
    FUN_00455880(gfx, f);
    FUN_00455890(gfx, FUN_00433320(0xffffff));
    let best = |g: &mut G, tank: i32, level: i32| -> i32 { crate::game::profile::FUN_00501300(g.profile(profile), tank, level) };
    let mut x = 0x14;
    let mut tank = 0;
    let mut last_y = 0;
    loop {
        tank += 1;
        let mut y = 0x8c;
        for level in 1..=5 {
            last_y = y;
            let bob = if is3d { push_bob(g, this, gfx, y) } else { 0.0 };
            crate::game::help_screen::FUN_005184e0(g, this, gfx, y);
            FUN_00455890(gfx, FUN_00433320(0xffff88));
            let lv = format!("{tank}-{level}").into_bytes();
            FUN_00455cf0(gfx, g, &lv, x, y);
            let t = best(g, tank, level);
            let s = if t < 0 { b"-----".to_vec() } else { FUN_00504c90(t) };
            crate::game::help_screen::FUN_005184e0(g, this, gfx, y);
            let fnt = FUN_00455870(gfx);
            let sw = font::string_width(g, fnt, &s);
            FUN_00455cf0(gfx, g, &s, (x + 100) - sw, y);
            y += 0xf;
            if is3d {
                gfx.s.mTransY -= bob;
            }
        }
        x += 0xa6;
        if 0x2ab < x {
            break;
        }
    }
    let pets = best(g, 5, 1);
    if -1 < pets {
        let y = last_y + 0x14;
        let bob = if is3d { push_bob(g, this, gfx, y) } else { 0.0 };
        crate::game::help_screen::FUN_005184e0(g, this, gfx, y);
        FUN_00455890(gfx, FUN_00433320(0xffff88));
        FUN_00455cf0(gfx, g, b"Final Boss", 0xdc, y);
        crate::game::help_screen::FUN_005184e0(g, this, gfx, y);
        let s = format!("{pets}/18 Pets Saved").into_bytes();
        FUN_00455cf0(gfx, g, &s, 0x140, y);
        if is3d {
            gfx.s.mTransY -= bob;
        }
    }
    for col in 0..2 {
        let cx = if col != 0 { 0x178 } else { 0x53 };
        let mut y = 0x13b;
        for tank in 1..=4 {
            let bob = if is3d { push_bob(g, this, gfx, y) } else { 0.0 };
            crate::game::help_screen::FUN_005184e0(g, this, gfx, y);
            FUN_00455890(gfx, FUN_00433320(0xffff88));
            let s = format!("Tank {tank}").into_bytes();
            FUN_00455cf0(gfx, g, &s, cx, y);
            let p = g.profile(profile).clone();
            let v = if col == 0 { crate::game::profile::FUN_00501330(&p, tank) } else { crate::game::profile::FUN_00501350(&p, tank) };
            let text = if v < 0 {
                b"-----".to_vec()
            } else if col == 0 {
                crate::sexy::app_host::FUN_004107c0(g, v)
            } else {
                FUN_00504c90(v)
            };
            crate::game::help_screen::FUN_005184e0(g, this, gfx, y);
            let fnt = FUN_00455870(gfx);
            let sw = font::string_width(g, fnt, &text);
            FUN_00455cf0(gfx, g, &text, (cx + 0xaa) - sw, y);
            y += 0xf;
            if is3d {
                gfx.s.mTransY -= bob;
            }
        }
    }
}

/// port: 00525440 Sexy::HighScoreScreen::vfunction27_for_Widget
/// `Draw(Graphics*)`: the gradient (4-pixel bands from light to dark blue), bubbles, the
/// waterline, the tab's table and title, the top bar with the Menu frame, "Hall of Fame".
pub fn vfunction27_for_Widget(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    let d = g.high_score_screen(this).clone();
    let app = d.field_0x0;
    let w = g.wc(this).offset_0x34;
    let mut l = 0x6b;
    let mut y = 0;
    while y < 0x1e0 {
        let c = crate::sexy::sexy_app_base::FUN_00489530(g, app, 0x97, 0x8a, l);
        FUN_00455890(gfx, FUN_00433320(c));
        FUN_00455920(gfx, 0, y, w, 4);
        y += 4;
        l -= 1;
        if l < 0x32 {
            l = 0x32;
        }
    }
    crate::game::board_parts::FUN_00504b60(g, d.offset_0x4, gfx);
    let t = g.wc(this).offset_0x24;
    crate::game::help_screen::FUN_00500d40(g, gfx, 0x3c, t);
    let title: &[u8] = match d.offset_0x1c {
        0 => b"Adventure",
        1 => b"Time Trial",
        2 => b"Challenge",
        3 => b"Personal Records",
        _ => b"",
    };
    match d.offset_0x1c {
        0..=2 => FUN_00524510(g, this, gfx),
        3 => FUN_00524c20(g, this, gfx),
        _ => {}
    }
    let f = g.res.DAT_005e8b04;
    FUN_00455880(gfx, f);
    FUN_00455890(gfx, FUN_00433320(0xffffff));
    let fnt = FUN_00455870(gfx);
    let tw = font::string_width(g, fnt, title);
    let o = g.res.DAT_005e8d1c;
    crate::game::help_screen::FUN_00500bf0(g, gfx, title, 0x140 - tw / 2, 0x3c, o, 0);
    let bar = g.res.DAT_005e8d3c;
    let h = g.image(bar).offset_0x24;
    FUN_004563f0(gfx, g, &Rect::new(0, 0, 0x280, h), bar);
    let frame = g.res.DAT_005e8e6c;
    let h = g.image(frame).offset_0x24;
    let m = g.wc(d.offset_0x8).clone();
    FUN_004563f0(gfx, g, &Rect::new(m.offset_0x2c - 1, m.offset_0x30 - 1, m.offset_0x34 + 2, h), frame);
    let f = g.res.DAT_005e8e40;
    FUN_00455880(gfx, f);
    FUN_00455890(gfx, CRect(0xff, 200, 0, 0xff));
    vcall!(g, this, w.vfunction63, gfx, 0x19, b"Hall of Fame");
}

/// port: 005522d0 FUN_005522d0
/// `ShowHallOfFame()`: closes the game selector and any Hall of Fame, then opens it
/// full-screen.
pub fn FUN_005522d0(g: &mut G, this: Ptr) {
    crate::game::win_fish_app::FUN_0054b360(g, this);
    FUN_00552280(g, this);
    let s = HighScoreScreen(g, this);
    g.wfa(this).offset_0x104 = s;
    let (w, h) = (g.sab(this).field_0xb8, g.sab(this).field_0xbc);
    vcall!(g, s, w.vfunction41, 0, 0, w, h);
    let wm = g.sab(this).offset_0x318;
    vcall!(g, wm, w.vfunction4, s);
}

/// port: 00552280 FUN_00552280
/// `RemoveHallOfFame()` (+0x830), back to the game selector.
pub fn FUN_00552280(g: &mut G, this: Ptr) {
    let s = g.wfa(this).offset_0x104;
    if s != NULL {
        let wm = g.sab(this).offset_0x318;
        vcall!(g, wm, w.vfunction5, s);
        crate::sexy::sexy_app_base::vfunction35(g, this, s);
        g.wfa(this).offset_0x104 = NULL;
        crate::game::win_fish_app::FUN_00552100(g, this);
    }
}
