//! `Sexy::Penta`: the starcatcher, crawling on the floor and eating stars. `Penta_data`
//! starts at object offset 0x154 (field names relative to it); the object is 0x1b0 bytes.

use crate::game::board_level::vec_index;
use crate::game::game_object::{FUN_004d6cc0, FUN_004d6db0, FUN_004d6f30, FUN_004d7040, GameObject};
use crate::sexy::crt::ftol;
use crate::sexy::graphics::{FUN_004558e0, FUN_00455e40, FUN_004560a0, FUN_004563d0, FUN_00456950};
use crate::sexy::prelude::*;
use crate::sexy::types::CRect;

/// `Penta_data` (object offset 0x154).
#[derive(Debug, Clone, Default)]
pub struct Penta_data {
    /// +0x158 x.
    pub offset_0x4: f64,
    /// +0x160 y.
    pub offset_0xc: f64,
    /// +0x168 x speed.
    pub offset_0x14: f64,
    /// +0x170 y speed (sinking by 0.4).
    pub offset_0x1c: f64,
    /// +0x178 speed divisor (2.5..2.7).
    pub offset_0x24: f64,
    /// +0x180 bubbles left to blow (45 at first).
    pub offset_0x2c: i32,
    /// +0x188 the x speed the current mode wants.
    pub offset_0x34: f64,
    /// +0x190 crawl mode (0..8).
    pub offset_0x3c: i32,
    /// +0x194 updates since the last steering.
    pub offset_0x40: i32,
    /// +0x198 updates since the last mode roll.
    pub offset_0x44: i32,
    /// +0x19c animation counter.
    pub offset_0x48: i32,
    /// +0x1a0 cel.
    pub offset_0x4c: i32,
    /// +0x1a4 stars eaten toward the next pearl (virtual tank: 3).
    pub offset_0x50: i32,
    /// +0x1a8.
    pub offset_0x54: i32,
}

impl G {
    pub fn penta(&mut self, p: Ptr) -> &mut Penta_data {
        match &mut self.go_ext(p).sub {
            GoSub::Penta(d) => d,
            s => panic!("{p} is not a Penta: {s:?}"),
        }
    }
}

fn alloc_penta(g: &mut G, wc: crate::sexy::object::WidgetContainer_data, w: crate::sexy::object::Widget_data,
               go: crate::game::game_object::GameObject_data, d: Penta_data) -> Ptr {
    g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__Penta_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::GameObject(Box::new(GameObjectExt { go, sub: GoSub::Penta(d) })) }),
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

/// port: 004ec040 Sexy::Penta::Penta
/// `Penta::Penta()` (used before loading from a save).
pub fn Penta__004ec040(g: &mut G) -> Ptr {
    let (mut wc, w, mut go) = GameObject(g);
    go.offset_0x4 = 8;
    wc.offset_0x3d = false;
    alloc_penta(g, wc, w, go, Penta_data::default())
}

/// port: 004ec080 Sexy::Penta::Penta
/// `Penta(int x)`: on the floor at x.
pub fn Penta__004ec080(g: &mut G, param_1: i32) -> Ptr {
    let (wc, w, go) = GameObject(g);
    let this = alloc_penta(g, wc, w, go, Penta_data::default());
    FUN_004d8d00(g, this);
    g.penta(this).offset_0x4 = param_1 as f64;
    let x = g.penta(this).offset_0x4;
    g.wc(this).offset_0x2c = ftol(x) as i32;
    this
}

/// port: 004ec0f0 Sexy::Penta::Penta
/// `Penta(int x, int y)`.
pub fn Penta__004ec0f0(g: &mut G, param_1: i32, param_2: i32) -> Ptr {
    let (wc, w, go) = GameObject(g);
    let this = alloc_penta(g, wc, w, go, Penta_data::default());
    FUN_004d8d00(g, this);
    g.penta(this).offset_0x4 = param_1 as f64;
    g.wc(this).offset_0x2c = param_1;
    g.wc(this).offset_0x30 = param_2;
    g.penta(this).offset_0xc = param_2 as f64;
    this
}

/// port: 004d8d00 FUN_004d8d00
/// `Penta::Init()`: type 8, 80x80 on the floor (y 360..364), still, a random speed
/// divisor, hunger 900..1099, a random crawl mode, 45 bubbles; mouse-visible as pets are.
pub fn FUN_004d8d00(g: &mut G, this: Ptr) {
    g.wc(this).offset_0x3d = false;
    g.go(this).offset_0x4 = 8;
    let r = rand(g, this);
    g.penta(this).offset_0xc = (r % 5 + 0x168) as f64;
    let y = g.penta(this).offset_0xc;
    {
        let d = g.penta(this);
        d.offset_0x14 = 0.0;
        d.offset_0x1c = 0.0;
        d.offset_0x34 = 0.0;
    }
    let wc = g.wc(this);
    wc.offset_0x30 = ftol(y) as i32;
    wc.offset_0x34 = 0x50;
    wc.offset_0x38 = 0x50;
    let r = rand(g, this);
    g.penta(this).offset_0x24 = match r % 3 {
        0 => f64::from_bits(0x400599999999999a),
        1 => 2.5,
        _ => f64::from_bits(0x4004cccccccccccd),
    };
    let r = rand(g, this);
    g.go(this).offset_0x14 = (r % 200) as i32 + 900;
    let r = rand(g, this);
    {
        let d = g.penta(this);
        d.offset_0x44 = 0;
        d.offset_0x48 = 0;
        d.offset_0x4c = 0;
        d.offset_0x50 = 0;
        d.offset_0x40 = 0x28;
        d.offset_0x54 = 1;
        d.offset_0x3c = (r % 10) as i32;
    }
    g.w(this).offset_0x1 = g.globals.DAT_005e8f14;
    g.penta(this).offset_0x2c = 0x2d;
}

/// port: 004ec170 Sexy::Penta::~Penta
pub fn dtor_Penta(g: &mut G, this: Ptr) {
    g.set_vt(this, &crate::sexy::vtables_gen::Sexy__Penta_vftable);
    crate::game::game_object::dtor_GameObject(g, this);
}

/// port: 004eff20 Sexy::Penta::deleting_destructor
pub fn deleting_destructor(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_Penta(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 004d8c20 Sexy::Penta::vfunction81
/// `Sync(DataSync&)`.
pub fn vfunction81(g: &mut G, this: Ptr, sync: &mut crate::sexy::data_sync::DataSync) -> crate::sexy::data_sync::SyncResult {
    crate::game::game_object::vfunction81(g, this, sync)?;
    use crate::sexy::data_sync::{FUN_00503010, FUN_005030a0};
    let mut d = g.penta(this).clone();
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
    *g.penta(this) = d;
    Ok(())
}

/// port: 004ec060 Sexy::Penta::vfunction72
/// The shadow kind (4).
pub fn vfunction72(_g: &mut G, _this: Ptr) -> i32 {
    4
}

/// port: 004ec070 Sexy::Penta::vfunction71
/// Counts a starcatcher in `stats[1]`.
pub fn vfunction71(_g: &mut G, _this: Ptr, stats: &mut [i32]) {
    stats[1] += 1;
}

/// port: 004f3ee0 Sexy::Penta::vfunction76
/// `Remove()`: leaves the tank with its shadow.
pub fn vfunction76(g: &mut G, this: Ptr) {
    FUN_004f3ad0(g, this, true);
}

/// port: 004f3ad0 FUN_004f3ad0
/// `Remove(bool withShadow)`: drops what it carries, leaves the widget manager and the
/// board, optionally its shadow; counts a lost starcatcher (+0x490). When it starved: in
/// the adventure on level 2-1 a warning (with hints the first times); in the +0x880 mode
/// "The purple guys eat coins!".
pub fn FUN_004f3ad0(g: &mut G, this: Ptr, param_1: bool) {
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
    g.board(board).ext_0x460[(0x490 - 0x460) / 4] += 1;
    if !g.wfa(app).offset_0x154 {
        if crate::game::board_update::FUN_00537bb0(g, board, 2, 1) && crate::game::game_object::FUN_004d69d0(g, this) {
            if g.board(board).ext_0x4b0[0x23] != 0 {
                FUN_0053e950(g, board, b"Hint: Star potions allow guppies to produce stars!", false, 0x24);
            }
            if g.board(board).ext_0x4b0[0x22] != 0 {
                FUN_0053e950(g, board, b"Hint: Starcatchers need stars to stay alive!", false, 0x23);
            }
            FUN_0053e950(g, board, b"Warning! Your starcatcher has died!", false, 0x22);
        }
    } else if crate::game::game_object::FUN_004d69d0(g, this) {
        FUN_0053e950(g, board, b"The purple guys eat coins!", false, -1);
    }
}

/// port: 004f3c60 FUN_004f3c60
/// `Die(bool sound)`: the death sound, removal (keeping the shadow) and a dead body where
/// it was, facing the way it crawled.
pub fn FUN_004f3c60(g: &mut G, this: Ptr, param_1: bool) {
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    if param_1 {
        let t = g.go(this).offset_0x4;
        crate::game::board_level::FUN_00538610(g, board, t);
    }
    FUN_004f3ad0(g, this, false);
    let d = g.penta(this).clone();
    let right = 0.0 < d.offset_0x14;
    let shadow = g.go(this).offset_0xc;
    let kind = g.go(this).offset_0x4;
    let y = ftol(d.offset_0xc) as i32;
    let x = ftol(d.offset_0x4) as i32;
    let board = g.wfa(app).offset_0x4;
    crate::game::board_level::FUN_005441f0(g, board, x, y, d.offset_0x14, 0.0, d.offset_0x24, kind, right, shadow);
}

/// port: 004d6bf0 FUN_004d6bf0
/// A coin a starcatcher eats: stars (3, 10 and 0x12) not yet collected; silver and gold
/// coins (1, 2) in the +0x880 mode.
pub fn FUN_004d6bf0(g: &mut G, this: Ptr, param_1: Ptr) -> bool {
    let c = g.coin(param_1).clone();
    if c.offset_0x44 {
        return false;
    }
    let k = c.offset_0x40;
    let app = g.go(this).offset_0x0;
    if !g.wfa(app).offset_0x154 {
        k == 3 || k == 10 || k == 0x12
    } else {
        k == 2 || k == 1
    }
}

/// port: 004ec180 FUN_004ec180
/// `FindStar()`: the nearest edible coin; within 100 pixels +0xf8 = 100. In the virtual
/// tank's feeding mode, the feeder's target instead.
pub fn FUN_004ec180(g: &mut G, this: Ptr) -> Ptr {
    if g.go(this).offset_0x68 != 0 {
        let wc = g.wc(this).clone();
        return crate::game::game_object::FUN_004ea010(g, this, wc.offset_0x34 / 2 + wc.offset_0x2c, wc.offset_0x38 / 2 + wc.offset_0x30);
    }
    let board = board_of(g, this);
    let mut best = 100000000;
    let mut star = NULL;
    for c in g.board(board).offset_0xc[vec_index(0xa8)].clone() {
        if FUN_004d6bf0(g, this, c) {
            let d = g.penta(this).clone();
            let (cx, cy) = (g.wc(c).offset_0x2c, g.wc(c).offset_0x30);
            let dx = ftol((cx + 0x24) as f64 - (d.offset_0x4 + 40.0)) as i32;
            let dy = ftol((cy + 0x24) as f64 - (d.offset_0xc + 40.0)) as i32;
            let n = dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx));
            if n < best {
                star = c;
                best = n;
            }
        }
    }
    if best < 10000 {
        g.go(this).offset_0x70 = 100;
    }
    star
}

/// port: 004ec2f0 FUN_004ec2f0
/// `TryEatStar()`: an edible coin in its mouth is eaten (vfunction 78). In the virtual
/// tank's feeding mode, the feeder decides.
pub fn FUN_004ec2f0(g: &mut G, this: Ptr) {
    if g.go(this).offset_0x68 == 0 {
        let board = board_of(g, this);
        for c in g.board(board).offset_0xc[vec_index(0xa8)].clone() {
            let d = g.penta(this).clone();
            let (x, y) = (d.offset_0x4 + 40.0, d.offset_0xc + 40.0);
            let (cx, cy) = (g.wc(c).offset_0x2c, g.wc(c).offset_0x30);
            if x < (cx + 0x38) as f64 && ((cx + 0x10) as f64) < x && y < (cy + 0x42) as f64 && ((cy + 0x10) as f64) < y && FUN_004d6bf0(g, this, c) {
                FUN_004d6cc0(g, this);
                crate::game::coin::vfunction76(g, c);
                vcall!(g, this, go.vfunction78, c as i32);
                return;
            }
        }
    } else {
        let wc = g.wc(this).clone();
        if 0 < crate::game::game_object::FUN_004ea2a0(g, this, wc.offset_0x34 / 2 + wc.offset_0x2c, wc.offset_0x38 / 2 + wc.offset_0x30) {
            FUN_004d6cc0(g, this);
        }
    }
}

/// port: 004eff50 FUN_004eff50
/// `ChaseStar()`: every 5 updates crawls toward the nearest star (faster when hungry),
/// then tries to eat it. True when there is one.
pub fn FUN_004eff50(g: &mut G, this: Ptr) -> bool {
    let t = FUN_004ec180(g, this);
    if 5 <= g.penta(this).offset_0x40 {
        if t == NULL {
            return false;
        }
        let hungry = g.go(this).offset_0x14 < 0x12d;
        let tx = g.wc(t).offset_0x2c;
        let d = g.penta(this);
        let x = d.offset_0x4 + 40.0;
        d.offset_0x40 = 0;
        let (big, small, lim_big, lim_small) = if hungry { (2.5, 1.2, 6.0, 2.5) } else { (1.8, 1.0, 4.5, 1.5) };
        let v = &mut d.offset_0x14;
        if x <= (tx + 0x2e) as f64 {
            if ((tx + 0x1a) as f64) <= x {
                if x <= (tx + 0x24) as f64 {
                    if x < (tx + 0x24) as f64 && *v < lim_small {
                        *v += small;
                    }
                } else if -lim_small < *v {
                    *v -= small;
                }
            } else if *v < lim_big {
                *v += big;
            }
        } else if -lim_big < *v {
            *v -= big;
        }
    }
    if t != NULL {
        FUN_004ec2f0(g, this);
    }
    t != NULL
}

/// port: 004f9460 FUN_004f9460
/// `Hunt()`: hunger ticks (the hungry look only with no aliens around); starving dies;
/// below 900 it chases stars.
pub fn FUN_004f9460(g: &mut G, this: Ptr) -> bool {
    crate::game::game_object::FUN_004d6c50(g, this);
    let board = board_of(g, this);
    if g.board(board).offset_0xc[vec_index(0xb8)].is_empty() && g.board(board).offset_0xc[vec_index(0xf4)].is_empty() {
        crate::game::game_object::FUN_004d6ef0(g, this);
    }
    if !crate::game::game_object::FUN_004d69d0(g, this) {
        if g.go(this).offset_0x14 < 900 {
            return FUN_004eff50(g, this);
        }
    } else {
        FUN_004f3c60(g, this, true);
    }
    false
}

/// port: 004d8f10 FUN_004d8f10
/// `Animate()`: the crawl cycle by speed (40 counts walking either way at low speed,
/// 20 counts running fast), carrying.
pub fn FUN_004d8f10(g: &mut G, this: Ptr) {
    if g.go(this).offset_0x74 {
        crate::game::game_object::FUN_004d6c90(g, this);
    }
    let d = g.penta(this);
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
    if g.go(this).offset_0x10 != NULL {
        crate::game::game_object::FUN_004d7020(g, this);
    }
}

/// port: 004d8e70 FUN_004d8e70
/// `Pearl()`: after a star (every third in the virtual tank, and there only when allowed)
/// a pearl (coin kind 5) unless hungry. True when one appeared.
pub fn FUN_004d8e70(g: &mut G, this: Ptr) -> bool {
    g.penta(this).offset_0x50 += 1;
    let app = g.go(this).offset_0x0;
    if g.wfa(app).offset_0x150 != 5 || 2 < g.penta(this).offset_0x50 {
        g.penta(this).offset_0x50 = 0;
        if g.wfa(app).offset_0x150 == 5 {
            if !crate::game::alien::FUN_004d70b0(g, this) {
                return false;
            }
            let board = g.wfa(app).offset_0x4;
            if !g.board(board).ext_0x500 {
                return false;
            }
        }
        if crate::game::game_object::FUN_004d6bd0(g, this) {
            let board = g.wfa(app).offset_0x4;
            let (mx, my) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
            crate::game::board_level::FUN_00544430(g, board, mx + 5, my - 0xc, 5, NULL, -1.0, 0);
            return true;
        }
    }
    false
}

/// port: 004e3140 Sexy::Penta::vfunction78
/// `Eat(Coin*)`: fed (+900, at most 1300), counted in the virtual tank; the pearl sound
/// when a pearl comes, else the eat sound.
pub fn vfunction78(g: &mut G, this: Ptr, _param_1: i32) {
    let hungry = FUN_004d6f30(g, this);
    crate::game::game_object::FUN_004d6a30(g, this, false);
    if -1 < g.go(this).offset_0x24 {
        crate::game::game_object::FUN_004d6f90(g, this);
    }
    g.go(this).offset_0x14 += 900;
    if 0x514 < g.go(this).offset_0x14 {
        g.go(this).offset_0x14 = 0x514;
    }
    let board = board_of(g, this);
    if FUN_004d8e70(g, this) {
        crate::game::board::FUN_00538230(g, board, 0x111, 3, 1.0);
        crate::game::game_object::FUN_004e13d0(g, this, hungry);
        return;
    }
    let high = g.go(this).offset_0x7c;
    crate::game::board_level::FUN_005384c0(g, board, high);
    crate::game::game_object::FUN_004e13d0(g, this, hungry);
}

/// port: 004f94e0 Sexy::Penta::vfunction23
/// `Update()` (not while paused): unless hunting, eases toward its mode's speed; the mode
/// re-roll (1 in 10 every 20 updates, or at the walls); kept in the tank; blowing its
/// first bubbles; slowing at the walls; sinking; animation; moving (fast while dropping in).
pub fn vfunction23(g: &mut G, this: Ptr) {
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    if board == NULL || g.board(board).field_0x8 {
        return;
    }
    crate::game::game_object::FUN_004f22c0(g, this);
    if !FUN_004f9460(g, this) {
        let d = g.penta(this);
        match d.offset_0x3c {
            0 | 5 | 6 => d.offset_0x34 = 0.0,
            1 => d.offset_0x34 = -0.5,
            2 => d.offset_0x34 = 0.5,
            3 => d.offset_0x34 = -1.0,
            4 => d.offset_0x34 = 1.0,
            7 => d.offset_0x34 = -2.5,
            8 => d.offset_0x34 = 2.5,
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
        let d = g.penta(this);
        d.offset_0x40 += 1;
        d.offset_0x44 += 1;
        0x14 < d.offset_0x44 || d.offset_0x4 <= 10.0 || 540.0 <= d.offset_0x4
    };
    if reroll {
        g.penta(this).offset_0x44 = 0;
        if rand(g, this) % 10 == 0 {
            let m = (rand(g, this) % 9) as i32;
            g.penta(this).offset_0x3c = m;
        }
    }
    {
        let d = g.penta(this);
        if 550.0 < d.offset_0x4 {
            d.offset_0x4 = 550.0;
        }
        if d.offset_0x4 < 10.0 {
            d.offset_0x4 = 10.0;
        }
        if 359.0 < d.offset_0xc {
            d.offset_0xc = 359.0;
            d.offset_0x1c = 0.0;
        }
    }
    let bubbles = g.penta(this).offset_0x2c;
    if bubbles < 1 {
        let d = g.penta(this);
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
        g.penta(this).offset_0x2c -= 1;
    }
    {
        let d = g.penta(this);
        if 535.0 < d.offset_0x4 && 0.1 < d.offset_0x14 {
            d.offset_0x14 -= 0.1;
        }
        if d.offset_0x4 < 15.0 && d.offset_0x14 < -0.1 {
            d.offset_0x14 += 0.1;
        }
        if d.offset_0xc < 359.0 {
            d.offset_0x1c += 0.4;
        }
    }
    FUN_004d8f10(g, this);
    let mut div = g.penta(this).offset_0x24;
    if g.go(this).offset_0x6e {
        div = if g.go(this).offset_0x70 == 0 { 0.3 } else { 0.8 };
    }
    let d = g.penta(this);
    d.offset_0x4 = d.offset_0x14 / div + d.offset_0x4;
    d.offset_0xc = d.offset_0x1c / div + d.offset_0xc;
    let (x, y) = (ftol(d.offset_0x4) as i32, ftol(d.offset_0xc) as i32);
    vcall!(g, this, w.vfunction42, x, y);
}

/// port: 004e2e30 FUN_004e2e30
/// `DrawBody(Graphics*)`: the crawl cel (the hungry row when hungry), the hungry tint
/// fading over it, what it carries, the sparkle; the plain sheet's cel in the
/// `DAT_005e89ce` mode.
pub fn FUN_004e2e30(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    let img = g.res.DAT_005e8e04;
    if !g.globals.DAT_005e89ce {
        let row = if FUN_004d6f30(g, this) { 0x50 } else { 0 };
        let cel = g.penta(this).offset_0x4c * 0x50;
        if g.go(this).offset_0x74 && crate::game::game_object::FUN_004d6e70(g, this, gfx, img, &Rect::new(cel, row, 0x50, 0x50), false) {
            return;
        }
        FUN_004d6db0(g, this, gfx, Color::WHITE);
        FUN_00455e40(gfx, g, img, 0, 0, &Rect::new(cel, row, 0x50, 0x50));
        let t = g.go(this).offset_0x18;
        if t != 0 {
            FUN_004d6db0(g, this, gfx, CRect(0xff, 0xff, 0xff, (t * 0xff) / 5));
            FUN_00455e40(gfx, g, img, 0, 0, &Rect::new(cel, 0x50, 0x50, 0x50));
        }
        FUN_004558e0(gfx, false);
        if g.go(this).offset_0x10 != NULL {
            FUN_004d7040(g, this, gfx, 0, 0);
        }
        if crate::game::game_object::FUN_004d6980(g, this, 500) {
            let s = g.res.DAT_005e8bc4;
            FUN_00456950(gfx, g, s, 0, -10, 2);
        }
    } else {
        let col = if FUN_004d6f30(g, this) { 4 } else { 9 };
        FUN_004560a0(gfx, g, img, 0, 0, &Rect::new(col * 0x50, 0xa0, 0x50, 0x50), false);
        if g.go(this).offset_0x10 != NULL {
            FUN_004d7040(g, this, gfx, 0, 0);
        }
        if crate::game::game_object::FUN_004d6980(g, this, 500) {
            let s = g.res.DAT_005e8bc4;
            FUN_00456950(gfx, g, s, 0, -10, 2);
        }
    }
}

/// port: 004e3030 Sexy::Penta::vfunction27
/// `Draw(Graphics*)`: the body, and its name label when it has one.
pub fn vfunction27(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    crate::game::food::FUN_004d6ae0(g, this);
    FUN_004e2e30(g, this, gfx);
    // +0xc4: the name's length.
    if !g.go(this).field_0x28.is_empty() {
        crate::game::game_object::FUN_004e1400(g, this, gfx, false);
    }
}

/// port: 004e3060 Sexy::Penta::vfunction80
/// `DrawIcon(Graphics*, int pose)`: its portrait cel offset by pose.
pub fn vfunction80(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: i32) {
    let (dx, dy) = match param_2 {
        0 => (0x14, 0x11),
        1 => (5, 0x11),
        2 => (0xf, -5),
        3 => (5, -1),
        4 => (-0x28, 0),
        _ => (0, 0),
    };
    FUN_004563d0(gfx, dx, dy);
    crate::game::game_object::FUN_004d6d10(g, this, gfx, Color::WHITE);
    let img = g.res.DAT_005e8e04;
    let col = g.go(this).offset_0xa8 * 0x50;
    FUN_00455e40(gfx, g, img, 0, 0, &Rect::new(col, 0, 0x50, 0x50));
    FUN_004558e0(gfx, false);
    FUN_004563d0(gfx, -dx, -dy);
}
