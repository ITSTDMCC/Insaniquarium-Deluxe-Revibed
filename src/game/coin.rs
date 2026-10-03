//! `Sexy::Coin`: everything a fish or pet drops that the player clicks to collect (coins,
//! stars, gems, chests, beetles, pearls), plus the floating score texts (kind 0x10) and the
//! falling bomb (kind 0x11) that kills the fish it lands on. `Coin_data` starts at object
//! offset 0x154.
//!
//! Kinds: 1/8 silver, 2/9 gold, 3/10 star, 4/0xb and 5/0xc diamond and treasure, 6/0xd
//! pearl, 7/0xe treasure chest (8..0xe rise instead of falling), 0xf a pet's bonus, 0x10
//! floating text, 0x11 bomb, 0x12 beetle-like rising item.

use crate::game::game_object::{FUN_004f22c0, GameObject};
use crate::game::larva;
use crate::sexy::crt::ftol;
use crate::sexy::graphics::{FUN_00455870, FUN_00455880, FUN_004558c0, FUN_004558e0, FUN_00455890, FUN_00455cf0, FUN_00455d20, FUN_00456950, FUN_00456980};
use crate::sexy::image_font as font;
use crate::sexy::prelude::*;
use crate::sexy::types::{CRect, FUN_00433320};

/// `Coin_data` (object offset 0x154, 112 bytes).
#[derive(Debug, Clone, Default)]
pub struct Coin_data {
    /// +0x158 x.
    pub offset_0x4: f64,
    /// +0x160 y.
    pub offset_0xc: f64,
    /// +0x168 a rising item has reached its height.
    pub offset_0x14: bool,
    /// +0x16c animation counter (0..0x4f).
    pub offset_0x18: i32,
    /// +0x170 age in updates.
    pub offset_0x1c: i32,
    /// +0x174 animation cel.
    pub offset_0x20: i32,
    /// +0x178 fade-out countdown (removed at 0).
    pub offset_0x24: i32,
    /// +0x17c collect flight step.
    pub offset_0x28: i32,
    /// +0x180 collect flight start x.
    pub field_0x2c: i32,
    /// +0x184 collect flight start y.
    pub field_0x30: i32,
    /// +0x188 falling speed (virtual tank).
    pub offset_0x34: f64,
    /// +0x190 the object that dropped it (kind 0xf: the pet).
    pub offset_0x3c: Ptr,
    /// +0x194 kind.
    pub offset_0x40: i32,
    /// +0x198 collected.
    pub offset_0x44: bool,
    /// +0x19c updates on the floor / text age.
    pub offset_0x48: i32,
    /// +0x1a0 text style (kind 0x10).
    pub offset_0x4c: i32,
    /// +0x1a4 the text (kind 0x10).
    pub field_0x50: Vec<u8>,
    /// +0x1c0 value multiplier (virtual-tank chains).
    pub offset_0x6c: i32,
}

impl G {
    pub fn coin(&mut self, p: Ptr) -> &mut Coin_data {
        match &mut self.go_ext(p).sub {
            GoSub::Coin(d) => d,
            s => panic!("{p} is not a Coin: {s:?}"),
        }
    }
}

fn alloc_coin(g: &mut G, wc: crate::sexy::object::WidgetContainer_data, w: crate::sexy::object::Widget_data,
              go: crate::game::game_object::GameObject_data, d: Coin_data) -> Ptr {
    g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__Coin_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::GameObject(Box::new(GameObjectExt { go, sub: GoSub::Coin(d) })) }),
    })
}

/// port: 004eea80 Sexy::Coin::Coin
/// `Coin::Coin()` (used before loading from a save).
pub fn Coin__004eea80(g: &mut G) -> Ptr {
    let (mut wc, w, mut go) = GameObject(g);
    wc.offset_0x3d = false;
    go.offset_0x4 = 0x19;
    alloc_coin(g, wc, w, go, Coin_data { offset_0x3c: NULL, ..Default::default() })
}

/// port: 004eead0 Sexy::Coin::Coin
/// `Coin::Coin(int x, int y, int kind, GameObject* from, double speed)`: 72x72 at (x, y);
/// texts (kind 0x10) ignore the mouse. `speed` -1 keeps the default 1.5.
pub fn Coin__004eead0(g: &mut G, param_1: i32, param_2: i32, param_3: i32, param_4: Ptr, param_5: f64) -> Ptr {
    let (mut wc, mut w, mut go) = GameObject(g);
    let mut d = Coin_data::default();
    d.offset_0x4 = param_1 as f64;
    go.offset_0x4 = 0x19;
    wc.offset_0x3d = false;
    d.offset_0x4c = 0;
    d.offset_0xc = param_2 as f64;
    d.offset_0x28 = 0;
    d.offset_0x6c = 1;
    d.offset_0x3c = param_4;
    d.offset_0x40 = param_3;
    wc.offset_0x2c = ftol(d.offset_0x4) as i32;
    wc.offset_0x30 = ftol(d.offset_0xc) as i32;
    wc.offset_0x34 = 0x48;
    wc.offset_0x38 = 0x48;
    if param_3 == 0x10 {
        w.offset_0x1 = false;
        w.offset_0x28 = false;
    } else {
        w.offset_0x28 = true;
    }
    d.offset_0x48 = 0;
    d.offset_0x34 = 1.5;
    d.offset_0x14 = false;
    d.offset_0x44 = false;
    d.offset_0x18 = 0;
    d.offset_0x1c = 0;
    d.offset_0x24 = 0;
    d.offset_0x20 = 0;
    if param_5 != -1.0 {
        d.offset_0x34 = param_5;
    }
    alloc_coin(g, wc, w, go, d)
}

/// port: 004f0c50 Sexy::Coin::vfunction1
/// The deleting destructor.
pub fn vfunction1(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    g.coin(this).field_0x50.clear();
    crate::game::game_object::dtor_GameObject(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 004d5510 Sexy::Coin::vfunction76
/// `Remove()`: leaves the tank; an uncollected bomb bursts (sound 0x11d and 2..4 sparkles).
pub fn vfunction76(g: &mut G, this: Ptr) {
    larva::vfunction76(g, this);
    let d = g.coin(this).clone();
    if d.offset_0x40 == 0x11 && !d.offset_0x44 {
        let app = g.go(this).offset_0x0;
        let board = g.wfa(app).offset_0x4;
        crate::game::board::FUN_00538230(g, board, 0x11d, 3, 1.0);
        let rng = g.wfa(app).offset_0x84;
        let mut n = (crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng)) % 3) as i32 + 2;
        while 0 < n {
            let a = (crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng)) % 3) as i32 + 3;
            let my = g.wc(this).offset_0x30;
            let b = (crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng)) % 0x1e) as i32 + my - 10;
            let mx = g.wc(this).offset_0x2c;
            let c = (crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng)) % 0x1e) as i32 + mx - 10;
            let board = g.wfa(app).offset_0x4;
            crate::game::board_level::FUN_005436f0(g, board, c, b, a);
            n -= 1;
        }
    }
}

/// port: 004dd680 Sexy::Coin::vfunction81
/// `Sync(DataSync&)`.
pub fn vfunction81(g: &mut G, this: Ptr, sync: &mut crate::sexy::data_sync::DataSync) -> crate::sexy::data_sync::SyncResult {
    crate::game::game_object::vfunction81(g, this, sync)?;
    use crate::sexy::data_sync::{FUN_00503010, FUN_00503070, FUN_005030a0, FUN_00505860};
    let mut d = g.coin(this).clone();
    FUN_005030a0(sync, &mut d.offset_0x4)?;
    FUN_005030a0(sync, &mut d.offset_0xc)?;
    FUN_00503070(sync, &mut d.offset_0x14)?;
    FUN_00503010(sync, &mut d.offset_0x18)?;
    FUN_00503010(sync, &mut d.offset_0x1c)?;
    FUN_00503010(sync, &mut d.offset_0x20)?;
    FUN_00503010(sync, &mut d.offset_0x24)?;
    FUN_00503010(sync, &mut d.offset_0x28)?;
    if 0 < d.offset_0x28 {
        FUN_00503010(sync, &mut d.field_0x2c)?;
        FUN_00503010(sync, &mut d.field_0x30)?;
    }
    FUN_005030a0(sync, &mut d.offset_0x34)?;
    FUN_00503010(sync, &mut d.offset_0x40)?;
    FUN_00503070(sync, &mut d.offset_0x44)?;
    FUN_00503010(sync, &mut d.offset_0x48)?;
    FUN_00503010(sync, &mut d.offset_0x6c)?;
    FUN_00503010(sync, &mut d.offset_0x4c)?;
    if 1 < d.offset_0x4c {
        FUN_00505860(sync, &mut d.field_0x50)?;
    }
    crate::sexy::data_sync::FUN_005122f0(sync, crate::sexy::data_sync::PtrSlot::CoinOffset0x3c(this));
    let r = Ok(());
    *g.coin(this) = d;
    r
}

/// port: 004d5360 FUN_004d5360
/// Virtual-tank look: mode 5 or the board's +0x2a5 flag.
pub fn FUN_004d5360(g: &mut G, this: Ptr) -> bool {
    let app = g.go(this).offset_0x0;
    if g.wfa(app).offset_0x150 != 5 {
        let board = g.wfa(app).offset_0x4;
        if !g.board(board).field_0x219 {
            return false;
        }
    }
    true
}

/// port: 004d5390 FUN_004d5390
/// `FadeOut()`: five updates of fade.
pub fn FUN_004d5390(g: &mut G, this: Ptr) {
    g.coin(this).offset_0x24 = 5;
}

/// port: 004d53a0 FUN_004d53a0
/// `GetValue()`: 15 / 35 / 40 / 200 / 500 / 2000 / 150 / 250 by kind, or the multiplier
/// times 1 / 2 / 3 / 5 / 10 / 20 / 5 / 10 in the virtual-tank look.
pub fn FUN_004d53a0(g: &mut G, this: Ptr) -> i32 {
    let kind = g.coin(this).offset_0x40;
    let (base, mul) = match kind {
        1 | 8 | 0x12 => (0xf, 1),
        2 | 9 => (0x23, 2),
        3 | 10 => (0x28, 3),
        4 | 0xb | 5 | 0xc => (200, 5),
        6 | 0xd => (500, 10),
        7 | 0xe => (2000, 0x14),
        0x11 => (0x96, 5),
        0xf => (0xfa, 10),
        _ => return 0,
    };
    if FUN_004d5360(g, this) { g.coin(this).offset_0x6c * mul } else { base }
}

/// port: 004d54e0 FUN_004d54e0
/// `Collected()`: marks a game in progress and adds the value to the money.
pub fn FUN_004d54e0(g: &mut G, this: Ptr) {
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    g.board(board).ext_0x4ee = true;
    let v = FUN_004d53a0(g, this);
    let board = g.wfa(app).offset_0x4;
    crate::game::board::FUN_0053c1e0(g, board, v);
}

/// port: 004e51f0 FUN_004e51f0
/// `Collect()`: starts the flight to the money counter, no more mouse; a pet's bonus (0xf)
/// tells its pet; then the collect sound by kind (the virtual tank's chain bonus may take
/// over).
pub fn FUN_004e51f0(g: &mut G, this: Ptr) {
    let app = g.go(this).offset_0x0;
    let d = g.coin(this);
    d.offset_0x28 = 1;
    d.field_0x2c = ftol(d.offset_0x4) as i32;
    d.field_0x30 = ftol(d.offset_0xc) as i32;
    if d.offset_0x40 == 0xf {
        d.offset_0x44 = true;
        let from = d.offset_0x3c;
        let board = g.wfa(app).offset_0x4;
        for o in g.board(board).offset_0xc[crate::game::board_level::vec_index(0xb0)].clone() {
            if from != NULL && o == from {
                g.other_pet(o).offset_0x6c = true;
            }
        }
        crate::game::board::FUN_00538230(g, board, 299, 3, 1.0);
    }
    let kind = g.coin(this).offset_0x40;
    g.w(this).offset_0x1 = false;
    if kind < 0xf || kind == 0x11 || kind == 0x12 {
        g.coin(this).offset_0x44 = true;
        let board = g.wfa(app).offset_0x4;
        if g.board(board).field_0x219 && crate::game::board_level::FUN_00545e20(g, board, this) {
            return;
        }
        let board = g.wfa(app).offset_0x4;
        match kind {
            4 | 5 | 0xb | 0xc => crate::game::board_level::FUN_00538470(g, board),
            0x12 => crate::game::board_level::FUN_00538560(g, board, false),
            6 | 0xd => crate::game::board::FUN_00538230(g, board, 299, 3, 1.0),
            7 | 0xe => {
                let id = if FUN_004d5360(g, this) { 0x10f } else { 0x142 };
                crate::game::board::FUN_00538230(g, board, id, 3, 1.0);
            }
            _ => crate::game::board::FUN_005383d0(g, board),
        }
    }
}

/// port: 004e5450 Sexy::Coin::vfunction55
/// `MouseDown(x, y, clicks)`: right clicks go to the board; otherwise collects (also firing
/// at aliens when there are any), except texts, collected coins and a fresh 0x12.
pub fn vfunction55(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32) {
    let app = g.go(this).offset_0x0;
    let (mx, my) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
    if param_3 < 0 {
        let board = g.wfa(app).offset_0x4;
        crate::game::board_level::FUN_0053bfb0(g, board, mx + param_1, my + param_2);
        return;
    }
    let d = g.coin(this).clone();
    if d.offset_0x40 == 0x12 && g.wc(this).offset_0x24 < 0xc {
        return;
    }
    if d.offset_0x40 == 0x10 || d.offset_0x44 {
        return;
    }
    let board = g.wfa(app).offset_0x4;
    if crate::game::board_update::FUN_004da780(g, board) || crate::game::board_level::FUN_004e3ea0(g, board) {
        crate::game::board_update::FUN_00539d40(g, board, mx + param_1, my + param_2);
    }
    FUN_004e51f0(g, this);
}

/// Whether the bomb at (x + 36, y + 36) is inside `o`'s box (its position + 10 .. + `ext`).
fn bomb_hits(g: &mut G, o: Ptr, x: f64, y: f64, ext: i32) -> bool {
    if g.go(o).offset_0x24 >= 0 {
        return false;
    }
    let (ox, oy) = (g.wc(o).offset_0x2c, g.wc(o).offset_0x30);
    let (cx, cy) = (x + 36.0, y + 36.0);
    cx < (ox + ext) as f64 && ((ox + 10) as f64) < cx && cy < (oy + ext) as f64 && ((oy + 10) as f64) < cy
}

/// `Move(ftol(x), ftol(y))` (vtable +0xa4).
fn move_to_pos(g: &mut G, this: Ptr) {
    let d = g.coin(this);
    let (x, y) = (d.offset_0x4, d.offset_0xc);
    let (ix, iy) = (ftol(x) as i32, ftol(y) as i32);
    vcall!(g, this, w.vfunction42, ix, iy);
}

/// port: 004f5050 Sexy::Coin::vfunction23
/// `Update()` (not while paused): the animation cel; fading; texts drift up; pet bonuses
/// expire; rising items float up to ~120; everything else falls (slower with pet 10, at its
/// own speed in the virtual tank) and fades out after a while on the floor (a bomb bursts
/// there instead, and kills the first fish, pet or alien it touches after 30 updates).
/// Collected items fly to the money counter (or the virtual tank's shell counter) and pay.
pub fn vfunction23(g: &mut G, this: Ptr) {
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    if board == NULL || g.board(board).field_0x8 {
        return;
    }
    FUN_004f22c0(g, this);
    let d = g.coin(this);
    d.offset_0x18 += 1;
    d.offset_0x1c += 1;
    if 0x4f < d.offset_0x18 {
        d.offset_0x18 = 0;
    }
    let kind = d.offset_0x40;
    let counter = d.offset_0x18;
    let (n, m) = if kind == 0x11 {
        (counter / 2, 5)
    } else if FUN_004d5360(g, this) && matches!(kind, 1 | 8 | 2 | 9 | 5 | 0xc | 4 | 0xb | 6 | 0xd | 7 | 0xe) {
        let div = (g.board(board).field_0xb8[10] != 0) as i32 * 2 + 2;
        (counter / div, 0x14)
    } else if g.board(board).field_0xb8[10] == 0 {
        (counter / 2, 10)
    } else {
        (counter / 4, 10)
    };
    let d = g.coin(this);
    d.offset_0x20 = n % m;
    let fade = d.offset_0x24;
    if fade != 0 {
        if !d.offset_0x44 {
            d.offset_0x24 = fade - 1;
            if fade - 1 == 0 {
                vfunction76(g, this);
            }
            return;
        }
        d.offset_0x24 = 0;
    }
    if g.coin(this).offset_0x44 {
        fly_to_counter(g, this);
        return;
    }
    if kind == 0xf {
        let d = g.coin(this);
        d.offset_0x48 += 1;
        if 0xd8 < d.offset_0x48 {
            vfunction76(g, this);
        }
        return;
    }
    if kind == 0x10 {
        let d = g.coin(this);
        let style = d.offset_0x4c;
        d.offset_0x48 += 1;
        let t = d.offset_0x48;
        let top = if style != 0 { -0x32 } else { 0x32 };
        if (style < 3 || t <= 0x32) && t <= 0x64 && (top as f64) <= d.offset_0xc {
            d.offset_0xc -= 1.0;
            move_to_pos(g, this);
        } else {
            vfunction76(g, this);
        }
        return;
    }
    if (kind == 0x12 || kind == 5 || (8..=0xe).contains(&kind)) && !g.coin(this).offset_0x14 {
        let d = g.coin(this);
        let y = d.offset_0xc;
        if y < 120.0 {
            d.offset_0x14 = true;
        } else {
            d.offset_0xc = if y < 125.0 {
                y - 0.5
            } else if y < 130.0 {
                y - 1.0
            } else if y < 135.0 {
                y - 1.5
            } else if y < 150.0 {
                y - 2.5
            } else {
                y - 8.0
            };
        }
        move_to_pos(g, this);
        return;
    }
    // Falling.
    let pet10 = g.board(board).field_0xb8[10] != 0;
    let virt = g.board(board).field_0x219;
    let d = g.coin(this);
    d.offset_0xc = if pet10 {
        d.offset_0xc + 0.8
    } else if virt {
        d.offset_0x34 + d.offset_0xc
    } else {
        d.offset_0xc + 1.5
    };
    if 370.0 <= d.offset_0xc {
        d.offset_0xc = 370.0;
        if g.board(board).field_0x219 {
            FUN_004d5390(g, this);
        }
        g.coin(this).offset_0x48 += 1;
        let lim = if g.board(board).field_0xb8[10] != 0 {
            200
        } else if g.wfa(app).offset_0x154 {
            0x6c
        } else if crate::game::board_level::FUN_00537b80(g, board) {
            0x96
        } else {
            0x14
        };
        if lim <= g.coin(this).offset_0x48 {
            if g.coin(this).offset_0x40 == 0x11 {
                vfunction76(g, this);
                return;
            }
            FUN_004d5390(g, this);
        }
    }
    move_to_pos(g, this);
    if g.coin(this).offset_0x40 != 0x11 || g.coin(this).offset_0x1c <= 0x1e {
        return;
    }
    // The bomb: the first fish, pet or alien under it dies.
    let (x, y) = (g.coin(this).offset_0x4, g.coin(this).offset_0xc);
    use crate::game::board_level::vec_index;
    let mut hit = false;
    'outer: for (off, ext) in [(0xa0, 0x46), (0xa4, 0x46), (0xd4, 0x96), (0xcc, 0x46), (0xd0, 0x46), (0xc4, 0x46), (0xc0, 0x46)] {
        let board = g.wfa(app).offset_0x4;
        for o in g.board(board).offset_0xc[vec_index(off)].clone() {
            if bomb_hits(g, o, x, y, ext) {
                match off {
                    0xd0 => crate::game::penta::FUN_004f3c60(g, o, true),
                    0xc4 => crate::game::grubber::FUN_004f2890(g, o, true),
                    0xc0 => crate::game::breeder::FUN_004dd5d0(g, o, true),
                    _ => vcall!(g, o, fish.vfunction89, true),
                }
                hit = true;
                break 'outer;
            }
        }
    }
    if !hit {
        return;
    }
    vfunction76(g, this);
    let rng = g.wfa(app).offset_0x84;
    let n = (crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng)) % 3) as i32 + 3;
    let (x, y) = (g.coin(this).offset_0x4, g.coin(this).offset_0xc);
    let (iy, ix) = (ftol(y) as i32, ftol(x) as i32);
    let board = g.wfa(app).offset_0x4;
    crate::game::board_level::FUN_005436f0(g, board, ix, iy, n);
}

/// The collected part of `Update`: in the virtual tank a straight 5 (near) or 15 step flight
/// to (0x122, 0x13b); otherwise a 1/7 ease towards (550, 30) until above y 40. Pays on
/// arrival.
fn fly_to_counter(g: &mut G, this: Ptr) {
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    if g.board(board).field_0x219 {
        let (tx, ty) = crate::game::board_level::FUN_00538cd0(g, board);
        let d = g.coin(this);
        d.offset_0x28 += 1;
        let (ex, ey) = (tx + 0x1e, ty + 0x32);
        let dx = d.field_0x2c - ex;
        let dy = d.field_0x30 - ey;
        let n = if dx * dx + dy * dy <= 0x57e4 { 5 } else { 0xf };
        let step = d.offset_0x28;
        if n <= step {
            vfunction76(g, this);
            FUN_004d54e0(g, this);
            return;
        }
        let r = n - step;
        d.offset_0x4 = ((r * d.field_0x2c + ex * step) / n) as f64;
        d.offset_0xc = ((r * d.field_0x30 + ey * step) / n) as f64;
        move_to_pos(g, this);
        return;
    }
    let d = g.coin(this);
    if 550.0 < d.offset_0x4 {
        d.offset_0x4 -= (d.offset_0x4 - 550.0) / 7.0;
    } else if d.offset_0x4 < 550.0 {
        d.offset_0x4 += (550.0 - d.offset_0x4) / 7.0;
    }
    if 30.0 < d.offset_0xc {
        d.offset_0xc -= (d.offset_0xc - 30.0) / 7.0;
    } else if d.offset_0xc < 30.0 {
        d.offset_0xc += (30.0 - d.offset_0xc) / 7.0;
    }
    if g.wc(this).offset_0x30 < 0x28 {
        vfunction76(g, this);
        FUN_004d54e0(g, this);
        return;
    }
    move_to_pos(g, this);
}

/// port: 004dd7a0 Sexy::Coin::vfunction27
/// `Draw(Graphics*)`: the item's animation cel (the virtual-tank sheet at +20,+20 or the
/// normal sheet, one row per kind), fading with `offset_0x24`; texts in their style color;
/// the bomb with its additive fuse.
pub fn vfunction27(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    crate::game::food::FUN_004d6ae0(g, this);
    let d = g.coin(this).clone();
    // (`offset_0x48 % 8` is never 8 or more: the blink this was meant for never happens.)
    let r = d.offset_0x48 % 8;
    if 7 < r && d.offset_0x40 != 6 && d.offset_0x40 != 7 && !d.offset_0x44 {
        FUN_004558e0(gfx, false);
        return;
    }
    if d.offset_0x24 != 0 {
        FUN_004558e0(gfx, true);
        FUN_00455890(gfx, CRect(0xff, 0xff, 0xff, (d.offset_0x24 * 0xff) / 5));
    }
    let virt = FUN_004d5360(g, this);
    let cel = d.offset_0x20;
    let (v_img, n_img) = (g.res.DAT_005e8d7c, g.res.DAT_005e8e60);
    // (image, x, y, col, row)
    let cell: Option<(Ptr, i32, i32, i32, i32)> = match d.offset_0x40 {
        1 | 8 => Some(if virt { (v_img, 0x14, 0x14, cel, 0) } else { (n_img, 0, 0, cel, 0) }),
        2 | 9 => Some(if virt { (v_img, 0x14, 0x14, cel, 1) } else { (n_img, 0, 0, cel, 1) }),
        5 | 0xc | 4 | 0xb => Some(if virt { (v_img, 0x14, 0x14, cel, 2) } else { (n_img, 0, 0, cel, 3) }),
        6 | 0xd => {
            if !virt {
                let img = g.res.DAT_005e8bd8;
                FUN_00455d20(gfx, g, img, 0, 0);
                FUN_004558e0(gfx, false);
                return;
            }
            Some((v_img, 0x14, 0x14, cel, 3))
        }
        7 | 0xe => Some(if virt { (g.res.DAT_005e8d88, 0x12, 0x12, 0, 0) } else { (n_img, 0, 0, cel, 4) }),
        0x12 => {
            let img = g.res.DAT_005e8c50;
            FUN_00455d20(gfx, g, img, 0, 0);
            FUN_004558e0(gfx, false);
            return;
        }
        3 | 10 => Some((n_img, 0, 0, cel, 2)),
        0xf if d.offset_0x44 => {
            let img = g.res.DAT_005e8bd8;
            FUN_00455d20(gfx, g, img, 0, 0);
            FUN_004558e0(gfx, false);
            return;
        }
        0x10 => {
            draw_text(g, this, gfx, &d);
            return;
        }
        0x11 => {
            let img = g.res.DAT_005e8bc4;
            FUN_00456950(gfx, g, img, 0, 0, 0);
            FUN_004558c0(gfx, 1);
            let fuse = g.res.DAT_005e8d04;
            FUN_00456950(gfx, g, fuse, 10, -10, cel);
            FUN_004558c0(gfx, 0);
            FUN_004558e0(gfx, false);
            return;
        }
        _ => None,
    };
    if let Some((img, x, y, col, row)) = cell {
        FUN_00456980(gfx, g, img, x, y, col, row);
    }
    FUN_004558e0(gfx, false);
}

/// The floating text of kind 0x10: style 0/1 the plain cel, 2 white, 3..7 colored (fading
/// out over the last 50 updates; nothing at all, and colorize left on, once faded).
fn draw_text(g: &mut G, this: Ptr, gfx: &mut Graphics, d: &Coin_data) {
    let style = d.offset_0x4c;
    if style <= 1 {
        let img = g.res.DAT_005e8bc4;
        FUN_00456950(gfx, g, img, 0, 0, 1);
        FUN_004558e0(gfx, false);
        return;
    }
    let c = if style < 3 {
        let f = g.res.DAT_005e8cf0;
        FUN_00455880(gfx, f);
        FUN_00433320(0xffffff)
    } else {
        let f = g.res.DAT_005e8e18;
        FUN_00455880(gfx, f);
        let mut c = FUN_00433320(match style {
            3 => 0xaaaaaa,
            4 => 0xfddc41,
            5 => 0x29e7ff,
            6 => 0x88ff88,
            _ => 0xffff00,
        });
        let t = d.offset_0x48;
        if 0 < t {
            let a = ((0x32 - t) * 0xff) / 0x32;
            if a < 1 {
                return;
            }
            c.mAlpha = a;
        }
        c
    };
    FUN_00455890(gfx, c);
    let f = FUN_00455870(gfx);
    let h = font::get_height(g, f);
    FUN_00455cf0(gfx, g, &d.field_0x50, 5, h);
    FUN_004558e0(gfx, false);
    let _ = this;
}

/// port: 004ddc40 FUN_004ddc40
/// `CollectedByPet()`: pays, then the collect sound by kind (no chain bonus).
pub fn FUN_004ddc40(g: &mut G, this: Ptr) {
    FUN_004d54e0(g, this);
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    match g.coin(this).offset_0x40 {
        4 | 5 | 0xb | 0xc => crate::game::board_level::FUN_00538470(g, board),
        6 | 0xd => crate::game::board::FUN_00538230(g, board, 0x12b, 3, 1.0),
        7 | 0xe => {
            let id = if FUN_004d5360(g, this) { 0x10f } else { 0x142 };
            crate::game::board::FUN_00538230(g, board, id, 3, 1.0);
        }
        _ => crate::game::board::FUN_005383d0(g, board),
    }
}
