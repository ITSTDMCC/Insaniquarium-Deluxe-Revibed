//! `Sexy::Larva`.

use crate::sexy::prelude::*;

/// port: 004d9570 Sexy::Larva::vfunction76
/// `Remove()`: the body every GameObject subclass shares for leaving the tank (the linker
/// folded the copies; the decompiler named it after Larva). Takes the object off the
/// widget manager, queues it for deferred deletion, and drops it from the board's lists.
pub fn vfunction76(g: &mut G, this: Ptr) {
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    let wm = g.wc(board).offset_0xc;
    vcall!(g, wm, w.vfunction5, this);
    // app->SafeDeleteWidget(this) (app vtable +0x88; not overridden)
    crate::sexy::sexy_app_base::vfunction35(g, app, this);
    let board = g.wfa(app).offset_0x4;
    crate::game::board_level::FUN_00541d90(g, board, this, false);
}

/// `Larva_data` (object offset 0x154; the object is 0x178 bytes): a beetle.
#[derive(Debug, Clone, Default)]
pub struct Larva_data {
    /// +0x158 x.
    pub offset_0x4: f64,
    /// +0x160 y.
    pub offset_0xc: f64,
    /// +0x168 rising speed (1.0).
    pub offset_0x14: f64,
    /// +0x170 animation cel (0..9).
    pub offset_0x1c: i32,
    /// +0x174 collected (flying to the money counter).
    pub offset_0x20: bool,
    /// +0x175 never clickable.
    pub offset_0x21: bool,
}

impl G {
    pub fn larva(&mut self, p: Ptr) -> &mut Larva_data {
        match &mut self.go_ext(p).sub {
            GoSub::Larva(d) => d,
            s => panic!("{p} is not a Larva: {s:?}"),
        }
    }
}

fn alloc_larva(g: &mut G, wc: crate::sexy::object::WidgetContainer_data, w: crate::sexy::object::Widget_data,
               go: crate::game::game_object::GameObject_data, d: Larva_data) -> Ptr {
    g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__Larva_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::GameObject(Box::new(GameObjectExt { go, sub: GoSub::Larva(d) })) }),
    })
}

/// port: 004ead20 Sexy::Larva::Larva
/// `Larva::Larva()` (used before loading from a save).
pub fn Larva__004ead20(g: &mut G) -> Ptr {
    let (wc, w, mut go) = crate::game::game_object::GameObject(g);
    go.offset_0x4 = 0x1d;
    alloc_larva(g, wc, w, go, Larva_data::default())
}

/// port: 004ead40 Sexy::Larva::Larva
/// `Larva(int x, int y)`: 72x72, not clickable until it rises above 320, rising at 1.
/// (A random number is drawn and dropped.)
pub fn Larva__004ead40(g: &mut G, param_1: i32, param_2: i32) -> Ptr {
    let (mut wc, mut w, mut go) = crate::game::game_object::GameObject(g);
    let d = Larva_data { offset_0x4: param_1 as f64, offset_0xc: param_2 as f64, offset_0x14: 1.0, ..Default::default() };
    go.offset_0x4 = 0x1d;
    wc.offset_0x2c = crate::sexy::crt::ftol(d.offset_0x4) as i32;
    wc.offset_0x30 = crate::sexy::crt::ftol(d.offset_0xc) as i32;
    wc.offset_0x34 = 0x48;
    wc.offset_0x38 = 0x48;
    w.offset_0x1 = false;
    w.offset_0x28 = false;
    let app = go.offset_0x0;
    let this = alloc_larva(g, wc, w, go, d);
    let r = g.wfa(app).offset_0x84;
    let _ = crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(r));
    this
}

/// port: 004d7df0 Sexy::Larva::vfunction81
/// `Sync(DataSync&)`.
pub fn vfunction81(g: &mut G, this: Ptr, sync: &mut crate::sexy::data_sync::DataSync) -> crate::sexy::data_sync::SyncResult {
    crate::game::game_object::vfunction81(g, this, sync)?;
    use crate::sexy::data_sync::{FUN_00503010, FUN_00503070, FUN_005030a0};
    let mut d = g.larva(this).clone();
    FUN_005030a0(sync, &mut d.offset_0x4)?;
    FUN_005030a0(sync, &mut d.offset_0xc)?;
    FUN_005030a0(sync, &mut d.offset_0x14)?;
    FUN_00503010(sync, &mut d.offset_0x1c)?;
    FUN_00503070(sync, &mut d.offset_0x20)?;
    FUN_00503070(sync, &mut d.offset_0x21)?;
    *g.larva(this) = d;
    Ok(())
}

/// port: 004d7ea0 Sexy::Larva::vfunction27
/// `Draw(Graphics*)`: the beetle cel (5 rows).
pub fn vfunction27(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    crate::game::food::FUN_004d6ae0(g, this);
    let img = g.res.DAT_005e8e60;
    let cel = g.larva(this).offset_0x1c;
    crate::sexy::graphics::FUN_00456980(gfx, g, img, 0, 0, cel, 5);
}

/// port: 004d7ed0 Sexy::Larva::vfunction55
/// `MouseDown(x, y, clicks)`: a left click collects it (the points sound); a right click
/// goes to the tank.
pub fn vfunction55(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32) {
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    if -1 < param_3 {
        g.larva(this).offset_0x20 = true;
        crate::game::board::FUN_005383d0(g, board);
        return;
    }
    let (mx, my) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
    crate::game::board_level::FUN_0053bfb0(g, board, mx + param_1, my + param_2);
}

/// port: 004d7e60 FUN_004d7e60
/// A beetle's value: 150 shells (7 in the virtual tank).
pub fn FUN_004d7e60(g: &mut G, this: Ptr) -> i32 {
    let app = g.go(this).offset_0x0;
    if g.wfa(app).offset_0x150 != 5 { 0x96 } else { 7 }
}

/// port: 004d7e80 FUN_004d7e80
/// The cel cycle (0..9).
pub fn FUN_004d7e80(g: &mut G, this: Ptr) {
    let d = g.larva(this);
    d.offset_0x1c = (d.offset_0x1c + 1) % 10;
}

/// port: 004f2930 Sexy::Larva::vfunction23
/// `Update()` (not while paused): clickable once above 320; rising until it leaves at the
/// top (y 64), or, collected, easing toward the money counter (550, 30) where it pays out
/// (+0x4a8 counts it).
pub fn vfunction23(g: &mut G, this: Ptr) {
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    if board == NULL || g.board(board).field_0x8 {
        return;
    }
    crate::game::game_object::FUN_004f22c0(g, this);
    if !g.w(this).offset_0x1 && g.larva(this).offset_0xc < 320.0 && !g.larva(this).offset_0x21 {
        let w = g.w(this);
        w.offset_0x1 = true;
        w.offset_0x28 = true;
    }
    if !g.larva(this).offset_0x20 {
        let d = g.larva(this);
        let y = d.offset_0xc - d.offset_0x14;
        d.offset_0xc = y;
        if y < 64.0 {
            vfunction76(g, this);
            return;
        }
        if 530.0 < d.offset_0x4 {
            d.offset_0x4 = 530.0;
        }
    } else {
        let d = g.larva(this);
        if d.offset_0x4 <= 550.0 {
            if d.offset_0x4 < 550.0 {
                d.offset_0x4 = (550.0 - d.offset_0x4) / 7.0 + d.offset_0x4;
            }
        } else {
            d.offset_0x4 -= (d.offset_0x4 - 550.0) / 7.0;
        }
        if d.offset_0xc <= 30.0 {
            if d.offset_0xc < 30.0 {
                d.offset_0xc = (30.0 - d.offset_0xc) / 7.0 + d.offset_0xc;
            }
        } else {
            d.offset_0xc -= (d.offset_0xc - 30.0) / 7.0;
        }
        if g.wc(this).offset_0x30 < 0x28 {
            vfunction76(g, this);
            let board = g.wfa(app).offset_0x4;
            g.board(board).ext_0x460[(0x4a8 - 0x460) / 4] += 1;
            let v = FUN_004d7e60(g, this);
            crate::game::board::FUN_0053c1e0(g, board, v);
            return;
        }
    }
    let d = g.larva(this).clone();
    let (x, y) = (crate::sexy::crt::ftol(d.offset_0x4) as i32, crate::sexy::crt::ftol(d.offset_0xc) as i32);
    vcall!(g, this, w.vfunction42, x, y);
    FUN_004d7e80(g, this);
}
