//! `Sexy::Fish`: the guppy, and the base of the other swimming fish. `Fish_data` starts at
//! object offset 0x154; the object is 0x230 bytes.

use crate::game::game_object::GameObject;
use crate::sexy::prelude::*;
use crate::sexy::types::CRect;

/// `Fish_data` (object offset 0x154, 0xd8 bytes; field names relative to 0x154).
#[derive(Debug, Clone, Default)]
pub struct Fish_data {
    /// +0x154.
    pub field_0x0: i32,
    /// +0x158 x.
    pub field_0x4: f64,
    /// +0x160 y.
    pub field_0xc: f64,
    /// +0x168 x speed.
    pub offset_0x14: f64,
    /// +0x170 y speed.
    pub field_0x1c: f64,
    /// +0x178 sweep direction (1 / -1) of swim mode 5+.
    pub field_0x24: i32,
    /// +0x17c.
    pub field_0x28: i32,
    /// +0x180 speed divisor (2.0, 1.8 or 1.6).
    pub field_0x2c: f64,
    /// +0x188 the x speed the facing follows (+1 / -1 initially).
    pub offset_0x34: f64,
    /// +0x190 lowest y (370).
    pub field_0x3c: i32,
    /// +0x194 leftmost x (10).
    pub field_0x40: i32,
    /// +0x198 highest y (95).
    pub field_0x44: i32,
    /// +0x19c rightmost x (540).
    pub field_0x48: i32,
    /// +0x1a0 size: 0 small, 1 medium, 2 large, 3 king, 4 star.
    pub offset_0x4c: i32,
    /// +0x1a4 food eaten toward the next size.
    pub field_0x50: i32,
    /// +0x1a8 food needed to grow (4..6).
    pub field_0x54: i32,
    /// +0x1ac swim mode (0..9; 0x7b = drifting).
    pub field_0x58: i32,
    /// +0x1b0 updates in the swim mode.
    pub field_0x5c: i32,
    /// +0x1b4 updates since the last mode roll.
    pub field_0x60: i32,
    /// +0x1b8 swim animation counter.
    pub field_0x64: i32,
    /// +0x1bc speed level (0..5).
    pub field_0x68: i32,
    /// +0x1c0 animation cel.
    pub field_0x6c: i32,
    /// +0x1c4 turn counter (+20 / -20 counting to 0).
    pub field_0x70: i32,
    /// +0x1c8 eating animation counter.
    pub field_0x74: i32,
    /// +0x1cc grow flash counter.
    pub field_0x78: i32,
    /// +0x1d0 colored (virtual tank fish drawn in three tinted layers).
    pub field_0x7c: bool,
    /// +0x1d1.
    pub field_0x7d: bool,
    /// +0x1d4.
    pub offset_0x80: i32,
    /// +0x1d8 color of the third layer.
    pub field_0x84: Color,
    /// +0x1e8 color of the second layer.
    pub field_0x94: Color,
    /// +0x1f8 a third color.
    pub field_0xa4: Color,
    /// +0x208 y-speed damping updates left.
    pub field_0xb4: i32,
    /// +0x20c push updates left.
    pub offset_0xb8: i32,
    /// +0x210.
    pub offset_0xbc: i32,
    /// +0x214.
    pub field_0xc0: i32,
    /// +0x218 push x speed.
    pub offset_0xc4: f64,
    /// +0x220 updates since the last coin.
    pub field_0xcc: i32,
    /// +0x224 updates between coins.
    pub field_0xd0: i32,
    /// +0x228 tutorial fish (hunger hints, never starves silently).
    pub field_0xd4: bool,
    /// +0x229.
    pub field_0xd5: bool,
    /// +0x22a.
    pub offset_0xd6: bool,
}

impl G {
    pub fn fish(&mut self, p: Ptr) -> &mut Fish_data {
        match &mut self.go_ext(p).sub {
            GoSub::Fish(d) | GoSub::FishTypePet(d, _) | GoSub::BiFish(d, _) => d,
            s => panic!("{p} is not a Fish: {s:?}"),
        }
    }
}

/// `Fish_data` of a fish.
pub fn fish_data(g: &mut G, p: Ptr) -> &mut Fish_data {
    g.fish(p)
}

fn alloc_fish(g: &mut G, wc: crate::sexy::object::WidgetContainer_data, w: crate::sexy::object::Widget_data,
              go: crate::game::game_object::GameObject_data, fish: Fish_data) -> Ptr {
    g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__Fish_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::GameObject(Box::new(GameObjectExt { go, sub: GoSub::Fish(fish) })) }),
    })
}

/// port: 004eee60 Sexy::Fish::Fish
/// `Fish::Fish()` (used before loading from a save).
pub fn Fish__004eee60(g: &mut G) -> Ptr {
    let (mut wc, w, mut go) = GameObject(g);
    let f = Fish_data {
        field_0x84: crate::sexy::types::FUN_00433300(),
        field_0x94: crate::sexy::types::FUN_00433300(),
        field_0xa4: crate::sexy::types::FUN_00433300(),
        offset_0xc4: 0.0,
        offset_0xb8: 0,
        offset_0xbc: 0,
        offset_0xd6: false,
        offset_0x80: 0,
        ..Default::default()
    };
    wc.offset_0x3d = false;
    go.offset_0x4 = 0;
    alloc_fish(g, wc, w, go, f)
}

/// port: 004eef00 Sexy::Fish::Fish
/// `Fish::Fish(int x, int y)`: a new small guppy.
pub fn Fish__004eef00(g: &mut G, param_1: i32, param_2: i32) -> Ptr {
    let (wc, w, go) = GameObject(g);
    let f = Fish_data {
        field_0x84: crate::sexy::types::FUN_00433300(),
        field_0x94: crate::sexy::types::FUN_00433300(),
        field_0xa4: crate::sexy::types::FUN_00433300(),
        ..Default::default()
    };
    let this = alloc_fish(g, wc, w, go, f);
    FUN_004de370(g, this, param_1, param_2);
    g.fish(this).offset_0x4c = 0;
    this
}

/// port: 004eef90 Sexy::Fish::Fish
/// `Fish::Fish(int x, int y, int size, bool facingRight)`.
pub fn Fish__004eef90(g: &mut G, param_1: i32, param_2: i32, param_3: i32, param_4: bool) -> Ptr {
    let (wc, w, go) = GameObject(g);
    let f = Fish_data {
        field_0x84: crate::sexy::types::FUN_00433300(),
        field_0x94: crate::sexy::types::FUN_00433300(),
        field_0xa4: crate::sexy::types::FUN_00433300(),
        ..Default::default()
    };
    let this = alloc_fish(g, wc, w, go, f);
    FUN_004de370(g, this, param_1, param_2);
    let dir = if param_4 { 1.0 } else { -1.0 };
    let d = g.fish(this);
    d.offset_0x4c = param_3;
    d.offset_0x14 = dir;
    d.offset_0x34 = dir;
    this
}

/// port: 004de370 FUN_004de370
/// `Fish::Init(int x, int y)`: 80x80 at (x, y), drifting with a random speed divisor,
/// hunger, growth need, coin interval and first swim mode; marked as a tutorial fish on the
/// first level; not mouse-visible while a selection (+0xb8 / +0xf4 vectors) is active.
pub fn FUN_004de370(g: &mut G, this: Ptr, param_1: i32, param_2: i32) {
    let app = g.go(this).offset_0x0;
    let rng = g.wfa(app).offset_0x84;
    let ftol = crate::sexy::crt::ftol;
    {
        let d = g.fish(this);
        d.field_0x4 = param_1 as f64;
        d.field_0xc = param_2 as f64;
    }
    g.wc(this).offset_0x3d = false;
    g.go(this).offset_0x4 = 0;
    g.fish(this).offset_0xd6 = false;
    let (x, y) = (g.fish(this).field_0x4, g.fish(this).field_0xc);
    g.wc(this).offset_0x2c = ftol(x) as i32;
    g.wc(this).offset_0x30 = ftol(y) as i32;
    {
        let d = g.fish(this);
        d.offset_0x14 = 0.0;
        d.field_0x1c = -0.5;
    }
    g.wc(this).offset_0x34 = 0x50;
    g.wc(this).offset_0x38 = 0x50;
    g.fish(this).offset_0x34 = 1.0;
    let r = crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng));
    if r & 1 == 0 {
        let d = g.fish(this);
        d.offset_0x14 = -0.1;
        d.offset_0x34 = -1.0;
    }
    {
        let d = g.fish(this);
        d.field_0x24 = 1;
        d.field_0x28 = 0;
        d.field_0x3c = 0x172;
        d.field_0x44 = 0x5f;
        d.field_0x40 = 10;
        d.field_0x48 = 0x21c;
    }
    let r = crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng));
    g.fish(this).field_0x2c = match r % 3 {
        0 => 2.0,
        1 => f64::from_bits(0x3ffccccccccccccd),
        _ => f64::from_bits(0x3ff999999999999a),
    };
    let r = crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng));
    g.fish(this).field_0x50 = 0;
    g.go(this).offset_0x14 = (r % 200) as i32 + 400;
    let need = if g.wfa(app).offset_0x150 == 5 {
        crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32 % 9 + 0xc
    } else {
        (crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng)) % 3) as i32 + 4
    };
    g.fish(this).field_0x54 = need;
    let r = crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng));
    {
        let d = g.fish(this);
        d.field_0x5c = 0x28;
        d.field_0x60 = 0;
        d.field_0x64 = 0;
        d.field_0x68 = 0;
        d.field_0x6c = 0;
        d.field_0x70 = 0;
        d.field_0x74 = 0;
        d.field_0x78 = 0;
        d.field_0x7c = false;
        d.field_0x7d = false;
        d.offset_0x80 = 0;
        d.field_0xcc = 0;
        d.field_0x58 = (r % 10) as i32;
    }
    let r = crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng));
    let iv = FUN_004d71d0(g, this, (r % 200) as i32 + 0x96);
    g.fish(this).field_0xd0 = iv;
    g.fish(this).field_0xd4 = false;
    g.w(this).offset_0x1 = true;
    let board = g.wfa(app).offset_0x4;
    if board != NULL {
        let first = crate::game::board_level::FUN_00537b80(g, board);
        g.fish(this).field_0xd4 = first;
        let busy = !g.board(board).offset_0xc[crate::game::board_level::vec_index(0xb8)].is_empty()
            || !g.board(board).offset_0xc[crate::game::board_level::vec_index(0xf4)].is_empty();
        if busy {
            g.w(this).offset_0x1 = false;
        }
    }
    let d = g.fish(this);
    d.field_0xb4 = 0;
    d.offset_0xb8 = 0;
    d.offset_0xc4 = 0.0;
    d.offset_0xbc = 0;
    d.field_0xd5 = true;
}

/// port: 004d71d0 FUN_004d71d0
/// The coin interval: `param_1`, or 4320..4519 updates in the virtual tank.
pub fn FUN_004d71d0(g: &mut G, this: Ptr, param_1: i32) -> i32 {
    let app = g.go(this).offset_0x0;
    if g.wfa(app).offset_0x150 == 5 {
        return crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32 % 200 + 0x10e0;
    }
    param_1
}

/// port: 004ef040 Sexy::Fish::~Fish
pub fn dtor_Fish(g: &mut G, this: Ptr) {
    crate::game::game_object::dtor_GameObject(g, this);
}

/// port: 004f0cb0 Sexy::Fish::deleting_destructor
pub fn deleting_destructor(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_Fish(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// `FUN_004da780` on the fish's board: aliens are in the tank.
fn aliens_present(g: &mut G, this: Ptr) -> bool {
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    crate::game::board_update::FUN_004da780(g, board)
}

/// port: 004f6970 Sexy::Fish::vfunction23
/// `Update()`: swim toward the cursor in follow mode (`DAT_005e89d0`, small live fish);
/// else chase food (vfunction 83), follow pet 12 while aliens attack, or run the current swim
/// mode; then the shared timers, bubbles when rising fast, the tank bounds, the animation
/// (vfunction 90) and the move.
pub fn vfunction23(g: &mut G, this: Ptr) {
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    if board == NULL || g.board(board).field_0x8 {
        return;
    }
    crate::game::game_object::FUN_004f22c0(g, this);
    let rng = g.wfa(app).offset_0x84;
    if g.globals.DAT_005e89d0 != 0 && g.fish(this).field_0xd5 && g.fish(this).offset_0x4c < 2 {
        follow_cursor(g, this);
    } else if !vcall!(g, this, fish.vfunction83) {
        let pet12 = g.board(board).field_0xb8[12];
        if aliens_present(g, this) && pet12 != 0 && g.fish(this).field_0xd5 {
            follow_pet(g, this, board);
        } else {
            swim_mode(g, this, rng);
        }
    }
    tail(g, this, board, rng);
}

/// The follow-the-cursor block (`DAT_005e89d4`, `DAT_005e89d8`).
fn follow_cursor(g: &mut G, this: Ptr) {
    let (tx, ty) = (g.globals.DAT_005e89d4, g.globals.DAT_005e89d8);
    let f = g.fish(this);
    if f.offset_0xb8 == 0 {
        f.offset_0x14 = if 0.0 <= f.offset_0x14 { 0.5 } else { -0.5 };
    }
    if f.field_0xb4 == 0 {
        f.field_0x1c = 0.0;
    }
    let x = f.field_0x4;
    if ((tx + 0x32) as f64) < x {
        f.field_0x4 = x - 5.0;
    } else if x < (tx - 0x32) as f64 {
        f.field_0x4 = x + 5.0;
    } else if ((tx + 5) as f64) < x {
        f.field_0x4 = x - 3.0;
    } else if x < (tx - 5) as f64 {
        f.field_0x4 = x + 3.0;
    }
    let y = f.field_0xc;
    if ((ty + 0x32) as f64) < y {
        f.field_0xc = y - 4.0;
    } else if y < (ty - 0x32) as f64 {
        f.field_0xc = y + 4.0;
    } else if ((ty + 5) as f64) < y {
        f.field_0xc = y - 3.0;
    } else if y < (ty - 5) as f64 {
        f.field_0xc = y + 3.0;
    }
}

/// Swimming toward pet 12 (the one that protects fish) while aliens attack.
fn follow_pet(g: &mut G, this: Ptr, board: Ptr) {
    let pets = g.board(board).offset_0xc[crate::game::board_level::vec_index(0xb4)].clone();
    let mut it = pets.len();
    for (i, &o) in pets.iter().enumerate() {
        if crate::game::fish_type_pet::pet_id(g, o) == 0xc {
            it = i;
            break;
        }
    }
    let pet = *pets.get(it).expect("vector iterator not dereferencable");
    let (px, py) = (g.wc(pet).offset_0x2c, g.wc(pet).offset_0x30);
    let f = g.fish(this);
    let d = f.field_0x4 + 40.0;
    let vx = f.offset_0x14;
    if ((px + 0x32) as f64) < d {
        if -4.0 < vx {
            f.offset_0x14 = vx - 1.3;
        }
    } else if d < (px + 0x1e) as f64 {
        if vx < 4.0 {
            f.offset_0x14 = vx + 1.3;
        }
    } else if ((px + 0x2d) as f64) < d {
        if -4.0 < vx {
            f.offset_0x14 = vx - 0.2;
        }
    } else if d < (px + 0x23) as f64 {
        if vx < 4.0 {
            f.offset_0x14 = vx + 0.2;
        }
    } else if ((px + 0x28) as f64) < d {
        if -4.0 < vx {
            f.offset_0x14 = vx - 0.05;
        }
    } else if d < (px + 0x28) as f64 && vx < 4.0 {
        f.offset_0x14 = vx + 0.05;
    }
    let d = f.field_0xc + 40.0;
    let vy = f.field_0x1c;
    if ((py + 0x19) as f64) < d {
        if -3.0 < vy {
            f.field_0x1c = vy - 1.0;
        }
    } else if d < (py + 0xf) as f64 {
        if vy < 4.0 {
            f.field_0x1c = vy + 1.3;
        }
    } else if ((py + 0x14) as f64) < d {
        if -3.0 < vy {
            f.field_0x1c = vy - 0.5;
        }
    } else if d < (py + 0x14) as f64 && vy < 4.0 {
        f.field_0x1c = vy + 0.7;
    }
    let my = g.wc(this).offset_0x30;
    let f = g.fish(this);
    if my <= f.field_0x44 && f.field_0x1c < 0.0 {
        f.field_0x1c = 0.0;
    }
    if f.field_0x68 < 5 {
        f.field_0x68 += 1;
    }
}

/// The idle swim modes (field_0x58): 0 sink slowly toward vx 0, 1..2 rise drifting right /
/// left, 3..4 dive, 5..9 sweep across the tank, 0x7b drift then pick a mode.
fn swim_mode(g: &mut G, this: Ptr, rng: Ptr) {
    let ftol = crate::sexy::crt::ftol;
    let mode = g.fish(this).field_0x58;
    if mode == 0x7b {
        let f = g.fish(this);
        let a = f.offset_0x14.abs();
        f.field_0x68 = ftol(a) as i32;
        f.field_0x60 = 0;
        let t = f.field_0x5c;
        if 0x1e < t {
            if 2.0 < a {
                f.offset_0x14 *= 0.95;
            }
            if 2.0 < f.field_0x1c.abs() {
                f.field_0x1c *= 0.95;
            }
        }
        if 0x32 < t {
            let r = crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng));
            let f = g.fish(this);
            f.field_0x5c = 0;
            f.field_0x58 = (r % 9) as i32 + 1;
            let x = f.field_0x4;
            let flip = if ((f.field_0x48 - 0x14) as f64) < x {
                true
            } else if x < (f.field_0x40 + 0x14) as f64 {
                true
            } else {
                crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32 % 3 == 0
            };
            if flip {
                g.fish(this).offset_0x14 *= -1.0;
            }
        }
        return;
    }
    if 4 < mode {
        let f = g.fish(this);
        if f.field_0xb4 == 0 {
            f.field_0x1c = if 115.0 <= f.field_0xc { -0.5 } else { -0.1 };
        }
        if 0x28 <= f.field_0x5c {
            f.field_0x5c = 0;
            if f.field_0x24 == 1 {
                f.offset_0x14 += if 0.0 <= f.offset_0x14 { 1.0 } else { 2.0 };
                f.field_0x68 = ftol(f.offset_0x14.abs()) as i32;
                if 250.0 < f.field_0x4 {
                    f.field_0x24 = -1;
                    f.offset_0x14 -= 2.0;
                }
            } else if f.field_0x24 == -1 {
                f.offset_0x14 -= if 0.0 < f.offset_0x14 { 2.0 } else { 1.0 };
                f.field_0x68 = ftol(f.offset_0x14.abs()) as i32;
                if f.field_0x4 < 175.0 {
                    f.field_0x24 = 1;
                    f.offset_0x14 += 2.0;
                }
            }
        }
        return;
    }
    let f = g.fish(this);
    let rate = match mode {
        0 => {
            if f.field_0xb4 == 0 {
                f.field_0x1c = 0.5;
            }
            if 0x28 <= f.field_0x5c {
                f.field_0x5c = 0;
                let vx = f.offset_0x14;
                if 0.0 < vx {
                    if 0.5 < vx {
                        f.offset_0x14 = vx - 0.5;
                    }
                } else if vx < 0.0 && vx < -0.5 {
                    f.offset_0x14 = vx + 0.5;
                }
                f.field_0x68 = ftol(f.offset_0x14.abs()) as i32;
            }
            Some(0.25)
        }
        1 | 2 => {
            if f.field_0xb4 == 0 {
                f.field_0x1c = -0.5;
            }
            if 0x28 <= f.field_0x5c {
                f.field_0x5c = 0;
                let vx = f.offset_0x14;
                let target = if mode == 1 { 1.0 } else { -1.0 };
                if target < vx {
                    f.offset_0x14 = vx - 1.0;
                } else if vx < target {
                    f.offset_0x14 = vx + 1.0;
                }
                f.field_0x68 = ftol(f.offset_0x14.abs()) as i32;
            }
            Some(0.5)
        }
        _ => {
            // 3, 4: dive, steering vx toward -1 / +1 and vy toward 3.
            if 0x28 <= f.field_0x5c {
                f.field_0x5c = 0;
                let vx = f.offset_0x14;
                let target = if mode == 3 { -1.0 } else { 1.0 };
                if target < vx {
                    f.offset_0x14 = vx - 1.0;
                } else if vx < target {
                    f.offset_0x14 = vx + 1.0;
                }
                let vy = f.field_0x1c;
                if 3.0 < vy {
                    f.field_0x1c = vy - 1.0;
                } else if vy < 3.0 {
                    f.field_0x1c = vy + 1.0;
                }
                if 4 < f.field_0x68 {
                    f.field_0x68 -= 1;
                } else if f.field_0x1c < 4.0 {
                    f.field_0x68 += 1;
                }
            }
            if 240.0 < f.field_0xc {
                f.field_0x58 = 0;
            }
            None
        }
    };
    if let Some(r) = rate {
        let f = g.fish(this);
        f.field_0xc -= r / f.field_0x2c;
    }
}

/// Everything after the movement decision (`LAB_004f7523`).
fn tail(g: &mut G, this: Ptr, board: Ptr, rng: Ptr) {
    let ftol = crate::sexy::crt::ftol;
    {
        let f = g.fish(this);
        f.field_0x5c += 1;
        f.field_0x60 += 1;
    }
    if 0x14 < g.fish(this).field_0x60 {
        g.fish(this).field_0x60 = 0;
        if crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng)) % 10 == 0 {
            let r = crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng));
            g.fish(this).field_0x58 = (r % 9) as i32 + 1;
        }
    }
    if !crate::game::board_update::FUN_004da780(g, board) {
        vcall!(g, this, fish.vfunction82);
    }
    {
        let f = g.fish(this);
        if f.field_0xb4 != 0 {
            f.field_0xb4 -= 1;
            f.field_0x1c *= 0.9;
        }
        if f.offset_0xb8 != 0 {
            f.offset_0xb8 -= 1;
            f.offset_0xc4 *= 0.9;
            f.field_0x4 += f.offset_0xc4;
        }
        if 0 < f.offset_0xbc {
            f.offset_0xbc -= 1;
        }
        let vx = f.offset_0x14;
        if vx == 0.0 {
            f.field_0xc += 1.0 / f.field_0x2c;
        }
        if vx == 1.0 {
            f.field_0xc += 0.75 / f.field_0x2c;
        }
        if vx == 2.0 {
            f.field_0xc += 0.5 / f.field_0x2c;
        }
        if vx == 3.0 {
            f.field_0xc += 0.25 / f.field_0x2c;
        }
        let max_y = f.field_0x3c;
        if max_y <= 0x140 {
            f.field_0xc -= 0.25;
        }
        if (f.field_0x48 as f64) < f.field_0x4 {
            f.field_0x4 = f.field_0x48 as f64;
        }
        if f.field_0x4 < f.field_0x40 as f64 {
            f.field_0x4 = f.field_0x40 as f64;
        }
        if (max_y as f64) < f.field_0xc {
            f.field_0xc = max_y as f64;
        }
    }
    let damp = g.fish(this).field_0xb4;
    if damp < 1 || g.fish(this).field_0x1c <= 0.0 {
        let f = g.fish(this);
        if f.field_0xc < f.field_0x44 as f64 {
            f.field_0xc = f.field_0x44 as f64;
        }
    } else if 0x1e < damp {
        let n = if 0x28 < damp { 1 } else { 2 };
        if crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng)) % n == 0 {
            rising_bubbles(g, this, board, rng);
        }
    }
    {
        let f = g.fish(this);
        if ((f.field_0x48 - 5) as f64) < f.field_0x4 && 0.1 < f.offset_0x14 {
            f.offset_0x14 -= 0.1;
        }
        if f.field_0x4 < 15.0 && f.offset_0x14 < -0.1 {
            f.offset_0x14 += 0.1;
        }
    }
    vcall!(g, this, fish.vfunction90);
    let mut d = g.fish(this).field_0x2c;
    if g.go(this).offset_0x6e {
        d = if g.go(this).offset_0x70 != 0 { 0.8 } else { 0.3 };
    }
    let f = g.fish(this);
    f.field_0x4 += f.offset_0x14 / d;
    f.field_0xc += f.field_0x1c / d;
    let (x, y) = (ftol(f.field_0x4) as i32, ftol(f.field_0xc) as i32);
    vcall!(g, this, w.vfunction42, x, y);
}

/// Bubbles trailing a fish that is being pushed up fast.
fn rising_bubbles(g: &mut G, this: Ptr, board: Ptr, rng: Ptr) {
    let r = |g: &mut G| crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng));
    let (mx, my) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
    let bubble = crate::game::board_update::FUN_00538ac0;
    if g.fish(this).offset_0x4c == 0 {
        if r(g) & 1 != 0 {
            return;
        }
        let dx = 0xf - (r(g) % 0x1e) as i32;
        let dy = 0xf - (r(g) % 0x1e) as i32;
        bubble(g, board, mx + dx + 0x19, my + dy + 0x19);
    } else if g.go(this).offset_0x4 == 6 {
        let dx = 0x3c - (r(g) % 0x78) as i32;
        let dy = 0x3c - (r(g) % 0x78) as i32;
        bubble(g, board, mx + dx + 0x41, my + dy + 0x41);
        let dx = 0x28 - (r(g) % 0x50) as i32;
        let dy = 0x28 - (r(g) % 0x50) as i32;
        bubble(g, board, mx + dx + 0x41, my + dy + 0x41);
        let dx = 0x28 - (r(g) % 0x50) as i32;
        let dy = 0x28 - (r(g) % 0x50) as i32;
        bubble(g, board, mx + dx + 0x41, my + dy + 0x41);
    } else {
        let dx = 0x1e - (r(g) % 0x3c) as i32;
        let dy = 0x1e - (r(g) % 0x3c) as i32;
        bubble(g, board, mx + dx + 0x19, my + dy + 0x19);
        let dx = 0x14 - (r(g) % 0x28) as i32;
        let dy = 0x14 - (r(g) % 0x28) as i32;
        bubble(g, board, mx + dx + 0x19, my + dy + 0x19);
    }
}

/// port: 004d5930 FUN_004d5930
pub fn FUN_004d5930(g: &mut G, this: Ptr) {
    g.fish(this).field_0x74 -= 1;
}

/// port: 004d5940 FUN_004d5940
/// Rainbow colors: three hues 60 apart cycling with the update count (`HSLToRGB`), from a
/// random seed picked once.
pub fn FUN_004d5940(g: &mut G, this: Ptr) {
    if g.fish(this).offset_0x80 == 0 {
        let r = crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32;
        g.fish(this).offset_0x80 = r % 0xff + 1;
    }
    let h = g.wc(this).offset_0x24.wrapping_add(g.fish(this).offset_0x80).wrapping_abs();
    let app = g.globals.DAT_005eb6a4;
    let c = crate::sexy::sexy_app_base::FUN_00489530(g, app, h % 0xff, 200, 0x80);
    g.fish(this).field_0x94 = FUN_00433320(c);
    let c = crate::sexy::sexy_app_base::FUN_00489530(g, app, (h + 0x3c) % 0xff, 200, 0x80);
    g.fish(this).field_0x84 = FUN_00433320(c);
    let c = crate::sexy::sexy_app_base::FUN_00489530(g, app, (h + 0x78) % 0xff, 200, 0x80);
    g.fish(this).field_0xa4 = FUN_00433320(c);
}

/// port: 004d5aa0 FUN_004d5aa0
/// The fish's strip: turning, swimming or eating, normal or hungry.
pub fn FUN_004d5aa0(g: &mut G, this: Ptr, param_1: bool) -> Ptr {
    let id = if g.fish(this).field_0x70 != 0 {
        param_1 as i32 + 0xab
    } else if g.fish(this).field_0x74 < 1 && g.go(this).offset_0x80 < 0x65 {
        if param_1 { 0xb3 } else { 0xe2 }
    } else {
        (!param_1) as i32 * 4 + 0xad
    };
    crate::sexy::res::FUN_005016a0(g, id) as Ptr
}

/// port: 004d5f20 FUN_004d5f20
/// `CanEat(Food*)`: not taken and settled; plain pellets always, the star potion (kind 2)
/// only by a small-to-large hungry fish (anything in the virtual tank).
pub fn FUN_004d5f20(g: &mut G, this: Ptr, param_1: Ptr) -> bool {
    let f = g.food(param_1).clone();
    if f.offset_0x31 || f.offset_0x44 != 0 {
        return false;
    }
    if f.offset_0x40 != 2 {
        return f.offset_0x40 == 0;
    }
    let app = g.go(this).offset_0x0;
    if g.wfa(app).offset_0x150 == 5 {
        return true;
    }
    g.fish(this).offset_0x4c < 3 && g.go(this).offset_0x14 < 0x6c
}

/// port: 004d5a80 Sexy::Fish::vfunction72
/// Label offset: 10 for kinds 0x23/0x24.
pub fn vfunction72(g: &mut G, this: Ptr) -> i32 {
    let t = g.go(this).offset_0x4;
    if t != 0x23 && t != 0x24 {
        return 0;
    }
    10
}

/// port: 004df130 Sexy::Fish::vfunction73
/// Sell value: the base value (half price for a fresh fish only when +0x22a), x2 / x3 / x5
/// for medium / large / star fish.
pub fn vfunction73(g: &mut G, this: Ptr) -> i32 {
    let mut v = crate::game::game_object::vfunction73(g, this);
    if g.fish(this).offset_0xd6 && v < 0 {
        v = g.go(this).field_0x58 / 2;
    }
    if g.fish(this).field_0xd5 && 0 < v {
        match g.fish(this).offset_0x4c {
            1 => v *= 2,
            2 => return v * 3,
            4 => return v * 5,
            _ => {}
        }
    }
    v
}

/// port: 004f01d0 Sexy::Fish::vfunction76
pub fn vfunction76(g: &mut G, this: Ptr) {
    vcall!(g, this, fish.vfunction88, true);
}

/// port: 004df070 Sexy::Fish::vfunction79
/// Screensaver animation tick (+0x28, eaten sparkle cycle, rainbow).
pub fn vfunction79(g: &mut G, this: Ptr) {
    g.wc(this).offset_0x24 += 1;
    crate::game::game_object::FUN_004d6cf0(g, this);
    if g.fish(this).field_0x7d {
        FUN_004d5940(g, this);
    }
    let v = g.go(this).offset_0xa4 / 2;
    g.go(this).offset_0xa8 = v % 10;
}

/// port: 004f1400 Sexy::Fish::vfunction82
/// The coin timer: every +0x224 updates (35 in the +0x89cc mode) a medium-or-bigger fish
/// drops a coin of its size (a +0x22a fish: silver 4 / gold 6 / star 7); first-level hints.
pub fn vfunction82(g: &mut G, this: Ptr) {
    if g.fish(this).offset_0x4c < 1 && g.go(this).offset_0x24 < 0 {
        return;
    }
    g.fish(this).field_0xcc += 1;
    let limit = if g.globals.DAT_005e89cc { 0x23 } else { g.fish(this).field_0xd0 };
    let mut t = g.fish(this).field_0xcc;
    let due = crate::game::game_object::FUN_004d7100(g, this, &mut t, limit);
    g.fish(this).field_0xcc = t;
    if !due {
        return;
    }
    g.fish(this).field_0xcc = 0;
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    if crate::game::game_object::FUN_004d6bd0(g, this) {
        let kind = g.go(this).offset_0x9c;
        let size = g.fish(this).offset_0x4c;
        let coin = if kind == 0 {
            Some(7)
        } else if kind == 5 {
            if size < 2 { None } else { Some(7) }
        } else if g.fish(this).offset_0xd6 {
            Some(match size {
                1 => 4,
                2 => 6,
                4 => 7,
                _ => 2,
            })
        } else if size <= 0 {
            None
        } else {
            Some(size)
        };
        if let Some(c) = coin {
            let (x, y) = (g.wc(this).offset_0x2c + 5, g.wc(this).offset_0x30 + 10);
            crate::game::board_level::FUN_00544430(g, board, x, y, c, NULL, -1.0, 0);
        }
    }
    if crate::game::board_level::FUN_00537b80(g, board) {
        if g.board(board).ext_0x4b0[0x19] != 0 {
            crate::game::board_level::FUN_0053e950(g, board, b"Click on coins for extra money!", true, 0x1a);
            return;
        }
        if g.board(board).ext_0x4b0[5] != 0 {
            crate::game::board_level::FUN_0053e950(g, board, b"Click on coins for extra money!", true, 0x19);
            return;
        }
        crate::game::board_level::FUN_0053e950(g, board, b"Click on coins for extra money!", true, 5);
    }
}

/// port: 004f15c0 Sexy::Fish::vfunction83
/// Hunger: the tint, hunger ticking (not while aliens attack), starving (death unless a
/// tutorial fish, which gets the hint messages and dies at -500 instead); below 500 it
/// goes for food (vfunction 85) unless an alien other than kind 4 is present. True while
/// chasing food.
pub fn vfunction83(g: &mut G, this: Ptr) -> bool {
    use crate::game::board_level::FUN_0053e950;
    crate::game::game_object::FUN_004d6c50(g, this);
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    let aliens = crate::game::board_level::vec_index(0xb8);
    if g.board(board).offset_0xc[aliens].is_empty() && g.board(board).offset_0xc[crate::game::board_level::vec_index(0xf4)].is_empty() {
        crate::game::game_object::FUN_004d6ef0(g, this);
    }
    let tut = g.fish(this).field_0xd4;
    if crate::game::game_object::FUN_004d69d0(g, this) && !tut {
        vcall!(g, this, fish.vfunction89, true);
        return false;
    }
    let h = g.go(this).offset_0x14;
    if h == -1 && tut {
        let board = g.wfa(app).offset_0x4;
        if g.board(board).ext_0x4b0[0x17] != 0 {
            FUN_0053e950(g, board, b"Fish are hungry! Click to drop food!", true, 0x18);
        }
        if g.board(board).ext_0x4b0[8] != 0 {
            FUN_0053e950(g, board, b"Fish are hungry! Click to drop food!", true, 0x17);
        }
        FUN_0053e950(g, board, b"Fish are hungry! Click to drop food!", true, 8);
        return false;
    }
    if h == -200 && tut && g.board(board).ext_0x4b0[0x15] == 0 {
        FUN_0053e950(g, board, b"Fish are REALLY hungry! Click to feed!", true, 0x15);
        crate::game::board::FUN_00538230(g, board, 0x106, 3, 1.0);
        return false;
    }
    if h == -400 && tut {
        FUN_0053e950(g, board, b"Fish are about to die of hunger! Click on aquarium!", true, 0x16);
        return false;
    }
    if h <= -500 && tut {
        vcall!(g, this, fish.vfunction89, true);
        return false;
    }
    if 500 <= h {
        return false;
    }
    if crate::game::board_update::FUN_004da780(g, board) {
        let v = &g.board(board).offset_0xc[aliens];
        if v.is_empty() {
            return false;
        }
        let first = v[0];
        if g.alien(first).offset_0x98 == 4 {
            return false;
        }
    }
    vcall!(g, this, fish.vfunction85)
}

/// port: 004d5b20 Sexy::Fish::vfunction85
/// `SwimToFood()`: every third update steers toward the nearest food (faster when
/// hungrier than 300), then tries to eat (vfunction 87). True when there is food.
pub fn vfunction85(g: &mut G, this: Ptr) -> bool {
    let ftol = crate::sexy::crt::ftol;
    let food = vcall!(g, this, fish.vfunction86);
    let f = g.fish(this).clone();
    let fx = ftol(f.field_0x4 + 40.0) as i32;
    let mut fy = ftol(40.0 + f.field_0xc) as i32;
    if g.go(this).offset_0x68 != 0 {
        fy += 0x14;
    }
    if 2 < f.field_0x5c {
        if food == NULL {
            return false;
        }
        g.fish(this).field_0x5c = 0;
        let cx = g.wc(food).offset_0x34 / 2 + g.wc(food).offset_0x2c;
        let cy = g.wc(food).offset_0x38 / 2 + g.wc(food).offset_0x30;
        let hungry = g.go(this).offset_0x14 < 0x12d;
        let d = g.fish(this);
        let (lim, s1, s2, s3) = if hungry { (4.0, 1.3, 0.2, 0.05) } else { (3.0, 1.0, 0.1, 0.05) };
        let vx = d.offset_0x14;
        if cx + 8 < fx {
            if -lim < vx {
                d.offset_0x14 = vx - s1;
            }
        } else if fx < cx - 8 {
            if vx < lim {
                d.offset_0x14 = vx + s1;
            }
        } else if cx + 4 < fx {
            if -lim < vx {
                d.offset_0x14 = vx - s2;
            }
        } else if fx < cx - 4 {
            if vx < lim {
                d.offset_0x14 = vx + s2;
            }
        } else if cx < fx {
            if -lim < vx {
                d.offset_0x14 = vx - s3;
            }
        } else if fx < cx && vx < lim {
            d.offset_0x14 = vx + s3;
        }
        let vy = d.field_0x1c;
        if hungry {
            if cy + 6 < fy {
                if -3.0 < vy {
                    d.field_0x1c = vy - 1.0;
                }
            } else if fy < cy - 6 {
                if vy < 4.0 {
                    d.field_0x1c = vy + 1.3;
                }
            } else if cy < fy {
                if -3.0 < vy {
                    d.field_0x1c = vy - 0.5;
                }
            } else if fy < cy && vy < 4.0 {
                d.field_0x1c = vy + 0.7;
            }
        } else if cy + 6 < fy {
            if -2.0 < vy {
                d.field_0x1c = vy - 0.6;
            }
        } else if fy < cy - 6 {
            if vy < 3.0 {
                d.field_0x1c = vy + 1.0;
            }
        } else if cy < fy {
            if -2.0 < vy {
                d.field_0x1c = vy - 0.3;
            }
        } else if fy < cy && vy < 3.0 {
            d.field_0x1c = vy + 0.5;
        }
        if d.field_0x68 < 5 {
            d.field_0x68 += 1;
        }
    }
    if food != NULL {
        vcall!(g, this, fish.vfunction87);
    }
    food != NULL
}

/// port: 004ef050 Sexy::Fish::vfunction86
/// `FindFood()`: the nearest edible food (by the center offsets the original uses); within
/// 100 pixels the mouth opens (150 updates) and +0xf8 is set to 100.
pub fn vfunction86(g: &mut G, this: Ptr) -> Ptr {
    let ftol = crate::sexy::crt::ftol;
    if g.go(this).offset_0x68 != 0 {
        let (x, y, w, h) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30, g.wc(this).offset_0x34, g.wc(this).offset_0x38);
        return crate::game::game_object::FUN_004ea010(g, this, w / 2 + x, h / 2 + y + 0x14);
    }
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    let mut best = 100000000;
    let mut found = NULL;
    for o in g.board(board).offset_0xc[crate::game::board_level::vec_index(0xac)].clone() {
        let (fx, fy) = (g.wc(o).offset_0x2c + 0x14, g.wc(o).offset_0x30 + 0x14);
        let f = g.fish(this).clone();
        let dx = ftol(fx as f64 - (f.field_0x4 + 40.0)) as i32;
        let dy = ftol(fy as f64 - (f.field_0xc + 40.0)) as i32;
        let d = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx));
        if FUN_004d5f20(g, this, o) && d < best {
            best = d;
            found = o;
        }
    }
    if best < 10000 {
        crate::game::game_object::FUN_004d6b00(g, this, 0x96);
        g.go(this).offset_0x70 = 100;
    }
    found
}

/// port: 004ef1b0 Sexy::Fish::vfunction87
/// `TryEat()`: eats food under its mouth (vfunction 78, then the food goes); food close by
/// starts the open-mouth animation.
pub fn vfunction87(g: &mut G, this: Ptr) {
    if g.go(this).offset_0x68 != 0 {
        let (x, y, w, h) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30, g.wc(this).offset_0x34, g.wc(this).offset_0x38);
        let r = crate::game::game_object::FUN_004ea2a0(g, this, w / 2 + x, h / 2 + y + 0x14);
        if g.fish(this).field_0x74 == 0 {
            if r == 1 {
                crate::game::game_object::FUN_004d6cc0(g, this);
                g.fish(this).field_0x74 = 8;
            } else if r == 2 {
                crate::game::game_object::FUN_004d6cc0(g, this);
                g.fish(this).field_0x74 = 0x14;
            }
        }
        return;
    }
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    for o in g.board(board).offset_0xc[crate::game::board_level::vec_index(0xac)].clone() {
        let (ox, oy) = (g.wc(o).offset_0x2c, g.wc(o).offset_0x30);
        let xx = g.fish(this).field_0x4 + 40.0;
        if xx < (ox + 0x23) as f64 && ((ox + 5) as f64) < xx {
            let yy = g.fish(this).field_0xc + 40.0;
            if yy < (oy + 0x23) as f64 && (oy as f64) < yy && FUN_004d5f20(g, this, o) {
                vcall!(g, this, go.vfunction78, o as i32);
                crate::game::larva::vfunction76(g, o);
                if g.fish(this).field_0x74 == 0 {
                    crate::game::game_object::FUN_004d6cc0(g, this);
                    g.fish(this).field_0x74 = 8;
                }
                return;
            }
        }
        if g.fish(this).field_0x74 == 0 && xx < (ox + 0x32) as f64 && ((ox - 10) as f64) < xx {
            let yy = 40.0 + g.fish(this).field_0xc;
            if yy < (oy + 0x28) as f64 && ((oy - 5) as f64) < yy && FUN_004d5f20(g, this, o) {
                crate::game::game_object::FUN_004d6cc0(g, this);
                g.fish(this).field_0x74 = 0x14;
            }
        }
    }
}

/// port: 004df0b0 Sexy::Fish::vfunction88
/// `Remove(bool withShadow)`: drops what it carries, leaves the widget manager and the
/// board (safe-deleted), optionally removes its shadow, counts a lost fish (+0x488).
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
    g.board(board).ext_0x460[(0x488 - 0x460) / 4] += 1;
}

/// port: 004d5f90 Sexy::Fish::vfunction89
/// `Die(bool sound)`: the death sound, removal (keeping the shadow) and a dead fish where
/// it was, facing the way it swam.
pub fn vfunction89(g: &mut G, this: Ptr, param_1: bool) {
    let ftol = crate::sexy::crt::ftol;
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    if param_1 {
        let t = g.go(this).offset_0x4;
        crate::game::board_level::FUN_00538610(g, board, t);
    }
    vcall!(g, this, fish.vfunction88, false);
    let f = g.fish(this).clone();
    let right = 0.0 < f.offset_0x14;
    let shadow = g.go(this).offset_0xc;
    let (y, x) = (ftol(f.field_0xc) as i32, ftol(f.field_0x4) as i32);
    let board = g.wfa(app).offset_0x4;
    crate::game::board_level::FUN_005441f0(g, board, x, y, f.offset_0x14, f.field_0x1c, f.field_0x2c, f.offset_0x4c, right, shadow);
}

/// port: 004de840 Sexy::Fish::vfunction90
/// `Animate()`: rainbow, drop-in fade, turn and eat counters, the cel (turn, eat or swim
/// cycle), the grow flash, the facing, the open mouth cel, the sparkle cycle.
pub fn vfunction90(g: &mut G, this: Ptr) {
    if g.fish(this).field_0x7d {
        FUN_004d5940(g, this);
    }
    if g.go(this).offset_0x74 {
        crate::game::game_object::FUN_004d6c90(g, this);
    }
    let period = 0x14;
    {
        let f = g.fish(this);
        if 0.0 < f.offset_0x34 && f.offset_0x14 < 0.0 {
            f.field_0x70 = 0x14;
        } else if f.offset_0x34 < 0.0 && 0.0 < f.offset_0x14 {
            f.field_0x70 = -0x14;
        }
    }
    let turn = g.fish(this).field_0x70;
    if turn != 0 {
        let t = if 0 < turn { turn - 1 } else { turn + 1 };
        g.fish(this).field_0x70 = t;
        if g.fish(this).field_0x74 != 0 {
            FUN_004d5930(g, this);
        }
    }
    let turn = g.fish(this).field_0x70;
    if turn == 0 {
        if 0 < g.fish(this).field_0x74 {
            FUN_004d5930(g, this);
            let f = g.fish(this);
            f.field_0x6c = 9 - f.field_0x74 / 2;
        } else {
            let f = g.fish(this);
            f.field_0x64 += if f.field_0x68 <= 1 { 1 } else { 2 };
            if period <= f.field_0x64 {
                f.field_0x64 = 0;
            }
            f.field_0x6c = f.field_0x64 / 2;
        }
    } else if 0 < turn {
        g.fish(this).field_0x6c = 9 - turn / 2;
    } else {
        g.fish(this).field_0x6c = turn / 2 + 9;
    }
    {
        let f = g.fish(this);
        if 0 < f.field_0x78 {
            f.field_0x78 -= 1;
        }
        if f.offset_0x14 != f.offset_0x34 && f.offset_0x14 != 0.0 && f.offset_0x34 != 0.0 {
            f.offset_0x34 = f.offset_0x14;
        }
    }
    if 100 < g.go(this).offset_0x80 && g.fish(this).field_0x70 == 0 {
        g.fish(this).field_0x6c = 4;
    }
    if g.go(this).offset_0x10 != NULL {
        crate::game::game_object::FUN_004d7020(g, this);
    }
}

/// port: 004d5650 Sexy::Fish::vfunction91
/// `SetColors(int seed, bool rainbow)`: virtual-tank coloring from the fish color tables
/// (12 schemes for kind 5, 30 otherwise), each channel brightened by 5.
pub fn vfunction91(g: &mut G, this: Ptr, param_1: i32, param_2: bool) {
    let c = |v: i32| if v + 5 < 0x100 { v + 5 } else { 0xff };
    let kind = g.go(this).offset_0x4;
    {
        let f = g.fish(this);
        f.field_0x7c = true;
        f.field_0x7d = param_2;
    }
    if kind == 5 {
        let k = (param_1 % 0xc).abs() as usize;
        let f = g.fish(this);
        f.field_0x84 = CRect(c(DAT_005df3d8[k]), c(DAT_005df408[k]), c(DAT_005df438[k]), 0xff);
        f.field_0x94 = CRect(c(DAT_005df4f8[k]), c(DAT_005df528[k]), c(DAT_005df558[k]), 0xff);
        f.field_0xa4 = CRect(c(DAT_005df468[k]), c(DAT_005df498[k]), c(DAT_005df4c8[k]), 0xff);
        return;
    }
    let k = (param_1 % 0x1e).abs() as usize;
    let f = g.fish(this);
    f.field_0x84 = CRect(c(DAT_005df108[k]), c(DAT_005df180[k]), c(DAT_005df1f8[k]), 0xff);
    f.field_0x94 = CRect(c(DAT_005df270[k]), c(DAT_005df2e8[k]), c(DAT_005df360[k]), 0xff);
}

/// port: 004e4a90 Sexy::Fish::vfunction27
/// `Draw(Graphics*)`: hidden under the cursor in follow mode (its shadow too); otherwise the
/// body mirrored by facing (or turn direction), then the name label.
pub fn vfunction27(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    let ftol = crate::sexy::crt::ftol;
    crate::game::food::FUN_004d6ae0(g, this);
    let shadow = g.go(this).offset_0xc;
    if g.globals.DAT_005e89d0 != 0 {
        let (tx, ty) = (g.globals.DAT_005e89d4, g.globals.DAT_005e89d8);
        let f = g.fish(this).clone();
        if f.field_0x4 < (tx + 8) as f64
            && ((tx - 8) as f64) < f.field_0x4
            && f.field_0xc < (ty + 8) as f64
            && ((ty - 8) as f64) < f.field_0xc
            && f.field_0xd5
            && f.offset_0x4c < 2
        {
            if shadow != NULL {
                g.shadow(shadow).offset_0x14 = 0.0;
            }
            return;
        }
    }
    if shadow != NULL {
        g.shadow(shadow).offset_0x14 = 1.0;
    }
    let m1 = g.go(this).offset_0x6d;
    let m0 = !m1;
    let f = g.fish(this).clone();
    let turn = f.field_0x70;
    let draw = if turn == 0 {
        let vx = f.offset_0x14;
        if vx < 0.0 {
            Some(m1)
        } else {
            let e = ftol(vx);
            if e == 0 && f.offset_0x34 < 0.0 {
                Some(m1)
            } else if 0.0 < vx {
                Some(m0)
            } else if e != 0 {
                None
            } else if 0.0 < f.offset_0x34 {
                Some(m0)
            } else {
                None
            }
        }
    } else if 0 < turn {
        Some(m0)
    } else {
        Some(m1)
    };
    if let Some(m) = draw {
        vcall!(g, this, fish.vfunction84, gfx, m);
    }
    if g.go(this).field_0x28.len() != 0 {
        crate::game::game_object::FUN_004e1400(g, this, gfx, false);
    }
}

/// port: 004dea00 Sexy::Fish::vfunction84
/// `DrawBody(Graphics*, bool mirror)`: the size row of the strip at the animation cel;
/// alternate fish art with profile flag 4; the drop-in portal; the grow flash (scaled up);
/// the king's translucent double draw; virtual-tank tinted layers; the hungry tint over
/// it; carried sparkle; the pet-13 hunger marker.
pub fn vfunction84(g: &mut G, this: Ptr, gfx: &mut Graphics, param_1: bool) {
    use crate::game::game_object::{FUN_004d6db0, FUN_004d6f30};
    use crate::game::shadow::FUN_005008f0;
    use crate::sexy::graphics::{FUN_004558c0, FUN_004558e0, FUN_00455900, FUN_00456950, FUN_004560a0};
    let ftol = crate::sexy::crt::ftol;
    let size = g.fish(this).offset_0x4c;
    let row = match size {
        s if s <= 2 => s,
        3 => 2,
        4 => 3,
        _ => param_1 as i32,
    };
    if g.globals.DAT_005e89ce {
        let hungry = FUN_004d6f30(g, this);
        let col = if hungry { 6 } else { 9 };
        let src = Rect::new(col * 0x50, row * 0x50, 0x50, 0x50);
        let img = g.res.DAT_005e8aa8;
        FUN_004560a0(gfx, g, img, 0, 0, &src, param_1);
        if g.go(this).offset_0x10 != NULL {
            crate::game::game_object::FUN_004d7040(g, this, gfx, 0, 0);
        }
        if crate::game::game_object::FUN_004d6980(g, this, 500) {
            let img = g.res.DAT_005e8bc4;
            FUN_00456950(gfx, g, img, 0, -5, 2);
        }
        return;
    }
    let hungry = FUN_004d6f30(g, this);
    let img = FUN_004d5aa0(g, this, hungry);
    let src = Rect::new(g.fish(this).field_0x6c * 0x50, row * 0x50, 0x50, 0x50);
    let mut dest = Rect::new(0, 0, 0x50, 0x50);
    if g.go(this).offset_0x74 && g.fish(this).field_0x78 == 0 && crate::game::game_object::FUN_004d6e70(g, this, gfx, img, &src, param_1) {
        return;
    }
    let flash = g.fish(this).field_0x78;
    let king = |g: &mut G, gfx: &mut Graphics, dest: &Rect| {
        FUN_004d6db0(g, this, gfx, CRect(0xff, 0xff, 0xff, 0x9b));
        FUN_005008f0(g, gfx, img, dest, &src, param_1);
        if !hungry || g.go(this).offset_0x18 != 0 {
            FUN_004558c0(gfx, 1);
            let t = g.go(this).offset_0x18;
            let a = if t == 0 { 200 } else { ((5 - t) * 200) / 5 };
            FUN_004d6db0(g, this, gfx, CRect(0xff, 0xff, 0xff, a));
            FUN_005008f0(g, gfx, img, dest, &src, param_1);
            FUN_004558c0(gfx, 0);
        }
        FUN_004558e0(gfx, false);
    };
    let mut after_tint = true;
    if 0 < flash && size == 3 {
        king(g, gfx, &dest);
        after_tint = false;
    } else {
        if 0 < flash {
            let k = if 3 < flash {
                ((10 - flash) as f64 * f64::from_bits(0x3fe6666660000000) / 7.0 + 0.5) as f32
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
        if g.fish(this).offset_0x4c == 3 {
            king(g, gfx, &dest);
            after_tint = false;
        } else if g.fish(this).field_0x7c && !hungry {
            let id = if g.fish(this).field_0x70 != 0 {
                0xae
            } else if 0 < g.fish(this).field_0x74 || 100 < g.go(this).offset_0x80 {
                0xb5
            } else {
                0xb8
            };
            FUN_004d6db0(g, this, gfx, CRect(0xff, 0xff, 0xff, 0xff));
            let i0 = crate::sexy::res::FUN_005016a0(g, id) as Ptr;
            FUN_005008f0(g, gfx, i0, &dest, &src, param_1);
            FUN_004558c0(gfx, 1);
            let c2 = g.fish(this).field_0x94;
            FUN_004d6db0(g, this, gfx, c2);
            let i1 = crate::sexy::res::FUN_005016a0(g, id + 1) as Ptr;
            FUN_005008f0(g, gfx, i1, &dest, &src, param_1);
            let c1 = g.fish(this).field_0x84;
            FUN_004d6db0(g, this, gfx, c1);
            let i2 = crate::sexy::res::FUN_005016a0(g, id + 2) as Ptr;
            FUN_005008f0(g, gfx, i2, &dest, &src, param_1);
            FUN_004558c0(gfx, 0);
        } else {
            FUN_004d6db0(g, this, gfx, Color::WHITE);
            FUN_005008f0(g, gfx, img, &dest, &src, param_1);
        }
    }
    if after_tint {
        FUN_004558e0(gfx, false);
        let t = g.go(this).offset_0x18;
        if t != 0 {
            let himg = FUN_004d5aa0(g, this, true);
            FUN_004d6db0(g, this, gfx, CRect(0xff, 0xff, 0xff, (t * 0xff) / 5));
            FUN_005008f0(g, gfx, himg, &dest, &src, param_1);
            FUN_004558e0(gfx, false);
        }
    }
    if g.go(this).offset_0x10 != NULL {
        crate::game::game_object::FUN_004d7040(g, this, gfx, 0, 0);
    }
    if crate::game::game_object::FUN_004d6980(g, this, 500) {
        let img = g.res.DAT_005e8bc4;
        FUN_00456950(gfx, g, img, 0, -5, 2);
    }
}

/// port: 004e4c10 Sexy::Fish::vfunction55
/// `MouseDown(x, y, clicks)`: a right click passes through to the board; a left click on a
/// fish in the tank drops food there (when nothing else is under the cursor and the money
/// allows), unless pets or aliens are around.
pub fn vfunction55(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32) {
    let ftol = crate::sexy::crt::ftol;
    if param_3 < 0 {
        let app = g.go(this).offset_0x0;
        let board = g.wfa(app).offset_0x4;
        let (x, y) = (g.wc(this).offset_0x2c + param_1, g.wc(this).offset_0x30 + param_2);
        crate::game::board_level::FUN_0053bfb0(g, board, x, y);
        return;
    }
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    let vi = crate::game::board_level::vec_index;
    let b = g.board(board);
    if !b.offset_0xc[vi(0xb8)].is_empty() || !b.offset_0xc[vi(0xf4)].is_empty() || !b.offset_0xc[vi(0xdc)].is_empty() {
        return;
    }
    if !g.fish(this).field_0xd5 {
        return;
    }
    let f = g.fish(this).clone();
    let dx = param_1 as f64 + f.field_0x4;
    if !(dx < 587.0 && 30.0 < dx) {
        return;
    }
    let dy = param_2 as f64 + f.field_0xc;
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
    let fy = ftol((f.field_0xc + param_2 as f64) - 10.0) as i32;
    let fx = ftol((f.field_0x4 + param_1 as f64) - 10.0) as i32;
    crate::game::board_level::FUN_00543280(g, board, fx, fy, 0, false, off, -1);
    g.board(board).ext_0x4ec = true;
    let t = crate::game::board::FUN_00537b60(g, board);
    g.board(board).field_0x334 = t;
}

/// `DAT_005df108` .. `DAT_005df360`: the 30 virtual-tank fish color schemes (r, g, b of the
/// two tinted layers) and `DAT_005df3d8` .. `DAT_005df558` the 12 schemes of kind 5.
pub const DAT_005df108: [i32; 30] = [125, 255, 135, 175, 195, 255, 255, 60, 255, 190, 190, 145, 240, 255, 210, 175, 255, 65, 240, 240, 255, 255, 175, 225, 130, 160, 205, 190, 240, 210];
pub const DAT_005df180: [i32; 30] = [210, 240, 250, 235, 140, 255, 255, 60, 65, 60, 190, 145, 145, 255, 215, 240, 180, 70, 240, 225, 230, 40, 215, 175, 165, 230, 50, 140, 250, 250];
pub const DAT_005df1f8: [i32; 30] = [255, 60, 245, 135, 90, 85, 255, 60, 65, 255, 190, 175, 235, 40, 175, 175, 110, 160, 240, 125, 0, 0, 110, 95, 145, 50, 180, 250, 240, 50];
pub const DAT_005df270: [i32; 30] = [50, 45, 255, 50, 65, 250, 230, 0, 75, 255, 250, 30, 110, 210, 110, 30, 210, 240, 0, 240, 0, 255, 15, 65, 125, 145, 255, 250, 80, 110];
pub const DAT_005df2e8: [i32; 30] = [110, 95, 0, 115, 120, 155, 130, 0, 75, 0, 250, 25, 10, 10, 95, 80, 10, 240, 5, 5, 125, 225, 30, 55, 15, 0, 145, 245, 250, 210];
pub const DAT_005df360: [i32; 30] = [210, 195, 125, 210, 65, 0, 250, 0, 75, 125, 250, 225, 225, 0, 210, 125, 0, 240, 130, 240, 0, 0, 15, 190, 15, 210, 220, 175, 145, 0];
pub const DAT_005df3d8: [i32; 12] = [165, 40, 30, 255, 145, 40, 150, 200, 130, 220, 185, 40];
pub const DAT_005df408: [i32; 12] = [210, 40, 30, 155, 210, 40, 125, 125, 190, 190, 100, 40];
pub const DAT_005df438: [i32; 12] = [215, 40, 30, 0, 145, 40, 85, 215, 85, 140, 240, 40];
pub const DAT_005df468: [i32; 12] = [40, 50, 75, 250, 35, 255, 90, 170, 245, 140, 40, 255];
pub const DAT_005df498: [i32; 12] = [125, 150, 75, 255, 150, 255, 70, 255, 245, 120, 40, 110];
pub const DAT_005df4c8: [i32; 12] = [130, 225, 75, 0, 30, 255, 40, 255, 140, 80, 40, 0];
pub const DAT_005df4f8: [i32; 12] = [75, 20, 40, 255, 20, 75, 160, 150, 245, 180, 105, 75];
pub const DAT_005df528: [i32; 12] = [75, 70, 40, 65, 75, 70, 65, 80, 170, 170, 15, 75];
pub const DAT_005df558: [i32; 12] = [75, 110, 40, 0, 120, 70, 25, 175, 0, 155, 245, 75];

/// port: 004f1ba0 Sexy::Fish::vfunction78
/// `Eat(Food*)`: the eat sound; fed (hunger by food grade, capped); food counts toward
/// growing (by grade, more outside the virtual tank). A star potion makes a small-to-large
/// guppy into a pet (outside the virtual tank) or shows "YUK!"; grade 3 (the potion pill)
/// kills small fish with a burst and hints, crowns a large one king. Growing to medium /
/// large unlocks shop items; 15x the need (8x in the virtual tank) makes a star fish.
pub fn vfunction78(g: &mut G, this: Ptr, param_1: i32) {
    use crate::game::board_level::FUN_0053e950;
    let food = param_1 as Ptr;
    let hungry0 = crate::game::game_object::FUN_004d6f30(g, this);
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    let high = g.go(this).offset_0x7c;
    crate::game::board_level::FUN_005384c0(g, board, high);
    let star = g.food(food).offset_0x40 == 2;
    crate::game::game_object::FUN_004d6a30(g, this, star);
    let mode = g.wfa(app).offset_0x150;
    let mut count = true;
    let mut grade_switch = true;
    if mode == 5 {
        if g.go(this).offset_0x24 < 0 || star {
            count = false;
            grade_switch = !star;
        } else {
            count = crate::game::game_object::FUN_004d6f90(g, this);
            if g.go(this).offset_0x9c == 0 {
                count = false;
            }
        }
    } else if star {
        grade_switch = false;
    }
    if !grade_switch {
        // A star potion.
        if mode == 5 {
            if 0x1e < g.go(this).offset_0x14 {
                g.go(this).offset_0x14 = 0x1e;
            }
            let (mx, my, w) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30, g.wc(this).offset_0x34);
            crate::game::board_level::FUN_005444e0(g, board, w / 2 + mx - 0x14, my - 5, 2, b"YUK!");
        } else if g.fish(this).offset_0x4c <= 2 {
            vcall!(g, this, fish.vfunction88, true);
            let (mx, my) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
            let pet = crate::game::board_level::FUN_00544a90(g, board, 0x15, mx, my, false, true);
            if pet != NULL {
                g.fish_type_pet(pet).offset_0x14 = 1;
            }
            if !crate::game::board_update::FUN_005392a0(g, board) {
                crate::game::board_level::FUN_00546d70(g, board);
            }
            return;
        }
    } else {
        match g.food(food).offset_0x34 {
            0 => {
                let go = g.go(this);
                go.offset_0x14 += 500;
                if g.fish(this).field_0xd4 {
                    g.go(this).offset_0x14 += 200;
                }
                let go = g.go(this);
                if 800 < go.offset_0x14 {
                    go.offset_0x14 = 800;
                }
                if count {
                    g.fish(this).field_0x50 += 1;
                }
            }
            1 => {
                let go = g.go(this);
                go.offset_0x14 += 700;
                if 1000 < go.offset_0x14 {
                    go.offset_0x14 = 1000;
                }
                if count {
                    g.fish(this).field_0x50 += (mode != 5) as i32 + 1;
                }
            }
            3 => {
                let size = g.fish(this).offset_0x4c;
                if size == 0 || size == 1 {
                    crate::game::board::FUN_00538230(g, board, 0x11d, 3, 1.0);
                    let rng = g.wfa(app).offset_0x84;
                    let mut n = (crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng)) % 3) as i32 + 2;
                    while 0 < n {
                        let a = (crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng)) % 3) as i32 + 3;
                        let my = g.wc(this).offset_0x30;
                        let b = (crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng)) % 0x1e) as i32 + my - 10;
                        let mx = g.wc(this).offset_0x2c;
                        let c = (crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng)) % 0x1e) as i32 + mx - 10;
                        crate::game::board_level::FUN_005436f0(g, board, c, b, a);
                        n -= 1;
                    }
                    let profile = g.wfa(app).offset_0x18c;
                    let done = g.profile(profile).field_0x59;
                    let (tank, level) = (g.board(board).field_0x33c, g.board(board).field_0x340);
                    if tank == 2 && level < 3 && !done {
                        if g.board(board).ext_0x4b0[0x20] != 0 {
                            FUN_0053e950(g, board, b"Hint: Feed star potions to BIG guppies!", false, 0x21);
                        }
                        if g.board(board).ext_0x4b0[0x1f] != 0 {
                            FUN_0053e950(g, board, b"Hint: Star potions are for big guppies only!", false, 0x20);
                        }
                        if g.board(board).ext_0x4b0[0x1e] != 0 {
                            FUN_0053e950(g, board, b"Hint: Only certain fish can handle star potions!", false, 0x1f);
                        }
                        FUN_0053e950(g, board, b"Warning! Use star potions carefully!", false, 0x1e);
                    }
                    g.board(board).ext_0x460[(0x49c - 0x460) / 4] += 1;
                    vcall!(g, this, fish.vfunction89, true);
                } else {
                    g.go(this).offset_0x14 += 0x44c;
                    if size == 2 {
                        if 0x578 < g.go(this).offset_0x14 {
                            g.go(this).offset_0x14 = 0x578;
                        }
                        g.fish(this).offset_0x4c = 3;
                        crate::game::board::FUN_00538230(g, board, 0x122, 3, 1.0);
                    } else if 0x578 < g.go(this).offset_0x14 {
                        g.go(this).offset_0x14 = 0x578;
                    }
                }
            }
            _ => {
                if mode == 5 {
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
                    g.fish(this).field_0x50 += (mode != 5) as i32 + 2;
                }
            }
        }
    }
    let f = g.fish(this).clone();
    if f.field_0x54 <= f.field_0x50 {
        if f.offset_0x4c < 2 {
            g.fish(this).offset_0x4c += 1;
            if g.wfa(app).offset_0x150 != 5 {
                FUN_004f1840(g, this);
            }
            let d = g.fish(this);
            d.field_0x50 = 0;
            d.field_0x78 = 10;
            crate::game::board::FUN_00538230(g, board, 0x122, 3, 1.0);
        }
        let k = if g.wfa(app).offset_0x150 != 5 { 7 } else { 0 } + 8;
        let f = g.fish(this).clone();
        if k * f.field_0x54 <= f.field_0x50 && 2 <= f.offset_0x4c && f.offset_0x4c != 4 && !f.field_0x7c {
            g.fish(this).offset_0x4c = 4;
            crate::game::board::FUN_00538230(g, board, 0x115, 3, 1.0);
        }
    }
    crate::game::game_object::FUN_004e13d0(g, this, hungry0);
}

/// port: 004f1840 FUN_004f1840
/// The shop unlocks (and first-level hints) for a fish that just grew to medium or large.
pub fn FUN_004f1840(g: &mut G, this: Ptr) {
    use crate::game::board_level::{FUN_005409b0, FUN_0053e950};
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    let (level, tank) = (g.board(board).field_0x340, g.board(board).field_0x33c);
    let profile = g.wfa(app).offset_0x18c;
    let done = g.profile(profile).field_0x59;
    let size = g.fish(this).offset_0x4c;
    let msg = g.board(board).offset_0x8c;
    if size == 1 {
        if !g.board(board).field_0x318[0] {
            FUN_005409b0(g, board, 0, true);
            if tank == 1 && level == 1 && !done {
                g.board(board).ext_0x4b0[2] = 1;
                if g.message_widget(msg).offset_0x48 <= 0 {
                    FUN_0053e950(g, board, b"Your fish has grown! Good work!", false, 9);
                }
            }
        }
    } else if size == 2 {
        if g.wfa(app).offset_0x150 == 1 {
            FUN_005409b0(g, board, 0xb, true);
        }
        if tank == 1 {
            if level == 1 {
                if !g.board(board).field_0x318[11] {
                    FUN_005409b0(g, board, 0xb, true);
                    g.board(board).ext_0x4b0[2] = 0;
                    g.board(board).ext_0x4b0[0x28] = 1;
                    if !done && g.message_widget(msg).offset_0x48 <= 0 {
                        FUN_0053e950(g, board, b"Buy 3 egg pieces to complete level!", false, 7);
                    }
                }
            } else if level == 2 {
                if !g.board(board).field_0x318[2] {
                    g.board(board).ext_0x4b0[0xf] = 1;
                    if !done {
                        FUN_0053e950(g, board, b"Upgrade Food Quality to make food more nourishing!", false, 0xb);
                    }
                    FUN_005409b0(g, board, 2, true);
                }
            } else {
                FUN_005409b0(g, board, 2, true);
                FUN_005409b0(g, board, 3, true);
                FUN_005409b0(g, board, 4, true);
            }
        } else if tank == 2 {
            FUN_005409b0(g, board, 2, true);
            FUN_005409b0(g, board, 3, true);
            FUN_005409b0(g, board, 5, true);
            if level == 1 {
                FUN_005409b0(g, board, 0xb, true);
            } else {
                FUN_005409b0(g, board, 6, true);
            }
        } else if tank == 3 {
            FUN_005409b0(g, board, 2, true);
            FUN_005409b0(g, board, 3, true);
            FUN_005409b0(g, board, 7, true);
        } else if tank == 4 {
            FUN_005409b0(g, board, 2, true);
            FUN_005409b0(g, board, 3, true);
            FUN_005409b0(g, board, 4, true);
        }
    }
}

/// port: 004d6040 FUN_004d6040
/// `Startle(x, y)` (virtual tank): a grown fish (stage 3+) swimming normally within ~90 of
/// the click darts away from it (up or down away from it, a quarter of the time toward),
/// at x speed 4 or 6 and y speed 1 or 3, turning if needed; its idle counters restart.
pub fn FUN_004d6040(g: &mut G, this: Ptr, param_1: i32, param_2: i32) {
    if g.fish(this).field_0x70 != 0 || g.fish(this).field_0x74 != 0 || g.fish(this).field_0x5c < 3 {
        return;
    }
    if 100 < crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32 % 0x4b {
        return;
    }
    let x0 = g.wc(this).offset_0x2c;
    let dx = param_1 - (x0 + 0x28);
    let cy = g.wc(this).offset_0x30 + 0x28;
    let dy = param_2 - cy;
    if 0x1fa3 < dx * dx + dy * dy {
        return;
    }
    let w = g.wc(this).offset_0x34;
    {
        let f = g.fish(this);
        f.field_0x58 = 0x7b;
        f.field_0x5c = 0;
        f.field_0x60 = 0;
    }
    let vx = g.fish(this).offset_0x14;
    let sx: i32 = if vx < 0.0 {
        if param_1 < x0 + 0x14 { 1 } else { -1 }
    } else if 0.0 < vx && w - 0x14 + x0 < param_1 {
        -1
    } else {
        1
    };
    let mut sy: i32 = if param_2 < cy { 1 } else { -1 };
    if (crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32) % 100 < 0x1a {
        sy = -sy;
    }
    match (crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32) % 2 {
        0 => g.fish(this).offset_0x14 = 4.0,
        1 => g.fish(this).offset_0x14 = 6.0,
        _ => {}
    }
    match (crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32) % 2 {
        0 => g.fish(this).field_0x1c = 1.0,
        1 => g.fish(this).field_0x1c = 3.0,
        _ => {}
    }
    let f = g.fish(this);
    let vx = sx as f64 * f.offset_0x14;
    f.offset_0x14 = vx;
    f.field_0x1c = sy as f64 * f.field_0x1c;
    if f.offset_0x34 <= 0.0 || 0.0 <= vx {
        if f.offset_0x34 < 0.0 && 0.0 < vx {
            f.offset_0x34 = vx;
            f.field_0x70 = -0x14;
            return;
        }
    } else {
        f.field_0x70 = 0x14;
    }
    f.offset_0x34 = vx;
}

/// port: 005036a0 FUN_005036a0
/// `SyncColor(DataSync&, Color&)`: the four channels, a byte each.
pub fn FUN_005036a0(param_1: &mut crate::sexy::data_sync::DataSync, param_2: &mut Color) -> crate::sexy::data_sync::SyncResult {
    use crate::sexy::data_sync::FUN_00503040;
    FUN_00503040(param_1, &mut param_2.mRed)?;
    FUN_00503040(param_1, &mut param_2.mGreen)?;
    FUN_00503040(param_1, &mut param_2.mBlue)?;
    FUN_00503040(param_1, &mut param_2.mAlpha)
}

/// port: 004de610 Sexy::Fish::vfunction81
/// `Sync(DataSync&)`: the GameObject part, its fields, the layer colors when colored (a
/// loaded rainbow fish has its colors recomputed), and the virtual-tank flag.
pub fn vfunction81(g: &mut G, this: Ptr, sync: &mut crate::sexy::data_sync::DataSync) -> crate::sexy::data_sync::SyncResult {
    use crate::sexy::data_sync::{DataIo, FUN_00503010, FUN_00503070, FUN_005030a0};
    crate::game::game_object::vfunction81(g, this, sync)?;
    let mut d = g.fish(this).clone();
    let vt = g.go(this).offset_0x24;
    let mut rainbow = false;
    let r = (|| {
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
            &mut d.field_0x6c, &mut d.field_0x70, &mut d.field_0x74, &mut d.field_0x78,
        ] {
            FUN_00503010(sync, f)?;
        }
        FUN_00503070(sync, &mut d.field_0x7c)?;
        if d.field_0x7c {
            FUN_00503070(sync, &mut d.field_0x7d)?;
            FUN_005036a0(sync, &mut d.field_0x84)?;
            FUN_005036a0(sync, &mut d.field_0x94)?;
            FUN_005036a0(sync, &mut d.field_0xa4)?;
            rainbow = matches!(sync.io, DataIo::Read(_)) && d.field_0x7d;
        }
        FUN_00503010(sync, &mut d.field_0xb4)?;
        FUN_00503010(sync, &mut d.field_0xcc)?;
        FUN_00503010(sync, &mut d.field_0xd0)?;
        FUN_00503070(sync, &mut d.field_0xd4)?;
        FUN_00503070(sync, &mut d.field_0xd5)?;
        if -1 < vt {
            FUN_00503070(sync, &mut d.offset_0xd6)?;
        }
        Ok(())
    })();
    // The original syncs in place, so the colors are recomputed mid-way; nothing after
    // the colors touches them.
    *g.fish(this) = d;
    if rainbow {
        FUN_004d5940(g, this);
    }
    r
}

/// port: 004dee90 Sexy::Fish::vfunction80
/// `DrawIcon(Graphics*, int pose)`: the fish's first-frame cel at a pose offset (0 the tall
/// store shelf, 1 a creature shelf, 2 the counter, 3, 4); the row is its size (king and star
/// rows 2 and 3, other sizes the pose); colored fish in their three tinted layers.
pub fn vfunction80(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: i32) {
    use crate::game::game_object::FUN_004d6d10;
    use crate::sexy::graphics::{FUN_004558c0, FUN_004558e0, FUN_00455e40, FUN_004560a0, FUN_004563d0};
    let (dx, dy) = match param_2 {
        0 => (0x13, 0x14),
        1 => (10, 0x11),
        2 => (0x13, 0),
        3 => (5, 0),
        4 => (-0x28, -5),
        _ => (0, 0),
    };
    FUN_004563d0(gfx, dx, dy);
    let size = g.fish(this).offset_0x4c;
    let row = if size <= 2 {
        size
    } else if size == 3 {
        2
    } else if size == 4 {
        3
    } else {
        param_2
    };
    let src = Rect::new(g.go(this).offset_0xa8 * 0x50, row * 0x50, 0x50, 0x50);
    if !g.fish(this).field_0x7c {
        FUN_004d6d10(g, this, gfx, Color::WHITE);
        let img = g.res.DAT_005e8ab8;
        FUN_00455e40(gfx, g, img, 0, 0, &src);
    } else {
        FUN_004d6d10(g, this, gfx, Color::WHITE);
        let i0 = crate::sexy::res::FUN_005016a0(g, 0xb8) as Ptr;
        FUN_004560a0(gfx, g, i0, 0, 0, &src, false);
        FUN_004558c0(gfx, 1);
        let c = g.fish(this).field_0x94;
        FUN_004d6d10(g, this, gfx, c);
        let i1 = crate::sexy::res::FUN_005016a0(g, 0xb9) as Ptr;
        FUN_004560a0(gfx, g, i1, 0, 0, &src, false);
        let c = g.fish(this).field_0x84;
        FUN_004d6d10(g, this, gfx, c);
        let i2 = crate::sexy::res::FUN_005016a0(g, 0xba) as Ptr;
        FUN_004560a0(gfx, g, i2, 0, 0, &src, false);
        FUN_004558c0(gfx, 0);
    }
    FUN_004558e0(gfx, false);
    FUN_004563d0(gfx, -dx, -dy);
}
