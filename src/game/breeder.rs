//! `Sexy::Breeder`: the breeder, a fish-like creature that grows in three stages and,
//! once grown, gives birth to guppies. `Breeder_data` starts at object offset 0x154 (field
//! names relative to it); the object is 0x1e0 bytes.

use crate::game::board_level::vec_index;
use crate::game::game_object::{FUN_004d6cc0, FUN_004d6db0, FUN_004d6f30, FUN_004d7040, GameObject};
use crate::sexy::crt::ftol;
use crate::sexy::graphics::{FUN_004558e0, FUN_00455900, FUN_00455e40, FUN_004560a0, FUN_004563d0, FUN_00456950};
use crate::sexy::prelude::*;
use crate::sexy::types::CRect;

/// `Breeder_data` (object offset 0x154).
#[derive(Debug, Clone, Default)]
pub struct Breeder_data {
    /// +0x158 x.
    pub field_0x4: f64,
    /// +0x160 y.
    pub field_0xc: f64,
    /// +0x168 x speed.
    pub offset_0x14: f64,
    /// +0x170 y speed.
    pub field_0x1c: f64,
    /// +0x178 sweep direction (1 / -1) of the swim modes 5+.
    pub field_0x24: i32,
    /// +0x17c.
    pub field_0x28: i32,
    /// +0x180 speed divisor (2.0, 1.8 or 1.6).
    pub field_0x2c: f64,
    /// +0x188 the x speed the facing follows.
    pub offset_0x34: f64,
    /// +0x190 lowest y (370).
    pub field_0x3c: i32,
    /// +0x194 leftmost x (10).
    pub field_0x40: i32,
    /// +0x198 highest y (95).
    pub field_0x44: i32,
    /// +0x19c rightmost x (540).
    pub field_0x48: i32,
    /// +0x1a0 growth stage (0..2).
    pub offset_0x4c: i32,
    /// +0x1a4 food eaten toward the next stage.
    pub field_0x50: i32,
    /// +0x1a8 food needed (4..5).
    pub field_0x54: i32,
    /// +0x1ac grow flash counter.
    pub field_0x58: i32,
    /// +0x1b0 swim mode (0..9).
    pub field_0x5c: i32,
    /// +0x1b4 updates in the swim mode (also the chase step counter).
    pub field_0x60: i32,
    /// +0x1b8 updates since the last mode roll.
    pub field_0x64: i32,
    /// +0x1bc swim animation counter.
    pub field_0x68: i32,
    /// +0x1c0 speed level (0..5).
    pub field_0x6c: i32,
    /// +0x1c4 cel.
    pub field_0x70: i32,
    /// +0x1c8 turn counter (+20 / -20 counting to 0).
    pub field_0x74: i32,
    /// +0x1cc eating animation counter.
    pub field_0x78: i32,
    /// +0x1d0 updates toward the next birth.
    pub field_0x7c: i32,
    /// +0x1d4 updates between births.
    pub offset_0x80: i32,
    /// +0x1d8 drop-in updates left (slowing the fall, blowing bubbles).
    pub offset_0x84: i32,
    /// +0x1dc births given (virtual tank).
    pub offset_0x88: i32,
}

impl G {
    pub fn breeder(&mut self, p: Ptr) -> &mut Breeder_data {
        match &mut self.go_ext(p).sub {
            GoSub::Breeder(d) => d,
            s => panic!("{p} is not a Breeder: {s:?}"),
        }
    }
}

fn alloc_breeder(g: &mut G, wc: crate::sexy::object::WidgetContainer_data, w: crate::sexy::object::Widget_data,
                 go: crate::game::game_object::GameObject_data, d: Breeder_data) -> Ptr {
    g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__Breeder_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::GameObject(Box::new(GameObjectExt { go, sub: GoSub::Breeder(d) })) }),
    })
}

fn board_of(g: &mut G, this: Ptr) -> Ptr {
    let app = g.go(this).offset_0x0;
    g.wfa(app).offset_0x4
}

fn mode(g: &mut G, this: Ptr) -> i32 {
    let app = g.go(this).offset_0x0;
    g.wfa(app).offset_0x150
}

fn rand(g: &mut G, this: Ptr) -> u32 {
    let app = g.go(this).offset_0x0;
    let r = g.wfa(app).offset_0x84;
    crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(r))
}

/// port: 004ee500 Sexy::Breeder::Breeder
/// `Breeder::Breeder()` (used before loading from a save).
pub fn Breeder__004ee500(g: &mut G) -> Ptr {
    let (mut wc, w, mut go) = GameObject(g);
    wc.offset_0x3d = false;
    go.offset_0x4 = 10;
    alloc_breeder(g, wc, w, go, Breeder_data::default())
}

/// port: 004ee530 Sexy::Breeder::Breeder
/// `Breeder(int x, int y)`: a newborn (stage 0), giving birth every 1000..1399 updates
/// once grown.
pub fn Breeder__004ee530(g: &mut G, param_1: i32, param_2: i32) -> Ptr {
    let (wc, w, go) = GameObject(g);
    let this = alloc_breeder(g, wc, w, go, Breeder_data::default());
    FUN_004dcbf0(g, this, param_1, param_2);
    g.breeder(this).offset_0x4c = 0;
    let r = rand(g, this);
    g.breeder(this).offset_0x80 = (r % 400) as i32 + 1000;
    this
}

/// port: 004ee5c0 Sexy::Breeder::Breeder
/// `Breeder(int x, int y, int stage, bool facingRight)` (a revived one).
pub fn Breeder__004ee5c0(g: &mut G, param_1: i32, param_2: i32, param_3: i32, param_4: bool) -> Ptr {
    let (wc, w, go) = GameObject(g);
    let this = alloc_breeder(g, wc, w, go, Breeder_data::default());
    FUN_004dcbf0(g, this, param_1, param_2);
    let v = if !param_4 { -1.0 } else { 1.0 };
    {
        let d = g.breeder(this);
        d.offset_0x14 = v;
        d.offset_0x34 = v;
        d.offset_0x4c = param_3;
    }
    let iv = if param_3 == 0 || param_3 == 1 {
        (rand(g, this) % 400) as i32 + 1000
    } else {
        (rand(g, this) % 200) as i32 + 800
    };
    g.breeder(this).offset_0x80 = iv;
    this
}

/// port: 004dcbf0 FUN_004dcbf0
/// `Breeder::Init(int x, int y)`: type 10, 80x80 at (x, y), sinking, a random facing and
/// speed divisor, hunger 400..599, the food needed to grow (4..5, or 12..20 in the virtual
/// tank), a random swim mode; not mouse-visible while aliens are around.
pub fn FUN_004dcbf0(g: &mut G, this: Ptr, param_1: i32, param_2: i32) {
    {
        let d = g.breeder(this);
        d.field_0x4 = param_1 as f64;
        d.field_0xc = param_2 as f64;
        d.offset_0x88 = 0;
    }
    g.wc(this).offset_0x3d = false;
    g.go(this).offset_0x4 = 10;
    let (x, y) = (g.breeder(this).field_0x4, g.breeder(this).field_0xc);
    g.wc(this).offset_0x2c = ftol(x) as i32;
    g.wc(this).offset_0x30 = ftol(y) as i32;
    {
        let d = g.breeder(this);
        d.offset_0x14 = 0.0;
        d.field_0x1c = -0.5;
        d.offset_0x34 = 1.0;
    }
    g.wc(this).offset_0x34 = 0x50;
    g.wc(this).offset_0x38 = 0x50;
    if rand(g, this) & 1 == 0 {
        let d = g.breeder(this);
        d.offset_0x14 = -0.1;
        d.offset_0x34 = -1.0;
    }
    {
        let d = g.breeder(this);
        d.field_0x24 = 1;
        d.field_0x28 = 0;
        d.field_0x3c = 0x172;
        d.field_0x44 = 0x5f;
        d.field_0x40 = 10;
        d.field_0x48 = 0x21c;
    }
    let r = rand(g, this);
    g.breeder(this).field_0x2c = match r % 3 {
        0 => 2.0,
        1 => f64::from_bits(0x3ffccccccccccccd),
        _ => f64::from_bits(0x3ff999999999999a),
    };
    let r = rand(g, this);
    g.breeder(this).field_0x50 = 0;
    g.go(this).offset_0x14 = (r % 200) as i32 + 400;
    let need = if mode(g, this) == 5 {
        crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32 % 9 + 0xc
    } else {
        (rand(g, this) & 1) as i32 + 4
    };
    g.breeder(this).field_0x54 = need;
    let r = rand(g, this);
    {
        let d = g.breeder(this);
        d.field_0x60 = 0x28;
        d.field_0x64 = 0;
        d.field_0x68 = 0;
        d.field_0x6c = 0;
        d.field_0x70 = 0;
        d.field_0x74 = 0;
        d.field_0x78 = 0;
        d.field_0x7c = 0;
    }
    g.w(this).offset_0x1 = true;
    g.breeder(this).field_0x5c = (r % 10) as i32;
    let board = board_of(g, this);
    if board != NULL {
        let b = g.board(board);
        if !b.offset_0xc[vec_index(0xb8)].is_empty() || !b.offset_0xc[vec_index(0xf4)].is_empty() {
            g.w(this).offset_0x1 = false;
        }
    }
    let d = g.breeder(this);
    d.field_0x58 = 0;
    d.offset_0x84 = 0;
}

/// port: 004ee6a0 Sexy::Breeder::~Breeder
pub fn dtor_Breeder(g: &mut G, this: Ptr) {
    g.set_vt(this, &crate::sexy::vtables_gen::Sexy__Breeder_vftable);
    crate::game::game_object::dtor_GameObject(g, this);
}

/// port: 004f0850 Sexy::Breeder::deleting_destructor
pub fn deleting_destructor(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_Breeder(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 004dce10 Sexy::Breeder::vfunction81
/// `Sync(DataSync&)`: the birth count only from save version 0x36 on (0 before).
pub fn vfunction81(g: &mut G, this: Ptr, sync: &mut crate::sexy::data_sync::DataSync) -> crate::sexy::data_sync::SyncResult {
    crate::game::game_object::vfunction81(g, this, sync)?;
    use crate::sexy::data_sync::{FUN_00503010, FUN_005030a0};
    let mut d = g.breeder(this).clone();
    FUN_005030a0(sync, &mut d.field_0x4)?;
    FUN_005030a0(sync, &mut d.field_0xc)?;
    FUN_005030a0(sync, &mut d.offset_0x14)?;
    FUN_005030a0(sync, &mut d.field_0x1c)?;
    FUN_00503010(sync, &mut d.field_0x24)?;
    FUN_00503010(sync, &mut d.field_0x28)?;
    FUN_005030a0(sync, &mut d.field_0x2c)?;
    FUN_005030a0(sync, &mut d.offset_0x34)?;
    for f in [
        &mut d.field_0x3c, &mut d.field_0x40, &mut d.field_0x44, &mut d.field_0x48, &mut d.offset_0x4c, &mut d.field_0x50,
        &mut d.field_0x54, &mut d.field_0x58, &mut d.field_0x5c, &mut d.field_0x60, &mut d.field_0x64, &mut d.field_0x68,
        &mut d.field_0x6c, &mut d.field_0x70, &mut d.field_0x74, &mut d.field_0x78, &mut d.field_0x7c, &mut d.offset_0x80,
        &mut d.offset_0x84,
    ] {
        FUN_00503010(sync, f)?;
    }
    if 0x35 < sync.offset_0x8 {
        FUN_00503010(sync, &mut d.offset_0x88)?;
    } else {
        d.offset_0x88 = 0;
    }
    *g.breeder(this) = d;
    Ok(())
}

/// port: 004dcfd0 Sexy::Breeder::vfunction73
/// The coin's worth (GameObject's), doubled at stage 1 and tripled at stage 2.
pub fn vfunction73(g: &mut G, this: Ptr) -> i32 {
    let r = crate::game::game_object::vfunction73(g, this);
    if 0 < r {
        // BallFish's vtable shares this function; its +0x1a0 is the Fish size stage.
        let stage = match &g.go_ext(this).sub {
            GoSub::Breeder(d) => d.offset_0x4c,
            _ => g.fish(this).offset_0x4c,
        };
        match stage {
            1 => return r * 2,
            2 => return r * 3,
            _ => {}
        }
    }
    r
}

/// port: 004ee520 Sexy::Breeder::vfunction76
/// `Remove()`: leaves the tank with its shadow.
pub fn vfunction76(g: &mut G, this: Ptr) {
    FUN_004dd530(g, this, true);
}

/// port: 0053a810 FUN_0053a810
/// The board's object whose virtual-tank id (+0xac) is `param_1` (null when none).
pub fn FUN_0053a810(g: &mut G, this: Ptr, param_1: i32) -> Ptr {
    let set: Vec<Ptr> = g.board(this).offset_0x7c.iter().copied().collect();
    for o in set {
        if g.go(o).offset_0x24 == param_1 {
            return o;
        }
    }
    NULL
}

/// port: 004dd530 FUN_004dd530
/// `Remove(bool withShadow)`: a virtual-tank breeder hands its id to the creature it bore
/// (id + 100); drops what it carries, leaves the widget manager and the board, optionally
/// its shadow.
pub fn FUN_004dd530(g: &mut G, this: Ptr, param_1: bool) {
    let id = g.go(this).offset_0x24;
    if -1 < id {
        let board = board_of(g, this);
        let o = FUN_0053a810(g, board, id + 100);
        if o != NULL {
            g.go(o).offset_0x24 = id;
        }
    }
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
}

/// port: 004dd5d0 FUN_004dd5d0
/// `Die(bool sound)`: the death sound, removal (keeping the shadow) and a dead body of its
/// stage (kind 10 + stage) where it was.
pub fn FUN_004dd5d0(g: &mut G, this: Ptr, param_1: bool) {
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    if param_1 {
        let t = g.go(this).offset_0x4;
        crate::game::board_level::FUN_00538610(g, board, t);
    }
    FUN_004dd530(g, this, false);
    let d = g.breeder(this).clone();
    let right = 0.0 < d.offset_0x14;
    let shadow = g.go(this).offset_0xc;
    let y = ftol(d.field_0xc) as i32;
    let x = ftol(d.field_0x4) as i32;
    let board = g.wfa(app).offset_0x4;
    crate::game::board_level::FUN_005441f0(g, board, x, y, d.offset_0x14, d.field_0x1c, d.field_0x2c, d.offset_0x4c + 10, right, shadow);
}

/// port: 004d5290 FUN_004d5290
/// One step of the eating animation.
pub fn FUN_004d5290(g: &mut G, this: Ptr) {
    g.breeder(this).field_0x78 -= 1;
}

/// port: 004e4ff0 FUN_004e4ff0
/// `std::vector::front()` (an STL instance).
pub fn FUN_004e4ff0(this: &[Ptr]) -> Ptr {
    this[0]
}

/// port: 004ee6b0 FUN_004ee6b0
/// `FindFood()`: the nearest food that is neither a star potion nor dissolving; within
/// 100 pixels the mouth opens (150 updates, +0xf8 = 100). In the virtual tank's feeding
/// mode, the feeder's target instead.
pub fn FUN_004ee6b0(g: &mut G, this: Ptr) -> Ptr {
    if g.go(this).offset_0x68 != 0 {
        let wc = g.wc(this).clone();
        return crate::game::game_object::FUN_004ea010(g, this, wc.offset_0x34 / 2 + wc.offset_0x2c, wc.offset_0x38 / 2 + wc.offset_0x30);
    }
    let board = board_of(g, this);
    let mut best = 100000000;
    let mut food = NULL;
    for o in g.board(board).offset_0xc[vec_index(0xac)].clone() {
        let d = g.breeder(this).clone();
        let (ox, oy) = (g.wc(o).offset_0x2c, g.wc(o).offset_0x30);
        let dx = ftol((ox + 0x14) as f64 - (d.field_0x4 + 40.0)) as i32;
        let dy = ftol((oy + 0x14) as f64 - (d.field_0xc + 40.0)) as i32;
        let n = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx));
        if n < best {
            let f = g.food(o).clone();
            if f.offset_0x40 != 2 && f.offset_0x44 == 0 {
                food = o;
                best = n;
            }
        }
    }
    if best < 10000 {
        crate::game::game_object::FUN_004d6b00(g, this, 0x96);
        g.go(this).offset_0x70 = 100;
    }
    food
}

/// port: 004ee850 FUN_004ee850
/// `TryEat()`: food in its mouth is eaten (vfunction 78, then it goes); food close by
/// opens the mouth (20 updates). In the virtual tank's feeding mode, the feeder decides.
pub fn FUN_004ee850(g: &mut G, this: Ptr) {
    if g.go(this).offset_0x68 == 0 {
        let board = board_of(g, this);
        for o in g.board(board).offset_0xc[vec_index(0xac)].clone() {
            let (ox, oy) = (g.wc(o).offset_0x2c, g.wc(o).offset_0x30);
            let d = g.breeder(this).clone();
            let x = d.field_0x4 + 40.0;
            let y = d.field_0xc + 35.0;
            let f = g.food(o).clone();
            if x < (ox + 0x28) as f64 && (ox as f64) < x && y < (oy + 0x23) as f64 && (oy as f64) < y && f.offset_0x40 != 2 && f.offset_0x44 == 0 {
                vcall!(g, this, go.vfunction78, o as i32);
                crate::game::larva::vfunction76(g, o);
                if g.breeder(this).field_0x78 != 0 {
                    return;
                }
                g.breeder(this).field_0x78 = 8;
                FUN_004d6cc0(g, this);
                return;
            }
            if g.breeder(this).field_0x78 == 0 && x < (ox + 0x37) as f64 && ((ox - 0xf) as f64) < x && y < (oy + 0x28) as f64 && ((oy - 5) as f64) < y && f.offset_0x44 == 0 {
                g.breeder(this).field_0x78 = 0x14;
                FUN_004d6cc0(g, this);
            }
        }
        return;
    }
    let wc = g.wc(this).clone();
    let r = crate::game::game_object::FUN_004ea2a0(g, this, wc.offset_0x34 / 2 + wc.offset_0x2c, wc.offset_0x38 / 2 + wc.offset_0x30);
    if g.breeder(this).field_0x78 == 0 {
        if r == 1 {
            g.breeder(this).field_0x78 = 8;
            FUN_004d6cc0(g, this);
            return;
        }
        if r == 2 {
            g.breeder(this).field_0x78 = 0x14;
            FUN_004d6cc0(g, this);
        }
    }
}

/// port: 004f0880 FUN_004f0880
/// `ChaseFood()`: every 3 updates steers its mouth toward the nearest food (faster when
/// hungry), then tries to eat. True when there is food.
pub fn FUN_004f0880(g: &mut G, this: Ptr) -> bool {
    let t = FUN_004ee6b0(g, this);
    let d = g.breeder(this).clone();
    let ix = ftol(d.field_0x4 + 40.0) as i32;
    let iy = ftol(d.field_0xc + 45.0) as i32;
    if 2 < d.field_0x60 {
        if t == NULL {
            return false;
        }
        g.breeder(this).field_0x60 = 0;
        let hungry = g.go(this).offset_0x14 < 0x12d;
        let (tx, ty) = (g.wc(t).offset_0x2c, g.wc(t).offset_0x30);
        let d = g.breeder(this);
        let (big, small, lim) = if hungry { (1.3, 0.2, 4.0) } else { (1.0, 0.1, 3.0) };
        let v = &mut d.offset_0x14;
        if tx + 0x1c < ix {
            if -lim < *v {
                *v -= big;
            }
        } else if ix < tx + 0xc {
            if *v < lim {
                *v += big;
            }
        } else if tx + 0x18 < ix {
            if -lim < *v {
                *v -= small;
            }
        } else if ix < tx + 0x10 {
            if *v < lim {
                *v += small;
            }
        } else if tx + 0x14 < ix {
            if -lim < *v {
                *v -= 0.05;
            }
        } else if ix < tx + 0x14 && *v < lim {
            *v += 0.05;
        }
        let vy = &mut d.field_0x1c;
        if hungry {
            if ty + 0x1a < iy {
                if -3.0 < *vy {
                    *vy -= 1.0;
                }
            } else if iy < ty + 0xe {
                if *vy < 4.0 {
                    *vy += 1.3;
                }
            } else if ty + 0x14 < iy {
                if -3.0 < *vy {
                    *vy -= 0.5;
                }
            } else if iy < ty + 0x14 && *vy < 4.0 {
                *vy += 0.7;
            }
        } else if ty + 0x1a < iy {
            if -2.0 < *vy {
                *vy -= 0.6;
            }
        } else if iy < ty + 0xe {
            if *vy < 3.0 {
                *vy += 1.0;
            }
        } else if ty + 0x14 < iy {
            if -2.0 < *vy {
                *vy -= 0.3;
            }
        } else if iy < ty + 0x14 && *vy < 3.0 {
            *vy += 0.5;
        }
        if d.field_0x6c < 5 {
            d.field_0x6c += 1;
        }
    }
    if t != NULL {
        FUN_004ee850(g, this);
    }
    t != NULL
}

/// port: 004f1340 FUN_004f1340
/// `Hunt()`: hunger ticks (the hungry look only with no aliens around); starving dies;
/// below 500 it chases food, unless aliens are around (but not when the first is the kind
/// that leaves fish alone).
pub fn FUN_004f1340(g: &mut G, this: Ptr) -> bool {
    crate::game::game_object::FUN_004d6c50(g, this);
    let board = board_of(g, this);
    let aliens_v = g.board(board).offset_0xc[vec_index(0xb8)].clone();
    if aliens_v.is_empty() && g.board(board).offset_0xc[vec_index(0xf4)].is_empty() {
        crate::game::game_object::FUN_004d6ef0(g, this);
    }
    if !crate::game::game_object::FUN_004d69d0(g, this) {
        if g.go(this).offset_0x14 < 500 {
            if !crate::game::board_update::FUN_004da780(g, board) {
                return FUN_004f0880(g, this);
            }
            if !aliens_v.is_empty() {
                let first = FUN_004e4ff0(&aliens_v);
                if g.alien(first).offset_0x98 != 4 {
                    return FUN_004f0880(g, this);
                }
            }
        }
    } else {
        FUN_004dd5d0(g, this, true);
    }
    false
}

/// port: 004f4030 FUN_004f4030
/// `BirthTimer()` (grown breeders): the mouth opens 30 updates before; when the time is up
/// a guppy is born unless it is hungry. In the virtual tank a bought breeder first tries for
/// a bought baby breeder (`FUN_004f3f00`); otherwise the timer becomes 2160, and a grown one
/// drops a guppy only while the guppies out don't outnumber the eaters.
pub fn FUN_004f4030(g: &mut G, this: Ptr) {
    if 0 < g.breeder(this).offset_0x4c {
        g.breeder(this).field_0x7c += 1;
        let d = g.breeder(this).clone();
        if g.go(this).offset_0x74 && d.field_0x7c == d.offset_0x80 - 0x1e && d.offset_0x4c == 2 {
            FUN_004d6cc0(g, this);
        }
        let d = g.breeder(this).clone();
        if d.offset_0x80 <= d.field_0x7c {
            g.breeder(this).field_0x7c = 0;
            if crate::game::game_object::FUN_004d6bd0(g, this) {
                if mode(g, this) == 5 && FUN_004f3f00(g, this) {
                    return;
                }
                if mode(g, this) == 5 {
                    g.breeder(this).offset_0x80 = 0x870;
                    if g.breeder(this).offset_0x4c != 2 {
                        return;
                    }
                    let board = board_of(g, this);
                    let need = crate::game::virtual_tank::FUN_0053a3c0(g, board);
                    let have = crate::game::virtual_tank::FUN_005397a0(g, board);
                    let d = need[0] - have[0];
                    if d == -1 || d + 1 < 0 {
                        return;
                    }
                }
                let board = board_of(g, this);
                let (mx, my) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
                let baby = crate::game::board_level::FUN_00546ef0(g, board, mx + 5, my + 10);
                if mode(g, this) == 5 {
                    g.go(baby).offset_0x98 = if g.go(this).offset_0x68 != 1000 { 0x28 } else { 0x96 };
                }
                crate::game::board_level::FUN_005382c0(g, board, true);
            }
        }
    }
}

/// port: 004f3f00 FUN_004f3f00
/// `HaveBaby()` (virtual tank): a bought, grown breeder with a free tank slot and no baby
/// yet gives birth to a small bought fish named after it with " JR.", bought now, in that
/// slot (dropped where it is, mouth open); true when born.
pub fn FUN_004f3f00(g: &mut G, this: Ptr) -> bool {
    let mut slot = 0;
    if !crate::game::game_object::FUN_004d5240(g, this, Some(&mut slot)) {
        return false;
    }
    let (x, y) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
    let baby = crate::game::fish::Fish__004eef00(g, x + 5, y + 10);
    let board = board_of(g, this);
    crate::game::board_level::FUN_005382c0(g, board, false);
    crate::game::game_object::FUN_004d6720(g, baby);
    crate::game::game_object::FUN_004f21e0(g, baby, this);
    let now = g.now_time64;
    let go = g.go(baby);
    go.offset_0x24 = slot;
    go.field_0x48 = now;
    go.field_0x28.extend_from_slice(b" JR.");
    go.offset_0x54 = 0;
    g.fish(baby).offset_0xd6 = true;
    if g.go(baby).offset_0x74 {
        crate::game::game_object::FUN_004d6cc0(g, baby);
    }
    let board = board_of(g, this);
    crate::game::store::FUN_00544940(g, board, baby, false);
    g.breeder(this).offset_0x88 += 1;
    true
}

/// port: 004dd000 FUN_004dd000
/// `Animate()`: the drop-in fade, turning, the eating cels or the swim cycle, the facing,
/// carrying, the open mouth cel, the grow flash.
pub fn FUN_004dd000(g: &mut G, this: Ptr) {
    if g.go(this).offset_0x74 {
        crate::game::game_object::FUN_004d6c90(g, this);
    }
    {
        let d = g.breeder(this);
        if d.offset_0x34 <= 0.0 || 0.0 <= d.offset_0x14 {
            if d.offset_0x34 < 0.0 && 0.0 < d.offset_0x14 {
                d.field_0x74 = -0x14;
            }
        } else {
            d.field_0x74 = 0x14;
        }
    }
    let v = g.breeder(this).field_0x74;
    if v != 0 {
        g.breeder(this).field_0x74 = if 0 < v { v - 1 } else { v + 1 };
        if g.breeder(this).field_0x78 != 0 {
            FUN_004d5290(g, this);
        }
    }
    let turn = g.breeder(this).field_0x74;
    if turn == 0 {
        if 0 < g.breeder(this).field_0x78 {
            FUN_004d5290(g, this);
            let d = g.breeder(this);
            d.field_0x70 = 9 - d.field_0x78 / 2;
        } else {
            let d = g.breeder(this);
            d.field_0x68 += if d.field_0x6c < 2 { 1 } else { 2 };
            if 0x13 < d.field_0x68 {
                d.field_0x68 = 0;
            }
            d.field_0x70 = d.field_0x68 / 2;
        }
    } else if 0 < turn {
        g.breeder(this).field_0x70 = 9 - turn / 2;
    } else {
        g.breeder(this).field_0x70 = turn / 2 + 9;
    }
    {
        let d = g.breeder(this);
        if d.offset_0x14 != d.offset_0x34 && 0.0 != d.offset_0x14 && 0.0 != d.offset_0x34 {
            d.offset_0x34 = d.offset_0x14;
        }
    }
    if g.go(this).offset_0x10 != NULL {
        crate::game::game_object::FUN_004d7020(g, this);
    }
    if 100 < g.go(this).offset_0x80 && g.breeder(this).field_0x74 == 0 {
        g.breeder(this).field_0x70 = 4;
    }
    let d = g.breeder(this);
    if 0 < d.field_0x58 {
        d.field_0x58 -= 1;
    }
}

/// port: 004d52a0 FUN_004d52a0
/// The sprite row (3 per stage): turning, swimming, eating or with the mouth open; the
/// hungry sheet's row for its stage.
pub fn FUN_004d52a0(g: &mut G, this: Ptr, param_1: bool) -> i32 {
    let d = g.breeder(this).clone();
    if d.field_0x74 != 0 {
        return d.offset_0x4c * 3 + 1;
    }
    if d.field_0x78 < 1 && g.go(this).offset_0x80 < 0x65 {
        return d.offset_0x4c * 3;
    }
    if param_1 {
        return d.offset_0x4c + 9;
    }
    d.offset_0x4c * 3 + 2
}

/// port: 004dd280 FUN_004dd280
/// `DrawBody(Graphics*, bool mirror)`: the cel in its row (from the hungry sheet when
/// hungry), scaled up while the grow flash runs, the hungry tint fading over it, what it
/// carries, the sparkle; the plain sheet's cel in the `DAT_005e89ce` mode.
pub fn FUN_004dd280(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: bool) {
    if !g.globals.DAT_005e89ce {
        let cel = g.breeder(this).field_0x70 * 0x50;
        let hungry = FUN_004d6f30(g, this);
        let row = FUN_004d52a0(g, this, hungry) * 0x50;
        let mut src = Rect::new(cel, row, 0x50, 0x50);
        let mut dest = Rect::new(0, 0, 0x50, 0x50);
        let img = if hungry { g.res.DAT_005e8dc4 } else { g.res.DAT_005e8d4c };
        if g.go(this).offset_0x74 && g.breeder(this).field_0x58 == 0 && crate::game::game_object::FUN_004d6e70(g, this, gfx, img, &src, param_2) {
            return;
        }
        let flash = g.breeder(this).field_0x58;
        if 0 < flash && g.breeder(this).offset_0x4c != 3 {
            let k = if 3 < flash {
                ((10 - flash) as f64 * 0.5 / 7.0 + f64::from_bits(0x3fe6666660000000)) as f32
            } else {
                (flash as f64 * f64::from_bits(0x3fc99999a0000000) / 3.0 + 1.0) as f32
            };
            let e = ftol((k as f64 - 1.0) * 80.0 * 0.5) as i32;
            dest.mX -= e;
            dest.mY -= e;
            dest.mWidth += e * 2;
            dest.mHeight += e * 2;
            let app = g.go(this).offset_0x0;
            let is3d = crate::sexy::sexy_app_base::dtor_MemoryImage__00489a20(g, app);
            FUN_00455900(gfx, !is3d);
        }
        FUN_004d6db0(g, this, gfx, Color::WHITE);
        let img = if hungry { g.res.DAT_005e8dc4 } else { g.res.DAT_005e8d4c };
        crate::game::shadow::FUN_005008f0(g, gfx, img, &dest, &src, param_2);
        let t = g.go(this).offset_0x18;
        if t != 0 {
            src.mY = FUN_004d52a0(g, this, true) * 0x50;
            FUN_004d6db0(g, this, gfx, CRect(0xff, 0xff, 0xff, (t * 0xff) / 5));
            let img = g.res.DAT_005e8dc4;
            crate::game::shadow::FUN_005008f0(g, gfx, img, &dest, &src, param_2);
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
        let stage = g.breeder(this).offset_0x4c;
        let col = if FUN_004d6f30(g, this) { 6 } else { 9 };
        let img = g.res.DAT_005e8dc4;
        FUN_004560a0(gfx, g, img, 0, 0, &Rect::new(col * 0x50, (stage * 3 + 2) * 0x50, 0x50, 0x50), param_2);
        if g.go(this).offset_0x10 != NULL {
            FUN_004d7040(g, this, gfx, 0, 0);
        }
        if crate::game::game_object::FUN_004d6980(g, this, 500) {
            let s = g.res.DAT_005e8bc4;
            FUN_00456950(gfx, g, s, 0, 0, 2);
        }
    }
}

/// port: 004e4490 Sexy::Breeder::vfunction27
/// `Draw(Graphics*)`: hidden while held in place under the cursor (stages 0 and 1);
/// otherwise the body mirrored by facing (or turn direction), then its name label.
pub fn vfunction27(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    crate::game::food::FUN_004d6ae0(g, this);
    let d = g.breeder(this).clone();
    if g.globals.DAT_005e89c0 != 0 {
        let (tx, ty) = (g.globals.DAT_005e89c4, g.globals.DAT_005e89c8);
        if d.field_0x4 < (tx + 8) as f64 && ((tx - 8) as f64) < d.field_0x4 && d.field_0xc < (ty + 8) as f64 && ((ty - 8) as f64) < d.field_0xc && d.offset_0x4c < 2 {
            return;
        }
    }
    let m1 = g.go(this).offset_0x6d;
    let m0 = !m1;
    let turn = d.field_0x74;
    let draw = if turn == 0 {
        let vx = d.offset_0x14;
        if vx < 0.0 {
            Some(m1)
        } else {
            let e = ftol(vx);
            if e == 0 && d.offset_0x34 < 0.0 {
                Some(m1)
            } else if 0.0 < vx {
                Some(m0)
            } else if e != 0 {
                None
            } else if 0.0 < d.offset_0x34 {
                Some(m0)
            } else {
                None
            }
        }
    } else if turn < 1 {
        if turn < 0 { Some(m1) } else { None }
    } else {
        Some(m0)
    };
    if let Some(m) = draw {
        FUN_004dd280(g, this, gfx, m);
    }
    // +0xc4: the name's length.
    if !g.go(this).field_0x28.is_empty() {
        crate::game::game_object::FUN_004e1400(g, this, gfx, false);
    }
}

/// port: 004e4600 Sexy::Breeder::vfunction78
/// `Eat(Food*)`: the eat sound; fed by the food's grade (counted toward growing, more
/// outside the virtual tank); grown a stage when it has eaten enough (stage 1: the birth
/// timer starts and the shop's second slot opens; stage 2: births every 500..699 updates,
/// the shop's last slot in the +0x87c mode 1) with the grow sound and flash.
pub fn vfunction78(g: &mut G, this: Ptr, param_1: i32) {
    let food = param_1 as Ptr;
    let hungry = FUN_004d6f30(g, this);
    crate::game::game_object::FUN_004d6a30(g, this, false);
    let m = mode(g, this);
    let count = if m == 5 {
        if g.go(this).offset_0x24 < 0 { false } else { crate::game::game_object::FUN_004d6f90(g, this) }
    } else {
        true
    };
    let board = board_of(g, this);
    let high = g.go(this).offset_0x7c;
    crate::game::board_level::FUN_005384c0(g, board, high);
    match crate::game::fish::eaten_word(g, food, 0x188) {
        0 => {
            g.go(this).offset_0x14 += 500;
            if 800 < g.go(this).offset_0x14 {
                g.go(this).offset_0x14 = 800;
            }
            if count {
                g.breeder(this).field_0x50 += 1;
            }
        }
        1 => {
            g.go(this).offset_0x14 += 700;
            if 1000 < g.go(this).offset_0x14 {
                g.go(this).offset_0x14 = 1000;
            }
            if count {
                g.breeder(this).field_0x50 += (m != 5) as i32 + 1;
            }
        }
        _ => {
            if m == 5 {
                g.go(this).offset_0x14 += 500;
                if 800 < g.go(this).offset_0x14 {
                    g.go(this).offset_0x14 = 800;
                }
            } else {
                g.go(this).offset_0x14 += 0x44c;
                if 0x578 < g.go(this).offset_0x14 {
                    g.go(this).offset_0x14 = 0x578;
                }
            }
            if count {
                g.breeder(this).field_0x50 += (m != 5) as i32 + 2;
            }
        }
    }
    let d = g.breeder(this).clone();
    if d.field_0x54 <= d.field_0x50 && d.offset_0x4c < 2 {
        let stage = d.offset_0x4c + 1;
        g.breeder(this).offset_0x4c = stage;
        if stage == 1 {
            g.breeder(this).field_0x7c = 900;
            if m != 5 {
                let r = rand(g, this);
                g.breeder(this).field_0x54 = (r % 5) as i32 + 5;
            }
            crate::game::board_level::FUN_005409b0(g, board, 1, true);
        }
        if g.breeder(this).offset_0x4c == 2 {
            if -1 < g.go(this).offset_0x24 {
                g.breeder(this).field_0x7c = 0;
            }
            let r = rand(g, this);
            g.breeder(this).offset_0x80 = (r % 200) as i32 + 500;
            if mode(g, this) == 1 {
                crate::game::board_level::FUN_005409b0(g, board, 0xb, true);
            }
        }
        let app = g.go(this).offset_0x0;
        {
            let d = g.breeder(this);
            d.field_0x50 = 0;
            d.field_0x58 = 10;
        }
        let snd = g.res.DAT_005e8af0;
        crate::sexy::sexy_app_base::vfunction55(g, app, snd);
    }
    crate::game::game_object::FUN_004e13d0(g, this, hungry);
}

/// port: 004e4870 Sexy::Breeder::vfunction55
/// `MouseDown(x, y, clicks)`: a right click goes to the tank; a left click on it (with no
/// aliens around, inside the tank, not on another object) drops food there when the money
/// allows, as clicking the tank would.
pub fn vfunction55(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32) {
    let board = board_of(g, this);
    if param_3 < 0 {
        let (x, y) = (g.wc(this).offset_0x2c + param_1, g.wc(this).offset_0x30 + param_2);
        crate::game::board_level::FUN_0053bfb0(g, board, x, y);
        return;
    }
    let b = g.board(board);
    if !b.offset_0xc[vec_index(0xb8)].is_empty() || !b.offset_0xc[vec_index(0xf4)].is_empty() || !b.offset_0xc[vec_index(0xdc)].is_empty() {
        return;
    }
    let d = g.breeder(this).clone();
    let dx = param_1 as f64 + d.field_0x4;
    if !(dx < 587.0 && 30.0 < dx) {
        return;
    }
    let dy = param_2 as f64 + d.field_0xc;
    if !(dy < 400.0 && 60.0 < dy) {
        return;
    }
    let (x, y) = (g.wc(this).offset_0x2c + param_1, g.wc(this).offset_0x30 + param_2);
    if crate::game::board_level::FUN_00539f30(g, board, x, y) {
        return;
    }
    let cost = g.board(board).ext_0x4ac;
    if !crate::game::board_update::FUN_00540b30(g, board, cost, true) {
        return;
    }
    let off = if param_1 < 10 || 0x46 < param_1 || param_2 < 10 || 0x46 < param_2 { 0 } else { (0x46 - param_2) / 2 };
    let fy = ftol((d.field_0xc + param_2 as f64) - 10.0) as i32;
    let fx = ftol((d.field_0x4 + param_1 as f64) - 10.0) as i32;
    crate::game::board_level::FUN_00543280(g, board, fx, fy, 0, false, off, -1);
    g.board(board).ext_0x4ec = true;
    let t = crate::game::board::FUN_00537b60(g, board);
    g.board(board).field_0x334 = t;
}

/// port: 004dd190 Sexy::Breeder::vfunction80
/// `DrawIcon(Graphics*, int pose)`: its stage's portrait cel offset by pose.
pub fn vfunction80(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: i32) {
    let (dx, dy) = match param_2 {
        0 => (0x12, 0x14),
        1 => (7, 0x14),
        2 => (0xf, 0),
        3 => (0xc, 0),
        4 => (-0x28, -5),
        _ => (0, 0),
    };
    FUN_004563d0(gfx, dx, dy);
    let stage = g.breeder(this).offset_0x4c;
    crate::game::game_object::FUN_004d6d10(g, this, gfx, Color::WHITE);
    let col = g.go(this).offset_0xa8 * 0x50;
    let img = g.res.DAT_005e8d4c;
    FUN_00455e40(gfx, g, img, 0, 0, &Rect::new(col, stage * 0xf0, 0x50, 0x50));
    FUN_004558e0(gfx, false);
    FUN_004563d0(gfx, -dx, -dy);
}

/// One axis of hiding behind Gumbo: beyond `far_hi` / `far_lo` by `big`, beyond `mid_hi` /
/// `mid_lo` by `mid`, around `c` by `small` (the pattern of `vfunction23`).
fn hide_x(v: &mut f64, x: f64, gx: i32) {
    if x <= (gx + 0x32) as f64 {
        if ((gx + 0x1e) as f64) <= x {
            if x <= (gx + 0x2d) as f64 {
                if ((gx + 0x23) as f64) <= x {
                    if x <= (gx + 0x28) as f64 {
                        if x < (gx + 0x28) as f64 && *v < 4.0 {
                            *v += 0.05;
                        }
                    } else if -4.0 < *v {
                        *v -= 0.05;
                    }
                } else if *v < 4.0 {
                    *v += 0.2;
                }
            } else if -4.0 < *v {
                *v -= 0.2;
            }
        } else if *v < 4.0 {
            *v += 1.3;
        }
    } else if -4.0 < *v {
        *v -= 1.3;
    }
}

/// port: 004f4180 Sexy::Breeder::vfunction23
/// `Update()` (not while paused): held in place under the cursor (stages 0 and 1, outside
/// the virtual tank); otherwise hunting, or hiding behind Gumbo while aliens are around,
/// or wandering in its swim mode; the mode re-roll; the birth timer (no aliens around);
/// the drop-in slowing; sinking when still; staying in the tank (bubbles while dropping
/// in); slowing at the walls; animation; moving (fast while dropping in).
pub fn vfunction23(g: &mut G, this: Ptr) {
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    if board == NULL || g.board(board).field_0x8 {
        return;
    }
    crate::game::game_object::FUN_004f22c0(g, this);
    'tail: {
        if g.wfa(app).offset_0x150 == 5 || g.globals.DAT_005e89c0 == 0 || 1 < g.breeder(this).offset_0x4c {
            if FUN_004f1340(g, this) {
                break 'tail;
            }
            let board = g.wfa(app).offset_0x4;
            if crate::game::board_update::FUN_004da780(g, board) && g.board(board).field_0xb8[12] != 0 {
                let pets = g.board(board).offset_0xc[vec_index(0xb4)].clone();
                let gumbo = *pets.iter().find(|&&p| crate::game::fish_type_pet::pet_id(g, p) == 0xc).expect("Breeder: Gumbo counted but not in the pet list");
                let (gx, gy) = (g.wc(gumbo).offset_0x2c, g.wc(gumbo).offset_0x30);
                let my = g.wc(this).offset_0x30;
                let d = g.breeder(this);
                let x = d.field_0x4 + 40.0;
                hide_x(&mut d.offset_0x14, x, gx);
                let y = d.field_0xc + 40.0;
                let vy = &mut d.field_0x1c;
                if y <= (gy + 0x19) as f64 {
                    if ((gy + 0xf) as f64) <= y {
                        if y <= (gy + 0x14) as f64 {
                            if y < (gy + 0x14) as f64 && *vy < 4.0 {
                                *vy += 0.7;
                            }
                        } else if -3.0 < *vy {
                            *vy -= 0.5;
                        }
                    } else if *vy < 4.0 {
                        *vy += 1.3;
                    }
                } else if -3.0 < *vy {
                    *vy -= 1.0;
                }
                if my <= d.field_0x44 && d.field_0x1c < 0.0 {
                    d.field_0x1c = 0.0;
                }
                if d.field_0x6c < 5 {
                    d.field_0x6c += 1;
                }
                break 'tail;
            }
            let d = g.breeder(this);
            let m = d.field_0x5c;
            if 4 < m {
                if d.offset_0x84 == 0 {
                    d.field_0x1c = if 115.0 <= d.field_0xc { -0.5 } else { -0.1 };
                }
                if 0x27 < d.field_0x60 {
                    let dir = d.field_0x24;
                    d.field_0x60 = 0;
                    if dir == 1 {
                        d.offset_0x14 += if d.offset_0x14 < 0.0 { 2.0 } else { 1.0 };
                        d.field_0x6c = ftol(d.offset_0x14.abs()) as i32;
                        if 250.0 < d.field_0x4 {
                            d.field_0x24 = -1;
                            d.offset_0x14 -= 2.0;
                        }
                    } else if dir == -1 {
                        d.offset_0x14 -= if 0.0 < d.offset_0x14 { 2.0 } else { 1.0 };
                        d.field_0x6c = ftol(d.offset_0x14.abs()) as i32;
                        if d.field_0x4 < 175.0 {
                            d.field_0x24 = 1;
                            d.offset_0x14 += 2.0;
                        }
                    }
                }
                break 'tail;
            }
            let fall;
            match m {
                0 => {
                    if d.offset_0x84 == 0 {
                        d.field_0x1c = 0.5;
                    }
                    if 0x27 < d.field_0x60 {
                        d.field_0x60 = 0;
                        if d.offset_0x14 <= 0.0 {
                            if d.offset_0x14 < 0.0 && d.offset_0x14 < -0.5 {
                                d.offset_0x14 += 0.5;
                            }
                        } else if 0.5 < d.offset_0x14 {
                            d.offset_0x14 -= 0.5;
                        }
                        d.field_0x6c = ftol(d.offset_0x14.abs()) as i32;
                    }
                    fall = 0.25;
                }
                1 => {
                    if d.offset_0x84 == 0 {
                        d.field_0x1c = -0.5;
                    }
                    if 0x27 < d.field_0x60 {
                        d.field_0x60 = 0;
                        if d.offset_0x14 <= 1.0 {
                            if d.offset_0x14 < 1.0 {
                                d.offset_0x14 += 1.0;
                            }
                        } else {
                            d.offset_0x14 -= 1.0;
                        }
                        d.field_0x6c = ftol(d.offset_0x14.abs()) as i32;
                    }
                    fall = 0.5;
                }
                2 => {
                    if d.offset_0x84 == 0 {
                        d.field_0x1c = -0.5;
                    }
                    if 0x27 < d.field_0x60 {
                        d.field_0x60 = 0;
                        if d.offset_0x14 <= -1.0 {
                            if d.offset_0x14 < -1.0 {
                                d.offset_0x14 += 1.0;
                            }
                        } else {
                            d.offset_0x14 -= 1.0;
                        }
                        d.field_0x6c = ftol(d.offset_0x14.abs()) as i32;
                    }
                    fall = 0.5;
                }
                3 | 4 => {
                    if 0x27 < d.field_0x60 {
                        d.field_0x60 = 0;
                        if m == 3 {
                            if d.offset_0x14 <= -1.0 {
                                if d.offset_0x14 < -1.0 {
                                    d.offset_0x14 += 1.0;
                                }
                            } else {
                                d.offset_0x14 -= 1.0;
                            }
                        } else if d.offset_0x14 <= 1.0 {
                            if d.offset_0x14 < 1.0 {
                                d.offset_0x14 += 1.0;
                            }
                        } else {
                            d.offset_0x14 -= 1.0;
                        }
                        if d.field_0x1c <= 3.0 {
                            if d.field_0x1c < 3.0 {
                                d.field_0x1c += 1.0;
                            }
                        } else {
                            d.field_0x1c -= 1.0;
                        }
                        if d.field_0x6c < 5 {
                            if d.field_0x1c < 4.0 {
                                d.field_0x6c += 1;
                            }
                        } else {
                            d.field_0x6c -= 1;
                        }
                    }
                    if 240.0 < d.field_0xc {
                        d.field_0x5c = 0;
                    }
                    break 'tail;
                }
                _ => {
                    if m == -1 {
                        d.field_0x1c = -15.0;
                    }
                    break 'tail;
                }
            }
            d.field_0xc -= fall / d.field_0x2c;
        } else {
            let (tx, ty) = (g.globals.DAT_005e89c4, g.globals.DAT_005e89c8);
            let d = g.breeder(this);
            d.offset_0x14 = if d.offset_0x14 < 0.0 { -0.5 } else { 0.5 };
            if d.offset_0x84 == 0 {
                d.field_0x1c = 0.0;
            }
            if d.field_0x4 <= (tx + 0x32) as f64 {
                if d.field_0x4 < (tx - 0x32) as f64 {
                    d.field_0x4 += 5.0;
                } else if ((tx + 5) as f64) < d.field_0x4 {
                    d.field_0x4 -= 3.0;
                } else if d.field_0x4 < (tx - 5) as f64 {
                    d.field_0x4 += 3.0;
                }
            } else {
                d.field_0x4 -= 5.0;
            }
            if d.field_0xc <= (ty + 0x32) as f64 {
                if ((ty - 0x32) as f64) <= d.field_0xc {
                    if d.field_0xc <= (ty + 5) as f64 {
                        if d.field_0xc < (ty - 5) as f64 {
                            d.field_0xc += 3.0;
                        }
                    } else {
                        d.field_0xc -= 3.0;
                    }
                } else {
                    d.field_0xc += 4.0;
                }
            } else {
                d.field_0xc -= 4.0;
            }
        }
    }
    {
        let d = g.breeder(this);
        d.field_0x60 += 1;
        d.field_0x64 += 1;
    }
    if 0x14 < g.breeder(this).field_0x64 {
        g.breeder(this).field_0x64 = 0;
        if rand(g, this) % 10 == 0 || g.breeder(this).field_0x5c == -1 {
            let m = (rand(g, this) % 9 + 1) as i32;
            g.breeder(this).field_0x5c = m;
        }
    }
    let board = g.wfa(app).offset_0x4;
    if g.board(board).offset_0xc[vec_index(0xb8)].is_empty() && g.board(board).offset_0xc[vec_index(0xf4)].is_empty() {
        FUN_004f4030(g, this);
    }
    let d = g.breeder(this);
    if d.offset_0x84 != 0 {
        let vy = d.field_0x1c;
        d.offset_0x84 -= 1;
        d.field_0x1c = vy * 0.9;
    }
    if d.offset_0x14 == 0.0 {
        d.field_0xc = 1.0 / d.field_0x2c + d.field_0xc;
    }
    if d.offset_0x14 == 1.0 {
        d.field_0xc = 0.75 / d.field_0x2c + d.field_0xc;
    }
    if d.offset_0x14 == 2.0 {
        d.field_0xc = 0.5 / d.field_0x2c + d.field_0xc;
    }
    if d.offset_0x14 == 3.0 {
        d.field_0xc = 0.25 / d.field_0x2c + d.field_0xc;
    }
    let floor = d.field_0x3c;
    if floor < 0x141 {
        d.field_0xc -= 0.25;
    }
    let right = d.field_0x48 as f64;
    if right < d.field_0x4 {
        d.field_0x4 = right;
    }
    let left = d.field_0x40 as f64;
    if d.field_0x4 < left {
        d.field_0x4 = left;
    }
    if (floor as f64) < d.field_0xc {
        d.field_0xc = floor as f64;
    }
    let drop = d.offset_0x84;
    if drop < 1 || d.field_0x1c <= 0.0 {
        let top = d.field_0x44 as f64;
        if d.field_0xc < top {
            d.field_0xc = top;
        }
    } else if 0x1e < drop {
        let k = if 0x28 < drop { 1 } else { 2 };
        if rand(g, this) % k == 0 {
            let r1 = rand(g, this);
            let r2 = rand(g, this);
            let (mx, my) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
            crate::game::board_update::FUN_00538ac0(g, board, mx + (0x28 - (r1 % 0x1e) as i32), my + (0x28 - (r2 % 0x1e) as i32));
        }
    }
    let d = g.breeder(this).clone();
    if d.field_0xc <= d.field_0x44 as f64 && d.field_0x5c == -1 {
        let m = (rand(g, this) % 9 + 1) as i32;
        g.breeder(this).field_0x5c = m;
    }
    {
        let d = g.breeder(this);
        if ((d.field_0x48 - 5) as f64) < d.field_0x4 && 0.1 < d.offset_0x14 {
            d.offset_0x14 -= 0.1;
        }
        if d.field_0x4 < 15.0 && d.offset_0x14 < -0.1 {
            d.offset_0x14 += 0.1;
        }
    }
    FUN_004dd000(g, this);
    let mut div = g.breeder(this).field_0x2c;
    if g.go(this).offset_0x6e {
        div = if g.go(this).offset_0x70 == 0 { 0.3 } else { 0.8 };
    }
    let d = g.breeder(this);
    d.field_0x4 = d.offset_0x14 / div + d.field_0x4;
    d.field_0xc = d.field_0x1c / div + d.field_0xc;
    let (x, y) = (ftol(d.field_0x4) as i32, ftol(d.field_0xc) as i32);
    vcall!(g, this, w.vfunction42, x, y);
}
