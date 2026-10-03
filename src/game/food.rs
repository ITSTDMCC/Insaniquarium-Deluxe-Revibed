//! `Sexy::Food`: pellets, pills, and the pizza/ice-cream/chicken treats. `Food_data` starts
//! at object offset 0x154.

use crate::game::game_object::{FUN_004f22c0, GameObject};
use crate::sexy::graphics::{FUN_00455890, FUN_004558e0, FUN_00455d20, FUN_00456980};
use crate::sexy::prelude::*;

/// `Food_data` (object offset 0x154, 72 bytes).
#[derive(Debug, Clone, Default)]
pub struct Food_data {
    /// +0x158: x.
    pub offset_0x4: f64,
    /// +0x160: y.
    pub offset_0xc: f64,
    /// +0x168: horizontal drift speed.
    pub offset_0x14: f64,
    /// +0x170: vertical pop speed.
    pub offset_0x1c: f64,
    /// +0x178: animation counter.
    pub offset_0x24: i32,
    /// +0x17c: updates per animation frame (3 or 4).
    pub offset_0x28: i32,
    /// +0x180: fade-out countdown (0 = not fading).
    pub offset_0x2c: i32,
    /// +0x184: done slowing at the surface.
    pub offset_0x30: bool,
    /// +0x185: picked up (flies to the counter).
    pub offset_0x31: bool,
    /// +0x188: food grade (cel row; 3 = the pill that explodes).
    pub offset_0x34: i32,
    /// +0x18c: drop direction (0 none, 1 left, 2 right).
    pub offset_0x38: i32,
    /// +0x190: dropped from the top (slows down while sinking).
    pub offset_0x3c: bool,
    /// +0x194: kind (2 = clickable bonus, 3/4/5 = pizza/ice cream/chicken).
    pub offset_0x40: i32,
    /// +0x198: countdown, 20 initially.
    pub offset_0x44: i32,
}

fn alloc_food(g: &mut G, wc: crate::sexy::object::WidgetContainer_data, w: crate::sexy::object::Widget_data,
              go: crate::game::game_object::GameObject_data, food: Food_data) -> Ptr {
    g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__Food_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::GameObject(Box::new(GameObjectExt { go, sub: GoSub::Food(food) })) }),
    })
}

/// port: 004ef820 Sexy::Food::Food
/// `Food::Food()` (used before loading from a save).
pub fn Food__004ef820(g: &mut G) -> Ptr {
    let (mut wc, w, mut go) = GameObject(g);
    wc.offset_0x3d = false;
    go.offset_0x4 = 0x1c;
    alloc_food(g, wc, w, go, Food_data::default())
}

/// port: 004ef840 Sexy::Food::Food
/// `Food::Food(int x, int y, int dropDir, bool fromTop, int kind)`.
pub fn Food__004ef840(g: &mut G, param_1: i32, param_2: i32, param_3: i32, param_4: bool, param_5: i32) -> Ptr {
    let (mut wc, mut w, mut go) = GameObject(g);
    let mut f = Food_data::default();
    f.offset_0x4 = param_1 as f64;
    wc.offset_0x3d = false;
    f.offset_0xc = param_2 as f64;
    go.offset_0x4 = 0x1c;
    f.offset_0x40 = param_5;
    f.offset_0x3c = param_4;
    f.offset_0x30 = false;
    f.offset_0x34 = 0;
    wc.offset_0x2c = crate::sexy::crt::ftol(f.offset_0x4) as i32;
    wc.offset_0x30 = crate::sexy::crt::ftol(f.offset_0xc) as i32;
    f.offset_0x1c = -2.0;
    wc.offset_0x34 = 0x28;
    wc.offset_0x38 = 0x28;
    f.offset_0x38 = param_3;
    f.offset_0x14 = if param_3 == 2 { 3.0 } else { -3.0 };
    if param_5 == 2 {
        w.offset_0x1 = true;
        w.offset_0x28 = true;
    } else {
        w.offset_0x1 = false;
    }
    f.offset_0x24 = 0;
    f.offset_0x2c = 0;
    // The MTRand is the app's (+0x7b0); the object is built before it is attached.
    let app = go.offset_0x0;
    let rng = g.wfa(app).offset_0x84;
    f.offset_0x28 = (g.mtrand(rng).next() & 1) as i32 + 3;
    f.offset_0x31 = false;
    f.offset_0x44 = 0x14;
    alloc_food(g, wc, w, go, f)
}

/// port: 004f0180 Sexy::Food::deleting_destructor
pub fn deleting_destructor(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    crate::game::game_object::dtor_GameObject(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 004d62f0 Sexy::Food::vfunction55
/// `MouseDown(x, y, clickCount)`: a left click on a bonus item collects it; anything else
/// passes the click through to the board at the absolute position.
pub fn vfunction55(g: &mut G, this: Ptr, x: i32, y: i32, clicks: i32) {
    if -1 < clicks && g.food(this).offset_0x40 == 2 {
        FUN_004d6290(g, this);
        return;
    }
    let (ox, oy) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    crate::game::board_level::FUN_0053bfb0(g, board, ox + x, oy + y);
}

/// port: 004d6290 FUN_004d6290
/// Collects a bonus food: hide the finger, stop taking the mouse, fly to the counter.
pub fn FUN_004d6290(g: &mut G, this: Ptr) {
    if !g.food(this).offset_0x31 {
        vcall!(g, this, w.vfunction39, false);
        g.w(this).offset_0x28 = false;
        g.w(this).offset_0x1 = false;
        g.food(this).offset_0x31 = true;
        let app = g.go(this).offset_0x0;
        let board = g.wfa(app).offset_0x4;
        crate::game::board::FUN_005383d0(g, board);
    }
}

/// port: 004d62d0 FUN_004d62d0
/// Starts the 15-update fade-out unless already fading.
pub fn FUN_004d62d0(g: &mut G, this: Ptr) {
    if g.food(this).offset_0x2c < 1 {
        g.food(this).offset_0x2c = 0xf;
    }
}

/// port: 004d6ae0 FUN_004d6ae0
/// Lets the board's timed-message list run during drawing while `DAT_005e89cd` is set.
pub fn FUN_004d6ae0(g: &mut G, this: Ptr) {
    if g.globals.DAT_005e89cd {
        let app = g.go(this).offset_0x0;
        let board = g.wfa(app).offset_0x4;
        crate::game::board::FUN_00538060(g, board);
    }
}

/// port: 004e11e0 Sexy::Food::vfunction27
/// `Draw(Graphics*)`.
pub fn vfunction27(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    FUN_004d6ae0(g, this);
    let fade = g.food(this).offset_0x2c;
    if fade != 0 {
        FUN_00455890(gfx, CRect(0xff, 0xff, 0xff, (fade * 0xff) / 0xf));
        FUN_004558e0(gfx, true);
    }
    let kind = g.food(this).offset_0x40;
    let frame = g.food(this).offset_0x24 / g.food(this).offset_0x28;
    if kind == 2 {
        FUN_004558e0(gfx, true);
        FUN_00455890(gfx, CRect(0xff, 0xff, 0xff, 200));
        let img = g.res.DAT_005e8aa0;
        FUN_00456980(gfx, g, img, 0, 0, frame, 4);
        FUN_004558e0(gfx, false);
        FUN_004558e0(gfx, false);
        return;
    }
    let treat = match kind {
        3 => Some(g.res.DAT_005e8d0c),
        4 => Some(g.res.DAT_005e8e0c),
        5 => Some(g.res.DAT_005e8b64),
        _ => None,
    };
    if let Some(img) = treat {
        FUN_00455d20(gfx, g, img, 0, 0);
        FUN_004558e0(gfx, false);
        return;
    }
    let grade = g.food(this).offset_0x34;
    let (x, y) = match grade {
        0 | 1 | 2 => (-5, -4),
        3 => (0, -3),
        _ => {
            FUN_004558e0(gfx, false);
            return;
        }
    };
    let img = g.res.DAT_005e8aa0;
    FUN_00456980(gfx, g, img, x, y, frame, grade);
    FUN_004558e0(gfx, false);
}

/// The shared `Remove()` body the decompiler filed under `Larva::vfunction76` (@ 004d9570).
fn remove(g: &mut G, this: Ptr) {
    crate::game::larva::vfunction76(g, this);
}

/// port: 004f8aa0 Sexy::Food::vfunction23
/// `Update()`: sinking, surface slow-down, drifting, collection flight, fade, and the
/// exploding pill.
pub fn vfunction23(g: &mut G, this: Ptr) {
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    if board == NULL {
        return;
    }
    if g.board(board).field_0x8 {
        return;
    }
    FUN_004f22c0(g, this);
    let f = g.food(this);
    if f.offset_0x44 != 0 {
        f.offset_0x44 -= 1;
    }
    f.offset_0x24 += 1;
    if f.offset_0x28 * 10 - 1 < f.offset_0x24 {
        f.offset_0x24 = 0;
    }
    if g.food(this).offset_0x34 == 3 {
        g.w(this).offset_0x1 = true;
    }
    let fade = g.food(this).offset_0x2c;
    if fade != 0 {
        g.food(this).offset_0x2c = fade - 1;
        if fade - 1 != 0 {
            return;
        }
        remove(g, this);
        return;
    }
    let f = g.food(this);
    if !f.offset_0x3c || f.offset_0x30 {
        if !f.offset_0x31 {
            f.offset_0xc += 1.5;
        } else {
            if f.offset_0x4 <= 550.0 {
                if f.offset_0x4 < 550.0 {
                    f.offset_0x4 = (550.0 - f.offset_0x4) / 7.0 + f.offset_0x4;
                }
            } else {
                let x = f.offset_0x4;
                f.offset_0x4 = x - (x - 550.0) / 7.0;
            }
            if f.offset_0xc <= 30.0 {
                if f.offset_0xc < 30.0 {
                    f.offset_0xc = (30.0 - f.offset_0xc) / 7.0 + f.offset_0xc;
                }
            } else {
                let y = f.offset_0xc;
                f.offset_0xc = y - (y - 30.0) / 7.0;
            }
            if g.wc(this).offset_0x30 < 0x28 {
                remove(g, this);
                crate::game::board::FUN_0053c1e0(g, board, 1);
                return;
            }
            let (x, y) = (g.food(this).offset_0x4, g.food(this).offset_0xc);
            let (iy, ix) = (crate::sexy::crt::ftol(y) as i32, crate::sexy::crt::ftol(x) as i32);
            vcall!(g, this, w.vfunction42, ix, iy);
        }
    } else if 120.0 <= f.offset_0xc {
        if 125.0 <= f.offset_0xc {
            if 130.0 <= f.offset_0xc {
                if 135.0 <= f.offset_0xc {
                    if 150.0 <= f.offset_0xc {
                        f.offset_0xc -= 8.0;
                    } else {
                        f.offset_0xc -= 2.5;
                    }
                } else {
                    f.offset_0xc -= 1.5;
                }
            } else {
                f.offset_0xc -= 1.0;
            }
        } else {
            f.offset_0xc -= 0.5;
        }
    } else {
        f.offset_0x30 = true;
    }
    let f = g.food(this);
    let dir = f.offset_0x38;
    if dir != 0 {
        if f.offset_0x1c < 0.0 {
            let v = f.offset_0x1c + 0.05;
            f.offset_0x1c = v;
            f.offset_0xc = v + f.offset_0xc;
        }
        if dir == 2 {
            if 0.0 < f.offset_0x14 {
                let v = f.offset_0x14 - 0.05;
                f.offset_0x14 = v;
                f.offset_0x4 = v + f.offset_0x4;
            }
        } else if dir == 1 && f.offset_0x14 < 0.0 {
            let v = 0.05 + f.offset_0x14;
            f.offset_0x14 = v;
            f.offset_0x4 = v + f.offset_0x4;
        }
        if 550.0 < f.offset_0x4 {
            f.offset_0x4 = 550.0;
        }
        if f.offset_0x4 < 20.0 {
            f.offset_0x4 = 20.0;
        }
    }
    let f = g.food(this);
    if 410.0 < f.offset_0xc || (f.offset_0x34 == 3 && 400.0 < f.offset_0xc) {
        if f.offset_0x34 == 3 {
            crate::game::board::FUN_00538230(g, board, 0x11d, 3, 1.0);
            let rng = g.wfa(app).offset_0x84;
            let mut n = g.mtrand(rng).next() % 3 + 2;
            while n != 0 {
                let count = g.mtrand(rng).next() % 3 + 3;
                let dy = (g.mtrand(rng).next() % 0x1e) as i32 - 10 + g.wc(this).offset_0x30;
                let dx = (g.mtrand(rng).next() % 0x1e) as i32 - 10 + g.wc(this).offset_0x2c;
                crate::game::board_level::FUN_005436f0(g, board, dx, dy, count as i32);
                n -= 1;
            }
            remove(g, this);
            return;
        }
        if f.offset_0x40 == 2 {
            remove(g, this);
            return;
        }
        FUN_004d62d0(g, this);
        g.food(this).offset_0xc = 410.0;
    }
    let (x, y) = (g.food(this).offset_0x4, g.food(this).offset_0xc);
    let (iy, ix) = (crate::sexy::crt::ftol(y) as i32, crate::sexy::crt::ftol(x) as i32);
    vcall!(g, this, w.vfunction42, ix, iy);
}

/// port: 004e1100 Sexy::Food::vfunction81
/// `Sync(DataSync&)`: the GameObject part, then every field in order. (The original reads
/// straight into the fields, so a failed load leaves the ones already read; so does this.)
pub fn vfunction81(g: &mut G, this: Ptr, sync: &mut crate::sexy::data_sync::DataSync) -> crate::sexy::data_sync::SyncResult {
    crate::game::game_object::vfunction81(g, this, sync)?;
    use crate::sexy::data_sync::{FUN_00503010, FUN_00503070, FUN_005030a0};
    let mut d = g.food(this).clone();
    let r = (|| {
        FUN_005030a0(sync, &mut d.offset_0x4)?;
        FUN_005030a0(sync, &mut d.offset_0xc)?;
        FUN_005030a0(sync, &mut d.offset_0x14)?;
        FUN_005030a0(sync, &mut d.offset_0x1c)?;
        FUN_00503010(sync, &mut d.offset_0x24)?;
        FUN_00503010(sync, &mut d.offset_0x28)?;
        FUN_00503010(sync, &mut d.offset_0x2c)?;
        FUN_00503070(sync, &mut d.offset_0x30)?;
        FUN_00503070(sync, &mut d.offset_0x31)?;
        FUN_00503010(sync, &mut d.offset_0x34)?;
        FUN_00503010(sync, &mut d.offset_0x38)?;
        FUN_00503070(sync, &mut d.offset_0x3c)?;
        FUN_00503010(sync, &mut d.offset_0x40)?;
        FUN_00503010(sync, &mut d.offset_0x44)
    })();
    *g.food(this) = d;
    r
}
