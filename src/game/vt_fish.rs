//! The virtual tank's extra fish: `Sexy::SylvesterFish` (type 0x22, eats carnivores while
//! small, ultravores once grown), `Sexy::BallFish` (type 0x23, eats one kind of pellet per
//! size) and `Sexy::BiFish` (type 0x24, the two-headed fish that eats guppies). All three
//! are `Fish` (same data; object 0x230 bytes, BiFish 0x238 with `BiFish_data` at 0x22c,
//! field names relative to it), built by the Fish constructors and then given their own
//! vftable.

use crate::game::game_object::{FUN_004d6cc0, FUN_004d6d10, FUN_004d6db0, FUN_004d6f30, FUN_004d7040};
use crate::sexy::crt::ftol;
use crate::sexy::graphics::{FUN_004558e0, FUN_00455900, FUN_00455e40, FUN_004560a0, FUN_004563d0, FUN_00456950};
use crate::sexy::prelude::*;

/// `BiFish_data` (object offset 0x22c).
#[derive(Debug, Clone, Default)]
pub struct BiFish_data {
    /// +0x230 which head colouring (0 or 1; picks the sprite rows).
    pub offset_0x4: i32,
    /// +0x234 which head leads (flips with every turn).
    pub offset_0x8: i32,
}

impl G {
    pub fn bi_fish(&mut self, p: Ptr) -> &mut BiFish_data {
        match &mut self.go_ext(p).sub {
            GoSub::BiFish(_, d) => d,
            s => panic!("{p} is not a BiFish: {s:?}"),
        }
    }
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

fn center(g: &mut G, p: Ptr) -> (i32, i32) {
    let wc = g.wc(p);
    (wc.offset_0x34 / 2 + wc.offset_0x2c, wc.offset_0x38 / 2 + wc.offset_0x30)
}

/// A board object that may be eaten: not bought in the virtual tank and not dropping in.
fn edible(g: &mut G, o: Ptr) -> bool {
    g.go(o).offset_0x24 < 0 && g.go(o).offset_0x98 < 1
}

/// The fish's data becomes a BiFish's (the derived part of construction).
fn become_bifish(g: &mut G, this: Ptr) {
    let ext = g.go_ext(this);
    let f = match std::mem::replace(&mut ext.sub, GoSub::None) {
        GoSub::Fish(f) => f,
        s => panic!("{this} is not a plain Fish: {s:?}"),
    };
    ext.sub = GoSub::BiFish(f, BiFish_data::default());
}

/// The chase shared by the three (`vfunction85`): every 3 updates steer toward the
/// target's point `c` pixels into it (x by 1.3 / 0.2 / 0.05 within ±4 when hungry, else
/// 1.0 / 0.1 / 0.05 within ±3; y by 1.3 when hungry, else 1.0 / 0.5), then try to eat.
fn chase(g: &mut G, this: Ptr, c: i32, mouth_dy: i32) -> bool {
    let t = vcall!(g, this, fish.vfunction86);
    let (ix, iy) = center(g, this);
    let iy = iy + mouth_dy;
    if 2 < g.fish(this).field_0x5c {
        if t == NULL {
            return false;
        }
        let hunger = g.go(this).offset_0x14;
        g.fish(this).field_0x5c = 0;
        let (tx, ty) = (g.wc(t).offset_0x2c, g.wc(t).offset_0x30);
        let f = g.fish(this);
        let (big, med, lim) = if hunger < 0x12d { (1.3, 0.2, 4.0) } else { (1.0, 0.1, 3.0) };
        let v = &mut f.offset_0x14;
        if tx + c + 4 < ix {
            if -lim < *v {
                *v -= big;
            }
        } else if ix < tx + c - 4 {
            if *v < lim {
                *v += big;
            }
        } else if tx + c + 2 < ix {
            if -lim < *v {
                *v -= med;
            }
        } else if ix < tx + c - 2 {
            if *v < lim {
                *v += med;
            }
        } else if tx + c < ix {
            if -lim < *v {
                *v -= 0.05;
            }
        } else if ix < tx + c && *v < lim {
            *v += 0.05;
        }
        let v = &mut f.field_0x1c;
        if hunger < 0x12d {
            let ty = ty + c;
            if ty < iy {
                if -4.0 < *v {
                    *v -= 1.3;
                }
            } else if iy < ty && *v < 4.0 {
                *v += 1.3;
            }
        } else if ty + c + 3 < iy {
            if -3.0 < *v {
                *v -= 1.0;
            }
        } else if iy < ty + c - 3 {
            if *v < 3.0 {
                *v += 1.0;
            }
        } else if ty + c < iy {
            if -3.0 < *v {
                *v -= 0.5;
            }
        } else if iy < ty + c && *v < 3.0 {
            *v += 0.5;
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

/// The growth pulse of the draw (`field_0x78` counting down from 10): how far the cel is
/// stretched beyond its box (`base` = the scale at the pulse's start: 0.6 / 0.5).
fn pulse(flash: i32, base: f64, small: f64) -> i32 {
    let k = if 3 < flash {
        ((10 - flash) as f64 * f64::from_bits(0x3fe6666660000000) / 7.0 + base) as f32
    } else {
        (flash as f64 * small / 3.0 + 1.0) as f32
    };
    ftol((k as f64 - 1.0) * 160.0 * 0.5) as i32
}

/// The draw shared by Sylvester and the ball fish (`vfunction84`): the drop-in portal, or
/// the cel (stretched by the growth pulse), what it carries, and the hunger sparkle.
fn draw(g: &mut G, this: Ptr, gfx: &mut Graphics, img: Ptr, src: &Rect, flip: bool, size: i32, base: f64, small: f64, spark: (i32, i32)) {
    if g.go(this).offset_0x74 && g.fish(this).field_0x78 == 0 && crate::game::game_object::FUN_004d6e70(g, this, gfx, img, src, flip) {
        return;
    }
    FUN_004d6db0(g, this, gfx, Color::WHITE);
    let flash = g.fish(this).field_0x78;
    if flash < 1 {
        FUN_004560a0(gfx, g, img, 0, 0, src, flip);
    } else {
        let e = pulse(flash, base, small);
        let dest = Rect::new(-e, -e, e * 2 + size, e * 2 + size);
        let app = g.go(this).offset_0x0;
        let fast = !crate::sexy::sexy_app_base::dtor_MemoryImage__00489a20(g, app);
        FUN_00455900(gfx, fast);
        crate::game::shadow::FUN_005008f0(g, gfx, img, &dest, src, flip);
    }
    FUN_004558e0(gfx, false);
    if g.go(this).offset_0x10 != NULL {
        FUN_004d7040(g, this, gfx, 0, 0);
    }
    if crate::game::game_object::FUN_004d6980(g, this, 500) {
        let s = g.res.DAT_005e8bc4;
        FUN_00456950(gfx, g, s, spark.0, spark.1, 2);
    }
}

/// The swim animation's turning and facing shared by the three (`vfunction90` start).
fn turn(g: &mut G, this: Ptr, n: i32) {
    if g.go(this).offset_0x74 {
        crate::game::game_object::FUN_004d6c90(g, this);
    }
    let f = g.fish(this);
    if f.offset_0x34 <= 0.0 || 0.0 <= f.offset_0x14 {
        if f.offset_0x34 < 0.0 && 0.0 < f.offset_0x14 {
            f.field_0x70 = -n;
        }
    } else {
        f.field_0x70 = n;
    }
    if 0 < f.field_0x70 {
        f.field_0x70 -= 1;
    } else if f.field_0x70 < 0 {
        f.field_0x70 += 1;
    }
}

/// The facing follows the x speed (end of `vfunction90`).
fn follow(g: &mut G, this: Ptr) {
    let f = g.fish(this);
    if f.offset_0x14 != f.offset_0x34 && f.offset_0x14 != 0.0 && f.offset_0x34 != 0.0 {
        f.offset_0x34 = f.offset_0x14;
    }
}

/// The sparkles where the prey was in the +0x882 mode (not for food).
fn sparkles(g: &mut G, this: Ptr, prey: Ptr) {
    let app = g.go(this).offset_0x0;
    if !g.wfa(app).offset_0x156 || g.go(prey).offset_0x4 == 0x1c {
        return;
    }
    let mut n = rand(g, this) % 3 + 2;
    while n != 0 {
        let off = if g.go(prey).offset_0x4 == 6 { 0x37 } else { 0xf };
        let y = (rand(g, this) % 0x14) as i32 + off + g.wc(prey).offset_0x30;
        let x = (rand(g, this) % 0x14) as i32 + off + g.wc(prey).offset_0x2c;
        let board = board_of(g, this);
        crate::game::board_level::FUN_005436f0(g, board, x, y, 1);
        n -= 1;
    }
}

// ---------------------------------------------------------------- SylvesterFish

/// port: 004f01b0 Sexy::SylvesterFish::SylvesterFish
/// `SylvesterFish::SylvesterFish()` (used before loading from a save).
pub fn SylvesterFish__004f01b0(g: &mut G) -> Ptr {
    let this = crate::game::fish::Fish__004eee60(g);
    g.set_vt(this, &crate::sexy::vtables_gen::Sexy__SylvesterFish_vftable);
    g.go(this).offset_0x4 = 0x22;
    this
}

/// port: 004f01e0 Sexy::SylvesterFish::SylvesterFish
/// `SylvesterFish(int x, int y, bool facingRight)`.
pub fn SylvesterFish__004f01e0(g: &mut G, param_1: i32, param_2: i32, param_3: bool) -> Ptr {
    let this = crate::game::fish::Fish__004eef00(g, param_1, param_2);
    g.set_vt(this, &crate::sexy::vtables_gen::Sexy__SylvesterFish_vftable);
    FUN_004d95c0(g, this, param_1, param_2);
    let v = if !param_3 { -1.0 } else { 1.0 };
    let f = g.fish(this);
    f.offset_0x14 = v;
    f.offset_0x34 = v;
    this
}

/// port: 004d95c0 FUN_004d95c0
/// `SylvesterFish::Init(int x, int y)`: type 0x22, small (size 1), swims between 105 and
/// 360, mouse-visible as pets are, a coin every 200..449 updates (scaled).
pub fn FUN_004d95c0(g: &mut G, this: Ptr, _param_1: i32, _param_2: i32) {
    g.go(this).offset_0x4 = 0x22;
    {
        let f = g.fish(this);
        f.field_0xd5 = false;
        f.offset_0x4c = 1;
        f.field_0x44 = 0x69;
        f.field_0x3c = 0x168;
    }
    g.w(this).offset_0x1 = g.globals.DAT_005e8f14;
    let r = rand(g, this);
    let t = crate::game::fish::FUN_004d71d0(g, this, (r % 0xfa) as i32 + 200);
    g.fish(this).field_0xd0 = t;
}

/// port: 004d9d50 FUN_004d9d50
/// `SetSize(int)`: size 1 is an 80x80 fish swimming 105..360 (its shadow small); size 2 a
/// 160x160 one swimming 75..310 (its shadow big), re-centred on the same spot.
pub fn FUN_004d9d50(g: &mut G, this: Ptr, param_1: i32) {
    if param_1 != 1 && param_1 != 2 {
        return;
    }
    g.fish(this).offset_0x4c = param_1;
    let shadow = g.go(this).offset_0xc;
    if param_1 == 1 {
        g.wc(this).offset_0x34 = 0x50;
        g.wc(this).offset_0x38 = 0x50;
        let f = g.fish(this);
        f.field_0x4 += 40.0;
        f.field_0x44 = 0x69;
        f.field_0x3c = 0x168;
        f.field_0x40 = 10;
        f.field_0xc += 40.0;
        f.field_0x48 = 0x21c;
        if shadow != NULL {
            g.shadow(shadow).offset_0x8 = 0;
        }
    } else {
        g.wc(this).offset_0x34 = 0xa0;
        g.wc(this).offset_0x38 = 0xa0;
        let f = g.fish(this);
        f.field_0x4 -= 40.0;
        f.field_0x3c = 0x136;
        f.field_0x44 = 0x4b;
        f.field_0x40 = 0;
        f.field_0xc -= 40.0;
        f.field_0x48 = 0x1e0;
        if shadow != NULL {
            g.shadow(shadow).offset_0x8 = 2;
        }
    }
    let (x, y) = (g.fish(this).field_0x4, g.fish(this).field_0xc);
    g.wc(this).offset_0x2c = ftol(x) as i32;
    g.wc(this).offset_0x30 = ftol(y) as i32;
}

/// port: 004f0d40 Sexy::SylvesterFish::deleting_destructor
pub fn deleting_destructor__004f0d40(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_SylvesterFish(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 004f0260 Sexy::SylvesterFish::~SylvesterFish
pub fn dtor_SylvesterFish(g: &mut G, this: Ptr) {
    g.set_vt(this, &crate::sexy::vtables_gen::Sexy__SylvesterFish_vftable);
    crate::game::fish::dtor_Fish(g, this);
}

/// port: 004d9630 Sexy::SylvesterFish::vfunction73
/// The coin's worth (GameObject's), tripled once grown.
pub fn vfunction73__004d9630(g: &mut G, this: Ptr) -> i32 {
    let r = crate::game::game_object::vfunction73(g, this);
    if 0 < r && g.fish(this).offset_0x4c == 2 {
        return r * 3;
    }
    r
}

/// port: 004d9650 Sexy::SylvesterFish::vfunction82
/// `CoinTimer()`: a gold coin while small, a star coin once grown (coin kinds 6 / 7), when
/// the timer is up and it is not hungry.
pub fn vfunction82__004d9650(g: &mut G, this: Ptr) {
    g.fish(this).field_0xcc += 1;
    let mut t = g.fish(this).field_0xcc;
    let period = g.fish(this).field_0xd0;
    let due = crate::game::game_object::FUN_004d7100(g, this, &mut t, period);
    g.fish(this).field_0xcc = t;
    if due {
        g.fish(this).field_0xcc = 0;
        if crate::game::game_object::FUN_004d6bd0(g, this) {
            let kind = (g.fish(this).offset_0x4c != 1) as i32 + 6;
            let (x, y) = (g.wc(this).offset_0x2c + 5, g.wc(this).offset_0x30 + 10);
            let board = board_of(g, this);
            crate::game::board_level::FUN_00544430(g, board, x, y, kind, NULL, -1.0, 0);
        }
    }
}

/// port: 004d96d0 Sexy::SylvesterFish::vfunction71
/// Counts it in `stats[3]` while small, else `stats[4]`.
pub fn vfunction71__004d96d0(g: &mut G, this: Ptr, param_1: &mut [i32]) {
    if g.fish(this).offset_0x4c == 1 {
        param_1[3] += 1;
        return;
    }
    param_1[4] += 1;
}

/// port: 004d9700 Sexy::SylvesterFish::vfunction90
/// `Animate()`: turning (10 updates); swimming fast it lunges in strokes (a short pull
/// back, a burst, a glide, a rest), otherwise the plain swim cels; the growth pulse; the
/// facing; the rest pose when idle long; what it carries follows.
pub fn vfunction90__004d9700(g: &mut G, this: Ptr) {
    turn(g, this, 10);
    let mut div = g.fish(this).field_0x2c;
    if g.go(this).offset_0x6e {
        div = if g.go(this).offset_0x70 == 0 { 0.3 } else { 0.8 };
    }
    let t = g.fish(this).field_0x70;
    'end: {
        let f = g.fish(this);
        if t == 0 {
            if f.offset_0x14 < -1.6 || 1.6 < f.offset_0x14 {
                f.field_0x64 += 1;
                let c = f.field_0x64;
                if c < 7 {
                    f.field_0x6c = c / 2;
                    f.field_0x4 -= (f.offset_0x14 / div) * 0.5 * 0.5;
                    f.field_0xc -= (f.field_0x1c / div) * 0.5 * 0.5;
                } else if c < 0xb {
                    f.field_0x6c = c / 2;
                    f.field_0x4 += (f.offset_0x14 / div) * 0.5 * 1.5;
                    f.field_0xc += (f.field_0x1c / div) * 0.5 * 1.5;
                } else {
                    if c < 0x28 {
                        f.field_0x6c = 6;
                    } else {
                        if 0x32 < c {
                            f.field_0x6c = 0;
                            f.field_0x64 = 0;
                            break 'end;
                        }
                        f.field_0x6c = (c - 0x32).wrapping_abs() / 2;
                    }
                    f.field_0x4 += (f.offset_0x14 / div) * 0.5;
                    f.field_0xc += (f.field_0x1c / div) * 0.5;
                }
            } else {
                f.field_0x64 += 1;
                if 0x13 < f.field_0x64 {
                    f.field_0x64 = 0;
                }
                f.field_0x6c = f.field_0x64 / 2;
            }
        } else if 0 < t {
            f.field_0x6c = 9 - t;
        } else {
            f.field_0x6c = t + 10;
        }
    }
    if 0 < g.fish(this).field_0x78 {
        g.fish(this).field_0x78 -= 1;
    }
    follow(g, this);
    if 100 < g.go(this).offset_0x80 && t == 0 {
        g.fish(this).field_0x6c = 0;
    }
    if g.go(this).offset_0x10 != NULL {
        crate::game::game_object::FUN_004d7020(g, this);
    }
}

/// port: 004d99c0 Sexy::SylvesterFish::vfunction85
/// `Chase()`: toward the prey's middle (its own mouth 30 higher once grown).
pub fn vfunction85__004d99c0(g: &mut G, this: Ptr) -> bool {
    let dy = if g.fish(this).offset_0x4c != 1 { -0x1e } else { 0 };
    chase(g, this, 0x24, dy)
}

/// port: 004db930 Sexy::SylvesterFish::vfunction83
/// `Hunt()` of the three: hunger ticks (and, with no aliens, the hungry look); below 500 it
/// chases.
pub fn vfunction83__004db930(g: &mut G, this: Ptr) -> bool {
    crate::game::game_object::FUN_004d6c50(g, this);
    let board = board_of(g, this);
    let b = g.board(board);
    if b.offset_0xc[crate::game::board_level::vec_index(0xb8)].is_empty() && b.offset_0xc[crate::game::board_level::vec_index(0xf4)].is_empty() {
        crate::game::game_object::FUN_004d6ef0(g, this);
    }
    if g.go(this).offset_0x14 < 500 {
        return vcall!(g, this, fish.vfunction85);
    }
    false
}

/// port: 004e3420 Sexy::SylvesterFish::vfunction84
/// `DrawFish(Graphics*, bool mirror)`: the small (80) or big (160) sheet, the turning row.
pub fn vfunction84__004e3420(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: bool) {
    let (img, s) = if g.fish(this).offset_0x4c == 1 { (g.res.DAT_005e8c0c, 0x50) } else { (g.res.DAT_005e8ebc, 0xa0) };
    let f = g.fish(this).clone();
    let src = Rect::new(f.field_0x6c * s, s * (f.field_0x70 != 0) as i32, s, s);
    draw(g, this, gfx, img, &src, param_2, 0xa0, 0.6, f64::from_bits(0x3fd3333340000000), (-10, -15));
}

/// port: 004e35c0 Sexy::SylvesterFish::vfunction80
/// `DrawIcon(Graphics*, int pose)`: the portrait cel (+0x130) offset by pose and size.
pub fn vfunction80__004e35c0(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: i32) {
    let (img, s, (dx, dy)) = if g.fish(this).offset_0x4c == 1 {
        let o = match param_2 {
            0 => (0x14, 0x11),
            1 => (0xf, 0x11),
            2 => (0x12, 0),
            3 => (0xc, 5),
            4 => (-0x28, 0),
            _ => (0, 0),
        };
        (g.res.DAT_005e8c0c, 0x50, o)
    } else {
        let o = match param_2 {
            0 => (-0x14, 0),
            1 => (-0x19, 0),
            2 => (-0x14, -10),
            3 => (-0x1e, -10),
            4 => (-0x50, -0x14),
            _ => (0, 0),
        };
        (g.res.DAT_005e8ebc, 0xa0, o)
    };
    FUN_004563d0(gfx, dx, dy);
    FUN_004d6d10(g, this, gfx, Color::WHITE);
    let col = g.go(this).offset_0xa8 * s;
    FUN_00455e40(gfx, g, img, 0, 0, &Rect::new(col, 0, s, s));
    FUN_004558e0(gfx, false);
    FUN_004563d0(gfx, -dx, -dy);
}

/// port: 004e3720 Sexy::SylvesterFish::vfunction78
/// `Eat(GameObject*)`: the chomp, fed (+700, at most 1000), sparkles; counted in the
/// virtual tank, and enough meals grow a small one (the growth sound and pulse).
pub fn vfunction78__004e3720(g: &mut G, this: Ptr, param_1: i32) {
    let prey = param_1 as Ptr;
    let hungry = FUN_004d6f30(g, this);
    crate::game::game_object::FUN_004d6a30(g, this, false);
    let board = board_of(g, this);
    let high = g.go(this).offset_0x7c;
    crate::game::board_level::FUN_00538560(g, board, high);
    g.go(this).offset_0x14 += 700;
    if 1000 < g.go(this).offset_0x14 {
        g.go(this).offset_0x14 = 1000;
    }
    sparkles(g, this, prey);
    if crate::game::game_object::FUN_004d6f90(g, this) {
        g.fish(this).field_0x50 += 1;
        if g.fish(this).offset_0x4c == 1 && g.fish(this).field_0x54 <= g.fish(this).field_0x50 {
            FUN_004d9d50(g, this, 2);
            g.fish(this).field_0x50 = 0;
            g.fish(this).field_0x78 = 10;
            let board = board_of(g, this);
            crate::game::board::FUN_00538230(g, board, 0x122, 3, 1.0);
        }
    }
    crate::game::game_object::FUN_004e13d0(g, this, hungry);
}

/// port: 004ec790 Sexy::SylvesterFish::vfunction86
/// `FindPrey()`: the nearest carnivore while small, ultravore once grown (not bought, not
/// dropping in); within 100 pixels the mouth opens. In the feeding mode, the feeder's
/// target instead.
pub fn vfunction86__004ec790(g: &mut G, this: Ptr) -> Ptr {
    let (cx, cy) = center(g, this);
    let small = g.fish(this).offset_0x4c == 1;
    let cy = if small { cy } else { cy - 0x1e };
    if g.go(this).offset_0x68 != 0 {
        return crate::game::game_object::FUN_004ea010(g, this, cx, cy);
    }
    let board = board_of(g, this);
    let objs: Vec<Ptr> = g.board(board).offset_0x7c.iter().copied().collect();
    let mut best = 100000000;
    let mut prey = NULL;
    for o in objs {
        if !edible(g, o) {
            continue;
        }
        let ok = match g.go(o).offset_0x4 {
            5 => small,
            6 => !small,
            _ => false,
        };
        if ok {
            let (ox, oy) = center(g, o);
            let (dx, dy) = (cx - ox, cy - oy);
            let d = dy * dy + dx * dx;
            if d < best {
                prey = o;
                best = d;
            }
        }
    }
    if best < 10000 {
        FUN_004d6cc0(g, this);
        crate::game::game_object::FUN_004d6b00(g, this, 0x96);
        g.go(this).offset_0x70 = 100;
    }
    prey
}

/// port: 004ec900 Sexy::SylvesterFish::vfunction87
/// `TryEat()`: eats the first prey whose middle is within 30 of its mouth (70 for an
/// ultravore). In the feeding mode, the feeder's check instead.
pub fn vfunction87__004ec900(g: &mut G, this: Ptr) {
    let (cx, cy) = center(g, this);
    let small = g.fish(this).offset_0x4c == 1;
    let cy = if small { cy } else { cy - 0x1e };
    if g.go(this).offset_0x68 != 0 {
        crate::game::game_object::FUN_004ea2a0(g, this, cx, cy);
        return;
    }
    let board = board_of(g, this);
    let objs: Vec<Ptr> = g.board(board).offset_0x7c.iter().copied().collect();
    for o in objs {
        if !edible(g, o) {
            continue;
        }
        let r = match g.go(o).offset_0x4 {
            5 if small => 0x1e,
            6 if !small => 0x46,
            _ => continue,
        };
        let (ox, oy) = center(g, o);
        let (dx, dy) = (cx - ox, cy - oy);
        if -r < dx && dx < r && -r < dy && dy < r {
            vcall!(g, this, go.vfunction78, o as i32);
            vcall!(g, o, go.vfunction76);
            return;
        }
    }
}

// ---------------------------------------------------------------- BallFish

/// port: 004f0550 Sexy::BallFish::BallFish
/// `BallFish::BallFish()` (used before loading from a save).
pub fn BallFish__004f0550(g: &mut G) -> Ptr {
    let this = crate::game::fish::Fish__004eee60(g);
    g.set_vt(this, &crate::sexy::vtables_gen::Sexy__BallFish_vftable);
    g.go(this).offset_0x4 = 0x23;
    this
}

/// port: 004f0570 Sexy::BallFish::BallFish
/// `BallFish(int x, int y, bool facingRight)`.
pub fn BallFish__004f0570(g: &mut G, param_1: i32, param_2: i32, param_3: bool) -> Ptr {
    let this = crate::game::fish::Fish__004eef00(g, param_1, param_2);
    g.set_vt(this, &crate::sexy::vtables_gen::Sexy__BallFish_vftable);
    FUN_004db2a0(g, this, param_1, param_2);
    let v = if !param_3 { -1.0 } else { 1.0 };
    let f = g.fish(this);
    f.offset_0x14 = v;
    f.offset_0x34 = v;
    this
}

/// port: 004db2a0 FUN_004db2a0
/// `BallFish::Init(int x, int y)`: type 0x23, size 0, 50x50, swims between 105 and 360,
/// mouse-visible as pets are, a coin every 200..449 updates (scaled).
pub fn FUN_004db2a0(g: &mut G, this: Ptr, _param_1: i32, _param_2: i32) {
    g.go(this).offset_0x4 = 0x23;
    {
        let f = g.fish(this);
        f.field_0xd5 = false;
        f.offset_0x4c = 0;
        f.field_0x44 = 0x69;
        f.field_0x3c = 0x168;
    }
    g.w(this).offset_0x1 = g.globals.DAT_005e8f14;
    let r = rand(g, this);
    let t = crate::game::fish::FUN_004d71d0(g, this, (r % 0xfa) as i32 + 200);
    g.fish(this).field_0xd0 = t;
    g.wc(this).offset_0x34 = 0x32;
    g.wc(this).offset_0x38 = 0x32;
}

/// port: 004f05f0 Sexy::BallFish::~BallFish
pub fn dtor_BallFish(g: &mut G, this: Ptr) {
    g.set_vt(this, &crate::sexy::vtables_gen::Sexy__BallFish_vftable);
    crate::game::fish::dtor_Fish(g, this);
}

/// port: 004f0e10 Sexy::BallFish::deleting_destructor
pub fn deleting_destructor__004f0e10(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_BallFish(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 004d4750 Sexy::BallFish::vfunction71
/// Counts it in `stats[7]` at size 0, `stats[5]` at 1, `stats[6]` at 2.
pub fn vfunction71__004d4750(g: &mut G, this: Ptr, param_1: &mut [i32]) {
    match g.fish(this).offset_0x4c {
        0 => param_1[7] += 1,
        1 => param_1[5] += 1,
        _ => param_1[6] += 1,
    }
}

/// port: 004d47c0 Sexy::BallFish::vfunction85
/// `Chase()`: toward the pellet's middle.
pub fn vfunction85__004d47c0(g: &mut G, this: Ptr) -> bool {
    chase(g, this, 0x14, 0)
}

/// port: 004d4790 FUN_004d4790
/// The pellet kind a ball fish eats: 4 at size 0, 5 at size 1, 3 at size 2.
pub fn FUN_004d4790(g: &mut G, this: Ptr) -> i32 {
    match g.fish(this).offset_0x4c {
        0 => 4,
        1 => 5,
        _ => 3,
    }
}

/// port: 00501020 FUN_00501020
/// A food pellet (type 0x1c) of kind `param_2` that is still falling free (+0x198 = 0).
pub fn FUN_00501020(g: &mut G, param_1: Ptr, param_2: i32) -> bool {
    if g.go(param_1).offset_0x4 != 0x1c {
        return false;
    }
    g.food(param_1).offset_0x44 == 0 && g.food(param_1).offset_0x40 == param_2
}

/// port: 004db320 Sexy::BallFish::vfunction82
/// `CoinTimer()`: a silver, gold or diamond-sized coin by size (kinds 1 / 2 / 4) when the
/// timer is up and it is not hungry.
pub fn vfunction82__004db320(g: &mut G, this: Ptr) {
    g.fish(this).field_0xcc += 1;
    let mut t = g.fish(this).field_0xcc;
    let period = g.fish(this).field_0xd0;
    let due = crate::game::game_object::FUN_004d7100(g, this, &mut t, period);
    g.fish(this).field_0xcc = t;
    if due {
        g.fish(this).field_0xcc = 0;
        if crate::game::game_object::FUN_004d6bd0(g, this) {
            let kind = match g.fish(this).offset_0x4c {
                1 => 2,
                2 => 4,
                _ => 1,
            };
            let (x, y) = (g.wc(this).offset_0x2c + 5, g.wc(this).offset_0x30 + 10);
            let board = board_of(g, this);
            crate::game::board_level::FUN_00544430(g, board, x, y, kind, NULL, -1.0, 0);
        }
    }
}

fn ball_row(g: &mut G, this: Ptr) -> i32 {
    match g.fish(this).offset_0x4c {
        1 => 2,
        2 => 1,
        _ => 0,
    }
}

/// port: 004db3b0 Sexy::BallFish::vfunction84
/// `DrawFish(Graphics*, bool mirror)`: the 50x50 cel of its size's row.
pub fn vfunction84__004db3b0(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: bool) {
    let row = ball_row(g, this) * 0x32;
    let src = Rect::new(g.fish(this).field_0x6c * 0x32, row, 0x32, 0x32);
    let img = g.res.DAT_005e8ee8;
    draw(g, this, gfx, img, &src, param_2, 0x32, 0.5, f64::from_bits(0x3fc99999a0000000), (-0xf, -0x14));
}

/// port: 004db550 Sexy::BallFish::vfunction80
/// `DrawIcon(Graphics*, int pose)`: the portrait cel (+0x130) of its size's row.
pub fn vfunction80__004db550(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: i32) {
    let row = ball_row(g, this);
    let (dx, dy) = match param_2 {
        0 => (0x23, 0x23),
        1 => (0x14, 0x1e),
        2 => (0x1e, 10),
        3 => (0x14, 0xf),
        4 => (-0x1c, 10),
        _ => (0, 0),
    };
    FUN_004563d0(gfx, dx, dy);
    FUN_004d6d10(g, this, gfx, Color::WHITE);
    let col = g.go(this).offset_0xa8 * 0x32;
    let img = g.res.DAT_005e8ee8;
    FUN_00455e40(gfx, g, img, 0, 0, &Rect::new(col, row * 0x32, 0x32, 0x32));
    FUN_004558e0(gfx, false);
    FUN_004563d0(gfx, -dx, -dy);
}

/// port: 004db650 Sexy::BallFish::vfunction79
/// `UpdateIcon()`: the screensaver tick; the portrait cel turns every 4 ticks.
pub fn vfunction79__004db650(g: &mut G, this: Ptr) {
    crate::game::game_object::FUN_004d6cf0(g, this);
    let v = g.go(this).offset_0xa4 / 4;
    g.go(this).offset_0xa8 = v % 10;
}

/// port: 004db680 Sexy::BallFish::vfunction90
/// `Animate()`: turning (10 updates); it rolls with its speed (faster the faster it swims,
/// backwards when swimming left); the growth pulse; the facing; what it carries follows.
pub fn vfunction90__004db680(g: &mut G, this: Ptr) {
    turn(g, this, 10);
    let f = g.fish(this);
    let vx = f.offset_0x14;
    f.field_0x64 += if vx < -1.0 {
        -3
    } else if 1.0 < vx {
        3
    } else if 0.5 < vx {
        2
    } else if vx < -0.5 {
        -2
    } else if vx < 0.0 {
        -1
    } else {
        1
    };
    let per = if f.offset_0x4c != 1 { 6 } else { 0xc };
    if f.field_0x64 < 0 {
        f.field_0x64 += per * 10;
    }
    f.field_0x6c = (f.field_0x64 / per) % 10;
    if 0 < f.field_0x78 {
        f.field_0x78 -= 1;
    }
    follow(g, this);
    if g.go(this).offset_0x10 != NULL {
        crate::game::game_object::FUN_004d7020(g, this);
    }
}

/// port: 004e3f90 Sexy::BallFish::vfunction78
/// `Eat(Food*)`: the munch, fed (+700, at most 1000); counted in the virtual tank, and
/// enough meals grow it a size (the growth sound and pulse), up to size 2.
pub fn vfunction78__004e3f90(g: &mut G, this: Ptr, _param_1: i32) {
    let hungry = FUN_004d6f30(g, this);
    crate::game::game_object::FUN_004d6a30(g, this, false);
    let board = board_of(g, this);
    let high = g.go(this).offset_0x7c;
    crate::game::board_level::FUN_005384c0(g, board, high);
    g.go(this).offset_0x14 += 700;
    if 1000 < g.go(this).offset_0x14 {
        g.go(this).offset_0x14 = 1000;
    }
    if crate::game::game_object::FUN_004d6f90(g, this) {
        g.fish(this).field_0x50 += 1;
        let size = g.fish(this).offset_0x4c;
        if size < 2 && g.fish(this).field_0x54 <= g.fish(this).field_0x50 {
            let f = g.fish(this);
            f.offset_0x4c = size + 1;
            f.field_0x50 = 0;
            f.field_0x78 = 10;
            let board = board_of(g, this);
            crate::game::board::FUN_00538230(g, board, 0x122, 3, 1.0);
        }
    }
    crate::game::game_object::FUN_004e13d0(g, this, hungry);
}

/// port: 004ed770 Sexy::BallFish::vfunction86
/// `FindPrey()`: the nearest free-falling pellet of its kind; within 100 pixels the mouth
/// opens. In the feeding mode, the feeder's target instead.
pub fn vfunction86__004ed770(g: &mut G, this: Ptr) -> Ptr {
    let (cx, cy) = center(g, this);
    if g.go(this).offset_0x68 != 0 {
        return crate::game::game_object::FUN_004ea010(g, this, cx, cy);
    }
    let mut best = 100000000;
    let mut prey = NULL;
    let kind = FUN_004d4790(g, this);
    let board = board_of(g, this);
    let objs: Vec<Ptr> = g.board(board).offset_0x7c.iter().copied().collect();
    for o in objs {
        if edible(g, o) && g.go(o).offset_0x4 == 0x1c && FUN_00501020(g, o, kind) {
            let (ox, oy) = center(g, o);
            let (dx, dy) = (cx - ox, cy - oy);
            let d = dy * dy + dx * dx;
            if d < best {
                prey = o;
                best = d;
            }
        }
    }
    if best < 10000 {
        g.go(this).offset_0x70 = 100;
    }
    prey
}

/// port: 004ed8b0 Sexy::BallFish::vfunction87
/// `TryEat()`: eats the first pellet of its kind within 30 of its middle (opening the
/// mouth). In the feeding mode, the feeder's check (opening the mouth on a hit).
pub fn vfunction87__004ed8b0(g: &mut G, this: Ptr) {
    let (cx, cy) = center(g, this);
    if g.go(this).offset_0x68 != 0 {
        if 0 < crate::game::game_object::FUN_004ea2a0(g, this, cx, cy) {
            FUN_004d6cc0(g, this);
        }
        return;
    }
    let kind = FUN_004d4790(g, this);
    let board = board_of(g, this);
    let objs: Vec<Ptr> = g.board(board).offset_0x7c.iter().copied().collect();
    for o in objs {
        if edible(g, o) && g.go(o).offset_0x4 == 0x1c && FUN_00501020(g, o, kind) {
            let (ox, oy) = center(g, o);
            let (dx, dy) = (cx - ox, cy - oy);
            if -0x1e < dx && dx < 0x1e && -0x1e < dy && dy < 0x1e {
                FUN_004d6cc0(g, this);
                vcall!(g, this, go.vfunction78, o as i32);
                vcall!(g, o, go.vfunction76);
                return;
            }
        }
    }
}

// ---------------------------------------------------------------- BiFish

/// port: 004f0600 Sexy::BiFish::BiFish
/// `BiFish::BiFish()` (used before loading from a save).
pub fn BiFish__004f0600(g: &mut G) -> Ptr {
    let this = crate::game::fish::Fish__004eee60(g);
    become_bifish(g, this);
    g.set_vt(this, &crate::sexy::vtables_gen::Sexy__BiFish_vftable);
    g.go(this).offset_0x4 = 0x24;
    this
}

/// port: 004f0630 Sexy::BiFish::BiFish
/// `BiFish(int x, int y, bool facingRight)`.
pub fn BiFish__004f0630(g: &mut G, param_1: i32, param_2: i32, param_3: bool) -> Ptr {
    let this = crate::game::fish::Fish__004eef00(g, param_1, param_2);
    become_bifish(g, this);
    g.set_vt(this, &crate::sexy::vtables_gen::Sexy__BiFish_vftable);
    FUN_004db820(g, this, param_1, param_2);
    let v = if !param_3 { -1.0 } else { 1.0 };
    let f = g.fish(this);
    f.offset_0x14 = v;
    f.offset_0x34 = v;
    this
}

/// port: 004db820 FUN_004db820
/// `BiFish::Init(int x, int y)`: type 0x24, a random colouring, 80x80, swims between 105
/// and 360, mouse-visible as pets are, a coin every 200..449 updates (scaled).
pub fn FUN_004db820(g: &mut G, this: Ptr, _param_1: i32, _param_2: i32) {
    g.go(this).offset_0x4 = 0x24;
    let c = crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32 % 2;
    {
        let d = g.bi_fish(this);
        d.offset_0x4 = c;
        d.offset_0x8 = 0;
    }
    {
        let f = g.fish(this);
        f.field_0xd5 = false;
        f.offset_0x4c = 0;
        f.field_0x44 = 0x69;
        f.field_0x3c = 0x168;
    }
    g.w(this).offset_0x1 = g.globals.DAT_005e8f14;
    let r = rand(g, this);
    let t = crate::game::fish::FUN_004d71d0(g, this, (r % 0xfa) as i32 + 200);
    g.fish(this).field_0xd0 = t;
    g.wc(this).offset_0x34 = 0x50;
    g.wc(this).offset_0x38 = 0x50;
}

/// port: 004f06b0 Sexy::BiFish::~BiFish
pub fn dtor_BiFish(g: &mut G, this: Ptr) {
    g.set_vt(this, &crate::sexy::vtables_gen::Sexy__BiFish_vftable);
    crate::game::fish::dtor_Fish(g, this);
}

/// port: 004f0e40 Sexy::BiFish::deleting_destructor
pub fn deleting_destructor__004f0e40(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_BiFish(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 004d4b40 Sexy::BiFish::vfunction85
/// `Chase()`: toward the guppy's middle.
pub fn vfunction85__004d4b40(g: &mut G, this: Ptr) -> bool {
    chase(g, this, 0x28, 0)
}

/// port: 004db8c0 Sexy::BiFish::vfunction82
/// `CoinTimer()`: a gold coin (kind 2) when the timer is up and it is not hungry.
pub fn vfunction82__004db8c0(g: &mut G, this: Ptr) {
    g.fish(this).field_0xcc += 1;
    let mut t = g.fish(this).field_0xcc;
    let period = g.fish(this).field_0xd0;
    let due = crate::game::game_object::FUN_004d7100(g, this, &mut t, period);
    g.fish(this).field_0xcc = t;
    if due {
        g.fish(this).field_0xcc = 0;
        if crate::game::game_object::FUN_004d6bd0(g, this) {
            let (x, y) = (g.wc(this).offset_0x2c + 5, g.wc(this).offset_0x30 + 10);
            let board = board_of(g, this);
            crate::game::board_level::FUN_00544430(g, board, x, y, 2, NULL, -1.0, 0);
        }
    }
}

/// port: 004db9b0 Sexy::BiFish::vfunction84
/// `DrawFish(Graphics*, bool mirror)`: its colouring's swim row, or while turning the
/// turning row (the other head's turn mirrored).
pub fn vfunction84__004db9b0(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: bool) {
    let b = g.bi_fish(this).clone();
    let (r0, r1, r2) = if b.offset_0x4 == 1 { (0, 1, 2) } else { (3, 4, 5) };
    let mut row = r2;
    let mut flip = param_2;
    if g.fish(this).field_0x70 != 0 {
        row = r1;
        if b.offset_0x8 != 0 {
            flip = !param_2;
            row = r0;
        }
    }
    let img = g.res.DAT_005e8f00;
    let src = Rect::new(g.fish(this).field_0x6c * 0x50, row * 0x50, 0x50, 0x50);
    if g.go(this).offset_0x74 && crate::game::game_object::FUN_004d6e70(g, this, gfx, img, &src, flip) {
        return;
    }
    FUN_004d6db0(g, this, gfx, Color::WHITE);
    FUN_004560a0(gfx, g, img, 0, 0, &src, flip);
    FUN_004558e0(gfx, false);
    if g.go(this).offset_0x10 != NULL {
        FUN_004d7040(g, this, gfx, 0, 0);
    }
    if crate::game::game_object::FUN_004d6980(g, this, 500) {
        let s = g.res.DAT_005e8bc4;
        FUN_00456950(gfx, g, s, 0, -10, 2);
    }
}

/// port: 004dbaf0 Sexy::BiFish::vfunction80
/// `DrawIcon(Graphics*, int pose)`: the portrait cel (+0x130) of its colouring's row.
pub fn vfunction80__004dbaf0(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: i32) {
    let (row, (dx, dy)) = if g.bi_fish(this).offset_0x4 != 0 {
        let o = match param_2 {
            0 => (0xf, 0x14),
            1 => (5, 0xf),
            2 => (0xf, 0),
            3 => (5, 0),
            4 => (-0x28, -5),
            _ => (0, 0),
        };
        (2, o)
    } else {
        let o = match param_2 {
            0 => (0xf, 0x14),
            1 => (5, 0xf),
            2 => (0xf, 5),
            3 => (5, 5),
            4 => (-0x28, -5),
            _ => (0, 0),
        };
        (5, o)
    };
    FUN_004563d0(gfx, dx, dy);
    FUN_004d6d10(g, this, gfx, Color::WHITE);
    let col = g.go(this).offset_0xa8 * 0x50;
    let img = g.res.DAT_005e8f00;
    FUN_00455e40(gfx, g, img, 0, 0, &Rect::new(col, row * 0x50, 0x50, 0x50));
    FUN_004558e0(gfx, false);
    FUN_004563d0(gfx, -dx, -dy);
}

/// port: 004dbc20 Sexy::BiFish::vfunction79
/// `UpdateIcon()`: the screensaver tick; the portrait swims (colouring 0) or ping-pongs.
pub fn vfunction79__004dbc20(g: &mut G, this: Ptr) {
    crate::game::game_object::FUN_004d6cf0(g, this);
    if 0x13 < g.go(this).offset_0xa4 {
        g.go(this).offset_0xa4 = 0;
    }
    let c = g.go(this).offset_0xa4;
    if g.bi_fish(this).offset_0x4 == 0 {
        g.go(this).offset_0xa8 = c / 2;
        return;
    }
    let v = if c - 10 < 0 { c - 9 } else { c - 10 };
    g.go(this).offset_0xa8 = v.wrapping_abs();
}

/// port: 004dbc80 Sexy::BiFish::vfunction90
/// `Animate()`: reversing starts a 20-update turn (and the other head leads); the swim
/// cels (colouring 0 plain, 1 ping-pong) or the turning cels; the facing; the rest pose
/// when idle long; what it carries follows.
pub fn vfunction90__004dbc80(g: &mut G, this: Ptr) {
    if g.go(this).offset_0x74 {
        crate::game::game_object::FUN_004d6c90(g, this);
    }
    let lead = g.bi_fish(this).offset_0x8;
    let f = g.fish(this);
    let mut swap = false;
    if f.offset_0x34 <= 0.0 || 0.0 <= f.offset_0x14 || f.field_0x70 != 0 {
        if f.offset_0x34 < 0.0 && 0.0 < f.offset_0x14 && f.field_0x70 == 0 {
            f.field_0x70 = -0x14;
            swap = true;
        }
    } else {
        f.field_0x70 = 0x14;
        swap = true;
    }
    if 0 < f.field_0x70 {
        f.field_0x70 -= 1;
    } else if f.field_0x70 < 0 {
        f.field_0x70 += 1;
    }
    if swap {
        g.bi_fish(this).offset_0x8 = (lead + 1) % 2;
    }
    let colour = g.bi_fish(this).offset_0x4;
    let f = g.fish(this);
    let t = f.field_0x70;
    if t == 0 {
        f.field_0x64 += 1;
        if 0x13 < f.field_0x64 {
            f.field_0x64 = 0;
        }
        let c = f.field_0x64;
        f.field_0x6c = if colour == 0 { c / 2 } else { (if c - 10 < 0 { c - 9 } else { c - 10 }).wrapping_abs() };
    } else if 0 < t {
        f.field_0x6c = 9 - t / 2;
    } else {
        f.field_0x6c = t / 2 + 9;
    }
    follow(g, this);
    if 100 < g.go(this).offset_0x80 && t == 0 {
        g.fish(this).field_0x6c = if colour != 1 { 0 } else { 9 };
    }
    if g.go(this).offset_0x10 != NULL {
        crate::game::game_object::FUN_004d7020(g, this);
    }
}

/// port: 004e4060 Sexy::BiFish::vfunction81
/// `Sync(DataSync&)`: the Fish part, then its colouring and leading head.
pub fn vfunction81__004e4060(g: &mut G, this: Ptr, sync: &mut crate::sexy::data_sync::DataSync) -> crate::sexy::data_sync::SyncResult {
    use crate::sexy::data_sync::FUN_00503010;
    crate::game::fish::vfunction81(g, this, sync)?;
    let mut d = g.bi_fish(this).clone();
    let r = FUN_00503010(sync, &mut d.offset_0x4).and_then(|_| FUN_00503010(sync, &mut d.offset_0x8));
    *g.bi_fish(this) = d;
    r
}

/// port: 004e40a0 Sexy::BiFish::vfunction78
/// `Eat(GameObject*)`: the chomp, fed (+700, at most 1000), sparkles; counted in the
/// virtual tank.
pub fn vfunction78__004e40a0(g: &mut G, this: Ptr, param_1: i32) {
    let prey = param_1 as Ptr;
    let hungry = FUN_004d6f30(g, this);
    crate::game::game_object::FUN_004d6a30(g, this, false);
    let board = board_of(g, this);
    let high = g.go(this).offset_0x7c;
    crate::game::board_level::FUN_00538560(g, board, high);
    g.go(this).offset_0x14 += 700;
    if 1000 < g.go(this).offset_0x14 {
        g.go(this).offset_0x14 = 1000;
    }
    sparkles(g, this, prey);
    if -1 < g.go(this).offset_0x24 {
        crate::game::game_object::FUN_004d6f90(g, this);
    }
    crate::game::game_object::FUN_004e13d0(g, this, hungry);
}

/// port: 004eda00 Sexy::BiFish::vfunction86
/// `FindPrey()`: the nearest guppy (not bought, not dropping in); within 100 pixels the
/// mouth opens. In the feeding mode, the feeder's target instead.
pub fn vfunction86__004eda00(g: &mut G, this: Ptr) -> Ptr {
    let (cx, cy) = center(g, this);
    if g.go(this).offset_0x68 != 0 {
        return crate::game::game_object::FUN_004ea010(g, this, cx, cy);
    }
    let board = board_of(g, this);
    let objs: Vec<Ptr> = g.board(board).offset_0x7c.iter().copied().collect();
    let mut best = 100000000;
    let mut prey = NULL;
    for o in objs {
        if edible(g, o) && g.go(o).offset_0x4 == 0 {
            let (ox, oy) = center(g, o);
            let (dx, dy) = (cx - ox, cy - oy);
            let d = dy * dy + dx * dx;
            if d < best {
                prey = o;
                best = d;
            }
        }
    }
    if best < 10000 {
        crate::game::game_object::FUN_004d6b00(g, this, 0xaf);
        g.go(this).offset_0x70 = 100;
    }
    prey
}

/// port: 004edb30 Sexy::BiFish::vfunction87
/// `TryEat()`: eats the first guppy within 30 of its middle (opening the mouth). In the
/// feeding mode, the feeder's check (opening the mouth on a hit).
pub fn vfunction87__004edb30(g: &mut G, this: Ptr) {
    let (cx, cy) = center(g, this);
    if g.go(this).offset_0x68 != 0 {
        if 0 < crate::game::game_object::FUN_004ea2a0(g, this, cx, cy) {
            FUN_004d6cc0(g, this);
        }
        return;
    }
    let board = board_of(g, this);
    let objs: Vec<Ptr> = g.board(board).offset_0x7c.iter().copied().collect();
    for o in objs {
        if edible(g, o) && g.go(o).offset_0x4 == 0 {
            let (ox, oy) = center(g, o);
            let (dx, dy) = (cx - ox, cy - oy);
            if -0x1e < dx && dx < 0x1e && -0x1e < dy && dy < 0x1e {
                FUN_004d6cc0(g, this);
                vcall!(g, this, go.vfunction78, o as i32);
                vcall!(g, o, go.vfunction76);
                return;
            }
        }
    }
}
