//! `Sexy::Alien`: the aliens that warp into the tank and eat fish (Sylvester kinds 1/2,
//! Balrog 3, Gus 4 who eats food instead, Destructor 5 and Ulysses 6 who fire missiles
//! from the floor, Psychosquid 7, the small alien 0x14, the final boss 0x15). Lasers push
//! them around and wear down their health; a dead alien leaves a `DeadAlien` and treasure.
//! `Alien_data` starts at object offset 0x154.

use crate::game::board_level::vec_index;
use crate::game::game_object::{FUN_004f22c0, GameObject};
use crate::sexy::crt::ftol;
use crate::sexy::graphics::{FUN_004558c0, FUN_004558e0, FUN_00455890, FUN_00455900, FUN_00455d20, FUN_00455e40, FUN_004560a0, FUN_004561c0};
use crate::sexy::prelude::*;
use crate::sexy::types::CRect;

/// `Alien_data` (object offset 0x154, 180 bytes).
#[derive(Debug, Clone, Default)]
pub struct Alien_data {
    /// +0x158 warp-in countdown (15; drawn growing while 1..9).
    pub offset_0x4: i32,
    /// +0x160 x.
    pub offset_0xc: f64,
    /// +0x168 y.
    pub offset_0x14: f64,
    /// +0x170 x speed.
    pub offset_0x1c: f64,
    /// +0x178 y speed.
    pub offset_0x24: f64,
    /// +0x180 wandering x speed.
    pub offset_0x2c: f64,
    /// +0x188 wandering y speed.
    pub offset_0x34: f64,
    /// +0x190 the target is food (Gus).
    pub field_0x3c: bool,
    /// +0x194 Psychosquid's cycle counter / random.
    pub offset_0x40: i32,
    /// +0x198 Psychosquid's cycle length (400).
    pub offset_0x44: i32,
    /// +0x19c Psychosquid has gone berserk once.
    pub offset_0x48: bool,
    /// +0x1a0 updates before Gus may hunt fish again (100).
    pub offset_0x4c: i32,
    /// +0x1a4 missile counter.
    pub offset_0x50: i32,
    /// +0x1a8 missile period (75).
    pub offset_0x54: i32,
    /// +0x1ac special-animation countdown (firing, transforming).
    pub offset_0x58: i32,
    /// +0x1b0 swim animation counter.
    pub offset_0x5c: i32,
    /// +0x1b4 turn animation (+-10, Ulysses +-20).
    pub offset_0x60: i32,
    /// +0x1b8 speed divisor.
    pub offset_0x64: f64,
    /// +0x1c0 animation cel.
    pub offset_0x6c: i32,
    /// +0x1c8 the direction it faces (sign of x speed).
    pub offset_0x74: f64,
    /// +0x1d0 wandering direction (0..3).
    pub offset_0x7c: i32,
    /// +0x1d4 age.
    pub offset_0x80: i32,
    /// +0x1d8 wandering re-roll counter.
    pub offset_0x84: i32,
    /// +0x1dc..0x1e8 the boss's minion and attack timers.
    pub offset_0x88: i32,
    pub offset_0x8c: i32,
    pub offset_0x90: i32,
    pub offset_0x94: i32,
    /// +0x1ec kind.
    pub offset_0x98: i32,
    /// +0x1f0 health.
    pub offset_0x9c: f64,
    /// +0x1f8 starting health.
    pub offset_0xa4: f64,
    /// +0x200 hit flash countdown.
    pub offset_0xac: i32,
    /// +0x204 Psychosquid is in its healing (red) form.
    pub offset_0xb0: bool,
}

impl G {
    pub fn alien(&mut self, p: Ptr) -> &mut Alien_data {
        match &mut self.go_ext(p).sub {
            GoSub::Alien(d) => d,
            s => panic!("{p} is not an Alien: {s:?}"),
        }
    }
}

fn alloc_alien(g: &mut G, wc: crate::sexy::object::WidgetContainer_data, w: crate::sexy::object::Widget_data,
               go: crate::game::game_object::GameObject_data, d: Alien_data) -> Ptr {
    g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__Alien_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::GameObject(Box::new(GameObjectExt { go, sub: GoSub::Alien(d) })) }),
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

/// port: 004ecf80 Sexy::Alien::Alien
/// `Alien::Alien()` (used before loading from a save).
pub fn Alien__004ecf80(g: &mut G) -> Ptr {
    let (mut wc, w, mut go) = GameObject(g);
    go.offset_0x4 = 0x16;
    wc.offset_0x3d = false;
    alloc_alien(g, wc, w, go, Alien_data::default())
}

/// port: 004ecfa0 Sexy::Alien::Alien
/// `Alien(int x, int y, int kind)`: 160x160 (80x80 for 0x14), swimming left or right at
/// random, with the kind's speed divisor and health (the virtual tank's are random and
/// fire less often; Destructor and Ulysses start on the floor).
pub fn Alien__004ecfa0(g: &mut G, param_1: i32, param_2: i32, param_3: i32) -> Ptr {
    let (mut wc, mut w, mut go) = GameObject(g);
    let app = go.offset_0x0;
    let mut d = Alien_data::default();
    d.offset_0xc = param_1 as f64;
    go.offset_0x4 = 0x16;
    wc.offset_0x3d = false;
    d.offset_0x14 = param_2 as f64;
    wc.offset_0x2c = ftol(d.offset_0xc) as i32;
    wc.offset_0x30 = ftol(d.offset_0x14) as i32;
    d.offset_0x2c = 0.0;
    d.offset_0x34 = 0.0;
    wc.offset_0x34 = 0xa0;
    wc.offset_0x38 = 0xa0;
    if rand(g, app) & 1 == 0 {
        d.offset_0x1c = -3.0;
        d.offset_0x74 = -1.0;
    } else {
        d.offset_0x1c = 3.0;
        d.offset_0x74 = 1.0;
    }
    d.offset_0x24 = 0.0;
    d.offset_0x6c = 1;
    d.offset_0x98 = param_3;
    w.offset_0x1 = false;
    d.offset_0x60 = 0;
    d.offset_0xac = 0;
    d.offset_0x4c = 100;
    d.offset_0x50 = 0;
    d.offset_0x54 = 0x4b;
    d.offset_0x58 = 0;
    d.offset_0x88 = 0;
    d.offset_0x90 = 300;
    d.offset_0x8c = 0;
    d.offset_0x94 = 0x4b;
    d.offset_0xb0 = false;
    let mut hp: Option<f64> = None;
    match param_3 {
        1 => {
            d.offset_0x64 = 2.0;
            hp = Some(50.0);
        }
        2 => {
            d.offset_0x64 = 1.6;
            hp = Some(60.0);
        }
        0x14 => {
            let r = rand(g, app);
            wc.offset_0x34 = 0x50;
            wc.offset_0x38 = 0x50;
            d.offset_0x4c = 0x28;
            d.offset_0xac = 0x1e;
            d.offset_0x64 = (r % 5) as f64 / 10.0 + 0.8;
            hp = Some(1.0);
        }
        3 => {
            d.offset_0x64 = 1.2;
            hp = Some(130.0);
        }
        0x15 => {
            d.offset_0x64 = 2.0;
            let profile = g.wfa(app).offset_0x18c;
            let n = (g.profile(profile).field_0x54 - 1).clamp(0, 0x14);
            hp = Some((5000 - (n * 0x9c4) / 0x14) as f64);
        }
        4 => {
            d.offset_0x64 = 1.6;
            hp = Some(100.0);
        }
        5 | 6 => {
            if param_3 == 5 {
                d.offset_0x64 = 1.2;
                d.offset_0x9c = 150.0;
            } else {
                d.offset_0x64 = 3.5;
                d.offset_0x9c = 220.0;
            }
            if g.wfa(app).offset_0x150 != 5 {
                wc.offset_0x30 = 0x118;
                d.offset_0x14 = 280.0;
            }
        }
        7 => {
            d.offset_0xb0 = false;
            d.offset_0x64 = 0.5;
            d.offset_0x40 = 200;
            hp = Some(260.0);
            d.offset_0x44 = 400;
            d.offset_0x48 = false;
        }
        _ => {}
    }
    if let Some(h) = hp {
        d.offset_0x9c = h;
    }
    if g.wfa(app).offset_0x154 {
        let v = if d.offset_0x98 == 4 {
            crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32 % 0x32 + 0x46
        } else {
            crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32 % 100 + 0x96
        };
        d.offset_0x9c = v as f64;
        d.offset_0x54 = 0x90;
    }
    let r = rand(g, app);
    d.offset_0xa4 = d.offset_0x9c;
    d.offset_0x80 = 0x28;
    d.offset_0x84 = 0x14;
    d.offset_0x5c = 0;
    d.offset_0x4 = 0xf;
    d.offset_0x7c = (r % 10) as i32;
    alloc_alien(g, wc, w, go, d)
}

/// port: 004da830 Sexy::Alien::vfunction81
/// `Sync(DataSync&)`.
pub fn vfunction81(g: &mut G, this: Ptr, sync: &mut crate::sexy::data_sync::DataSync) -> crate::sexy::data_sync::SyncResult {
    crate::game::game_object::vfunction81(g, this, sync)?;
    use crate::sexy::data_sync::{FUN_00503010, FUN_00503070, FUN_005030a0};
    let mut d = g.alien(this).clone();
    FUN_00503010(sync, &mut d.offset_0x4)?;
    FUN_005030a0(sync, &mut d.offset_0xc)?;
    FUN_005030a0(sync, &mut d.offset_0x14)?;
    FUN_005030a0(sync, &mut d.offset_0x1c)?;
    FUN_005030a0(sync, &mut d.offset_0x24)?;
    FUN_005030a0(sync, &mut d.offset_0x2c)?;
    FUN_005030a0(sync, &mut d.offset_0x34)?;
    FUN_00503010(sync, &mut d.offset_0x40)?;
    FUN_00503010(sync, &mut d.offset_0x44)?;
    FUN_00503070(sync, &mut d.offset_0x48)?;
    FUN_00503010(sync, &mut d.offset_0x4c)?;
    FUN_00503010(sync, &mut d.offset_0x50)?;
    FUN_00503010(sync, &mut d.offset_0x54)?;
    FUN_00503010(sync, &mut d.offset_0x58)?;
    FUN_00503010(sync, &mut d.offset_0x5c)?;
    FUN_00503010(sync, &mut d.offset_0x60)?;
    FUN_005030a0(sync, &mut d.offset_0x64)?;
    FUN_00503010(sync, &mut d.offset_0x6c)?;
    FUN_005030a0(sync, &mut d.offset_0x74)?;
    FUN_00503010(sync, &mut d.offset_0x7c)?;
    FUN_00503010(sync, &mut d.offset_0x80)?;
    FUN_00503010(sync, &mut d.offset_0x84)?;
    FUN_00503010(sync, &mut d.offset_0x98)?;
    FUN_005030a0(sync, &mut d.offset_0x9c)?;
    FUN_005030a0(sync, &mut d.offset_0xa4)?;
    FUN_00503010(sync, &mut d.offset_0xac)?;
    FUN_00503070(sync, &mut d.offset_0xb0)?;
    if d.offset_0x98 == 0x15 {
        FUN_00503010(sync, &mut d.offset_0x88)?;
        FUN_00503010(sync, &mut d.offset_0x8c)?;
        FUN_00503010(sync, &mut d.offset_0x90)?;
        FUN_00503010(sync, &mut d.offset_0x94)?;
    }
    *g.alien(this) = d;
    Ok(())
}

/// port: 004d4580 FUN_004d4580
/// Whether there is something to hunt: the boss is up, or anything is alive.
pub fn FUN_004d4580(g: &mut G, this: Ptr) -> bool {
    let board = board_of(g, this);
    if g.board(board).field_0x84 == NULL {
        return crate::game::board_update::FUN_005392a0(g, board);
    }
    true
}

/// port: 00500ff0 FUN_00500ff0
/// A small guppy (size < 2) sheltered by the whale (`DAT_005e89d0`).
pub fn FUN_00500ff0(g: &mut G, param_1: Ptr) -> bool {
    if g.globals.DAT_005e89d0 != 0 && g.go(param_1).offset_0x4 == 0 {
        return g.fish(param_1).offset_0x4c < 2;
    }
    false
}

/// port: 00503580 FUN_00503580
/// Whether aliens may hunt fish: always outside the virtual tank (+0x880); there, only
/// with at least two creatures, or six seconds after the last hunt (`DAT_005e8f18`).
pub fn FUN_00503580(g: &mut G) -> bool {
    let app = g.globals.DAT_005eb6a4;
    if !g.wfa(app).offset_0x154 {
        return true;
    }
    if 0x167 < g.sab(app).field_0x47c - g.globals.DAT_005e8f18 {
        let board = g.wfa(app).offset_0x4;
        let b = g.board(board);
        let n: usize = [0xa0, 0xa4, 0xd4, 0xc0, 0xd0, 0xc4, 0xcc].iter().map(|&o| b.offset_0xc[vec_index(o)].len()).sum();
        if 1 < n {
            return true;
        }
        g.globals.DAT_005e8f18 = g.sab(app).field_0x47c - 500;
    }
    false
}

/// Whether board object `o` may be hunted (`FUN_004e8b60` / `FUN_004e8df0` / `FUN_004e8f70`):
/// alive and not hiding; guppies unless sheltered, the big creatures 5..10 always, pets
/// only while the boss is up.
pub fn huntable(g: &mut G, this: Ptr, o: Ptr) -> bool {
    if g.go(o).offset_0x24 >= 0 || 0 < g.go(o).offset_0x98 {
        return false;
    }
    match g.go(o).offset_0x4 {
        0 => !FUN_00500ff0(g, o),
        5..=10 => true,
        0x14 | 0x15 => {
            let board = board_of(g, this);
            g.board(board).field_0x84 != NULL
        }
        _ => false,
    }
}

pub fn center(g: &mut G, p: Ptr) -> (i32, i32) {
    let wc = g.wc(p);
    (wc.offset_0x34 / 2 + wc.offset_0x2c, wc.offset_0x38 / 2 + wc.offset_0x30)
}

/// port: 004e8b60 FUN_004e8b60
/// `FindTarget()`: Gus first looks for the nearest food (preferring it by 2500); then,
/// when hunting is allowed (not for Gus while full), the nearest huntable creature.
pub fn FUN_004e8b60(g: &mut G, this: Ptr) -> Ptr {
    let (cx, cy) = center(g, this);
    g.alien(this).field_0x3c = false;
    let mut best = 100000000;
    let mut target = NULL;
    let board = board_of(g, this);
    if g.alien(this).offset_0x98 == 4 {
        for f in g.board(board).offset_0xc[vec_index(0xac)].clone() {
            let fd = g.food(f).clone();
            if fd.offset_0x44 < 1 && fd.offset_0x40 != 2 {
                let (fx, fy) = (cx - g.wc(f).offset_0x34 / 2 - g.wc(f).offset_0x2c, cy - g.wc(f).offset_0x38 / 2 - g.wc(f).offset_0x30);
                let d = fy * fy + fx * fx;
                if d < best {
                    best = d - 0x9c4;
                    g.alien(this).field_0x3c = true;
                    target = f;
                }
            }
        }
    }
    if !FUN_00503580(g) || (g.alien(this).offset_0x98 == 4 && 0 < g.alien(this).offset_0x4c) {
        return target;
    }
    let objs: Vec<Ptr> = g.board(board).offset_0x7c.iter().copied().collect();
    for o in objs {
        if huntable(g, this, o) {
            let (ox, oy) = center(g, o);
            let (dx, dy) = (cx - ox, cy - oy);
            let d = dy * dy + dx * dx;
            if d < best {
                target = o;
                best = d;
            }
        }
    }
    target
}

/// port: 004e8df0 FUN_004e8df0
/// `FindMissileTarget()`: the farthest huntable creature without a missile on it.
pub fn FUN_004e8df0(g: &mut G, this: Ptr) -> Ptr {
    let (cx, cy) = center(g, this);
    g.alien(this).field_0x3c = false;
    let board = board_of(g, this);
    let mut best = 0;
    let mut target = NULL;
    let objs: Vec<Ptr> = g.board(board).offset_0x7c.iter().copied().collect();
    for o in objs {
        if g.go(o).offset_0x10 == NULL && huntable(g, this, o) {
            let (ox, oy) = center(g, o);
            let (dx, dy) = (cx - ox, cy - oy);
            let d = dy * dy + dx * dx;
            if best < d {
                target = o;
                best = d;
            }
        }
    }
    target
}

/// port: 004ed460 FUN_004ed460
/// `Chase()`: steers towards the target (Gus at food: towards its corner, faster down),
/// then tries to eat (not while Gus is full). True when there is a target.
pub fn FUN_004ed460(g: &mut G, this: Ptr) -> bool {
    let t = FUN_004e8b60(g, this);
    if t != NULL {
        let (tx, ty) = (g.wc(t).offset_0x2c, g.wc(t).offset_0x30);
        let kind = g.alien(this).offset_0x98;
        let food = g.alien(this).field_0x3c;
        let d = g.alien(this);
        let (ox, oy, dxo, dyo, vy_max, vy_step) = if kind == 4 && food {
            (80.0, 80.0, 0x14, 0x14, 3.0, 0.3)
        } else if kind == 0x14 {
            (40.0, 40.0, 0x28, 0x28, 1.8, 0.1)
        } else {
            (80.0, 80.0, 0x28, 0x28, 1.8, 0.1)
        };
        let a = d.offset_0xc + ox;
        let b = (tx + dxo) as f64;
        if a <= b {
            if a < b && d.offset_0x1c < 1.8 {
                d.offset_0x1c += 0.1;
            }
        } else if -1.8 < d.offset_0x1c {
            d.offset_0x1c -= 0.1;
        }
        let a = d.offset_0x14 + oy;
        let b = (ty + dyo) as f64;
        if a <= b {
            if a < b && d.offset_0x24 < vy_max {
                d.offset_0x24 += vy_step;
            }
        } else if -1.8 < d.offset_0x24 {
            d.offset_0x24 -= 0.1;
        }
    }
    if (g.alien(this).offset_0x4c < 1 || g.alien(this).offset_0x98 == 4) && t != NULL {
        FUN_004e8f70(g, this);
    }
    t != NULL
}

/// port: 004e8f70 FUN_004e8f70
/// `TryEat()`: Gus swallows a food pellet it touches (hurting itself: a star potion 20
/// with a burst, kinds 3/5/4 30/25/20, else 2 x grade + 4; 8 in the virtual tank); then,
/// when hunting is allowed, the first huntable creature within reach is eaten.
pub fn FUN_004e8f70(g: &mut G, this: Ptr) {
    let (cx, cy) = center(g, this);
    let kind = g.alien(this).offset_0x98;
    let ra = if kind != 0x14 { 0x2d } else { 0x1e };
    let rb = if kind != 0x14 { 0x41 } else { 0x1e };
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    if kind == 4 {
        for f in g.board(board).offset_0xc[vec_index(0xac)].clone() {
            let (fx, fy) = (g.wc(f).offset_0x2c, g.wc(f).offset_0x30);
            let d = g.alien(this).clone();
            let ax = d.offset_0xc + 80.0;
            let ay = d.offset_0x14 + 80.0;
            if ax < (fx + 0x41) as f64 && ((fx - 0x19) as f64) < ax && ay < (fy + 0x41) as f64 && ((fy - 0x19) as f64) < ay
                && g.food(f).offset_0x44 == 0
                && d.offset_0xac < 6
            {
                if g.wfa(app).offset_0x154 {
                    g.alien(this).offset_0x9c -= 8.0;
                } else if g.food(f).offset_0x34 == 3 {
                    g.alien(this).offset_0x9c -= 20.0;
                    crate::game::board::FUN_00538230(g, board, 0x11d, 3, 1.0);
                    let mut n = rand(g, app) % 3 + 1;
                    while 0 < n {
                        let a = (rand(g, app) % 3 + 3) as i32;
                        let y = (rand(g, app) % 0x28) as i32 + g.wc(this).offset_0x30 + 10;
                        let x = (rand(g, app) % 0x28) as i32 + g.wc(this).offset_0x2c + 10;
                        crate::game::board_level::FUN_005436f0(g, board, x, y, a);
                        let a = (rand(g, app) % 3 + 3) as i32;
                        let y = (rand(g, app) % 0x14) as i32 + g.wc(this).offset_0x30 + 0x14;
                        let x = (rand(g, app) % 0x14) as i32 + g.wc(this).offset_0x2c + 0x14;
                        crate::game::board_level::FUN_005436f0(g, board, x, y, a);
                        n -= 1;
                    }
                } else {
                    let dmg = match g.food(f).offset_0x40 {
                        3 => 30.0,
                        5 => 25.0,
                        4 => 20.0,
                        _ => (g.food(f).offset_0x34 * 2 + 4) as f64,
                    };
                    g.alien(this).offset_0x9c -= dmg;
                }
                g.alien(this).offset_0xac = 10;
                crate::game::board_level::FUN_005384c0(g, board, false);
                crate::game::larva::vfunction76(g, f);
                break;
            }
        }
    }
    if !FUN_00503580(g) || (g.alien(this).offset_0x98 == 4 && 0 < g.alien(this).offset_0x4c) {
        return;
    }
    let objs: Vec<Ptr> = g.board(board).offset_0x7c.iter().copied().collect();
    for o in objs {
        if !huntable(g, this, o) {
            continue;
        }
        let (a, b) = if g.go(o).offset_0x4 == 6 { (0x46, 0x55) } else { (ra, rb) };
        let (ox, oy) = center(g, o);
        let (dx, dy) = (cx - ox, cy - oy);
        if -a < dx && dx < a && -b < dy && dy < b {
            vcall!(g, this, go.vfunction78, o as i32);
            vcall!(g, o, go.vfunction76);
            return;
        }
    }
}

/// port: 004d4540 Sexy::Alien::vfunction71
/// `CountKill(int stats[])`: Gus counts in slots 5..7 (+3 each), the others in 0, 3, 4.
pub fn vfunction71(g: &mut G, this: Ptr, param_1: &mut [i32]) {
    if g.alien(this).offset_0x98 == 4 {
        param_1[7] += 3;
        param_1[5] += 3;
        param_1[6] += 3;
        return;
    }
    param_1[0] += 1;
    param_1[3] += 1;
    param_1[4] += 1;
}

/// port: 004d45a0 Sexy::Alien::vfunction78
/// `Ate(GameObject* prey)`: remembers the time (virtual tank), the chomp; Gus is full for
/// a while and a little weaker; sparkles where the prey was (with +0x882).
pub fn vfunction78(g: &mut G, this: Ptr, param_1: i32) {
    let prey = param_1 as Ptr;
    let app = g.go(this).offset_0x0;
    g.globals.DAT_005e8f18 = g.sab(app).field_0x47c;
    let board = g.wfa(app).offset_0x4;
    crate::game::board_level::FUN_00538560(g, board, false);
    if g.alien(this).offset_0x98 == 4 {
        g.alien(this).offset_0xac = 10;
        g.alien(this).offset_0x9c -= 15.0;
    }
    if g.wfa(app).offset_0x156 {
        let mut n = rand(g, app) % 3 + 2;
        while n != 0 {
            let off = if g.go(prey).offset_0x4 == 6 { 0x37 } else { 0xf };
            let y = (rand(g, app) % 0x14) as i32 + off + g.wc(prey).offset_0x30;
            let x = (rand(g, app) % 0x14) as i32 + off + g.wc(prey).offset_0x2c;
            let board = g.wfa(app).offset_0x4;
            crate::game::board_level::FUN_005436f0(g, board, x, y, 1);
            n -= 1;
        }
    }
}

/// port: 004fae00 Sexy::Alien::vfunction75
/// `Removed()`: the boss's cleanup, then the aliens-gone check.
pub fn vfunction75(g: &mut G, this: Ptr) {
    FUN_004f9950(g, this);
    let board = board_of(g, this);
    FUN_00539c90(g, board);
}

/// port: 004fd550 Sexy::Alien::vfunction76
/// `Remove()`: dies (with treasure and effects).
pub fn vfunction76(g: &mut G, this: Ptr) {
    FUN_004f9c70(g, this, true);
}

/// port: 004f9950 FUN_004f9950
/// The boss's death: the board forgets it, marks the level won (+0x4f4) and removes every
/// +0xbc, +0xb8 and +0xdc object (the minions and its projectiles).
pub fn FUN_004f9950(g: &mut G, this: Ptr) {
    if g.alien(this).offset_0x98 != 0x15 {
        return;
    }
    let board = board_of(g, this);
    g.board(board).field_0x84 = NULL;
    g.board(board).ext_0x4f4 = true;
    let mut list: Vec<Ptr> = Vec::new();
    for off in [0xbc, 0xb8, 0xdc] {
        for o in g.board(board).offset_0xc[vec_index(off)].clone() {
            crate::game::board_level::FUN_005420e0(&mut list, o);
        }
    }
    for o in list {
        FUN_004d6830(g, o, true);
    }
}

/// port: 004d6830 FUN_004d6830
/// `Destroy(bool deleteIt)`: when the board still had it, also destroys what it carries
/// and its shadow, then `Removed()`, off the widget manager, and deferred deletion.
pub fn FUN_004d6830(g: &mut G, this: Ptr, param_1: bool) {
    let board = board_of(g, this);
    if crate::game::board_level::FUN_00541d90(g, board, this, true) {
        let carried = g.go(this).offset_0x10;
        if carried != NULL {
            FUN_004d6830(g, carried, true);
            g.go(this).offset_0x10 = NULL;
        }
        let shadow = g.go(this).offset_0xc;
        if shadow != NULL {
            FUN_004d6830(g, shadow, true);
            g.go(this).offset_0xc = NULL;
        }
        vcall!(g, this, go.vfunction75);
        let app = g.go(this).offset_0x0;
        let board = g.wfa(app).offset_0x4;
        let mgr = g.wc(board).offset_0xc;
        vcall!(g, mgr, w.vfunction5, this);
        if param_1 {
            crate::sexy::sexy_app_base::vfunction35(g, app, this);
        }
    }
}

/// port: 004d70b0 FUN_004d70b0
/// Whether a dead alien drops treasure: always, except in the virtual tank (only with
/// +0x500 and either a trial ended or few shells).
pub fn FUN_004d70b0(g: &mut G, this: Ptr) -> bool {
    let app = g.go(this).offset_0x0;
    if g.wfa(app).offset_0x150 != 5 {
        return true;
    }
    let board = g.wfa(app).offset_0x4;
    if g.board(board).ext_0x500 {
        if !crate::game::win_fish_app::thunk_FUN_00479fc0(g, app) {
            return true;
        }
        if g.board(board).field_0x374 <= g.globals.DAT_005df590 {
            return true;
        }
    }
    false
}

/// port: 004f9c70 FUN_004f9c70
/// `Die(bool effects)`: drops its missile; a treasure chest (not with the boss up);
/// removed; then, with effects, bubbles, sparkles and the death sounds (Gus and the small
/// alien explode without a body), else a `DeadAlien` where it was.
pub fn FUN_004f9c70(g: &mut G, this: Ptr, param_1: bool) {
    let missile = g.go(this).offset_0x10;
    if missile != NULL {
        crate::game::missle::vfunction76(g, missile);
    }
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    if g.board(board).field_0x84 == NULL && param_1 && FUN_004d70b0(g, this) {
        let (x, y) = (g.wc(this).offset_0x2c + 0x19, g.wc(this).offset_0x30 + 0x19);
        crate::game::board_level::FUN_00544430(g, board, x, y, 4, NULL, -1.0, 0);
    }
    crate::game::dead_fish::vfunction82(g, this);
    let board = g.wfa(app).offset_0x4;
    FUN_00539c90(g, board);
    if param_1 {
        let kind = g.alien(this).offset_0x98;
        let (mx, my) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
        let bubble = |g: &mut G, m: u32, a: i32| {
            let y = (rand(g, app) % m) as i32 + a + my;
            let x = (rand(g, app) % m) as i32 + a + mx;
            let board = g.wfa(app).offset_0x4;
            crate::game::board_update::FUN_00538ac0(g, board, x, y);
        };
        let spark = |g: &mut G, kind_rand: bool, m: u32, a: i32| {
            let k = if kind_rand { (rand(g, app) % 3 + 3) as i32 } else { ((rand(g, app) & 1) + 6) as i32 };
            let y = (rand(g, app) % m) as i32 + a + my;
            let x = (rand(g, app) % m) as i32 + a + mx;
            let board = g.wfa(app).offset_0x4;
            crate::game::board_level::FUN_005436f0(g, board, x, y, k);
        };
        if kind == 4 || kind == 0x14 {
            let big = kind == 4;
            let mut n = (rand(g, app) & 1) + 2;
            while n != 0 {
                if big {
                    bubble(g, 0x46, 0x1e);
                    bubble(g, 0x32, 0x32);
                } else {
                    bubble(g, 0x23, 0);
                    bubble(g, 0x19, 10);
                }
                n -= 1;
            }
            let mut n = rand(g, app) % 3 + 4;
            while n != 0 {
                if big {
                    spark(g, true, 0x32, 0x14);
                    spark(g, true, 0x1e, 0x28);
                } else {
                    spark(g, true, 0x19, 0);
                    spark(g, true, 0xf, 10);
                }
                n -= 1;
            }
            let mut n = (rand(g, app) & 1) + 2;
            while n != 0 {
                if big {
                    spark(g, false, 0x32, 0x14);
                    spark(g, false, 0x1e, 0x28);
                } else {
                    spark(g, false, 0x19, 0);
                    spark(g, false, 0xf, 10);
                }
                n -= 1;
            }
            let board = g.wfa(app).offset_0x4;
            if big {
                crate::game::board::FUN_00538230(g, board, 0x11d, 3, 1.0);
                crate::game::board::FUN_00538230(g, board, 0x11e, 3, 1.0);
            } else {
                crate::game::board::FUN_00538230(g, board, 0x11f, 3, 1.0);
            }
            FUN_004f9950(g, this);
            return;
        }
        let mut n = rand(g, app) & 1;
        while n != 0 {
            bubble(g, 0x46, 0x1e);
            bubble(g, 0x32, 0x32);
            n -= 1;
        }
        let mut n = rand(g, app) % 3;
        while n != 0 {
            spark(g, true, 0x32, 0x14);
            spark(g, true, 0x1e, 0x28);
            n -= 1;
        }
        let mut n = rand(g, app) & 1;
        while n != 0 {
            spark(g, false, 0x32, 0x14);
            spark(g, false, 0x1e, 0x28);
            n -= 1;
        }
        let board = g.wfa(app).offset_0x4;
        crate::game::board::FUN_00538230(g, board, 0x11d, 3, 1.0);
        crate::game::board::FUN_00538230(g, board, 0x11e, 3, 1.0);
        let d = g.alien(this).clone();
        let facing = 0.0 < d.offset_0x1c;
        let (iy, ix) = (ftol(d.offset_0x14) as i32, ftol(d.offset_0xc) as i32);
        let board = g.wfa(app).offset_0x4;
        crate::game::board_level::FUN_005442c0(g, board, ix, iy, d.offset_0x6c, d.offset_0x98, facing);
    }
    FUN_004f9950(g, this);
}

/// port: 004fa6f0 FUN_004fa6f0
/// `Shoot(int x, int y)`: a laser hit inside the 160x160 box (not while flashing): damage
/// by weapon level (Psychosquid's red form heals instead), sparkles and the hit sound; the
/// alien is knocked away from where it was hit (Destructor and Ulysses only sideways).
/// True when that killed it.
pub fn FUN_004fa6f0(g: &mut G, this: Ptr, param_1: i32, param_2: i32) -> bool {
    let fx = param_1 as f64;
    let d = g.alien(this).clone();
    if fx <= d.offset_0xc || d.offset_0xc + 160.0 <= fx {
        return false;
    }
    let fy = param_2 as f64;
    if fy <= d.offset_0x14 || d.offset_0x14 + 160.0 <= fy {
        return false;
    }
    if 0 < d.offset_0xac {
        return false;
    }
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    let weapon = g.board(board).field_0x358;
    let kind = d.offset_0x98;
    if kind == 7 && d.offset_0xb0 {
        if g.board(board).field_0x84 == NULL {
            g.alien(this).offset_0x9c += (weapon * 3) as f64;
            FUN_0053e950_board(g, board, b"Stop shooting! Alien regains health!", true, -1);
        }
    } else {
        let dmg = if kind == 5 || kind == 6 { weapon * 2 + 2 } else { weapon * 3 };
        g.alien(this).offset_0x9c -= dmg as f64;
        crate::game::board_level::FUN_005436f0(g, board, param_1 - 0x28, param_2 - 0x28, 2);
        crate::game::board::FUN_00538230(g, board, 0x126, 3, 1.0);
        if kind == 7 && g.alien(this).offset_0x9c < 100.0 && !g.alien(this).offset_0x48 && !g.wfa(app).offset_0x154 {
            let a = g.alien(this);
            a.offset_0xb0 = true;
            a.offset_0x48 = true;
            a.offset_0x58 = 10;
            let r = rand(g, app) % 100;
            g.alien(this).offset_0x40 = r as i32;
        }
    }
    let d = g.alien(this).clone();
    let (x, y, div) = (d.offset_0xc, d.offset_0x14, d.offset_0x64);
    let a = g.alien(this);
    if kind == 5 || kind == 6 {
        if a.offset_0x58 == 0 {
            if x + 60.0 <= fx {
                if x + 100.0 < fx {
                    a.offset_0x1c = div * -3.0;
                }
            } else {
                a.offset_0x1c = div * 3.0;
            }
        }
    } else {
        let (k1, k2, k3) = if kind == 7 { (3.5, 4.0, 4.0) } else { (2.5, 3.0, 3.0) };
        let x60 = x + 60.0;
        let x100 = x + 100.0;
        if kind != 7 && fx < x60 && fy < y + 60.0 {
            a.offset_0x24 = div * 2.5;
            a.offset_0x1c = div * 2.5;
        } else if kind == 7 && fx < x60 && fy < y + 60.0 {
            a.offset_0x24 = div * 3.5;
            a.offset_0x1c = div * 3.5;
        } else if fx < x60 && fy < y + 100.0 {
            a.offset_0x1c = div * k2;
        } else if fx < x60 {
            a.offset_0x1c = div * k1;
            a.offset_0x24 = div * -k1;
        } else if fx < x100 && fy < y + 60.0 {
            a.offset_0x24 = div * k3;
        } else if x100 < fx && y + 100.0 < fy {
            a.offset_0x1c = div * -k1;
            a.offset_0x24 = div * -k1;
        } else if x100 < fx && fy < y + 60.0 {
            a.offset_0x1c = div * -k1;
            a.offset_0x24 = div * k1;
        } else if x100 < fx {
            a.offset_0x1c = div * -k2;
        } else if y + 100.0 < fy {
            a.offset_0x24 = div * -k3;
        }
    }
    if g.alien(this).offset_0x9c <= 0.0 {
        FUN_004f9c70(g, this, true);
        return true;
    }
    g.alien(this).offset_0xac = 10;
    false
}

fn FUN_0053e950_board(g: &mut G, board: Ptr, s: &[u8], blink: bool, id: i32) {
    crate::game::board_level::FUN_0053e950(g, board, s, blink, id);
}

/// port: 00539c90 FUN_00539c90
/// When the last alien is gone: the hold-to-fire ends, the laser cools down (+0x2c8), the
/// alien music stops (unless the boss is up) and the tank music resumes.
pub fn FUN_00539c90(g: &mut G, this: Ptr) {
    let b = g.board(this);
    if b.offset_0xc[vec_index(0xdc)].is_empty() && b.offset_0xc[vec_index(0xb8)].is_empty() && b.offset_0xc[vec_index(0xf4)].is_empty() {
        let app = g.board(this).field_0x0;
        let board = g.wfa(app).offset_0x4;
        let bd = g.board(board);
        bd.ext_0x4ed = false;
        bd.ext_0x4ec = false;
        bd.field_0x23c = 0x24;
        if g.board(this).field_0x84 == NULL {
            crate::game::win_fish_app::FUN_0054c390(g, app, true);
        }
        crate::game::board_update::FUN_00539ad0(g, this, true);
    }
}

/// port: 004f03d0 FUN_004f03d0
/// `Hunt()`: with prey around (or the boss up, or Gus with food), every kind but the
/// floor-bound ones (and Psychosquid's red form) chases it; true when it did. The boss
/// instead summons minions and attacks on its timers.
pub fn FUN_004f03d0(g: &mut G, this: Ptr) -> bool {
    let board = board_of(g, this);
    let mut chase = FUN_004d4580(g, this);
    if !chase {
        let kind = g.alien(this).offset_0x98;
        if (kind == 4 && !g.board(board).offset_0xc[vec_index(0xac)].is_empty()) || g.board(board).field_0x84 != NULL {
            chase = true;
        }
    }
    let kind = g.alien(this).offset_0x98;
    if chase && kind != 0x15 {
        if kind != 5 && kind != 6 && !g.alien(this).offset_0xb0 {
            if FUN_004e8b60(g, this) == NULL {
                return false;
            }
            FUN_004ed460(g, this);
            return true;
        }
    }
    if kind != 0x15 {
        return false;
    }
    let app = g.go(this).offset_0x0;
    let b = g.board(board);
    if b.offset_0xc[vec_index(0xb8)].is_empty() && b.offset_0xc[vec_index(0xf4)].is_empty() {
        g.alien(this).offset_0x88 += 1;
        if g.alien(this).offset_0x90 <= g.alien(this).offset_0x88 {
            g.alien(this).offset_0x88 = 0;
            let r = rand(g, app) % 6 + 4;
            g.board(board).field_0x230 = r as i32;
            crate::game::board_level::FUN_005475b0(g, board, r as i32, true);
        }
    }
    g.alien(this).offset_0x8c += 1;
    if g.alien(this).offset_0x94 <= g.alien(this).offset_0x8c {
        g.alien(this).offset_0x8c = 0;
        let (x, y) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
        crate::game::board_level::FUN_00545580(g, board, x, y);
    }
    false
}

/// port: 004daa00 FUN_004daa00
/// `Animate()`: starts the turn animation when the swim direction flips, runs the special
/// animation, then picks the cel: the turn, the swim cycle (lunging forward while fast:
/// the burst cycle moves it on its own), Ulysses' fast cycle; carries its missile along.
pub fn FUN_004daa00(g: &mut G, this: Ptr) {
    let d = g.alien(this);
    let kind = d.offset_0x98;
    if 0.0 < d.offset_0x74 && d.offset_0x1c < 0.0 {
        d.offset_0x60 = if kind == 6 { 0x14 } else { 10 };
    } else if d.offset_0x74 < 0.0 && 0.0 < d.offset_0x1c {
        d.offset_0x60 = if kind == 6 { -0x14 } else { -10 };
    }
    let t = d.offset_0x60;
    if 0 < t {
        d.offset_0x60 = t - 1;
    } else if t < 0 {
        d.offset_0x60 = t + 1;
    }
    if 0 < d.offset_0x58 {
        d.offset_0x58 -= 1;
        if d.offset_0x58 == 0 {
            d.offset_0x5c = 0;
        }
    }
    let mut cel: Option<i32> = None;
    if d.offset_0x58 < 1 {
        let turn = d.offset_0x60;
        if turn == 0 {
            if kind != 6 {
                if kind == 0x15 || kind == 5 || kind == 7 || (-1.6 <= d.offset_0x1c && d.offset_0x1c <= 1.6) {
                    d.offset_0x5c += 1;
                    if 0x13 < d.offset_0x5c {
                        d.offset_0x5c = 0;
                    }
                    cel = Some(d.offset_0x5c / 2);
                } else {
                    d.offset_0x5c += 1;
                    let n = d.offset_0x5c;
                    let (sx, sy) = (d.offset_0x1c / d.offset_0x64, d.offset_0x24 / d.offset_0x64);
                    if n < 7 {
                        d.offset_0xc -= sx * 0.5 * 0.5;
                        d.offset_0x14 -= sy * 0.5 * 0.5;
                        cel = Some(n / 2);
                    } else if n < 0xb {
                        d.offset_0xc += sx * 0.5 * 1.5;
                        d.offset_0x14 += sy * 0.5 * 1.5;
                        cel = Some(n / 2);
                    } else if n < 0x28 {
                        d.offset_0x6c = 6;
                        d.offset_0xc += sx * 0.5;
                        d.offset_0x14 += sy * 0.5;
                    } else if 0x32 < n {
                        d.offset_0x6c = 0;
                        d.offset_0x5c = 0;
                    } else {
                        let m = (n - 0x32).abs();
                        d.offset_0xc += sx * 0.5;
                        d.offset_0x14 += sy * 0.5;
                        cel = Some(m / 2);
                    }
                }
            } else {
                let step = if d.offset_0x1c < -8.0 || 8.0 < d.offset_0x1c { 2 } else { 1 };
                let n = (d.offset_0x5c + step) % 0x50;
                d.offset_0x5c = n;
                cel = Some(n / 8);
            }
        } else if kind == 6 {
            if 0 < turn {
                d.offset_0x6c = 9 - turn / 2;
            } else if turn < 0 {
                cel = Some(turn / 2 + 9);
            }
        } else if 0 < turn {
            d.offset_0x6c = 9 - turn;
        } else if turn < 0 {
            cel = Some(turn + 10);
        }
    } else {
        let s = d.offset_0x58;
        match kind {
            5 => cel = Some(s),
            6 => {
                if 0x14 < s {
                    d.offset_0x6c = 9 - (s - 0x14) / 2;
                } else {
                    cel = Some((s - 1) / 2);
                }
            }
            7 => {
                if d.offset_0xb0 {
                    d.offset_0x6c = 10 - s;
                } else {
                    cel = Some(s);
                }
            }
            _ => {}
        }
    }
    let d = g.alien(this);
    if let Some(c) = cel {
        d.offset_0x6c = c;
    }
    if d.offset_0x1c != d.offset_0x74 && d.offset_0x1c != 0.0 && d.offset_0x74 != 0.0 {
        d.offset_0x74 = d.offset_0x1c;
    }
    if g.go(this).offset_0x10 != NULL {
        crate::game::game_object::FUN_004d7020(g, this);
    }
}

/// port: 004ed370 FUN_004ed370
/// The warp hole it arrives through (not for the small alien), on top.
pub fn FUN_004ed370(g: &mut G, this: Ptr) {
    if g.alien(this).offset_0x98 != 0x14 {
        let (x, y) = (g.wc(this).offset_0x2c + 0x1e, g.wc(this).offset_0x30 - 0x28);
        let w = crate::game::warp::Warp__004ecf00(g, x, y);
        let board = board_of(g, this);
        crate::game::board_level::FUN_00542ee0(g, board, w, 1);
        let mgr = g.wc(board).offset_0xc;
        vcall!(g, mgr, w.vfunction4, w);
        vcall!(g, mgr, w.vfunction12, w);
    }
}

/// port: 004db210 Sexy::Alien::vfunction27
/// `Draw(Graphics*)`: facing left or right.
pub fn vfunction27(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    crate::game::food::FUN_004d6ae0(g, this);
    let right = !(g.alien(this).offset_0x1c < 0.0);
    FUN_004dae20(g, this, gfx, right);
}

/// port: 004dae20 FUN_004dae20
/// `DrawBody(Graphics*, bool right)`: the kind's sheet (row 0 swim, 1 turn, 2.. specials;
/// cel `offset_0x6c`), growing in while warping, flashing white when hit (or while
/// Destructor fires); the health bar for the boss, or with Blip.
pub fn FUN_004dae20(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: bool) {
    let d = g.alien(this).clone();
    let mut cel = d.offset_0x6c;
    let mut size = 0xa0;
    let mut row = 0;
    let mut mirror = param_2;
    let kind = d.offset_0x98;
    if d.offset_0x58 != 0 && (kind == 7 || kind == 6) {
        row = (kind == 7) as i32 * 2 + 2;
    } else if d.offset_0x60 != 0 {
        row = 1;
        mirror = 0 < d.offset_0x60;
    }
    let board = board_of(g, this);
    let img = match kind {
        1 | 2 => g.res.DAT_005e8ebc,
        3 => g.res.DAT_005e8cec,
        4 => {
            if 0 < d.offset_0xac && g.board(board).field_0x84 == NULL {
                mirror = param_2;
                row = 2;
                cel = d.offset_0xac;
            }
            g.res.DAT_005e8ae4
        }
        5 => g.res.DAT_005e8ec8,
        6 => g.res.DAT_005e8e58,
        7 => {
            if d.offset_0x58 == 0 && d.offset_0xb0 {
                row = (d.offset_0x60 != 0) as i32 + 2;
            }
            g.res.DAT_005e8c4c
        }
        0x14 => {
            size = 0x50;
            g.res.DAT_005e8c0c
        }
        0x15 => g.res.DAT_005e8cb8,
        _ => return,
    };
    let src = Rect::new(cel * size, row * size, size, size);
    let n = d.offset_0x4;
    if 0 < n {
        if 9 < n {
            return;
        }
        let scale = (n as f64 / 10.0) as f32;
        if scale <= 0.0 {
            return;
        }
        let s = ftol(scale as f64 * size as f64) as i32;
        let dest = Rect::new(s / 2, s / 2, size - s, size - s);
        let app = g.go(this).offset_0x0;
        let fast = !crate::sexy::sexy_app_base::dtor_MemoryImage__00489a20(g, app);
        FUN_00455900(gfx, fast);
        FUN_004561c0(gfx, img, &dest, &src, mirror);
        return;
    }
    FUN_004560a0(gfx, g, img, 0, 0, &src, mirror);
    let flash = if 0 < d.offset_0xac && kind != 0x14 && !(kind == 4 && g.board(board).field_0x84 == NULL) && !d.offset_0xb0 {
        Some(d.offset_0xac * 0x19)
    } else if 0 < d.offset_0x58 && d.offset_0x58 < 10 && kind == 5 {
        let t = if 5 < d.offset_0x58 { 10 - d.offset_0x58 } else { d.offset_0x58 };
        Some((t * 0xff) / 5)
    } else {
        None
    };
    if let Some(a) = flash {
        FUN_004558c0(gfx, 1);
        FUN_004558e0(gfx, true);
        FUN_00455890(gfx, CRect(0xff, 0xff, 0xff, a));
        FUN_004560a0(gfx, g, img, 0, 0, &src, mirror);
        FUN_004558c0(gfx, 0);
        FUN_004558e0(gfx, false);
    }
    if kind == 0x15 || (g.board(board).field_0xb8[13] != 0 && g.board(board).field_0x33c != 5) {
        let bar = g.res.DAT_005e8e3c;
        let bw = g.image(bar).offset_0x20;
        let y = g.wc(this).offset_0x38 - 3;
        let d = g.alien(this).clone();
        let w = ftol(bw as f64 * d.offset_0x9c / d.offset_0xa4) as i32;
        let w = if w < 0 { 0 } else if bw < w { bw } else { w };
        let bh = g.image(bar).offset_0x24;
        FUN_00455e40(gfx, g, bar, 10, y, &Rect::new(0, 0, w, bh));
        let frame = g.res.DAT_005e8eb8;
        FUN_00455d20(gfx, g, frame, 10, y);
    }
}

/// port: 004fd560 Sexy::Alien::vfunction23
/// `Update()` (not while paused): Destructor and Ulysses sink to the floor in the virtual
/// tank; the warp-in; hunting or wandering (and drifting to the wandering speed);
/// Psychosquid's form changes; staying inside the tank; Ulysses' strokes; moving; the
/// floor-bound ones' missiles at the farthest prey; the animation; death at no health,
/// and in the virtual tank a laser blast on aliens that stayed 45 seconds.
pub fn vfunction23(g: &mut G, this: Ptr) {
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    if board == NULL || g.board(board).field_0x8 {
        return;
    }
    FUN_004f22c0(g, this);
    let kind = g.alien(this).offset_0x98;
    if (kind == 5 || kind == 6) && g.wfa(app).offset_0x150 == 5 {
        let d = g.alien(this);
        if 280.0 <= d.offset_0x14 {
            d.offset_0x14 = 280.0;
            d.offset_0x24 = 0.0;
        } else {
            d.offset_0x24 = d.offset_0x64 * 0.4 + d.offset_0x24;
        }
    }
    let n = g.alien(this).offset_0x4;
    let mut jitter = 0;
    if n != 0 {
        if n < 0xb {
            jitter = if 0.0 <= g.alien(this).offset_0x1c { 3 } else { -3 };
        }
        g.alien(this).offset_0x4 = n - 1;
        if 8 < n - 1 {
            return;
        }
    }
    if !FUN_004f03d0(g, this) {
        let d = g.alien(this);
        let kind = d.offset_0x98;
        if kind == 5 || kind == 6 {
            match d.offset_0x7c {
                0 | 2 => d.offset_0x2c = -1.0,
                1 | 3 => d.offset_0x2c = 1.0,
                _ => {}
            }
        } else {
            match d.offset_0x7c {
                0 => d.offset_0x2c = -1.5,
                1 => d.offset_0x2c = 1.5,
                2 => d.offset_0x34 = -1.5,
                3 => d.offset_0x34 = 1.5,
                _ => {}
            }
            if d.offset_0x34 < d.offset_0x24 {
                d.offset_0x24 -= 0.1;
            }
            if d.offset_0x24 < d.offset_0x34 {
                d.offset_0x24 += 0.1;
            }
        }
        if d.offset_0x2c < d.offset_0x1c {
            d.offset_0x1c -= 0.1;
        }
        if d.offset_0x1c < d.offset_0x2c {
            d.offset_0x1c += 0.1;
        }
        d.offset_0x80 += 1;
        if d.offset_0x58 == 0 {
            d.offset_0x84 += 1;
            if 0x14 < d.offset_0x84 {
                d.offset_0x84 = 0;
                if rand(g, app) % 10 == 0 {
                    let r = rand(g, app) & 3;
                    g.alien(this).offset_0x7c = r as i32;
                }
            }
        }
    }
    if g.alien(this).offset_0x98 == 7 {
        g.alien(this).offset_0x40 += 1;
        if g.alien(this).offset_0x44 <= g.alien(this).offset_0x40 && !g.wfa(app).offset_0x154 {
            let red = !g.alien(this).offset_0xb0;
            g.alien(this).offset_0xb0 = red;
            if red {
                g.alien(this).offset_0x48 = true;
            }
            g.alien(this).offset_0x58 = 10;
            let r = rand(g, app) % 100;
            g.alien(this).offset_0x40 = r as i32;
            if !g.alien(this).offset_0xb0 {
                g.alien(this).offset_0x64 = 0.5;
                crate::game::board::FUN_00538230(g, board, 0x135, 3, 1.0);
                crate::game::board_level::FUN_00537f90(g, board, -1);
            } else {
                g.alien(this).offset_0x64 = 2.0;
            }
        }
    }
    let kind = g.alien(this).offset_0x98;
    {
        let d = g.alien(this);
        let top = if kind == 0x14 {
            if 540.0 < d.offset_0xc {
                d.offset_0xc = 540.0;
            }
            if d.offset_0xc < 10.0 {
                d.offset_0xc = 10.0;
            }
            if 370.0 < d.offset_0x14 {
                d.offset_0x14 = 370.0;
            }
            95.0
        } else {
            if 490.0 < d.offset_0xc {
                d.offset_0xc = 490.0;
            }
            if d.offset_0xc < -10.0 {
                d.offset_0xc = -10.0;
            }
            if 290.0 < d.offset_0x14 {
                d.offset_0x14 = 290.0;
            }
            85.0
        };
        if d.offset_0x14 < top {
            d.offset_0x14 = top;
        }
        if kind == 6 {
            if d.offset_0x58 < 1 {
                let m = d.offset_0x6c % 5;
                if 0 < m && m < 3 {
                    if 0.0 < d.offset_0x1c {
                        d.offset_0x1c += 0.55;
                    } else if d.offset_0x1c < 0.0 {
                        d.offset_0x1c -= 0.55;
                    }
                } else if !(m < 4 && 0 < m) {
                    if 0.9 < d.offset_0x1c {
                        d.offset_0x1c -= 0.8;
                    } else if d.offset_0x1c < -0.9 {
                        d.offset_0x1c += 0.8;
                    }
                }
            } else if 0.1 < d.offset_0x1c {
                d.offset_0x1c = 0.1;
            } else if d.offset_0x1c < -0.1 {
                d.offset_0x1c = -0.1;
            }
        }
        d.offset_0xc = d.offset_0x1c / d.offset_0x64 + d.offset_0xc + jitter as f64;
        d.offset_0x14 = d.offset_0x24 / d.offset_0x64 + d.offset_0x14;
        if 0 < d.offset_0xac {
            d.offset_0xac -= 1;
        }
        if 0 < d.offset_0x4c {
            d.offset_0x4c -= 1;
        }
    }
    if kind == 5 && FUN_004d4580(g, this) {
        fire_missiles(g, this, false);
    } else if g.alien(this).offset_0x98 == 6 && FUN_004d4580(g, this) {
        let d = g.alien(this);
        d.offset_0x50 += 1;
        let (c, p) = (d.offset_0x50, d.offset_0x54);
        if c <= p {
            if c == p - 0xf {
                if g.alien(this).offset_0x60 == 0 {
                    if FUN_004e8df0(g, this) == NULL {
                        g.alien(this).offset_0x50 = 0;
                    } else {
                        g.alien(this).offset_0x58 = 0x28;
                    }
                } else {
                    g.alien(this).offset_0x50 = c - 2;
                }
            }
        } else {
            fire_missiles(g, this, true);
        }
    }
    let (x, y) = (g.alien(this).offset_0xc, g.alien(this).offset_0x14);
    let (ix, iy) = (ftol(x) as i32, ftol(y) as i32);
    vcall!(g, this, w.vfunction42, ix, iy);
    FUN_004daa00(g, this);
    if 0.0 < g.alien(this).offset_0x9c {
        if g.wfa(app).offset_0x150 == 5 && 0x10e0 < g.wc(this).offset_0x24 {
            let wc = g.wc(this).clone();
            let board = g.wfa(app).offset_0x4;
            crate::game::board_level::FUN_00543640(g, board, wc.offset_0x34 / 2 - 0x28 + wc.offset_0x2c, wc.offset_0x38 / 2 - 0x28 + wc.offset_0x30);
            crate::game::board::FUN_00538230(g, board, 0x136, 3, 1.0);
            FUN_004d6830(g, this, true);
        }
        return;
    }
    FUN_004f9c70(g, this, true);
}

/// The missile volley of Destructor (`ulysses` false: three from its gun) and Ulysses (two
/// homing ones), each at a target; then the next period (longer in the virtual tank).
fn fire_missiles(g: &mut G, this: Ptr, ulysses: bool) {
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    if !ulysses {
        g.alien(this).offset_0x50 += 1;
        if g.alien(this).offset_0x50 <= g.alien(this).offset_0x54 {
            return;
        }
    }
    let right = 0.0 <= g.alien(this).offset_0x1c;
    let (mx, my) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
    let mut fired = false;
    let vt = g.wfa(app).offset_0x154;
    if !ulysses {
        let (x1, x2, x3) = if right { (0x5a, 0x69, 0x78) } else { (-10, -0x19, -0x28) };
        let t = FUN_004e8df0(g, this);
        if t != NULL {
            fired = true;
            crate::game::board_level::FUN_00544130(g, board, mx + x1, my - 0x23, t, 0);
            crate::game::board::FUN_00538230(g, board, 0x128, 3, 1.0);
            if !vt {
                let t = FUN_004e8df0(g, this);
                if t != NULL {
                    crate::game::board_level::FUN_00544130(g, board, mx + x2, my - 0x1e, t, 0);
                    let t = FUN_004e8df0(g, this);
                    if t != NULL {
                        crate::game::board_level::FUN_00544130(g, board, mx + x3, my - 0x19, t, 0);
                    }
                }
            }
        }
    } else {
        let (x1, x2) = if right { (0x46, 0x4e) } else { (10, 2) };
        let t = FUN_004e8df0(g, this);
        if t != NULL {
            crate::game::board_level::FUN_00544130(g, board, mx + x1, my - 0x18, t, 1);
            crate::game::board::FUN_00538230(g, board, 0x143, 3, 1.0);
            if !vt {
                let t = FUN_004e8df0(g, this);
                if t != NULL {
                    crate::game::board_level::FUN_00544130(g, board, mx + x2, my - 0x10, t, 1);
                }
            }
        }
    }
    g.alien(this).offset_0x50 = 0;
    let p = if !vt { rand(g, app) % 0x32 + 0x96 } else { rand(g, app) % 0x32 + 0x168 };
    g.alien(this).offset_0x54 = p as i32;
    if fired {
        g.alien(this).offset_0x58 = 10;
    }
}

/// port: 004d46f0 FUN_004d46f0
/// Whether the alien's center is within (-35, +75) of the point on both axes.
pub fn FUN_004d46f0(g: &mut G, this: Ptr, param_1: i32, param_2: i32) -> bool {
    let wc = g.wc(this).clone();
    let cx = wc.offset_0x34 / 2 + wc.offset_0x2c;
    let cy = wc.offset_0x38 / 2 + wc.offset_0x30;
    cx < param_1 + 0x4b && param_1 - 0x23 < cx && cy < param_2 + 0x4b && param_2 - 0x23 < cy
}
