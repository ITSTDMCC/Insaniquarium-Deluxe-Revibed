//! `Sexy::FishTypePet`: the swimming pets (a `Fish` plus `FishTypePet_data` at object
//! offset 0x22c; the object is 0x268 bytes), and `Sexy::BoxingGlove`, the punch one of them
//! throws. Pet ids are the board's per-pet counters (`Board +0x144[id]`).
//!
//! The pets' targets live in the board's object vectors: +0xa0 guppies, +0xac food,
//! +0xa8 coins, +0x98 dead fish, +0xb8 aliens, +0xf4 the parts of a multi-part alien
//! (each points at its body through +0x158, whose health is at +0x1d8).

use crate::game::board_level::vec_index;
use crate::game::game_object::{FUN_004d7020, FUN_004d7040, FUN_004f22c0};
use crate::sexy::crt::ftol;
use crate::sexy::graphics::{FUN_00455890, FUN_004558c0, FUN_004558e0, FUN_00455d20, FUN_004560a0, FUN_004563d0};
use crate::sexy::prelude::*;
use crate::sexy::types::CRect;

/// `DAT_005df034` (360): Walter's punch cool-down after five punches.
const DAT_005df034: i32 = 0x168;
/// `DAT_005df588` (2160): the age (updates) after which Nimbus ignores a coin while the
/// trial is over.
const DAT_005df588: i32 = 0x870;

/// `FishTypePet_data` (object offset 0x22c, 60 bytes; field names relative to 0x22c).
#[derive(Debug, Clone, Default)]
pub struct FishTypePet_data {
    /// +0x230 is Presto in disguise.
    pub offset_0x4: bool,
    /// +0x234 pet id.
    pub offset_0x8: i32,
    /// +0x238 Presto's recharge (360 updates).
    pub offset_0xc: i32,
    /// +0x23c a pet timer (Stanley's missile, Brinkley's meal pause).
    pub offset_0x10: i32,
    /// +0x240 a pet counter (Zorf's drops, Brinkley's meals; 1 marks a hatched Gash).
    pub offset_0x14: i32,
    /// +0x244 Meryl's blink.
    pub offset_0x18: bool,
    /// +0x245 asleep (virtual tank).
    pub offset_0x19: bool,
    /// +0x248 Presto's appear sparkle (0..20).
    pub offset_0x1c: i32,
    /// +0x24c Walter's glove (`BoxingGlove*`).
    pub offset_0x20: Ptr,
    /// +0x250 Wadsworth is sheltering the guppies.
    pub offset_0x24: bool,
    /// +0x258 a pulse (-1..1).
    pub offset_0x2c: f64,
    /// +0x260 a pet counter (Amp's charges, Walter's punches, Stanley's glow).
    pub offset_0x34: i32,
    /// +0x264 Walter's cool-down.
    pub offset_0x38: i32,
}

/// `BoxingGlove_data` (object offset 0x154, 20 bytes).
#[derive(Debug, Clone, Default)]
pub struct BoxingGlove_data {
    /// +0x158 the pet throwing it.
    pub offset_0x4: Ptr,
    /// +0x15c updates left (30).
    pub offset_0x8: i32,
    /// +0x160 punching to the right.
    pub offset_0xc: bool,
    /// +0x164 hit sound cool-down.
    pub offset_0x10: i32,
}

impl G {
    pub fn fish_type_pet(&mut self, p: Ptr) -> &mut FishTypePet_data {
        match &mut self.go_ext(p).sub {
            GoSub::FishTypePet(_, d) => d,
            s => panic!("{p} is not a FishTypePet: {s:?}"),
        }
    }

    pub fn boxing_glove(&mut self, p: Ptr) -> &mut BoxingGlove_data {
        match &mut self.go_ext(p).sub {
            GoSub::BoxingGlove(d) => d,
            s => panic!("{p} is not a BoxingGlove: {s:?}"),
        }
    }
}

fn app_of(g: &mut G, this: Ptr) -> Ptr {
    g.go(this).offset_0x0
}

fn board_of(g: &mut G, this: Ptr) -> Ptr {
    let app = app_of(g, this);
    g.wfa(app).offset_0x4
}

fn mode(g: &mut G, this: Ptr) -> i32 {
    let app = app_of(g, this);
    g.wfa(app).offset_0x150
}

/// The app's generator (`FUN_0040aeb0` on `mApp +0x7b0`).
fn rand(g: &mut G, this: Ptr) -> u32 {
    let app = app_of(g, this);
    let r = g.wfa(app).offset_0x84;
    crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(r))
}

fn objs(g: &mut G, board: Ptr, off: usize) -> Vec<Ptr> {
    g.board(board).offset_0xc[vec_index(off)].clone()
}

fn pet(g: &mut G, this: Ptr) -> i32 {
    g.fish_type_pet(this).offset_0x8
}

/// The distance the pets measure to an object: `ftol(sqrt(dx * dx + dy * dy))` of
/// `ftol((obj.mX + k) - (x + 40))` (and y), through `float` as the original stores it.
fn dist(g: &mut G, this: Ptr, o: Ptr, k: i32) -> i32 {
    let f = g.fish(this).clone();
    let (ox, oy) = (g.wc(o).offset_0x2c, g.wc(o).offset_0x30);
    let dx = ftol((ox + k) as f64 - (f.field_0x4 + 40.0)) as i32;
    let dy = ftol((oy + k) as f64 - (f.field_0xc + 40.0)) as i32;
    let s = ((dx as f64) * (dx as f64) + dy.wrapping_mul(dy) as f64) as f32;
    let r = (s as f64).sqrt() as f32;
    ftol(r as f64) as i32
}

/// `|x + 40 - (obj.mX + k)|` and the same for y.
fn gap(g: &mut G, this: Ptr, o: Ptr, k: i32) -> (f64, f64) {
    let f = g.fish(this).clone();
    let (ox, oy) = (g.wc(o).offset_0x2c, g.wc(o).offset_0x30);
    (((f.field_0x4 + 40.0) - (ox + k) as f64).abs(), ((f.field_0xc + 40.0) - (oy + k) as f64).abs())
}

/// port: 004e3da0 FUN_004e3da0
/// `std::vector::empty()` (an STL instance).
pub fn FUN_004e3da0(this: &[Ptr]) -> bool {
    this.is_empty()
}

/// port: 004e5040 FUN_004e5040
/// `std::vector::begin()` (an STL instance; an index here).
pub fn FUN_004e5040(_this: &[Ptr]) -> usize {
    0
}

/// port: 004e5010 FUN_004e5010
/// `std::vector::end()` (an STL instance; an index here).
pub fn FUN_004e5010(this: &[Ptr]) -> usize {
    this.len()
}

/// port: 004e3dd0 FUN_004e3dd0
/// `iterator != iterator` (an STL instance).
pub fn FUN_004e3dd0(this: usize, param_1: usize) -> bool {
    this != param_1
}

/// port: 004e3e00 FUN_004e3e00
/// `*iterator` (an STL instance).
pub fn FUN_004e3e00(v: &[Ptr], this: usize) -> Ptr {
    v[this]
}

/// port: 004e5070 FUN_004e5070
/// `iterator++` (an STL instance): the old position.
pub fn FUN_004e5070(this: &mut usize) -> usize {
    let old = *this;
    *this += 1;
    old
}

/// port: 004ef3e0 Sexy::FishTypePet::FishTypePet
/// `FishTypePet::FishTypePet()` (used before loading from a save).
pub fn FishTypePet__004ef3e0(g: &mut G) -> Ptr {
    let this = crate::game::fish::Fish__004eee60(g);
    become_pet(g, this, FishTypePet_data::default());
    g.wc(this).offset_0x3d = false;
    g.go(this).offset_0x4 = 0x15;
    this
}

/// The derived part of construction: the fish gets the pet's data and vftable.
fn become_pet(g: &mut G, this: Ptr, d: FishTypePet_data) {
    let e = g.go_ext(this);
    let sub = std::mem::replace(&mut e.sub, GoSub::None);
    e.sub = match sub {
        GoSub::Fish(f) => GoSub::FishTypePet(f, d),
        s => panic!("{this} is not a Fish: {s:?}"),
    };
    g.set_vt(this, &crate::sexy::vtables_gen::Sexy__FishTypePet_vftable);
}

/// port: 004ef420 Sexy::FishTypePet::FishTypePet
/// `FishTypePet(int x, int y, int pet, bool presto)`: Presto recharges first (not in the
/// virtual tank); each pet's speed, floor, ceiling and timers.
pub fn FishTypePet__004ef420(g: &mut G, param_1: i32, param_2: i32, param_3: i32, param_4: bool) -> Ptr {
    let this = crate::game::fish::Fish__004eef00(g, param_1, param_2);
    become_pet(g, this, FishTypePet_data::default());
    g.wc(this).offset_0x3d = false;
    let app = app_of(g, this);
    let m = g.wfa(app).offset_0x150;
    {
        let d = g.fish_type_pet(this);
        d.offset_0x19 = false;
        d.offset_0x8 = param_3;
        d.offset_0x4 = param_4;
        d.offset_0x1c = 0;
        d.offset_0xc = if !param_4 || param_3 == 0x13 { 0 } else { 0x168 };
    }
    g.go(this).offset_0x4 = 0x15;
    if m == 5 {
        g.fish_type_pet(this).offset_0xc = 0;
    }
    {
        let d = g.fish_type_pet(this);
        d.offset_0x10 = 0;
        d.offset_0x2c = 0.0;
        d.offset_0x14 = 0;
        d.offset_0x38 = 0;
    }
    g.fish(this).field_0xd5 = false;
    g.fish_type_pet(this).offset_0x24 = false;
    g.fish(this).field_0x74 = 0;
    {
        let d = g.fish_type_pet(this);
        d.offset_0x34 = 0;
        d.offset_0x20 = NULL;
        d.offset_0x18 = true;
    }
    let not_virtual = m != 5;
    g.w(this).offset_0x1 = g.globals.DAT_005e8f14;
    match param_3 {
        0x17 => {
            let w = g.w(this);
            w.offset_0x1 = true;
            w.offset_0x28 = true;
            w.field_0x18[1] = 10;
            w.field_0x18[3] = 0xf;
        }
        3 => {
            let f = g.fish(this);
            f.field_0xd0 = 0x3a2;
            f.field_0x3c = 0x168;
        }
        4 => {
            let f = g.fish(this);
            f.field_0xd0 = 0x41;
            f.field_0x2c = 3.0;
            f.field_0x3c = 0x10e;
        }
        8 => g.fish(this).field_0xd0 = if m != 5 { 0x578 } else { 0x10e0 },
        9 => {
            let f = g.fish(this);
            f.field_0xd0 = 0x78;
            f.field_0x2c = 4.0;
        }
        6 => {
            let v = if m == 5 { (crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32) % 200 + 0x438 } else { 0xd8 };
            g.fish(this).field_0xd0 = v;
        }
        _ => {}
    }
    let p = pet(g, this);
    if p == 0x16 {
        g.fish(this).field_0xd0 = 100;
    }
    if p == 0x15 {
        let v = if m == 5 {
            (crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32) % 200 + 0x438
        } else {
            (rand(g, this) % 300 + 300) as i32
        };
        g.fish(this).field_0xd0 = v;
    }
    if pet(g, this) == 0xb {
        let v = if m == 5 {
            (crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32) % 200 + 0x438
        } else {
            (rand(g, this) % 0x14 + 0x279) as i32
        };
        g.fish(this).field_0xd0 = v;
    }
    let p = pet(g, this);
    if p == 0xf {
        let f = g.fish(this);
        f.field_0x2c = 0.5;
        f.field_0x44 = 0x140;
    }
    if p == 0x10 {
        let f = g.fish(this);
        f.field_0x2c = 0.5;
        f.field_0xd0 = 3000;
        g.wc(this).offset_0x34 = 0xa0;
        g.wc(this).offset_0x38 = 0x50;
        let f = g.fish(this);
        f.field_0x48 = 0x1cc;
        f.field_0xcc = 300;
        let w = g.w(this);
        w.field_0x18[1] = 0;
        w.field_0x18[3] = 0x19;
    }
    if p == 0x11 {
        g.fish(this).field_0xd0 = 0x622;
        let presto = g.fish_type_pet(this).offset_0x4;
        g.fish(this).field_0xcc = if not_virtual { -0x60e } else { 0 };
        if presto {
            g.fish(this).field_0xcc = 0x5f0;
        }
    }
    this
}

/// port: 004ef750 Sexy::FishTypePet::~FishTypePet
/// Ends what the pet was doing for the tank (Wadsworth's shelter, Meryl's song) and
/// deletes Walter's glove.
pub fn dtor_FishTypePet(g: &mut G, this: Ptr) {
    g.set_vt(this, &crate::sexy::vtables_gen::Sexy__FishTypePet_vftable);
    FUN_004d6250(g, this);
    let glove = g.fish_type_pet(this).offset_0x20;
    if glove != NULL {
        vcall!(g, glove, w.vfunction1, 1);
    }
    crate::game::fish::dtor_Fish(g, this);
}

/// port: 004f0ce0 Sexy::FishTypePet::deleting_destructor
pub fn deleting_destructor(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_FishTypePet(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 004d6250 FUN_004d6250
/// When the pet goes: Wadsworth stops sheltering (one shelter fewer, `DAT_005e89d0`),
/// Meryl's song ends (`DAT_005e89cc`).
pub fn FUN_004d6250(g: &mut G, this: Ptr) {
    if pet(g, this) == 9 && g.fish_type_pet(this).offset_0x24 {
        g.fish_type_pet(this).offset_0x24 = false;
        g.globals.DAT_005e89d0 -= 1;
        if g.globals.DAT_005e89d0 < 0 {
            g.globals.DAT_005e89d0 = 0;
        }
    }
    if pet(g, this) == 8 {
        g.globals.DAT_005e89cc = false;
    }
}

/// port: 004df180 Sexy::FishTypePet::vfunction81
/// `Sync(DataSync&)`: the fish, then the pet's fields.
pub fn vfunction81(g: &mut G, this: Ptr, sync: &mut crate::sexy::data_sync::DataSync) -> crate::sexy::data_sync::SyncResult {
    crate::game::fish::vfunction81(g, this, sync)?;
    use crate::sexy::data_sync::{FUN_00503010, FUN_00503070, FUN_005030a0};
    let mut d = g.fish_type_pet(this).clone();
    FUN_00503070(sync, &mut d.offset_0x4)?;
    FUN_00503010(sync, &mut d.offset_0x8)?;
    FUN_00503010(sync, &mut d.offset_0x10)?;
    FUN_00503010(sync, &mut d.offset_0x14)?;
    FUN_00503070(sync, &mut d.offset_0x18)?;
    FUN_00503070(sync, &mut d.offset_0x24)?;
    FUN_005030a0(sync, &mut d.offset_0x2c)?;
    FUN_00503010(sync, &mut d.offset_0x1c)?;
    FUN_00503010(sync, &mut d.offset_0xc)?;
    FUN_00503010(sync, &mut d.offset_0x34)?;
    FUN_00503010(sync, &mut d.offset_0x38)?;
    *g.fish_type_pet(this) = d;
    Ok(())
}

/// port: 004d6230 Sexy::FishTypePet::vfunction71
/// Counts Amp and Gash (pets 0x10 and 0x11) in `stats[0]`.
pub fn vfunction71(g: &mut G, this: Ptr, stats: &mut [i32]) {
    let p = pet(g, this);
    if p == 0x11 || p == 0x10 {
        stats[0] += 1;
    }
}

/// port: 004f89a0 Sexy::FishTypePet::vfunction74
/// Presto turning into pet `param_1` (when charged): the new pet in its place, this one
/// leaves; Walter's cool-down carries over between Walter and Presto.
pub fn vfunction74(g: &mut G, this: Ptr, param_1: i32) {
    if param_1 == pet(g, this) {
        return;
    }
    let charge = g.fish_type_pet(this).offset_0xc;
    if !crate::game::other_pet::FUN_004f2650(g, this, charge) {
        return;
    }
    let board = board_of(g, this);
    crate::game::board::FUN_00538230(g, board, 0x122, 3, 1.0);
    let (mx, my) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
    let p = crate::game::board_level::FUN_00544a90(g, board, param_1, mx, my, true, false);
    let app = app_of(g, this);
    let board = g.wfa(app).offset_0x4;
    let mgr = g.wc(board).offset_0xc;
    vcall!(g, mgr, w.vfunction5, this);
    crate::sexy::sexy_app_base::vfunction35(g, app, this);
    crate::game::board_level::FUN_00541d90(g, board, this, false);
    let shadow = g.go(this).offset_0xc;
    if shadow != NULL {
        crate::game::shadow::vfunction76(g, shadow);
    }
    let id = pet(g, this);
    if (id == 0x17 && param_1 == 0x13) || (id == 0x13 && param_1 == 0x17) {
        let d = g.fish_type_pet(this).clone();
        let n = g.fish_type_pet(p);
        n.offset_0x38 = d.offset_0x38;
        n.offset_0x34 = d.offset_0x34;
    }
    FUN_004d6250(g, this);
}

/// port: 004fbe20 Sexy::FishTypePet::vfunction55
/// `MouseDown(x, y, clicks)`: unless the pet handled it, the click goes to the board
/// (guarded against re-entry by `DAT_005e89f9`).
pub fn vfunction55(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32) {
    if !FUN_004fb680(g, this, param_1, param_2, param_3) && !g.globals.DAT_005e89f9 {
        g.globals.DAT_005e89f9 = true;
        let board = board_of(g, this);
        let (mx, my) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
        vcall!(g, board, w.vfunction55, mx + param_1, my + param_2, param_3);
        g.globals.DAT_005e89f9 = false;
    }
}

/// port: 004fb5b0 FUN_004fb5b0
/// A bite on an alien: Itchy takes 1 (0.25 off the two big bosses), Gash 3 (0.5), and
/// Gash's bite can kill.
pub fn FUN_004fb5b0(g: &mut G, this: Ptr, param_1: Ptr) {
    let p = pet(g, this);
    let board = board_of(g, this);
    if p == 2 {
        let a = g.alien(param_1);
        a.offset_0x9c -= if a.offset_0x98 == 5 || a.offset_0x98 == 4 { 0.25 } else { 1.0 };
        crate::game::board_level::FUN_00538340(g, board, 0xb);
        return;
    }
    if p == 0x11 {
        let a = g.alien(param_1);
        a.offset_0x9c -= if a.offset_0x98 == 5 || a.offset_0x98 == 4 { 0.5 } else { 3.0 };
        if g.alien(param_1).offset_0x9c <= 0.0 {
            crate::game::alien::FUN_004f9c70(g, param_1, true);
        }
        crate::game::board_level::FUN_00538340(g, board, 9);
    }
}

/// port: 004dfc50 Sexy::FishTypePet::vfunction90
/// `Animate()`: turning (Prego about to give birth and Walter while punching stop instead),
/// the turn and eat counters, each pet's cel cycle (and the pulses of Gumbo, Shrapnel,
/// Angie and Amp), Meryl's blink, the facing, carrying.
pub fn vfunction90(g: &mut G, this: Ptr) {
    let p = pet(g, this);
    let glove = g.fish_type_pet(this).offset_0x20;
    {
        let f = g.fish(this);
        let stop = (p == 3 && f.field_0xd0 - 0xdc < f.field_0xcc) || (p == 0x17 && glove != NULL);
        if f.offset_0x34 <= 0.0 || 0.0 <= f.offset_0x14 {
            if f.offset_0x34 < 0.0 && 0.0 < f.offset_0x14 {
                if stop {
                    f.offset_0x14 = 0.0;
                } else {
                    f.field_0x70 = -0x14;
                }
            }
        } else if stop {
            f.offset_0x14 = 0.0;
        } else {
            f.field_0x70 = 0x14;
        }
        let v = f.field_0x70;
        if v != 0 {
            let v = if 0 < v { v - 1 } else { v + 1 };
            f.field_0x70 = v;
            if v == 0 {
                f.field_0x74 = 0;
            }
        }
    }
    let turn = g.fish(this).field_0x70;
    if turn != 0 {
        let f = g.fish(this);
        if 0 < turn {
            f.field_0x6c = 9 - turn / 2;
        } else {
            f.field_0x6c = turn / 2 + 9;
        }
    } else {
        let aliens = {
            let board = board_of(g, this);
            crate::game::board_update::FUN_004da780(g, board)
        };
        let asleep = g.fish_type_pet(this).offset_0x19;
        let glow = g.fish_type_pet(this).offset_0x34;
        let f = g.fish(this);
        let step = if 1 < f.field_0x68 { 2 } else { 1 };
        let default = |f: &mut crate::game::fish::Fish_data| {
            f.field_0x64 += if 1 < f.field_0x68 { 2 } else { 1 };
            if 0x13 < f.field_0x64 {
                f.field_0x64 = 0;
            }
            f.field_0x6c = f.field_0x64 / 2;
        };
        // `Some(d)`: the pulse goes up by `d`, wrapping to -1 at 1.
        let mut pulse: Option<f64> = None;
        match p {
            6 => {
                f.field_0x64 += step;
                if 0x13 < f.field_0x64 {
                    f.field_0x64 = 0;
                }
                let v = f.field_0x64;
                f.field_0x6c = if v < 10 { v } else { (v - 0x13).abs() };
            }
            0x17 => {
                let t = f.field_0x74;
                if t < 1 {
                    f.field_0x64 += if f.field_0x68 < 2 { 1 } else { 2 };
                    if 0x27 < f.field_0x64 {
                        f.field_0x64 = 0;
                    }
                    let v = f.field_0x64;
                    f.field_0x6c = if v < 0x14 { v / 2 } else { (v / 2 - 0x13).abs() };
                } else {
                    let v = t - 1;
                    f.field_0x74 = v;
                    f.field_0x6c = if v < 0x1e {
                        if v < 10 { v / 2 } else { 4 }
                    } else {
                        9 - (t - 0x15) / 2
                    };
                }
            }
            0xf | 0x14 => {
                f.field_0x64 += if f.field_0x68 < 2 || asleep { 1 } else { 2 };
                if 0x27 < f.field_0x64 {
                    f.field_0x64 = 0;
                }
                f.field_0x6c = f.field_0x64 / 4;
            }
            0x16 => {
                if 0 < glow && glow < 10 {
                    f.field_0x6c = glow;
                } else if f.field_0x74 < 1 {
                    f.field_0x64 += if f.field_0x68 < 2 { 1 } else { 2 };
                    if 0x27 < f.field_0x64 {
                        f.field_0x64 = 0;
                    }
                    f.field_0x6c = f.field_0x64 / 4;
                } else {
                    f.field_0x74 -= 1;
                    f.field_0x6c = f.field_0x74;
                }
            }
            0x11 => {
                if f.field_0x74 < 1 {
                    default(f);
                } else {
                    f.field_0x74 -= 1;
                    f.field_0x6c = f.field_0x74;
                }
            }
            0xc => {
                f.field_0x64 += step;
                if 0x13 < f.field_0x64 {
                    f.field_0x64 = 0;
                }
                f.field_0x6c = f.field_0x64 / 2;
                if aliens {
                    pulse = Some(0.1);
                }
            }
            0xb => {
                f.field_0x64 += if f.field_0x68 < 3 { 1 } else { 2 };
                if 0x3b < f.field_0x64 {
                    f.field_0x64 = 0;
                }
                f.field_0x6c = f.field_0x64 / 6;
                pulse = Some(0.1);
            }
            0x12 => {
                f.field_0x64 += step;
                if 0x13 < f.field_0x64 {
                    f.field_0x64 = 0;
                }
                f.field_0x6c = f.field_0x64 / 2;
                pulse = Some(0.01);
            }
            10 => {
                f.field_0x64 += step;
                if 0x4f < f.field_0x64 {
                    f.field_0x64 = 0;
                }
                f.field_0x6c = f.field_0x64 / 8;
            }
            4 => {
                f.field_0x64 += 1;
                if 0x13 < f.field_0x64 {
                    f.field_0x64 = 0;
                }
                f.field_0x6c = f.field_0x64 / 2;
                if f.field_0xcc == -1 {
                    f.field_0x64 = 10;
                }
            }
            0x10 => {
                f.field_0x64 += 1;
                if 0x13 < f.field_0x64 {
                    f.field_0x64 = 0;
                }
                f.field_0x6c = f.field_0x64 / 2;
                pulse = Some(if f.field_0xcc < f.field_0xd0 { 0.5 } else { 0.1 });
            }
            _ => default(f),
        }
        if let Some(d) = pulse {
            let t = g.fish_type_pet(this);
            t.offset_0x2c += d;
            if 1.0 <= t.offset_0x2c {
                t.offset_0x2c = -1.0;
            }
        }
        if p == 8 && g.fish(this).field_0x6c == 0 {
            if !g.fish_type_pet(this).offset_0x18 {
                let r = rand(g, this);
                g.fish_type_pet(this).offset_0x18 = r % 10 == 0;
            } else {
                g.fish_type_pet(this).offset_0x18 = false;
            }
        }
    }
    {
        let f = g.fish(this);
        if f.offset_0x14 != f.offset_0x34 && f.offset_0x14 != 0.0 && f.offset_0x34 != 0.0 {
            f.offset_0x34 = f.offset_0x14;
        }
    }
    if g.go(this).offset_0x10 == NULL {
        return;
    }
    FUN_004d7020(g, this);
}

/// The steering step `vfunction85` uses: at or left of `want` up by `d` (below `lim`),
/// right of it down by `d` (above `-lim`).
fn toward(v: &mut f64, at: f64, want: f64, d: f64, lim: f64) {
    if at <= want {
        if at < want && *v < lim {
            *v += d;
        }
    } else if -lim < *v {
        *v -= d;
    }
}

/// The vertical steering of Angie, Nimbus and Brinkley: above `hi` down by 0.6 (to -2),
/// below `lo` up by 1 (to 3), else up by 0.5 below `mid` / down by 0.3 above it.
fn vertical(v: &mut f64, y: f64, hi: i32, lo: i32, mid: i32) {
    if y <= hi as f64 {
        if y < lo as f64 {
            if *v < 3.0 {
                *v += 1.0;
            }
        } else if y <= mid as f64 {
            if y < mid as f64 && *v < 3.0 {
                *v += 0.5;
            }
        } else if -2.0 < *v {
            *v -= 0.3;
        }
    } else if -2.0 < *v {
        *v -= 0.6;
    }
}

/// The fine horizontal steering of Angie and Brinkley: outside `c - out..c + out` by 1,
/// outside `c - mid..c + mid` by 0.1, else by 0.05 toward `c` (speeds within -3..3).
fn fine(v: &mut f64, x: f64, lo_out: i32, hi_out: i32, lo_mid: i32, hi_mid: i32, c: i32) {
    if x <= hi_out as f64 {
        if lo_out as f64 <= x {
            if x <= hi_mid as f64 {
                if x < lo_mid as f64 {
                    if *v < 3.0 {
                        *v += 0.1;
                    }
                } else if x <= c as f64 {
                    if x < c as f64 && *v < 3.0 {
                        *v += 0.05;
                    }
                } else if -3.0 < *v {
                    *v -= 0.05;
                }
            } else if -3.0 < *v {
                *v -= 0.1;
            }
        } else if *v < 3.0 {
            *v += 1.0;
        }
    } else if -3.0 < *v {
        *v -= 1.0;
    }
}

/// port: 004e02d0 Sexy::FishTypePet::vfunction85
/// `Chase()`: steers toward the target (every 5 updates; Nimbus slows to 1.8 without
/// one), each pet with its own reach and speeds, then tries to eat it. True when there
/// is a target.
pub fn vfunction85(g: &mut G, this: Ptr) -> bool {
    let t = vcall!(g, this, fish.vfunction86);
    let p = pet(g, this);
    if g.fish(this).field_0x5c < 5 || t == NULL {
        if p != 0xf {
            if t == NULL {
                return false;
            }
            vcall!(g, this, fish.vfunction87);
            return true;
        }
        if t == NULL {
            g.fish(this).field_0x2c = 1.8;
            return false;
        }
    } else {
        g.fish(this).field_0x5c = 0;
        let (tx, ty) = (g.wc(t).offset_0x2c, g.wc(t).offset_0x30);
        let reach = if g.go(t).offset_0x4 != 0x17 { 0x50 } else { 0x28 };
        let aliens = if p == 0x11 {
            let board = board_of(g, this);
            crate::game::board_update::FUN_004da780(g, board)
        } else {
            false
        };
        let dead_size = if p == 0x12 { g.dead_fish(t).offset_0x50 } else { 0 };
        let f = g.fish(this);
        let (x, y) = (f.field_0x4 + 40.0, f.field_0xc + 40.0);
        let mut grow = false;
        if p == 2 {
            toward(&mut f.offset_0x14, x, (tx + reach) as f64, 2.5, 10.0);
            toward(&mut f.field_0x1c, y, (ty + reach) as f64, 1.5, 4.0);
            if f.field_0x68 < 5 {
                f.field_0x68 += 1;
            }
        }
        if p == 0x11 {
            let r = if aliens { reach } else { 0x28 };
            toward(&mut f.offset_0x14, x, (tx + r) as f64, 2.5, if aliens { 9.0 } else { 8.0 });
            toward(&mut f.field_0x1c, y, (ty + r) as f64, 1.5, 4.0);
            grow = true;
        } else if p == 0xc {
            let yy = ty + reach;
            if yy < 0x105 || f.field_0x1c <= -8.0 {
                if yy < 300 && f.field_0x1c < 8.0 {
                    f.field_0x1c += 2.0;
                }
            } else {
                f.field_0x1c -= 2.0;
            }
            let xx = reach + tx;
            if xx < 0x123 || f.offset_0x14 <= -8.0 {
                if xx < 0x14a && f.offset_0x14 < 8.0 {
                    f.offset_0x14 += 2.0;
                }
            } else {
                f.offset_0x14 -= 2.0;
            }
        } else if p == 0x12 {
            let k = if dead_size == 6 { 0x50 } else { 0x28 };
            let c = tx + k;
            fine(&mut f.offset_0x14, x, c - 4, c + 4, c - 2, c + 2, c);
            let c = ty + k;
            vertical(&mut f.field_0x1c, y, c + 3, c - 3, c);
            grow = true;
        } else if p == 0xf {
            let c = tx;
            let v = &mut f.offset_0x14;
            if x <= (c + 0x36) as f64 {
                if ((c + 0x12) as f64) <= x {
                    if x <= (c + 0x2c) as f64 {
                        if ((c + 0x1c) as f64) <= x {
                            if x <= (c + 0x24) as f64 {
                                if x < (c + 0x24) as f64 && *v < 0.3 {
                                    *v = 0.0;
                                }
                            } else if -0.3 < *v {
                                *v = 0.0;
                            }
                        } else if *v < 1.3 {
                            *v += 0.1;
                        }
                    } else if -1.3 < *v {
                        *v -= 0.1;
                    }
                } else if *v < 2.3 {
                    *v += 0.5;
                }
            } else if -2.3 < *v {
                *v -= 0.5;
            }
            vertical(&mut f.field_0x1c, y, ty + 0x2a, ty + 0x1e, ty + 0x24);
        } else if p == 0x14 {
            let c = tx;
            fine(&mut f.offset_0x14, x, c + 0xc, c + 0x1c, c + 0x10, c + 0x18, c + 0x14);
            vertical(&mut f.field_0x1c, y, ty + 0x1a, ty + 0xe, ty + 0x14);
            grow = true;
        }
        if grow && f.field_0x68 < 5 {
            f.field_0x68 += 1;
        }
    }
    vcall!(g, this, fish.vfunction87);
    t != NULL
}

/// port: 004e5900 Sexy::FishTypePet::vfunction82
/// `PetTimer()` (not while the level-end hold runs or paused): Prego gives birth, Vert
/// drops gold, Nostradamus feeds, Shrapnel drops a bomb, Zorf feeds the hungry, Meryl
/// sings (`DAT_005e89cc`), Gash counts, Amp charges, Stanley shoots at aliens.
pub fn vfunction82(g: &mut G, this: Ptr) {
    let board = board_of(g, this);
    if g.board(board).field_0x33c == 5 {
        return;
    }
    if g.board(board).field_0x8 {
        return;
    }
    let p = pet(g, this);
    let (mx, my) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
    match p {
        3 => {
            g.fish(this).field_0xcc += 1;
            let f = g.fish(this).clone();
            if mode(g, this) == 5 && f.field_0xcc == f.field_0xd0 - 0xd2 {
                // Virtual tank: with more guppies out than eaters, the next birth waits 300
                // more updates.
                let need = crate::game::virtual_tank::FUN_0053a3c0(g, board);
                let have = crate::game::virtual_tank::FUN_005397a0(g, board);
                let d = need[0] - have[0];
                if d == -1 || d + 1 < 0 {
                    g.fish(this).field_0xcc -= 300;
                }
            }
            let f = g.fish(this).clone();
            if f.field_0xcc < f.field_0xd0 {
                return;
            }
            g.fish(this).field_0xcc = 0;
            let baby = crate::game::board_level::FUN_00546ef0(g, board, mx + 7, my + 0x19);
            if mode(g, this) == 5 {
                g.go(baby).offset_0x98 = 0x28;
            }
            crate::game::board_level::FUN_005382c0(g, board, false);
            let n = g.board(board).offset_0xc[vec_index(0xa0)].len();
            if 10 < n {
                g.fish(this).field_0xd0 = 2000;
                return;
            }
            let n = crate::game::other_pet::FUN_004da710(&objs(g, board, 0xa0));
            g.fish(this).field_0xd0 = if 5 < n as u32 { 0x4ce } else { 0x3a2 };
        }
        6 => {
            g.fish(this).field_0xcc += 1;
            let f = g.fish(this).clone();
            if f.field_0xcc < f.field_0xd0 {
                return;
            }
            g.fish(this).field_0xcc = 0;
            if !crate::game::alien::FUN_004d70b0(g, this) {
                return;
            }
            crate::game::board_level::FUN_00544430(g, board, mx + 0xf, my + 10, 2, NULL, -1.0, 0);
        }
        0x15 => {
            g.fish(this).field_0xcc += 1;
            let f = g.fish(this).clone();
            if f.field_0xcc < f.field_0xd0 {
                return;
            }
            g.fish(this).field_0xcc = 0;
            crate::game::board_level::FUN_005434e0(g, board, mx + 0xf, my + 10);
        }
        0xb => {
            g.fish(this).field_0xcc += 1;
            let f = g.fish(this).clone();
            if f.field_0xcc < f.field_0xd0 {
                return;
            }
            g.fish(this).field_0xcc = 0;
            crate::game::board_level::FUN_00544430(g, board, mx + 0xf, my + 10, 0x11, NULL, -1.0, 0);
            crate::game::board::FUN_00538230(g, board, 0x143, 3, 1.0);
        }
        4 => {
            g.fish(this).field_0xcc += 1;
            let f = g.fish(this).clone();
            if f.field_0xcc < f.field_0xd0 {
                return;
            }
            let hungry = |g: &mut G, v: &[Ptr]| {
                let mut it = FUN_004e5040(v);
                while FUN_004e3dd0(it, FUN_004e5010(v)) {
                    let o = FUN_004e3e00(v, it);
                    if g.go(o).offset_0x14 < 300 && g.go(o).offset_0x68 == 0 {
                        return true;
                    }
                    FUN_004e5070(&mut it);
                }
                false
            };
            let guppies = objs(g, board, 0xa0);
            let mut found = hungry(g, &guppies);
            if !found {
                let others = objs(g, board, 0xc0);
                found = hungry(g, &others);
            }
            if g.board(board).field_0xb8[0x14] == 0 {
                if !found {
                    return;
                }
            } else {
                let m = mode(g, this);
                let d = g.fish_type_pet(this);
                d.offset_0x14 += 1;
                let n = d.offset_0x14;
                let div = (m == 5) as i32 * 2 + 2;
                if n % div != 0 && !found {
                    g.fish(this).field_0xcc = 0;
                    return;
                }
            }
            g.fish(this).field_0xcc = -10;
            let dir = if g.fish(this).offset_0x14 < 0.0 { 1 } else { 2 };
            crate::game::board_level::FUN_00543280(g, board, mx + 0xf, my + 10, dir, false, 0, -1);
        }
        8 => {
            g.fish(this).field_0xcc += 1;
            let f = g.fish(this).clone();
            if f.field_0xcc != f.field_0xd0 - 100 {
                if f.field_0xcc < f.field_0xd0 {
                    return;
                }
                g.globals.DAT_005e89cc = false;
                g.fish(this).field_0xcc = 0;
                return;
            }
            g.globals.DAT_005e89cc = true;
            crate::game::board_level::FUN_00544430(g, board, mx + 0xf, my - 5, 0x10, NULL, -1.0, 0);
            crate::game::board::FUN_00538230(g, board, 0x137, 3, 1.0);
        }
        0x11 => {
            g.fish(this).field_0xcc += 1;
        }
        0x10 => {
            g.fish(this).field_0xcc += 1;
            let f = g.fish(this).clone();
            if f.field_0xcc < f.field_0xd0 {
                return;
            }
            if f.field_0xcc == f.field_0xd0 {
                crate::game::board::FUN_00538230(g, board, 0x11a, 3, 1.0);
            }
            let w = g.w(this);
            w.offset_0x1 = true;
            w.offset_0x28 = true;
        }
        0x16 => {
            let aliens = crate::game::board_update::FUN_004da780(g, board);
            let d = g.fish_type_pet(this);
            if !aliens {
                if 0 < d.offset_0x34 {
                    d.offset_0x34 -= 1;
                }
            } else if d.offset_0x34 < 10 {
                d.offset_0x34 += 1;
            }
            g.fish(this).field_0xcc += 1;
            FUN_004e55d0(g, this);
            let f = g.fish(this).clone();
            if f.field_0xd0 < f.field_0xcc {
                g.fish(this).field_0xcc = 0;
                if g.fish_type_pet(this).offset_0x10 == 0 {
                    g.fish_type_pet(this).offset_0x10 = 0x32;
                }
                let right = 0.0 <= g.fish(this).offset_0x14;
                let t = vcall!(g, this, fish.vfunction86);
                if t != NULL {
                    let (mx, my) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
                    let x = if right { mx + 0x28 } else { mx };
                    crate::game::board_level::FUN_00544130(g, board, x, my, t, 2);
                    crate::game::board::FUN_00538230(g, board, 0x122, 3, 1.0);
                    g.fish(this).field_0x74 = 10;
                }
            }
        }
        _ => {}
    }
}

/// port: 004e55d0 FUN_004e55d0
/// Stanley (with no timer running) zaps a missile: of the ones still homing on a target
/// (kinds 0 and 1, past launch, not already zapped) the nearest to its target, preferring a
/// free-flying kind 1, then a homing kind 1, then kind 0. He then waits 30, 60 or 90
/// updates, and the missile's bolt runs from his mouth with a length of a sixteenth of the
/// distance. (The original first walks the alien list without using it.)
pub fn FUN_004e55d0(g: &mut G, this: Ptr) {
    let board = board_of(g, this);
    if 0 < g.fish_type_pet(this).offset_0x10 {
        return;
    }
    let mut best_d = 100000000;
    let mut best = NULL;
    let mut level = 0;
    for m in objs(g, board, 0xdc) {
        let d = g.missle(m).clone();
        if !(d.offset_0x28 < 0xb && d.offset_0x54 < 1 && (d.offset_0x48 == 1 || d.offset_0x48 == 0) && d.offset_0x50 != NULL) {
            continue;
        }
        let prev = level;
        if d.offset_0x48 == 0 {
            if level != 0 {
                continue;
            }
        } else if !d.offset_0x3c {
            if 1 < level {
                continue;
            }
            level = 1;
        } else {
            level = 2;
        }
        let t = g.wc(d.offset_0x50).clone();
        let mw = g.wc(m).clone();
        let dy = (t.offset_0x38 / 2 + t.offset_0x30) - (mw.offset_0x38 / 2 + mw.offset_0x30);
        let dx = (t.offset_0x34 / 2 + t.offset_0x2c) - (mw.offset_0x34 / 2 + mw.offset_0x2c);
        let d2 = dx * dx + dy * dy;
        if d2 < best_d || level != prev {
            best_d = d2;
            best = m;
        }
    }
    if g.fish_type_pet(this).offset_0x10 < 1 {
        if best != NULL {
            let r = crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32 % 100;
            g.fish_type_pet(this).offset_0x10 = if r < 10 { 0x1e } else if 0x3b < r { 0x5a } else { 0x3c };
            g.fish(this).field_0xcc = 0;
            crate::game::board::FUN_00538230(g, board, 0x122, 3, 1.0);
            let me = g.wc(this).clone();
            let mw = g.wc(best).clone();
            let sx = me.offset_0x34 / 2 + me.offset_0x2c;
            let sy = me.offset_0x30 + 10;
            let mx = mw.offset_0x34 / 2 + mw.offset_0x2c;
            let my = mw.offset_0x38 / 2 + mw.offset_0x30;
            let md = g.missle(best);
            md.offset_0x5c = sx;
            md.offset_0x60 = sy;
            md.offset_0x54 = 1;
            let (dx, dy) = (sx - mx, sy - my);
            let len = ((dx * dx + dy * dy) as f64).sqrt() as f32;
            g.missle(best).offset_0x58 = crate::sexy::crt::ftol(len as f64 * 0.0625) as i32;
            if g.fish(this).field_0x74 == 0 {
                g.fish(this).field_0x74 = 10;
            }
        }
    } else if 1 < level {
        let t = g.fish_type_pet(this).offset_0x10 - 2;
        g.fish_type_pet(this).offset_0x10 = t;
        if t < 0 {
            g.fish_type_pet(this).offset_0x10 = 0;
        }
    }
}

/// port: 004e5fc0 Sexy::FishTypePet::vfunction86
/// `FindTarget()`: the nearest of what each pet goes for: Itchy aliens (Stanley only free
/// ones), Gash aliens or else (once hungry) guppies of the level, Gumbo aliens, Angie the
/// newly dead, Nimbus free food (no aliens around, not in the virtual tank) and coins it
/// may take, Brinkley food.
pub fn vfunction86(g: &mut G, this: Ptr) -> Ptr {
    let p = pet(g, this);
    let board = board_of(g, this);
    let mut best = 10000;
    let mut target = NULL;
    if p == 2 || p == 0x16 {
        for o in objs(g, board, 0xf4) {
            let d = dist(g, this, o, 0x28);
            if d < best && (p != 0x16 || g.go(o).offset_0x10 == NULL) {
                target = o;
                best = d;
            }
        }
        for o in objs(g, board, 0xb8) {
            let d = dist(g, this, o, 0x50);
            if !g.alien(o).offset_0xb0 && d < best && (g.go(o).offset_0x10 == NULL || pet(g, this) != 0x16) {
                target = o;
                best = d;
            }
        }
    }
    let p = pet(g, this);
    if p == 0x11 {
        if objs(g, board, 0xb8).is_empty() && objs(g, board, 0xf4).is_empty() {
            let f = g.fish(this).clone();
            if f.field_0xcc <= f.field_0xd0 {
                return target;
            }
            let guppies = objs(g, board, 0xa0);
            if guppies.is_empty() {
                return target;
            }
            for o in guppies {
                if g.go(o).offset_0x24 < 0 {
                    let d = dist(g, this, o, 0x28);
                    if d < best {
                        target = o;
                        best = d;
                    }
                }
            }
            return target;
        }
        for o in objs(g, board, 0xf4) {
            let d = dist(g, this, o, 0x28);
            if d < best {
                target = o;
                best = d;
            }
        }
        for o in objs(g, board, 0xb8) {
            let d = dist(g, this, o, 0x50);
            if !g.alien(o).offset_0xb0 && d < best {
                target = o;
                best = d;
            }
        }
    } else if p == 0xc {
        for o in objs(g, board, 0xf4) {
            let d = dist(g, this, o, 0x28);
            if d < best {
                target = o;
                best = d;
            }
        }
        for o in objs(g, board, 0xb8) {
            let d = dist(g, this, o, 0x50);
            if d < best {
                target = o;
                best = d;
            }
        }
    } else if p == 0x12 {
        for o in objs(g, board, 0x98) {
            if 0x5f < g.dead_fish(o).offset_0x4c {
                let d = dist(g, this, o, 0x28);
                if d < best {
                    target = o;
                    best = d;
                }
            }
        }
    } else if p == 0xf {
        let app = app_of(g, this);
        let trial_over = crate::game::win_fish_app::thunk_FUN_00479fc0(g, app);
        if objs(g, board, 0xb8).is_empty() && objs(g, board, 0xf4).is_empty() && mode(g, this) != 5 {
            for o in objs(g, board, 0xac) {
                let d = dist(g, this, o, 0x14);
                let food = g.food(o).clone();
                if food.offset_0x40 == 0 && food.offset_0x38 == 0 && d < best {
                    target = o;
                    best = d;
                }
            }
        }
        let catchers = objs(g, board, 0xd0).len();
        for o in objs(g, board, 0xa8) {
            let c = g.coin(o).clone();
            if c.offset_0x40 == 0x11 {
                continue;
            }
            if (c.offset_0x40 == 3 || c.offset_0x40 == 10) && catchers != 0 {
                continue;
            }
            if c.offset_0x44 || c.offset_0x40 == 0x12 {
                continue;
            }
            if crate::game::other_pet::FUN_004e4f90(g, this, o) {
                continue;
            }
            if trial_over && DAT_005df588 < g.wc(o).offset_0x24 {
                continue;
            }
            let d = dist(g, this, o, 0x24);
            if d < best {
                target = o;
                best = d;
            }
        }
    } else if p == 0x14 {
        for o in objs(g, board, 0xac) {
            let d = dist(g, this, o, 0x14);
            if d < best {
                target = o;
                best = d;
            }
        }
    }
    target
}

/// port: 004e9790 Sexy::FishTypePet::vfunction83
/// `Hunt()`: whether the pet chases this update (then `Chase()`): Angie for the dead,
/// Nimbus for coins and (no aliens) food unless asleep (slow when not chasing), Itchy
/// and Gumbo for aliens, Brinkley for food when its pause is over, Gash for aliens or
/// guppies once hungry. Wadsworth shelters the guppies (`DAT_005e89d0`, around him at
/// `DAT_005e89d4`/`d8`) while aliens are around (or in the time trial with too few big
/// guppies), dropping them off when they leave.
pub fn vfunction83(g: &mut G, this: Ptr) -> bool {
    let app = app_of(g, this);
    let board = g.wfa(app).offset_0x4;
    let coins = objs(g, board, 0xa8);
    let food = objs(g, board, 0xac);
    let p = pet(g, this);
    let chase = |g: &mut G| {
        vcall!(g, this, fish.vfunction85);
        true
    };
    if p == 0x12 && !objs(g, board, 0x98).is_empty() {
        return chase(g);
    }
    if p == 0xf {
        if g.wfa(app).offset_0x150 == 5 {
            let mut asleep = g.fish_type_pet(this).offset_0x19;
            crate::game::other_pet::FUN_004d6b70(g, this, &mut asleep);
            g.fish_type_pet(this).offset_0x19 = asleep;
        }
        if (!coins.is_empty() || (!food.is_empty() && g.wfa(app).offset_0x150 != 5)) && !g.fish_type_pet(this).offset_0x19 {
            g.fish(this).field_0x2c = 0.5;
            return chase(g);
        }
        g.fish(this).field_0x2c = 1.8;
    }
    let p = pet(g, this);
    let aliens = |g: &mut G| crate::game::board_update::FUN_004da780(g, board);
    let to_chase = if p == 2 && aliens(g) {
        true
    } else if p == 0x14 && !aliens(g) && crate::game::board_update::FUN_004da800(g, board) && g.fish_type_pet(this).offset_0x10 == 0 {
        true
    } else if p == 0x11 && (aliens(g) || {
        let f = g.fish(this).clone();
        f.field_0xd0 < f.field_0xcc && crate::game::board_update::FUN_004da7d0(g, board)
    }) {
        true
    } else if p == 0xc && aliens(g) {
        true
    } else {
        if p == 9 {
            let cc = g.fish(this).field_0xcc;
            if cc < 1 {
                if g.fish_type_pet(this).offset_0x24 && FUN_004e7010(g, this) {
                    g.fish(this).field_0xcc = 100;
                }
            } else {
                g.fish(this).field_0xcc = cc - 1;
            }
            let leave = (!aliens(g) && !crate::game::board_level::FUN_004e3ea0(g, board))
                || (g.wfa(app).offset_0x150 == 4 && 1 < g.board(board).ext_0x45c && FUN_004e7130(g, this));
            if leave {
                if g.fish_type_pet(this).offset_0x24 {
                    g.fish_type_pet(this).offset_0x24 = false;
                    g.globals.DAT_005e89d0 -= 1;
                    if g.globals.DAT_005e89d0 < 0 {
                        g.globals.DAT_005e89d0 = 0;
                    }
                    let (mx, my) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
                    g.fish(this).field_0xcc = 0x1e;
                    crate::game::board_update::FUN_00538ac0(g, board, mx + 0xb, my + 5);
                    let (mx, my) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
                    crate::game::board_update::FUN_00538ac0(g, board, mx + 4, my + 2);
                    return false;
                }
            } else {
                if !g.fish_type_pet(this).offset_0x24 {
                    g.globals.DAT_005e89d0 += 1;
                    let d0 = g.fish(this).field_0xd0;
                    g.fish_type_pet(this).offset_0x24 = true;
                    g.fish(this).field_0xcc = d0;
                }
                g.globals.DAT_005e89d4 = g.wc(this).offset_0x2c;
                g.globals.DAT_005e89d8 = g.wc(this).offset_0x30;
            }
        }
        return false;
    };
    if to_chase {
        return chase(g);
    }
    false
}

/// port: 004e7010 FUN_004e7010
/// Wadsworth (with at most one shelter going): a small guppy is more than 10 pixels
/// from him on both axes.
pub fn FUN_004e7010(g: &mut G, this: Ptr) -> bool {
    if 1 < g.globals.DAT_005e89d0 {
        return false;
    }
    let board = board_of(g, this);
    for o in objs(g, board, 0xa0) {
        let (dx, dy) = gap(g, this, o, 0x28);
        if 10.0 < dx && !(dy < 10.0) && dy != 10.0 && g.fish(o).offset_0x4c < 2 {
            return true;
        }
    }
    false
}

/// port: 004e7130 FUN_004e7130
/// Nothing else is in the tank (no +0xa4, +0xd0, +0xc4, +0xcc, +0xc0 or +0xd4 objects)
/// and every guppy is small or medium.
pub fn FUN_004e7130(g: &mut G, this: Ptr) -> bool {
    let board = board_of(g, this);
    if objs(g, board, 0xa4).is_empty()
        && objs(g, board, 0xd0).is_empty()
        && objs(g, board, 0xc4).is_empty()
        && !crate::game::board_update::FUN_004e3f30(g, board)
        && !crate::game::board_update::FUN_004e3f60(g, board)
        && !crate::game::board_update::FUN_004e3ed0(g, board)
    {
        let v = objs(g, board, 0xa0);
        let mut it = FUN_004e5040(&v);
        while FUN_004e3dd0(it, FUN_004e5010(&v)) {
            let o = FUN_004e3e00(&v, it);
            if 1 < g.fish(o).offset_0x4c {
                return false;
            }
            FUN_004e5070(&mut it);
        }
        return true;
    }
    false
}

/// port: 004f7ad0 Sexy::FishTypePet::vfunction23
/// `Update()` (not while paused): Blip reveals the shop prices at a new level, Walter's
/// glove and timers, wandering in the current swim mode unless hunting (the sweep modes 5+
/// patrol 175..250), the mode re-roll (1 in 10 every 20 updates, or near Nostradamus'
/// floor), the pet timer (with aliens around only Stanley's), being pushed (Presto turns
/// back when the push ends), sinking when still, Presto's timers, Brinkley's drift, staying
/// in the tank (Zorf in disguise and Nimbus switch modes at the edges), the walls, animation
/// and moving (5x slower asleep).
pub fn vfunction23(g: &mut G, this: Ptr) {
    let app = app_of(g, this);
    let board = g.wfa(app).offset_0x4;
    if board == NULL {
        return;
    }
    if g.board(board).field_0x8 {
        return;
    }
    FUN_004f22c0(g, this);
    if pet(g, this) == 0xd && g.wfa(app).offset_0x150 != 5 {
        let b = g.board(board);
        if b.field_0x254 < b.field_0x250
            && b.field_0x33c != 5
            && 200 < b.ext_0x444
            && (!g.fish_type_pet(this).offset_0x4 || 0x2d0 < g.wc(this).offset_0x24)
        {
            crate::game::board::FUN_00538230(g, board, 0x13c, 3, 1.0);
            for i in 0..0xc {
                crate::game::board_level::FUN_005409b0(g, board, i, true);
            }
        }
    }
    let glove = g.fish_type_pet(this).offset_0x20;
    if glove != NULL {
        vcall!(g, glove, w.vfunction23);
        let glove = g.fish_type_pet(this).offset_0x20;
        if g.boxing_glove(glove).offset_0x8 < 1 {
            if glove != NULL {
                vcall!(g, glove, w.vfunction1, 1);
            }
            g.fish_type_pet(this).offset_0x20 = NULL;
        }
    }
    let cool = g.fish_type_pet(this).offset_0x38;
    if cool != 0 {
        let cool = cool - 1;
        g.fish_type_pet(this).offset_0x38 = cool;
        if pet(g, this) == 0x17 && cool == 0 {
            g.w(this).offset_0x28 = true;
        }
    }
    let t = g.fish_type_pet(this).offset_0x10;
    if t != 0 {
        g.fish_type_pet(this).offset_0x10 = t - 1;
    }
    let hunting = g.board(board).field_0x33c != 5 && vcall!(g, this, fish.vfunction83);
    if !hunting {
        let walter_cool = 0 < g.fish_type_pet(this).offset_0x38 && pet(g, this) == 0x17;
        let f = g.fish(this);
        let m = f.field_0x58;
        if m < 5 {
            let mut fall = 0.5;
            let mut drift = true;
            match m {
                0 => {
                    f.field_0x1c = if walter_cool { 1.0 } else { 0.5 };
                    if f.field_0x5c < 0x28 {
                        fall = 0.25;
                    } else {
                        f.field_0x5c = 0;
                        if f.offset_0x14 <= 0.0 {
                            if f.offset_0x14 < 0.0 && f.offset_0x14 < -0.5 {
                                f.offset_0x14 += 0.5;
                            }
                        } else if 0.5 < f.offset_0x14 {
                            f.offset_0x14 -= 0.5;
                        }
                        f.field_0x68 = ftol(f.offset_0x14.abs()) as i32;
                        fall = 0.25;
                    }
                }
                1 => {
                    f.field_0x1c = -0.5;
                    if 0x27 < f.field_0x5c {
                        f.field_0x5c = 0;
                        if f.offset_0x14 <= 1.0 {
                            if f.offset_0x14 < 1.0 {
                                f.offset_0x14 += 1.0;
                            }
                        } else {
                            f.offset_0x14 -= 1.0;
                        }
                        f.field_0x68 = ftol(f.offset_0x14.abs()) as i32;
                    }
                }
                2 => {
                    f.field_0x1c = -0.5;
                    if 0x27 < f.field_0x5c {
                        f.field_0x5c = 0;
                        if f.offset_0x14 <= -1.0 {
                            if f.offset_0x14 < -1.0 {
                                f.offset_0x14 += 1.0;
                            }
                        } else {
                            f.offset_0x14 -= 1.0;
                        }
                        f.field_0x68 = ftol(f.offset_0x14.abs()) as i32;
                    }
                }
                3 | 4 => {
                    drift = false;
                    if 0x27 < f.field_0x5c {
                        f.field_0x5c = 0;
                        if m == 3 {
                            if f.offset_0x14 <= -1.0 {
                                if f.offset_0x14 < -1.0 {
                                    f.offset_0x14 += 1.0;
                                }
                            } else {
                                f.offset_0x14 -= 1.0;
                            }
                        } else if f.offset_0x14 <= 1.0 {
                            if f.offset_0x14 < 1.0 {
                                f.offset_0x14 += 1.0;
                            }
                        } else {
                            f.offset_0x14 -= 1.0;
                        }
                        if f.field_0x1c <= 3.0 {
                            if f.field_0x1c < 3.0 {
                                f.field_0x1c += 1.0;
                            }
                        } else {
                            f.field_0x1c -= 1.0;
                        }
                        if f.field_0x68 < 5 {
                            if f.field_0x1c < 4.0 {
                                f.field_0x68 += 1;
                            }
                        } else {
                            f.field_0x68 -= 1;
                        }
                    }
                    if 240.0 < f.field_0xc {
                        f.field_0x58 = 0;
                    }
                }
                _ => {
                    drift = false;
                    if m == -1 {
                        f.field_0x1c = -10.0;
                    }
                }
            }
            if drift {
                f.field_0xc -= fall / f.field_0x2c;
            }
        } else {
            f.field_0x1c = if f.field_0xc < 115.0 { -0.1 } else { -0.5 };
            if 0x27 < f.field_0x5c {
                let dir = f.field_0x24;
                f.field_0x5c = 0;
                if dir == 1 {
                    f.offset_0x14 += if f.offset_0x14 < 0.0 { 2.0 } else { 1.0 };
                    f.field_0x68 = ftol(f.offset_0x14.abs()) as i32;
                    if 250.0 < f.field_0x4 {
                        f.field_0x24 = -1;
                        f.offset_0x14 -= 2.0;
                    }
                } else if dir == -1 {
                    f.offset_0x14 -= if 0.0 < f.offset_0x14 { 2.0 } else { 1.0 };
                    f.field_0x68 = ftol(f.offset_0x14.abs()) as i32;
                    if f.field_0x4 < 175.0 {
                        f.field_0x24 = 1;
                        f.offset_0x14 += 2.0;
                    }
                }
            }
        }
    }
    {
        let f = g.fish(this);
        f.field_0x5c += 1;
        f.field_0x60 += 1;
    }
    let p = pet(g, this);
    let reroll = {
        let f = g.fish(this);
        0x14 < f.field_0x60 || (p == 0x15 && ((f.field_0x3c - 0x32) as f64) < f.field_0xc)
    };
    if reroll {
        g.fish(this).field_0x60 = 0;
        if rand(g, this) % 10 == 0 && (g.fish_type_pet(this).offset_0x38 == 0 || pet(g, this) != 0x17) {
            let m = (rand(g, this) % 9 + 1) as i32;
            g.fish(this).field_0x58 = m;
        }
    }
    let aliens = crate::game::board_update::FUN_004da780(g, board);
    let parts = !objs(g, board, 0xf4).is_empty();
    let timer = if !aliens && !parts {
        true
    } else {
        pet(g, this) == 0x16 && (crate::game::board_update::FUN_004da780(g, board) || parts)
    };
    if timer {
        vcall!(g, this, fish.vfunction82);
    }
    let push = g.fish(this).offset_0xb8;
    if push != 0 {
        let push = push - 1;
        let presto = g.fish_type_pet(this).offset_0x4;
        let f = g.fish(this);
        let v = f.offset_0xc4 * 0.9;
        f.offset_0xb8 = push;
        f.offset_0xc4 = v;
        f.field_0x4 = v + f.field_0x4;
        if presto && push < 0x29 && pet(g, this) != 0x13 {
            g.fish_type_pet(this).offset_0xc = 0;
            vcall!(g, this, go.vfunction74, 0x13);
            return;
        }
    }
    {
        let f = g.fish(this);
        if 0 < f.offset_0xbc {
            f.offset_0xbc -= 1;
        }
    }
    let p = pet(g, this);
    if p != 0x10 {
        let f = g.fish(this);
        if f.offset_0x14 == 0.0 {
            f.field_0xc = 1.0 / f.field_0x2c + f.field_0xc;
        }
        if f.offset_0x14 == 1.0 {
            f.field_0xc = 0.75 / f.field_0x2c + f.field_0xc;
        }
        if f.offset_0x14 == 2.0 {
            f.field_0xc = 0.5 / f.field_0x2c + f.field_0xc;
        }
        if f.offset_0x14 == 3.0 {
            f.field_0xc = 0.25 / f.field_0x2c + f.field_0xc;
        }
    }
    if p == 0x10 {
        let f = g.fish(this);
        if f.field_0x1c <= 0.5 {
            if f.field_0x1c < -0.5 {
                f.field_0x1c = -0.5;
            }
        } else {
            f.field_0x1c = 0.5;
        }
    }
    {
        let d = g.fish_type_pet(this);
        if d.offset_0xc != 0 {
            d.offset_0xc -= 1;
        }
    }
    let presto = g.fish_type_pet(this).offset_0x4;
    {
        let d = g.fish_type_pet(this);
        if (presto || p == 0x15) && d.offset_0x1c < 0x14 {
            d.offset_0x1c += 1;
        }
    }
    let f = g.fish(this);
    let mut done = false;
    if p == 0x14 {
        let a = f.offset_0x14.abs();
        if a <= 0.0 {
            f.field_0xc += 1.0;
            done = true;
        } else if a <= 1.0 {
            f.field_0xc += 0.75;
            done = true;
        }
    }
    if !done && p == 0x14 {
        let a = f.offset_0x14.abs();
        if a <= 2.0 || a <= 3.0 {
            f.field_0xc += 0.5;
        }
    }
    let right = f.field_0x48 as f64;
    if right < f.field_0x4 {
        f.field_0x4 = right;
    }
    let left = f.field_0x40 as f64;
    if f.field_0x4 < left {
        f.field_0x4 = left;
    }
    let floor = f.field_0x3c as f64;
    if f.field_0xc <= floor || !presto || p != 4 {
        if floor < f.field_0xc {
            f.field_0xc = floor;
        }
    } else {
        f.field_0x58 = 2;
    }
    let top = f.field_0x44 as f64;
    if top <= f.field_0xc || !presto || p != 0xf {
        if f.field_0xc < top {
            f.field_0xc = top;
        }
    } else {
        f.field_0x58 = 0;
        f.field_0x1c += 0.25;
    }
    if f.field_0xc <= top && f.field_0x58 == -1 {
        let m = (rand(g, this) % 9 + 1) as i32;
        g.fish(this).field_0x58 = m;
    }
    let f = g.fish(this);
    if ((f.field_0x48 - 5) as f64) < f.field_0x4 && 0.1 < f.offset_0x14 {
        f.offset_0x14 -= 0.1;
    }
    if f.field_0x4 < 15.0 && f.offset_0x14 < -0.1 {
        f.offset_0x14 += 0.1;
    }
    vcall!(g, this, fish.vfunction90);
    let asleep = g.fish_type_pet(this).offset_0x19;
    let f = g.fish(this);
    if !asleep {
        f.field_0x4 = f.offset_0x14 / f.field_0x2c + f.field_0x4;
        let dy = f.field_0x1c / f.field_0x2c;
        f.field_0xc = dy + f.field_0xc;
    } else {
        let s = f.field_0x2c * 5.0;
        f.field_0x4 = f.offset_0x14 / s + f.field_0x4;
        let dy = f.field_0x1c / s;
        f.field_0xc = dy + f.field_0xc;
    }
    let (x, y) = (ftol(f.field_0x4) as i32, ftol(f.field_0xc) as i32);
    vcall!(g, this, w.vfunction42, x, y);
}

/// Gash's (and Itchy's) attack on the aliens: a bite within 20 pixels (not on a
/// Psychosquid in its healing form) ends the turn; within 30 the mouth opens.
fn bite_aliens(g: &mut G, this: Ptr, board: Ptr) {
    for o in objs(g, board, 0xb8) {
        let (dx, dy) = gap(g, this, o, 0x50);
        if dx < 20.0 && dy < 20.0 && !g.alien(o).offset_0xb0 {
            FUN_004fb5b0(g, this, o);
            return;
        }
        let (dx, dy) = gap(g, this, o, 0x50);
        if dx < 30.0 && dy < 30.0 && g.fish(this).field_0x74 == 0 {
            g.fish(this).field_0x74 = 10;
            return;
        }
    }
}

/// port: 004fe740 Sexy::FishTypePet::vfunction87
/// `TryEat()`: Itchy bites alien parts and aliens; Gash bites alien parts (killing their
/// body) and aliens, or once hungry eats a guppy of the level; Angie revives a dead fish;
/// Nimbus collects a coin (+7 to the kinds below 8) or, with no aliens around, pops food
/// into a pellet; Brinkley eats food (a treasure every third meal).
pub fn vfunction87(g: &mut G, this: Ptr) {
    let board = board_of(g, this);
    let p = pet(g, this);
    if p == 2 {
        for o in objs(g, board, 0xf4) {
            let (dx, dy) = gap(g, this, o, 0x28);
            if dx < 20.0 && dy < 20.0 {
                crate::game::bilaterus::alien_part_body_hurt(g, o, 1.0);
                crate::game::board_level::FUN_00538340(g, board, 0xb);
            }
        }
        for o in objs(g, board, 0xb8) {
            let (dx, dy) = gap(g, this, o, 0x50);
            if dx < 20.0 && dy < 20.0 && !g.alien(o).offset_0xb0 {
                FUN_004fb5b0(g, this, o);
                return;
            }
        }
    } else if p == 0x11 {
        if crate::game::board_update::FUN_004da780(g, board) {
            for o in objs(g, board, 0xf4) {
                let (dx, dy) = gap(g, this, o, 0x28);
                if 20.0 <= dx || 20.0 <= dy {
                    if dx < 30.0 && dy < 30.0 && g.fish(this).field_0x74 == 0 {
                        g.fish(this).field_0x74 = 10;
                        break;
                    }
                } else {
                    let hp = crate::game::bilaterus::alien_part_body_hurt(g, o, 3.0);
                    crate::game::board_level::FUN_00538340(g, board, 9);
                    if hp <= 0.0 {
                        let body = crate::game::bilaterus::alien_part_body(g, o);
                        crate::game::bilaterus::FUN_004fb4c0(g, body, true);
                        break;
                    }
                }
            }
            bite_aliens(g, this, board);
            return;
        }
        let f = g.fish(this).clone();
        if f.field_0xd0 < f.field_0xcc && crate::game::board_update::FUN_004da7d0(g, board) {
            let v = objs(g, board, 0xa0);
            let mut it = FUN_004e5040(&v);
            while it != v.len() {
                let o = v[it];
                if g.go(o).offset_0x24 < 0 {
                    let (dx, dy) = gap(g, this, o, 0x28);
                    if dx < 20.0 && dy < 20.0 {
                        let o = FUN_004e3e00(&v, it);
                        vcall!(g, o, fish.vfunction88, true);
                        crate::game::board_level::FUN_00538560(g, board, false);
                        g.fish(this).field_0xcc = 0;
                        return;
                    }
                    if dx < 30.0 && dy < 30.0 && g.fish(this).field_0x74 == 0 {
                        g.fish(this).field_0x74 = 10;
                        return;
                    }
                }
                FUN_004e5070(&mut it);
            }
        }
    } else if p == 0x12 {
        for o in objs(g, board, 0x98) {
            let big = g.dead_fish(o).offset_0x50 == 6;
            let (k, r) = if big { (0x50, 60.0) } else { (0x28, 30.0) };
            let (dx, dy) = gap(g, this, o, k);
            if dx < r && dy < r && 0x5f < g.dead_fish(o).offset_0x4c {
                crate::game::dead_fish::FUN_004d5630(g, o);
                return;
            }
        }
    } else if p == 0xf {
        let app = app_of(g, this);
        let trial_over = crate::game::win_fish_app::thunk_FUN_00479fc0(g, app);
        let no_catchers = FUN_004e3da0(&objs(g, board, 0xd0));
        let coins = objs(g, board, 0xa8);
        let mut it = FUN_004e5040(&coins);
        let mut hit = NULL;
        while it != coins.len() {
            let c = coins[it];
            let (dx, dy) = gap(g, this, c, 0x24);
            let d = g.coin(c).clone();
            let kind = d.offset_0x40;
            if dx < 30.0
                && dy < 30.0
                && kind != 0x11
                && ((kind != 3 && kind != 10) || no_catchers)
                && !d.offset_0x44
                && kind != 0x12
                && !crate::game::other_pet::FUN_004e4f90(g, this, c)
                && (!trial_over || g.wc(FUN_004e3e00(&coins, it)).offset_0x24 <= DAT_005df588)
            {
                hit = c;
                break;
            }
            it += 1;
        }
        if hit != NULL {
            let mut kind = g.coin(hit).offset_0x40;
            if kind < 8 {
                kind += 7;
            }
            let x = g.wc(hit).offset_0x2c;
            let age = g.wc(hit).offset_0x24;
            crate::game::coin::vfunction76(g, hit);
            let y = ftol(g.fish(this).field_0xc - 25.0) as i32;
            crate::game::board_level::FUN_00544430(g, board, x, y, kind, NULL, -1.0, age);
        }
        if !crate::game::board_update::FUN_004da780(g, board) && mode(g, this) != 5 {
            for o in objs(g, board, 0xac) {
                let (dx, dy) = gap(g, this, o, 0x14);
                if dx < 30.0 && dy < 30.0 && g.food(o).offset_0x38 == 0 {
                    let x = g.wc(o).offset_0x2c;
                    let grade = g.food(o).offset_0x34;
                    crate::game::larva::vfunction76(g, o);
                    let y = ftol(g.fish(this).field_0xc - 30.0) as i32;
                    crate::game::board_level::FUN_00543280(g, board, x, y, 0, true, 0, grade);
                    return;
                }
            }
        }
    } else if p == 0x14 {
        let v = objs(g, board, 0xac);
        let mut it = FUN_004e5040(&v);
        while it != v.len() {
            let o = v[it];
            let fx = g.fish(this).field_0x4 + 40.0;
            let (ox, oy) = (g.wc(o).offset_0x2c, g.wc(o).offset_0x30);
            if fx < (ox + 0x23) as f64 && ((ox + 5) as f64) < fx {
                let fy = g.fish(this).field_0xc + 40.0;
                if fy < (oy + 0x23) as f64 && (oy as f64) < fy {
                    crate::game::board_level::FUN_005384c0(g, board, false);
                    crate::game::larva::vfunction76(g, o);
                    let m = mode(g, this);
                    let d = g.fish_type_pet(this);
                    d.offset_0x14 += 1;
                    d.offset_0x10 = if m != 5 { 0x2d } else { 0x6c };
                    let n = d.offset_0x14;
                    if n % 3 != 0 {
                        return;
                    }
                    if !crate::game::alien::FUN_004d70b0(g, this) {
                        return;
                    }
                    let n = g.fish_type_pet(this).offset_0x14;
                    let kind = if n % 0x87 == 0 {
                        0xe
                    } else if n % 0x2d == 0 {
                        0xd
                    } else if n % 0xf == 0 {
                        0xb
                    } else {
                        10
                    };
                    let (mx, my) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
                    crate::game::board_level::FUN_00544430(g, board, mx + 5, my - 10, kind, NULL, -1.0, 0);
                    return;
                }
            }
            it += 1;
        }
    }
}

/// port: 004f8610 Sexy::FishTypePet::vfunction84
/// `DrawPet(Graphics*, bool mirror)`: Walter's glove (drawn where it is), Presto white,
/// the pet's own art, Presto's appear sparkle, what it carries, Presto's charge ring.
pub fn vfunction84(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: bool) {
    let glove = g.fish_type_pet(this).offset_0x20;
    if glove != NULL {
        let (gx, gy) = (g.wc(glove).offset_0x2c, g.wc(glove).offset_0x30);
        let y = ftol(gy as f64 - gfx.s.mTransY as f64) as i32;
        let x = ftol(gx as f64 - gfx.s.mTransX as f64) as i32;
        FUN_004563d0(gfx, x, y);
        vcall!(g, glove, w.vfunction27, gfx);
        let (mx, my) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
        let y = ftol(my as f64 - gfx.s.mTransY as f64) as i32;
        let x = ftol(mx as f64 - gfx.s.mTransX as f64) as i32;
        FUN_004563d0(gfx, x, y);
    }
    let presto = g.fish_type_pet(this).offset_0x4;
    if presto {
        FUN_00455890(gfx, crate::sexy::types::FUN_00433320(0xffffff));
    }
    let p = pet(g, this);
    match p {
        2 => FUN_004df710(g, this, gfx, param_2),
        3 => FUN_004df230(g, this, gfx, param_2),
        4 => FUN_004df980(g, this, gfx, param_2),
        6 => {
            let img = g.res.DAT_005e8d38;
            FUN_004dfbb0(g, this, gfx, img, param_2)
        }
        8 => FUN_004df670(g, this, gfx, param_2),
        9 => FUN_004e4e40(g, this, gfx, param_2),
        10 => {
            let img = g.res.DAT_005e8b0c;
            FUN_004dfbb0(g, this, gfx, img, param_2)
        }
        0xb => FUN_004df890(g, this, gfx, param_2),
        0xc => FUN_004df470(g, this, gfx, param_2),
        0xd => {
            let img = g.res.DAT_005e8a14;
            FUN_004dfbb0(g, this, gfx, img, param_2)
        }
        0xf => {
            let img = g.res.DAT_005e8a08;
            FUN_004dfbb0(g, this, gfx, img, param_2)
        }
        0x10 => FUN_004df2f0(g, this, gfx, param_2),
        0x11 => FUN_004df810(g, this, gfx, param_2),
        0x12 => FUN_004df590(g, this, gfx, param_2),
        0x13 => {
            let img = g.res.DAT_005e8d60;
            FUN_004dfbb0(g, this, gfx, img, param_2)
        }
        0x14 => {
            let img = g.res.DAT_005e8b4c;
            FUN_004dfbb0(g, this, gfx, img, param_2)
        }
        0x15 => {
            let board = board_of(g, this);
            let b = g.board(board);
            let shake = b.field_0x228;
            let (mut sx, mut sy) = (0, 0);
            if shake != 0 && shake < 0x19 && !b.field_0x8 {
                sx = crate::sexy::crt::rand(g) % 0x15 - 10;
                sy = crate::sexy::crt::rand(g) % 0x15 - 10;
                FUN_004563d0(gfx, sx, sy);
            }
            let img = g.res.DAT_005e8ed4;
            FUN_004dfbb0(g, this, gfx, img, param_2);
            FUN_004563d0(gfx, -sx, -sy);
        }
        0x16 => FUN_004dfac0(g, this, gfx, param_2),
        0x17 => FUN_004dfa00(g, this, gfx, param_2),
        _ => {}
    }
    let d = g.fish_type_pet(this).clone();
    if (d.offset_0x4 || d.offset_0x8 == 0x15) && d.offset_0x1c < 0x14 {
        FUN_004558c0(gfx, 1);
        FUN_004558e0(gfx, true);
        FUN_00455890(gfx, CRect(0xaf, 0xaf, 0xaf, 0xff));
        let img = g.res.DAT_005e8d60;
        FUN_004560a0(gfx, g, img, 0, 0, &Rect::new((d.offset_0x1c / 2) * 0x50, 0xa0, 0x50, 0x50), param_2);
        FUN_004558c0(gfx, 0);
        FUN_004558e0(gfx, false);
    }
    if g.go(this).offset_0x10 != NULL {
        FUN_004d7040(g, this, gfx, 0, 0);
    }
    let d = g.fish_type_pet(this).clone();
    if d.offset_0x4 && d.offset_0x8 != 0x13 {
        crate::game::other_pet::FUN_004f2450(g, this, gfx, d.offset_0xc);
    }
}

/// The cel row for a pet drawn turning: (mirror, row 1) while the turn counter runs.
fn turning(g: &mut G, this: Ptr, mirror: bool) -> (bool, bool) {
    let t = g.fish(this).field_0x70;
    if t != 0 {
        (0 < t, true)
    } else {
        (mirror, false)
    }
}

/// port: 004df230 FUN_004df230
/// Prego: swimming, turning, about to give birth (row 3, swelling in the last 10 and
/// the first updates after) or pregnant (row 1).
pub fn FUN_004df230(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: bool) {
    let f = g.fish(this).clone();
    let mut mirror = param_2;
    let mut col = f.field_0x6c;
    let mut row = 0;
    if f.field_0x70 == 0 {
        let (t, n) = (f.field_0xcc, f.field_0xd0);
        if n - 200 < t {
            if t < n - 0xbe {
                row = 3;
                col = ((t - n) + 200) / 2;
            } else if n - 5 < t {
                col = (t - n) + 10;
                row = 3;
            } else {
                row = 1;
            }
        }
    } else {
        mirror = 0 < f.field_0x70;
        row = 2;
    }
    let img = g.res.DAT_005e8ca0;
    FUN_004560a0(gfx, g, img, 0, 0, &Rect::new(col * 0x50, row * 0x50, 0x50, 0x50), mirror);
}

/// port: 004df2f0 FUN_004df2f0
/// Amp (160x60): the body, and the additive charge glow (white-yellow, orange at the
/// second charge, red at the third) once charged, or while the charge is negative with no
/// aliens around.
pub fn FUN_004df2f0(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: bool) {
    let f = g.fish(this).clone();
    let (mirror, turn) = turning(g, this, param_2);
    let src = Rect::new(f.field_0x6c * 0xa0, turn as i32 * 0x3c, 0xa0, 0x3c);
    let img = g.res.DAT_005e8de8;
    FUN_004560a0(gfx, g, img, 0, 0, &src, mirror);
    let pulse = g.fish_type_pet(this).offset_0x2c;
    let alpha = || ftol(255.0_f64.min(pulse.abs() * 2.0 * 255.0)) as i32;
    let (green, blue, a);
    if f.field_0xcc < f.field_0xd0 {
        if -1 < f.field_0xcc {
            return;
        }
        let board = board_of(g, this);
        if crate::game::board_update::FUN_004da780(g, board) {
            return;
        }
        FUN_004558e0(gfx, true);
        FUN_004558c0(gfx, 1);
        a = alpha();
        green = 0xff;
        blue = 200;
    } else {
        FUN_004558e0(gfx, true);
        FUN_004558c0(gfx, 1);
        a = alpha();
        let charges = g.fish_type_pet(this).offset_0x34;
        if charges != 0 {
            blue = 100;
            green = if charges == 1 { 200 } else { 100 };
        } else {
            green = 0xff;
            blue = 200;
        }
    }
    FUN_00455890(gfx, CRect(0xff, green, blue, a));
    let img = g.res.DAT_005e8ce8;
    FUN_004560a0(gfx, g, img, 0, 0, &src, mirror);
    FUN_004558c0(gfx, 0);
    FUN_004558e0(gfx, false);
}

/// port: 004df470 FUN_004df470
/// Gumbo, and its yellow additive glow while aliens are around.
pub fn FUN_004df470(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: bool) {
    let f = g.fish(this).clone();
    let (mirror, turn) = turning(g, this, param_2);
    let src = Rect::new(f.field_0x6c * 0x50, turn as i32 * 0x50, 0x50, 0x50);
    let img = g.res.DAT_005e8cb4;
    FUN_004560a0(gfx, g, img, 0, 0, &src, mirror);
    let board = board_of(g, this);
    if objs(g, board, 0xb8).is_empty() && objs(g, board, 0xf4).is_empty() {
        return;
    }
    FUN_004558e0(gfx, true);
    let a = ftol(g.fish_type_pet(this).offset_0x2c.abs() * 255.0) as i32;
    FUN_00455890(gfx, CRect(0xff, 0xff, 0, a));
    FUN_004558c0(gfx, 1);
    let img = g.res.DAT_005e8bcc;
    FUN_004560a0(gfx, g, img, 0, 0, &src, mirror);
    FUN_004558e0(gfx, false);
    FUN_004558c0(gfx, 0);
}

/// port: 004df590 FUN_004df590
/// Angie, and her white additive glow.
pub fn FUN_004df590(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: bool) {
    let f = g.fish(this).clone();
    let (mirror, turn) = turning(g, this, param_2);
    let src = Rect::new(f.field_0x6c * 0x50, turn as i32 * 0x50, 0x50, 0x50);
    let img = g.res.DAT_005e8e8c;
    FUN_004560a0(gfx, g, img, 0, 0, &src, mirror);
    FUN_004558c0(gfx, 1);
    FUN_004558e0(gfx, true);
    let a = ftol(g.fish_type_pet(this).offset_0x2c.abs() * 255.0) as i32;
    FUN_00455890(gfx, CRect(0xff, 0xff, 0xff, a));
    let img = g.res.DAT_005e8c7c;
    FUN_004560a0(gfx, g, img, 0, 0, &src, mirror);
    FUN_004558c0(gfx, 0);
    FUN_004558e0(gfx, false);
}

/// port: 004df670 FUN_004df670
/// Meryl: singing (row 1, the last 100 updates of her timer), turning (row 2), blinking
/// (row 3, early cels).
pub fn FUN_004df670(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: bool) {
    let f = g.fish(this).clone();
    let blink = g.fish_type_pet(this).offset_0x18;
    let mut mirror = param_2;
    let mut row = 0;
    if f.field_0x70 == 0 {
        if f.field_0xd0 - 100 < f.field_0xcc {
            row = 1;
        } else if f.field_0x6c < 5 && blink {
            row = 3;
        }
    } else {
        mirror = 0 < f.field_0x70;
        row = 2;
    }
    let img = g.res.DAT_005e8b08;
    FUN_004560a0(gfx, g, img, 0, 0, &Rect::new(f.field_0x6c * 0x50, row * 0x50, 0x50, 0x50), mirror);
}

/// port: 004df710 FUN_004df710
/// Itchy: rows 2 (alert) and 3 (alert turning) while aliens are around, else 0 and 1.
pub fn FUN_004df710(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: bool) {
    let f = g.fish(this).clone();
    let board = board_of(g, this);
    let aliens = !objs(g, board, 0xb8).is_empty() || !objs(g, board, 0xf4).is_empty();
    let mut mirror = param_2;
    let row = if f.field_0x70 == 0 {
        if aliens { 2 } else { 0 }
    } else {
        mirror = 0 < f.field_0x70;
        if aliens { 3 } else { 1 }
    };
    let img = g.res.DAT_005e8aec;
    FUN_004560a0(gfx, g, img, 0, 0, &Rect::new(f.field_0x6c * 0x50, row * 0x50, 0x50, 0x50), mirror);
}

/// port: 004df810 FUN_004df810
/// Gash: turning (row 1), biting (row 2).
pub fn FUN_004df810(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: bool) {
    let f = g.fish(this).clone();
    let mut mirror = param_2;
    let row = if f.field_0x70 == 0 {
        if 0 < f.field_0x74 { 2 } else { 0 }
    } else {
        mirror = 0 < f.field_0x70;
        1
    };
    let img = g.res.DAT_005e8bfc;
    FUN_004560a0(gfx, g, img, 0, 0, &Rect::new(f.field_0x6c * 0x50, row * 0x50, 0x50, 0x50), mirror);
}

/// port: 004df890 FUN_004df890
/// Shrapnel, flashing white (additive) in the last 50 updates before its bomb.
pub fn FUN_004df890(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: bool) {
    let f = g.fish(this).clone();
    let (mirror, turn) = turning(g, this, param_2);
    let src = Rect::new(f.field_0x6c * 0x50, turn as i32 * 0x50, 0x50, 0x50);
    let img = g.res.DAT_005e8c2c;
    FUN_004560a0(gfx, g, img, 0, 0, &src, mirror);
    if f.field_0xd0 - 0x32 < f.field_0xcc {
        FUN_004558e0(gfx, true);
        FUN_004558c0(gfx, 1);
        let a = ftol(g.fish_type_pet(this).offset_0x2c.abs() * 255.0) as i32;
        FUN_00455890(gfx, CRect(0xff, 0xff, 0xff, a));
        let img = g.res.DAT_005e8c2c;
        FUN_004560a0(gfx, g, img, 0, 0, &src, mirror);
        FUN_004558e0(gfx, false);
        FUN_004558c0(gfx, 0);
    }
}

/// port: 004df980 FUN_004df980
/// Zorf: turning (row 1), dropping food (row 2, cel from the negative timer).
pub fn FUN_004df980(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: bool) {
    let f = g.fish(this).clone();
    let mut mirror = param_2;
    let mut col = f.field_0x6c;
    let mut row = 0;
    if f.field_0x70 == 0 {
        if f.field_0xcc < 0 {
            row = 2;
            col = f.field_0xcc + 10;
        }
    } else {
        mirror = 0 < f.field_0x70;
        row = 1;
    }
    let img = g.res.DAT_005e8e30;
    FUN_004560a0(gfx, g, img, 0, 0, &Rect::new(col * 0x50, row * 0x50, 0x50, 0x50), mirror);
}

/// port: 004dfa00 FUN_004dfa00
/// Walter: the sleep bubble during the cool-down (after its first 25 updates); turning
/// (row 1), punching (row 2).
pub fn FUN_004dfa00(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: bool) {
    let cool = g.fish_type_pet(this).offset_0x38;
    if 0 < cool && cool < DAT_005df034 - 0x19 {
        let img = g.res.DAT_005e8b44;
        FUN_00455d20(gfx, g, img, if param_2 { 0x32 } else { 0x1e }, -0xf);
    }
    let f = g.fish(this).clone();
    let mut mirror = param_2;
    let row = if f.field_0x70 == 0 {
        if 0 < f.field_0x74 { 2 } else { 0 }
    } else {
        mirror = 0 < f.field_0x70;
        1
    };
    let img = g.res.DAT_005e8e50;
    FUN_004560a0(gfx, g, img, 0, 0, &Rect::new(f.field_0x6c * 0x50, row * 0x50, 0x50, 0x50), mirror);
}

/// port: 004dfac0 FUN_004dfac0
/// Stanley: glowing (row 5) as it charges, firing (row 4), alert (rows 2/3) with aliens
/// around; else turning (row 1).
pub fn FUN_004dfac0(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: bool) {
    let f = g.fish(this).clone();
    let glow = g.fish_type_pet(this).offset_0x34;
    let board = board_of(g, this);
    let aliens = !objs(g, board, 0xb8).is_empty();
    let parts = !objs(g, board, 0xf4).is_empty();
    let mut mirror = param_2;
    let row;
    if aliens || parts {
        if glow < 10 {
            row = 5;
        } else if f.field_0x74 != 0 {
            row = 4;
        } else if f.field_0x70 == 0 {
            row = 2;
        } else {
            row = 3;
            mirror = -1 < f.field_0x70;
        }
    } else if glow != 0 {
        row = 5;
    } else if f.field_0x70 == 0 {
        row = 0;
    } else {
        row = 1;
        mirror = -1 < f.field_0x70;
    }
    let img = g.res.DAT_005e8d00;
    FUN_004560a0(gfx, g, img, 0, 0, &Rect::new(f.field_0x6c * 0x50, row * 0x50, 0x50, 0x50), mirror);
}

/// port: 004dfbb0 FUN_004dfbb0
/// A plain pet strip (`param_2`): the sleep bubble when asleep, swimming or turning.
pub fn FUN_004dfbb0(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: Ptr, param_3: bool) {
    if g.fish_type_pet(this).offset_0x19 {
        let img = g.res.DAT_005e8b44;
        FUN_00455d20(gfx, g, img, if param_3 { 0x32 } else { 0x1e }, -10);
    }
    let f = g.fish(this).clone();
    let (mirror, turn) = turning(g, this, param_3);
    FUN_004560a0(gfx, g, param_2, 0, 0, &Rect::new(f.field_0x6c * 0x50, turn as i32 * 0x50, 0x50, 0x50), mirror);
}

/// port: 004e4e40 FUN_004e4e40
/// Wadsworth: opening and closing his mouth around the shelter (row 2), turning (row 1);
/// the sleep bubble while not sheltering with aliens (or +0xdc objects) around, outside
/// the virtual tank.
pub fn FUN_004e4e40(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: bool) {
    let f = g.fish(this).clone();
    let shelter = g.fish_type_pet(this).offset_0x24;
    let mut mirror = param_2;
    let mut col = f.field_0x6c;
    let t = f.field_0xcc;
    let mut row = 0;
    if t < 1 {
        if f.field_0x70 != 0 {
            mirror = 0 < f.field_0x70;
            row = 1;
        }
    } else {
        row = 2;
        let mut set = false;
        if !shelter {
            if 0x14 < t {
                col = 9 - (0x1d - t) / 2;
                set = true;
            }
        } else if f.field_0xd0 - 10 < t {
            col = 9 - ((f.field_0xd0 - t) - 1) / 2;
            set = true;
        }
        if !set {
            col = if t < 0xb { (t - 1) / 2 } else { 5 };
        }
    }
    let img = g.res.DAT_005e8b30;
    FUN_004560a0(gfx, g, img, 0, 0, &Rect::new(col * 0x50, row * 0x50, 0x50, 0x50), mirror);
    if !shelter {
        let board = board_of(g, this);
        if objs(g, board, 0xb8).is_empty() && objs(g, board, 0xf4).is_empty() && objs(g, board, 0xdc).is_empty() {
            return;
        }
        if g.board(board).field_0x33c != 5 {
            let img = g.res.DAT_005e8b44;
            FUN_00455d20(gfx, g, img, 0x28, -0xf);
        }
    }
}

/// port: 004fb680 FUN_004fb680
/// The pet's own click (true when handled): a right click is Presto's transform (when
/// charged) or the tank's; Amp, charged, takes two more clicks to charge up, then zaps
/// every guppy of the level and every hatched Gash (coins for up to `(n - 20) / 2 + 20`
/// of them, sparkles for all), recharging 200 slower; Walter (idle) throws a punch the way
/// it faces, resting 360 updates after five.
pub fn FUN_004fb680(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32) -> bool {
    let board = board_of(g, this);
    if param_3 < 0 {
        if !g.fish_type_pet(this).offset_0x4 {
            let (mx, my) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
            crate::game::board_level::FUN_0053bfb0(g, board, mx + param_1, my + param_2);
            return true;
        }
        let charge = g.fish_type_pet(this).offset_0xc;
        if crate::game::other_pet::FUN_004f2650(g, this, charge) {
            let app = app_of(g, this);
            crate::game::pet_dialog::FUN_0054c6f0(g, app, this);
        }
        return true;
    }
    let p = pet(g, this);
    let f = g.fish(this).clone();
    if p == 0x10 && f.field_0xd0 <= f.field_0xcc {
        if g.fish_type_pet(this).offset_0x34 < 2 {
            g.fish_type_pet(this).offset_0x34 += 1;
            crate::game::board::FUN_00538230(g, board, 0x11b, 3, 1.0);
            return true;
        }
        g.w(this).offset_0x1 = g.globals.DAT_005e8f14;
        g.w(this).offset_0x28 = false;
        vcall!(g, this, w.vfunction39, false);
        g.fish_type_pet(this).offset_0x2c = 0.0;
        let mut killed = 0;
        let mut zapped: Vec<Ptr> = Vec::new();
        let mut n = 0;
        let v = objs(g, board, 0xa0);
        let mut it = FUN_004e5040(&v);
        while it != v.len() {
            if g.go(v[it]).offset_0x24 < 0 {
                n += 1;
                let o = FUN_004e3e00(&v, it);
                crate::game::board_level::FUN_005420e0(&mut zapped, o);
            }
            it += 1;
        }
        let mut cap = (n - 0x14) / 2 + 0x14;
        if cap < 0x14 {
            cap = 0x14;
        }
        for o in zapped {
            vcall!(g, o, fish.vfunction89, false);
            if killed < cap {
                let f = g.fish(o).clone();
                let x = ftol(f.field_0x4 + 15.0) as i32;
                let y = ftol(f.field_0xc + 10.0) as i32;
                crate::game::board_level::FUN_00544430(g, board, x, y, 5, NULL, -1.0, 0);
            }
            let k = (rand(g, this) % 3 + 3) as i32;
            let f = g.fish(o).clone();
            let y = ftol(f.field_0xc) as i32;
            let x = ftol(f.field_0x4) as i32;
            crate::game::board_level::FUN_005436f0(g, board, x, y, k);
            killed += 1;
        }
        let mut gashes: Vec<Ptr> = Vec::new();
        for o in objs(g, board, 0xb4) {
            let d = g.fish_type_pet(o).clone();
            if !d.offset_0x4 && d.offset_0x8 == 0x15 && 0 < d.offset_0x14 {
                crate::game::board_level::FUN_005420e0(&mut gashes, o);
            }
        }
        for o in gashes {
            crate::game::alien::FUN_004d6830(g, o, true);
            let f = g.fish(o).clone();
            let x = ftol(f.field_0x4 + 15.0) as i32;
            let y = ftol(f.field_0xc + 10.0) as i32;
            crate::game::board_level::FUN_00544430(g, board, x, y, 5, NULL, -1.0, 0);
            let k = (rand(g, this) % 3 + 3) as i32;
            let f = g.fish(o).clone();
            let y = ftol(f.field_0xc) as i32;
            let x = ftol(f.field_0x4) as i32;
            crate::game::board_level::FUN_005436f0(g, board, x, y, k);
            killed += 1;
        }
        if !crate::game::board_update::FUN_005392a0(g, board) && mode(g, this) != 5 {
            crate::game::board_level::FUN_00546d70(g, board);
        }
        crate::game::board::FUN_00538230(g, board, 0x119, 3, 1.0);
        if 0 < killed {
            crate::game::board_level::FUN_00538610(g, board, -1);
        }
        if mode(g, this) != 5 {
            g.fish(this).field_0xd0 += 200;
        }
        g.fish(this).field_0xcc = -0x14;
        g.fish_type_pet(this).offset_0x34 = 0;
        return true;
    }
    let d = g.fish_type_pet(this).clone();
    if p != 0x17 || f.field_0x74 != 0 || d.offset_0x20 != NULL || d.offset_0x38 != 0 || f.field_0x70 != 0 {
        return false;
    }
    let vx = f.offset_0x14;
    let k = if vx < 0.0 { 1 } else { ftol(vx) as i32 };
    let glove = if vx < 0.0 || (k == 0 && f.offset_0x34 < 0.0) {
        BoxingGlove__004ef7c0(g, this, false)
    } else if 0.0 < vx || (k == 0 && 0.0 < f.offset_0x34) {
        BoxingGlove__004ef7c0(g, this, true)
    } else {
        NULL
    };
    if glove != NULL {
        g.fish_type_pet(this).offset_0x20 = glove;
    }
    if g.fish_type_pet(this).offset_0x20 != NULL {
        if mode(g, this) != 5 {
            g.fish_type_pet(this).offset_0x34 += 1;
            if 5 <= g.fish_type_pet(this).offset_0x34 {
                g.fish_type_pet(this).offset_0x38 = DAT_005df034;
                g.fish_type_pet(this).offset_0x34 = 0;
                g.w(this).offset_0x28 = false;
                g.fish(this).field_0x58 = 0;
                vcall!(g, this, w.vfunction39, false);
            }
        }
        g.fish(this).field_0x6c = 0;
        g.fish(this).field_0x74 = 0x28;
        crate::game::board::FUN_00538230(g, board, 0x136, 3, 1.0);
        crate::game::board_level::FUN_0053aa40(g, board);
    }
    true
}

fn alloc_glove(g: &mut G, wc: crate::sexy::object::WidgetContainer_data, w: crate::sexy::object::Widget_data,
               go: crate::game::game_object::GameObject_data, d: BoxingGlove_data) -> Ptr {
    g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__BoxingGlove_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::GameObject(Box::new(GameObjectExt { go, sub: GoSub::BoxingGlove(d) })) }),
    })
}

/// port: 004ef7c0 Sexy::BoxingGlove::BoxingGlove
/// `BoxingGlove(FishTypePet* pet, bool right)`: 160x80, off screen until its first
/// update, 30 updates.
pub fn BoxingGlove__004ef7c0(g: &mut G, param_1: Ptr, param_2: bool) -> Ptr {
    let (mut wc, mut w, go) = crate::game::game_object::GameObject(g);
    wc.offset_0x2c = -200;
    wc.offset_0x30 = -200;
    w.offset_0x1 = false;
    wc.offset_0x34 = 0xa0;
    wc.offset_0x38 = 0x50;
    alloc_glove(g, wc, w, go, BoxingGlove_data { offset_0x4: param_1, offset_0x8: 0x1e, offset_0xc: param_2, offset_0x10: 0 })
}

/// port: 004e9a60 Sexy::BoxingGlove::vfunction23
/// `Update()` (not while paused): follows Walter (once its wind-up is over), counts down,
/// and while extended punches every fish-like creature (guppies, the other fish, pets
/// that can be pushed) in reach; a hit plays the punch sound (every 5 updates at most).
pub fn vfunction23__004e9a60(g: &mut G, this: Ptr) {
    let app = app_of(g, this);
    let board = g.wfa(app).offset_0x4;
    if board == NULL || g.board(board).field_0x8 {
        return;
    }
    let owner = g.boxing_glove(this).offset_0x4;
    let right = g.boxing_glove(this).offset_0xc;
    if owner != NULL {
        if 0x23 < g.fish(owner).field_0x74 {
            return;
        }
        let (ox, oy) = (g.wc(owner).offset_0x2c, g.wc(owner).offset_0x30);
        g.wc(this).offset_0x2c = if !right { ox - 0x3c } else { ox - 0x14 };
        g.wc(this).offset_0x30 = oy;
    }
    g.boxing_glove(this).offset_0x8 -= 1;
    let t = g.boxing_glove(this).offset_0x8;
    if 0 < t {
        let d = g.boxing_glove(this);
        if 0 < d.offset_0x10 {
            d.offset_0x10 -= 1;
        }
        let mut hit = false;
        if 0xc < t {
            let reach = if 0x19 < t { 0 } else if 0x14 < t { 0x1e } else { 0x3c };
            let (mx, my) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
            let px = if right { mx + reach + 0x50 } else { mx - reach + 0x50 };
            let py = my + 0x28;
            let dir = right as i32 * 2 - 1;
            let set: Vec<Ptr> = g.board(board).offset_0x7c.iter().copied().collect();
            for o in set {
                if o == owner || !g.go(o).offset_0x6c {
                    continue;
                }
                let amount = match g.go(o).offset_0x4 {
                    0 | 5 | 6 | 7 | 0x22 | 0x23 | 0x24 => Some(5),
                    0x15 => match g.fish_type_pet(o).offset_0x8 {
                        3 => None,
                        6 | 0xb | 0x10 => Some(5),
                        0x15 => Some(10),
                        _ => Some(-1),
                    },
                    _ => None,
                };
                if let Some(a) = amount {
                    if FUN_004e0ed0(g, o, px, py, dir, a) {
                        hit = true;
                    }
                }
            }
            if hit && g.boxing_glove(this).offset_0x10 == 0 {
                crate::game::board_level::FUN_00538340(g, board, 3);
                g.boxing_glove(this).offset_0x10 = 5;
            }
        }
    }
}

/// port: 004e1000 Sexy::BoxingGlove::vfunction27
/// `Draw(Graphics*)`: the arm (cels 1..6, extending then retracting) and the glove.
pub fn vfunction27__004e1000(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    let d = g.boxing_glove(this).clone();
    let t = d.offset_0x8;
    let e = if t < 0x10 { (t - 0xf).abs() } else { t - 0xf };
    let mut cel = 9 - e;
    let img = g.res.DAT_005e8e50;
    if 0 < cel {
        if cel <= 5 {
            cel = 6;
        }
        let x = if d.offset_0xc { 0x53 } else { -3 };
        FUN_004560a0(gfx, g, img, x, 2, &Rect::new(cel * 0x50, 0xa0, 0x50, 0x50), d.offset_0xc);
    }
    let src = Rect::new(400, 0xa0, 0x50, 0x50);
    if !d.offset_0xc {
        FUN_004560a0(gfx, g, img, e * 6 - 0x14, 0, &src, false);
        return;
    }
    FUN_004560a0(gfx, g, img, e * -6 + 100, 0, &src, true);
}

/// port: 004e0ed0 FUN_004e0ed0
/// A punch at (`param_x`, `param_y`) on a fish-like creature (not a Presto just
/// appeared): within 40 pixels of its center it is knocked `dir` * 15 or 10 (random)
/// for 50 updates and, by `amount`, made hungry. True when hit.
pub fn FUN_004e0ed0(g: &mut G, target: Ptr, param_x: i32, param_y: i32, param_1: i32, param_2: i32) -> bool {
    let f = g.fish(target).clone();
    let fresh_presto = g.go(target).offset_0x4 == 0x15 && g.fish_type_pet(target).offset_0x4 && g.wc(target).offset_0x24 < 0x1e;
    if f.offset_0xb8 < 1 && !fresh_presto {
        let wc = g.wc(target).clone();
        let cx = wc.offset_0x34 / 2 + wc.offset_0x2c;
        let cy = wc.offset_0x38 / 2 + wc.offset_0x30;
        if param_x - 0x28 < cx && cx < param_x + 0x28 && param_y - 0x28 < cy && cy < param_y + 0x28 {
            if 0 < param_2 {
                let f = g.fish(target);
                f.field_0xcc = f.field_0xd0 - param_2;
            }
            let f = g.fish(target);
            f.offset_0xb8 = 0x32;
            f.offset_0xbc = 0xb4;
            let msg = g.go(target).offset_0x94;
            if msg != -1 {
                let board = board_of(g, target);
                let m = g.board(board).offset_0x4;
                crate::game::timed_messages::FUN_005156f0(g, m, msg);
            }
            crate::game::game_object::FUN_004d6cc0(g, target);
            let r = crate::sexy::sexy_app_base::FUN_0040f9e0(g);
            let even = (r as i32 & (0x80000001u32 as i32)) == 0;
            g.fish(target).offset_0xc4 = (param_1 * if even { 0xf } else { 10 }) as f64;
            return true;
        }
    }
    false
}

/// The pet id (+0x234) of a `FishTypePet`.
pub fn pet_id(g: &mut G, this: Ptr) -> i32 {
    g.fish_type_pet(this).offset_0x8
}
