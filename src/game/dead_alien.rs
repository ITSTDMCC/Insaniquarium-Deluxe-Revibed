//! `Sexy::DeadAlien`: a defeated alien's body, drifting down with explosions for a while,
//! then fading. `DeadAlien_data` starts at object offset 0x154.

use crate::game::game_object::{FUN_004f22c0, GameObject};
use crate::sexy::crt::ftol;
use crate::sexy::graphics::{FUN_004558c0, FUN_004558e0, FUN_00455890, FUN_004560a0};
use crate::sexy::prelude::*;
use crate::sexy::types::CRect;

/// `DeadAlien_data` (object offset 0x154, 80 bytes).
#[derive(Debug, Clone, Default)]
pub struct DeadAlien_data {
    /// +0x158 facing right (drawn mirrored).
    pub offset_0x4: bool,
    /// +0x160 y.
    pub offset_0xc: f64,
    /// +0x168 x.
    pub offset_0x14: f64,
    /// +0x170 x speed.
    pub offset_0x1c: f64,
    /// +0x178 y speed (sinking up to 2).
    pub offset_0x24: f64,
    /// +0x180 (synced, unused).
    pub field_0x2c: f64,
    /// +0x188 the cel it died in.
    pub offset_0x34: i32,
    /// +0x190 alpha (1.0).
    pub offset_0x3c: f64,
    /// +0x198 life (125).
    pub offset_0x44: i32,
    /// +0x19c (5).
    pub offset_0x48: i32,
    /// +0x1a0 the alien kind.
    pub offset_0x4c: i32,
}

impl G {
    pub fn dead_alien(&mut self, p: Ptr) -> &mut DeadAlien_data {
        match &mut self.go_ext(p).sub {
            GoSub::DeadAlien(d) => d,
            s => panic!("{p} is not a DeadAlien: {s:?}"),
        }
    }
}

fn alloc_dead_alien(g: &mut G, wc: crate::sexy::object::WidgetContainer_data, w: crate::sexy::object::Widget_data,
                    go: crate::game::game_object::GameObject_data, d: DeadAlien_data) -> Ptr {
    g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__DeadAlien_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::GameObject(Box::new(GameObjectExt { go, sub: GoSub::DeadAlien(d) })) }),
    })
}

/// port: 004eebe0 Sexy::DeadAlien::DeadAlien
/// `DeadAlien::DeadAlien()` (used before loading from a save).
pub fn DeadAlien__004eebe0(g: &mut G) -> Ptr {
    let (wc, w, mut go) = GameObject(g);
    go.offset_0x4 = 0x1a;
    alloc_dead_alien(g, wc, w, go, DeadAlien_data::default())
}

/// port: 004eec10 Sexy::DeadAlien::DeadAlien
/// `DeadAlien(double x, double y, int cel, int kind, bool facingRight)`: 160x160, on the
/// board's widget manager right away.
pub fn DeadAlien__004eec10(g: &mut G, param_1: f64, param_2: f64, param_3: i32, param_4: i32, param_5: bool) -> Ptr {
    let (mut wc, mut w, mut go) = GameObject(g);
    go.offset_0x4 = 0x1a;
    wc.offset_0x2c = ftol(param_1) as i32;
    wc.offset_0x30 = ftol(param_2) as i32;
    let d = DeadAlien_data {
        offset_0xc: param_2,
        offset_0x14: param_1,
        offset_0x1c: 0.0,
        offset_0x24: 0.0,
        offset_0x4c: param_4,
        offset_0x44: 0x7d,
        offset_0x48: 5,
        offset_0x4: param_5,
        offset_0x34: param_3,
        ..Default::default()
    };
    wc.offset_0x34 = 0xa0;
    wc.offset_0x38 = 0xa0;
    w.offset_0x1 = false;
    let app = go.offset_0x0;
    let this = alloc_dead_alien(g, wc, w, go, d);
    let board = g.wfa(app).offset_0x4;
    let mgr = g.wc(board).offset_0xc;
    vcall!(g, mgr, w.vfunction4, this);
    g.dead_alien(this).offset_0x3c = 1.0;
    this
}

/// port: 004ddd00 Sexy::DeadAlien::vfunction81
/// `Sync(DataSync&)`.
pub fn vfunction81(g: &mut G, this: Ptr, sync: &mut crate::sexy::data_sync::DataSync) -> crate::sexy::data_sync::SyncResult {
    crate::game::game_object::vfunction81(g, this, sync)?;
    use crate::sexy::data_sync::{FUN_00503010, FUN_00503070, FUN_005030a0};
    let mut d = g.dead_alien(this).clone();
    FUN_00503070(sync, &mut d.offset_0x4)?;
    FUN_005030a0(sync, &mut d.offset_0xc)?;
    FUN_005030a0(sync, &mut d.offset_0x14)?;
    FUN_005030a0(sync, &mut d.offset_0x1c)?;
    FUN_005030a0(sync, &mut d.offset_0x24)?;
    FUN_005030a0(sync, &mut d.field_0x2c)?;
    FUN_00503010(sync, &mut d.offset_0x34)?;
    FUN_005030a0(sync, &mut d.offset_0x3c)?;
    FUN_00503010(sync, &mut d.offset_0x44)?;
    FUN_00503010(sync, &mut d.offset_0x48)?;
    FUN_00503010(sync, &mut d.offset_0x4c)?;
    *g.dead_alien(this) = d;
    Ok(())
}

/// port: 004eec00 Sexy::DeadAlien::vfunction76
/// `Remove()`: a thunk through its vtable slot 82, which holds the shared
/// `Larva::vfunction76` (0x4d9570 at vtable +0x144).
pub fn vfunction76__004eec00(g: &mut G, this: Ptr) {
    crate::game::larva::vfunction76(g, this);
}

/// port: 004e54f0 FUN_004e54f0
/// Whether it is the freshest dead alien (most life left), the one that makes the sound.
pub fn FUN_004e54f0(g: &mut G, this: Ptr) -> bool {
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    let mut most = 0;
    for o in g.board(board).offset_0xc[crate::game::board_level::vec_index(0x9c)].clone() {
        let l = g.dead_alien(o).offset_0x44;
        if most < l {
            most = l;
        }
    }
    most <= g.dead_alien(this).offset_0x44
}

/// port: 004f6050 Sexy::DeadAlien::vfunction23
/// `Update()` (not while paused): removed when its life runs out; fades below 105;
/// explosions (softer as it dies) and bubbles and sparkles every few updates while above
/// 50; drifts (x speed swings around 0) and sinks, staying in the tank.
pub fn vfunction23(g: &mut G, this: Ptr) {
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    if board == NULL || g.board(board).field_0x8 {
        return;
    }
    FUN_004f22c0(g, this);
    let life = g.dead_alien(this).offset_0x44;
    if life < 1 {
        vcall!(g, this, go.vfunction76);
        return;
    }
    let life = life - 1;
    let d = g.dead_alien(this);
    d.offset_0x44 = life;
    if life < 0x69 {
        d.offset_0x3c -= 0.02;
        if d.offset_0x3c < 0.0 {
            d.offset_0x3c = 0.0;
        }
    }
    if life % 5 == 0 && 0x32 < life && FUN_004e54f0(g, this) {
        let v = crate::game::help_screen::FUN_005036e0(0x19, 100, g.dead_alien(this).offset_0x44 - 0x32, 0x4b, false).clamp(0x19, 100);
        crate::game::board::FUN_00538230(g, board, 0x11d, 3, v as f64 / 100.0);
    }
    let life = g.dead_alien(this).offset_0x44;
    if life % 7 == 0 && 0x32 < life {
        let rng = g.wfa(app).offset_0x84;
        let (mx, my) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
        let rnd = |g: &mut G| crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng));
        let y = (rnd(g) % 0x46) as i32 + 0x1e + my;
        let x = (rnd(g) % 0x46) as i32 + 0x1e + mx;
        crate::game::board_update::FUN_00538ac0(g, board, x, y);
        let y = (rnd(g) % 0x32) as i32 + 0x32 + my;
        let x = (rnd(g) % 0x32) as i32 + 0x32 + mx;
        crate::game::board_update::FUN_00538ac0(g, board, x, y);
        for (k_rand, m, a) in [(true, 0x32, 0x14), (true, 0x1e, 0x28), (false, 0x32, 0x14), (false, 0x1e, 0x28)] {
            let k = if k_rand { (rnd(g) % 3 + 3) as i32 } else { ((rnd(g) & 1) + 6) as i32 };
            let y = (rnd(g) % m) as i32 + a + my;
            let x = (rnd(g) % m) as i32 + a + mx;
            crate::game::board_level::FUN_005436f0(g, board, x, y, k);
        }
    }
    let d = g.dead_alien(this);
    d.offset_0x1c = if d.offset_0x1c <= 0.0 { d.offset_0x1c + 0.03 } else { d.offset_0x1c - 0.03 };
    if d.offset_0x24 < 2.0 {
        d.offset_0x24 += 0.05;
    }
    if 490.0 < d.offset_0x14 {
        d.offset_0x14 = 490.0;
    }
    if d.offset_0x14 < -10.0 {
        d.offset_0x14 = -10.0;
    }
    if (d.offset_0x4c == 5 || d.offset_0x4c == 6) && 280.0 < d.offset_0xc {
        d.offset_0xc = 280.0;
    }
    d.offset_0x14 += d.offset_0x1c;
    d.offset_0xc += d.offset_0x24;
    let (x, y) = (d.offset_0x14, d.offset_0xc);
    let (ix, iy) = (ftol(x) as i32, ftol(y) as i32);
    vcall!(g, this, w.vfunction42, ix, iy);
}

/// port: 004dddb0 Sexy::DeadAlien::vfunction27
/// `Draw(Graphics*)`: the kind's dead sheet (resource 0xbd..0xc5), faded; while it still
/// explodes (life above 115) an additive glow drawn five times.
pub fn vfunction27(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    crate::game::food::FUN_004d6ae0(g, this);
    let d = g.dead_alien(this).clone();
    FUN_004558e0(gfx, true);
    FUN_00455890(gfx, CRect(0xff, 0xff, 0xff, ftol(d.offset_0x3c * 255.0) as i32));
    let id = match d.offset_0x4c {
        1 | 2 => 0xbd,
        3 => 0xbe,
        5 => 0xc0,
        6 => 0xc1,
        7 => 0xc2,
        0x15 => 0xc5,
        // (The original passes its Graphics pointer as the resource id here; Gus and the
        // small alien never leave a body, so no kind reaches it.)
        k => crate::sexy::pending(0x004dddb0, &format!("DeadAlien::Draw for kind {k}")),
    };
    let img = crate::sexy::res::FUN_005016a0(g, id) as Ptr;
    let src = Rect::new(d.offset_0x34 * 0xa0, 0, 0xa0, 0xa0);
    FUN_004560a0(gfx, g, img, 0, 0, &src, d.offset_0x4);
    if 0x73 < d.offset_0x44 {
        FUN_004558c0(gfx, 1);
        FUN_00455890(gfx, CRect(0xff, 0xff, 0xff, (d.offset_0x44 - 0x73) * 0x14));
        for _ in 0..5 {
            let img = crate::sexy::res::FUN_005016a0(g, id) as Ptr;
            FUN_004560a0(gfx, g, img, 0, 0, &src, d.offset_0x4);
        }
        FUN_004558c0(gfx, 0);
    }
    FUN_004558e0(gfx, false);
}
