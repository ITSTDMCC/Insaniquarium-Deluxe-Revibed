//! `Sexy::DeadFish`: the belly-up body a fish, pet or alien leaves behind. It drifts with
//! the speed the creature had, sinks to the floor, fades and goes; a reviving pet can count
//! it back to life (`offset_0x3c` 10..1), which recreates the creature of the same kind.
//! `DeadFish_data` starts at object offset 0x154.

use crate::game::game_object::{FUN_004f22c0, GameObject};
use crate::sexy::crt::ftol;
use crate::sexy::graphics::{FUN_004558e0, FUN_00455890, FUN_00455e40, FUN_004560a0};
use crate::sexy::prelude::*;
use crate::sexy::types::CRect;

/// `DeadFish_data` (object offset 0x154, 84 bytes).
#[derive(Debug, Clone, Default)]
pub struct DeadFish_data {
    /// +0x158 facing (drawn mirrored).
    pub offset_0x4: bool,
    /// +0x160 y.
    pub offset_0xc: f64,
    /// +0x168 x.
    pub offset_0x14: f64,
    /// +0x170 x speed (slows to 0 by 0.03).
    pub offset_0x1c: f64,
    /// +0x178 y speed (sinks towards 2, rises towards -1.5 while `offset_0x34`).
    pub offset_0x24: f64,
    /// +0x180 speed divisor.
    pub offset_0x2c: f64,
    /// +0x188 rising.
    pub offset_0x34: bool,
    /// +0x18c cel.
    pub offset_0x38: i32,
    /// +0x190 revive countdown (100: none).
    pub offset_0x3c: i32,
    /// +0x198 alpha (1.0, fading by 0.02 once on the floor).
    pub offset_0x44: f64,
    /// +0x1a0 life (125 down to 0).
    pub offset_0x4c: i32,
    /// +0x1a4 the kind that died (guppy sizes 0..4, 5..12 other creatures).
    pub offset_0x50: i32,
}

impl G {
    pub fn dead_fish(&mut self, p: Ptr) -> &mut DeadFish_data {
        match &mut self.go_ext(p).sub {
            GoSub::DeadFish(d) => d,
            s => panic!("{p} is not a DeadFish: {s:?}"),
        }
    }
}

fn alloc_dead_fish(g: &mut G, wc: crate::sexy::object::WidgetContainer_data, w: crate::sexy::object::Widget_data,
                   go: crate::game::game_object::GameObject_data, d: DeadFish_data) -> Ptr {
    g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__DeadFish_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::GameObject(Box::new(GameObjectExt { go, sub: GoSub::DeadFish(d) })) }),
    })
}

/// port: 004eed00 Sexy::DeadFish::DeadFish
/// `DeadFish::DeadFish()` (used before loading from a save).
pub fn DeadFish__004eed00(g: &mut G) -> Ptr {
    let (wc, w, mut go) = GameObject(g);
    go.offset_0x4 = 0x1b;
    alloc_dead_fish(g, wc, w, go, DeadFish_data::default())
}

/// port: 004eed20 Sexy::DeadFish::DeadFish
/// `DeadFish(double x, double y, double vx, double vy, double div, int kind, bool facing)`:
/// 80x80 (160x160 for kind 6), sinking 1 (high up or already rising fast) or 2 slower than
/// it moved; added to the board's widget manager right away.
pub fn DeadFish__004eed20(g: &mut G, param_1: f64, param_2: f64, param_3: f64, param_4: f64, param_5: f64, param_6: i32, param_7: bool) -> Ptr {
    let (mut wc, mut w, mut go) = GameObject(g);
    go.offset_0x4 = 0x1b;
    wc.offset_0x2c = ftol(param_1) as i32;
    wc.offset_0x30 = ftol(param_2) as i32;
    let mut d = DeadFish_data::default();
    d.offset_0xc = param_2;
    d.offset_0x14 = param_1;
    d.offset_0x50 = param_6;
    d.offset_0x1c = param_3;
    d.offset_0x24 = param_4;
    d.offset_0x24 = param_4 - if param_2 < 115.0 || param_4 < -3.0 { 1.0 } else { 2.0 };
    d.offset_0x2c = param_5;
    wc.offset_0x34 = 0x50;
    wc.offset_0x38 = 0x50;
    if param_6 == 6 {
        wc.offset_0x34 = 0xa0;
        wc.offset_0x38 = 0xa0;
    }
    d.offset_0x4c = 0x7d;
    w.offset_0x1 = false;
    d.offset_0x4 = param_7;
    d.offset_0x38 = 0;
    d.offset_0x3c = 100;
    d.offset_0x34 = false;
    let app = go.offset_0x0;
    let this = alloc_dead_fish(g, wc, w, go, d);
    let board = g.wfa(app).offset_0x4;
    let mgr = g.wc(board).offset_0xc;
    vcall!(g, mgr, w.vfunction4, this);
    g.dead_fish(this).offset_0x44 = 1.0;
    this
}

/// port: 004db250 Sexy::DeadFish::vfunction82
/// `Remove()`: off the widget manager, deferred delete, off the board, and the shadow too.
pub fn vfunction82(g: &mut G, this: Ptr) {
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    let mgr = g.wc(board).offset_0xc;
    vcall!(g, mgr, w.vfunction5, this);
    crate::sexy::sexy_app_base::vfunction35(g, app, this);
    let board = g.wfa(app).offset_0x4;
    crate::game::board_level::FUN_00541d90(g, board, this, false);
    let shadow = g.go(this).offset_0xc;
    if shadow != NULL {
        crate::game::shadow::vfunction76(g, shadow);
    }
}

/// port: 004ddf60 Sexy::DeadFish::vfunction81
/// `Sync(DataSync&)`.
pub fn vfunction81(g: &mut G, this: Ptr, sync: &mut crate::sexy::data_sync::DataSync) -> crate::sexy::data_sync::SyncResult {
    crate::game::game_object::vfunction81(g, this, sync)?;
    use crate::sexy::data_sync::{FUN_00503010, FUN_00503070, FUN_005030a0};
    let mut d = g.dead_fish(this).clone();
    // (+0x194 is synced too; it is never used otherwise.)
    let mut unused_0x40 = 0i32;
    FUN_00503070(sync, &mut d.offset_0x4)?;
    FUN_005030a0(sync, &mut d.offset_0xc)?;
    FUN_005030a0(sync, &mut d.offset_0x14)?;
    FUN_005030a0(sync, &mut d.offset_0x1c)?;
    FUN_005030a0(sync, &mut d.offset_0x24)?;
    FUN_005030a0(sync, &mut d.offset_0x2c)?;
    FUN_00503070(sync, &mut d.offset_0x34)?;
    FUN_00503010(sync, &mut d.offset_0x38)?;
    FUN_00503010(sync, &mut d.offset_0x3c)?;
    FUN_00503010(sync, &mut unused_0x40)?;
    FUN_005030a0(sync, &mut d.offset_0x44)?;
    FUN_00503010(sync, &mut d.offset_0x4c)?;
    FUN_00503010(sync, &mut d.offset_0x50)?;
    *g.dead_fish(this) = d;
    Ok(())
}

/// port: 004f64e0 Sexy::DeadFish::vfunction23
/// `Update()` (not while paused): the cel (turning over, or the revive count), the fade on
/// the floor (shared with the shadow), removal when life runs out, revival when the count
/// reaches 0 (sound 0x125 and the creature again), else drifting and sinking within the
/// tank, losing life on the floor.
pub fn vfunction23(g: &mut G, this: Ptr) {
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    if board == NULL || g.board(board).field_0x8 {
        return;
    }
    FUN_004f22c0(g, this);
    let d = g.dead_fish(this);
    let rc = d.offset_0x3c;
    if (0..=10).contains(&rc) {
        let v = rc - 1;
        d.offset_0x3c = v;
        d.offset_0x38 = v;
        if v == 10 {
            d.offset_0x3c = v;
        }
    } else {
        let life = d.offset_0x4c;
        d.offset_0x38 = if d.offset_0x50 == 9 || d.offset_0x50 == 8 {
            if 0x69 < life { 9 - (life - 0x6a) / 2 } else { 9 }
        } else if 0x69 < life {
            9 - (life - 0x6a) / 2
        } else if life == 0x68 || life == 0x67 {
            8
        } else if life == 0x66 || life == 0x65 {
            7
        } else if 100 < life {
            9
        } else {
            6
        };
    }
    let d = g.dead_fish(this);
    if d.offset_0x4c < 0x69 {
        d.offset_0x44 -= 0.02;
        if d.offset_0x44 < 0.0 {
            d.offset_0x44 = 0.0;
        }
        let a = d.offset_0x44;
        let shadow = g.go(this).offset_0xc;
        if shadow != NULL {
            g.shadow(shadow).offset_0x14 = a;
        }
    }
    let life = g.dead_fish(this).offset_0x4c;
    if life < 1 {
        vfunction82(g, this);
        return;
    }
    if g.dead_fish(this).offset_0x3c < 1 {
        crate::game::board::FUN_00538230(g, board, 0x125, 3, 1.0);
        vfunction82(g, this);
        let d = g.dead_fish(this).clone();
        let (mx, my) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
        let board = g.wfa(app).offset_0x4;
        match d.offset_0x50 {
            7 => crate::game::board_level::FUN_005451c0(g, board, mx, my, d.offset_0x4),
            5 => crate::game::board_level::FUN_00544d80(g, board, mx, my, d.offset_0x4),
            9 => crate::game::board_level::FUN_005454f0(g, board, mx),
            8 => crate::game::board_level::FUN_00545360(g, board, mx),
            k @ 10..=0xc => crate::game::board_level::FUN_00547270(g, board, mx, my, k - 10, d.offset_0x4),
            6 => crate::game::board_level::FUN_00544fe0(g, board, mx, my, d.offset_0x4),
            k => {
                crate::game::board_level::FUN_00546fc0(g, board, mx, my, k, d.offset_0x4);
            }
        }
        return;
    }
    let d = g.dead_fish(this);
    let kind = d.offset_0x50;
    if (kind == 6 && 300.0 < d.offset_0xc) || ((kind == 9 || kind == 8) && 365.0 < d.offset_0xc) || 370.0 < d.offset_0xc || 0x69 < life {
        d.offset_0x4c = life - 1;
    }
    if 0.0 < d.offset_0x1c {
        d.offset_0x1c -= 0.03;
        if d.offset_0x1c < 0.0 {
            d.offset_0x1c = 0.0;
        }
    } else if d.offset_0x1c < 0.0 {
        d.offset_0x1c += 0.03;
        if 0.0 < d.offset_0x1c {
            d.offset_0x1c = 0.0;
        }
    }
    if d.offset_0x34 && -1.5 < d.offset_0x24 {
        d.offset_0x24 -= 0.05;
    } else {
        let lim = if d.offset_0x34 { -1.5 } else { 2.0 };
        if d.offset_0x24 < lim {
            d.offset_0x24 += 0.05;
        }
    }
    d.offset_0x14 += d.offset_0x1c / d.offset_0x2c;
    d.offset_0xc += d.offset_0x24 / d.offset_0x2c;
    if 540.0 < d.offset_0x14 {
        d.offset_0x14 = 540.0;
    }
    if d.offset_0x14 < 10.0 {
        d.offset_0x14 = 10.0;
    }
    let floor = if kind == 9 || kind == 8 { 370.0 } else if kind == 6 { 310.0 } else { 380.0 };
    if floor < d.offset_0xc {
        d.offset_0xc = floor;
    }
    if d.offset_0xc < 85.0 {
        d.offset_0xc = 85.0;
    }
    let (x, y) = (d.offset_0x14, d.offset_0xc);
    let (ix, iy) = (ftol(x) as i32, ftol(y) as i32);
    vcall!(g, this, w.vfunction42, ix, iy);
}

/// The pale tint of kinds 6 and 9 on their first frames on the floor.
fn tint(gfx: &mut Graphics, cel: i32) {
    let r = (cel * 5 + 0x4e2) / 5;
    let gr = (cel * 0x28 + 0x433) / 5;
    let b = (cel * 0xa0 + 0x1db) / 5;
    if cel < 5 {
        FUN_00455890(gfx, CRect(r, gr, b, 0xff));
        FUN_004558e0(gfx, true);
    }
}

/// port: 004de030 Sexy::DeadFish::vfunction27
/// `Draw(Graphics*)`: the cel of the kind's dead row (80x80; 160x160 for kind 6), faded by
/// the alpha and sunk into the sand as life runs out; mirrored by facing except kinds 8/9.
pub fn vfunction27(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    crate::game::food::FUN_004d6ae0(g, this);
    let d = g.dead_fish(this).clone();
    let kind = d.offset_0x50;
    FUN_004558e0(gfx, true);
    FUN_00455890(gfx, CRect(0xff, 0xff, 0xff, ftol(d.offset_0x44 * 255.0) as i32));
    let life = d.offset_0x4c;
    let yoff = if life < 0x5a { (0x5a - life) / 2 } else { 0 };
    let cel = d.offset_0x38;
    // (image, source y, size); `None` draws nothing.
    let src: Option<(Ptr, i32, i32)> = match kind {
        7 => Some((g.res.DAT_005e8da8, 400, 0x50)),
        6 => {
            if 0x59 < life {
                tint(gfx, cel);
            }
            Some((g.res.DAT_005e8cf4, 0x1e0, 0xa0))
        }
        9 | 8 => {
            let (img, sy) = if kind == 9 {
                if 0x59 < life {
                    tint(gfx, cel);
                }
                (g.res.DAT_005e8c98, 0xf0)
            } else {
                (g.res.DAT_005e8e04, 0xa0)
            };
            let r = Rect::new(cel * 0x50, sy, 0x50, 0x50);
            FUN_00455e40(gfx, g, img, 0, yoff, &r);
            FUN_004558e0(gfx, false);
            return;
        }
        5 => Some((g.res.DAT_005e8aa8, 0x140, 0x50)),
        k if k <= 4 => {
            let row = if 3 <= k { k - 1 } else { k };
            Some((g.res.DAT_005e8aa8, row * 0x50, 0x50))
        }
        k if 10 <= k => Some((g.res.DAT_005e8dc4, k * 0xf0 - 0x8c0, 0x50)),
        _ => None,
    };
    if let Some((img, sy, size)) = src {
        let r = Rect::new(cel * size, sy, size, size);
        FUN_004560a0(gfx, g, img, 0, yoff, &r, d.offset_0x4);
    }
    FUN_004558e0(gfx, false);
}

/// port: 004d5630 FUN_004d5630
/// Angie's touch: a dead fish not yet reviving (100) starts reviving (10 updates).
pub fn FUN_004d5630(g: &mut G, this: Ptr) {
    if g.dead_fish(this).offset_0x3c == 100 {
        g.dead_fish(this).offset_0x3c = 10;
    }
}
