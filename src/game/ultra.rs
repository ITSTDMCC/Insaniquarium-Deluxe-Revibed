//! `Sexy::Ultra`: the ultravore, a 160x160 `Fish` (same data, object 0x230 bytes) that
//! eats carnivores and drops treasure chests.

use crate::game::board_level::vec_index;
use crate::game::game_object::{FUN_004d6cc0, FUN_004d6db0, FUN_004d6f30, FUN_004d7040};
use crate::sexy::crt::ftol;
use crate::sexy::graphics::{FUN_00455890, FUN_004558e0, FUN_004560a0, FUN_004563d0, FUN_00456950, FUN_00456980};
use crate::sexy::prelude::*;
use crate::sexy::types::CRect;

fn board_of(g: &mut G, this: Ptr) -> Ptr {
    let app = g.go(this).offset_0x0;
    g.wfa(app).offset_0x4
}

/// The fish's vftable becomes the Ultra's (the derived part of construction).
fn become_ultra(g: &mut G, this: Ptr) {
    g.set_vt(this, &crate::sexy::vtables_gen::Sexy__Ultra_vftable);
}

/// port: 004f0270 Sexy::Ultra::Ultra
/// `Ultra::Ultra()` (used before loading from a save).
pub fn Ultra__004f0270(g: &mut G) -> Ptr {
    let this = crate::game::fish::Fish__004eee60(g);
    become_ultra(g, this);
    g.go(this).offset_0x4 = 6;
    this
}

/// port: 004f02a0 Sexy::Ultra::Ultra
/// `Ultra(int x, int y)`.
pub fn Ultra__004f02a0(g: &mut G, param_1: i32, param_2: i32) -> Ptr {
    let this = crate::game::fish::Fish__004eef00(g, param_1, param_2);
    become_ultra(g, this);
    FUN_004d9ee0(g, this);
    this
}

/// port: 004f0310 Sexy::Ultra::Ultra
/// `Ultra(int x, int y, bool facingRight)`.
pub fn Ultra__004f0310(g: &mut G, param_1: i32, param_2: i32, param_3: bool) -> Ptr {
    let this = crate::game::fish::Fish__004eef00(g, param_1, param_2);
    become_ultra(g, this);
    FUN_004d9ee0(g, this);
    let v = if !param_3 { -1.0 } else { 1.0 };
    let f = g.fish(this);
    f.offset_0x14 = v;
    f.offset_0x34 = v;
    this
}

/// port: 004d9ee0 FUN_004d9ee0
/// `Ultra::Init()`: type 6, drawn as size 6, 160x160, swims between 75 and 310 within
/// x 0..480, hunger 600..799, a chest every 200..449 updates (or the virtual tank's
/// interval); mouse-visible as pets are.
pub fn FUN_004d9ee0(g: &mut G, this: Ptr) {
    g.go(this).offset_0x4 = 6;
    g.fish(this).offset_0x4c = 6;
    g.wc(this).offset_0x34 = 0xa0;
    g.wc(this).offset_0x38 = 0xa0;
    {
        let f = g.fish(this);
        f.field_0xd5 = false;
        f.field_0x3c = 0x136;
        f.field_0x44 = 0x4b;
        f.field_0x40 = 0;
        f.field_0x48 = 0x1e0;
    }
    let app = g.go(this).offset_0x0;
    let rng = g.wfa(app).offset_0x84;
    let r = crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng));
    g.go(this).offset_0x14 = (r % 200) as i32 + 600;
    g.w(this).offset_0x1 = g.globals.DAT_005e8f14;
    let r = crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng));
    let iv = crate::game::fish::FUN_004d71d0(g, this, (r % 0xfa) as i32 + 200);
    g.fish(this).field_0xd0 = iv;
}

/// port: 004f0390 Sexy::Ultra::~Ultra
pub fn dtor_Ultra(g: &mut G, this: Ptr) {
    g.set_vt(this, &crate::sexy::vtables_gen::Sexy__Ultra_vftable);
    crate::game::fish::dtor_Fish(g, this);
}

/// port: 004f0d70 Sexy::Ultra::deleting_destructor
pub fn deleting_destructor(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_Ultra(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 004f0290 Sexy::Ultra::vfunction71
/// Counts an ultravore in `stats[3]`.
pub fn vfunction71(_g: &mut G, _this: Ptr, stats: &mut [i32]) {
    stats[3] += 1;
}

/// port: 004d9f90 Sexy::Ultra::vfunction82
/// `CoinTimer()`: a treasure chest (coin kind 7) under its mouth when the timer is up and
/// it is not hungry.
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
            crate::game::board_level::FUN_00544430(g, board, mx + 0x28, my + 0x5a, 7, NULL, -1.0, 0);
        }
    }
}

/// port: 004da000 Sexy::Ultra::vfunction90
/// `Animate()`: the drop-in fade, the turn and eat counters, the cel (turn, eat or swim
/// cycle), the facing, the open mouth cel, carrying.
pub fn vfunction90(g: &mut G, this: Ptr) {
    if g.go(this).offset_0x74 {
        crate::game::game_object::FUN_004d6c90(g, this);
    }
    let f = g.fish(this);
    if f.offset_0x34 <= 0.0 || 0.0 <= f.offset_0x14 {
        if f.offset_0x34 < 0.0 && 0.0 < f.offset_0x14 {
            f.field_0x70 = -0x14;
        }
    } else {
        f.field_0x70 = 0x14;
    }
    let v = f.field_0x70;
    if v != 0 {
        f.field_0x70 = if 0 < v { v - 1 } else { v + 1 };
        if 0 < f.field_0x74 {
            f.field_0x74 -= 1;
        }
    }
    let turn = f.field_0x70;
    if turn == 0 {
        if f.field_0x74 < 1 {
            f.field_0x64 += if f.field_0x68 < 2 { 1 } else { 2 };
            if 0x13 < f.field_0x64 {
                f.field_0x64 = 0;
            }
            f.field_0x6c = f.field_0x64 / 2;
        } else {
            f.field_0x74 -= 1;
            f.field_0x6c = 9 - f.field_0x74 / 2;
        }
    } else {
        f.field_0x74 = 0;
        if 0 < turn {
            f.field_0x6c = 9 - turn / 2;
        } else {
            f.field_0x6c = turn / 2 + 9;
        }
    }
    if f.offset_0x14 != f.offset_0x34 && f.offset_0x14 != 0.0 && f.offset_0x34 != 0.0 {
        f.offset_0x34 = f.offset_0x14;
    }
    if 100 < g.go(this).offset_0x80 && turn == 0 {
        g.fish(this).field_0x6c = 4;
    }
    if g.go(this).offset_0x10 == NULL {
        return;
    }
    crate::game::game_object::FUN_004d7020(g, this);
}

/// port: 004da260 Sexy::Ultra::vfunction85
/// `Chase()`: every 3 updates steers its mouth toward the nearest carnivore (faster when
/// hungry), then tries to eat. True when there is prey.
pub fn vfunction85(g: &mut G, this: Ptr) -> bool {
    let t = vcall!(g, this, fish.vfunction86);
    let f = g.fish(this).clone();
    let ix = ftol(f.field_0x4 + 80.0) as i32;
    let mut iy = ftol(f.field_0xc + 100.0) as i32;
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
        let (small, lim) = if hunger < 0x12d { (0.05, 5.0) } else { (0.1, 4.0) };
        let v = &mut f.offset_0x14;
        if tx + 0x2c < ix {
            if -lim < *v {
                *v -= 1.5;
            }
        } else if ix < tx + 0x24 {
            if *v < lim {
                *v += 1.5;
            }
        } else if tx + 0x2a < ix {
            if -lim < *v {
                *v -= 0.2;
            }
        } else if ix < tx + 0x26 {
            if *v < lim {
                *v += 0.2;
            }
        } else if tx + 0x28 < ix {
            if -lim < *v {
                *v -= small;
            }
        } else if ix < tx + 0x28 && *v < lim {
            *v += small;
        }
        let vy = &mut f.field_0x1c;
        if hunger < 0x12d {
            let c = ty + 0x28;
            if c < iy {
                if -3.0 < *vy {
                    *vy -= 1.3;
                }
            } else if iy < c && *vy < 4.0 {
                *vy += 1.3;
            }
        } else if ty + 0x2b < iy {
            if -3.0 < *vy {
                *vy -= 0.8;
            }
        } else if iy < ty + 0x25 {
            if *vy < 4.0 {
                *vy += 1.3;
            }
        } else if ty + 0x28 < iy {
            if -3.0 < *vy {
                *vy -= 0.3;
            }
        } else if iy < ty + 0x28 && *vy < 4.0 {
            *vy += 0.5;
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

/// port: 004e38f0 Sexy::Ultra::vfunction83
/// `Hunt()`: hunger ticks (the hungry look only with no aliens around); starving dies;
/// below 500 it chases carnivores. The virtual tank's pizza variant (+0x124 == 4) asks for
/// pizza at 290 and only chases between messages.
pub fn vfunction83(g: &mut G, this: Ptr) -> bool {
    crate::game::game_object::FUN_004d6c50(g, this);
    let board = board_of(g, this);
    if g.board(board).offset_0xc[vec_index(0xb8)].is_empty() && g.board(board).offset_0xc[vec_index(0xf4)].is_empty() {
        crate::game::game_object::FUN_004d6ef0(g, this);
    }
    if !crate::game::game_object::FUN_004d69d0(g, this) {
        let h = g.go(this).offset_0x14;
        if h < 500 {
            if g.go(this).offset_0x9c == 4 {
                let go = g.go(this).clone();
                if h == 0x122 && go.offset_0x90 % 5 == 0 && go.offset_0x94 == -1 {
                    g.go(this).offset_0x8c = 5;
                    return vcall!(g, this, fish.vfunction85);
                }
                if 0x121 < h {
                    return false;
                }
                if go.offset_0x8c != 0 || go.offset_0x94 != -1 {
                    let m = g.board(board).offset_0x4;
                    if crate::game::timed_messages::FUN_00505120(g, m, go.offset_0x94) < 0x88b9 {
                        return false;
                    }
                }
            }
            return vcall!(g, this, fish.vfunction85);
        }
    } else {
        vcall!(g, this, fish.vfunction89, true);
    }
    false
}

/// port: 004eca70 Sexy::Ultra::vfunction86
/// `FindPrey()`: the nearest carnivore of the level not just bought; within 200 pixels
/// the mouth opens (175 updates), within 100 +0xf8 = 100. In the virtual tank's feeding
/// mode, the feeder's target instead.
pub fn vfunction86(g: &mut G, this: Ptr) -> Ptr {
    if g.go(this).offset_0x68 != 0 {
        let wc = g.wc(this).clone();
        return crate::game::game_object::FUN_004ea010(g, this, wc.offset_0x34 / 2 + wc.offset_0x2c, wc.offset_0x38 / 2 + wc.offset_0x30 + 0x14);
    }
    let board = board_of(g, this);
    let mut best = 100000000;
    let mut prey = NULL;
    for o in g.board(board).offset_0xc[vec_index(0xa4)].clone() {
        if g.go(o).offset_0x24 < 0 && g.go(o).offset_0x98 < 1 {
            let f = g.fish(this).clone();
            let (ox, oy) = (g.wc(o).offset_0x2c, g.wc(o).offset_0x30);
            let dx = ftol((ox + 0x28) as f64 - (f.field_0x4 + 80.0)) as i32;
            let dy = ftol((oy + 0x28) as f64 - (f.field_0xc + 80.0)) as i32;
            let d = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx));
            if d < best {
                prey = o;
                best = d;
            }
        }
    }
    if best < 40000 {
        crate::game::game_object::FUN_004d6b00(g, this, 0xaf);
    }
    if best < 10000 {
        g.go(this).offset_0x70 = 100;
    }
    prey
}

/// port: 004ecbf0 Sexy::Ultra::vfunction87
/// `TryEat()`: a carnivore in its jaws is eaten (vfunction 78, then it goes); one close
/// by opens the mouth (20 updates). In the virtual tank's feeding mode, the feeder's
/// result opens it instead.
pub fn vfunction87(g: &mut G, this: Ptr) {
    if g.go(this).offset_0x68 == 0 {
        let board = board_of(g, this);
        for o in g.board(board).offset_0xc[vec_index(0xa4)].clone() {
            if g.go(o).offset_0x24 < 0 && g.go(o).offset_0x98 < 1 {
                let f = g.fish(this).clone();
                let (x, y) = (f.field_0x4 + 80.0, f.field_0xc + 90.0);
                let (ox, oy) = (g.wc(o).offset_0x2c, g.wc(o).offset_0x30);
                if x < (ox + 0x78) as f64 && ((ox - 0x28) as f64) < x && y < (oy + 0x50) as f64 && (oy as f64) < y {
                    vcall!(g, this, go.vfunction78, o as i32);
                    vcall!(g, o, fish.vfunction88, true);
                    if g.fish(this).field_0x74 != 0 {
                        return;
                    }
                    FUN_004d6cc0(g, this);
                    g.fish(this).field_0x74 = 8;
                    return;
                }
                if g.fish(this).field_0x74 == 0
                    && x < (ox + 0x96) as f64
                    && ((ox - 0x46) as f64) < x
                    && y < (oy + 100) as f64
                    && ((oy - 0x14) as f64) < y
                {
                    FUN_004d6cc0(g, this);
                    g.fish(this).field_0x74 = 0x14;
                }
            }
        }
        return;
    }
    let wc = g.wc(this).clone();
    let r = crate::game::game_object::FUN_004ea2a0(g, this, wc.offset_0x34 / 2 + wc.offset_0x2c, wc.offset_0x38 / 2 + wc.offset_0x30 + 0x14);
    if g.fish(this).field_0x74 == 0 {
        if r == 1 {
            FUN_004d6cc0(g, this);
            g.fish(this).field_0x74 = 8;
            return;
        }
        if r == 2 {
            FUN_004d6cc0(g, this);
            g.fish(this).field_0x74 = 0x14;
        }
    }
}

/// port: 004e3c60 Sexy::Ultra::vfunction78
/// `Eat(Fish*)`: counted in the virtual tank (the pizza variant's message ends), the bite
/// sound, fed (+900, at most 1300); colored sparkles in the +0x882 mode (not for food).
pub fn vfunction78(g: &mut G, this: Ptr, param_1: i32) {
    let prey = param_1 as Ptr;
    let hungry = FUN_004d6f30(g, this);
    crate::game::game_object::FUN_004d6a30(g, this, false);
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    if -1 < g.go(this).offset_0x24 {
        crate::game::game_object::FUN_004d6f90(g, this);
        if g.go(this).offset_0x9c == 4 {
            let m = g.go(this).offset_0x94;
            if m != -1 {
                crate::game::board::FUN_005384a0(g, board, m);
            }
        }
    }
    let high = g.go(this).offset_0x7c;
    crate::game::board_level::FUN_00538560(g, board, high);
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
}

/// port: 004da5e0 Sexy::Ultra::vfunction88
/// `Remove(bool withShadow)`: drops what it carries, leaves the widget manager and the
/// board, optionally its shadow; counts a lost ultravore (+0x474).
pub fn vfunction88(g: &mut G, this: Ptr, param_1: bool) {
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
    g.board(board).ext_0x460[(0x474 - 0x460) / 4] += 1;
}

/// port: 004e3a10 Sexy::Ultra::vfunction84
/// `DrawFish(Graphics*, bool mirror)`: the 160x160 cel (row 1 eating, 2 turning), tinted
/// pink for the virtual tank's pizza variant, the hungry yellow tint over it (fading),
/// what it carries, the sparkle; the plain sheet's cel (yellow when hungry) in the
/// `DAT_005e89ce` mode.
pub fn vfunction84(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: bool) {
    let img = g.res.DAT_005e8cf4;
    if g.globals.DAT_005e89ce {
        if FUN_004d6f30(g, this) {
            FUN_004558e0(gfx, true);
            FUN_00455890(gfx, crate::sexy::types::FUN_00433320(0xfad75f));
        }
        FUN_004560a0(gfx, g, img, 0, 0, &Rect::new(0x5a0, 0x1e0, 0xa0, 0xa0), param_2);
        FUN_004558e0(gfx, false);
        if g.go(this).offset_0x10 != NULL {
            FUN_004d7040(g, this, gfx, 0x28, 0x28);
        }
        if !crate::game::game_object::FUN_004d6980(g, this, 500) {
            return;
        }
        let s = g.res.DAT_005e8bc4;
        FUN_00456950(gfx, g, s, 0x28, 0, 2);
        return;
    }
    let f = g.fish(this).clone();
    let row = if f.field_0x70 == 0 {
        if 0 < f.field_0x74 || 100 < g.go(this).offset_0x80 { 0xa0 } else { 0 }
    } else {
        0x140
    };
    let src = Rect::new(f.field_0x6c * 0xa0, row, 0xa0, 0xa0);
    if g.go(this).offset_0x74 && crate::game::game_object::FUN_004d6e70(g, this, gfx, img, &src, param_2) {
        return;
    }
    let mut overlay = true;
    if !FUN_004d6f30(g, this) {
        let color = if g.go(this).offset_0x9c == 4 { crate::sexy::types::FUN_00433320(0xffaaff) } else { Color::WHITE };
        FUN_004d6db0(g, this, gfx, color);
        FUN_004560a0(gfx, g, img, 0, 0, &src, param_2);
        let t = g.go(this).offset_0x18;
        if t == 0 {
            overlay = false;
        } else {
            FUN_004d6db0(g, this, gfx, CRect(0xfa, 0xd7, 0x5f, (t * 0xff) / 5));
        }
    } else {
        FUN_004d6db0(g, this, gfx, CRect(0xfa, 0xd7, 0x5f, 0xff));
    }
    if overlay {
        FUN_004560a0(gfx, g, img, 0, 0, &src, param_2);
    }
    FUN_004558e0(gfx, false);
    if g.go(this).offset_0x10 != NULL {
        FUN_004d7040(g, this, gfx, 0x28, 0x28);
    }
    if crate::game::game_object::FUN_004d6980(g, this, 500) {
        let s = g.res.DAT_005e8bc4;
        FUN_00456950(gfx, g, s, 0x28, 0, 2);
    }
}

/// port: 004da180 Sexy::Ultra::vfunction80
/// `DrawIcon(Graphics*, int pose)`: its portrait cel offset by pose, pink for the pizza
/// variant.
pub fn vfunction80(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: i32) {
    let (dx, dy) = match param_2 {
        0 => (0xd, -0x16),
        1 => (3, -0x16),
        2 => (3, -0x2a),
        3 => (10, -0x28),
        4 => (-0x46, -0x2d),
        _ => (0, 0),
    };
    FUN_004563d0(gfx, dx, dy);
    let color = if g.go(this).offset_0x9c == 4 { crate::sexy::types::FUN_00433320(0xffaaff) } else { Color::WHITE };
    crate::game::game_object::FUN_004d6d10(g, this, gfx, color);
    let img = g.res.DAT_005e8cf4;
    let cel = g.go(this).offset_0xa8;
    FUN_00456980(gfx, g, img, 0, 0, cel, 0);
    FUN_004558e0(gfx, false);
    FUN_004563d0(gfx, -dx, -dy);
}
