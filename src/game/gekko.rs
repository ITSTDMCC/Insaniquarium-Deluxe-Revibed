//! `Sexy::Gekko`: the beetlemuncher. It is a `Fish` (same data, object 0x230 bytes)
//! that eats beetles (`Larva`) and the green beetle coins (kind 0x12), and drops pearls.

use crate::game::board_level::vec_index;
use crate::game::game_object::{FUN_004d6cc0, FUN_004d6db0, FUN_004d6f30, FUN_004d7040};
use crate::sexy::crt::ftol;
use crate::sexy::graphics::{FUN_004558c0, FUN_004558e0, FUN_00455e40, FUN_004560a0, FUN_004563d0, FUN_00456950};
use crate::sexy::prelude::*;
use crate::sexy::types::CRect;

fn board_of(g: &mut G, this: Ptr) -> Ptr {
    let app = g.go(this).offset_0x0;
    g.wfa(app).offset_0x4
}

/// The fish's vftable becomes the Gekko's (the derived part of construction).
fn become_gekko(g: &mut G, this: Ptr) {
    g.set_vt(this, &crate::sexy::vtables_gen::Sexy__Gekko_vftable);
}

/// port: 004ef970 Sexy::Gekko::Gekko
/// `Gekko::Gekko()` (used before loading from a save).
pub fn Gekko__004ef970(g: &mut G) -> Ptr {
    let this = crate::game::fish::Fish__004eee60(g);
    become_gekko(g, this);
    g.go(this).offset_0x4 = 7;
    this
}

/// port: 004ef9a0 Sexy::Gekko::Gekko
/// `Gekko(int x, int y)`.
pub fn Gekko__004ef9a0(g: &mut G, param_1: i32, param_2: i32) -> Ptr {
    let this = crate::game::fish::Fish__004eef00(g, param_1, param_2);
    become_gekko(g, this);
    FUN_004d72b0(g, this);
    this
}

/// port: 004efa10 Sexy::Gekko::Gekko
/// `Gekko(int x, int y, bool facingRight)`.
pub fn Gekko__004efa10(g: &mut G, param_1: i32, param_2: i32, param_3: bool) -> Ptr {
    let this = crate::game::fish::Fish__004eef00(g, param_1, param_2);
    become_gekko(g, this);
    FUN_004d72b0(g, this);
    let v = if !param_3 { -1.0 } else { 1.0 };
    let f = g.fish(this);
    f.offset_0x14 = v;
    f.offset_0x34 = v;
    this
}

/// port: 004d72b0 FUN_004d72b0
/// `Gekko::Init()`: type 7, drawn as size 7, swims between 105 and 360, a pearl every
/// 200..449 updates (or the virtual tank's interval); mouse-visible as pets are.
pub fn FUN_004d72b0(g: &mut G, this: Ptr) {
    g.go(this).offset_0x4 = 7;
    {
        let f = g.fish(this);
        f.field_0xd5 = false;
        f.offset_0x4c = 7;
        f.field_0x44 = 0x69;
        f.field_0x3c = 0x168;
    }
    g.w(this).offset_0x1 = g.globals.DAT_005e8f14;
    let app = g.go(this).offset_0x0;
    let rng = g.wfa(app).offset_0x84;
    let r = crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng));
    let iv = crate::game::fish::FUN_004d71d0(g, this, (r % 0xfa) as i32 + 200);
    g.fish(this).field_0xd0 = iv;
}

/// port: 004efa90 Sexy::Gekko::~Gekko
pub fn dtor_Gekko(g: &mut G, this: Ptr) {
    g.set_vt(this, &crate::sexy::vtables_gen::Sexy__Gekko_vftable);
    crate::game::fish::dtor_Fish(g, this);
}

/// port: 004f0d10 Sexy::Gekko::deleting_destructor
pub fn deleting_destructor(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_Gekko(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 004ef990 Sexy::Gekko::vfunction71
/// Counts a beetlemuncher in `stats[2]`.
pub fn vfunction71(_g: &mut G, _this: Ptr, stats: &mut [i32]) {
    stats[2] += 1;
}

/// port: 004d7090 FUN_004d7090
/// The mouth's y offset: 20 in the virtual tank's feeding modes (but mode 6), else 0.
pub fn FUN_004d7090(g: &mut G, this: Ptr) -> i32 {
    let m = g.go(this).offset_0x68;
    if m != 0 && m != 6 { 0x14 } else { 0 }
}

/// port: 004d7370 FUN_004d7370
/// Keeps the nearer of `o` (measured from its +20 corner to (`param_1`, `param_2`)) in
/// `best`: true when it is nearer.
pub fn FUN_004d7370(g: &mut G, o: Ptr, best: &mut i32, param_1: i32, param_2: i32) -> bool {
    let (ox, oy) = (g.wc(o).offset_0x2c, g.wc(o).offset_0x30);
    let dx = (ox - param_1) + 0x14;
    let dy = (oy - param_2) + 0x14;
    let d = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx));
    if d < *best {
        *best = d;
        return true;
    }
    false
}

/// port: 004ea3f0 Sexy::Gekko::vfunction86
/// `FindPrey()`: the nearest uncollected beetle or green beetle coin; in the virtual
/// tank's feeding mode, the feeder's target instead.
pub fn vfunction86(g: &mut G, this: Ptr) -> Ptr {
    if g.go(this).offset_0x68 == 0 {
        let board = board_of(g, this);
        let wc = g.wc(this).clone();
        let cx = wc.offset_0x34 / 2 + wc.offset_0x2c;
        let cy = wc.offset_0x38 / 2 + wc.offset_0x30;
        let mut best = 100000000;
        let mut prey = NULL;
        for o in g.board(board).offset_0xc[vec_index(0xc8)].clone() {
            if !g.larva(o).offset_0x20 && FUN_004d7370(g, o, &mut best, cx, cy) {
                prey = o;
            }
        }
        for o in g.board(board).offset_0xc[vec_index(0xa8)].clone() {
            let c = g.coin(o).clone();
            if c.offset_0x40 == 0x12 && !c.offset_0x44 && FUN_004d7370(g, o, &mut best, cx, cy) {
                prey = o;
            }
        }
        return prey;
    }
    let wc = g.wc(this).clone();
    let cx = wc.offset_0x34 / 2 + wc.offset_0x2c;
    let k = FUN_004d7090(g, this);
    crate::game::game_object::FUN_004ea010(g, this, cx, k + wc.offset_0x38 / 2 + wc.offset_0x30)
}

/// port: 004d7740 FUN_004d7740
/// A bite at `param_1`: in its mouth it is eaten (vfunction 78, then it goes) and the
/// mouth closes on it (true); close by the mouth opens (20 updates).
pub fn FUN_004d7740(g: &mut G, this: Ptr, param_1: Ptr) -> bool {
    let (ox, oy) = (g.wc(param_1).offset_0x2c, g.wc(param_1).offset_0x30);
    let f = g.fish(this).clone();
    let x = f.field_0x4 + 40.0;
    let y = f.field_0xc + 40.0;
    if x < (ox + 0x36) as f64 && ((ox + 0x12) as f64) < x && y < (oy + 0x36) as f64 && ((oy + 0x12) as f64) < y {
        vcall!(g, this, go.vfunction78, param_1 as i32);
        vcall!(g, param_1, go.vfunction76);
        if g.fish(this).field_0x74 == 0 {
            FUN_004d6cc0(g, this);
            g.fish(this).field_0x74 = 8;
        }
        return true;
    }
    if g.fish(this).field_0x74 == 0 && x < (ox + 0x4c) as f64 && ((ox - 4) as f64) < x && y < (oy + 0x42) as f64 && ((oy + 6) as f64) < y {
        FUN_004d6cc0(g, this);
        g.fish(this).field_0x74 = 0x14;
        return false;
    }
    false
}

/// port: 004ea590 Sexy::Gekko::vfunction87
/// `TryEat()`: bites the first beetle, then the first green beetle coin, in reach; in the
/// virtual tank's feeding mode, the feeder's result opens the mouth.
pub fn vfunction87(g: &mut G, this: Ptr) {
    if g.go(this).offset_0x68 == 0 {
        let board = board_of(g, this);
        for o in g.board(board).offset_0xc[vec_index(0xc8)].clone() {
            if !g.larva(o).offset_0x20 && FUN_004d7740(g, this, o) {
                return;
            }
        }
        for o in g.board(board).offset_0xc[vec_index(0xa8)].clone() {
            let c = g.coin(o).clone();
            if c.offset_0x40 == 0x12 && !c.offset_0x44 && FUN_004d7740(g, this, o) {
                return;
            }
        }
        return;
    }
    let wc = g.wc(this).clone();
    let cx = wc.offset_0x34 / 2 + wc.offset_0x2c;
    let k = FUN_004d7090(g, this);
    let r = crate::game::game_object::FUN_004ea2a0(g, this, cx, k + wc.offset_0x38 / 2 + wc.offset_0x30);
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

/// port: 004d73a0 Sexy::Gekko::vfunction85
/// `Chase()`: every 3 updates steers its mouth toward the nearest prey (faster when
/// hungry), then tries to eat. True when there is prey.
pub fn vfunction85(g: &mut G, this: Ptr) -> bool {
    let t = vcall!(g, this, fish.vfunction86);
    let f = g.fish(this).clone();
    let ix = ftol(f.field_0x4 + 40.0) as i32;
    let k = FUN_004d7090(g, this);
    let iy = ftol(k as f64 + (f.field_0xc + 45.0)) as i32;
    if 2 < f.field_0x5c {
        if t == NULL {
            return false;
        }
        let hunger = g.go(this).offset_0x14;
        g.fish(this).field_0x5c = 0;
        let (tx, ty) = (g.wc(t).offset_0x2c, g.wc(t).offset_0x30);
        let f = g.fish(this);
        let (big, small, tiny, lim) = if hunger < 0x12d { (1.3, 0.2, 0.05, 4.0) } else { (1.0, 0.1, 0.05, 3.0) };
        let v = &mut f.offset_0x14;
        if tx + 0x28 < ix {
            if -lim < *v {
                *v -= big;
            }
        } else if ix < tx + 0x20 {
            if *v < lim {
                *v += big;
            }
        } else if tx + 0x26 < ix {
            if -lim < *v {
                *v -= small;
            }
        } else if ix < tx + 0x22 {
            if *v < lim {
                *v += small;
            }
        } else if tx + 0x24 < ix {
            if -lim < *v {
                *v -= tiny;
            }
        } else if ix < tx + 0x24 && *v < lim {
            *v += tiny;
        }
        let vy = &mut f.field_0x1c;
        if hunger < 0x12d {
            let c = ty + 0x24;
            if c < iy {
                if -4.0 < *vy {
                    *vy -= 1.3;
                }
            } else if iy < c && *vy < 4.0 {
                *vy += 1.3;
            }
        } else if ty + 0x27 < iy {
            if -3.0 < *vy {
                *vy -= 1.0;
            }
        } else if iy < ty + 0x21 {
            if *vy < 3.0 {
                *vy += 1.0;
            }
        } else if ty + 0x24 < iy {
            if -3.0 < *vy {
                *vy -= 0.5;
            }
        } else if iy < ty + 0x24 && *vy < 3.0 {
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

/// port: 004e1950 Sexy::Gekko::vfunction78
/// `Eat(GameObject*)`: counted in the virtual tank; the eat sound (food) or bite sound;
/// fed (+700, at most 1000).
pub fn vfunction78(g: &mut G, this: Ptr, param_1: i32) {
    let prey = param_1 as Ptr;
    let hungry = FUN_004d6f30(g, this);
    crate::game::game_object::FUN_004d6a30(g, this, false);
    if -1 < g.go(this).offset_0x24 {
        crate::game::game_object::FUN_004d6f90(g, this);
    }
    let board = board_of(g, this);
    let high = g.go(this).offset_0x7c;
    if g.go(prey).offset_0x4 == 0x1c {
        crate::game::board_level::FUN_005384c0(g, board, high);
    } else {
        crate::game::board_level::FUN_00538560(g, board, high);
    }
    g.go(this).offset_0x14 += 700;
    if 1000 < g.go(this).offset_0x14 {
        g.go(this).offset_0x14 = 1000;
    }
    crate::game::game_object::FUN_004e13d0(g, this, hungry);
}

/// port: 004d9e60 Sexy::Gekko::vfunction88
/// `Remove(bool withShadow)`: drops what it carries, leaves the widget manager and the
/// board, optionally its shadow; counts a lost beetlemuncher (+0x484).
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
    g.board(board).ext_0x460[(0x484 - 0x460) / 4] += 1;
}

/// port: 004fbe80 Sexy::Gekko::vfunction82
/// `CoinTimer()`: a pearl (coin kind 6) when the timer is up and it is not hungry; the
/// virtual tank's feeding variant (+0x124 == 2) instead drops a food some eaters lack.
pub fn vfunction82(g: &mut G, this: Ptr) {
    g.fish(this).field_0xcc += 1;
    let cc = g.fish(this).field_0xcc;
    if g.go(this).offset_0x9c == 2 {
        if 0x21c < cc || cc == 0x1f8 {
            // The foods some eaters lack (guppies only without pet 3 in the tank); one at
            // random is dropped when the timer is up, 30 updates after it opens its mouth.
            let app = g.go(this).offset_0x0;
            let board = g.wfa(app).offset_0x4;
            let need = crate::game::virtual_tank::FUN_0053a3c0(g, board);
            let have = crate::game::virtual_tank::FUN_005397a0(g, board);
            let mut kinds: Vec<i32> = Vec::new();
            for i in 0..8usize {
                if need[i] != have[i] && -1 < need[i] - have[i] && (i != 0 || g.board(board).field_0xb8[3] == 0) {
                    kinds.push(i as i32);
                }
            }
            if !kinds.is_empty() {
                let r = (crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32 % kinds.len() as i32) as usize;
                let cc = g.fish(this).field_0xcc;
                if cc < 0x21d {
                    if cc == 0x1f8 {
                        g.fish(this).field_0x78 = 0x48;
                    }
                } else {
                    crate::game::food_dialog::FUN_00547530(g, board, kinds[r]);
                    if g.fish(this).field_0x78 == 0 {
                        g.fish(this).field_0x78 = 0x24;
                    }
                }
            }
            if 0x21c < g.fish(this).field_0xcc {
                g.fish(this).field_0xcc = 0;
            }
        }
        return;
    }
    let mut t = cc;
    let period = g.fish(this).field_0xd0;
    let due = crate::game::game_object::FUN_004d7100(g, this, &mut t, period);
    g.fish(this).field_0xcc = t;
    if due {
        g.fish(this).field_0xcc = 0;
        if crate::game::game_object::FUN_004d6bd0(g, this) {
            let board = board_of(g, this);
            let (mx, my) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
            crate::game::board_level::FUN_00544430(g, board, mx + 5, my + 10, 6, NULL, -1.0, 0);
        }
    }
}

/// port: 004d7320 FUN_004d7320
/// The sprite row: 1 / 4 (hungry) turning, 0 / 3 (hungry) swimming, 2 / 6 (hungry) eating
/// or with the mouth open.
pub fn FUN_004d7320(g: &mut G, this: Ptr, param_1: bool) -> i32 {
    let f = g.fish(this).clone();
    if f.field_0x70 != 0 {
        return if param_1 { 4 } else { 1 };
    }
    if f.field_0x74 < 1 && g.go(this).offset_0x80 < 0x65 {
        return if param_1 { 3 } else { 0 };
    }
    if param_1 { 6 } else { 2 }
}

/// port: 004e15e0 FUN_004e15e0
/// `DrawBody(Graphics*, bool mirror)`: the cel in its row (white when the virtual tank
/// variant 2 is not hungry), the hungry tint fading over it, what it carries, the
/// sparkle; the plain sheet's cel in the `DAT_005e89ce` mode.
pub fn FUN_004e15e0(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: bool) {
    let img = g.res.DAT_005e8da8;
    if !g.globals.DAT_005e89ce {
        let cel = g.fish(this).field_0x6c * 0x50;
        let hungry = FUN_004d6f30(g, this);
        let row = FUN_004d7320(g, this, hungry) * 0x50;
        let mut src = Rect::new(cel, row, 0x50, 0x50);
        if g.go(this).offset_0x74 && crate::game::game_object::FUN_004d6e70(g, this, gfx, img, &src, param_2) {
            return;
        }
        let color = if g.go(this).offset_0x9c == 2 && !hungry { crate::sexy::types::FUN_00433320(0xffffff) } else { Color::WHITE };
        FUN_004d6db0(g, this, gfx, color);
        FUN_004560a0(gfx, g, img, 0, 0, &src, param_2);
        let t = g.go(this).offset_0x18;
        if t != 0 {
            src.mY = FUN_004d7320(g, this, true) * 0x50;
            FUN_004d6db0(g, this, gfx, CRect(0xff, 0xff, 0xff, (t * 0xff) / 5));
            FUN_004560a0(gfx, g, img, 0, 0, &src, param_2);
        }
        FUN_004558e0(gfx, false);
        if g.go(this).offset_0x10 != NULL {
            FUN_004d7040(g, this, gfx, 0, 0);
        }
        if crate::game::game_object::FUN_004d6980(g, this, 500) {
            let s = g.res.DAT_005e8bc4;
            FUN_00456950(gfx, g, s, 0, 0, 2);
        }
    } else {
        let col = if FUN_004d6f30(g, this) { 6 } else { 9 };
        FUN_004560a0(gfx, g, img, 0, 0, &Rect::new(col * 0x50, 400, 0x50, 0x50), param_2);
        if g.go(this).offset_0x10 != NULL {
            FUN_004d7040(g, this, gfx, 0, 0);
        }
        if crate::game::game_object::FUN_004d6980(g, this, 500) {
            let s = g.res.DAT_005e8bc4;
            FUN_00456950(gfx, g, s, 0, 0, 2);
        }
    }
}

/// port: 004e17f0 Sexy::Gekko::vfunction84
/// `DrawFish(Graphics*, bool mirror)`: the body, flashing additively while +0x1cc counts
/// (every other 8 updates).
pub fn vfunction84(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: bool) {
    FUN_004e15e0(g, this, gfx, param_2);
    let n = g.fish(this).field_0x78;
    if 0 < n && (n / 8) % 2 == 0 {
        FUN_004558c0(gfx, 1);
        FUN_004e15e0(g, this, gfx, param_2);
        FUN_004558c0(gfx, 0);
    }
}

/// port: 004e1850 Sexy::Gekko::vfunction80
/// `DrawIcon(Graphics*, int pose)`: its portrait cel offset by pose.
pub fn vfunction80(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: i32) {
    let (dx, dy) = match param_2 {
        0 => (0x17, 0x14),
        1 => (0xf, 0x11),
        2 => (0x14, 0),
        3 => (0xc, 5),
        4 => (-0x28, 0),
        _ => (0, 0),
    };
    FUN_004563d0(gfx, dx, dy);
    let color = if g.go(this).offset_0x9c == 2 { crate::sexy::types::FUN_00433320(0xffffff) } else { Color::WHITE };
    crate::game::game_object::FUN_004d6d10(g, this, gfx, color);
    let img = g.res.DAT_005e8da8;
    let col = g.go(this).offset_0xa8 * 0x50;
    FUN_00455e40(gfx, g, img, 0, 0, &Rect::new(col, 0, 0x50, 0x50));
    FUN_004558e0(gfx, false);
    FUN_004563d0(gfx, -dx, -dy);
}
