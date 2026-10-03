//! `Sexy::Shadow`: the shadow a fish, pet or alien casts on the tank floor. `Shadow_data`
//! starts at object offset 0x154.

use crate::game::game_object::{FUN_004f22c0, GameObject};
use crate::sexy::graphics::{FUN_004558e0, FUN_00455890, FUN_00455900, FUN_00455d20, FUN_004560a0, FUN_004561c0};
use crate::sexy::prelude::*;

/// `Shadow_data` (object offset 0x154).
#[derive(Debug, Clone, Default)]
pub struct Shadow_data {
    /// +0x154.
    pub offset_0x0: i32,
    /// +0x158 the owner (its +0x94 points back here).
    pub offset_0x4: Ptr,
    /// +0x15c kind: 0 under the owner, 1 full-screen image, 2 offset 40 pixels right.
    pub offset_0x8: i32,
    /// +0x160 darkness from the owner's height ((owner y - 50) / 2).
    pub offset_0xc: i32,
    /// +0x168 darkness scale (1.0).
    pub offset_0x14: f64,
}

impl G {
    pub fn shadow(&mut self, p: Ptr) -> &mut Shadow_data {
        match &mut self.go_ext(p).sub {
            GoSub::Shadow(d) => d,
            s => panic!("{p} is not a Shadow: {s:?}"),
        }
    }
}

fn alloc_shadow(g: &mut G, wc: crate::sexy::object::WidgetContainer_data, w: crate::sexy::object::Widget_data,
                go: crate::game::game_object::GameObject_data, d: Shadow_data) -> Ptr {
    g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__Shadow_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::GameObject(Box::new(GameObjectExt { go, sub: GoSub::Shadow(d) })) }),
    })
}

/// port: 004ec4b0 Sexy::Shadow::Shadow
/// `Shadow::Shadow()` (used before loading from a save).
pub fn Shadow__004ec4b0(g: &mut G) -> Ptr {
    let (mut wc, w, mut go) = GameObject(g);
    wc.offset_0x3d = false;
    go.offset_0x4 = 0x1f;
    alloc_shadow(g, wc, w, go, Shadow_data::default())
}

/// port: 004ec4f0 Sexy::Shadow::Shadow
/// `Shadow::Shadow(int kind, GameObject* owner)`: 80x40 near the floor.
pub fn Shadow__004ec4f0(g: &mut G, param_1: i32, param_2: Ptr) -> Ptr {
    let (mut wc, mut w, mut go) = GameObject(g);
    go.offset_0x4 = 0x1f;
    let mut d = Shadow_data { offset_0x4: param_2, ..Default::default() };
    let this_id = g.objs.len() as Ptr;
    if param_2 == NULL {
        d.offset_0xc = 0;
    } else {
        d.offset_0xc = (g.wc(param_2).offset_0x30 - 0x32) / 2;
    }
    d.offset_0x14 = 1.0;
    wc.offset_0x2c = -200;
    let app = go.offset_0x0;
    let rng = g.wfa(app).offset_0x84;
    let r = crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng));
    wc.offset_0x34 = 0x50;
    wc.offset_0x38 = 0x28;
    w.offset_0x1 = false;
    wc.offset_0x3d = false;
    wc.offset_0x30 = (r % 5) as i32 + 0x19a;
    d.offset_0x8 = param_1;
    let this = alloc_shadow(g, wc, w, go, d);
    debug_assert_eq!(this, this_id);
    if param_2 != NULL {
        g.go(param_2).offset_0xc = this;
    }
    this
}

/// port: 004f3d00 Sexy::Shadow::vfunction23
/// `Update()`: follows the owner's x (40 pixels right for kind 2) and, for kinds 0 and 2,
/// sits lower the higher the owner swims.
pub fn vfunction23(g: &mut G, this: Ptr) {
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    if board != NULL && !g.board(board).field_0x8 {
        FUN_004f22c0(g, this);
        let owner = g.shadow(this).offset_0x4;
        if owner != NULL {
            let (oy, ox) = (g.wc(owner).offset_0x30, g.wc(owner).offset_0x2c);
            g.shadow(this).offset_0xc = (oy - 0x32) / 2;
            let kind = g.shadow(this).offset_0x8;
            let mut x = ox;
            if kind == 2 {
                x += 0x28;
            }
            g.wc(this).offset_0x2c = x;
            if kind == 0 || kind == 2 {
                let h = (0x172 - oy).max(0);
                g.wc(this).offset_0x30 = 0x19f - (h * 0x1e) / 0x172;
            }
        }
    }
}

/// port: 004e31f0 Sexy::Shadow::vfunction27
/// `Draw(Graphics*)`: IMAGE shadow in the backdrop's tint, its alpha from the owner's
/// height (capped at 150, or 255 for kind 2), scaled up (with 3D) as the owner rises.
/// Nothing for a dropping owner of kind 8/9 (the colorize flag is left set, as in the
/// original).
pub fn vfunction27(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    let ftol = crate::sexy::crt::ftol;
    crate::game::food::FUN_004d6ae0(g, this);
    FUN_004558e0(gfx, true);
    let owner = g.shadow(this).offset_0x4;
    if g.go(owner).offset_0x74 {
        let t = g.go(owner).offset_0x4;
        if 8 <= t && t <= 9 {
            return;
        }
    }
    let d = g.shadow(this).clone();
    let mut c = g.globals.DAT_005e89e8;
    let (v, max) = if d.offset_0x8 == 2 {
        ((d.offset_0xc * 2) as f64 * d.offset_0x14, 255.0)
    } else {
        (d.offset_0xc as f64 * d.offset_0x14, 150.0)
    };
    let a = if 0.0 <= v {
        if max < v { max } else { v }
    } else {
        0.0
    };
    c.mAlpha = ftol(a) as i32;
    FUN_00455890(gfx, c);
    let img = g.res.DAT_005e8da4;
    if d.offset_0x8 == 1 {
        FUN_00455d20(gfx, g, img, 0, 0);
        FUN_004558e0(gfx, false);
        return;
    }
    let (w, h) = (g.image(img).offset_0x20, g.image(img).offset_0x24);
    let src = Rect::new(0, 0, w, h);
    let mut dest = Rect::new(0, 0, w, h);
    if owner != NULL {
        let app = g.go(this).offset_0x0;
        if crate::sexy::sexy_app_base::dtor_MemoryImage__00489a20(g, app) {
            let dy = g.wc(this).offset_0x30 - g.wc(owner).offset_0x30;
            let mut t = (dy as f64 / 100.0) as f32;
            if (t as f64) < f32::from_bits(0x3f666666) as f64 {
                t = f32::from_bits(0x3f666666);
            } else if f32::from_bits(0x3fa66666) < t {
                t = f32::from_bits(0x3fa66666);
            }
            if d.offset_0x8 == 2 {
                t = (t as f64 + f32::from_bits(0x3ecccccd) as f64) as f32;
            }
            let k = t as f64 - 1.0;
            let ex = ftol(w as f64 * k) as i32;
            let ey = ftol(k * h as f64) as i32;
            dest.mX -= ex;
            dest.mY -= ey;
            dest.mWidth = w + ex * 2;
            dest.mHeight = h + ey * 2;
        }
    }
    FUN_00455900(gfx, false);
    FUN_005008f0(g, gfx, img, &dest, &src, false);
    FUN_004558e0(gfx, false);
}

/// port: 005008f0 FUN_005008f0
/// `DrawImageMirrorRect(Graphics*, Image*, const Rect& dest, const Rect& src, bool mirror)`:
/// unscaled when the sizes match, stretched otherwise.
pub fn FUN_005008f0(g: &mut G, param_1: &mut Graphics, param_2: Ptr, param_3: &Rect, param_4: &Rect, param_5: bool) {
    if param_3.mWidth == param_4.mWidth && param_3.mHeight == param_4.mHeight {
        FUN_004560a0(param_1, g, param_2, param_3.mX, param_3.mY, param_4, param_5);
        return;
    }
    FUN_004561c0(param_1, param_2, param_3, param_4, param_5);
}

/// port: 004d90e0 Sexy::Shadow::vfunction75
/// Forgets the owner (and the owner forgets it).
pub fn vfunction75(g: &mut G, this: Ptr) {
    let owner = g.shadow(this).offset_0x4;
    if owner != NULL {
        g.go(owner).offset_0xc = NULL;
        g.shadow(this).offset_0x4 = NULL;
    }
}

/// port: 004d9080 Sexy::Shadow::vfunction76
/// `Remove()`: unlinks from the owner, leaves the widget manager, is safe-deleted (app
/// vtable +0x88) and leaves the board.
pub fn vfunction76(g: &mut G, this: Ptr) {
    vfunction75(g, this);
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    let mgr = g.wc(board).offset_0xc;
    vcall!(g, mgr, w.vfunction5, this);
    crate::sexy::sexy_app_base::vfunction35(g, app, this);
    crate::game::board_level::FUN_00541d90(g, board, this, false);
}

/// port: 004d9030 Sexy::Shadow::vfunction81
/// `Sync(DataSync&)`.
pub fn vfunction81(g: &mut G, this: Ptr, sync: &mut crate::sexy::data_sync::DataSync) -> crate::sexy::data_sync::SyncResult {
    crate::game::game_object::vfunction81(g, this, sync)?;
    use crate::sexy::data_sync::{FUN_00503010, FUN_005030a0};
    let mut d = g.shadow(this).clone();
    FUN_00503010(sync, &mut d.offset_0x8)?;
    FUN_00503010(sync, &mut d.offset_0xc)?;
    FUN_005030a0(sync, &mut d.offset_0x14)?;
    *g.shadow(this) = d;
    crate::sexy::data_sync::FUN_005122f0(sync, crate::sexy::data_sync::PtrSlot::ShadowOwner(this));
    Ok(())
}
