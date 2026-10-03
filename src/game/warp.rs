//! `Sexy::Warp`: the swirling hole an alien arrives through (36 updates). `Warp_data`
//! starts at object offset 0x154.

use crate::game::game_object::{FUN_004f22c0, GameObject};
use crate::sexy::graphics::{FUN_004558c0, FUN_00456950};
use crate::sexy::prelude::*;

/// `Warp_data` (object offset 0x154).
#[derive(Debug, Clone, Default)]
pub struct Warp_data {
    /// +0x158 updates left (36).
    pub offset_0x4: i32,
}

impl G {
    pub fn warp(&mut self, p: Ptr) -> &mut Warp_data {
        match &mut self.go_ext(p).sub {
            GoSub::Warp(d) => d,
            s => panic!("{p} is not a Warp: {s:?}"),
        }
    }
}

fn alloc_warp(g: &mut G, wc: crate::sexy::object::WidgetContainer_data, w: crate::sexy::object::Widget_data,
              go: crate::game::game_object::GameObject_data, d: Warp_data) -> Ptr {
    g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__Warp_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::GameObject(Box::new(GameObjectExt { go, sub: GoSub::Warp(d) })) }),
    })
}

/// port: 004ecee0 Sexy::Warp::Warp
/// `Warp::Warp()` (used before loading from a save).
pub fn Warp__004ecee0(g: &mut G) -> Ptr {
    let (wc, w, mut go) = GameObject(g);
    go.offset_0x4 = 0x21;
    alloc_warp(g, wc, w, go, Warp_data::default())
}

/// port: 004ecf00 Sexy::Warp::Warp
/// `Warp(int x, int y)`: 100x220, 36 updates, ignores the mouse.
pub fn Warp__004ecf00(g: &mut G, param_1: i32, param_2: i32) -> Ptr {
    let (mut wc, mut w, mut go) = GameObject(g);
    wc.offset_0x2c = param_1;
    go.offset_0x4 = 0x21;
    wc.offset_0x30 = param_2;
    wc.offset_0x34 = 100;
    wc.offset_0x38 = 0xdc;
    w.offset_0x1 = false;
    alloc_warp(g, wc, w, go, Warp_data { offset_0x4: 0x24 })
}

/// port: 004da660 Sexy::Warp::vfunction81
/// `Sync(DataSync&)`.
pub fn vfunction81(g: &mut G, this: Ptr, sync: &mut crate::sexy::data_sync::DataSync) -> crate::sexy::data_sync::SyncResult {
    crate::game::game_object::vfunction81(g, this, sync)?;
    let mut n = g.warp(this).offset_0x4;
    let r = crate::sexy::data_sync::FUN_00503010(sync, &mut n);
    g.warp(this).offset_0x4 = n;
    r
}

/// port: 004da690 Sexy::Warp::vfunction27
/// `Draw(Graphics*)`: the hole and its additive glow, cel 17 - left / 2.
pub fn vfunction27(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    crate::game::food::FUN_004d6ae0(g, this);
    let cel = 0x11 - g.warp(this).offset_0x4 / 2;
    let img = g.res.DAT_005e8e5c;
    FUN_00456950(gfx, g, img, 0x14, 0, cel);
    FUN_004558c0(gfx, 1);
    let img = g.res.DAT_005e8c08;
    FUN_00456950(gfx, g, img, 0, 0, cel);
    FUN_004558c0(gfx, 0);
}

/// port: 004f3e70 Sexy::Warp::vfunction23
/// `Update()` (not while paused): removed once its time is up.
pub fn vfunction23(g: &mut G, this: Ptr) {
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    if board == NULL || g.board(board).field_0x8 {
        return;
    }
    FUN_004f22c0(g, this);
    if g.warp(this).offset_0x4 < 1 {
        let mgr = g.wc(board).offset_0xc;
        vcall!(g, mgr, w.vfunction5, this);
        crate::sexy::sexy_app_base::vfunction35(g, app, this);
        let board = g.wfa(app).offset_0x4;
        crate::game::board_level::FUN_00541d90(g, board, this, false);
    }
    g.warp(this).offset_0x4 -= 1;
}
