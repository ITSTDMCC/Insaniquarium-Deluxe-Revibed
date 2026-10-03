//! `Sexy::Missle` (sic): the homing missiles. Kind 0 is an alien's missile at a fish,
//! kind 1 the alien's at a pet, kind 2 Stanley's at an alien; kinds 3..5 fly straight
//! (no target). `Missle_data` starts at object offset 0x154 (field names relative to it);
//! the object is 0x1b8 bytes.

use crate::game::board_level::vec_index;
use crate::sexy::crt::ftol;
use crate::sexy::graphics::{FUN_00455890, FUN_004558c0, FUN_004558e0, FUN_00455e40, FUN_004560a0, FUN_00456950, FUN_00456980};
use crate::sexy::prelude::*;
use crate::sexy::types::CRect;

/// `Missle_data` (object offset 0x154).
#[derive(Debug, Clone, Default)]
pub struct Missle_data {
    /// +0x158 x.
    pub offset_0x4: f64,
    /// +0x160 y.
    pub offset_0xc: f64,
    /// +0x168 x speed.
    pub offset_0x14: f64,
    /// +0x170 y speed.
    pub offset_0x1c: f64,
    /// +0x178.
    pub field_0x24: i32,
    /// +0x17c launch updates left (15).
    pub offset_0x28: i32,
    /// +0x180 spin counter (Stanley's missile).
    pub offset_0x2c: i32,
    /// +0x188 speed divisor (0.8; 0.4 for Stanley's).
    pub offset_0x34: f64,
    /// +0x190 flying free (no longer homing; set for the straight kinds and on a hit).
    pub offset_0x3c: bool,
    /// +0x194 cel.
    pub offset_0x40: i32,
    /// +0x198 cel row cycles (the straight kinds flip each cycle).
    pub offset_0x44: i32,
    /// +0x19c kind (0..5).
    pub offset_0x48: i32,
    /// +0x1a0 variant (0..3, random).
    pub offset_0x4c: i32,
    /// +0x1a4 the target.
    pub offset_0x50: Ptr,
    /// +0x1a8 bolt updates (it leaves when this reaches +0x1ac).
    pub offset_0x54: i32,
    /// +0x1ac bolt length (10).
    pub offset_0x58: i32,
    /// +0x1b0 bolt start x.
    pub offset_0x5c: i32,
    /// +0x1b4 bolt start y.
    pub offset_0x60: i32,
}

impl G {
    pub fn missle(&mut self, p: Ptr) -> &mut Missle_data {
        match &mut self.go_ext(p).sub {
            GoSub::Missle(d) => d,
            s => panic!("{p} is not a Missle: {s:?}"),
        }
    }
}

fn alloc_missle(g: &mut G, wc: crate::sexy::object::WidgetContainer_data, w: crate::sexy::object::Widget_data,
                go: crate::game::game_object::GameObject_data, d: Missle_data) -> Ptr {
    g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__Missle_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::GameObject(Box::new(GameObjectExt { go, sub: GoSub::Missle(d) })) }),
    })
}

fn board_of(g: &mut G, this: Ptr) -> Ptr {
    let app = g.go(this).offset_0x0;
    g.wfa(app).offset_0x4
}

fn rand(g: &mut G, this: Ptr) -> u32 {
    let app = g.go(this).offset_0x0;
    let r = g.wfa(app).offset_0x84;
    crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(r))
}

fn objs(g: &mut G, board: Ptr, off: usize) -> Vec<Ptr> {
    g.board(board).offset_0xc[vec_index(off)].clone()
}

/// port: 004eae00 Sexy::Missle::Missle
/// `Missle::Missle()` (used before loading from a save).
pub fn Missle__004eae00(g: &mut G) -> Ptr {
    let (mut wc, w, mut go) = crate::game::game_object::GameObject(g);
    wc.offset_0x3d = false;
    go.offset_0x4 = 0x1e;
    alloc_missle(g, wc, w, go, Missle_data { offset_0x50: NULL, offset_0x54: 0, ..Default::default() })
}

/// port: 004eae40 Sexy::Missle::Missle
/// `Missle(int x, int y, GameObject* target, int kind)`: 80x80 (50x50 for Stanley's),
/// ignoring the mouse, a random variant; the straight kinds get a random speed of 4..6
/// either way on both axes.
pub fn Missle__004eae40(g: &mut G, param_1: i32, param_2: i32, param_3: Ptr, param_4: i32) -> Ptr {
    let (mut wc, mut w, mut go) = crate::game::game_object::GameObject(g);
    go.offset_0x4 = 0x1e;
    wc.offset_0x3d = false;
    let mut d = Missle_data { offset_0x4: param_1 as f64, offset_0xc: param_2 as f64, offset_0x48: param_4, offset_0x50: param_3, ..Default::default() };
    wc.offset_0x2c = ftol(d.offset_0x4) as i32;
    wc.offset_0x30 = ftol(d.offset_0xc) as i32;
    d.offset_0x14 = 0.0;
    d.offset_0x1c = 0.0;
    wc.offset_0x34 = 0x50;
    wc.offset_0x38 = 0x50;
    d.offset_0x40 = 1;
    d.offset_0x44 = 1;
    d.offset_0x54 = 0;
    d.offset_0x58 = 10;
    d.offset_0x28 = 0xf;
    d.offset_0x2c = 0;
    let r = crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32;
    d.offset_0x4c = r % 4;
    w.offset_0x1 = false;
    if d.offset_0x48 == 2 {
        d.offset_0x34 = f64::from_bits(0x3fd999999999999a);
        wc.offset_0x34 = 0x32;
        wc.offset_0x38 = 0x32;
    } else {
        d.offset_0x34 = f64::from_bits(0x3fe999999999999a);
    }
    d.offset_0x3c = false;
    let this = alloc_missle(g, wc, w, go, d);
    if !FUN_004d80c0(g, this) {
        return this;
    }
    const SPEEDS: [f64; 6] = [-6.0, -5.0, -4.0, 4.0, 5.0, 6.0];
    let r1 = (rand(g, this) % 6) as usize;
    let r2 = (rand(g, this) % 6) as usize;
    let d = g.missle(this);
    d.offset_0x14 = SPEEDS[r1];
    d.offset_0x1c = SPEEDS[r2];
    d.offset_0x3c = true;
    this
}

/// port: 004d80c0 FUN_004d80c0
/// One of the straight kinds (3..5).
pub fn FUN_004d80c0(g: &mut G, this: Ptr) -> bool {
    let k = g.missle(this).offset_0x48;
    k == 3 || k == 4 || k == 5
}

/// port: 004d7fa0 Sexy::Missle::vfunction81
/// `Sync(DataSync&)`: the bolt fields only while a bolt runs; the target by id.
pub fn vfunction81(g: &mut G, this: Ptr, sync: &mut crate::sexy::data_sync::DataSync) -> crate::sexy::data_sync::SyncResult {
    crate::game::game_object::vfunction81(g, this, sync)?;
    use crate::sexy::data_sync::{FUN_00503010, FUN_00503070, FUN_005030a0};
    let mut d = g.missle(this).clone();
    FUN_005030a0(sync, &mut d.offset_0x4)?;
    FUN_005030a0(sync, &mut d.offset_0xc)?;
    FUN_005030a0(sync, &mut d.offset_0x14)?;
    FUN_005030a0(sync, &mut d.offset_0x1c)?;
    FUN_00503010(sync, &mut d.field_0x24)?;
    FUN_00503010(sync, &mut d.offset_0x28)?;
    FUN_00503010(sync, &mut d.offset_0x2c)?;
    FUN_005030a0(sync, &mut d.offset_0x34)?;
    FUN_00503070(sync, &mut d.offset_0x3c)?;
    FUN_00503010(sync, &mut d.offset_0x40)?;
    FUN_00503010(sync, &mut d.offset_0x44)?;
    FUN_00503010(sync, &mut d.offset_0x48)?;
    FUN_00503010(sync, &mut d.offset_0x4c)?;
    FUN_00503010(sync, &mut d.offset_0x54)?;
    if 0 < d.offset_0x54 {
        FUN_00503010(sync, &mut d.offset_0x58)?;
        FUN_00503010(sync, &mut d.offset_0x5c)?;
        FUN_00503010(sync, &mut d.offset_0x60)?;
    }
    crate::sexy::data_sync::FUN_005122f0(sync, crate::sexy::data_sync::PtrSlot::MissleTarget(this));
    *g.missle(this) = d;
    Ok(())
}

/// port: 004d80e0 Sexy::Missle::vfunction76
/// `Explode()`: lets go of the target, leaves the tank (a homing missile gone may end the
/// alien alert), and, except Stanley's, explodes (the bigger sound for kind 1) in a few
/// sparkles.
pub fn vfunction76(g: &mut G, this: Ptr) {
    let t = g.missle(this).offset_0x50;
    if t != NULL {
        g.go(t).offset_0x10 = NULL;
        g.missle(this).offset_0x50 = NULL;
    }
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    let mgr = g.wc(board).offset_0xc;
    vcall!(g, mgr, w.vfunction5, this);
    crate::sexy::sexy_app_base::vfunction35(g, app, this);
    crate::game::board_level::FUN_00541d90(g, board, this, false);
    if !FUN_004d80c0(g, this) {
        crate::game::alien::FUN_00539c90(g, board);
    }
    let kind = g.missle(this).offset_0x48;
    if kind != 2 {
        if kind == 1 {
            crate::game::board::FUN_00538230(g, board, 0x11f, 3, 1.0);
        }
        crate::game::board::FUN_00538230(g, board, 0x11d, 3, 1.0);
        let mut n = rand(g, this) % 3 + 2;
        while n != 0 {
            let k = (rand(g, this) % 3) as i32 + 3;
            let my = g.wc(this).offset_0x30;
            let y = (rand(g, this) % 0x1e) as i32 - 10 + my;
            let mx = g.wc(this).offset_0x2c;
            let x = (rand(g, this) % 0x1e) as i32 - 10 + mx;
            crate::game::board_level::FUN_005436f0(g, board, x, y, k);
            n -= 1;
        }
    }
}

/// port: 004d8260 Sexy::Missle::vfunction75
/// `Removed()`: lets go of the target; a homing missile gone may end the alien alert.
pub fn vfunction75(g: &mut G, this: Ptr) {
    let t = g.missle(this).offset_0x50;
    if t != NULL {
        g.go(t).offset_0x10 = NULL;
        g.missle(this).offset_0x50 = NULL;
    }
    if !FUN_004d80c0(g, this) {
        let board = board_of(g, this);
        crate::game::alien::FUN_00539c90(g, board);
    }
}

/// port: 004f93a0 Sexy::Missle::vfunction78
/// `HitTarget()`: the target dies (a starcatcher, guppycruncher or breeder its own way, a
/// fish-like creature by vfunction 89, a pet just leaves) and the missile explodes.
pub fn vfunction78(g: &mut G, this: Ptr, _param_1: i32) {
    let t = g.missle(this).offset_0x50;
    g.go(t).offset_0x10 = NULL;
    match g.go(t).offset_0x4 {
        8 => {
            crate::game::penta::FUN_004f3c60(g, t, true);
            vfunction76(g, this);
            return;
        }
        9 => {
            crate::game::grubber::FUN_004f2890(g, t, true);
            vfunction76(g, this);
            return;
        }
        5 | 7 | 6 | 0 => {
            vcall!(g, t, fish.vfunction89, true);
        }
        10 => {
            crate::game::breeder::FUN_004dd5d0(g, t, true);
            vfunction76(g, this);
            return;
        }
        0x14 | 0x15 => {
            crate::game::other_pet::vfunction76(g, t);
            vfunction76(g, this);
            return;
        }
        _ => {}
    }
    vfunction76(g, this);
}

/// The homing step toward `t` along one axis: four bands around the target (far, near,
/// close, centre) with their speed limits.
fn home(v: &mut f64, at: f64, lo_far: i32, hi_far: i32, lo_near: i32, hi_near: i32, c: i32, far: f64, near: f64, close: f64) {
    if at <= hi_far as f64 {
        if lo_far as f64 <= at {
            if at <= hi_near as f64 {
                if at < lo_near as f64 {
                    if *v < near {
                        *v += 0.2;
                    }
                } else if at <= c as f64 {
                    if at < c as f64 && *v < close {
                        *v += 0.2;
                    }
                } else if -close < *v {
                    *v -= 0.2;
                }
            } else if -near < *v {
                *v -= 0.2;
            }
        } else if *v < far {
            *v += 0.2;
        }
    } else if -far < *v {
        *v -= 0.2;
    }
}

/// The vertical homing step: like `home` but the centre band eases by 0.1 (within 0.2).
fn home_y(v: &mut f64, at: f64, lo_far: i32, hi_far: i32, lo_near: i32, hi_near: i32, c: i32) {
    if (hi_far as f64) < at {
        if -1.8 < *v {
            *v -= 0.2;
        }
    } else if at < lo_far as f64 {
        if *v < 1.8 {
            *v += 0.2;
        }
    } else if (hi_near as f64) < at {
        if -1.4 < *v {
            *v -= 0.2;
        }
    } else if at < lo_near as f64 {
        if *v < 1.4 {
            *v += 0.2;
        }
    } else if at <= c as f64 {
        if at < c as f64 && *v < 0.2 {
            *v += 0.1;
        }
    } else if -0.2 < *v {
        *v -= 0.1;
    }
}

/// port: 004ff4e0 FUN_004ff4e0
/// `Home()`: a homing missile whose target is gone explodes (the straight kinds fly on);
/// otherwise it steers toward the target (wider for the ultravore; Stanley's aims 25 in at
/// the alien, 40 at an alien part, only with aliens around), then checks for a hit. True
/// when the missile is gone.
pub fn FUN_004ff4e0(g: &mut G, this: Ptr) -> bool {
    let t = g.missle(this).offset_0x50;
    let kind = g.missle(this).offset_0x48;
    'steer: {
        if (t == NULL || g.go(t).offset_0x10 == NULL) && kind != 2 {
            if !FUN_004d80c0(g, this) {
                vfunction76(g, this);
                return true;
            }
            break 'steer;
        }
        if g.missle(this).offset_0x3c {
            break 'steer;
        }
        let board = board_of(g, this);
        let parts = !objs(g, board, 0xf4).is_empty();
        let k = if kind == 2 {
            if !parts && !crate::game::board_update::FUN_004da780(g, board) {
                break 'steer;
            }
            if parts { 0x28 } else { 0x19 }
        } else {
            0x28
        };
        let (tx, ty) = (g.wc(t).offset_0x2c, g.wc(t).offset_0x30);
        let wide = g.go(t).offset_0x4 == 6 || (kind == 2 && !parts);
        let d = g.missle(this);
        if wide {
            let x = d.offset_0x4 + k as f64;
            home(&mut d.offset_0x14, x, tx - 0x28, tx + 200, tx + 0x32, tx + 0x6e, tx + 0x50, 1.8, 1.4, 0.6);
            let y = d.offset_0xc + k as f64;
            home_y(&mut d.offset_0x1c, y, ty - 0x28, ty + 200, ty + 0x32, ty + 0x6e, ty + 0x50);
        } else {
            let x = d.offset_0x4 + 40.0;
            home(&mut d.offset_0x14, x, tx, tx + 0x50, tx + 0x14, tx + 0x3c, tx + 0x28, 1.8, 1.4, 0.6);
            let y = d.offset_0xc + 40.0;
            home_y(&mut d.offset_0x1c, y, ty, ty + 0x50, ty + 0x14, ty + 0x3c, ty + 0x28);
        }
    }
    if t != NULL {
        return FUN_004fc070(g, this);
    }
    false
}

/// `|(x + o) - (obj.mX + lo..hi)|`: inside the box `lo < x + o < hi` (and the same for y).
fn inside(g: &mut G, this: Ptr, o: Ptr, off: f64, lo: i32, hi: i32, ylo: i32, yhi: i32) -> bool {
    let d = g.missle(this).clone();
    let (ox, oy) = (g.wc(o).offset_0x2c, g.wc(o).offset_0x30);
    let x = d.offset_0x4 + off;
    let y = d.offset_0xc + off;
    x < (ox + hi) as f64 && ((ox + lo) as f64) < x && y < (oy + yhi) as f64 && ((oy + ylo) as f64) < y
}

/// port: 004fc070 FUN_004fc070
/// `CheckHit()`: an alien's missile on its target (the ultravore's box is larger) hits it
/// (vfunction 78); Stanley's missile on its alien (or alien part) bounces off it and hurts
/// it (15, or 55 for the variants 4+), killing at 0; a kind-1 missile flying free (outside
/// the +0x880 mode) hurts the first alien, alien part or boss it meets (30), and with the
/// boss around destroys a pet it meets, else kills the first creature of each kind it
/// meets. True when the missile is gone.
pub fn FUN_004fc070(g: &mut G, this: Ptr) -> bool {
    let app = g.go(this).offset_0x0;
    let kind = g.missle(this).offset_0x48;
    if kind != 1 || !g.missle(this).offset_0x3c || g.wfa(app).offset_0x154 {
        if kind != 2 {
            if FUN_004d80c0(g, this) {
                return false;
            }
            let t = g.missle(this).offset_0x50;
            let hit = if g.go(t).offset_0x4 == 6 { inside(g, this, t, 40.0, 8, 0x8c, 4, 0x8c) } else { inside(g, this, t, 40.0, 10, 0x46, 10, 0x46) };
            if !hit {
                return false;
            }
            vcall!(g, this, go.vfunction78, 0);
            return true;
        }
        if g.missle(this).offset_0x3c {
            return false;
        }
        let t = g.missle(this).offset_0x50;
        let part = g.go(t).offset_0x4 == 0x17;
        let hit = if part { inside(g, this, t, 25.0, 10, 0x46, 10, 0x46) } else { inside(g, this, t, 25.0, 0x14, 0x8c, 0x14, 0x8c) };
        if !hit {
            return false;
        }
        g.missle(this).offset_0x3c = true;
        g.go(t).offset_0x10 = NULL;
        g.missle(this).offset_0x50 = NULL;
        let board = board_of(g, this);
        let (div, dmg) = if g.missle(this).offset_0x4c < 4 {
            crate::game::board::FUN_00538230(g, board, 0x136, 3, 1.0);
            (1.5, 0xf)
        } else {
            crate::game::board_level::FUN_00538340(g, board, 3);
            (f64::from_bits(0x3fe3333333333333), 0x37)
        };
        {
            let d = g.missle(this);
            d.offset_0x34 = div;
            d.offset_0x14 = -d.offset_0x14;
            d.offset_0x1c = -d.offset_0x1c;
        }
        if part {
            let body = crate::game::bilaterus::alien_part_body(g, t);
            if body == NULL {
                return false;
            }
            if 0.0 < crate::game::bilaterus::alien_part_body_hurt(g, t, dmg as f64) {
                return false;
            }
            crate::game::bilaterus::FUN_004fb4c0(g, body, true);
            return false;
        }
        if g.go(t).offset_0x4 != 0x16 {
            return false;
        }
        if g.alien(t).offset_0xb0 {
            return false;
        }
        g.alien(t).offset_0x9c -= dmg as f64;
        if 0.0 < g.alien(t).offset_0x9c {
            return false;
        }
        crate::game::alien::FUN_004f9c70(g, t, true);
        return false;
    }
    let board = board_of(g, this);
    for a in objs(g, board, 0xb8) {
        if inside(g, this, a, 40.0, 0x1e, 0x8c, 10, 0x96) && !g.alien(a).offset_0xb0 {
            let al = g.alien(a);
            al.offset_0x9c -= 30.0;
            al.offset_0xac = 10;
            if g.alien(a).offset_0x9c <= 0.0 {
                crate::game::alien::FUN_004f9c70(g, a, true);
            }
            vfunction76(g, this);
            return true;
        }
    }
    for p in objs(g, board, 0xf4) {
        let body = crate::game::bilaterus::alien_part_body(g, p);
        if inside(g, this, body, 25.0, 10, 0x46, 10, 0x46) {
            let hp = crate::game::bilaterus::alien_part_body_hurt(g, p, 30.0);
            crate::game::bilaterus::alien_part_body_flash(g, p, 10);
            if hp <= 0.0 {
                crate::game::bilaterus::FUN_004fb4c0(g, body, true);
            }
            vfunction76(g, this);
            return true;
        }
    }
    let boss = g.board(board).field_0x84;
    if boss != NULL {
        if inside(g, this, boss, 40.0, 0x1e, 0x8c, 10, 0x96) {
            let b = g.board(board).field_0x84;
            g.alien(b).offset_0x9c -= 30.0;
            g.alien(b).offset_0xac = 10;
            if g.alien(b).offset_0x9c <= 0.0 {
                crate::game::alien::FUN_004f9c70(g, b, true);
            }
            vfunction76(g, this);
            return true;
        }
        let mut hit_fish_pet = false;
        for p in objs(g, board, 0xb4) {
            if inside(g, this, p, 40.0, 10, 0x46, 10, 0x46) {
                crate::game::other_pet::vfunction76(g, p);
                crate::game::board_level::FUN_00538610(g, board, -1);
                hit_fish_pet = true;
                break;
            }
        }
        let _ = hit_fish_pet;
        for p in objs(g, board, 0xb0) {
            if inside(g, this, p, 40.0, 10, 0x46, 10, 0x46) {
                crate::game::other_pet::vfunction76(g, p);
                crate::game::board_level::FUN_00538610(g, board, -1);
                return false;
            }
        }
        return false;
    }
    for f in objs(g, board, 0xa0) {
        if g.go(f).offset_0x24 < 0 {
            if g.globals.DAT_005e89d0 != 0 && g.fish(f).offset_0x4c < 2 {
                continue;
            }
            if inside(g, this, f, 40.0, 10, 0x46, 10, 0x46) {
                vcall!(g, f, fish.vfunction89, true);
                break;
            }
        }
    }
    for f in objs(g, board, 0xa4) {
        if g.go(f).offset_0x24 < 0 && inside(g, this, f, 40.0, 10, 0x46, 10, 0x46) {
            vcall!(g, f, fish.vfunction89, true);
            break;
        }
    }
    for f in objs(g, board, 0xd0) {
        if g.go(f).offset_0x24 < 0 && inside(g, this, f, 40.0, 10, 0x46, 10, 0x46) {
            crate::game::penta::FUN_004f3c60(g, f, true);
            break;
        }
    }
    for f in objs(g, board, 0xd4) {
        if g.go(f).offset_0x24 < 0 && inside(g, this, f, 40.0, 10, 0x96, 10, 0x96) {
            vcall!(g, f, fish.vfunction89, true);
            break;
        }
    }
    for f in objs(g, board, 0xc4) {
        if g.go(f).offset_0x24 < 0 && inside(g, this, f, 40.0, 10, 0x46, 10, 0x46) {
            crate::game::grubber::FUN_004f2890(g, f, true);
            break;
        }
    }
    for f in objs(g, board, 0xcc) {
        if g.go(f).offset_0x24 < 0 && inside(g, this, f, 40.0, 10, 0x46, 10, 0x46) {
            vcall!(g, f, fish.vfunction89, true);
            break;
        }
    }
    for f in objs(g, board, 0xc0) {
        if g.go(f).offset_0x24 < 0 && inside(g, this, f, 40.0, 10, 0x46, 10, 0x46) {
            crate::game::breeder::FUN_004dd5d0(g, f, true);
            return false;
        }
    }
    false
}

/// port: 004e2170 FUN_004e2170
/// `Animate()`: kind 1 cycles 5 cels; the straight kinds cycle 10 (counting the cycles);
/// Stanley's spins by its x speed (slower once free); the others point along their
/// velocity (16 directions).
pub fn FUN_004e2170(g: &mut G, this: Ptr) {
    let kind = g.missle(this).offset_0x48;
    if kind == 1 {
        let d = g.missle(this);
        d.offset_0x40 = (d.offset_0x40 + 1) % 5;
        return;
    }
    if kind != 2 {
        if FUN_004d80c0(g, this) {
            let d = g.missle(this);
            d.offset_0x40 = (d.offset_0x40 + 1) % 10;
            if d.offset_0x40 == 0 {
                d.offset_0x44 += 1;
            }
            return;
        }
        let d = g.missle(this);
        let (vx, vy) = (d.offset_0x14, d.offset_0x1c);
        let cel = if 0.8 <= vx || vx <= -0.8 {
            if 2.5 < vy {
                0xc
            } else if vy < -2.5 {
                4
            } else if vx <= 0.0 {
                if 2.0 < vy {
                    0xb
                } else if vy < -2.0 {
                    5
                } else if 1.5 < vy {
                    10
                } else if -1.5 <= vy {
                    if 1.0 < vy {
                        9
                    } else if -1.0 <= vy {
                        8
                    } else {
                        7
                    }
                } else {
                    6
                }
            } else if 2.0 < vy {
                0xd
            } else if vy < -2.0 {
                3
            } else if 1.5 < vy {
                0xe
            } else if -1.5 <= vy {
                if 1.0 < vy {
                    0xf
                } else if -1.0 <= vy {
                    0
                } else {
                    1
                }
            } else {
                2
            }
        } else if 0.0 >= vy {
            4
        } else {
            0xc
        };
        d.offset_0x40 = cel;
        return;
    }
    let d = g.missle(this);
    let step = if !d.offset_0x3c { 2 } else { 1 };
    if d.offset_0x14 < 0.0 {
        d.offset_0x2c = (d.offset_0x2c + step) % 0x14;
    } else {
        d.offset_0x2c -= step;
        if d.offset_0x2c < 0 {
            d.offset_0x2c += 0x14;
        }
    }
    d.offset_0x40 = d.offset_0x2c / 2;
}

/// port: 004e1d00 FUN_004e1d00
/// `DrawBody(Graphics*, bool)`: kind 1 an additive glow (white-blue, yellow once free);
/// Stanley's the spinning sheet; the straight kinds their rows (mirrored every other
/// cycle); the others the 16-direction sheet. While a bolt runs, its spark at the bolt's
/// current start point.
pub fn FUN_004e1d00(g: &mut G, this: Ptr, gfx: &mut Graphics, _param_2: bool) {
    let d = g.missle(this).clone();
    match d.offset_0x48 {
        1 => {
            FUN_004558c0(gfx, 1);
            FUN_004558e0(gfx, true);
            let c = if !d.offset_0x3c { CRect(100, 100, 0xff, 0x37) } else { CRect(0xaf, 0xaf, 0x32, 0xff) };
            FUN_00455890(gfx, c);
            let img = g.res.DAT_005e8a44;
            FUN_00456950(gfx, g, img, 0, 0, d.offset_0x40);
            FUN_00455890(gfx, CRect(100, 100, 0xff, 0xff));
            FUN_00456950(gfx, g, img, 0, 0, 5);
            FUN_004558c0(gfx, 0);
            FUN_004558e0(gfx, false);
        }
        2 => {
            let img = g.res.DAT_005e8ee8;
            FUN_00456980(gfx, g, img, 0, 0, d.offset_0x40, d.offset_0x4c);
        }
        3 => {
            let img = g.res.DAT_005e8f00;
            FUN_00455e40(gfx, g, img, 0, 0, &Rect::new(d.offset_0x40 * 0x50, 0x1e0, 0x50, 0x50));
        }
        k @ (4 | 5) => {
            let row = if k != 4 { 1 } else { 4 };
            let img = g.res.DAT_005e8f00;
            FUN_004560a0(gfx, g, img, 0, 0, &Rect::new(d.offset_0x40 * 0x50, row * 0x50, 0x50, 0x50), d.offset_0x44 % 2 == 0);
        }
        _ => {
            let img = g.res.DAT_005e8a2c;
            FUN_00455e40(gfx, g, img, 0, 0, &Rect::new(d.offset_0x40 * 0x50, 0, 0x50, 0x50));
        }
    }
    if 0 < d.offset_0x54 {
        // The bolt's start point moves from (+0x1b0, +0x1b4) to the centre; the original
        // also normalizes the bolt's segment toward the centre here (`FUN_004d7f20`), a
        // result nothing uses.
        let wc = g.wc(this).clone();
        let cx = wc.offset_0x34 / 2 + wc.offset_0x2c;
        let cy = wc.offset_0x38 / 2 + wc.offset_0x30;
        let ix = crate::game::help_screen::FUN_005036e0(d.offset_0x5c, cx, d.offset_0x54, d.offset_0x58, false);
        let iy = crate::game::help_screen::FUN_005036e0(d.offset_0x60, cy, d.offset_0x54, d.offset_0x58, false);
        let x = ftol(ix as f64 - gfx.s.mTransX as f64) as i32;
        let y = ftol(iy as f64 - gfx.s.mTransY as f64) as i32;
        let img = g.res.DAT_005e8ee8;
        FUN_00456980(gfx, g, img, x - 0x19, y - 0x19, d.offset_0x54 % 10, d.offset_0x4c);
    }
}

/// port: 004d7f20 FUN_004d7f20
/// `Vector3::Normalize()`: the vector divided by its length (each component a float), or
/// unchanged when the length is 0.
pub fn FUN_004d7f20(this: [f32; 3]) -> [f32; 3] {
    let [x, y, z] = this;
    let sum = ((y as f64 * y as f64 + x as f64 * x as f64) + z as f64 * z as f64) as f32;
    let len = (sum as f64).sqrt() as f32;
    if len == 0.0 {
        return this;
    }
    let l = len as f64;
    [(x as f64 / l) as f32, (y as f64 / l) as f32, (z as f64 / l) as f32]
}

/// port: 004e2120 Sexy::Missle::vfunction27
/// `Draw(Graphics*)`.
pub fn vfunction27(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    crate::game::food::FUN_004d6ae0(g, this);
    let kind = g.missle(this).offset_0x48;
    if kind != 1 && kind != 2 && !FUN_004d80c0(g, this) {
        FUN_004e1d00(g, this, gfx, false);
        return;
    }
    FUN_004e1d00(g, this, gfx, true);
}

/// port: 004ffb30 Sexy::Missle::vfunction23
/// `Update()` (not while paused): a running bolt ends it; homing (or exploding); kept in
/// the tank (kind 0, and kind 1 still homing) or gone once it flies out; Stanley's falls
/// once free; moving; kind 0 trails sparks; animation.
pub fn vfunction23(g: &mut G, this: Ptr) {
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    if board == NULL || g.board(board).field_0x8 {
        return;
    }
    crate::game::game_object::FUN_004f22c0(g, this);
    let bolt = g.missle(this).offset_0x54;
    if 0 < bolt {
        g.missle(this).offset_0x54 = bolt + 1;
        if g.missle(this).offset_0x58 <= bolt + 1 {
            vfunction76(g, this);
            return;
        }
    }
    if FUN_004ff4e0(g, this) {
        return;
    }
    let kind = g.missle(this).offset_0x48;
    let keep_in = kind == 0 || (kind == 1 && !g.missle(this).offset_0x3c);
    let edge = !keep_in && (kind == 1 || kind == 2 || FUN_004d80c0(g, this));
    if keep_in {
        let d = g.missle(this);
        if 550.0 < d.offset_0x4 {
            d.offset_0x4 = 550.0;
        }
        if d.offset_0x4 < 10.0 {
            d.offset_0x4 = 10.0;
        }
        if 370.0 < d.offset_0xc {
            d.offset_0xc = 370.0;
        }
        if d.offset_0xc < 95.0 {
            d.offset_0xc = 95.0;
        }
    } else if edge {
        let d = g.missle(this);
        if 580.0 < d.offset_0x4 || d.offset_0x4 < -20.0 || 380.0 < d.offset_0xc || d.offset_0xc < 45.0 {
            vfunction76(g, this);
            return;
        }
    }
    {
        let d = g.missle(this);
        if kind == 2 && d.offset_0x3c {
            d.offset_0x1c += 0.2;
        }
        d.offset_0x4 = d.offset_0x14 / d.offset_0x34 + d.offset_0x4;
        d.offset_0xc = d.offset_0x1c / d.offset_0x34 + d.offset_0xc;
        if 0 < d.offset_0x28 {
            d.offset_0x28 -= 1;
        }
    }
    let d = g.missle(this).clone();
    let (x, y) = (ftol(d.offset_0x4) as i32, ftol(d.offset_0xc) as i32);
    vcall!(g, this, w.vfunction42, x, y);
    if g.missle(this).offset_0x48 == 0 && rand(g, this) % 3 == 0 {
        let r1 = (rand(g, this) % 5) as i32;
        let r2 = (rand(g, this) % 5) as i32;
        let d = g.missle(this).clone();
        let (mx, my) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
        let y = my - (ftol(d.offset_0x1c) as i32) * 0x10 + r2 + 0x11;
        let x = mx - (ftol(d.offset_0x14) as i32) * 0x14 + r1 + 0x11;
        crate::game::board_level::FUN_005436f0(g, board, x, y, 8);
    }
    FUN_004e2170(g, this);
    vcall!(g, this, w.vfunction18);
}

/// port: 004d82a0 FUN_004d82a0
/// A laser shot at (x, y) on a missile (not one fired within the last updates, +0x17c):
/// a homing missile (kind 1) is knocked away from the hit point (at least 1 each way) and
/// set loose; a plain one (kind 0) pops. Bubbles at the shot; true on a hit.
pub fn FUN_004d82a0(g: &mut G, this: Ptr, param_1: i32, param_2: i32) -> bool {
    let d = g.missle(this).clone();
    match d.offset_0x48 {
        1 => {
            let (x, y) = (param_1 as f64, param_2 as f64);
            if !(d.offset_0x4 + 10.0 < x && x < d.offset_0x4 + 70.0 && d.offset_0xc + 10.0 < y && y < d.offset_0xc + 70.0) {
                return false;
            }
            if d.offset_0x28 < 1 {
                let m = g.missle(this);
                let vx = ((x - m.offset_0x4) - 40.0) / -5.0;
                m.offset_0x14 = vx;
                let vy = ((y - m.offset_0xc) - 40.0) / -5.0;
                m.offset_0x1c = vy;
                if vx < 1.0 && 0.0 <= vx {
                    m.offset_0x14 = 1.0;
                }
                if -1.0 < m.offset_0x14 && m.offset_0x14 <= 0.0 {
                    m.offset_0x14 = -1.0;
                }
                if vy < 1.0 && 0.0 <= vy {
                    m.offset_0x1c = 1.0;
                }
                if !(m.offset_0x1c <= -1.0 || 0.0 < m.offset_0x1c) {
                    m.offset_0x1c = -1.0;
                }
                m.offset_0x3c = true;
            }
        }
        0 => {
            let cx = d.offset_0x4 + 40.0;
            if param_1 as f64 <= cx - 30.0 || !((param_1 as f64) < cx + 30.0) {
                return false;
            }
            let cy = d.offset_0xc + 40.0;
            if !(cy - 30.0 < param_2 as f64 && (param_2 as f64) < cy + 30.0 && d.offset_0x28 < 1) {
                return false;
            }
            vfunction76(g, this);
        }
        _ => return false,
    }
    let board = board_of(g, this);
    crate::game::board_level::FUN_005436f0(g, board, param_1 - 0x28, param_2 - 0x28, 2);
    true
}
