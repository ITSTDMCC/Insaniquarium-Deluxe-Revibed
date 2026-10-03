//! `Sexy::OtherTypePet`: the pets that are not fish: Stinky the snail (0) and Clyde the
//! jellyfish (5) collect coins, Niko the oyster (1) makes pearls, Rufus the crab (7) pinches
//! aliens on the floor, Rhubarb the hermit crab (0xe) snaps fish up off the floor. Presto
//! (`offset_0x74`) can be any of them. `OtherTypePet_data` starts at object offset 0x154.

use crate::game::board_level::vec_index;
use crate::game::game_object::{FUN_004f22c0, GameObject};
use crate::sexy::crt::ftol;
use crate::sexy::graphics::{FUN_00455800, FUN_00455870, FUN_00455880, FUN_004558c0, FUN_004558e0, FUN_00455890, FUN_00455cf0, FUN_00455d20, FUN_00455e40, FUN_004560a0, FUN_00456340};
use crate::sexy::image_font as font;
use crate::sexy::prelude::*;
use crate::sexy::types::{CRect, FUN_00433320};

/// `OtherTypePet_data` (object offset 0x154, 120 bytes).
#[derive(Debug, Clone, Default)]
pub struct OtherTypePet_data {
    /// +0x158 x.
    pub offset_0x4: f64,
    /// +0x160 y.
    pub offset_0xc: f64,
    /// +0x168 x speed.
    pub offset_0x14: f64,
    /// +0x170 y speed.
    pub offset_0x1c: f64,
    /// +0x178 the direction it faces (1.0 initially).
    pub offset_0x24: f64,
    /// +0x180 wandering x speed.
    pub offset_0x2c: f64,
    /// +0x188 speed divisor (2; Stinky 1.2, Clyde 1.5).
    pub offset_0x34: f64,
    /// +0x190 wandering direction.
    pub offset_0x3c: i32,
    /// +0x194 updates since it last steered at its target (40 initially).
    pub offset_0x40: i32,
    /// +0x198 wandering re-roll counter.
    pub offset_0x44: i32,
    /// +0x19c turn animation (+-20).
    pub offset_0x48: i32,
    /// +0x1a0 animation counter.
    pub offset_0x4c: i32,
    /// +0x1a4 animation cel.
    pub offset_0x50: i32,
    /// +0x1a8 Presto's transform glow counter (to 20).
    pub offset_0x54: i32,
    /// +0x1ac Presto's recharge countdown (360).
    pub offset_0x58: i32,
    /// +0x1b0 asleep (virtual tank) / sleeping bubble.
    pub offset_0x5c: bool,
    /// +0x1b4 pet timer (Stinky: hiding in its shell while aliens attack; Niko: pearl
    /// cycle; Rhubarb: snap).
    pub offset_0x60: i32,
    /// +0x1b8 a random period (250..499).
    pub offset_0x64: i32,
    /// +0x1bc updates without a coin (Stinky turns red when idle long).
    pub offset_0x68: i32,
    /// +0x1c0 Niko's pearl is out.
    pub offset_0x6c: bool,
    /// +0x1c4 pet id.
    pub offset_0x70: i32,
    /// +0x1c8 this is Presto.
    pub offset_0x74: bool,
}

impl G {
    pub fn other_pet(&mut self, p: Ptr) -> &mut OtherTypePet_data {
        match &mut self.go_ext(p).sub {
            GoSub::OtherTypePet(d) => d,
            s => panic!("{p} is not an OtherTypePet: {s:?}"),
        }
    }
}

fn alloc_pet(g: &mut G, wc: crate::sexy::object::WidgetContainer_data, w: crate::sexy::object::Widget_data,
             go: crate::game::game_object::GameObject_data, d: OtherTypePet_data) -> Ptr {
    g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__OtherTypePet_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::GameObject(Box::new(GameObjectExt { go, sub: GoSub::OtherTypePet(d) })) }),
    })
}

fn rand(g: &mut G, app: Ptr) -> u32 {
    let rng = g.wfa(app).offset_0x84;
    crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng))
}

fn board_of(g: &mut G, this: Ptr) -> Ptr {
    let app = g.go(this).offset_0x0;
    g.wfa(app).offset_0x4
}

/// port: 004eb590 Sexy::OtherTypePet::OtherTypePet
/// `OtherTypePet::OtherTypePet()` (used before loading from a save).
pub fn OtherTypePet__004eb590(g: &mut G) -> Ptr {
    let (mut wc, w, mut go) = GameObject(g);
    wc.offset_0x3d = false;
    go.offset_0x4 = 0x14;
    alloc_pet(g, wc, w, go, OtherTypePet_data::default())
}

/// port: 004eb5c0 Sexy::OtherTypePet::OtherTypePet
/// `OtherTypePet(int x, int y, int pet, int backdrop, bool presto)`: 80x80; the floor
/// crawlers start on the floor; Presto recharges first (not in the virtual tank).
pub fn OtherTypePet__004eb5c0(g: &mut G, param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: bool) -> Ptr {
    let (mut wc, mut w, mut go) = GameObject(g);
    wc.offset_0x3d = false;
    let mut d = OtherTypePet_data { offset_0x74: param_5, offset_0x70: param_3, ..Default::default() };
    go.offset_0x4 = 0x14;
    w.offset_0x1 = g.globals.DAT_005e8f14;
    d.offset_0x54 = 0;
    d.offset_0x58 = if !param_5 || param_3 == 0x13 { 0 } else { 0x168 };
    let app = go.offset_0x0;
    if g.wfa(app).offset_0x150 == 5 {
        d.offset_0x58 = 0;
    }
    d.offset_0x4 = param_1 as f64;
    d.offset_0xc = param_2 as f64;
    if !param_5 {
        match param_3 {
            0 => d.offset_0xc = 370.0,
            0xe => d.offset_0xc = 355.0,
            7 => d.offset_0xc = 365.0,
            _ => {}
        }
    }
    if d.offset_0x70 == 1 && !d.offset_0x74 {
        // FUN_004d89f0(backdrop), as it runs inside the constructor.
        let (x, y) = niko_spot(param_4);
        d.offset_0x4 = x;
        d.offset_0xc = y;
    }
    wc.offset_0x2c = ftol(d.offset_0x4) as i32;
    wc.offset_0x30 = ftol(d.offset_0xc) as i32;
    d.offset_0x14 = 0.0;
    d.offset_0x1c = 0.0;
    d.offset_0x2c = 0.0;
    wc.offset_0x34 = 0x50;
    wc.offset_0x38 = 0x50;
    d.offset_0x24 = 1.0;
    d.offset_0x34 = 2.0;
    if d.offset_0x70 == 0 {
        d.offset_0x34 = 1.2;
    }
    if d.offset_0x70 == 5 {
        d.offset_0x34 = 1.5;
    }
    d.offset_0x3c = (rand(g, app) % 10) as i32;
    d.offset_0x40 = 0x28;
    d.offset_0x44 = 0;
    d.offset_0x4c = 0;
    d.offset_0x48 = 0;
    d.offset_0x50 = 0;
    d.offset_0x60 = 0;
    let r = rand(g, app);
    d.offset_0x6c = false;
    d.offset_0x68 = 0;
    d.offset_0x64 = (r % 0xfa) as i32 + 0xfa;
    alloc_pet(g, wc, w, go, d)
}

/// Niko's spot on each backdrop.
fn niko_spot(param_1: i32) -> (f64, f64) {
    match param_1 {
        1 => (95.0, 253.0),
        2 => (175.0, 163.0),
        3 => (65.0, 156.0),
        4 => (145.0, 260.0),
        5 => (67.0, 185.0),
        _ => (160.0, 176.0),
    }
}

/// port: 004d89f0 FUN_004d89f0
/// Niko (not Presto) sits on its backdrop's spot.
pub fn FUN_004d89f0(g: &mut G, this: Ptr, param_1: i32) {
    let d = g.other_pet(this);
    if d.offset_0x70 == 1 && !d.offset_0x74 {
        let (x, y) = niko_spot(param_1);
        d.offset_0x4 = x;
        d.offset_0xc = y;
        g.wc(this).offset_0x2c = ftol(x) as i32;
        g.wc(this).offset_0x30 = ftol(y) as i32;
    }
}

/// port: 004d8ad0 Sexy::OtherTypePet::vfunction81
/// `Sync(DataSync&)`.
pub fn vfunction81(g: &mut G, this: Ptr, sync: &mut crate::sexy::data_sync::DataSync) -> crate::sexy::data_sync::SyncResult {
    crate::game::game_object::vfunction81(g, this, sync)?;
    use crate::sexy::data_sync::{FUN_00503010, FUN_00503070, FUN_005030a0};
    let mut d = g.other_pet(this).clone();
    FUN_005030a0(sync, &mut d.offset_0x4)?;
    FUN_005030a0(sync, &mut d.offset_0xc)?;
    FUN_005030a0(sync, &mut d.offset_0x14)?;
    FUN_005030a0(sync, &mut d.offset_0x1c)?;
    FUN_005030a0(sync, &mut d.offset_0x24)?;
    FUN_005030a0(sync, &mut d.offset_0x2c)?;
    FUN_005030a0(sync, &mut d.offset_0x34)?;
    FUN_00503010(sync, &mut d.offset_0x3c)?;
    FUN_00503010(sync, &mut d.offset_0x40)?;
    FUN_00503010(sync, &mut d.offset_0x44)?;
    FUN_00503010(sync, &mut d.offset_0x48)?;
    FUN_00503010(sync, &mut d.offset_0x4c)?;
    FUN_00503010(sync, &mut d.offset_0x50)?;
    FUN_00503010(sync, &mut d.offset_0x54)?;
    FUN_00503010(sync, &mut d.offset_0x58)?;
    FUN_00503010(sync, &mut d.offset_0x60)?;
    FUN_00503010(sync, &mut d.offset_0x64)?;
    FUN_00503010(sync, &mut d.offset_0x68)?;
    FUN_00503070(sync, &mut d.offset_0x6c)?;
    FUN_00503010(sync, &mut d.offset_0x70)?;
    FUN_00503070(sync, &mut d.offset_0x74)?;
    *g.other_pet(this) = d;
    Ok(())
}

/// port: 004e0e70 Sexy::OtherTypePet::vfunction76
/// `Remove()`: drops the missile on it, leaves the tank with its shadow.
pub fn vfunction76(g: &mut G, this: Ptr) {
    let m = g.go(this).offset_0x10;
    if m != NULL {
        crate::game::missle::vfunction76(g, m);
    }
    crate::game::dead_fish::vfunction82(g, this);
}

/// port: 004d6b70 FUN_004d6b70
/// Virtual tank: falls asleep (1 in 60 per update) after the player has been idle a while;
/// awake again once there was input.
pub fn FUN_004d6b70(g: &mut G, this: Ptr, param_1: &mut bool) {
    let app = g.go(this).offset_0x0;
    if !crate::game::win_fish_app::thunk_FUN_00479fc0(g, app) {
        let wm = g.wc(this).offset_0xc;
        let idle = g.wc(wm).offset_0x24 - crate::sexy::widget_manager::wm(g, wm).offset_0x9c;
        if idle < 0x1950 {
            *param_1 = false;
            return;
        }
        if !*param_1 && crate::sexy::sexy_app_base::FUN_0040f9e0(g) % 0x3c == 0 {
            *param_1 = true;
        }
    }
}

/// port: 004da710 FUN_004da710
/// `std::vector::size()` (an STL instance).
pub fn FUN_004da710(this: &[Ptr]) -> i32 {
    this.len() as i32
}

/// port: 004e4f90 FUN_004e4f90
/// Clyde leaves silver and gold coins alone in the virtual tank's breeding (with +0xd0).
pub fn FUN_004e4f90(g: &mut G, this: Ptr, param_1: Ptr) -> bool {
    let app = g.go(this).offset_0x0;
    if g.wfa(app).offset_0x154 {
        let board = g.wfa(app).offset_0x4;
        if !g.board(board).offset_0xc[vec_index(0xd0)].is_empty() {
            let k = g.coin(param_1).offset_0x40;
            if k == 2 || k == 1 {
                return true;
            }
        }
    }
    false
}

/// `ftol((obj.mX + k) - (x + 40))` squared plus the same for y (`FUN_004e7270`'s distance).
fn dist(g: &mut G, this: Ptr, o: Ptr, k: i32) -> i32 {
    let d = g.other_pet(this).clone();
    let (ox, oy) = (g.wc(o).offset_0x2c, g.wc(o).offset_0x30);
    let dx = ftol((ox + k) as f64 - (d.offset_0x4 + 40.0)) as i32;
    let dy = ftol((oy + k) as f64 - (d.offset_0xc + 40.0)) as i32;
    dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx))
}

/// port: 004e7270 FUN_004e7270
/// `FindTarget()`: Stinky and Clyde the nearest coin they can take (stars stay for the
/// starcatchers; no bombs, collected coins or beetles); Rufus the nearest alien on the
/// floor (+0xf4, then +0xb8 not in its healing form); Rhubarb the nearest creature low in
/// the tank (below 260, 250 with aliens around).
pub fn FUN_004e7270(g: &mut G, this: Ptr) -> Ptr {
    let board = board_of(g, this);
    let pet = g.other_pet(this).offset_0x70;
    let mut best = 100000000;
    let mut target = NULL;
    if pet == 0 || pet == 5 {
        let no_catchers = g.board(board).offset_0xc[vec_index(0xd0)].is_empty();
        for c in g.board(board).offset_0xc[vec_index(0xa8)].clone() {
            let cd = g.coin(c).clone();
            if !no_catchers && (cd.offset_0x40 == 3 || cd.offset_0x40 == 10) {
                continue;
            }
            if cd.offset_0x40 == 0x11 || cd.offset_0x44 || cd.offset_0x40 == 0x12 {
                continue;
            }
            if g.other_pet(this).offset_0x70 == 5 && FUN_004e4f90(g, this, c) {
                continue;
            }
            let d = dist(g, this, c, 0x24);
            if d < best {
                best = d;
                target = c;
            }
        }
    } else if pet == 7 {
        for a in g.board(board).offset_0xc[vec_index(0xf4)].clone() {
            let d = dist(g, this, a, 0x28);
            if d < best {
                best = d;
                target = a;
            }
        }
        for a in g.board(board).offset_0xc[vec_index(0xb8)].clone() {
            let d = dist(g, this, a, 0x50);
            if !g.alien(a).offset_0xb0 && d < best {
                best = d;
                target = a;
            }
        }
    } else if pet == 0xe {
        let low = if crate::game::board_update::FUN_004da780(g, board) { 0xfa } else { 0x104 };
        for (off, k, lim) in [(0xa0, 0x28, low), (0xa4, 0x28, low), (0xcc, 0x28, low), (0xc0, 0x28, low), (0xd4, 0x50, low - 0x28)] {
            for o in g.board(board).offset_0xc[vec_index(off)].clone() {
                let d = dist(g, this, o, k);
                if lim < g.wc(o).offset_0x30 && d < best {
                    best = d;
                    target = o;
                }
            }
        }
    }
    target
}

/// port: 004e7c30 FUN_004e7c30
/// `TouchTarget()`: Stinky and Clyde collect a coin they touch (the pet way: paid, the
/// collect sound, gone); Rufus pinches aliens within 10 (2, Destructor 0.25, Ulysses and
/// Gus 0.5) and on the floor in reach, stopping each time; Rhubarb flings creatures above
/// its claws upward once it has wound up (timer 5), and starts winding up under them.
pub fn FUN_004e7c30(g: &mut G, this: Ptr) {
    let board = board_of(g, this);
    let pet = g.other_pet(this).offset_0x70;
    let app = g.go(this).offset_0x0;
    if pet == 0 || pet == 5 {
        let no_catchers = g.board(board).offset_0xc[vec_index(0xd0)].is_empty();
        for c in g.board(board).offset_0xc[vec_index(0xa8)].clone() {
            let cd = g.coin(c).clone();
            if !no_catchers && (cd.offset_0x40 == 3 || cd.offset_0x40 == 10) {
                continue;
            }
            if cd.offset_0x44 || cd.offset_0x40 == 0x12 {
                continue;
            }
            if g.other_pet(this).offset_0x70 == 5 && FUN_004e4f90(g, this, c) {
                continue;
            }
            let d = g.other_pet(this).clone();
            let (cx, cy) = (g.wc(c).offset_0x2c, g.wc(c).offset_0x30);
            let px = d.offset_0x4 + 40.0;
            let py = d.offset_0xc + 40.0;
            if px < (cx + 0x38) as f64 && ((cx + 0x10) as f64) < px && py < (cy + 0x38) as f64 && ((cy + 0x10) as f64) < py && cd.offset_0x40 < 0xf {
                crate::game::coin::FUN_004ddc40(g, c);
                crate::game::coin::vfunction76(g, c);
                g.other_pet(this).offset_0x68 = 0;
                return;
            }
        }
        return;
    }
    if pet == 7 {
        for a in g.board(board).offset_0xc[vec_index(0xf4)].clone() {
            let d = g.other_pet(this).clone();
            let (ax, ay) = (g.wc(a).offset_0x2c, g.wc(a).offset_0x30);
            if ((d.offset_0x4 + 40.0) - (ax + 0x28) as f64).abs() < 10.0 && ((d.offset_0xc + 40.0) - (ay + 0x28) as f64).abs() < 10.0 {
                crate::game::bilaterus::alien_part_body_hurt(g, a, 2.0);
                crate::game::board_level::FUN_00538340(g, board, 10);
                let p = g.other_pet(this);
                p.offset_0x14 = 0.0;
                p.offset_0x2c = 0.0;
            }
        }
        for a in g.board(board).offset_0xc[vec_index(0xb8)].clone() {
            let d = g.other_pet(this).clone();
            let (ax, ay) = (g.wc(a).offset_0x2c, g.wc(a).offset_0x30);
            let px = d.offset_0x4 + 40.0;
            let py = d.offset_0xc + 40.0;
            if px < (ax + 0x8c) as f64 && ((ax + 0x1e) as f64) < px && py < (ay + 0x96) as f64 && ((ay + 10) as f64) < py && !g.alien(a).offset_0xb0 {
                let dmg = match g.alien(a).offset_0x98 {
                    5 => 0.25,
                    6 | 4 => 0.5,
                    _ => 2.0,
                };
                g.alien(a).offset_0x9c -= dmg;
                crate::game::board_level::FUN_00538340(g, board, 10);
                let p = g.other_pet(this);
                p.offset_0x14 = 0.0;
                p.offset_0x2c = 0.0;
            }
        }
        return;
    }
    if pet == 0xe {
        let low = if crate::game::board_update::FUN_004da780(g, board) { 0x104 } else { 0x10e };
        for (off, w1, w2, lim, field) in [
            (0xa0, 0x50, 0x5a, low, 0x208),
            (0xa4, 0x50, 0x5a, low, 0x208),
            (0xcc, 0x50, 0x5a, low, 0x208),
            (0xc0, 0x50, 0x5a, low, 0x1d8),
            (0xd4, 0xa0, 0xaa, low - 0x28, 0x208),
        ] {
            for o in g.board(board).offset_0xc[vec_index(off)].clone() {
                let px = g.other_pet(this).offset_0x4 + 40.0;
                let (ox, oy) = (g.wc(o).offset_0x2c, g.wc(o).offset_0x30);
                if px < (ox + w1) as f64 && (ox as f64) < px && lim < oy && g.other_pet(this).offset_0x60 == 5 {
                    set_flung(g, o, field, 0x32);
                    let up = if rand(g, app) & 1 == 0 { -20.0 } else { -30.0 };
                    set_vy(g, o, up);
                }
                let px = g.other_pet(this).offset_0x4 + 40.0;
                if px < (ox + w2) as f64 && ((ox - 10) as f64) < px && lim < oy && g.other_pet(this).offset_0x60 == 0 {
                    g.other_pet(this).offset_0x60 = 0x14;
                }
            }
        }
    }
}

/// port: 004ebc80 FUN_004ebc80
/// `Chase()`: steers at the target every fifth update (Stinky in steps by distance, and
/// counting idle time with Wadsworth around; Clyde in both axes; the crabs fast); then
/// touches it. True when there is a target.
pub fn FUN_004ebc80(g: &mut G, this: Ptr) -> bool {
    let t = FUN_004e7270(g, this);
    if 4 < g.other_pet(this).offset_0x40 {
        if t == NULL {
            return false;
        }
        let pet = g.other_pet(this).offset_0x70;
        g.other_pet(this).offset_0x40 = 0;
        let (tx, ty) = (g.wc(t).offset_0x2c, g.wc(t).offset_0x30);
        let board = board_of(g, this);
        let d = g.other_pet(this);
        let p = d.offset_0x4 + 40.0;
        match pet {
            0 => {
                let v = d.offset_0x14;
                if ((tx + 0x30) as f64) < p {
                    if -2.3 < v {
                        d.offset_0x14 = v - 1.0;
                    }
                } else if p < (tx + 0x18) as f64 {
                    if v < 2.3 {
                        d.offset_0x14 = v + 1.0;
                    }
                } else if ((tx + 0x28) as f64) < p {
                    if -1.3 < v {
                        d.offset_0x14 = v - 0.5;
                    }
                } else if p < (tx + 0x20) as f64 {
                    if v < 1.3 {
                        d.offset_0x14 = v + 0.5;
                    }
                } else if ((tx + 0x24) as f64) < p {
                    if -0.3 < v {
                        d.offset_0x14 = 0.0;
                    }
                } else if p < (tx + 0x24) as f64 && v < 0.3 {
                    d.offset_0x14 = 0.0;
                }
                if g.board(board).field_0xb8[15] != 0 {
                    g.other_pet(this).offset_0x68 += 1;
                }
            }
            5 => {
                let b = (tx + 0x24) as f64;
                if p <= b {
                    if p < b && d.offset_0x14 < 2.0 {
                        d.offset_0x14 += 1.0;
                    }
                } else if -2.0 < d.offset_0x14 {
                    d.offset_0x14 -= 1.0;
                }
                let p = d.offset_0xc + 40.0;
                let b = (ty + 0x24) as f64;
                if p <= b {
                    if p < b && d.offset_0x1c < 2.0 {
                        d.offset_0x1c += 1.0;
                    }
                } else if -2.0 < d.offset_0x1c {
                    d.offset_0x1c -= 1.0;
                }
            }
            7 | 0xe => {
                let k = if pet == 7 {
                    if g.go(t).offset_0x4 != 0x17 { 0x50 } else { 0x28 }
                } else {
                    0x28
                };
                let d = g.other_pet(this);
                let b = (tx + k) as f64;
                if b < p {
                    if -5.0 < d.offset_0x14 {
                        d.offset_0x14 -= 1.8;
                    }
                } else if p < b && d.offset_0x14 < 5.0 {
                    d.offset_0x14 += 1.8;
                }
            }
            _ => {}
        }
    }
    if t != NULL {
        FUN_004e7c30(g, this);
    }
    t != NULL
}

/// port: 004efe80 FUN_004efe80
/// `Hunt()`: Stinky and Clyde chase coins (Stinky not while aliens are around, Clyde not
/// while asleep); Rufus chases aliens while they are around; Rhubarb chases fish. True when
/// it is chasing.
pub fn FUN_004efe80(g: &mut G, this: Ptr) -> bool {
    let pet = g.other_pet(this).offset_0x70;
    let board = board_of(g, this);
    if pet == 0 || pet == 5 {
        if !g.board(board).offset_0xc[vec_index(0xa8)].is_empty() {
            if pet == 0 && crate::game::board_update::FUN_004da780(g, board) {
                return false;
            }
            if !g.other_pet(this).offset_0x5c {
                FUN_004ebc80(g, this);
                return true;
            }
        }
    } else if pet == 7 {
        if crate::game::board_update::FUN_004da780(g, board) {
            FUN_004ebc80(g, this);
            return true;
        }
    } else if pet == 0xe && FUN_004ebc80(g, this) {
        return true;
    }
    false
}

/// port: 004e2a20 FUN_004e2a20
/// `PetTimer()`: Stinky hides in its shell (to 9) while aliens attack and comes out again;
/// Niko (outside tank 5) opens at 1224 with a pearl (kind 0xf) at 1233 and closes at 1440,
/// starting over at a random point of the first 50.
pub fn FUN_004e2a20(g: &mut G, this: Ptr) {
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    let pet = g.other_pet(this).offset_0x70;
    if pet == 0 {
        if g.board(board).offset_0xc[vec_index(0xb8)].is_empty() && g.board(board).offset_0xc[vec_index(0xf4)].is_empty() {
            let t = g.other_pet(this).offset_0x60;
            if 0 < t {
                g.other_pet(this).offset_0x60 = t - 1;
            }
            return;
        }
        let d = g.other_pet(this);
        if d.offset_0x60 < 9 {
            d.offset_0x60 += 1;
        }
        d.offset_0x68 = 0;
        return;
    }
    if g.board(board).field_0x33c != 5 && pet == 1 {
        g.other_pet(this).offset_0x60 += 1;
        match g.other_pet(this).offset_0x60 {
            0x4c8 => crate::game::board::FUN_00538230(g, board, 0x129, 3, 1.0),
            0x5a0 => crate::game::board::FUN_00538230(g, board, 0x12a, 3, 1.0),
            0x4d1 => {
                let (mx, my) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
                crate::game::board_level::FUN_00544430(g, board, mx + 1, my - 2, 0xf, this, -1.0, 0);
                g.other_pet(this).offset_0x6c = false;
                crate::game::board_update::FUN_00538ac0(g, board, mx + 0xb, my + 5);
                crate::game::board_update::FUN_00538ac0(g, board, mx + 7, my + 3);
            }
            0x5aa => {
                let r = rand(g, app) % 0x32;
                g.other_pet(this).offset_0x60 = r as i32;
            }
            _ => {}
        }
    }
}

/// port: 004eb7c0 FUN_004eb7c0
/// `Animate()`: Stinky's turn and crawl cycles (or its shell cel); Rufus' scuttle (or
/// idling claws when still); Rhubarb's snap and walk; Clyde's pulse; Niko's cycle; the
/// carried missile's sparkle cycle.
pub fn FUN_004eb7c0(g: &mut G, this: Ptr) {
    let pet = g.other_pet(this).offset_0x70;
    let board = board_of(g, this);
    if pet != 0 {
        let d = g.other_pet(this).clone();
        let v = d.offset_0x14;
        let walk_left_right = |g: &mut G, step: i32| {
            let d = g.other_pet(this);
            let n = (d.offset_0x4c + step) % 0x28;
            d.offset_0x4c = n;
            d.offset_0x50 = n / 4;
        };
        let back = |g: &mut G, step: i32| {
            let d = g.other_pet(this);
            d.offset_0x4c -= step;
            if d.offset_0x4c < 0 {
                d.offset_0x4c += 0x28;
            }
            d.offset_0x50 = d.offset_0x4c / 4;
        };
        if pet == 7 {
            let still = v <= 0.01 && -0.01 <= v && d.offset_0x2c == 0.0;
            if still && FUN_004da710(&g.board(board).offset_0xc[vec_index(0xb8)].clone()) == 0 {
                let p = g.other_pet(this);
                let n = (p.offset_0x4c + 1) % 0x3c;
                p.offset_0x4c = n;
                if n < 3 {
                    p.offset_0x50 = n / 2;
                } else if n < 0x18 {
                    p.offset_0x50 = 2;
                } else if n < 0x21 {
                    p.offset_0x50 = n / 2 - 10;
                } else if n < 0x36 {
                    p.offset_0x50 = 7;
                } else if n < 0x3c {
                    p.offset_0x50 = n / 2 - 0x14;
                }
            } else {
                g.other_pet(this).offset_0x4c %= 0x28;
                let d = g.other_pet(this).clone();
                let still = d.offset_0x14 <= 0.01 && -0.01 <= d.offset_0x14 && d.offset_0x2c == 0.0 && g.board(board).field_0x84 == NULL;
                if still && FUN_004e7270(g, this) != NULL {
                    walk_left_right(g, 4);
                } else {
                    let v = g.other_pet(this).offset_0x14;
                    if v <= 1.0 {
                        if v <= -1.0 {
                            back(g, 2);
                        } else if 0.0 < v {
                            walk_left_right(g, 1);
                        } else {
                            back(g, 1);
                        }
                    } else {
                        walk_left_right(g, 2);
                    }
                }
            }
        } else if pet == 0xe {
            if 0 < d.offset_0x60 {
                let p = g.other_pet(this);
                let t = p.offset_0x60 - 1;
                p.offset_0x60 = t;
                p.offset_0x50 = t / 2;
                if t == 10 {
                    let app = g.go(this).offset_0x0;
                    let s = g.res.DAT_005e8c40;
                    crate::sexy::sexy_app_base::vfunction55(g, app, s);
                }
            } else if 1.0 < v {
                back(g, 2);
            } else if v <= -1.0 {
                walk_left_right(g, 2);
            } else if 0.0 < v {
                back(g, 1);
            } else {
                walk_left_right(g, 1);
            }
        } else if pet == 5 {
            let p = g.other_pet(this);
            let n = (p.offset_0x4c + 1) % 0x28;
            p.offset_0x4c = n;
            if n < 7 {
                p.offset_0x50 = n / 2;
            } else if n < 0x10 {
                p.offset_0x50 = 4;
            } else {
                p.offset_0x50 = n / 4;
            }
        } else {
            let p = g.other_pet(this);
            p.offset_0x4c = (p.offset_0x4c + 1) % 0x13;
        }
    } else {
        let d = g.other_pet(this);
        if 0.0 < d.offset_0x24 && d.offset_0x14 < 0.0 {
            d.offset_0x48 = 0x14;
        } else if d.offset_0x24 < 0.0 && 0.0 < d.offset_0x14 {
            d.offset_0x48 = -0x14;
        }
        let t = d.offset_0x48;
        if 0 < t {
            d.offset_0x48 = t - 1;
        } else if t < 0 {
            d.offset_0x48 = t + 1;
        }
        let t = d.offset_0x48;
        if t == 0 {
            if 0.3 <= d.offset_0x14.abs() {
                let n = (d.offset_0x4c + 1) % 0x14;
                d.offset_0x4c = n;
                d.offset_0x50 = n / 2;
            } else {
                let n = (d.offset_0x4c + 1) % 0x28;
                d.offset_0x4c = n;
                d.offset_0x50 = n / 4;
            }
        } else if t < 0 {
            d.offset_0x50 = t / 2 + 9;
        } else {
            d.offset_0x50 = 9 - t / 2;
        }
        let shell = d.offset_0x60;
        if 0 < shell {
            d.offset_0x50 = shell;
        }
        if d.offset_0x14 != d.offset_0x24 && d.offset_0x14 != 0.0 && d.offset_0x24 != 0.0 && shell == 0 {
            d.offset_0x24 = d.offset_0x14;
        }
    }
    if g.go(this).offset_0x10 != NULL {
        crate::game::game_object::FUN_004d7020(g, this);
    }
}

/// port: 004f3940 Sexy::OtherTypePet::vfunction74
/// `Morph(int pet)` (Presto): when charged, the transformation sound, the new pet where it
/// is (as Presto), and this one leaves.
pub fn vfunction74(g: &mut G, this: Ptr, param_1: i32) {
    if param_1 != g.other_pet(this).offset_0x70 {
        let charge = g.other_pet(this).offset_0x58;
        if FUN_004f2650(g, this, charge) {
            let board = board_of(g, this);
            crate::game::board::FUN_00538230(g, board, 0x122, 3, 1.0);
            let (mx, my) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
            crate::game::board_level::FUN_00544a90(g, board, param_1, mx, my, true, false);
            crate::game::dead_fish::vfunction82(g, this);
        }
    }
}

/// port: 004f2650 FUN_004f2650
/// Presto's charge check: ready when the recharge is over; else a buzz and "Recharging...".
pub fn FUN_004f2650(g: &mut G, this: Ptr, param_1: i32) -> bool {
    if param_1 < 1 {
        return true;
    }
    let board = board_of(g, this);
    crate::game::board::FUN_00538230(g, board, 0x112, 3, 1.0);
    let wc = g.wc(this).clone();
    crate::game::board_level::FUN_005444e0(g, board, wc.offset_0x34 / 2 - 0x14 + wc.offset_0x2c, wc.offset_0x30 - 10, 2, b"Recharging...");
    false
}

/// port: 004f3a00 FUN_004f3a00
/// A right click on the pet: Presto opens its pet choice when charged; any other pet
/// passes the click to the tank. True when handled.
pub fn FUN_004f3a00(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32) -> bool {
    if -1 < param_3 {
        return false;
    }
    if g.other_pet(this).offset_0x74 {
        let charge = g.other_pet(this).offset_0x58;
        if FUN_004f2650(g, this, charge) {
            let app = g.go(this).offset_0x0;
            crate::game::pet_dialog::FUN_0054c6f0(g, app, this);
        }
        return true;
    }
    let board = board_of(g, this);
    let (mx, my) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
    crate::game::board_level::FUN_0053bfb0(g, board, mx + param_1, my + param_2);
    true
}

/// port: 004f3a70 Sexy::OtherTypePet::vfunction55
/// `MouseDown(x, y, clicks)`: unless it handled a right click, the click goes to the board
/// (guarded against re-entry by `DAT_005e89f8`).
pub fn vfunction55(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32) {
    if !FUN_004f3a00(g, this, param_1, param_2, param_3) && !g.globals.DAT_005e89f8 {
        g.globals.DAT_005e89f8 = true;
        let board = board_of(g, this);
        let (mx, my) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
        vcall!(g, board, w.vfunction55, mx + param_1, my + param_2, param_3);
        g.globals.DAT_005e89f8 = false;
    }
}

/// port: 004f2e80 Sexy::OtherTypePet::vfunction23
/// `Update()` (not while paused): Presto as a crawler sinks; each pet's wandering (Clyde
/// sinks and pulses up; the crabs scuttle at several speeds; Stinky creeps, stopping inside
/// its shell) unless it is chasing; direction re-rolls; Presto's timers; the pet timer;
/// staying in the tank (each crawler on its floor); turning at the walls; animating;
/// moving (5x slower asleep).
pub fn vfunction23(g: &mut G, this: Ptr) {
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    if board == NULL || g.board(board).field_0x8 {
        return;
    }
    FUN_004f22c0(g, this);
    {
        let d = g.other_pet(this);
        let pet = d.offset_0x70;
        if d.offset_0x74 && (pet == 1 || pet == 7 || pet == 0xe || pet == 0) && d.offset_0xc < 380.0 {
            d.offset_0x1c += 0.1;
        }
    }
    let tank = g.board(board).field_0x33c;
    let pet = g.other_pet(this).offset_0x70;
    if tank == 5 || pet != 0 {
        if pet == 5 {
            if g.wfa(app).offset_0x150 == 5 {
                let mut s = g.other_pet(this).offset_0x5c;
                FUN_004d6b70(g, this, &mut s);
                g.other_pet(this).offset_0x5c = s;
            }
            let board = g.wfa(app).offset_0x4;
            if g.board(board).field_0x33c == 5 || !FUN_004efe80(g, this) {
                let d = g.other_pet(this);
                if d.offset_0x3c == 0 || d.offset_0x60 != 0 {
                    d.offset_0x2c = 0.0;
                } else if d.offset_0x3c == 1 {
                    d.offset_0x2c = -0.5;
                } else if d.offset_0x3c == 2 {
                    d.offset_0x2c = 0.5;
                }
            }
            let d = g.other_pet(this);
            if d.offset_0x2c < d.offset_0x14 {
                d.offset_0x14 -= 0.1;
            }
            if d.offset_0x14 < d.offset_0x2c {
                d.offset_0x14 += 0.1;
            }
            if d.offset_0x4c == 2 || d.offset_0x4c == 3 {
                d.offset_0x1c -= if 240.0 <= d.offset_0xc { 0.75 } else { 0.3 };
            } else {
                d.offset_0x1c += 0.03;
            }
        } else if pet == 7 || (pet == 0xe && tank != 5) {
            let wander = if pet == 7 {
                tank == 5 || !FUN_004efe80(g, this)
            } else {
                FUN_004efe80(g, this);
                true
            };
            if wander {
                let d = g.other_pet(this);
                let v = match (pet, d.offset_0x3c) {
                    (_, 0) => Some(0.0),
                    (_, 1) => Some(-0.5),
                    (_, 2) => Some(0.5),
                    (_, 3) => Some(-1.0),
                    (_, 4) => Some(1.0),
                    (7, 5) | (7, 6) => Some(0.0),
                    (_, 5) => Some(1.5),
                    (_, 6) => Some(-1.5),
                    (_, 7) => Some(-2.5),
                    (_, 8) => Some(2.5),
                    _ => None,
                };
                if let Some(v) = v {
                    d.offset_0x2c = v;
                }
            }
            let d = g.other_pet(this);
            if d.offset_0x2c < d.offset_0x14 {
                d.offset_0x14 -= 0.1;
            }
            if d.offset_0x14 < d.offset_0x2c {
                d.offset_0x14 += 0.1;
            }
        }
    } else {
        if g.wfa(app).offset_0x150 == 5 {
            let mut s = g.other_pet(this).offset_0x5c;
            FUN_004d6b70(g, this, &mut s);
            g.other_pet(this).offset_0x5c = s;
            if s {
                g.other_pet(this).offset_0x68 = 0;
            }
        }
        if !FUN_004efe80(g, this) {
            let d = g.other_pet(this);
            if d.offset_0x3c == 0 || d.offset_0x60 != 0 {
                d.offset_0x2c = 0.0;
            } else if d.offset_0x3c == 1 {
                d.offset_0x2c = -0.5;
            } else if d.offset_0x3c == 2 {
                d.offset_0x2c = 0.5;
            }
            if d.offset_0x2c < d.offset_0x14 {
                d.offset_0x14 -= 0.1;
                if d.offset_0x14 < d.offset_0x2c {
                    d.offset_0x14 = d.offset_0x2c;
                }
            } else if d.offset_0x14 < d.offset_0x2c {
                d.offset_0x14 += 0.1;
                if d.offset_0x2c < d.offset_0x14 {
                    d.offset_0x14 = d.offset_0x2c;
                }
            }
        }
        if g.other_pet(this).offset_0x60 != 0 {
            g.other_pet(this).offset_0x14 = 0.0;
        }
    }
    let d = g.other_pet(this);
    d.offset_0x44 += 1;
    d.offset_0x40 += 1;
    if 0x14 < d.offset_0x44 || (d.offset_0x4 <= 10.0 && d.offset_0x2c <= 0.0) || (540.0 <= d.offset_0x4 && 0.0 <= d.offset_0x2c) {
        d.offset_0x44 = 0;
        if rand(g, app) % 10 == 0 {
            let pet = g.other_pet(this).offset_0x70;
            let m = if pet == 7 || pet == 0xe { 9 } else { 3 };
            let r = rand(g, app) % m;
            g.other_pet(this).offset_0x3c = r as i32;
        }
    }
    let d = g.other_pet(this);
    if d.offset_0x58 != 0 {
        d.offset_0x58 -= 1;
    }
    if d.offset_0x54 < 0x14 {
        d.offset_0x54 += 1;
    }
    FUN_004e2a20(g, this);
    let d = g.other_pet(this);
    let pet = d.offset_0x70;
    let right = if pet == 0 || pet == 5 { 550.0 } else if pet == 7 { 560.0 } else { 540.0 };
    if right < d.offset_0x4 {
        d.offset_0x4 = right;
    }
    if d.offset_0x4 < 10.0 {
        d.offset_0x4 = 10.0;
    }
    let floor = match pet {
        1 => 350.0,
        7 => 365.0,
        0xe => 355.0,
        _ => 370.0,
    };
    if floor < d.offset_0xc && (matches!(pet, 1 | 7 | 0xe) || 370.0 < d.offset_0xc) {
        d.offset_0xc = floor;
        d.offset_0x1c = 0.0;
    }
    if d.offset_0xc < 95.0 {
        d.offset_0xc = 95.0;
    }
    if 535.0 < d.offset_0x4 && 0.1 < d.offset_0x14 {
        d.offset_0x3c = 1;
    }
    if d.offset_0x4 < 15.0 && d.offset_0x14 < -0.1 {
        d.offset_0x3c = 2;
    }
    FUN_004eb7c0(g, this);
    let d = g.other_pet(this);
    if !d.offset_0x5c {
        d.offset_0x4 = d.offset_0x14 / d.offset_0x34 + d.offset_0x4;
        d.offset_0xc = d.offset_0x1c / d.offset_0x34 + d.offset_0xc;
    } else {
        let s = d.offset_0x34 * 5.0;
        d.offset_0x4 = d.offset_0x14 / s + d.offset_0x4;
        d.offset_0xc = d.offset_0x1c / s + d.offset_0xc;
    }
    let (x, y) = (d.offset_0x4, d.offset_0xc);
    let (ix, iy) = (ftol(x) as i32, ftol(y) as i32);
    vcall!(g, this, w.vfunction42, ix, iy);
}

/// port: 004e2cc0 FUN_004e2cc0
/// Stinky: the sleep bubble, red when idle long (with Wadsworth), the crawl / turn /
/// shell row.
pub fn FUN_004e2cc0(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: bool) {
    let d = g.other_pet(this).clone();
    let mut mirror = param_2;
    if d.offset_0x5c {
        let img = g.res.DAT_005e8b44;
        FUN_00455d20(gfx, g, img, if param_2 { 0x37 } else { 0x19 }, -0xe);
    }
    if 0x96 < d.offset_0x68 {
        FUN_004558e0(gfx, true);
        FUN_00455890(gfx, CRect(0xff, 0x4b, 0x4b, 0xff));
    }
    let row = if d.offset_0x60 == 0 {
        if d.offset_0x48 != 0 {
            mirror = 0 < d.offset_0x48;
            1
        } else {
            0
        }
    } else {
        2
    };
    let img = g.res.DAT_005e8be0;
    FUN_004560a0(gfx, g, img, 0, 0, &Rect::new(d.offset_0x50 * 0x50, row * 0x50, 0x50, 0x50), mirror);
    FUN_004558e0(gfx, false);
}

/// port: 004e2bd0 FUN_004e2bd0
/// Niko: the breathing cycle, opening, the pearl showing (row 2 with the pearl out) and
/// closing; the (non-Presto) pedestal on backdrop 3.
pub fn FUN_004e2bd0(g: &mut G, this: Ptr, gfx: &mut Graphics, _param_2: bool) {
    let d = g.other_pet(this).clone();
    let t = d.offset_0x60;
    let (col, row) = if t < 0x4c8 {
        ((d.offset_0x4c - 9).abs(), 0)
    } else if t < 0x4d2 {
        ((t - 0x4c8) % 10, 1)
    } else {
        let c = if (0x5a0..=0x5a9).contains(&t) { 9 - (t - 0x5a0) % 10 } else { 9 };
        (c, d.offset_0x6c as i32 + 1)
    };
    let img = g.res.DAT_005e8e44;
    FUN_00455e40(gfx, g, img, 0, 0, &Rect::new(col * 0x50, row * 0x50, 0x50, 0x50));
    let board = board_of(g, this);
    if !d.offset_0x74 && g.board(board).field_0x35c == 3 {
        let img = g.res.DAT_005e8aac;
        FUN_00455d20(gfx, g, img, 4, 0x43);
    }
}

/// port: 004e2db0 FUN_004e2db0
/// Clyde: the sleep bubble, then its pulse cel.
pub fn FUN_004e2db0(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: bool) {
    let d = g.other_pet(this).clone();
    if d.offset_0x5c {
        let img = g.res.DAT_005e8b44;
        FUN_00455d20(gfx, g, img, if param_2 { 0x1e } else { 0x32 }, -0x1b);
    }
    let img = g.res.DAT_005e8bd0;
    FUN_00455e40(gfx, g, img, 0, 0, &Rect::new(d.offset_0x50 * 0x50, 0, 0x50, 0x50));
}

/// port: 004f3590 FUN_004f3590
/// `DrawPet(Graphics*, bool mirror)`: the pet's own drawing (Rufus: idle row when still;
/// Rhubarb: snap row while snapping); Presto's transform glow; the carried missile's
/// sparkle; Presto's name tag with its recharge.
pub fn FUN_004f3590(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: bool) {
    let d = g.other_pet(this).clone();
    match d.offset_0x70 {
        0 => FUN_004e2cc0(g, this, gfx, param_2),
        1 => FUN_004e2bd0(g, this, gfx, param_2),
        5 => FUN_004e2db0(g, this, gfx, param_2),
        7 => {
            let moving = 0.01 < d.offset_0x14 || d.offset_0x14 < -0.01 || d.offset_0x2c != 0.0;
            let y = if moving { 0 } else { 0x50 };
            let img = g.res.DAT_005e8b60;
            FUN_00455e40(gfx, g, img, 0, 0, &Rect::new(d.offset_0x50 * 0x50, y, 0x50, 0x50));
        }
        0xe => {
            let y = if d.offset_0x60 < 1 { 0 } else { 0x50 };
            let img = g.res.DAT_005e8a98;
            FUN_00455e40(gfx, g, img, 0, 0, &Rect::new(d.offset_0x50 * 0x50, y, 0x50, 0x50));
        }
        _ => {}
    }
    if d.offset_0x74 && d.offset_0x54 < 0x14 {
        FUN_004558c0(gfx, 1);
        FUN_004558e0(gfx, true);
        FUN_00455890(gfx, CRect(0xaf, 0xaf, 0xaf, 0xff));
        let img = g.res.DAT_005e8d60;
        FUN_004560a0(gfx, g, img, 0, 0, &Rect::new((d.offset_0x54 / 2) * 0x50, 0xa0, 0x50, 0x50), param_2);
        FUN_004558c0(gfx, 0);
        FUN_004558e0(gfx, false);
    }
    if g.go(this).offset_0x10 != NULL {
        crate::game::game_object::FUN_004d7040(g, this, gfx, 0, 0);
    }
    if d.offset_0x74 && d.offset_0x70 != 0x13 {
        FUN_004f2450(g, this, gfx, d.offset_0x58);
    }
}

/// port: 004f2450 FUN_004f2450
/// Presto's name tag under it, the recharging part in red.
pub fn FUN_004f2450(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: i32) {
    FUN_00455890(gfx, FUN_00433320(0xaaffaa));
    let f = g.res.DAT_005e8cf0;
    FUN_00455880(gfx, f);
    let f = FUN_00455870(gfx);
    let w = font::string_width(g, f, b"Presto");
    let x = (g.wc(this).offset_0x34 - w) / 2;
    let y = g.wc(this).offset_0x38 - 2;
    FUN_00455cf0(gfx, g, b"Presto", x, y);
    if 0 < param_2 {
        let mut g2 = FUN_00455800(gfx);
        FUN_00455890(&mut g2, FUN_00433320(0xff3333));
        let (cw, ch) = (g.wc(this).offset_0x34, g.wc(this).offset_0x38);
        FUN_00456340(&mut g2, (w - (w * param_2) / 0x168) + x, 0, cw, ch);
        FUN_00455cf0(&mut g2, g, b"Presto", x, y);
    }
}

/// port: 004f37c0 Sexy::OtherTypePet::vfunction27
/// `Draw(Graphics*)`: facing by its turn, speed or heading (Stinky; nothing at all when it
/// is perfectly still with no heading); Niko, Clyde and Rufus unmirrored, Rhubarb too.
pub fn vfunction27(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    crate::game::food::FUN_004d6ae0(g, this);
    let d = g.other_pet(this).clone();
    let mirror: Option<bool> = match d.offset_0x70 {
        1 | 5 | 7 | 0xe => Some(false),
        0 => {
            if 0 < d.offset_0x60 {
                Some(0.0 <= d.offset_0x24)
            } else if 0 < d.offset_0x48 {
                Some(true)
            } else if d.offset_0x48 < 0 {
                Some(false)
            } else if d.offset_0x14 < 0.0 {
                Some(false)
            } else {
                let n = ftol(d.offset_0x14) as i32;
                if n == 0 && d.offset_0x24 < 0.0 {
                    Some(false)
                } else if 0.0 < d.offset_0x14 {
                    Some(true)
                } else if n != 0 {
                    None
                } else if 0.0 < d.offset_0x24 {
                    Some(true)
                } else {
                    None
                }
            }
        }
        _ => None,
    };
    if let Some(m) = mirror {
        FUN_004f3590(g, this, gfx, m);
    }
    FUN_004558e0(gfx, false);
}


/// Rhubarb's fling on a creature: its drop-in/damping countdown (+0x208 for the fish
/// kinds, +0x1d8 for breeders).
fn set_flung(g: &mut G, o: Ptr, field: i32, v: i32) {
    match (&g.go_ext(o).sub, field) {
        (GoSub::Fish(_) | GoSub::FishTypePet(..) | GoSub::BiFish(..), 0x208) => g.fish(o).field_0xb4 = v,
        (GoSub::Breeder(_), 0x1d8) => g.breeder(o).offset_0x84 = v,
        _ => unreachable!("Rhubarb fling: +{field:#x} of object type {}", g.go(o).offset_0x4),
    }
}

/// Rhubarb's fling: the creature's y speed (+0x170).
fn set_vy(g: &mut G, o: Ptr, v: f64) {
    match &g.go_ext(o).sub {
        GoSub::Fish(_) | GoSub::FishTypePet(..) | GoSub::BiFish(..) => g.fish(o).field_0x1c = v,
        GoSub::Breeder(_) => g.breeder(o).field_0x1c = v,
        _ => unreachable!("Rhubarb fling: +0x170 of object type {}", g.go(o).offset_0x4),
    }
}
