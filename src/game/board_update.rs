//! `Board::Update()` (`vfunction23_for_Widget`) and the small Board methods only it uses:
//! one game tick of the tank (timers, the alien schedule, holding the mouse to feed or
//! shoot, shop prices in challenge mode, level clocks, game over).

use crate::game::board_level::{vec_index, FUN_005409b0, FUN_00537b80, FUN_00537c20, FUN_00538a10, FUN_00538a50, FUN_0053a360, FUN_0053aa40, FUN_0053e950};
use crate::sexy::prelude::*;
use crate::sexy::types::CRect;

fn app_rand(g: &mut G, app: Ptr) -> u32 {
    let r = g.wfa(app).offset_0x84;
    crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(r))
}

/// The alien-difficulty period of challenge mode for the tank (the original leaves it
/// uninitialized for other tanks, which challenge mode never uses).
fn challenge_period(tank: i32) -> i32 {
    match tank {
        1 => 4,
        2 | 3 => 5,
        4 => 6,
        _ => 0,
    }
}

/// The three alien warning colors written into the message line.
fn alert_colors(g: &mut G, msg: Ptr) {
    let m = g.message_widget(msg);
    m.offset_0x20 = CRect(0xff, 0x32, 0x32, 0xff);
    m.offset_0x30 = CRect(100, 0x14, 0x14, 0xff);
}

/// Four random spots in the tank for the next aliens (+0x2cc..+0x2d8).
fn roll_alien_spots(g: &mut G, this: Ptr) {
    let app = g.board(this).field_0x0;
    let a = (app_rand(g, app) % 0x1c2) as i32 + 0x14;
    g.board(this).field_0x240 = a;
    let b = (app_rand(g, app) % 0xc3) as i32 + 0x69;
    g.board(this).field_0x244 = b;
    let c = (app_rand(g, app) % 0x1c2) as i32 + 0x14;
    g.board(this).field_0x248 = c;
    let d = (app_rand(g, app) % 0xc3) as i32 + 0x69;
    g.board(this).field_0x24c = d;
}

/// port: 00547610 Sexy::Board::vfunction23_for_Widget
/// `Update()`.
pub fn vfunction23_for_Widget(g: &mut G, this: Ptr) {
    crate::sexy::widget_container::vfunction23(g, this);
    let app = g.board(this).field_0x0;
    let mgr_app = g.sab(app).offset_0x318;
    let (mx, my) = { let m = crate::sexy::widget_manager::wm(g, mgr_app); (m.offset_0x8c, m.offset_0x90) };
    if g.board(this).field_0x23c != 0 {
        g.board(this).field_0x23c -= 1;
    }
    let list = g.board(this).offset_0x4;
    g.globals.DAT_005e89cd = !g.board(this).field_0x8 && 0 < g.timed_messages(list).field_0x4.len() as i32;
    if g.board(this).field_0x8 {
        return;
    }
    crate::game::timed_messages::FUN_00515ec0(g, list);
    let profile = g.wfa(app).offset_0x18c;
    if (g.profile(profile).field_0x84 >> 3) & 1 != 0 {
        let sf = g.board(this).offset_0xb4;
        crate::game::board_parts::FUN_00511680(g, sf);
    }
    let bm = g.board(this).offset_0x90;
    crate::game::board_parts::FUN_005110e0(g, bm);
    if crate::game::win_fish_app::thunk_FUN_00479fc0(g, app) && g.wc(this).offset_0x24 % 0x2a30 == 0 && g.wfa(app).offset_0x10e {
        let mut v: Vec<i32> = Vec::new();
        for i in 0..6 {
            let profile = g.wfa(app).offset_0x18c;
            if g.profile(profile).field_0x72[i] && i as i32 + 1 != g.board(this).field_0x35c {
                v.push(i as i32);
            }
        }
        if !v.is_empty() {
            let r = crate::sexy::sexy_app_base::FUN_0040f9e0(g) % v.len() as u32;
            if g.board(this).field_0x360 == -1 {
                g.board(this).field_0x360 = g.board(this).field_0x35c;
            }
            FUN_00538a10(g, this, v[r as usize] + 1);
            crate::game::board_update::FUN_00539a50(g, this);
        }
    }
    g.wfa(app).offset_0x168 += 1;
    if g.wfa(app).offset_0x150 == 4 && g.board(this).field_0x254 < g.board(this).field_0x250 && 0x2a30 < g.board(this).ext_0x444 {
        let s = g.res.DAT_005e8c40;
        crate::sexy::sexy_app_base::vfunction55(g, app, s);
        for i in 0..0xc {
            FUN_005409b0(g, this, i, true);
        }
    }
    if g.board(this).field_0x21b {
        g.board(this).field_0x21b = false;
        if g.wfa(app).offset_0x150 == 5 {
            crate::game::board_update::FUN_0053a4f0(g, this);
        }
        let v = g.board(this).field_0x21a;
        FUN_00539ad0(g, this, v);
    }
    if g.board(this).field_0x228 != 0 {
        g.board(this).field_0x228 -= 1;
    }
    if g.board(this).ext_0x4fd {
        let t = g.board(this).ext_0x448 % 500;
        let v = if t < 0x3c { t % 0x14 } else { 0 };
        g.board(this).ext_0x44c = v;
        if v == 0xd {
            if t < 0x14 {
                crate::game::board::FUN_00538230(g, this, 0x110, 3, 1.0);
            }
            let (px, py) = FUN_00538b60(g, this);
            let mut n = crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32 % 5 + 5;
            while 0 < n {
                let dx = crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32 % 0x29 - 0x14;
                let dy = crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32 % 10;
                FUN_00538ac0(g, this, px + dx + 0x19, py - dy - 0xf);
                n -= 1;
            }
        }
        g.board(this).ext_0x448 += 1;
    }
    if g.wfa(app).offset_0x154 {
        let n = g.board(this).offset_0xc[vec_index(0xa0)].len() as i32;
        if g.globals.DAT_005e89dc != 2 && 5 <= n {
            g.globals.DAT_005e89dc = 2;
        }
        if g.globals.DAT_005df58c < 9 && g.globals.DAT_005df58c < n {
            g.globals.DAT_005df58c = n;
        }
    }
    if g.wfa(app).offset_0x150 == 1 {
        let t = (crate::game::board::FUN_00537b60(g, this) - g.board(this).field_0x32c) / 1000;
        g.board(this).field_0x324 = t;
        if g.board(this).field_0x330 - t < 0 {
            g.board(this).ext_0x4ee = false;
            FUN_0053c2d0(g, this);
            let (hs, tank, money) = (g.wfa(app).offset_0x180, g.board(this).field_0x33c, g.board(this).field_0x364);
            let profile = g.wfa(app).offset_0x18c;
            crate::game::high_score::FUN_00514470(g, hs, tank, profile, money);
            crate::game::win_fish_app::FUN_0054eb80(g, app);
            return;
        }
    }
    if g.wfa(app).offset_0x150 == 4 {
        let t = (crate::game::board::FUN_00537b60(g, this) - g.board(this).field_0x32c) / 1000;
        g.board(this).field_0x324 = t;
    }
    if g.board(this).field_0x219 {
        if !g.board(this).field_0x218 {
            if 0xa0 <= g.board(this).ext_0x444 - g.board(this).field_0x21c {
                crate::game::board_level::FUN_00537f40(g, this);
            }
        } else {
            let t = (crate::game::board::FUN_00537b60(g, this) - g.board(this).field_0x32c) / 1000;
            g.board(this).field_0x324 = t;
            if g.board(this).field_0x330 - t < 0 {
                if g.board(this).offset_0xc[vec_index(0xa8)].is_empty() {
                    let old = g.board(this).field_0x36c;
                    g.board(this).field_0x36c = old + 1;
                    if 100 < old {
                        let profile = g.wfa(app).offset_0x18c;
                        crate::game::profile::FUN_00501370(g.profile(profile));
                        let level = g.board(this).field_0x340;
                        if level <= 5 {
                            let (hs, tank, score) = (g.wfa(app).offset_0x180, g.board(this).field_0x33c, g.board(this).field_0x370);
                            let profile = g.wfa(app).offset_0x18c;
                            crate::game::high_score::FUN_00514510(g, hs, tank, level, profile, score);
                        }
                        crate::game::bonus_screen::FUN_0054bf60(g, app);
                        return;
                    }
                }
            } else if g.board(this).ext_0x444 % 10 == 0 {
                crate::game::board_level::FUN_005460f0(g, this);
            }
        }
    }
    if g.wfa(app).offset_0x150 == 4 && (g.board(this).field_0x318[0] || g.board(this).field_0x318[1]) {
        let period = match g.board(this).field_0x33c {
            1 | 2 => 0x96,
            3 => 0xaf,
            4 => 0xc8,
            _ => 0,
        };
        let c = if g.board(this).ext_0x454 <= 0 && g.board(this).ext_0x43c <= 5 {
            let r = g.board(this).ext_0x444 % period;
            if r == 0 {
                for i in 0..0xc {
                    let b = FUN_00538090(g, this, i);
                    if b != NULL {
                        let d = g.board(this);
                        d.field_0x288[i] += d.field_0x2e8[i];
                        if 99999 < d.field_0x288[i] {
                            d.field_0x288[i] = 99999;
                        }
                        let price = d.field_0x288[i];
                        FUN_00534380(g, b, price);
                    }
                }
                CRect(0xfa, 0x9b, 0x6e, 0xff)
            } else if period - 10 < r {
                CRect(0xfa, 0x6e, 0x37, 0xff)
            } else {
                CRect(0xfa, 0x9b, 0x6e, 0xff)
            }
        } else {
            let mut v = g.board(this).ext_0x454 % 0xe;
            if 7 <= v {
                v = 0xe - v;
            }
            let k = v * 0xe + 0x6e;
            CRect(k, 0xfa, k, 0xff)
        };
        FUN_00538700(g, this, c);
    }
    if g.wfa(app).offset_0x150 == 5 {
        let profile = g.wfa(app).offset_0x18c;
        if g.profile(profile).field_0x48 == 0 && g.board(this).ext_0x444 == 0x96 {
            FUN_0053e950(g, this, b"Here you'll be able to build your own custom fish tank!", false, 0x2c);
        }
    }
    if g.board(this).ext_0x4ec {
        let mgr = g.wc(this).offset_0xc;
        if !crate::sexy::widget_manager::FUN_0046cce0(g, mgr) {
            g.board(this).ext_0x4ec = false;
        } else if g.board(this).ext_0x444 % (0x10 - g.globals.DAT_005df58c) == 0
            && g.board(this).field_0x334 + 200 < crate::game::board::FUN_00537b60(g, this)
            && ((mx - 0x1f) as u32) <= 0x22b
            && ((my - 0x3d) as u32) <= 0x152
        {
            let cost = g.board(this).ext_0x4ac;
            if FUN_00540b30(g, this, cost, false) {
                crate::game::board_level::FUN_00543280(g, this, mx - 10, my - 10, 0, false, 0x14, -1);
            }
        }
    }
    if g.board(this).ext_0x4ed {
        let mgr = g.wc(this).offset_0xc;
        if !crate::sexy::widget_manager::FUN_0046cce0(g, mgr) {
            g.board(this).ext_0x4ed = false;
        } else {
            let w = g.board(this).field_0x358;
            if 0xb < w
                && g.board(this).ext_0x444 % (0xb - w / 2) == 0
                && g.board(this).field_0x338 + 100 < crate::game::board::FUN_00537b60(g, this)
                && 0x28 < my
            {
                for o in g.board(this).offset_0xc[vec_index(0xf4)].clone() {
                    if crate::game::bilaterus::FUN_004ffae0(g, o, mx, my) {
                        break;
                    }
                }
                for o in g.board(this).offset_0xc[vec_index(0xdc)].clone() {
                    if crate::game::missle::FUN_004d82a0(g, o, mx, my) {
                        break;
                    }
                }
                for o in g.board(this).offset_0xc[vec_index(0xb8)].clone() {
                    if crate::game::alien::FUN_004fa6f0(g, o, mx, my) {
                        break;
                    }
                }
                crate::game::board_level::FUN_00543640(g, this, mx - 0x28, my - 0x28);
                crate::game::board_update::FUN_00538ae0(g, this);
            }
        }
    }
    {
        let b = g.board(this);
        if 0 < b.ext_0x450 {
            b.ext_0x450 -= 1;
        }
        if 0 < b.ext_0x454 {
            b.ext_0x454 -= 1;
        }
        if 0 < b.ext_0x458 {
            b.ext_0x458 -= 1;
        }
    }
    stars_step(g, this);
    if g.wfa(app).offset_0x150 == 3 && g.board(this).field_0x254 < g.board(this).field_0x250 && g.board(this).field_0x33c != 5 {
        for i in 0..0xc {
            FUN_005409b0(g, this, i, true);
        }
        crate::game::board_level::FUN_005380c0(g, this, 0xb, 0);
    }
    FUN_0053aa40(g, this);
    if !g.board(this).field_0x8 {
        let b = g.board(this);
        b.ext_0x444 = b.ext_0x444.wrapping_add(1);
        if b.ext_0x444 < 0 {
            b.ext_0x444 = 0x40000000;
        }
        if b.ext_0x444 % 0x800 == 0 {
            FUN_0053a450(g, this);
        }
        if !g.board(this).ext_0x4fd && app_rand(g, app) % 1000 == 0 {
            let mut n = (app_rand(g, app) & 3) as i32 + 2;
            loop {
                n -= 1;
                FUN_00538a50(g, this);
                if n < 0 {
                    break;
                }
            }
            crate::game::board::FUN_00538230(g, this, 0x110, 3, 1.0);
        }
    }
    if g.board(this).field_0x238 != 0 {
        g.board(this).field_0x238 -= 1;
    }
    if !g.board(this).field_0x8 && !(g.wfa(app).offset_0x150 == 5 && !g.board(this).ext_0x4fe) {
        alien_schedule(g, this);
        game_over_checks(g, this);
        tutorial_messages(g, this);
    }
    {
        let b = g.board(this);
        if b.field_0x340 == 1 && b.field_0x33c == 1 && b.ext_0x4e6 {
            if b.field_0x288[11] < b.field_0x364 {
                b.ext_0x4e8 += 1;
            }
            if 0x5dc < b.ext_0x4e8 {
                b.ext_0x4b0[0x28] = 1;
            }
        }
    }
    {
        let b = g.board(this);
        if b.field_0x33c == 5 && b.field_0x84 == NULL && b.ext_0x4f4 && !b.field_0x219 && !b.field_0x318[11] {
            FUN_005409b0(g, this, 0xb, true);
        }
    }
    vcall!(g, this, w.vfunction18);
}

/// The star drift of the space backdrop: three x positions wrapping at 0 / 640. The
/// original's loop runs over ten consecutive words (+0x3d8..+0x3fc) as floats; the seven
/// after the stars are ints (+0x3e4.. backdrop, money, counters), checked here the same way
/// through their bit patterns.
fn stars_step(g: &mut G, this: Ptr) {
    let b = g.board(this);
    b.field_0x34c[0] = (b.field_0x34c[0] as f64 - 1.6) as f32;
    b.field_0x34c[1] = (b.field_0x34c[1] as f64 + 1.8) as f32;
    b.field_0x34c[2] = (b.field_0x34c[2] as f64 + 2.2) as f32;
    let wrap = |v: f32| -> f32 {
        if 640.0 < v {
            0.0
        } else if v < 0.0 {
            640.0
        } else {
            v
        }
    };
    for i in 0..3 {
        b.field_0x34c[i] = wrap(b.field_0x34c[i]);
    }
    let ints: [&mut i32; 7] = [
        &mut b.field_0x358,
        &mut b.field_0x35c,
        &mut b.field_0x360,
        &mut b.field_0x364,
        &mut b.field_0x368,
        &mut b.field_0x36c,
        &mut b.field_0x370,
    ];
    for v in ints {
        let f = f32::from_bits(*v as u32);
        let n = wrap(f);
        if n.to_bits() != f.to_bits() {
            *v = n.to_bits() as i32;
        }
    }
}

/// The alien timer (+0x2c0) and what happens at its marks: warnings, the incoming-alien
/// alert, the spawn and the next schedule.
fn alien_schedule(g: &mut G, this: Ptr) {
    let app = g.board(this).field_0x0;
    {
        let b = g.board(this);
        if !b.offset_0xc[vec_index(0xb8)].is_empty() || !b.offset_0xc[vec_index(0xf4)].is_empty() || !b.offset_0xc[vec_index(0xdc)].is_empty() {
            return;
        }
    }
    if g.board(this).field_0x230 != 0 && g.board(this).field_0x84 == NULL && !g.board(this).ext_0x4f4 {
        g.board(this).field_0x234 -= 1;
        if g.board(this).ext_0x4fe && g.board(this).field_0x234 == 0x21 {
            crate::game::board::FUN_00538230(g, this, 0x143, 3, 1.0);
        }
    }
    if g.board(this).field_0x234 == 0x1e && g.board(this).field_0xb8[21] != 0 && g.board(this).field_0x33c != 5 {
        FUN_00543f40(g, this);
    }
    let mode = g.wfa(app).offset_0x150;
    let msg = g.board(this).offset_0x8c;
    if mode == 4 && g.board(this).field_0x234 == 0xbb7 {
        let period = challenge_period(g.board(this).field_0x33c);
        let n = g.board(this).ext_0x45c;
        if n % period == 0 && n != 0 {
            let m = g.message_widget(msg);
            m.field_0x4 = b"WARNING! ALIEN DIFFICULTY INCREASED!".to_vec();
            m.offset_0x4c = true;
            m.offset_0x48 = 0x112;
            alert_colors(g, msg);
        }
        return;
    }
    let t = g.board(this).field_0x234;
    if t == 0x114 {
        if !g.wfa(app).offset_0x154 && mode != 5 {
            let board = g.wfa(app).offset_0x4;
            if g.board(board).ext_0x4b0[3] == 0 && FUN_00537bb0(g, this, 1, 2) {
                crate::game::win_fish_app::FUN_0054b5b0(
                    g,
                    app,
                    0xe,
                    1,
                    b"DANGER!",
                    b"A vicious alien is about to enter your tank! Defeat it with your laser weapon by clicking on it!",
                    b"Click to Continue",
                    3,
                );
                let board = g.wfa(app).offset_0x4;
                g.board(board).ext_0x4b0[3] = 1;
            } else if g.board(board).ext_0x4b0[4] == 0 && FUN_00537bb0(g, this, 1, 2) {
                crate::game::win_fish_app::FUN_0054b5b0(
                    g,
                    app,
                    0xe,
                    1,
                    b"BATTLE TIP",
                    b"Shoot the alien's head to push it downwards! Shoot its tail to deflect it upwards!",
                    b"Click to Continue",
                    3,
                );
                let board = g.wfa(app).offset_0x4;
                g.board(board).ext_0x4b0[4] = 1;
            } else if g.board(board).ext_0x4b0[0x11] == 0 && FUN_00537bb0(g, this, 2, 3) {
                crate::game::win_fish_app::FUN_0054b5b0(
                    g,
                    app,
                    0xe,
                    1,
                    b"WARNING!",
                    b"A new breed of alien is fast approaching!  Lasers can't hurt this baddie, so find another way to defeat it!",
                    b"Click to Continue",
                    3,
                );
                let board = g.wfa(app).offset_0x4;
                g.board(board).ext_0x4b0[0x11] = 1;
            }
        }
    } else if t == 0x113 {
        let board = g.wfa(app).offset_0x4;
        let kind = g.board(this).field_0x230;
        let text: Vec<u8> = if g.board(board).ext_0x4b0[0x2e] == 0
            && g.board(this).field_0x340 == 4
            && g.board(this).field_0x33c == 4
            && (9..=0xc).contains(&kind)
        {
            b"FOURTEEN ALIEN SIGNATURES DETECTED".to_vec()
        } else {
            match kind {
                4 => b"ALIEN SIGNATURE TYPE-G DETECTED".to_vec(),
                5 => b"ALIEN SIGNATURE TYPE-D DETECTED".to_vec(),
                6 => b"ALIEN SIGNATURE TYPE-U DETECTED".to_vec(),
                7 => b"ALIEN SIGNATURE TYPE-P DETECTED".to_vec(),
                8 => b"ALIEN SIGNATURE TYPE-II DETECTED".to_vec(),
                0x15 => {
                    let profile = g.wfa(app).offset_0x18c;
                    let n = g.profile(profile).field_0x50 + 1;
                    let name = crate::game::game_selector::FUN_00506a50(g, n);
                    let mut s = name.clone();
                    s.extend_from_slice(b" OF DOOM APPROACHING");
                    s
                }
                9..=0xc => b"MULTIPLE ALIEN SIGNATURES DETECTED".to_vec(),
                _ => b"ENEMY APPROACHING".to_vec(),
            }
        };
        let m = g.message_widget(msg);
        m.field_0x4 = text;
        m.offset_0x4c = false;
        m.offset_0x48 = 0x112;
        alert_colors(g, msg);
        crate::game::win_fish_app::FUN_0054c480(g, app, true);
        crate::game::board::FUN_00538230(g, this, 0x106, 3, 1.0);
        if g.wfa(app).offset_0x150 == 5 {
            crate::game::board_update::FUN_00539330(g, this);
        } else {
            roll_alien_spots(g, this);
        }
    } else if t <= 0 {
        {
            let b = g.board(this);
            b.field_0x238 = 0x23;
            b.ext_0x4ed = false;
            b.ext_0x4ec = false;
        }
        let (kind, x, y) = (g.board(this).field_0x230, g.board(this).field_0x240, g.board(this).field_0x244);
        crate::game::board_level::FUN_00545620(g, this, kind, x, y, true);
        if g.wfa(app).offset_0x150 == 4 {
            let period = challenge_period(g.board(this).field_0x33c);
            let mut n = g.board(this).ext_0x45c / period;
            if g.board(this).field_0x230 == 0xc {
                n -= 1;
            }
            while 0 < n {
                roll_alien_spots(g, this);
                let (kind, x, y) = (g.board(this).field_0x230, g.board(this).field_0x240, g.board(this).field_0x244);
                crate::game::board_level::FUN_00545620(g, this, kind, x, y, false);
                n -= 1;
            }
        }
        if g.wfa(app).offset_0x154 {
            FUN_00537c20(g, this);
        } else if g.wfa(app).offset_0x150 == 5 {
            crate::game::virtual_tank::FUN_005394e0(g, this);
        } else {
            next_alien_kind(g, this);
        }
        let b = g.board(this);
        b.ext_0x45c += 1;
        b.field_0x234 = 3000;
    } else if t < 0x113 {
        let board = g.wfa(app).offset_0x4;
        let kind = g.board(this).field_0x230;
        if g.board(board).ext_0x4b0[0x2e] == 0
            && g.board(this).field_0x340 == 4
            && g.board(this).field_0x33c == 4
            && (9..=0xc).contains(&kind)
            && t == 100
        {
            g.message_widget(msg).field_0x4 = b"JUST KIDDING. ONLY TWO!".to_vec();
            let board = g.wfa(app).offset_0x4;
            g.board(board).ext_0x4b0[0x2e] = 1;
        }
        if g.board(this).field_0x234 == 1 {
            crate::game::win_fish_app::FUN_0054b1a0(g, app, 1, 1, false);
        }
    }
}

/// The alien kind for the next wave, by tank and level (some levels alternate at random).
fn next_alien_kind(g: &mut G, this: Ptr) {
    let app = g.board(this).field_0x0;
    let (tank, level) = (g.board(this).field_0x33c, g.board(this).field_0x340);
    if tank == 1 && level == 5 {
        let r = app_rand(g, app);
        g.board(this).field_0x230 = if r & 1 != 0 { 3 } else { 9 };
    } else if tank == 2 && level == 5 {
        if g.board(this).field_0x230 == 9 {
            let r = app_rand(g, app);
            g.board(this).field_0x230 = ((!(r & 0xff)) & 1 | 4) as i32;
        }
        let r = app_rand(g, app);
        if r % 10 == 0 {
            let k = g.board(this).field_0x230;
            g.board(this).field_0x230 = (k != 5) as i32 + 4;
        } else {
            let r = app_rand(g, app);
            if r % 0x14 == 0 {
                g.board(this).field_0x230 = 9;
            }
        }
    } else if tank == 3 && level == 2 {
        let r = app_rand(g, app);
        if r % 10 == 0 {
            let k = g.board(this).field_0x230;
            g.board(this).field_0x230 = (k != 5) as i32 + 4;
        }
    } else if tank == 3 && level == 5 {
        let r = app_rand(g, app);
        g.board(this).field_0x230 = ((!(r & 0xff)) & 1 | 6) as i32;
    } else if tank == 4 && level == 3 {
        let r = app_rand(g, app);
        g.board(this).field_0x230 = if r & 1 != 0 { 4 } else { 10 };
    } else if tank == 4 && level == 4 {
        let r = app_rand(g, app);
        g.board(this).field_0x230 = if r & 1 != 0 { 8 } else { 0xb };
    } else if tank == 4 && level == 5 {
        let r = app_rand(g, app) % 5;
        g.board(this).field_0x230 = if r <= 1 { 10 } else { ((3 < r) as i32 - 1 & 3) + 8 };
        if g.wfa(app).offset_0x150 == 4 && 5 <= g.board(this).ext_0x45c && g.board(this).field_0x230 == 8 {
            g.board(this).field_0x230 = 0xc;
        }
    }
}

/// The last fish died (adventure gives one more on the first level) or, in tank 5, the
/// last pet.
fn game_over_checks(g: &mut G, this: Ptr) {
    let app = g.board(this).field_0x0;
    if g.board(this).field_0x219 || g.wfa(app).offset_0x150 == 5 {
        return;
    }
    if !FUN_005392a0(g, this) && g.board(this).field_0x33c != 5 {
        crate::game::board_update::FUN_0053db80(g, this, true);
        if FUN_00537b80(g, this) {
            crate::game::win_fish_app::FUN_0054b5b0(
                g,
                app,
                0x10,
                1,
                b"YOUR LAST FISH HAS DIED!",
                b"Normally this would end your game, but this time we'll give you another fish to keep playing! Make sure you keep it well fed!",
                b"Click to Continue",
                3,
            );
        } else {
            g.board(this).ext_0x4ee = false;
            FUN_0053c2d0(g, this);
            crate::game::win_fish_app::FUN_0054b5b0(g, app, 0x11, 1, b"GAME OVER", b"Oops!  All of your fish have died!", b"Click to Continue", 3);
        }
    }
    if g.board(this).field_0x33c == 5 && g.board(this).offset_0xc[vec_index(0xb4)].is_empty() && g.board(this).offset_0xc[vec_index(0xb0)].is_empty() {
        crate::game::board_update::FUN_0053db80(g, this, true);
        g.board(this).ext_0x4ee = false;
        let sel = g.board(this).field_0x84;
        if sel != NULL && 1000.0 < { let a = g.alien(sel); a.offset_0xa4 - a.offset_0x9c } {
            let profile = g.wfa(app).offset_0x18c;
            g.profile(profile).field_0x54 += 1;
        }
        crate::game::win_fish_app::FUN_0054b5b0(g, app, 0x11, 1, b"GAME OVER", b"Oops!  All of your pets have died!", b"Click to Continue", 3);
    }
}

/// The one-time hints shown 150 updates into certain levels and modes.
fn tutorial_messages(g: &mut G, this: Ptr) {
    let app = g.board(this).field_0x0;
    let mode = g.wfa(app).offset_0x150;
    if mode == 5 {
        return;
    }
    let at = g.board(this).ext_0x444 == 0x96;
    if mode == 0 && FUN_00537bb0(g, this, 1, 1) && at {
        FUN_0053e950(g, this, b"Here are your first fish! Take good care of them!", false, 1);
    } else if mode == 0 && FUN_00537bb0(g, this, 1, 2) && at {
        FUN_0053e950(g, this, b"On this level you will meet your 1st alien opponent!", false, 0x14);
    } else if mode == 1 && at {
        FUN_0053e950(g, this, b"How much money can you earn before time runs out?", false, 0x35);
    } else if mode == 4 && at {
        FUN_0053e950(g, this, b"Can you fend off the increasingly hungry aliens?", false, 0x33);
    } else if mode == 3 && at {
        FUN_0053e950(g, this, b"Make sure caps-lock is off and GO TO TOWN!!", false, 0x30);
    }
}

/// port: 00539ad0 FUN_00539ad0
/// `SetFishClickable(bool)`: the mouse visibility of the guppies, carnivores and (unless on
/// the first virtual-tank level) two pet kinds.
pub fn FUN_00539ad0(g: &mut G, this: Ptr, param_1: bool) {
    g.board(this).field_0x21a = param_1;
    for o in g.board(this).offset_0xc[vec_index(0xa0)].clone() {
        g.w(o).offset_0x1 = param_1;
    }
    for o in g.board(this).offset_0xc[vec_index(0xc0)].clone() {
        g.w(o).offset_0x1 = param_1;
    }
    if g.board(this).field_0x33c != 5 || g.board(this).field_0x340 != 1 {
        for o in g.board(this).offset_0xc[vec_index(0xb4)].clone() {
            let id = crate::game::fish_type_pet::pet_id(g, o);
            if id == 0x10 && g.fish(o).field_0xd0 <= g.fish(o).field_0xcc {
                g.w(o).offset_0x1 = param_1;
            } else if id == 0x17 {
                g.w(o).offset_0x1 = param_1;
            }
        }
    }
}

/// port: 0053a450 FUN_0053a450
/// Every 2048 updates: lets every object count elapsed days (virtual-tank hunger).
pub fn FUN_0053a450(g: &mut G, this: Ptr) {
    let day = crate::game::win_fish_app::FUN_005005b0(g);
    let now = g.now_time64;
    for o in g.board(this).offset_0x7c.clone() {
        crate::game::game_object::FUN_004d6780(g, o, day, now);
    }
}

/// port: 005392a0 FUN_005392a0
/// Whether anything alive is left to play with (guppies, carnivores, breeders, or one of
/// four pet kinds).
pub fn FUN_005392a0(g: &mut G, this: Ptr) -> bool {
    let b = g.board(this);
    if !b.offset_0xc[vec_index(0xa0)].is_empty() || !b.offset_0xc[vec_index(0xa4)].is_empty() || !b.offset_0xc[vec_index(0xd0)].is_empty() {
        return true;
    }
    FUN_004e3ed0(g, this) || FUN_004e3f00(g, this) || FUN_004e3f30(g, this) || FUN_004e3f60(g, this)
}

/// port: 00537bb0 FUN_00537bb0
/// Adventure (not yet finished) at tank `param_1`, level `param_2`.
pub fn FUN_00537bb0(g: &mut G, this: Ptr, param_1: i32, param_2: i32) -> bool {
    let app = g.board(this).field_0x0;
    let profile = g.wfa(app).offset_0x18c;
    g.board(this).field_0x33c == param_1 && g.board(this).field_0x340 == param_2 && !g.profile(profile).field_0x59
}

/// port: 00540b30 FUN_00540b30
/// `Spend(int amount, bool playSound)`: false (and the "no money" flash, with a hint the
/// first time on level 1-1) when money plus money in flight cannot cover it.
pub fn FUN_00540b30(g: &mut G, this: Ptr, param_1: i32, param_2: bool) -> bool {
    let app = g.board(this).field_0x0;
    if g.wfa(app).offset_0x150 == 5 {
        return true;
    }
    if g.wfa(app).offset_0x154 && g.board(this).field_0x364 < 5 {
        g.board(this).field_0x364 = 5;
    }
    let money = g.board(this).field_0x364;
    if money < param_1 {
        let flying = FUN_0053a0b0(g, this);
        if flying + money < param_1 {
            if param_2 {
                crate::game::board::FUN_00538230(g, this, 0x112, 3, 1.0);
                g.board(this).ext_0x450 = 0x46;
            } else if g.board(this).ext_0x450 == 0 {
                g.board(this).ext_0x450 = 0x46;
            }
            if g.board(this).field_0x364 == 0 && FUN_00537b80(g, this) {
                crate::game::win_fish_app::FUN_0054b5b0(
                    g,
                    app,
                    0x13,
                    1,
                    b"OUT OF MONEY!",
                    b"You've run out of money!  Because you're new, we'll float you a loan, but be careful next time!",
                    b"Click to Continue",
                    3,
                );
            }
            return false;
        }
    }
    let left = money - param_1;
    g.board(this).field_0x364 = left;
    if g.wfa(app).offset_0x154 && left < 5 && -1 < left {
        g.board(this).field_0x364 = 5;
    }
    FUN_0053a360(g, this);
    true
}

/// port: 0053a0b0 FUN_0053a0b0
/// Money in flight: the value of coins already clicked (+0xa8, +0xec) and of the +0xc8
/// objects collected, not yet counted.
pub fn FUN_0053a0b0(g: &mut G, this: Ptr) -> i32 {
    let mut sum = 0;
    for off in [0xa8, 0xec] {
        for o in g.board(this).offset_0xc[vec_index(off)].clone() {
            if g.coin(o).offset_0x44 {
                sum += crate::game::coin::FUN_004d53a0(g, o);
            }
        }
    }
    for o in g.board(this).offset_0xc[vec_index(0xc8)].clone() {
        if g.larva(o).offset_0x20 {
            sum += crate::game::larva::FUN_004d7e60(g, o);
        }
    }
    sum
}

/// port: 0053c2d0 FUN_0053c2d0
/// Counts the money in flight into the money (clamped to 0..9,999,999).
pub fn FUN_0053c2d0(g: &mut G, this: Ptr) {
    let v = FUN_0053a0b0(g, this);
    let b = g.board(this);
    b.field_0x364 += v;
    if b.field_0x364 < 0 {
        b.field_0x364 = 0;
        FUN_0053a360(g, this);
        return;
    }
    if 9999999 < b.field_0x364 {
        b.field_0x364 = 9999999;
    }
    FUN_0053a360(g, this);
}

/// port: 00538ac0 FUN_00538ac0
/// A bubble at (x, y), at most 30.
pub fn FUN_00538ac0(g: &mut G, this: Ptr, param_1: i32, param_2: i32) {
    let bm = g.board(this).offset_0x90;
    if (g.bubble_mgr(bm).offset_0x40.len() as i32) < 0x1e {
        crate::game::board_parts::FUN_00510f70(g, bm, param_1, param_2);
    }
}

/// port: 00538b60 FUN_00538b60
/// Where the backdrop's bubble vent is.
pub fn FUN_00538b60(g: &mut G, this: Ptr) -> (i32, i32) {
    match g.board(this).field_0x35c {
        1 => (0x1d6, 300),
        2 | 3 => (0x226, 0x13b),
        4 => (10, 0x145),
        5 => (0x21c, 0x13b),
        6 => (0x168, 0x13b),
        _ => (0x1e0, 300),
    }
}

/// port: 00538090 FUN_00538090
/// The shop button of slot `param_1` (null when the slot is empty).
pub fn FUN_00538090(g: &mut G, this: Ptr, param_1: usize) -> Ptr {
    let item = g.board(this).field_0x258[param_1];
    if item < 0 {
        return NULL;
    }
    g.board(this).field_0x378[item as usize]
}

/// port: 00534380 FUN_00534380
/// `StoreButton::SetPrice(int)`: "$n" (empty for 0) while the button is shown.
pub fn FUN_00534380(g: &mut G, this: Ptr, param_1: i32) {
    if g.w(this).offset_0x1 {
        let s = if param_1 < 1 { Vec::new() } else { format!("${param_1}").into_bytes() };
        g.menu_button(this).field_0x20 = s;
    }
}

/// port: 00538700 FUN_00538700
/// Tints every shown shop button's price.
pub fn FUN_00538700(g: &mut G, this: Ptr, param_1: Color) {
    for i in 0..7 {
        let b = g.board(this).field_0x378[i];
        if b != NULL && g.w(b).offset_0x1 {
            crate::game::menu_button::FUN_005303e0(g, b, param_1);
        }
    }
}

/// port: 004da780 FUN_004da780
/// Aliens are in the tank (the +0xb8 or +0xf4 vectors are not empty).
pub fn FUN_004da780(g: &mut G, this: Ptr) -> bool {
    let b = g.board(this);
    !b.offset_0xc[vec_index(0xb8)].is_empty() || !b.offset_0xc[vec_index(0xf4)].is_empty()
}

/// port: 00538ae0 FUN_00538ae0
/// The laser sound (one or two of 0x144 / 0x145), sometimes with 0x132.
pub fn FUN_00538ae0(g: &mut G, this: Ptr) -> u32 {
    let app = g.board(this).field_0x0;
    let r = app_rand(g, app);
    let mut id = 0x144;
    if r & 1 != 0 {
        crate::game::board::FUN_00538230(g, this, 0x144, 3, 1.0);
        id = 0x145;
    }
    crate::game::board::FUN_00538230(g, this, id, 3, 1.0);
    let r = app_rand(g, app);
    if r % 10 == 0 {
        crate::game::board::FUN_00538230(g, this, 0x132, 3, 1.0);
        return 0;
    }
    r / 10
}

/// port: 004e3ed0 FUN_004e3ed0
/// The +0xd4 vector is not empty.
pub fn FUN_004e3ed0(g: &mut G, this: Ptr) -> bool {
    !g.board(this).offset_0xc[vec_index(0xd4)].is_empty()
}

/// port: 004e3f00 FUN_004e3f00
/// The +0xc4 vector is not empty.
pub fn FUN_004e3f00(g: &mut G, this: Ptr) -> bool {
    !g.board(this).offset_0xc[vec_index(0xc4)].is_empty()
}

/// port: 004e3f30 FUN_004e3f30
/// The +0xcc vector is not empty.
pub fn FUN_004e3f30(g: &mut G, this: Ptr) -> bool {
    !g.board(this).offset_0xc[vec_index(0xcc)].is_empty()
}

/// port: 004e3f60 FUN_004e3f60
/// The +0xc0 vector is not empty.
pub fn FUN_004e3f60(g: &mut G, this: Ptr) -> bool {
    !g.board(this).offset_0xc[vec_index(0xc0)].is_empty()
}

/// port: 0053db80 FUN_0053db80
/// `Pause(bool)`: pausing only takes when no trial dialog forbids it; resuming ends the
/// holds and refreshes the money, the virtual tank's buttons, the shop prices, the alien
/// spots, Niko's spots (and the virtual tank's state), and shifts the melodies' timing by
/// the time spent paused.
pub fn FUN_0053db80(g: &mut G, this: Ptr, param_1: bool) {
    if g.board(this).field_0x8 == param_1 {
        return;
    }
    if !param_1 {
        let b = g.board(this);
        b.ext_0x4ec = false;
        b.ext_0x4ed = false;
        crate::game::board_level::FUN_0053a360(g, this);
        FUN_0053a4f0(g, this);
        FUN_0053a450(g, this);
        FUN_00539330(g, this);
        FUN_00539a50(g, this);
        let app = g.board(this).field_0x0;
        if g.wfa(app).offset_0x150 == 5 {
            crate::game::virtual_tank::FUN_0053a980(g, this);
        }
        let m = g.board(this).offset_0x4;
        crate::game::timed_messages::FUN_005051b0(g, m);
        g.board(this).field_0x8 = false;
    } else {
        let app = g.board(this).field_0x0;
        if !crate::game::win_fish_app::thunk_FUN_00479fc0(g, app) {
            g.board(this).field_0x8 = true;
        }
    }
}

/// port: 0053a4f0 FUN_0053a4f0
/// The virtual tank's state (nothing in the other modes): laser level 8, the food allowed
/// from its guppies and breeders (5..9), the food upgrades the profile lacks switched off,
/// Special Food shown when some creature eats it, Add/Remove Fish when a bought creature is
/// there, Add/Remove Pets when the profile has pets; in the screensaver every button off.
pub fn FUN_0053a4f0(g: &mut G, this: Ptr) {
    let app = g.board(this).field_0x0;
    if g.wfa(app).offset_0x150 != 5 {
        return;
    }
    g.board(this).field_0x358 = 8;
    let n = g.board(this).offset_0xc[vec_index(0xa0)].len() as i32 + g.board(this).offset_0xc[vec_index(0xc0)].len() as i32;
    g.globals.DAT_005df58c = n.clamp(5, 9);
    let profile = g.wfa(app).offset_0x18c;
    if g.profile(profile).field_0x90 == 0 {
        g.board(this).ext_0x4fd = false;
    }
    if g.profile(profile).field_0x94 == 0 {
        g.board(this).ext_0x4fe = false;
    }
    let feed = g.board(this).field_0x3a4;
    if feed != NULL {
        let c = crate::game::virtual_tank::FUN_0053a3c0(g, this);
        let any = c.iter().any(|&v| 0 < v);
        vcall!(g, feed, w.vfunction32, any);
    }
    let fish = g.board(this).field_0x398;
    if fish != NULL {
        let mut bought = false;
        let objs: Vec<Ptr> = g.board(this).offset_0x7c.iter().copied().collect();
        for o in objs {
            let id = g.go(o).offset_0x24;
            if -1 < id && id < 0x6c {
                bought = true;
            }
        }
        vcall!(g, fish, w.vfunction32, bought);
    }
    let pets = g.board(this).field_0x39c;
    if pets != NULL {
        let has = 0 < g.profile(profile).field_0x18;
        vcall!(g, pets, w.vfunction32, has);
    }
    if crate::game::win_fish_app::thunk_FUN_00479fc0(g, app) {
        let b = g.board(this).clone();
        for w in [b.field_0x394, b.field_0x398, b.field_0x39c, b.field_0x3a0, b.field_0x3a4, b.field_0x3a8, b.offset_0x3ac, b.offset_0x88] {
            vcall!(g, w, w.vfunction32, false);
        }
    }
}

/// port: 00539330 FUN_00539330
/// The virtual tank's alien spots (nothing in the other modes): the first by the portal
/// (left of it, or right of it for kind 8), the second anywhere in the tank.
pub fn FUN_00539330(g: &mut G, this: Ptr) {
    let app = g.board(this).field_0x0;
    if g.wfa(app).offset_0x150 != 5 {
        return;
    }
    let (px, py) = crate::game::virtual_tank::FUN_00538c10(g, this);
    g.board(this).field_0x240 = px - 0x27;
    g.board(this).field_0x244 = py - 0xb4;
    if g.board(this).field_0x230 == 8 {
        g.board(this).field_0x240 = px + 1;
    }
    let rng = g.wfa(app).offset_0x84;
    let r = crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng));
    g.board(this).field_0x248 = (r % 0x1c2) as i32 + 0x14;
    let r = crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng));
    g.board(this).field_0x24c = (r % 0xc3) as i32 + 0x69;
}

/// port: 00539a50 FUN_00539a50
/// Puts every Niko (+0xb0 pets) back on the backdrop's spot.
pub fn FUN_00539a50(g: &mut G, this: Ptr) {
    for p in g.board(this).offset_0xc[vec_index(0xb0)].clone() {
        let backdrop = g.board(this).field_0x35c;
        crate::game::other_pet::FUN_004d89f0(g, p, backdrop);
    }
}

/// port: 004da7d0 FUN_004da7d0
/// There are guppies (+0xa0).
pub fn FUN_004da7d0(g: &mut G, this: Ptr) -> bool {
    !g.board(this).offset_0xc[vec_index(0xa0)].is_empty()
}

/// port: 004da800 FUN_004da800
/// There is food (+0xac).
pub fn FUN_004da800(g: &mut G, this: Ptr) -> bool {
    !g.board(this).offset_0xc[vec_index(0xac)].is_empty()
}

/// port: 00539d40 FUN_00539d40
/// A click at (x, y) that lands on a +0xdc object, an alien or an alien part's body is
/// passed to the tank as a click there (guarded against re-entry by `DAT_005e9631`): true
/// when it was.
pub fn FUN_00539d40(g: &mut G, this: Ptr, param_1: i32, param_2: i32) -> bool {
    let mut hit = false;
    for o in g.board(this).offset_0xc[vec_index(0xdc)].clone() {
        if hit {
            break;
        }
        if vcall!(g, o, w.vfunction69, param_1, param_2) {
            hit = true;
        }
    }
    for o in g.board(this).offset_0xc[vec_index(0xb8)].clone() {
        if hit {
            break;
        }
        if vcall!(g, o, w.vfunction69, param_1, param_2) {
            hit = true;
        }
    }
    for o in g.board(this).offset_0xc[vec_index(0xf4)].clone() {
        if hit {
            break;
        }
        let body = crate::game::bilaterus::alien_part_body(g, o);
        if body != NULL && vcall!(g, body, w.vfunction69, param_1, param_2) {
            hit = true;
        }
    }
    if !hit {
        return false;
    }
    if g.globals.DAT_005e9631 {
        return false;
    }
    g.globals.DAT_005e9631 = true;
    vcall!(g, this, w.vfunction55, param_1, param_2, 1);
    g.globals.DAT_005e9631 = false;
    true
}

/// port: 00543f40 FUN_00543f40
/// Nostradamus at an alien attack's 30-update warning: half the time he sneezes (five
/// bubbles from each Nostradamus, the sneeze sound and up to five high notes), the attack
/// is put off (+0x2c0 = 635) with the tank music back, and "Attack Postponed by Sneeze of
/// Power!". (Returns `Rand() / 100`, which no caller reads.)
pub fn FUN_00543f40(g: &mut G, this: Ptr) -> i32 {
    use crate::game::timed_messages::{FUN_00511770, FUN_00511830, FUN_00515e30, Melody, Note};
    let r = crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32;
    if 0x32 <= r % 100 {
        return r / 100;
    }
    let mut n = 0;
    for p in g.board(this).offset_0xc[vec_index(0xb4)].clone() {
        if g.fish_type_pet(p).offset_0x8 == 0x15 {
            n += 1;
            for _ in 0..5 {
                let y = (crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32) % 0x15 + g.wc(p).offset_0x30 + 0x28;
                let x = (crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32) % 0x15 + g.wc(p).offset_0x2c + 0x1e;
                FUN_00538ac0(g, this, x, y);
            }
        }
    }
    g.board(this).field_0x228 = 0x1e;
    crate::game::board::FUN_00538230(g, this, 0x110, 3, 1.0);
    let mut m = Melody::default();
    FUN_00511770(&mut m);
    m.field_0x1c = g.res.DAT_005e8d8c;
    let n = n.min(5);
    for _ in 0..n {
        let d = (crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32) % 0x96 + 100;
        FUN_00511830(&mut m.field_0x4, Note { field_0x0: 0.0, field_0x8: 1.0, field_0x10: d as u32 });
    }
    let tm = g.board(this).offset_0x4;
    FUN_00515e30(g, tm, m);
    g.board(this).field_0x234 = 0x27b;
    let app = g.board(this).field_0x0;
    crate::game::win_fish_app::FUN_0054c390(g, app, false);
    FUN_0053e950(g, this, b"Attack Postponed by Sneeze of Power!", false, -1);
    0
}
