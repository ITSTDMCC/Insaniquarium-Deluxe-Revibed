//! `Sexy::Shot`: short-lived effects drawn over the tank: the laser shot where the player
//! clicks (kind 0, colored by the weapon level) and the sparkles / bursts of kinds 1..8.
//! `Shot_data` starts at object offset 0x154.

use crate::game::game_object::{FUN_004f22c0, GameObject};
use crate::sexy::graphics::{FUN_004558c0, FUN_004558e0, FUN_00455890, FUN_00456950, FUN_00456980};
use crate::sexy::prelude::*;
use crate::sexy::types::CRect;

/// `Shot_data` (object offset 0x154, 24 bytes).
#[derive(Debug, Clone, Default)]
pub struct Shot_data {
    /// +0x158 frame.
    pub offset_0x4: i32,
    /// +0x15c cel.
    pub offset_0x8: i32,
    /// +0x160 kind.
    pub offset_0xc: i32,
    /// +0x164 start delay in updates.
    pub offset_0x10: i32,
    /// +0x168 alpha.
    pub offset_0x14: i32,
}

impl G {
    pub fn shot(&mut self, p: Ptr) -> &mut Shot_data {
        match &mut self.go_ext(p).sub {
            GoSub::Shot(d) => d,
            s => panic!("{p} is not a Shot: {s:?}"),
        }
    }
}

fn alloc_shot(g: &mut G, wc: crate::sexy::object::WidgetContainer_data, w: crate::sexy::object::Widget_data,
              go: crate::game::game_object::GameObject_data, d: Shot_data) -> Ptr {
    g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__Shot_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::GameObject(Box::new(GameObjectExt { go, sub: GoSub::Shot(d) })) }),
    })
}

/// port: 004ec5d0 Sexy::Shot::Shot
/// `Shot::Shot()` (used before loading from a save).
pub fn Shot__004ec5d0(g: &mut G) -> Ptr {
    let (wc, w, mut go) = GameObject(g);
    go.offset_0x4 = 0x20;
    alloc_shot(g, wc, w, go, Shot_data::default())
}

/// port: 004ec600 Sexy::Shot::Shot
/// `Shot::Shot(int x, int y)`: the laser shot (kind 0), 80x80, opaque, ignores the mouse.
pub fn Shot__004ec600(g: &mut G, param_1: i32, param_2: i32) -> Ptr {
    let (mut wc, mut w, mut go) = GameObject(g);
    wc.offset_0x2c = param_1;
    wc.offset_0x34 = 0x50;
    wc.offset_0x38 = 0x50;
    w.offset_0x1 = false;
    go.offset_0x4 = 0x20;
    wc.offset_0x30 = param_2;
    let d = Shot_data { offset_0x4: 0, offset_0x8: 0, offset_0xc: 0, offset_0x10: 0, offset_0x14: 0xff };
    alloc_shot(g, wc, w, go, d)
}

/// port: 004ec660 Sexy::Shot::Shot
/// `Shot::Shot(int x, int y, int kind)`: a sparkle with a random alpha (50..249) and a
/// size by kind (50x30 for kind 1, 60 for 4/7, 40 for 6/8, else 80).
///
/// The original tests the kind field *before* storing `kind` into it, i.e. it reads the
/// freshly allocated (uninitialized) object memory to decide whether to draw a start delay
/// of `rand() % 3` (and consume a random number). That memory is modeled as zero here, so
/// there is never a delay and no random number is drawn for it.
pub fn Shot__004ec660(g: &mut G, param_1: i32, param_2: i32, param_3: i32) -> Ptr {
    let (mut wc, mut w, mut go) = GameObject(g);
    wc.offset_0x2c = param_1;
    let uninit_kind = 0;
    go.offset_0x4 = 0x20;
    wc.offset_0x30 = param_2;
    let mut d = Shot_data::default();
    let app = go.offset_0x0;
    let rng = g.wfa(app).offset_0x84;
    if uninit_kind == 0 || uninit_kind == 2 {
        d.offset_0x10 = 0;
    } else {
        d.offset_0x10 = (crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng)) % 3) as i32;
    }
    w.offset_0x1 = false;
    d.offset_0xc = param_3;
    d.offset_0x14 = (crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng)) % 200) as i32 + 0x32;
    let size = match param_3 {
        3 => 0x50,
        7 | 4 => 0x3c,
        6 | 8 => 0x28,
        1 => {
            wc.offset_0x34 = 0x32;
            wc.offset_0x38 = 0x1e;
            return alloc_shot(g, wc, w, go, d);
        }
        _ => 0x50,
    };
    wc.offset_0x34 = size;
    wc.offset_0x38 = size;
    alloc_shot(g, wc, w, go, d)
}

/// port: 004d9100 Sexy::Shot::vfunction81
/// `Sync(DataSync&)`.
pub fn vfunction81(g: &mut G, this: Ptr, sync: &mut crate::sexy::data_sync::DataSync) -> crate::sexy::data_sync::SyncResult {
    crate::game::game_object::vfunction81(g, this, sync)?;
    use crate::sexy::data_sync::FUN_00503010;
    let mut d = g.shot(this).clone();
    FUN_00503010(sync, &mut d.offset_0x4)?;
    FUN_00503010(sync, &mut d.offset_0x8)?;
    FUN_00503010(sync, &mut d.offset_0xc)?;
    FUN_00503010(sync, &mut d.offset_0x10)?;
    FUN_00503010(sync, &mut d.offset_0x14)?;
    *g.shot(this) = d;
    Ok(())
}

/// port: 004f3d90 Sexy::Shot::vfunction23
/// `Update()` (not while paused): waits out the delay, then advances the frame and leaves
/// after it (14 frames for the laser, 29 for kind 2, 10 for kind 1, 39 rising for 6/7,
/// else 19, twice as fast when bright).
pub fn vfunction23(g: &mut G, this: Ptr) {
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    if board == NULL || g.board(board).field_0x8 {
        return;
    }
    FUN_004f22c0(g, this);
    let d = g.shot(this);
    if d.offset_0x10 != 0 {
        d.offset_0x10 -= 1;
        return;
    }
    d.offset_0x4 += 1;
    let kind = d.offset_0xc;
    let frame = d.offset_0x4;
    let done = if (6..=7).contains(&kind) {
        g.wc(this).offset_0x30 -= 2;
        0x27 < frame
    } else if kind == 0 {
        0xe < frame
    } else if kind == 2 {
        0x1d < frame
    } else if kind == 1 {
        10 < frame
    } else {
        if 0x96 < d.offset_0x14 {
            d.offset_0x4 = frame + 1;
        }
        0x13 < d.offset_0x4
    };
    if done {
        crate::game::larva::vfunction76(g, this);
    }
    let d = g.shot(this);
    let frame = d.offset_0x4;
    d.offset_0x8 = frame / 2;
    if d.offset_0xc == 0 {
        d.offset_0x8 = frame;
    }
    d.offset_0x8 -= 1;
}

/// port: 004d9160 FUN_004d9160
/// The laser glow color for the weapon level, with alpha `param_2` (levels 0, 1 and above
/// 12 leave the color as it is).
pub fn FUN_004d9160(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: i32) {
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    let (r, gr, b) = match g.board(board).field_0x358 {
        2 | 0xb => (0x32, 0xb4, 200),
        3 => (0xff, 0, 0),
        4 => (0xff, 0x37, 0),
        5 => (0xff, 200, 0),
        6 => (0x7d, 0xff, 0x5a),
        7 => (10, 200, 0x23),
        8 => (5, 0x96, 0xfa),
        9 => (0xa0, 0, 0xff),
        10 => (0xff, 0, 0),
        0xc => (0xeb, 0, 0xd7),
        _ => return,
    };
    FUN_00455890(gfx, CRect(r, gr, b, param_2));
}

/// port: 004d9280 FUN_004d9280
/// The laser core color for the weapon level (white-ish; the top levels tinted, at the
/// shot's own alpha).
pub fn FUN_004d9280(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: i32) {
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    let own = g.shot(this).offset_0x14;
    let (r, gr, b, a) = match g.board(board).field_0x358 {
        4 | 7 => (0xff, 0xff, 100, param_2),
        6 => (0xff, 0xff, 0, param_2),
        10 => (100, 0xff, 0xff, own),
        0xb => (0xb4, 0xe1, 0x46, own),
        0xc => (0, 0xff, 0, own),
        _ => (0xff, 0xff, 0xff, own),
    };
    FUN_00455890(gfx, CRect(r, gr, b, a));
}

/// port: 004d9340 Sexy::Shot::vfunction27
/// `Draw(Graphics*)` (after the delay): the laser as a glow, an additive glow and an
/// additive core (rows by weapon level), or the kind's sparkle cel.
pub fn vfunction27(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    crate::game::food::FUN_004d6ae0(g, this);
    let d = g.shot(this).clone();
    if d.offset_0x10 != 0 {
        return;
    }
    FUN_004558e0(gfx, true);
    FUN_00455890(gfx, CRect(0xff, 0xff, 0xff, d.offset_0x14));
    match d.offset_0xc {
        0 => {
            let app = g.go(this).offset_0x0;
            let board = g.wfa(app).offset_0x4;
            let lvl = g.board(board).field_0x358;
            let row = if lvl < 10 { (6 < lvl) as i32 } else { 2 } * 2;
            let img = g.res.DAT_005e8e34;
            FUN_004d9160(g, this, gfx, 100);
            FUN_00456980(gfx, g, img, 0, 0, d.offset_0x8, row);
            FUN_004558c0(gfx, 1);
            FUN_004d9160(g, this, gfx, d.offset_0x14);
            FUN_00456980(gfx, g, img, 0, 0, d.offset_0x8, row);
            FUN_004d9280(g, this, gfx, d.offset_0x14);
            FUN_00456980(gfx, g, img, 0, 0, d.offset_0x8, row + 1);
            FUN_004558c0(gfx, 0);
            FUN_004558e0(gfx, false);
        }
        2 => {
            FUN_004558c0(gfx, 1);
            FUN_004558c0(gfx, 0);
            FUN_004558e0(gfx, false);
        }
        3 | 4 => {
            FUN_004558c0(gfx, 1);
            let img = if d.offset_0xc == 3 { g.res.DAT_005e8d40 } else { g.res.DAT_005e8ef0 };
            FUN_00456950(gfx, g, img, 0, 0, d.offset_0x8);
            FUN_004558c0(gfx, 0);
            FUN_004558e0(gfx, false);
        }
        k => {
            let img = match k {
                1 => Some(g.res.DAT_005e8a34),
                5 => Some(g.res.DAT_005e8df4),
                6 => Some(g.res.DAT_005e8a90),
                7 => Some(g.res.DAT_005e8bb0),
                8 => Some(g.res.DAT_005e8d58),
                _ => None,
            };
            if let Some(img) = img {
                FUN_00456950(gfx, g, img, 0, 0, d.offset_0x8);
            }
            FUN_004558e0(gfx, false);
        }
    }
}
