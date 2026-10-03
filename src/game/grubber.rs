//! `Sexy::Grubber`: the guppycruncher, crawling on the floor, jumping at small guppies
//! and spitting beetles. `Grubber_data` starts at object offset 0x154 (field names
//! relative to it); the object is 0x1b0 bytes.

use crate::game::board_level::vec_index;
use crate::game::game_object::{FUN_004d6cc0, FUN_004d6db0, FUN_004d6f30, FUN_004d7040, GameObject};
use crate::sexy::crt::ftol;
use crate::sexy::graphics::{FUN_004558e0, FUN_00455e40, FUN_004560a0, FUN_004563d0, FUN_00456950};
use crate::sexy::prelude::*;
use crate::sexy::types::CRect;

/// `Grubber_data` (object offset 0x154).
#[derive(Debug, Clone, Default)]
pub struct Grubber_data {
    /// +0x158 x.
    pub offset_0x4: f64,
    /// +0x160 y.
    pub offset_0xc: f64,
    /// +0x168 x speed.
    pub offset_0x14: f64,
    /// +0x170 y speed (sinking by 0.4, -14 on a jump).
    pub offset_0x1c: f64,
    /// +0x178 speed divisor (2.5..2.7).
    pub offset_0x24: f64,
    /// +0x180 bubbles left to blow (45 at first).
    pub offset_0x2c: i32,
    /// +0x188 the x speed the current mode wants.
    pub offset_0x34: f64,
    /// +0x190 crawl mode (0..7).
    pub offset_0x3c: i32,
    /// +0x194 updates since the last steering.
    pub offset_0x40: i32,
    /// +0x198 updates since the last mode roll.
    pub offset_0x44: i32,
    /// +0x19c animation counter.
    pub offset_0x48: i32,
    /// +0x1a0 cel.
    pub offset_0x4c: i32,
    /// +0x1a4 bite animation counter.
    pub offset_0x50: i32,
    /// +0x1a8 updates toward the next beetle.
    pub offset_0x54: i32,
    /// +0x1ac updates between beetles.
    pub offset_0x58: i32,
}

impl G {
    pub fn grubber(&mut self, p: Ptr) -> &mut Grubber_data {
        match &mut self.go_ext(p).sub {
            GoSub::Grubber(d) => d,
            s => panic!("{p} is not a Grubber: {s:?}"),
        }
    }
}

fn alloc_grubber(g: &mut G, wc: crate::sexy::object::WidgetContainer_data, w: crate::sexy::object::Widget_data,
                 go: crate::game::game_object::GameObject_data, d: Grubber_data) -> Ptr {
    g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__Grubber_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::GameObject(Box::new(GameObjectExt { go, sub: GoSub::Grubber(d) })) }),
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

/// port: 004ea700 Sexy::Grubber::Grubber
/// `Grubber::Grubber()` (used before loading from a save).
pub fn Grubber__004ea700(g: &mut G) -> Ptr {
    let (mut wc, w, mut go) = GameObject(g);
    go.offset_0x4 = 9;
    wc.offset_0x3d = false;
    alloc_grubber(g, wc, w, go, Grubber_data::default())
}

/// port: 004ea740 Sexy::Grubber::Grubber
/// `Grubber(int x)`: on the floor at x.
pub fn Grubber__004ea740(g: &mut G, param_1: i32) -> Ptr {
    let (wc, w, go) = GameObject(g);
    let this = alloc_grubber(g, wc, w, go, Grubber_data::default());
    FUN_004d7970(g, this);
    g.grubber(this).offset_0x4 = param_1 as f64;
    let x = g.grubber(this).offset_0x4;
    g.wc(this).offset_0x2c = ftol(x) as i32;
    this
}

/// port: 004ea7b0 Sexy::Grubber::Grubber
/// `Grubber(int x, int y)`.
pub fn Grubber__004ea7b0(g: &mut G, param_1: i32, param_2: i32) -> Ptr {
    let (wc, w, go) = GameObject(g);
    let this = alloc_grubber(g, wc, w, go, Grubber_data::default());
    FUN_004d7970(g, this);
    g.grubber(this).offset_0x4 = param_1 as f64;
    g.wc(this).offset_0x2c = param_1;
    g.wc(this).offset_0x30 = param_2;
    g.grubber(this).offset_0xc = param_2 as f64;
    this
}

/// port: 004d7970 FUN_004d7970
/// `Grubber::Init()`: type 9, 80x80 on the floor (y 360..364), still, a random speed
/// divisor, hunger 900..1099, a random crawl mode, the beetle interval (300..499, or the
/// virtual tank's), 45 bubbles; mouse-visible as pets are.
pub fn FUN_004d7970(g: &mut G, this: Ptr) {
    g.go(this).offset_0x4 = 9;
    g.wc(this).offset_0x3d = false;
    let r = rand(g, this);
    g.grubber(this).offset_0xc = (r % 5 + 0x168) as f64;
    let y = g.grubber(this).offset_0xc;
    {
        let d = g.grubber(this);
        d.offset_0x14 = 0.0;
        d.offset_0x1c = 0.0;
        d.offset_0x34 = 0.0;
    }
    let wc = g.wc(this);
    wc.offset_0x30 = ftol(y) as i32;
    wc.offset_0x34 = 0x50;
    wc.offset_0x38 = 0x50;
    let r = rand(g, this);
    g.grubber(this).offset_0x24 = match r % 3 {
        0 => f64::from_bits(0x400599999999999a),
        1 => 2.5,
        _ => f64::from_bits(0x4004cccccccccccd),
    };
    let r = rand(g, this);
    g.go(this).offset_0x14 = (r % 200) as i32 + 900;
    let r = rand(g, this);
    {
        let d = g.grubber(this);
        d.offset_0x40 = 0x28;
        d.offset_0x44 = 0;
        d.offset_0x48 = 0;
        d.offset_0x4c = 0;
        d.offset_0x50 = 0;
        d.offset_0x54 = 0;
        d.offset_0x3c = (r % 10) as i32;
    }
    let r = rand(g, this) as i32;
    let iv = crate::game::fish::FUN_004d71d0(g, this, r % 200 + 300);
    g.grubber(this).offset_0x58 = iv;
    g.w(this).offset_0x1 = g.globals.DAT_005e8f14;
    g.grubber(this).offset_0x2c = 0x2d;
}

/// port: 004ea830 Sexy::Grubber::~Grubber
pub fn dtor_Grubber(g: &mut G, this: Ptr) {
    g.set_vt(this, &crate::sexy::vtables_gen::Sexy__Grubber_vftable);
    crate::game::game_object::dtor_GameObject(g, this);
}

/// port: 004efaa0 Sexy::Grubber::deleting_destructor
pub fn deleting_destructor(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_Grubber(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 004d7880 Sexy::Grubber::vfunction81
/// `Sync(DataSync&)`.
pub fn vfunction81(g: &mut G, this: Ptr, sync: &mut crate::sexy::data_sync::DataSync) -> crate::sexy::data_sync::SyncResult {
    crate::game::game_object::vfunction81(g, this, sync)?;
    use crate::sexy::data_sync::{FUN_00503010, FUN_005030a0};
    let mut d = g.grubber(this).clone();
    FUN_005030a0(sync, &mut d.offset_0x4)?;
    FUN_005030a0(sync, &mut d.offset_0xc)?;
    FUN_005030a0(sync, &mut d.offset_0x14)?;
    FUN_005030a0(sync, &mut d.offset_0x1c)?;
    FUN_005030a0(sync, &mut d.offset_0x24)?;
    FUN_00503010(sync, &mut d.offset_0x2c)?;
    FUN_005030a0(sync, &mut d.offset_0x34)?;
    FUN_00503010(sync, &mut d.offset_0x3c)?;
    FUN_00503010(sync, &mut d.offset_0x40)?;
    FUN_00503010(sync, &mut d.offset_0x44)?;
    FUN_00503010(sync, &mut d.offset_0x48)?;
    FUN_00503010(sync, &mut d.offset_0x4c)?;
    FUN_00503010(sync, &mut d.offset_0x50)?;
    FUN_00503010(sync, &mut d.offset_0x54)?;
    FUN_00503010(sync, &mut d.offset_0x58)?;
    *g.grubber(this) = d;
    Ok(())
}

/// port: 004ea720 Sexy::Grubber::vfunction72
/// The shadow kind (7).
pub fn vfunction72(_g: &mut G, _this: Ptr) -> i32 {
    7
}

/// port: 004f3ef0 Sexy::Grubber::vfunction76
/// `Remove()`: leaves the tank with its shadow.
pub fn vfunction76(g: &mut G, this: Ptr) {
    FUN_004f2740(g, this, true);
}

/// port: 004f2740 FUN_004f2740
/// `Remove(bool withShadow)`: drops what it carries, leaves the widget manager and the
/// board, optionally its shadow; counts a lost guppycruncher (+0x47c); on level 3-1 of the
/// adventure a warning (with hints the first times).
pub fn FUN_004f2740(g: &mut G, this: Ptr, param_1: bool) {
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
    g.board(board).ext_0x460[(0x47c - 0x460) / 4] += 1;
    if crate::game::board_update::FUN_00537bb0(g, board, 3, 1) {
        if g.board(board).ext_0x4b0[0x26] != 0 {
            FUN_0053e950(g, board, b"Try luring small guppies down to your guppycrunchers!", false, 0x27);
        }
        if g.board(board).ext_0x4b0[0x25] != 0 {
            FUN_0053e950(g, board, b"Guppycrunchers and carnivores share the same diet!", false, 0x26);
        }
        FUN_0053e950(g, board, b"Warning! Your guppycruncher has died!", false, 0x25);
    }
}

/// port: 004f2890 FUN_004f2890
/// `Die(bool sound)`: the death sound, removal (keeping the shadow) and a dead body where
/// it was, facing the way it crawled.
pub fn FUN_004f2890(g: &mut G, this: Ptr, param_1: bool) {
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    if param_1 {
        let t = g.go(this).offset_0x4;
        crate::game::board_level::FUN_00538610(g, board, t);
    }
    FUN_004f2740(g, this, false);
    let d = g.grubber(this).clone();
    let right = 0.0 < d.offset_0x14;
    let shadow = g.go(this).offset_0xc;
    let kind = g.go(this).offset_0x4;
    let y = ftol(d.offset_0xc) as i32;
    let x = ftol(d.offset_0x4) as i32;
    let board = g.wfa(app).offset_0x4;
    crate::game::board_level::FUN_005441f0(g, board, x, y, d.offset_0x14, 0.0, d.offset_0x24, kind, right, shadow);
}

/// port: 004d7ca0 Sexy::Grubber::vfunction78
/// `Eat(Fish*)`: counted in the virtual tank, the bite sound, fed (+1000, at most 1400);
/// colored sparkles in the +0x882 mode (not for food).
pub fn vfunction78(g: &mut G, this: Ptr, param_1: i32) {
    let prey = param_1 as Ptr;
    let _ = FUN_004d6f30(g, this);
    crate::game::game_object::FUN_004d6a30(g, this, false);
    g.go(this).offset_0x70 = 100;
    if -1 < g.go(this).offset_0x24 {
        crate::game::game_object::FUN_004d6f90(g, this);
    }
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    let high = g.go(this).offset_0x7c;
    crate::game::board_level::FUN_00538560(g, board, high);
    g.go(this).offset_0x14 += 1000;
    if 0x578 < g.go(this).offset_0x14 {
        g.go(this).offset_0x14 = 0x578;
    }
    if g.wfa(app).offset_0x156 && g.go(prey).offset_0x4 != 0x1c {
        let mut n = rand(g, this) % 3 + 2;
        while n != 0 {
            let r = rand(g, this);
            let y = (r % 0x14) as i32 + 0xf + g.wc(prey).offset_0x30;
            let r = rand(g, this);
            let x = (r % 0x14) as i32 + 0xf + g.wc(prey).offset_0x2c;
            crate::game::board_level::FUN_005436f0(g, board, x, y, 1);
            n -= 1;
        }
    }
}

/// port: 004ea840 FUN_004ea840
/// `FindPrey()`: the nearest small guppy of the level not just bought; within 100 pixels
/// the mouth opens (150 updates, +0xf8 = 100). In the virtual tank's feeding mode, the
/// feeder's target instead.
pub fn FUN_004ea840(g: &mut G, this: Ptr) -> Ptr {
    if g.go(this).offset_0x68 != 0 {
        let wc = g.wc(this).clone();
        return crate::game::game_object::FUN_004ea010(g, this, wc.offset_0x34 / 2 + wc.offset_0x2c, wc.offset_0x38 / 2 + wc.offset_0x30);
    }
    let board = board_of(g, this);
    let mut best = 100000000;
    let mut prey = NULL;
    for o in g.board(board).offset_0xc[vec_index(0xa0)].clone() {
        if g.fish(o).offset_0x4c == 0 && g.go(o).offset_0x24 < 0 && g.go(o).offset_0x98 < 1 {
            let d = g.grubber(this).clone();
            let (ox, oy) = (g.wc(o).offset_0x2c, g.wc(o).offset_0x30);
            let dx = ftol((ox + 0x28) as f64 - (d.offset_0x4 + 40.0)) as i32;
            let dy = ftol((oy + 0x28) as f64 - (d.offset_0xc + 40.0)) as i32;
            let n = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx));
            if n < best {
                prey = o;
                best = n;
            }
        }
    }
    if best < 10000 {
        crate::game::game_object::FUN_004d6b00(g, this, 0x96);
        g.go(this).offset_0x70 = 100;
    }
    prey
}

/// port: 004ea9f0 FUN_004ea9f0
/// `TryEat()`: a small guppy in its jaws is eaten (vfunction 78, then the guppy goes);
/// one above it within reach starts the bite (20 updates). In the virtual tank's feeding
/// mode, the feeder decides.
pub fn FUN_004ea9f0(g: &mut G, this: Ptr) {
    if g.go(this).offset_0x68 == 0 {
        let board = board_of(g, this);
        for o in g.board(board).offset_0xc[vec_index(0xa0)].clone() {
            if g.go(o).offset_0x24 < 0 && g.go(o).offset_0x98 < 1 {
                let d = g.grubber(this).clone();
                let (x, y) = (d.offset_0x4 + 40.0, d.offset_0xc + 40.0);
                let (ox, oy) = (g.wc(o).offset_0x2c, g.wc(o).offset_0x30);
                if x < (ox + 0x50) as f64 && (ox as f64) < x && y < (oy + 0x46) as f64 && ((oy + 10) as f64) < y && g.fish(o).offset_0x4c == 0 {
                    vcall!(g, this, go.vfunction78, o as i32);
                    vcall!(g, o, fish.vfunction88, true);
                    if g.grubber(this).offset_0x50 != 0 {
                        return;
                    }
                    FUN_004d6cc0(g, this);
                    g.grubber(this).offset_0x50 = 0xc;
                    return;
                }
                if g.grubber(this).offset_0x50 == 0
                    && x < (ox + 0x50) as f64
                    && (ox as f64) < x
                    && y < (oy + 0xa0) as f64
                    && ((oy - 0x14) as f64) < y
                    && g.fish(o).offset_0x4c == 0
                {
                    g.grubber(this).offset_0x50 = 0x14;
                    FUN_004d6cc0(g, this);
                    return;
                }
            }
        }
        return;
    }
    let wc = g.wc(this).clone();
    let r = crate::game::game_object::FUN_004ea2a0(g, this, wc.offset_0x34 / 2 + wc.offset_0x2c, wc.offset_0x38 / 2 + wc.offset_0x30);
    if g.grubber(this).offset_0x50 == 0 {
        if r == 1 {
            FUN_004d6cc0(g, this);
            g.grubber(this).offset_0x50 = 4;
            return;
        }
        if r == 2 {
            FUN_004d6cc0(g, this);
            g.grubber(this).offset_0x50 = 8;
        }
    }
}

/// port: 004efad0 FUN_004efad0
/// `ChasePrey()`: every 5 updates crawls under the nearest small guppy (faster when
/// hungry); on the floor right under it, it jumps (y speed -14); then tries to eat. True
/// when there is prey.
pub fn FUN_004efad0(g: &mut G, this: Ptr) -> bool {
    let t = FUN_004ea840(g, this);
    if 5 <= g.grubber(this).offset_0x40 {
        if t == NULL {
            return false;
        }
        g.grubber(this).offset_0x40 = 0;
        let hungry = g.go(this).offset_0x14 < 0x12d;
        let tx = g.wc(t).offset_0x2c;
        let d = g.grubber(this);
        let x = d.offset_0x4 + 40.0;
        let v = &mut d.offset_0x14;
        let (far, near) = ((tx + 0x30) as f64, (tx + 0x18) as f64);
        let c = (tx + 0x24) as f64;
        if hungry {
            if x <= far {
                if near <= x {
                    if c < x {
                        if -2.5 < *v {
                            *v -= 0.8;
                        }
                    } else if x < c && *v < 2.5 {
                        *v += 0.8;
                    }
                } else if *v < 4.0 {
                    *v += 1.5;
                }
            } else if -4.0 < *v {
                *v -= 1.5;
            }
        } else if x <= far {
            if near <= x {
                if x <= c {
                    if x < c && *v < 1.5 {
                        *v += 0.6;
                    }
                } else if -1.5 < *v {
                    *v -= 0.6;
                }
            } else if *v < 2.5 {
                *v += 0.8;
            }
        } else if -2.5 < *v {
            *v -= 0.8;
        }
    }
    if t != NULL {
        let wc = g.wc(t).clone();
        let d = g.grubber(this).clone();
        let y = d.offset_0xc + 40.0;
        let reach = if 0x50 < wc.offset_0x38 { 0xf0 } else { 0xa0 } + wc.offset_0x30;
        if y < reach as f64 && ((wc.offset_0x30 - 0x14) as f64) < y {
            let x = d.offset_0x4 + 40.0;
            if x < (wc.offset_0x2c + 0x50) as f64 && (wc.offset_0x2c as f64) < x && 355.0 <= d.offset_0xc {
                g.go(this).offset_0x70 = 100;
                FUN_004d6cc0(g, this);
                g.grubber(this).offset_0x1c = -14.0;
                FUN_004ea9f0(g, this);
                return true;
            }
        }
        FUN_004ea9f0(g, this);
        return true;
    }
    false
}

/// port: 004f8f60 FUN_004f8f60
/// `Hunt()`: hunger ticks (the hungry look only with no aliens around); starving dies;
/// below 900 it chases guppies.
pub fn FUN_004f8f60(g: &mut G, this: Ptr) -> bool {
    crate::game::game_object::FUN_004d6c50(g, this);
    let board = board_of(g, this);
    if g.board(board).offset_0xc[vec_index(0xb8)].is_empty() && g.board(board).offset_0xc[vec_index(0xf4)].is_empty() {
        crate::game::game_object::FUN_004d6ef0(g, this);
    }
    if !crate::game::game_object::FUN_004d69d0(g, this) {
        if g.go(this).offset_0x14 < 900 {
            return FUN_004efad0(g, this);
        }
    } else {
        FUN_004f2890(g, this, true);
    }
    false
}

/// port: 004d7ae0 FUN_004d7ae0
/// `BeetleTimer()`: a beetle (`Larva`) above it when the timer is up and it is not hungry.
pub fn FUN_004d7ae0(g: &mut G, this: Ptr) {
    g.grubber(this).offset_0x54 += 1;
    let mut t = g.grubber(this).offset_0x54;
    let period = g.grubber(this).offset_0x58;
    let due = crate::game::game_object::FUN_004d7100(g, this, &mut t, period);
    g.grubber(this).offset_0x54 = t;
    if due {
        g.grubber(this).offset_0x54 = 0;
        if crate::game::game_object::FUN_004d6bd0(g, this) {
            let board = board_of(g, this);
            let (mx, my) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
            crate::game::board_level::FUN_005445a0(g, board, mx + 4, my - 10);
        }
    }
}

/// port: 004d7b40 FUN_004d7b40
/// `Animate()`: the crawl cycle by speed (or the bite cels), the open mouth, carrying.
pub fn FUN_004d7b40(g: &mut G, this: Ptr) {
    crate::game::game_object::FUN_004d6c90(g, this);
    let d = g.grubber(this);
    if d.offset_0x50 < 1 {
        let v = d.offset_0x14;
        let cel = if v < 1.0 {
            if -1.0 < v {
                let c = if v <= 0.0 {
                    let mut c = (d.offset_0x48 - 1) % 0x28;
                    if c < 0 {
                        c += 0x28;
                    }
                    c
                } else {
                    (d.offset_0x48 + 1) % 0x28
                };
                d.offset_0x48 = c;
                c / 4
            } else {
                let mut c = (d.offset_0x48 - 1) % 0x14;
                if c < 0 {
                    c += 0x14;
                }
                d.offset_0x48 = c;
                c / 2
            }
        } else {
            let c = (d.offset_0x48 + 1) % 0x14;
            d.offset_0x48 = c;
            c / 2
        };
        d.offset_0x4c = cel;
    } else {
        d.offset_0x50 -= 1;
        d.offset_0x4c = 9 - d.offset_0x50 / 2;
    }
    if 100 < g.go(this).offset_0x80 {
        g.grubber(this).offset_0x4c = 4;
    }
    if g.go(this).offset_0x10 != NULL {
        crate::game::game_object::FUN_004d7020(g, this);
    }
}

/// port: 004d7c60 FUN_004d7c60
/// The sprite row: 0 / 2 (hungry) crawling, 1 / 4 (hungry) biting or with the mouth open.
pub fn FUN_004d7c60(g: &mut G, this: Ptr, param_1: bool) -> i32 {
    if g.grubber(this).offset_0x50 < 1 && g.go(this).offset_0x80 < 0x65 {
        return if param_1 { 2 } else { 0 };
    }
    if param_1 { 4 } else { 1 }
}

/// port: 004f8fe0 Sexy::Grubber::vfunction23
/// `Update()` (not while paused): unless hunting, eases toward its mode's speed; the mode
/// re-roll (1 in 10 every 20 updates or at the walls, not mid-bite); the beetle timer
/// (not with aliens around); kept in the tank; blowing its first bubbles; slowing at the
/// walls; sinking; animation; moving (fast while dropping in).
pub fn vfunction23(g: &mut G, this: Ptr) {
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    if board == NULL || g.board(board).field_0x8 {
        return;
    }
    crate::game::game_object::FUN_004f22c0(g, this);
    if !FUN_004f8f60(g, this) {
        let d = g.grubber(this);
        match d.offset_0x3c {
            0 => d.offset_0x34 = -0.5,
            1 => d.offset_0x34 = 0.5,
            2 => d.offset_0x34 = -1.0,
            3 => d.offset_0x34 = 1.0,
            4 | 5 => d.offset_0x34 = 0.0,
            6 => d.offset_0x34 = -2.5,
            7 => d.offset_0x34 = 2.5,
            _ => {}
        }
        if d.offset_0x34 < d.offset_0x14 {
            d.offset_0x14 -= 0.1;
        }
        if d.offset_0x14 < d.offset_0x34 {
            d.offset_0x14 += 0.1;
        }
    }
    let reroll = {
        let d = g.grubber(this);
        d.offset_0x40 += 1;
        d.offset_0x44 += 1;
        (0x14 < d.offset_0x44 || d.offset_0x4 <= 10.0 || 540.0 <= d.offset_0x4) && d.offset_0x50 == 0
    };
    if reroll {
        g.grubber(this).offset_0x44 = 0;
        if rand(g, this) % 10 == 0 {
            let m = (rand(g, this) & 7) as i32;
            g.grubber(this).offset_0x3c = m;
        }
    }
    if !crate::game::board_update::FUN_004da780(g, board) {
        FUN_004d7ae0(g, this);
    }
    {
        let d = g.grubber(this);
        if 560.0 < d.offset_0x4 {
            d.offset_0x4 = 560.0;
        }
        if d.offset_0x4 < 10.0 {
            d.offset_0x4 = 10.0;
        }
        if 355.0 < d.offset_0xc {
            d.offset_0xc = 355.0;
            d.offset_0x1c = 0.0;
        }
    }
    let bubbles = g.grubber(this).offset_0x2c;
    if bubbles < 1 {
        let d = g.grubber(this);
        if d.offset_0xc < 95.0 {
            d.offset_0xc = 95.0;
        }
    } else {
        let k = if 0x23 < bubbles { 8 } else { 5 };
        if rand(g, this) % k == 0 {
            let r1 = rand(g, this);
            let r2 = rand(g, this);
            let (mx, my) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
            crate::game::board_update::FUN_00538ac0(g, board, mx + (0x37 - (r1 % 0x3c) as i32), my + (0x37 - (r2 % 0x3c) as i32));
        }
        g.grubber(this).offset_0x2c -= 1;
    }
    {
        let d = g.grubber(this);
        if 535.0 < d.offset_0x4 && 0.1 < d.offset_0x14 {
            d.offset_0x14 -= 0.1;
        }
        if d.offset_0x4 < 15.0 && d.offset_0x14 < -0.1 {
            d.offset_0x14 += 0.1;
        }
        if d.offset_0xc < 355.0 {
            d.offset_0x1c += 0.4;
        }
    }
    FUN_004d7b40(g, this);
    let mut div = g.grubber(this).offset_0x24;
    if g.go(this).offset_0x6e {
        div = if g.go(this).offset_0x70 == 0 { 0.3 } else { 1.5 };
    }
    let d = g.grubber(this);
    d.offset_0x4 = d.offset_0x14 / div + d.offset_0x4;
    d.offset_0xc = d.offset_0x1c / div + d.offset_0xc;
    let (x, y) = (ftol(d.offset_0x4) as i32, ftol(d.offset_0xc) as i32);
    vcall!(g, this, w.vfunction42, x, y);
}

/// port: 004e19f0 FUN_004e19f0
/// `DrawBody(Graphics*)`: the cel in its row, the hungry tint fading over it, what it
/// carries, the sparkle; the plain sheet's cel in the `DAT_005e89ce` mode.
pub fn FUN_004e19f0(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    let img = g.res.DAT_005e8c98;
    if !g.globals.DAT_005e89ce {
        let cel = g.grubber(this).offset_0x4c * 0x50;
        let hungry = FUN_004d6f30(g, this);
        let row = FUN_004d7c60(g, this, hungry) * 0x50;
        let src = Rect::new(cel, row, 0x50, 0x50);
        if g.go(this).offset_0x74 && crate::game::game_object::FUN_004d6e70(g, this, gfx, img, &src, false) {
            return;
        }
        FUN_004d6db0(g, this, gfx, Color::WHITE);
        FUN_00455e40(gfx, g, img, 0, 0, &src);
        let t = g.go(this).offset_0x18;
        if t != 0 {
            let _ = FUN_004d7c60(g, this, true);
            FUN_004d6db0(g, this, gfx, CRect(0xff, 0xff, 0xff, (t * 0xff) / 5));
            let cel = g.grubber(this).offset_0x4c * 0x50;
            FUN_00455e40(gfx, g, img, 0, 0, &Rect::new(cel, 0xa0, 0x50, 0x50));
        }
        FUN_004558e0(gfx, false);
        if g.go(this).offset_0x10 != NULL {
            FUN_004d7040(g, this, gfx, 0, 0);
        }
        if crate::game::game_object::FUN_004d6980(g, this, 900) {
            let s = g.res.DAT_005e8bc4;
            FUN_00456950(gfx, g, s, 0, 0, 2);
        }
    } else {
        let col = if FUN_004d6f30(g, this) { 6 } else { 9 };
        FUN_004560a0(gfx, g, img, 0, 0, &Rect::new(col * 0x50, 0xf0, 0x50, 0x50), false);
        if g.go(this).offset_0x10 != NULL {
            FUN_004d7040(g, this, gfx, 0, 0);
        }
        if crate::game::game_object::FUN_004d6980(g, this, 900) {
            let s = g.res.DAT_005e8bc4;
            FUN_00456950(gfx, g, s, 0, 0, 2);
        }
    }
}

/// port: 004e1bf0 Sexy::Grubber::vfunction27
/// `Draw(Graphics*)`: the body, and its name label when it has one.
pub fn vfunction27(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    crate::game::food::FUN_004d6ae0(g, this);
    FUN_004e19f0(g, this, gfx);
    // +0xc4: the name's length.
    if !g.go(this).field_0x28.is_empty() {
        crate::game::game_object::FUN_004e1400(g, this, gfx, false);
    }
}

/// port: 004e1c20 Sexy::Grubber::vfunction80
/// `DrawIcon(Graphics*, int pose)`: its portrait cel offset by pose.
pub fn vfunction80(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: i32) {
    let (dx, dy) = match param_2 {
        0 => (0x13, 0x11),
        1 => (5, 0xd),
        2 => (0xf, -5),
        3 => (5, 0),
        4 => (-0x28, 0),
        _ => (0, 0),
    };
    FUN_004563d0(gfx, dx, dy);
    crate::game::game_object::FUN_004d6d10(g, this, gfx, Color::WHITE);
    let img = g.res.DAT_005e8c98;
    let col = g.go(this).offset_0xa8 * 0x50;
    FUN_00455e40(gfx, g, img, 0, 0, &Rect::new(col, 0, 0x50, 0x50));
    FUN_004558e0(gfx, false);
    FUN_004563d0(gfx, -dx, -dy);
}
