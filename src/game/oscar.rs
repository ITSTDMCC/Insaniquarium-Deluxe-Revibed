//! `Sexy::Oscar`: the carnivore. It is a `Fish` (same data, object 0x230 bytes) that eats
//! small guppies and drops diamonds. Also the two shared virtual functions its vftable
//! takes from other classes (`Grubber::vfunction71`, `Gekko::vfunction83`).

use crate::game::board_level::vec_index;
use crate::game::game_object::{FUN_004d6cc0, FUN_004d6db0, FUN_004d6d10, FUN_004d6f30, FUN_004d7040};
use crate::sexy::crt::ftol;
use crate::sexy::graphics::{FUN_004558c0, FUN_004558e0, FUN_00455e40, FUN_004560a0, FUN_004563d0, FUN_00456950};
use crate::sexy::prelude::*;
use crate::sexy::types::CRect;

fn board_of(g: &mut G, this: Ptr) -> Ptr {
    let app = g.go(this).offset_0x0;
    g.wfa(app).offset_0x4
}

/// The fish's own vftable becomes Oscar's (the derived part of construction).
fn become_oscar(g: &mut G, this: Ptr) {
    g.set_vt(this, &crate::sexy::vtables_gen::Sexy__Oscar_vftable);
}

/// port: 004efd80 Sexy::Oscar::Oscar
/// `Oscar::Oscar()` (used before loading from a save).
pub fn Oscar__004efd80(g: &mut G) -> Ptr {
    let this = crate::game::fish::Fish__004eee60(g);
    become_oscar(g, this);
    g.go(this).offset_0x4 = 5;
    this
}

/// port: 004efda0 Sexy::Oscar::Oscar
/// `Oscar(int x, int y)`.
pub fn Oscar__004efda0(g: &mut G, param_1: i32, param_2: i32) -> Ptr {
    let this = crate::game::fish::Fish__004eef00(g, param_1, param_2);
    become_oscar(g, this);
    FUN_004d84c0(g, this);
    this
}

/// port: 004efe00 Sexy::Oscar::Oscar
/// `Oscar(int x, int y, bool facingRight)`.
pub fn Oscar__004efe00(g: &mut G, param_1: i32, param_2: i32, param_3: bool) -> Ptr {
    let this = crate::game::fish::Fish__004eef00(g, param_1, param_2);
    become_oscar(g, this);
    FUN_004d84c0(g, this);
    let v = if !param_3 { -1.0 } else { 1.0 };
    let f = g.fish(this);
    f.offset_0x14 = v;
    f.offset_0x34 = v;
    this
}

/// port: 004d84c0 FUN_004d84c0
/// `Oscar::Init()`: type 5, drawn as size 5, swims between 105 and 360, hunger 600..799,
/// mouse-visible as pets are; colored in the +0x880 mode.
pub fn FUN_004d84c0(g: &mut G, this: Ptr) {
    g.go(this).offset_0x4 = 5;
    {
        let f = g.fish(this);
        f.offset_0x4c = 5;
        f.field_0xd5 = false;
        f.field_0x44 = 0x69;
        f.field_0x3c = 0x168;
    }
    let app = g.go(this).offset_0x0;
    let rng = g.wfa(app).offset_0x84;
    let r = crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng));
    g.go(this).offset_0x14 = (r % 200) as i32 + 600;
    g.w(this).offset_0x1 = g.globals.DAT_005e8f14;
    if g.wfa(app).offset_0x154 {
        let seed = crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32 % 0xc + 0x1e;
        vcall!(g, this, fish.vfunction91, seed, false);
        g.fish(this).field_0x7c = true;
    }
}

/// port: 004f03a0 Sexy::Oscar::deleting_destructor
pub fn deleting_destructor(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    g.set_vt(this, &crate::sexy::vtables_gen::Sexy__Oscar_vftable);
    crate::game::fish::dtor_Fish(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 004ea730 Sexy::Grubber::vfunction71
/// Counts the creature in `stats[0]` (shared by the carnivores).
pub fn vfunction71(_g: &mut G, _this: Ptr, stats: &mut [i32]) {
    stats[0] += 1;
}

/// port: 004e1550 Sexy::Gekko::vfunction83
/// `Hunt()` of the creatures that eat other fish: hunger ticks (and, with no aliens, the
/// hungry look); starving dies; below 500 it chases (vfunction 85).
pub fn vfunction83(g: &mut G, this: Ptr) -> bool {
    crate::game::game_object::FUN_004d6c50(g, this);
    let board = board_of(g, this);
    if g.board(board).offset_0xc[vec_index(0xb8)].is_empty() && g.board(board).offset_0xc[vec_index(0xf4)].is_empty() {
        crate::game::game_object::FUN_004d6ef0(g, this);
    }
    if !crate::game::game_object::FUN_004d69d0(g, this) {
        if g.go(this).offset_0x14 < 500 {
            return vcall!(g, this, fish.vfunction85);
        }
    } else {
        vcall!(g, this, fish.vfunction89, true);
    }
    false
}

/// port: 004d8560 Sexy::Oscar::vfunction82
/// `CoinTimer()`: a diamond (coin kind 4) when the timer is up and it is not hungry.
pub fn vfunction82(g: &mut G, this: Ptr) {
    g.fish(this).field_0xcc += 1;
    let mut t = g.fish(this).field_0xcc;
    let period = g.fish(this).field_0xd0;
    let due = crate::game::game_object::FUN_004d7100(g, this, &mut t, period);
    g.fish(this).field_0xcc = t;
    if due {
        g.fish(this).field_0xcc = 0;
        if crate::game::game_object::FUN_004d6bd0(g, this) {
            let board = board_of(g, this);
            let (mx, my) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
            crate::game::board_level::FUN_00544430(g, board, mx + 5, my + 10, 4, NULL, -1.0, 0);
        }
    }
}

/// One axis of the carnivore's steering: far (beyond `far`) by `big`, closer (beyond
/// `mid`) by `med`, near (around `c`) by `small`, within `-lim..lim`.
fn steer(v: &mut f64, at: i32, c: i32, far_hi: i32, far_lo: i32, mid_hi: i32, mid_lo: i32, big: f64, big_up: f64, med: f64, med_up: f64, small: f64, small_up: f64, lim_lo: f64, lim_hi: f64) {
    if far_hi < at {
        if lim_lo < *v {
            *v -= big;
        }
    } else if at < far_lo {
        if *v < lim_hi {
            *v += big_up;
        }
    } else if mid_hi < at {
        if lim_lo < *v {
            *v -= med;
        }
    } else if at < mid_lo {
        if *v < lim_hi {
            *v += med_up;
        }
    } else if c < at {
        if lim_lo < *v {
            *v -= small;
        }
    } else if at < c && *v < lim_hi {
        *v += small_up;
    }
}

/// port: 004d85d0 Sexy::Oscar::vfunction85
/// `Chase()`: every 3 updates steers its mouth toward the nearest small guppy (faster when
/// hungry), then tries to eat. True when there is prey.
pub fn vfunction85(g: &mut G, this: Ptr) -> bool {
    let t = vcall!(g, this, fish.vfunction86);
    let f = g.fish(this).clone();
    let ix = ftol(f.field_0x4 + 40.0) as i32;
    let mut iy = ftol(f.field_0xc + 50.0) as i32;
    if g.go(this).offset_0x68 != 0 {
        iy += 0x14;
    }
    if 2 < f.field_0x5c {
        if t == NULL {
            return false;
        }
        let hunger = g.go(this).offset_0x14;
        g.fish(this).field_0x5c = 0;
        let (tx, ty) = (g.wc(t).offset_0x2c, g.wc(t).offset_0x30);
        let f = g.fish(this);
        if hunger < 0x12d {
            steer(&mut f.offset_0x14, ix, tx + 0x28, tx + 0x30, tx + 0x20, tx + 0x2c, tx + 0x24, 1.5, 1.5, 0.2, 0.2, 0.05, 0.05, -5.0, 5.0);
            steer(&mut f.field_0x1c, iy, ty + 0x28, ty + 0x2e, ty + 0x22, ty + 0x28, ty + 0x28, 1.3, 1.5, 0.5, 0.7, 0.0, 0.0, -3.0, 4.0);
        } else {
            steer(&mut f.offset_0x14, ix, tx + 0x28, tx + 0x30, tx + 0x20, tx + 0x2c, tx + 0x24, 1.5, 1.5, 0.2, 0.2, 0.1, 0.1, -4.0, 4.0);
            steer(&mut f.field_0x1c, iy, ty + 0x28, ty + 0x2e, ty + 0x22, ty + 0x28, ty + 0x28, 0.8, 1.3, 0.3, 0.5, 0.0, 0.0, -3.0, 4.0);
        }
        if f.field_0x68 < 5 {
            f.field_0x68 += 1;
        }
    }
    if t != NULL {
        vcall!(g, this, fish.vfunction87);
    }
    t != NULL
}

/// port: 004eb080 Sexy::Oscar::vfunction86
/// `FindPrey()`: the nearest small guppy of the level not just bought; within 100 pixels
/// the mouth opens (150 updates, +0xf8 = 100). In the virtual tank's feeding mode, the
/// feeder's target instead.
pub fn vfunction86(g: &mut G, this: Ptr) -> Ptr {
    if g.go(this).offset_0x68 != 0 {
        let wc = g.wc(this).clone();
        return crate::game::game_object::FUN_004ea010(g, this, wc.offset_0x34 / 2 + wc.offset_0x2c, wc.offset_0x38 / 2 + wc.offset_0x30 + 0x14);
    }
    let board = board_of(g, this);
    let mut best = 100000000;
    let mut prey = NULL;
    for o in g.board(board).offset_0xc[vec_index(0xa0)].clone() {
        let f = g.fish(this).clone();
        let (ox, oy) = (g.wc(o).offset_0x2c, g.wc(o).offset_0x30);
        let dx = ftol((ox + 0x28) as f64 - (f.field_0x4 + 40.0)) as i32;
        let dy = ftol((oy + 0x28) as f64 - (f.field_0xc + 40.0)) as i32;
        let d = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx));
        if d < best && g.fish(o).offset_0x4c == 0 && g.go(o).offset_0x24 < 0 && g.go(o).offset_0x98 < 1 {
            prey = o;
            best = d;
        }
    }
    if best < 10000 {
        crate::game::game_object::FUN_004d6b00(g, this, 0x96);
        g.go(this).offset_0x70 = 100;
    }
    prey
}

/// port: 004d4520 FUN_004d4520
/// The guppy is protected from carnivores: bought in the virtual tank, or still in its
/// drop-in time (+0x120).
pub fn FUN_004d4520(g: &mut G, this: Ptr) -> bool {
    !(g.go(this).offset_0x24 < 0 && g.go(this).offset_0x98 < 1)
}

/// port: 004eb230 Sexy::Oscar::vfunction87
/// `TryEat()`: a small guppy in its jaws is eaten (vfunction 78, then the guppy goes);
/// one close by opens the mouth (20 updates). In the virtual tank's feeding mode, the
/// feeder's result opens it instead.
pub fn vfunction87(g: &mut G, this: Ptr) {
    if g.go(this).offset_0x68 != 0 {
        let wc = g.wc(this).clone();
        let r = crate::game::game_object::FUN_004ea2a0(g, this, wc.offset_0x34 / 2 + wc.offset_0x2c, wc.offset_0x38 / 2 + wc.offset_0x30 + 0x14);
        if g.fish(this).field_0x74 == 0 {
            if r == 1 {
                g.fish(this).field_0x74 = 8;
                FUN_004d6cc0(g, this);
                return;
            }
            if r == 2 {
                g.fish(this).field_0x74 = 0x14;
                FUN_004d6cc0(g, this);
                return;
            }
        }
        return;
    }
    let board = board_of(g, this);
    for o in g.board(board).offset_0xc[vec_index(0xa0)].clone() {
        let f = g.fish(this).clone();
        let (x, y) = (f.field_0x4 + 40.0, f.field_0xc + 45.0);
        let (ox, oy) = (g.wc(o).offset_0x2c, g.wc(o).offset_0x30);
        let in_jaws = ((ox + 0x46) as f64) > x
            && x > (ox + 10) as f64
            && ((oy + 0x3a) as f64) > y
            && y > (oy + 0x16) as f64
            && g.fish(o).offset_0x4c == 0;
        if in_jaws {
            if g.go(o).offset_0x24 < 0 && g.go(o).offset_0x98 < 1 {
                vcall!(g, this, go.vfunction78, o as i32);
                vcall!(g, o, fish.vfunction88, true);
                if g.fish(this).field_0x74 != 0 {
                    return;
                }
                FUN_004d6cc0(g, this);
                g.fish(this).field_0x74 = 8;
                return;
            }
        } else if g.fish(this).field_0x74 == 0
            && x < (ox + 0x57) as f64
            && ((ox - 7) as f64) < x
            && y < (oy + 0x46) as f64
            && ((oy + 10) as f64) < y
            && g.fish(o).offset_0x4c == 0
            && !FUN_004d4520(g, o)
        {
            g.fish(this).field_0x74 = 0x14;
            FUN_004d6cc0(g, this);
        }
    }
}

/// port: 004f2b00 Sexy::Oscar::vfunction78
/// `Eat(Fish*)`: the eat sound, fed (+900, at most 1300), counted in the virtual tank;
/// colored sparkles in the +0x882 mode (not for food); in the virtual tank's pizza event,
/// one time in three, "This is my last pizza, I swear!".
pub fn vfunction78(g: &mut G, this: Ptr, param_1: i32) {
    let prey = param_1 as Ptr;
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    let high = g.go(this).offset_0x7c;
    crate::game::board_level::FUN_00538560(g, board, high);
    let hungry = FUN_004d6f30(g, this);
    crate::game::game_object::FUN_004d6a30(g, this, false);
    if -1 < g.go(this).offset_0x24 {
        crate::game::game_object::FUN_004d6f90(g, this);
    }
    g.go(this).offset_0x14 += 900;
    if 0x514 < g.go(this).offset_0x14 {
        g.go(this).offset_0x14 = 0x514;
    }
    if g.wfa(app).offset_0x156 && g.go(prey).offset_0x4 != 0x1c {
        let rng = g.wfa(app).offset_0x84;
        let mut n = crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng)) % 3 + 2;
        while n != 0 {
            let r = crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng));
            let y = (r % 0x14) as i32 + 0xf + g.wc(prey).offset_0x30;
            let r = crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng));
            let x = (r % 0x14) as i32 + 0xf + g.wc(prey).offset_0x2c;
            crate::game::board_level::FUN_005436f0(g, board, x, y, 1);
            n -= 1;
        }
    }
    crate::game::game_object::FUN_004e13d0(g, this, hungry);
    if g.go(this).offset_0x9c == 3 {
        let r = crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32;
        if r % 3 == 0 {
            let wc = g.wc(this).clone();
            crate::game::board_level::FUN_005444e0(g, board, wc.offset_0x34 / 2 - 0x28 + wc.offset_0x2c, wc.offset_0x30 - 10, 2, b"This is my last pizza,");
            let wc = g.wc(this).clone();
            crate::game::board_level::FUN_005444e0(g, board, wc.offset_0x34 / 2 + wc.offset_0x2c, wc.offset_0x30 + 5, 2, b"I swear!");
        }
    }
}

/// port: 004f2d30 Sexy::Oscar::vfunction88
/// `Remove(bool withShadow)`: drops what it carries, leaves the widget manager and the
/// board, optionally its shadow; counts a lost carnivore (+0x468) and on level 1-3 of the
/// adventure warns (with hints the first times).
pub fn vfunction88(g: &mut G, this: Ptr, param_1: bool) {
    use crate::game::board_level::FUN_0053e950;
    let carried = g.go(this).offset_0x10;
    if carried != NULL {
        crate::game::missle::vfunction76(g, carried);
    }
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    let mgr = g.wc(board).offset_0xc;
    vcall!(g, mgr, w.vfunction5, this);
    crate::sexy::sexy_app_base::vfunction35(g, app, this);
    crate::game::board_level::FUN_00541d90(g, board, this, false);
    let shadow = g.go(this).offset_0xc;
    if param_1 && shadow != NULL {
        crate::game::shadow::vfunction76(g, shadow);
    }
    let board = g.wfa(app).offset_0x4;
    g.board(board).ext_0x460[(0x468 - 0x460) / 4] += 1;
    if crate::game::board_update::FUN_00537bb0(g, board, 1, 3) {
        if g.board(board).ext_0x4b0[0x1c] != 0 {
            FUN_0053e950(g, board, b"Try feeding small guppies to your carnivores!", false, 0x1d);
        }
        if g.board(board).ext_0x4b0[0x1b] != 0 {
            FUN_0053e950(g, board, b"Hint: Carnivores won't eat fish food!", false, 0x1c);
        }
        FUN_0053e950(g, board, b"Warning! Your carnivore has died!", false, 0x1b);
    }
}

/// port: 004e2480 Sexy::Oscar::vfunction84
/// `DrawFish(Graphics*, bool mirror)`: the carnivore row (or, colored in the +0x880
/// mode, three tinted layers by pose), the hungry tint over it, what it carries, the
/// sparkle; the plain sprite sheet's carnivore cel in the `DAT_005e89ce` mode.
pub fn vfunction84(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: bool) {
    if !g.globals.DAT_005e89ce {
        let src = Rect::new(g.fish(this).field_0x6c * 0x50, 0x140, 0x50, 0x50);
        let hungry = FUN_004d6f30(g, this);
        if g.go(this).offset_0x74 {
            let img = crate::game::fish::FUN_004d5aa0(g, this, hungry);
            if crate::game::game_object::FUN_004d6e70(g, this, gfx, img, &src, param_2) {
                return;
            }
        }
        if !g.fish(this).field_0x7c || hungry {
            FUN_004d6db0(g, this, gfx, Color::WHITE);
            let img = crate::game::fish::FUN_004d5aa0(g, this, hungry);
            FUN_004560a0(gfx, g, img, 0, 0, &src, param_2);
        } else {
            FUN_004d6db0(g, this, gfx, CRect(0xff, 0xff, 0xff, 0xff));
            let f = g.fish(this).clone();
            let pose = if f.field_0x70 == 0 {
                if 0 < f.field_0x74 || 100 < g.go(this).offset_0x80 { 8 } else { 0 }
            } else {
                4
            };
            let img = g.res.DAT_005e8b48;
            let col = f.field_0x6c * 0x50;
            FUN_004560a0(gfx, g, img, 0, 0, &Rect::new(col, (pose * 5 + 0xf) * 0x10, 0x50, 0x50), param_2);
            FUN_004558c0(gfx, 1);
            FUN_004d6db0(g, this, gfx, f.field_0x94);
            FUN_004560a0(gfx, g, img, 0, 0, &Rect::new(col, pose * 0x50 + 0x50, 0x50, 0x50), param_2);
            FUN_004d6db0(g, this, gfx, f.field_0x84);
            FUN_004560a0(gfx, g, img, 0, 0, &Rect::new(col, pose * 0x50, 0x50, 0x50), param_2);
            FUN_004d6db0(g, this, gfx, f.field_0xa4);
            FUN_004560a0(gfx, g, img, 0, 0, &Rect::new(col, (pose * 5 + 10) * 0x10, 0x50, 0x50), param_2);
            FUN_004558c0(gfx, 0);
        }
        FUN_004558e0(gfx, false);
        let t = g.go(this).offset_0x18;
        if t != 0 {
            let img = crate::game::fish::FUN_004d5aa0(g, this, true);
            FUN_004d6db0(g, this, gfx, CRect(0xff, 0xff, 0xff, (t * 0xff) / 5));
            FUN_004560a0(gfx, g, img, 0, 0, &src, param_2);
            FUN_004558e0(gfx, false);
        }
        if g.go(this).offset_0x10 != NULL {
            FUN_004d7040(g, this, gfx, 0, 0);
        }
        if crate::game::game_object::FUN_004d6980(g, this, 500) {
            let img = g.res.DAT_005e8bc4;
            FUN_00456950(gfx, g, img, 0, -10, 2);
        }
    } else {
        let hungry = FUN_004d6f30(g, this);
        let col = if hungry { 6 } else { 9 };
        let img = g.res.DAT_005e8aa8;
        FUN_004560a0(gfx, g, img, 0, 0, &Rect::new(col * 0x50, 0x140, 0x50, 0x50), param_2);
        if g.go(this).offset_0x10 != NULL {
            FUN_004d7040(g, this, gfx, 0, 0);
        }
        if crate::game::game_object::FUN_004d6980(g, this, 500) {
            let img = g.res.DAT_005e8bc4;
            FUN_00456950(gfx, g, img, 0, -10, 2);
        }
    }
}

/// port: 004e2800 Sexy::Oscar::vfunction80
/// `DrawIcon(Graphics*, int pose)`: the carnivore's portrait cel (+0x130) offset by pose,
/// plain or as three tinted layers when colored.
pub fn vfunction80(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: i32) {
    let (dx, dy) = match param_2 {
        0 => (0x11, 0x11),
        1 => (7, 0x11),
        2 => (0x11, 0),
        3 => (8, 2),
        4 => (-0x28, 0),
        _ => (0, 0),
    };
    FUN_004563d0(gfx, dx, dy);
    let col = g.go(this).offset_0xa8 * 0x50;
    if !g.fish(this).field_0x7c {
        FUN_004d6d10(g, this, gfx, Color::WHITE);
        let img = g.res.DAT_005e8ab8;
        FUN_00455e40(gfx, g, img, 0, 0, &Rect::new(col, 0x140, 0x50, 0x50));
    } else {
        FUN_004d6d10(g, this, gfx, Color::WHITE);
        let img = g.res.DAT_005e8b48;
        FUN_004560a0(gfx, g, img, 0, 0, &Rect::new(col, 0xf0, 0x50, 0x50), false);
        FUN_004558c0(gfx, 1);
        let f = g.fish(this).clone();
        FUN_004d6d10(g, this, gfx, f.field_0x94);
        FUN_004560a0(gfx, g, img, 0, 0, &Rect::new(col, 0x50, 0x50, 0x50), false);
        FUN_004d6d10(g, this, gfx, f.field_0x84);
        FUN_004560a0(gfx, g, img, 0, 0, &Rect::new(col, 0, 0x50, 0x50), false);
        FUN_004d6d10(g, this, gfx, f.field_0xa4);
        FUN_004560a0(gfx, g, img, 0, 0, &Rect::new(col, 0xa0, 0x50, 0x50), false);
        FUN_004558c0(gfx, 0);
    }
    FUN_004558e0(gfx, false);
    FUN_004563d0(gfx, -dx, -dy);
}
