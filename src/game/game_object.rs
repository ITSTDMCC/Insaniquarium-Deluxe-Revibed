//! `Sexy::GameObject`: base of every tank object (fish, food, coins, aliens, pets). It is a
//! `Sexy::Widget` placed on the Board. `GameObject_data` starts at object offset 0x88.

use crate::sexy::object::{WidgetContainer_data, Widget_data};
use crate::sexy::prelude::*;

/// `GameObject_data` (object offset 0x88, 204 bytes).
#[derive(Debug, Clone, Default)]
pub struct GameObject_data {
    /// +0x88: the `WinFishApp*` (`gApp` at construction).
    pub offset_0x0: Ptr,
    /// +0x8c: object type id (Food sets 0x1c).
    pub offset_0x4: i32,
    /// +0x90.
    pub offset_0x8: i32,
    /// +0x94: the object's shadow.
    pub offset_0xc: Ptr,
    /// +0x98: a pointer saved through `DataSync::SyncPointer`.
    pub offset_0x10: Ptr,
    /// +0x9c.
    pub offset_0x14: i32,
    /// +0xa0.
    pub offset_0x18: i32,
    /// +0xa4 (300 initially).
    pub offset_0x1c: i32,
    /// +0xa8.
    pub offset_0x20: bool,
    /// +0xac (-1 initially); `Sync` saves the rest only when it is >= 0.
    pub offset_0x24: i32,
    /// +0xb0: a `std::string` (0xb0..0xcc; the decompiler's offset_0x2c/0x3c/0x40 are its
    /// buffer, size and capacity). Byte string, as the original.
    pub field_0x28: Vec<u8>,
    /// +0xd0: a `__time64_t` (purchase time).
    pub field_0x48: i64,
    /// +0xd8.
    pub offset_0x50: i32,
    /// +0xdc.
    pub offset_0x54: i32,
    /// +0xe0: price.
    pub field_0x58: i32,
    /// +0xe4..0xf0.
    pub offset_0x5c: [i32; 3],
    /// +0xf0.
    pub offset_0x68: i32,
    /// +0xf4.
    pub offset_0x6c: bool,
    /// +0xf5.
    pub offset_0x6d: bool,
    /// +0xf6.
    pub offset_0x6e: bool,
    /// +0xf8: countdown.
    pub offset_0x70: i32,
    /// +0xfc.
    pub offset_0x74: bool,
    /// +0x100.
    pub offset_0x78: i32,
    /// +0x104.
    pub offset_0x7c: bool,
    /// +0x108: countdown.
    pub offset_0x80: i32,
    /// +0x10c: countdown.
    pub offset_0x84: i32,
    /// +0x110.
    pub offset_0x88: bool,
    /// +0x111.
    pub offset_0x89: bool,
    /// +0x114: speech-bubble counter.
    pub offset_0x8c: i32,
    /// +0x118.
    pub offset_0x90: i32,
    /// +0x11c: timed-message id (-1 = none).
    pub offset_0x94: i32,
    /// +0x120: countdown.
    pub offset_0x98: i32,
    /// +0x124: 4 bytes saved raw (`SyncBytes`), used as an int.
    pub offset_0x9c: i32,
    /// +0x128.
    pub offset_0xa0: i32,
    /// +0x12c.
    pub offset_0xa4: i32,
    /// +0x130.
    pub offset_0xa8: i32,
    /// +0x138.
    pub offset_0xb0: i32,
    /// +0x13c.
    pub offset_0xb4: i32,
    /// +0x140.
    pub offset_0xb8: i32,
    /// +0x144.
    pub offset_0xbc: i32,
    /// +0x148.
    pub offset_0xc0: i32,
    /// +0x14c.
    pub offset_0xc4: i32,
    /// +0x150.
    pub offset_0xc8: i32,
}

/// port: 004e9cb0 Sexy::GameObject::GameObject
/// The parts of a freshly constructed GameObject (the Widget base, then its own fields).
pub fn GameObject(g: &mut G) -> (WidgetContainer_data, Widget_data, GameObject_data) {
    let (wc, w) = crate::sexy::widget::Widget();
    let go = GameObject_data {
        offset_0x0: g.globals.DAT_005eb6a4,
        offset_0x4: -1,
        offset_0x1c: 300,
        offset_0x6c: true,
        offset_0x89: true,
        offset_0x24: -1,
        offset_0x94: -1,
        offset_0x9c: -1,
        offset_0xc4: 3,
        ..Default::default()
    };
    (wc, w, go)
}

/// port: 004e9e10 Sexy::GameObject::~GameObject
/// Drops its timed message from the board (if the board still exists), then the Widget part.
pub fn dtor_GameObject(g: &mut G, this: Ptr) {
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    let msg = g.go(this).offset_0x94;
    if board != NULL && msg != -1 {
        crate::game::board::FUN_005384a0(g, board, msg);
    }
    g.go(this).field_0x28.clear();
    crate::sexy::widget::dtor_Widget(g, this);
}

/// port: 004e9df0 Sexy::GameObject::vfunction72
pub fn vfunction72(_g: &mut G, _this: Ptr) -> i32 {
    0
}

/// port: 004d68d0 Sexy::GameObject::vfunction73
/// Sell value: half the price, or -1 for an object bought less than a day (86400 s) ago
/// while `DAT_005e8f1c` <= 0.
pub fn vfunction73(g: &mut G, this: Ptr) -> i32 {
    if g.go(this).offset_0x89 && g.globals.DAT_005e8f1c < 1 {
        let now = g.now_time64; // __time64(NULL)
        let diff = now.wrapping_sub(g.go(this).field_0x48);
        let hi = (diff >> 32) as i32;
        if hi < 0 || (hi == 0 && (diff as u32) <= 0x15180) {
            return -1;
        }
    }
    g.go(this).field_0x58 / 2
}

/// port: 004e9e00 Sexy::GameObject::vfunction75
pub fn vfunction75(_g: &mut G, _this: Ptr) {}

/// port: 004f22c0 FUN_004f22c0
/// `GameObject::Update()` (non-virtual base part every subclass calls first): counts down
/// the timers and keeps the speech bubble (timed message `offset_0x94`) going.
pub fn FUN_004f22c0(g: &mut G, this: Ptr) {
    crate::sexy::widget_container::vfunction23(g, this);
    let go = g.go(this);
    if go.offset_0x80 != 0 {
        go.offset_0x80 -= 1;
    }
    if go.offset_0x84 != 0 {
        go.offset_0x84 -= 1;
    }
    if go.offset_0x70 != 0 {
        go.offset_0x70 -= 1;
    }
    if 0 < g.go(this).offset_0x8c {
        FUN_004d6cc0(g, this);
        let msg = g.go(this).offset_0x94;
        if msg == -1 {
            g.go(this).offset_0x8c += -1;
            if g.go(this).offset_0x8c == 0 {
                let (app, ty, arg) = (g.go(this).offset_0x0, g.go(this).offset_0x4, g.go(this).offset_0x9c);
                let board = g.wfa(app).offset_0x4;
                let id = crate::game::board_level::FUN_00540cc0(g, board, ty, arg);
                g.go(this).offset_0x94 = id;
                if id != -1 {
                    g.go(this).offset_0x8c = 1;
                }
            }
        } else {
            g.go(this).offset_0x8c += 1;
            let app = g.go(this).offset_0x0;
            let board = g.wfa(app).offset_0x4;
            if !crate::game::board::FUN_00538490(g, board, msg) {
                g.go(this).offset_0x8c = 0;
                g.go(this).offset_0x94 = -1;
            } else {
                let n = g.go(this).offset_0x8c;
                if n == 2 || n % 0x1e == 0 {
                    let s: Vec<u8> = Vec::new(); // std::string(&DAT_005ad7a6), an empty string
                    let (x, y, w) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30, g.wc(this).offset_0x34);
                    crate::game::board_level::FUN_005444e0(g, board, w / 2 + x - 0x14, y - 0x32, 1, &s);
                }
            }
        }
    }
    let go = g.go(this);
    if go.offset_0x98 != 0 {
        go.offset_0x98 -= 1;
    }
}

/// port: 004d6cc0 FUN_004d6cc0
/// While `offset_0x74` is set, keeps `offset_0x78` within 1..=10.
pub fn FUN_004d6cc0(g: &mut G, this: Ptr) {
    let go = g.go(this);
    if go.offset_0x74 {
        if go.offset_0x78 == 0 {
            go.offset_0x78 = 1;
            return;
        }
        if 10 < go.offset_0x78 {
            go.offset_0x78 = 10;
        }
    }
}

/// port: 004d6780 FUN_004d6780
/// `NewDay(__int64 day, __time64_t now)` (virtual-tank objects only, `offset_0x24 >= 0`):
/// drops the "bought today" flag a day after purchase; on a new day takes the days missed
/// (beyond one) off the +0x14c counter (not below 0).
pub fn FUN_004d6780(g: &mut G, this: Ptr, day: i64, now: i64) {
    let go = g.go(this);
    if go.offset_0x24 < 0 {
        return;
    }
    if go.offset_0x89 {
        let t0 = go.field_0x48;
        let keep = t0 <= now && now - t0 < 0x15180;
        if !keep {
            go.offset_0x89 = false;
        }
    }
    let (lo, hi) = (day as i32, (day >> 32) as i32);
    if lo != go.offset_0xb0 || hi != go.offset_0xb4 {
        let mut d = lo.wrapping_sub(go.offset_0xb8).wrapping_sub(1);
        if d < 0 {
            go.offset_0xb8 = lo;
            go.offset_0xbc = hi;
            d = 0;
        }
        go.offset_0xc4 -= d;
        go.offset_0xc0 = 0;
        go.offset_0xb0 = lo;
        go.offset_0xb4 = hi;
        if go.offset_0xc4 < 0 {
            go.offset_0xc4 = 0;
        }
    }
}

/// port: 004d6c50 FUN_004d6c50
/// Steps the "hungry" tint (+0xa0) toward or away from full (5), by +0xa8.
pub fn FUN_004d6c50(g: &mut G, this: Ptr) {
    let go = g.go(this);
    let v = go.offset_0x18;
    if 0 < v {
        if !go.offset_0x20 {
            go.offset_0x18 = v - 1;
        } else {
            go.offset_0x18 = v + 1;
            if 5 < v + 1 {
                go.offset_0x18 = 0;
            }
        }
    }
}

/// port: 004d6ef0 FUN_004d6ef0
/// One tick of hunger (not while a message is attached); turns hungry-looking just below
/// +0xa4, never below -1000.
pub fn FUN_004d6ef0(g: &mut G, this: Ptr) {
    if g.go(this).offset_0x94 == -1 {
        g.go(this).offset_0x14 -= 1;
    }
    let go = g.go(this);
    if go.offset_0x14 == go.offset_0x1c + 4 {
        FUN_004d6920(g, this, true);
        return;
    }
    if go.offset_0x14 < -1000 {
        go.offset_0x14 = -1000;
    }
}

/// port: 004d6920 FUN_004d6920
/// `SetHungryLook(bool)`: starts the tint fading in (or out); not in the screensaver.
pub fn FUN_004d6920(g: &mut G, this: Ptr, param_1: bool) {
    let app = g.go(this).offset_0x0;
    if crate::game::win_fish_app::thunk_FUN_00479fc0(g, app) {
        return;
    }
    g.go(this).offset_0x20 = param_1;
    if !param_1 {
        g.go(this).offset_0x18 = 5;
    } else {
        let board = g.wfa(app).offset_0x4;
        if g.go(this).offset_0xc0 < 3 || g.board(board).ext_0x4ff {
            g.go(this).offset_0x18 = 1;
        }
    }
}

/// port: 004d69d0 FUN_004d69d0
/// Starved: hunger below 1 (below -2160 / -4320 for pets in the +0x880 mode); never for
/// virtual-tank objects.
pub fn FUN_004d69d0(g: &mut G, this: Ptr) -> bool {
    if -1 < g.go(this).offset_0x24 {
        return false;
    }
    let app = g.go(this).offset_0x0;
    let h = g.go(this).offset_0x14;
    if g.wfa(app).offset_0x154 {
        let t = g.go(this).offset_0x4;
        if t != 7 && t != 9 && t != 8 {
            return h < -0x86f;
        }
        return h < -0x10df;
    }
    h < 1
}

/// port: 004d6f30 FUN_004d6f30
/// Looks hungry (hunger at most 300, tint settled); never in the screensaver.
pub fn FUN_004d6f30(g: &mut G, this: Ptr) -> bool {
    let app = g.go(this).offset_0x0;
    if crate::game::win_fish_app::thunk_FUN_00479fc0(g, app) {
        return false;
    }
    let board = g.wfa(app).offset_0x4;
    let go = g.go(this).clone();
    go.offset_0x14 < 0x12d && go.offset_0x18 == 0 && (go.offset_0xc0 < 3 || g.board(board).ext_0x4ff)
}

/// port: 004d6bd0 FUN_004d6bd0
/// May drop coins (not when starving in the +0x880 mode).
pub fn FUN_004d6bd0(g: &mut G, this: Ptr) -> bool {
    let app = g.go(this).offset_0x0;
    !(g.wfa(app).offset_0x154 && g.go(this).offset_0x14 < 1)
}

/// port: 004d6980 FUN_004d6980
/// Shows the pet-13 hunger marker: hunger below `param_1` while pet 13 is in the tank.
pub fn FUN_004d6980(g: &mut G, this: Ptr, param_1: i32) -> bool {
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    g.go(this).offset_0x14 < param_1 && g.board(board).field_0xb8[13] != 0 && (g.go(this).offset_0xc0 < 3 || g.board(board).ext_0x4ff)
}

/// port: 004d6a30 FUN_004d6a30
/// `Fed(bool)`: marks the game as in progress; unless `param_1`, raises hunger to at least
/// 300 (virtual tank or the +0x880 mode) and counts toward the +0x114 timer. Returns the
/// last intermediate value, as the original leaves it in EAX.
pub fn FUN_004d6a30(g: &mut G, this: Ptr, param_1: bool) -> i32 {
    let app = g.go(this).offset_0x0;
    let mut r = app as i32;
    g.go(this).offset_0x70 = 0;
    let board = g.wfa(app).offset_0x4;
    g.board(board).ext_0x4ee = true;
    if !param_1 {
        if -1 < g.go(this).offset_0x24 || g.wfa(app).offset_0x154 {
            r = 300;
            if g.go(this).offset_0x14 < 300 {
                g.go(this).offset_0x14 = 300;
            }
        }
        let go = g.go(this);
        if go.offset_0x88 {
            let n = go.offset_0x90;
            r = n / 5;
            go.offset_0x90 = n + 1;
            if n % 5 == 0 && go.offset_0x9c != 4 && go.offset_0x8c == 0 {
                r = go.offset_0x84.max(5);
                go.offset_0x8c = r;
            }
        }
    }
    r
}

/// port: 004d6b00 FUN_004d6b00
/// Opens the mouth (+0x108 for `param_1` updates, sound 0x10e) when food is near.
pub fn FUN_004d6b00(g: &mut G, this: Ptr, param_1: i32) {
    let go = g.go(this).clone();
    if go.offset_0x7c && (go.offset_0x9c != 4 || go.offset_0x94 != -1) && go.offset_0x80 == 0 {
        let app = go.offset_0x0;
        let board = g.wfa(app).offset_0x4;
        crate::game::board::FUN_00538230(g, board, 0x10e, 3, 1.0);
        let go = g.go(this);
        go.offset_0x80 = param_1;
        go.offset_0x84 = 0x50;
    }
}

/// port: 004d6c90 FUN_004d6c90
/// Advances the drop-in fade (+0x100) to 80, then ends it.
pub fn FUN_004d6c90(g: &mut G, this: Ptr) {
    let go = g.go(this);
    if 0 < go.offset_0x78 {
        go.offset_0x78 += 1;
        if 0x50 < go.offset_0x78 {
            go.offset_0x78 = 0;
        }
    }
}

/// port: 004d6db0 FUN_004d6db0
/// Colorized drawing in `param_2`, faded while dropping in (alpha ramps over +0x100).
pub fn FUN_004d6db0(g: &mut G, this: Ptr, param_1: &mut Graphics, param_2: Color) {
    crate::sexy::graphics::FUN_004558e0(param_1, true);
    if !g.go(this).offset_0x74 {
        crate::sexy::graphics::FUN_00455890(param_1, param_2);
        return;
    }
    let v = g.go(this).offset_0x78;
    let k = if v < 10 {
        (v * 0xff) / 10
    } else if v < 0x32 {
        0xff
    } else {
        ((0x50 - v) * 0xff) / 0x1e
    };
    let mut c = param_2;
    c.mAlpha = (param_2.mAlpha * k) / 0xff;
    crate::sexy::graphics::FUN_00455890(param_1, c);
}

/// port: 004d6d10 FUN_004d6d10
/// Colorized drawing in `param_2`; while dropping in, alpha pulses with +0x128.
pub fn FUN_004d6d10(g: &mut G, this: Ptr, param_1: &mut Graphics, param_2: Color) {
    crate::sexy::graphics::FUN_004558e0(param_1, true);
    if !g.go(this).offset_0x74 {
        crate::sexy::graphics::FUN_00455890(param_1, param_2);
        return;
    }
    let mut v = g.go(this).offset_0xa0 % 300;
    if 0x96 < v {
        v = 300 - v;
    }
    let mut c = param_2;
    c.mAlpha = (param_2.mAlpha * v) / 600 + 10;
    crate::sexy::graphics::FUN_00455890(param_1, c);
}

/// port: 004d6e70 FUN_004d6e70
/// The drop-in portal (backdrop-specific image) behind the object; true while it covers
/// the object.
pub fn FUN_004d6e70(g: &mut G, this: Ptr, param_1: &mut Graphics, param_2: Ptr, param_3: &Rect, param_4: bool) -> bool {
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    let bd = g.board(board).field_0x35c;
    if ((bd - 1) as u32) < 6 {
        let img = crate::sexy::res::FUN_005016a0(g, bd + 0x9e) as Ptr;
        let (x, y, t) = (g.wc(this).offset_0x2c - 2, g.wc(this).offset_0x30 - 2, g.wc(this).offset_0x24);
        FUN_005006a0(g, param_1, 0.0, 0.0, img, x, y, param_2, param_3, param_4, t);
    }
    g.go(this).offset_0x78 == 0
}

/// port: 005006a0 FUN_005006a0
/// `DrawPortal(Graphics*, float x, float y, Image* backdrop, int bx, int by, Image* sprite,
/// const Rect& src, bool mirror, int t)`: the creature's silhouette (the opaque pixels of
/// `src`, mirrored when asked) filled with the backdrop behind it at (`bx`, `by`), each row
/// shifted up or down a pixel by `sin(5 * (t + row))` degrees, drawn at (x, y). (A stack
/// `MemoryImage`; see `G::frame_temp_images`.)
pub fn FUN_005006a0(g: &mut G, param_1: &mut Graphics, param_2: f32, param_3: f32, param_4: Ptr, param_5: i32, param_6: i32, param_7: Ptr, param_8: &Rect, param_9: bool, param_10: i32) {
    let (w, h) = (param_8.mWidth, param_8.mHeight);
    let mem = crate::sexy::blit::new_memory_image(g, w, h);
    let (bw, bh) = (g.image(param_4).offset_0x20, g.image(param_4).offset_0x24);
    let sw = g.image(param_7).offset_0x20;
    let bg = std::mem::take(&mut g.image(param_4).mBits);
    let sp = if param_7 == param_4 { bg.clone() } else { std::mem::take(&mut g.image(param_7).mBits) };
    {
        let dst = &mut g.image(mem).mBits;
        let mut r = 0;
        while r < h {
            let ang = ((param_10 as f64 * 5.0 + r as f64 * 5.0) * 3.141590118408203 / 180.0) as f32;
            let s = (ang as f64).sin() as f32;
            let sy = crate::sexy::crt::ftol(s as f64 + (r + param_6) as f64) as i32;
            if -1 < sy && sy < bh {
                let mut bi = (bw * sy + param_5) as isize;
                let mut di = (w * r) as isize;
                let row = (param_8.mY + r) * sw + param_8.mX;
                let (mut si, step) = if param_9 { ((row + param_8.mWidth - 1) as isize, -1) } else { (row as isize, 1) };
                let mut bx = param_5;
                for _ in 0..w {
                    if -1 < bx && bx < bw {
                        let a = sp.get(si as usize).copied().unwrap_or(0);
                        si += step;
                        if a & 0xff000000 != 0 {
                            dst[di as usize] = bg.get(bi as usize).copied().unwrap_or(0);
                        }
                        di += 1;
                        bi += 1;
                    }
                    bx += 1;
                }
            }
            r += 1;
        }
    }
    if param_7 != param_4 {
        g.image(param_7).mBits = sp;
    }
    g.image(param_4).mBits = bg;
    let y = crate::sexy::crt::ftol(param_3 as f64) as i32;
    let x = crate::sexy::crt::ftol(param_2 as f64) as i32;
    crate::sexy::graphics::FUN_00455d20(param_1, g, mem, x, y);
    g.frame_temp_images.push(mem);
}

/// port: 004d5240 FUN_004d5240
/// `CanHaveBaby(int* slot)`: a bought (+0xac >= 0), grown (+0x1a0 == 2) breeder with no
/// baby yet, while the tank has a free slot (stored through `param_1`).
pub fn FUN_004d5240(g: &mut G, this: Ptr, param_1: Option<&mut i32>) -> bool {
    if g.go(this).offset_0x24 < 0 || g.breeder(this).offset_0x4c != 2 {
        return false;
    }
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    let slot = crate::game::store::FUN_0053a890(g, board);
    if slot < 0 || 0 < g.breeder(this).offset_0x88 {
        return false;
    }
    if let Some(p) = param_1 {
        *p = slot;
    }
    true
}

/// port: 004f21e0 FUN_004f21e0
/// Copies a creature's identity (name, purchase time, price and the other shop fields,
/// the flags at +0xf5/+0xf6/+0xfc/+0x104/+0x110) from `param_1`.
pub fn FUN_004f21e0(g: &mut G, this: Ptr, param_1: Ptr) {
    let s = g.go(param_1).clone();
    let d = g.go(this);
    d.field_0x28 = s.field_0x28.clone();
    d.field_0x48 = s.field_0x48;
    d.field_0x58 = s.field_0x58;
    d.offset_0x50 = s.offset_0x50;
    d.offset_0x54 = s.offset_0x54;
    d.offset_0x5c = s.offset_0x5c;
    d.offset_0x68 = s.offset_0x68;
    d.offset_0x6d = s.offset_0x6d;
    d.offset_0x6e = s.offset_0x6e;
    d.offset_0x74 = s.offset_0x74;
    d.offset_0x7c = s.offset_0x7c;
    d.offset_0x88 = s.offset_0x88;
}

/// port: 004d6f90 FUN_004d6f90
/// Counts a feeding toward today's three (virtual tank): the third raises +0x14c (to 6)
/// and remembers the day.
pub fn FUN_004d6f90(g: &mut G, this: Ptr) -> bool {
    let now = g.now_time64;
    let day = crate::game::win_fish_app::FUN_005005b0(g);
    FUN_004d6780(g, this, day, now);
    let go = g.go(this);
    if go.offset_0xc0 < 3 {
        go.offset_0xc0 += 1;
        if go.offset_0xc0 == 3 {
            go.offset_0xc4 += 1;
            go.offset_0xb8 = day as i32;
            go.offset_0xbc = (day >> 32) as i32;
            if 6 < go.offset_0xc4 {
                go.offset_0xc4 = 6;
            }
        }
        return true;
    }
    false
}

/// port: 004d7020 FUN_004d7020
/// Advances the +0x90 cycle (20 steps).
pub fn FUN_004d7020(g: &mut G, this: Ptr) -> i32 {
    let n = g.go(this).offset_0x8 + 1;
    g.go(this).offset_0x8 = n % 0x14;
    n / 0x14
}

/// port: 004d7040 FUN_004d7040
/// The sparkle (IMAGE `DAT_005e8e1c`, cel +0x90 / 4) of an object carrying something.
pub fn FUN_004d7040(g: &mut G, this: Ptr, param_1: &mut Graphics, param_2: i32, param_3: i32) {
    crate::sexy::graphics::FUN_004558c0(param_1, 1);
    let img = g.res.DAT_005e8e1c;
    let cel = g.go(this).offset_0x8 / 4;
    crate::sexy::graphics::FUN_00456950(param_1, g, img, param_2, param_3, cel);
    crate::sexy::graphics::FUN_004558c0(param_1, 0);
}

/// port: 004d7100 FUN_004d7100
/// Whether a timer (`*param_1`) has reached `param_2`; in the virtual tank only for owned
/// objects, with the period shortened by +0x14c days (not below 360) and reset rules.
pub fn FUN_004d7100(g: &mut G, this: Ptr, param_1: &mut i32, mut param_2: i32) -> bool {
    let app = g.go(this).offset_0x0;
    if g.wfa(app).offset_0x150 != 5 {
        return param_2 <= *param_1;
    }
    if -1 < g.go(this).offset_0x24 {
        let board = g.wfa(app).offset_0x4;
        if !crate::game::win_fish_app::thunk_FUN_00479fc0(g, app) || g.board(board).field_0x374 <= g.globals.DAT_005df590 {
            if 0x168 < param_2 {
                param_2 += g.go(this).offset_0xc4 * -0x21c;
                if param_2 < 0x168 {
                    param_2 = 0x168;
                }
            }
            if g.board(board).ext_0x500 {
                if g.go(this).offset_0x14 < 0 {
                    if param_2 < *param_1 {
                        *param_1 = param_2 - 0x5a;
                    }
                    return false;
                }
                return param_2 <= *param_1;
            }
            if param_2 < *param_1 {
                *param_1 = 0;
            }
        }
    }
    false
}

/// port: 004e13d0 FUN_004e13d0
/// After eating: fades the hungry tint out once no longer hungry-looking.
pub fn FUN_004e13d0(g: &mut G, this: Ptr, param_1: bool) {
    if param_1 && !FUN_004d6f30(g, this) {
        FUN_004d6920(g, this, false);
    }
}

/// port: 004d6cf0 FUN_004d6cf0
/// Screensaver tick: +0x12c, and +0x128 while dropping in.
pub fn FUN_004d6cf0(g: &mut G, this: Ptr) {
    let go = g.go(this);
    go.offset_0xa4 += 1;
    if go.offset_0x74 {
        go.offset_0xa0 += 1;
    }
}

/// port: 004d5300 Sexy::OtherTypePet::vfunction57
/// `MouseUp(x, y, clicks)` shared by the GameObject classes (the decompiler named the folded
/// copy after OtherTypePet): `Widget::MouseUp`, then ends the board's hold-to-feed and
/// hold-to-fire.
pub fn vfunction57__004d5300(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32) {
    crate::sexy::widget::vfunction57(g, this, param_1, param_2, param_3);
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    g.board(board).ext_0x4ec = false;
    let board = g.wfa(app).offset_0x4;
    g.board(board).ext_0x4ed = false;
}

/// port: 004d8e40 Sexy::Grubber::vfunction77
/// `SetPos(int x, int y)` shared by the creatures: the widget position and the creature's
/// own x / y (+0x158 / +0x160).
pub fn vfunction77__004d8e40(g: &mut G, this: Ptr, param_1: i32, param_2: i32) {
    g.wc(this).offset_0x2c = param_1;
    g.wc(this).offset_0x30 = param_2;
    match &mut g.go_ext(this).sub {
        GoSub::Fish(f) | GoSub::FishTypePet(f, _) | GoSub::BiFish(f, _) => {
            f.field_0x4 = param_1 as f64;
            f.field_0xc = param_2 as f64;
        }
        GoSub::Penta(d) => {
            d.offset_0x4 = param_1 as f64;
            d.offset_0xc = param_2 as f64;
        }
        GoSub::Grubber(d) => {
            d.offset_0x4 = param_1 as f64;
            d.offset_0xc = param_2 as f64;
        }
        GoSub::Missle(d) => {
            d.offset_0x4 = param_1 as f64;
            d.offset_0xc = param_2 as f64;
        }
        GoSub::Breeder(d) => {
            d.field_0x4 = param_1 as f64;
            d.field_0xc = param_2 as f64;
        }
        GoSub::OtherTypePet(d) => {
            d.offset_0x4 = param_1 as f64;
            d.offset_0xc = param_2 as f64;
        }
        s => panic!("vfunction77__004d8e40: no x / y at +0x158 known for {s:?}"),
    }
}

/// port: 004d7dc0 Sexy::Grubber::vfunction55
/// `MouseDown(x, y, clicks)` shared by the creatures: a right click goes to the tank.
pub fn vfunction55__004d7dc0(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32) {
    if param_3 < 0 {
        let app = g.go(this).offset_0x0;
        let board = g.wfa(app).offset_0x4;
        let (mx, my) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
        crate::game::board_level::FUN_0053bfb0(g, board, mx + param_1, my + param_2);
    }
}

/// port: 004d9000 Sexy::Grubber::vfunction79
/// The creatures' portrait animation: the age counters, then the portrait cel (+0x130)
/// from half the age, in 10s.
pub fn vfunction79__004d9000(g: &mut G, this: Ptr) {
    FUN_004d6cf0(g, this);
    let go = g.go(this);
    let i = go.offset_0xa4 / 2;
    go.offset_0xa8 = i % 10;
}

/// port: 004d6330 Sexy::GameObject::vfunction81
/// `Sync(DataSync&)`: registers the object for pointer fix-ups, then its type, the carried
/// missile (a pointer), the timers, the drop-in timer (a byte) and the widget rectangle
/// and flags; for virtual-tank objects (+0xac >= 0) also the name, the mood and growth
/// counters (the 64-bit ones as their low 32 bits) and the speech bubble id.
pub fn vfunction81(g: &mut G, this: Ptr, sync: &mut crate::sexy::data_sync::DataSync) -> crate::sexy::data_sync::SyncResult {
    use crate::sexy::data_sync::{DataIo, FUN_00502ff0, FUN_00503010, FUN_00503040, FUN_00503070, FUN_00505860, FUN_00512270, FUN_005122f0, PtrSlot};
    FUN_00512270(sync, this);
    let mut go = g.go(this).clone();
    let mut wc = g.wc(this).clone();
    let mut w = g.w(this).clone();
    let reading = matches!(sync.io, DataIo::Read(_));
    let r = (|| {
        FUN_00503010(sync, &mut go.offset_0x4)?;
        FUN_00503010(sync, &mut go.offset_0x8)?;
        FUN_005122f0(sync, PtrSlot::GameObjectMissle(this));
        FUN_00503010(sync, &mut go.offset_0x14)?;
        FUN_00503010(sync, &mut go.offset_0x18)?;
        FUN_00503010(sync, &mut go.offset_0x1c)?;
        FUN_00503070(sync, &mut go.offset_0x20)?;
        FUN_00503040(sync, &mut go.offset_0x98)?;
        FUN_00503010(sync, &mut wc.offset_0x2c)?;
        FUN_00503010(sync, &mut wc.offset_0x30)?;
        FUN_00503010(sync, &mut wc.offset_0x34)?;
        FUN_00503010(sync, &mut wc.offset_0x38)?;
        FUN_00503070(sync, &mut w.offset_0x0)?;
        FUN_00503070(sync, &mut w.offset_0x1)?;
        FUN_00503070(sync, &mut w.offset_0x2)?;
        FUN_00503070(sync, &mut w.offset_0x28)?;
        FUN_00503010(sync, &mut go.offset_0x24)?;
        if -1 < go.offset_0x24 {
            FUN_00505860(sync, &mut go.field_0x28)?;
            FUN_00503070(sync, &mut go.offset_0x6c)?;
            FUN_00503070(sync, &mut go.offset_0x89)?;
            if reading {
                go.field_0x48 = 0;
            }
            let mut lo = go.field_0x48 as i32;
            FUN_00503010(sync, &mut lo)?;
            go.field_0x48 = (go.field_0x48 & !0xffff_ffff) | (lo as u32 as i64);
            FUN_00503010(sync, &mut go.field_0x58)?;
            FUN_00503010(sync, &mut go.offset_0x54)?;
            FUN_00503040(sync, &mut go.offset_0x50)?;
            FUN_00503010(sync, &mut go.offset_0x68)?;
            FUN_00503070(sync, &mut go.offset_0x6d)?;
            FUN_00503070(sync, &mut go.offset_0x6e)?;
            FUN_00503010(sync, &mut go.offset_0x70)?;
            FUN_00503070(sync, &mut go.offset_0x74)?;
            FUN_00503010(sync, &mut go.offset_0x78)?;
            FUN_00503070(sync, &mut go.offset_0x7c)?;
            FUN_00503010(sync, &mut go.offset_0x80)?;
            FUN_00503010(sync, &mut go.offset_0x84)?;
            FUN_00503070(sync, &mut go.offset_0x88)?;
            FUN_00503010(sync, &mut go.offset_0x8c)?;
            FUN_00503010(sync, &mut go.offset_0x90)?;
            FUN_00503010(sync, &mut go.offset_0x94)?;
            if reading {
                go.offset_0xb0 = 0;
                go.offset_0xb4 = 0;
            }
            FUN_00503010(sync, &mut go.offset_0xb0)?;
            if reading {
                go.offset_0xb8 = 0;
                go.offset_0xbc = 0;
            }
            FUN_00503010(sync, &mut go.offset_0xb8)?;
            FUN_00503010(sync, &mut go.offset_0xc0)?;
            FUN_00503010(sync, &mut go.offset_0xc4)?;
            FUN_00503010(sync, &mut go.offset_0xc8)?;
            let mut b = go.offset_0x9c.to_le_bytes();
            FUN_00502ff0(sync, &mut b, 4)?;
            go.offset_0x9c = i32::from_le_bytes(b);
            for v in go.offset_0x5c.iter_mut() {
                FUN_00503010(sync, v)?;
            }
        }
        Ok(())
    })();
    // The carried missile is filled in later by the pointer fix-up; keep the current one.
    go.offset_0x10 = g.go(this).offset_0x10;
    *g.go(this) = go;
    *g.wc(this) = wc;
    *g.w(this) = w;
    r
}

/// The current value of a pointer field synced with `SyncPointer`.
pub fn ptr_slot_get(g: &mut G, slot: crate::sexy::data_sync::PtrSlot) -> Ptr {
    use crate::sexy::data_sync::PtrSlot;
    match slot {
        PtrSlot::GameObjectMissle(p) => g.go(p).offset_0x10,
        PtrSlot::MissleTarget(p) => g.missle(p).offset_0x50,
        PtrSlot::ShadowOwner(p) => g.shadow(p).offset_0x4,
        PtrSlot::CoinOffset0x3c(p) => g.coin(p).offset_0x3c,
    }
}

/// Fills a pointer field synced with `SyncPointer`.
pub fn ptr_slot_set(g: &mut G, slot: crate::sexy::data_sync::PtrSlot, v: Ptr) {
    use crate::sexy::data_sync::PtrSlot;
    match slot {
        PtrSlot::GameObjectMissle(p) => g.go(p).offset_0x10 = v,
        PtrSlot::MissleTarget(p) => g.missle(p).offset_0x50 = v,
        PtrSlot::ShadowOwner(p) => g.shadow(p).offset_0x4 = v,
        PtrSlot::CoinOffset0x3c(p) => g.coin(p).offset_0x3c = v,
    }
}

/// port: 004d7210 FUN_004d7210
/// Adds a creature to the shop's counts: a bought creature (+0xf4) counts itself
/// (`CountKill`, vtable +0x118) unless it is a feeder (+0xf0): feeder kinds 3/4/5 count in
/// slots 6/7/5, 1000/1001/1002 in slots 0/3/4.
pub fn FUN_004d7210(g: &mut G, this: Ptr, param_1: &mut [i32]) {
    if !g.go(this).offset_0x6c {
        return;
    }
    match g.go(this).offset_0x68 {
        0 => vcall!(g, this, go.vfunction71, param_1),
        3 => param_1[6] += 1,
        4 => param_1[7] += 1,
        5 => param_1[5] += 1,
        1000 => param_1[0] += 1,
        0x3ed => param_1[3] += 1,
        0x3ee => param_1[4] += 1,
        _ => {}
    }
}

/// port: 004d6720 FUN_004d6720
/// `InitVirtualStats()` for a bought creature: fed and cared-for today (yesterday's day
/// number as the last check), mood 3, a random 0..999 counter and 0..11 phase, then its
/// preferences.
pub fn FUN_004d6720(g: &mut G, this: Ptr) {
    let day = crate::game::win_fish_app::FUN_005005b0(g);
    let go = g.go(this);
    go.offset_0xb0 = day as i32;
    go.offset_0xb4 = (day >> 32) as i32;
    let prev = day.wrapping_sub(1);
    go.offset_0xb8 = prev as i32;
    go.offset_0xbc = (prev >> 32) as i32;
    go.offset_0xc4 = 3;
    let r = crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32;
    g.go(this).offset_0xc8 = r % 1000;
    let r = crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32;
    g.go(this).offset_0x50 = r % 0xc;
    FUN_004d66a0(g, this);
}

/// port: 004d66a0 FUN_004d66a0
/// `PickPreferences()`: a random 1..374 (+0xdc) and three distinct 0..330 picks (+0xe4..).
/// (The original returns a leftover register; no caller reads it.)
pub fn FUN_004d66a0(g: &mut G, this: Ptr) {
    let r = crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32;
    g.go(this).offset_0x54 = r % 0x176 + 1;
    let mut n = 0;
    while n < 3 {
        let v = crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32 % 0x14b;
        g.go(this).offset_0x5c[n] = v;
        if g.go(this).offset_0x5c[..n].contains(&v) {
            continue;
        }
        n += 1;
    }
}

/// port: 004e1400 FUN_004e1400
/// `DrawName(Graphics*, bool force)`: in the virtual tank (board +0x4fc), a named creature's
/// name centered under it (yellow when hungry, with the hunger flash over it); a +0xfc
/// creature only while +0x100 is set or when forced.
pub fn FUN_004e1400(g: &mut G, this: Ptr, param_1: &mut Graphics, param_2: bool) {
    use crate::sexy::graphics::{FUN_00455870, FUN_00455880, FUN_00455890, FUN_00455cf0};
    use crate::sexy::types::FUN_00433320;
    if g.go(this).field_0x28.is_empty() {
        return;
    }
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    if !g.board(board).ext_0x4fc {
        return;
    }
    if g.go(this).offset_0x74 && g.go(this).offset_0x78 == 0 && !param_2 {
        return;
    }
    let dy = vcall!(g, this, go.vfunction72);
    let c = if FUN_004d6f30(g, this) { FUN_00433320(0xd6cf29) } else { FUN_00433320(0xffffff) };
    FUN_00455890(param_1, c);
    let f = g.res.DAT_005e8cf0;
    FUN_00455880(param_1, f);
    let name = g.go(this).field_0x28.clone();
    let (w, h) = (g.wc(this).offset_0x34, g.wc(this).offset_0x38);
    let font = FUN_00455870(param_1);
    let sw = crate::sexy::image_font::string_width(g, font, &name);
    FUN_00455cf0(param_1, g, &name, (w - sw) / 2, h - 2 + dy);
    let t = g.go(this).offset_0x18;
    if t != 0 {
        let mut c = FUN_00433320(0xd6cf29);
        c.mAlpha = (t * 0xff) / 5;
        FUN_00455890(param_1, c);
        let font = FUN_00455870(param_1);
        let sw = crate::sexy::image_font::string_width(g, font, &name);
        FUN_00455cf0(param_1, g, &name, (w - sw) / 2, h - 2 + dy);
    }
}

/// port: 004d65f0 FUN_004d65f0
/// The creature's mood (+0x14c) in words; the happiest picks one of five by +0x150.
pub fn FUN_004d65f0(g: &mut G, this: Ptr) -> &'static [u8] {
    let go = g.go(this);
    match go.offset_0xc4 {
        0 => b"Horribly Depressed",
        1 => b"Feeling Neglected",
        2 => b"Quite Hungry",
        3 => b"Contented",
        4 => b"Happy",
        5 => b"Chipper",
        6 => {
            let t: [&'static [u8]; 5] = [b"Super Pumped", b"Lovin' It!", b"High on Life", b"Totally Stoked", b"Fish-tastic!"];
            t[(go.offset_0xc8 % 5) as usize]
        }
        _ => b"Unknown",
    }
}

/// port: 004e9ec0 FUN_004e9ec0
/// `FindPrey(x, y, kind)`: for a creature fed on other creatures (+0xf0 >= 1000), the
/// nearest live creature of that kind (not itself), measured from its center to (x, y);
/// opens the mouth when close (a Carnivore-like kind 6 from further away).
pub fn FUN_004e9ec0(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32) -> Ptr {
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    let objs = g.board(board).offset_0x7c.clone();
    let mut found = NULL;
    let mut best = 100000000;
    let mut it = objs.first().copied();
    while let Some(o) = it {
        let go = g.go(o).clone();
        if go.offset_0x24 < 0 && go.offset_0x98 < 1 && go.offset_0x4 == param_3 && o != this {
            let wc = g.wc(o).clone();
            let dx = (param_1 - wc.offset_0x34 / 2) - wc.offset_0x2c;
            let dy = (param_2 - wc.offset_0x38 / 2) - wc.offset_0x30;
            let d = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx));
            if d < best {
                found = o;
                best = d;
            }
        }
        it = crate::game::store::FUN_004e3e30(&objs, o);
    }
    if g.go(this).offset_0x4 == 6 && best < 40000 {
        FUN_004d6b00(g, this, 0xaf);
    } else if param_3 == 6 && best <= 0x57e3 {
        FUN_004d6b00(g, this, 0x96);
    }
    if best < 10000 {
        FUN_004d6b00(g, this, 0x96);
        g.go(this).offset_0x70 = 100;
    }
    found
}

/// port: 004ea010 FUN_004ea010
/// `FindFood(x, y)` for a creature with a food preference (+0xf0): prey (>= 1000), else the
/// nearest uneaten, unclaimed pellet of that kind (kind 6 eats kind 0); opens the mouth
/// when close.
pub fn FUN_004ea010(g: &mut G, this: Ptr, param_1: i32, param_2: i32) -> Ptr {
    let mut kind = g.go(this).offset_0x68;
    if 999 < kind {
        return FUN_004e9ec0(g, this, param_1, param_2, kind - 1000);
    }
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    let mut best = 100000000;
    let mut found = NULL;
    if kind == 6 {
        kind = 0;
    }
    for f in g.board(board).offset_0xc[crate::game::board_level::vec_index(0xac)].clone() {
        let fd = g.food(f).clone();
        if fd.offset_0x40 == kind && !fd.offset_0x31 && fd.offset_0x44 == 0 {
            let dx = (g.wc(f).offset_0x2c - param_1) + 0x14;
            let dy = (g.wc(f).offset_0x30 - param_2) + 0x14;
            let d = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx));
            if d < best {
                best = d;
                found = f;
            }
        }
    }
    if g.go(this).offset_0x4 == 6 && best < 40000 {
        FUN_004d6b00(g, this, 0xaf);
    }
    if best < 10000 {
        FUN_004d6b00(g, this, 0x96);
        g.go(this).offset_0x70 = 100;
    }
    found
}

/// port: 004ea160 FUN_004ea160
/// `TryEatPrey(x, y, kind)`: a live creature of that kind under the mouth is eaten
/// (vfunction 78, then it leaves the tank): 1; one close by: 2; else 0.
pub fn FUN_004ea160(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32) -> i32 {
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    let objs = g.board(board).offset_0x7c.clone();
    let mut r = 0;
    let mut it = objs.first().copied();
    while let Some(o) = it {
        let go = g.go(o).clone();
        if go.offset_0x24 < 0 && go.offset_0x98 < 1 && go.offset_0x4 == param_3 && o != this {
            let wc = g.wc(o).clone();
            let hw = wc.offset_0x34 / 2;
            let hh = wc.offset_0x38 / 2;
            let dx = (-hw - wc.offset_0x2c) + param_1;
            let dy = (-hh - wc.offset_0x30) + param_2;
            if -hw < dx && dx < hw && -hh < dy && dy < hh {
                vcall!(g, this, go.vfunction78, o as i32);
                vcall!(g, o, go.vfunction76);
                return 1;
            }
            if -(hw + 0x14) < dx && dx < hw + 0x14 && -(hh + 0x14) < dy && dy < hh + 0x14 {
                r = 2;
            }
        }
        it = crate::game::store::FUN_004e3e30(&objs, o);
    }
    r
}

/// port: 004ea2a0 FUN_004ea2a0
/// `TryEatFood(x, y)` for a creature with a food preference: prey (>= 1000), else a pellet
/// of its kind under the mouth is eaten (vfunction 78, then it goes): 1; one close by: 2.
pub fn FUN_004ea2a0(g: &mut G, this: Ptr, param_1: i32, param_2: i32) -> i32 {
    let mut kind = g.go(this).offset_0x68;
    if 999 < kind {
        return FUN_004ea160(g, this, param_1, param_2, kind - 1000);
    }
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    if kind == 6 {
        kind = 0;
    }
    let mut r = 0;
    for f in g.board(board).offset_0xc[crate::game::board_level::vec_index(0xac)].clone() {
        let fd = g.food(f).clone();
        if fd.offset_0x40 == kind && !fd.offset_0x31 && fd.offset_0x44 == 0 {
            let dx = g.wc(f).offset_0x2c - param_1;
            let dy = g.wc(f).offset_0x30 - param_2;
            let dy2 = dy + 0x14;
            if ((dx + 0x22) as u32) < 0x1d && -0x14 < dy2 && dy2 < 0x14 {
                vcall!(g, this, go.vfunction78, f as i32);
                crate::game::larva::vfunction76(g, f);
                return 1;
            }
            if ((dx + 0x31) as u32) < 0x3b && ((dy + 0x2c) as u32) < 0x31 {
                r = 2;
            }
        }
    }
    r
}
