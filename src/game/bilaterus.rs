//! `Sexy::Bilaterus`: the two-headed worm alien (alien wave 8). The body object (type 0x17,
//! filed in the board's +0xf4 vector) owns two `BilaterusHead`s and six `BilaterusBone`s,
//! which are widgets of the app's widget manager but not board objects: the body's update
//! drives them. The leading head (+0x15e) steers and drags the body widget along; the
//! bones each follow the link before them, the trailing head follows the last bone. Every
//! 1000 updates (or when the leading head dies) the worm turns around: the heads swap
//! roles and the bone chain is reversed. Only the leading head (`Bilaterus +0x158`) has the
//! health that counts (+0x1d8).
//!
//! `Bilaterus_data` starts at object offset 0x154 (object 0x174 bytes), `BilaterusHead_data`
//! at 0x154 (object 0x208), `BilaterusBone_data` at 0x154 (object 0x1f0); field names are
//! relative to it.
//!
//! The original keeps the bones in a heap-allocated `std::vector<BilaterusBone*>*` (+0x160)
//! that is null only if its allocation fails; here it is a plain `Vec`.

use crate::game::board_level::FUN_005420e0;
use crate::game::game_object::GameObject;
use crate::sexy::crt::ftol;
use crate::sexy::graphics::{FUN_00455890, FUN_004558c0, FUN_004558e0, FUN_00455900, FUN_00455d20, FUN_00455e40, FUN_004560a0, FUN_004561c0};
use crate::sexy::prelude::*;
use crate::sexy::types::CRect;

/// `Bilaterus_data` (object offset 0x154).
#[derive(Debug, Clone, Default)]
pub struct Bilaterus_data {
    /// +0x158 the leading head.
    pub offset_0x4: Ptr,
    /// +0x15c the trailing head.
    pub offset_0x8: Ptr,
    /// +0x160 the six bones, from the leading head back.
    pub offset_0xc: Vec<Ptr>,
    /// +0x164 dying: the leading head is gone, the next head kill finishes it.
    pub offset_0x10: bool,
    /// +0x168 (8; unused).
    pub offset_0x14: i32,
    /// +0x16c updates before it starts moving (15; the parts shrink in while it counts).
    pub offset_0x18: i32,
    /// +0x170 updates since the last turn-around.
    pub offset_0x1c: i32,
}

/// `BilaterusHead_data` (object offset 0x154).
#[derive(Debug, Clone, Default)]
pub struct BilaterusHead_data {
    /// +0x158 the body.
    pub offset_0x4: Ptr,
    /// +0x15c (trailing head) facing left, behind the last bone.
    pub offset_0x8: bool,
    /// +0x15d just turned around (suppresses the turning animation once).
    pub offset_0x9: bool,
    /// +0x15e the leading head.
    pub offset_0xa: bool,
    /// +0x160 which head this was made as (0 the first leader, 1 the other; picks the
    /// sprite rows).
    pub offset_0xc: i32,
    /// +0x168 x.
    pub offset_0x14: f64,
    /// +0x170 y.
    pub offset_0x1c: f64,
    /// +0x178 x speed.
    pub offset_0x24: f64,
    /// +0x180 y speed.
    pub offset_0x2c: f64,
    /// +0x188 the x speed the wander mode wants.
    pub offset_0x34: f64,
    /// +0x190 the y speed the wander mode wants.
    pub offset_0x3c: f64,
    /// +0x198 speed divisor (0.8).
    pub offset_0x44: f64,
    /// +0x1a0 facing (sign).
    pub offset_0x4c: f64,
    /// +0x1a8 lowest y (370).
    pub offset_0x54: i32,
    /// +0x1ac leftmost x (10).
    pub offset_0x58: i32,
    /// +0x1b0 highest y (95).
    pub offset_0x5c: i32,
    /// +0x1b4 rightmost x (540).
    pub offset_0x60: i32,
    /// +0x1b8 wander mode (0 left, 1 right, 2 up, 3 down; others keep the speed).
    pub offset_0x64: i32,
    /// +0x1bc updates (counted up while wandering; 40 at first).
    pub offset_0x68: i32,
    /// +0x1c0 updates since the last wander-mode roll.
    pub offset_0x6c: i32,
    /// +0x1c4 swim animation counter (0..19).
    pub offset_0x70: i32,
    /// +0x1c8 cel.
    pub offset_0x74: i32,
    /// +0x1cc turning animation (counts from ±20 to 0).
    pub offset_0x78: i32,
    /// +0x1d0 hit flash.
    pub offset_0x7c: i32,
    /// +0x1d4 updates before it may bite (100).
    pub offset_0x80: i32,
    /// +0x1d8 health (100).
    pub offset_0x84: f64,
    /// +0x1e0 the last bone's x (trailing head).
    pub offset_0x8c: f64,
    /// +0x1e8 the last bone's y.
    pub offset_0x94: f64,
    /// +0x1f0 the last bone's x speed (sampled every 12 updates).
    pub offset_0x9c: f64,
    /// +0x1f8 the last bone's y speed.
    pub offset_0xa4: f64,
    /// +0x200 updates since the last sample.
    pub offset_0xac: i32,
}

/// `BilaterusBone_data` (object offset 0x154).
#[derive(Debug, Clone, Default)]
pub struct BilaterusBone_data {
    /// +0x158 the body.
    pub offset_0x4: Ptr,
    /// +0x15c index in the chain (0 behind the leading head).
    pub offset_0x8: i32,
    /// +0x160 x.
    pub offset_0xc: f64,
    /// +0x168 y.
    pub offset_0x14: f64,
    /// +0x170 x speed.
    pub offset_0x1c: f64,
    /// +0x178 y speed.
    pub offset_0x24: f64,
    /// +0x180.
    pub offset_0x2c: f64,
    /// +0x188.
    pub offset_0x34: f64,
    /// +0x190 speed divisor (0.8).
    pub offset_0x3c: f64,
    /// +0x198 facing (sign).
    pub offset_0x44: f64,
    /// +0x1a0 lowest y (370).
    pub offset_0x4c: i32,
    /// +0x1a4 leftmost x (10).
    pub offset_0x50: i32,
    /// +0x1a8 highest y (95).
    pub offset_0x54: i32,
    /// +0x1ac rightmost x (540).
    pub offset_0x58: i32,
    /// +0x1b0.
    pub offset_0x5c: i32,
    /// +0x1b4.
    pub offset_0x60: i32,
    /// +0x1b8 (negated on each turn-around; otherwise unused).
    pub offset_0x64: i32,
    /// +0x1bc hit flash.
    pub offset_0x68: i32,
    /// +0x1c0 updates before it may bite (100).
    pub offset_0x6c: i32,
    /// +0x1c8 the link ahead's x.
    pub offset_0x74: f64,
    /// +0x1d0 the link ahead's y.
    pub offset_0x7c: f64,
    /// +0x1d8 the link ahead's x speed (sampled every 12 updates).
    pub offset_0x84: f64,
    /// +0x1e0 the link ahead's y speed.
    pub offset_0x8c: f64,
    /// +0x1e8 updates since the last sample.
    pub offset_0x94: i32,
}

impl G {
    pub fn bilaterus(&mut self, p: Ptr) -> &mut Bilaterus_data {
        match &mut self.go_ext(p).sub {
            GoSub::Bilaterus(d) => d,
            s => panic!("{p} is not a Bilaterus: {s:?}"),
        }
    }
    pub fn bil_head(&mut self, p: Ptr) -> &mut BilaterusHead_data {
        match &mut self.go_ext(p).sub {
            GoSub::BilaterusHead(d) => d,
            s => panic!("{p} is not a BilaterusHead: {s:?}"),
        }
    }
    pub fn bil_bone(&mut self, p: Ptr) -> &mut BilaterusBone_data {
        match &mut self.go_ext(p).sub {
            GoSub::BilaterusBone(d) => d,
            s => panic!("{p} is not a BilaterusBone: {s:?}"),
        }
    }
}

fn alloc_part(g: &mut G, vt: &'static crate::sexy::vtables_gen::VTable, wc: crate::sexy::object::WidgetContainer_data,
              w: crate::sexy::object::Widget_data, go: crate::game::game_object::GameObject_data, sub: GoSub) -> Ptr {
    g.alloc(Obj { vt: Some(vt), node: Node::Widget(WidgetObj { wc, w, ext: WExt::GameObject(Box::new(GameObjectExt { go, sub })) }) })
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

/// The app's widget manager (`*(app +800)`).
fn app_manager(g: &mut G, this: Ptr) -> Ptr {
    let app = g.go(this).offset_0x0;
    g.sab(app).offset_0x318
}

fn paused(g: &mut G, this: Ptr) -> bool {
    let board = board_of(g, this);
    board == NULL || g.board(board).field_0x8
}

fn sound(g: &mut G, this: Ptr, id: i32) {
    let board = board_of(g, this);
    crate::game::board::FUN_00538230(g, board, id, 3, 1.0);
}

/// The body of a multi-part alien (an entry of the board's +0xf4 vector) is its leading head
/// (`part +0x158`).
pub fn alien_part_body(g: &mut G, part: Ptr) -> Ptr {
    g.bilaterus(part).offset_0x4
}

/// A hit on a multi-part alien hurts its leading head (`*(part +0x158) +0x1d8 -= dmg`); the
/// health after.
pub fn alien_part_body_hurt(g: &mut G, part: Ptr, dmg: f64) -> f64 {
    let head = g.bilaterus(part).offset_0x4;
    let d = g.bil_head(head);
    d.offset_0x84 -= dmg;
    d.offset_0x84
}

/// The leading head's hit flash (`*(part +0x158) +0x1d0 = n`).
pub fn alien_part_body_flash(g: &mut G, part: Ptr, n: i32) {
    let head = g.bilaterus(part).offset_0x4;
    g.bil_head(head).offset_0x7c = n;
}

/// port: 004f98b0 Sexy::Bilaterus::Bilaterus
/// `Bilaterus::Bilaterus()` (used before loading from a save): no parts yet.
pub fn Bilaterus__004f98b0(g: &mut G) -> Ptr {
    let (wc, w, mut go) = GameObject(g);
    go.offset_0x4 = 0x17;
    alloc_part(g, &crate::sexy::vtables_gen::Sexy__Bilaterus_vftable, wc, w, go, GoSub::Bilaterus(Bilaterus_data::default()))
}

/// port: 004fe000 Sexy::Bilaterus::Bilaterus
/// `Bilaterus(int x, int y)`: both heads and the six bones at (x, y), each added to the
/// app's widget manager; not mouse-visible; still for 15 updates; heading a random way
/// (the turn-around plays its sound, as does the other way).
pub fn Bilaterus__004fe000(g: &mut G, param_1: i32, param_2: i32) -> Ptr {
    let (wc, w, mut go) = GameObject(g);
    go.offset_0x4 = 0x17;
    let d = Bilaterus_data { offset_0x14: 8, ..Default::default() };
    let this = alloc_part(g, &crate::sexy::vtables_gen::Sexy__Bilaterus_vftable, wc, w, go, GoSub::Bilaterus(d));
    let h = BilaterusHead(g, this, param_1, param_2, true);
    g.bilaterus(this).offset_0x4 = h;
    let mgr = app_manager(g, this);
    vcall!(g, mgr, w.vfunction4, h);
    let h = BilaterusHead(g, this, param_1, param_2, false);
    g.bilaterus(this).offset_0x8 = h;
    let mgr = app_manager(g, this);
    vcall!(g, mgr, w.vfunction4, h);
    for i in 0..6 {
        let b = BilaterusBone(g, this, param_1, param_2, i);
        let mgr = app_manager(g, this);
        vcall!(g, mgr, w.vfunction4, b);
        FUN_005420e0(&mut g.bilaterus(this).offset_0xc, b);
    }
    g.bilaterus(this).offset_0x10 = false;
    g.w(this).offset_0x1 = false;
    g.bilaterus(this).offset_0x1c = 0;
    g.bilaterus(this).offset_0x18 = 0xf;
    if rand(g, this) & 1 == 0 {
        FUN_004fb1c0(g, this);
    } else {
        sound(g, this, 0x131);
    }
    this
}

/// port: 004ee210 Sexy::BilaterusHead::BilaterusHead
/// `BilaterusHead(Bilaterus* body, int x, int y, bool leading)`: 80x80 at (x, y), sinking,
/// facing a random way, a random wander mode, health 100; not mouse-visible.
pub fn BilaterusHead(g: &mut G, param_1: Ptr, param_2: i32, param_3: i32, param_4: bool) -> Ptr {
    let (mut wc, w, go) = GameObject(g);
    wc.offset_0x3d = false;
    let d = BilaterusHead_data { offset_0x14: param_2 as f64, offset_0x1c: param_3 as f64, offset_0x4: param_1, offset_0xa: param_4, ..Default::default() };
    wc.offset_0x2c = ftol(d.offset_0x14) as i32;
    wc.offset_0x30 = ftol(d.offset_0x1c) as i32;
    let this = alloc_part(g, &crate::sexy::vtables_gen::Sexy__BilaterusHead_vftable, wc, w, go, GoSub::BilaterusHead(d));
    {
        let d = g.bil_head(this);
        d.offset_0x24 = 0.0;
        d.offset_0x2c = -0.5;
        d.offset_0x4c = 1.0;
    }
    g.wc(this).offset_0x34 = 0x50;
    g.wc(this).offset_0x38 = 0x50;
    if rand(g, this) & 1 == 0 {
        let d = g.bil_head(this);
        d.offset_0x24 = f64::from_bits(0xbfb999999999999a);
        d.offset_0x4c = -1.0;
    }
    {
        let d = g.bil_head(this);
        d.offset_0x34 = 0.0;
        d.offset_0x54 = 0x172;
        d.offset_0x3c = 0.0;
        d.offset_0x5c = 0x5f;
        d.offset_0x58 = 10;
        d.offset_0x44 = f64::from_bits(0x3fe999999999999a);
        d.offset_0x60 = 0x21c;
    }
    let r = rand(g, this);
    {
        let d = g.bil_head(this);
        d.offset_0x84 = 100.0;
        d.offset_0x8c = 0.0;
        d.offset_0x94 = 0.0;
        d.offset_0x9c = 0.0;
        d.offset_0xa4 = 0.0;
        d.offset_0x68 = 0x28;
        d.offset_0x6c = 0;
        d.offset_0x70 = 0;
        d.offset_0x74 = 0;
        d.offset_0x78 = 0;
    }
    g.w(this).offset_0x1 = false;
    let d = g.bil_head(this);
    d.offset_0xac = 0xc;
    d.offset_0x80 = 100;
    d.offset_0x7c = 0;
    d.offset_0x8 = false;
    d.offset_0x9 = false;
    d.offset_0x64 = (r % 10) as i32;
    d.offset_0xc = (!d.offset_0xa) as i32;
    this
}

/// port: 004edd30 Sexy::BilaterusBone::BilaterusBone
/// `BilaterusBone(Bilaterus* body, int x, int y, int index)`: 80x80 at (x, y), sinking,
/// facing a random way; not mouse-visible.
pub fn BilaterusBone(g: &mut G, param_1: Ptr, param_2: i32, param_3: i32, param_4: i32) -> Ptr {
    let (wc, w, go) = GameObject(g);
    let d = BilaterusBone_data { offset_0xc: param_2 as f64, offset_0x4: param_1, offset_0x14: param_3 as f64, offset_0x8: param_4, ..Default::default() };
    let mut wc = wc;
    wc.offset_0x2c = ftol(d.offset_0xc) as i32;
    wc.offset_0x30 = ftol(d.offset_0x14) as i32;
    let this = alloc_part(g, &crate::sexy::vtables_gen::Sexy__BilaterusBone_vftable, wc, w, go, GoSub::BilaterusBone(d));
    {
        let d = g.bil_bone(this);
        d.offset_0x1c = 0.0;
        d.offset_0x24 = -0.5;
        d.offset_0x44 = 1.0;
    }
    g.wc(this).offset_0x34 = 0x50;
    g.wc(this).offset_0x38 = 0x50;
    if rand(g, this) & 1 == 0 {
        let d = g.bil_bone(this);
        d.offset_0x1c = f64::from_bits(0xbfb999999999999a);
        d.offset_0x44 = -1.0;
    }
    {
        let d = g.bil_bone(this);
        d.offset_0x4c = 0x172;
        d.offset_0x2c = 0.0;
        d.offset_0x54 = 0x5f;
        d.offset_0x34 = 0.0;
        d.offset_0x50 = 10;
        d.offset_0x58 = 0x21c;
        d.offset_0x3c = f64::from_bits(0x3fe999999999999a);
        d.offset_0x6c = 100;
        d.offset_0x5c = 0;
        d.offset_0x60 = 0;
        d.offset_0x74 = 0.0;
        d.offset_0x64 = 0;
        d.offset_0x7c = 0.0;
    }
    g.w(this).offset_0x1 = false;
    let d = g.bil_bone(this);
    d.offset_0x84 = 0.0;
    d.offset_0x68 = 0;
    d.offset_0x8c = 0.0;
    d.offset_0x94 = 0xc;
    this
}

/// port: 004f06c0 Sexy::Bilaterus::~Bilaterus
/// Deletes the bones, then both heads.
pub fn dtor_Bilaterus(g: &mut G, this: Ptr) {
    g.set_vt(this, &crate::sexy::vtables_gen::Sexy__Bilaterus_vftable);
    let bones = std::mem::take(&mut g.bilaterus(this).offset_0xc);
    for b in bones {
        if b != NULL {
            vcall!(g, b, w.vfunction1, 1);
        }
    }
    let h = g.bilaterus(this).offset_0x4;
    if h != NULL {
        vcall!(g, h, w.vfunction1, 1);
    }
    let h = g.bilaterus(this).offset_0x8;
    if h != NULL {
        vcall!(g, h, w.vfunction1, 1);
    }
    crate::game::game_object::dtor_GameObject(g, this);
}

/// port: 004f0e70 Sexy::Bilaterus::deleting_destructor
pub fn deleting_destructor(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_Bilaterus(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 004dbe40 Sexy::Bilaterus::vfunction18
/// `MarkDirty()`: also the heads and the bones.
pub fn vfunction18(g: &mut G, this: Ptr) {
    crate::sexy::widget_container::vfunction18(g, this);
    let h = g.bilaterus(this).offset_0x4;
    if h != NULL {
        vcall!(g, h, w.vfunction18);
    }
    for i in 0..6 {
        let b = g.bilaterus(this).offset_0xc[i];
        vcall!(g, b, w.vfunction18);
    }
    let h = g.bilaterus(this).offset_0x8;
    if h != NULL {
        vcall!(g, h, w.vfunction18);
    }
}

/// port: 004dbf20 Sexy::Bilaterus::vfunction22
/// `RemovedFromManager(WidgetManager*)`: takes the heads and the bones out of it too.
pub fn vfunction22(g: &mut G, this: Ptr, param_1: Ptr) {
    crate::sexy::widget_container::vfunction22(g, this, param_1);
    let h = g.bilaterus(this).offset_0x4;
    if h != NULL {
        vcall!(g, param_1, w.vfunction5, h);
    }
    for i in 0..6 {
        let b = g.bilaterus(this).offset_0xc[i];
        vcall!(g, param_1, w.vfunction5, b);
    }
    let h = g.bilaterus(this).offset_0x8;
    if h != NULL {
        vcall!(g, param_1, w.vfunction5, h);
    }
}

/// port: 004dbeb0 Sexy::Bilaterus::vfunction31
/// Brings the parts to the front of its manager: the trailing head, the bones from the
/// back, the leading head on top.
pub fn vfunction31(g: &mut G, this: Ptr) {
    crate::sexy::trivial::vfunction2__00486bf0();
    let mgr = g.wc(this).offset_0xc;
    let h = g.bilaterus(this).offset_0x8;
    if h != NULL {
        vcall!(g, mgr, w.vfunction12, h);
    }
    for i in (0..6).rev() {
        let mgr = g.wc(this).offset_0xc;
        let b = g.bilaterus(this).offset_0xc[i];
        vcall!(g, mgr, w.vfunction12, b);
    }
    let mgr = g.wc(this).offset_0xc;
    let h = g.bilaterus(this).offset_0x4;
    vcall!(g, mgr, w.vfunction12, h);
}

/// port: 004d4ec0 Sexy::Bilaterus::vfunction71
/// `CountKill(int stats[])`: an alien (slots 0, 3, 4).
pub fn vfunction71(_g: &mut G, _this: Ptr, param_1: &mut [i32]) {
    param_1[0] += 1;
    param_1[3] += 1;
    param_1[4] += 1;
}

/// port: 004d4ee0 Sexy::Bilaterus::vfunction75
/// `Removed()`: the aliens-gone check.
pub fn vfunction75(g: &mut G, this: Ptr) {
    let board = board_of(g, this);
    crate::game::alien::FUN_00539c90(g, board);
}

/// port: 004f07f0 Sexy::Bilaterus::vfunction76
/// `Remove()`: dies with treasure and effects.
pub fn vfunction76__004f07f0(g: &mut G, this: Ptr) {
    FUN_004dbfb0(g, this, true);
}

/// port: 004fae20 Sexy::Bilaterus::vfunction81
/// `Sync(DataSync&)`: its fields, then whether each head exists and its data, then the bone
/// count and each bone's data. Reading builds the parts anew at the body widget's position
/// and adds them to the app's widget manager.
pub fn vfunction81__004fae20(g: &mut G, this: Ptr, sync: &mut crate::sexy::data_sync::DataSync) -> crate::sexy::data_sync::SyncResult {
    use crate::sexy::data_sync::{FUN_00500340, FUN_005003a0, FUN_005004e0, FUN_00500540, FUN_00503010, FUN_00503070};
    crate::game::game_object::vfunction81(g, this, sync)?;
    let mut d = g.bilaterus(this).clone();
    FUN_00503070(sync, &mut d.offset_0x10)?;
    FUN_00503010(sync, &mut d.offset_0x14)?;
    FUN_00503010(sync, &mut d.offset_0x18)?;
    FUN_00503010(sync, &mut d.offset_0x1c)?;
    {
        let b = g.bilaterus(this);
        b.offset_0x10 = d.offset_0x10;
        b.offset_0x14 = d.offset_0x14;
        b.offset_0x18 = d.offset_0x18;
        b.offset_0x1c = d.offset_0x1c;
    }
    if let crate::sexy::data_sync::DataIo::Write(w) = &mut sync.io {
        FUN_00500540(w, d.offset_0x4 != NULL);
        if d.offset_0x4 != NULL {
            vcall!(g, d.offset_0x4, go.vfunction81, sync)?;
        }
        let crate::sexy::data_sync::DataIo::Write(w) = &mut sync.io else { unreachable!() };
        FUN_00500540(w, d.offset_0x8 != NULL);
        if d.offset_0x8 != NULL {
            vcall!(g, d.offset_0x8, go.vfunction81, sync)?;
        }
        let crate::sexy::data_sync::DataIo::Write(w) = &mut sync.io else { unreachable!() };
        FUN_005004e0(w, d.offset_0xc.len() as i32);
        for i in 0..d.offset_0xc.len() {
            let b = g.bilaterus(this).offset_0xc[i];
            vcall!(g, b, go.vfunction81, sync)?;
        }
        return Ok(());
    }
    g.bilaterus(this).offset_0xc = Vec::new();
    let (x, y) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
    let crate::sexy::data_sync::DataIo::Read(r) = &mut sync.io else { unreachable!() };
    if FUN_005003a0(r)? {
        let h = BilaterusHead(g, this, x, y, true);
        g.bilaterus(this).offset_0x4 = h;
        vcall!(g, h, go.vfunction81, sync)?;
        let mgr = app_manager(g, this);
        vcall!(g, mgr, w.vfunction4, h);
    }
    let crate::sexy::data_sync::DataIo::Read(r) = &mut sync.io else { unreachable!() };
    if FUN_005003a0(r)? {
        let h = BilaterusHead(g, this, x, y, false);
        g.bilaterus(this).offset_0x8 = h;
        vcall!(g, h, go.vfunction81, sync)?;
        let mgr = app_manager(g, this);
        vcall!(g, mgr, w.vfunction4, h);
    }
    let crate::sexy::data_sync::DataIo::Read(r) = &mut sync.io else { unreachable!() };
    let n = FUN_00500340(r)?;
    let mut i = 0;
    while i < n {
        let b = BilaterusBone(g, this, x, y, i);
        FUN_005420e0(&mut g.bilaterus(this).offset_0xc, b);
        vcall!(g, b, go.vfunction81, sync)?;
        let mgr = app_manager(g, this);
        vcall!(g, mgr, w.vfunction4, b);
        i += 1;
    }
    Ok(())
}

/// port: 004dc730 Sexy::BilaterusHead::vfunction81
/// `Sync(DataSync&)`: all its fields but the body pointer.
pub fn vfunction81__004dc730(g: &mut G, this: Ptr, sync: &mut crate::sexy::data_sync::DataSync) -> crate::sexy::data_sync::SyncResult {
    use crate::sexy::data_sync::{FUN_00503010, FUN_00503070, FUN_005030a0};
    crate::game::game_object::vfunction81(g, this, sync)?;
    let mut d = g.bil_head(this).clone();
    FUN_00503070(sync, &mut d.offset_0x8)?;
    FUN_00503070(sync, &mut d.offset_0x9)?;
    FUN_00503070(sync, &mut d.offset_0xa)?;
    FUN_00503010(sync, &mut d.offset_0xc)?;
    for f in [&mut d.offset_0x14, &mut d.offset_0x1c, &mut d.offset_0x24, &mut d.offset_0x2c, &mut d.offset_0x34, &mut d.offset_0x3c, &mut d.offset_0x44, &mut d.offset_0x4c] {
        FUN_005030a0(sync, f)?;
    }
    for f in [
        &mut d.offset_0x54, &mut d.offset_0x58, &mut d.offset_0x5c, &mut d.offset_0x60, &mut d.offset_0x64, &mut d.offset_0x68,
        &mut d.offset_0x6c, &mut d.offset_0x70, &mut d.offset_0x74, &mut d.offset_0x78, &mut d.offset_0x7c, &mut d.offset_0x80,
    ] {
        FUN_00503010(sync, f)?;
    }
    for f in [&mut d.offset_0x84, &mut d.offset_0x8c, &mut d.offset_0x94, &mut d.offset_0x9c, &mut d.offset_0xa4] {
        FUN_005030a0(sync, f)?;
    }
    FUN_00503010(sync, &mut d.offset_0xac)?;
    *g.bil_head(this) = d;
    Ok(())
}

/// port: 004dc380 Sexy::BilaterusBone::vfunction81
/// `Sync(DataSync&)`: all its fields but the body pointer.
pub fn vfunction81__004dc380(g: &mut G, this: Ptr, sync: &mut crate::sexy::data_sync::DataSync) -> crate::sexy::data_sync::SyncResult {
    use crate::sexy::data_sync::{FUN_00503010, FUN_005030a0};
    crate::game::game_object::vfunction81(g, this, sync)?;
    let mut d = g.bil_bone(this).clone();
    FUN_00503010(sync, &mut d.offset_0x8)?;
    for f in [&mut d.offset_0xc, &mut d.offset_0x14, &mut d.offset_0x1c, &mut d.offset_0x24, &mut d.offset_0x2c, &mut d.offset_0x34, &mut d.offset_0x3c, &mut d.offset_0x44] {
        FUN_005030a0(sync, f)?;
    }
    for f in [
        &mut d.offset_0x4c, &mut d.offset_0x50, &mut d.offset_0x54, &mut d.offset_0x58, &mut d.offset_0x5c, &mut d.offset_0x60,
        &mut d.offset_0x64, &mut d.offset_0x68, &mut d.offset_0x6c,
    ] {
        FUN_00503010(sync, f)?;
    }
    for f in [&mut d.offset_0x74, &mut d.offset_0x7c, &mut d.offset_0x84, &mut d.offset_0x8c] {
        FUN_005030a0(sync, f)?;
    }
    FUN_00503010(sync, &mut d.offset_0x94)?;
    *g.bil_bone(this) = d;
    Ok(())
}

/// port: 004edc70 FUN_004edc70
/// The warp hole it arrives through, at the leading head, on top.
pub fn FUN_004edc70(g: &mut G, this: Ptr) {
    let h = g.bilaterus(this).offset_0x4;
    let (x, y) = (g.wc(h).offset_0x2c - 10, g.wc(h).offset_0x30 - 0x46);
    let w = crate::game::warp::Warp__004ecf00(g, x, y);
    let board = board_of(g, this);
    crate::game::board_level::FUN_00542ee0(g, board, w, 1);
    let mgr = g.wc(board).offset_0xc;
    vcall!(g, mgr, w.vfunction4, w);
    vcall!(g, mgr, w.vfunction12, w);
}

/// port: 004da730 FUN_004da730
/// `vector<BilaterusBone*>::at(i)` (the original returns the element's address).
pub fn FUN_004da730(this: &[Ptr], param_1: u32) -> Ptr {
    this[param_1 as usize]
}

/// port: 004fb1c0 FUN_004fb1c0
/// `TurnAround()`: the heads swap roles (and turning directions, stopping); with both
/// heads the bone chain is reversed and renumbered, each head re-targets the chain end it
/// now borders, each bone but the last takes over its new leader's motion, and the
/// turn-around sound plays.
pub fn FUN_004fb1c0(g: &mut G, this: Ptr) {
    let (h1, h2) = {
        let d = g.bilaterus(this);
        let (a, b) = (d.offset_0x8, d.offset_0x4);
        d.offset_0x4 = a;
        d.offset_0x8 = b;
        (a, b)
    };
    if h1 == NULL || h2 == NULL {
        return;
    }
    g.bil_head(h1).offset_0xa = true;
    g.bil_head(h2).offset_0xa = false;
    g.bil_head(h1).offset_0x78 = -g.bil_head(h1).offset_0x78;
    g.bil_head(h2).offset_0x78 = -g.bil_head(h2).offset_0x78;
    g.bil_head(h1).offset_0x24 = 0.0;
    g.bil_head(h2).offset_0x24 = 0.0;
    let mut v = Vec::new();
    let mut n = 0;
    let mut i = 5usize;
    loop {
        let b = g.bilaterus(this).offset_0xc[i];
        g.bil_bone(b).offset_0x8 = n;
        FUN_005420e0(&mut v, b);
        n += 1;
        if 5 < n {
            break;
        }
        i -= 1;
    }
    g.bilaterus(this).offset_0xc.clear();
    g.bilaterus(this).offset_0xc = v;
    FUN_004dca80(g, h1);
    for i in 0..5 {
        let b = g.bilaterus(this).offset_0xc[i];
        FUN_004dc5d0(g, b);
        let d = g.bil_bone(b);
        d.offset_0x64 = -d.offset_0x64;
        d.offset_0x1c = 0.0;
    }
    let h2 = g.bilaterus(this).offset_0x8;
    FUN_004dca80(g, h2);
    sound(g, this, 0x131);
}

/// port: 004dca80 FUN_004dca80
/// A head borders the chain end (bone 5): heads off at speed 1.5 the way it faces, takes
/// the last bone's motion and position, and faces away from it (marking the turn-around).
pub fn FUN_004dca80(g: &mut G, this: Ptr) {
    let v = if !g.bil_head(this).offset_0x8 { 1.5 } else { -1.5 };
    g.bil_head(this).offset_0x24 = v;
    let body = g.bil_head(this).offset_0x4;
    let b5 = g.bilaterus(body).offset_0xc[5];
    let bd = g.bil_bone(b5).clone();
    let d = g.bil_head(this);
    d.offset_0x9c = bd.offset_0x1c;
    d.offset_0xa4 = bd.offset_0x24;
    d.offset_0x8c = bd.offset_0xc;
    d.offset_0x94 = bd.offset_0x14;
    if d.offset_0x14 - bd.offset_0xc < 0.0 {
        d.offset_0x8 = true;
        d.offset_0x9 = true;
        return;
    }
    d.offset_0x8 = false;
    d.offset_0x9 = true;
}

/// port: 004dc5d0 FUN_004dc5d0
/// A bone takes the position and motion of the link ahead (the leading head for bone 0).
pub fn FUN_004dc5d0(g: &mut G, this: Ptr) {
    let i = g.bil_bone(this).offset_0x8;
    let body = g.bil_bone(this).offset_0x4;
    let (x, y, vx, vy) = if i == 0 {
        let h = g.bilaterus(body).offset_0x4;
        let hd = g.bil_head(h);
        (hd.offset_0x14, hd.offset_0x1c, hd.offset_0x24, hd.offset_0x2c)
    } else {
        let p = g.bilaterus(body).offset_0xc[(i - 1) as usize];
        let pd = g.bil_bone(p);
        (pd.offset_0xc, pd.offset_0x14, pd.offset_0x1c, pd.offset_0x24)
    };
    let d = g.bil_bone(this);
    d.offset_0x84 = vx;
    d.offset_0x8c = vy;
    d.offset_0x74 = x;
    d.offset_0x7c = y;
}

/// port: 004fe250 Sexy::Bilaterus::vfunction23
/// `Update()` (not while paused). In the virtual tank after 72 seconds a laser blast on
/// the heads, the zap sound, and it leaves. Otherwise, once its start delay is over: a
/// turn-around every 1000 updates (unless dying), the leading head, the bones, the trailing
/// head; at no health the leading head dies.
pub fn vfunction23(g: &mut G, this: Ptr) {
    let app = g.go(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    if board != NULL && g.board(board).field_0x8 {
        return;
    }
    if g.wfa(app).offset_0x150 == 5 && 0x10e0 < g.wc(this).offset_0x24 {
        let h1 = g.bilaterus(this).offset_0x4;
        if h1 != NULL {
            let h2 = g.bilaterus(this).offset_0x8;
            let c = |g: &mut G, p: Ptr| {
                let wc = g.wc(p);
                (wc.offset_0x34 / 2 - 0x28 + wc.offset_0x2c, wc.offset_0x38 / 2 - 0x28 + wc.offset_0x30)
            };
            let (x, y) = if h2 == NULL {
                c(g, h1)
            } else {
                let (x1, y1) = c(g, h1);
                let board = board_of(g, this);
                crate::game::board_level::FUN_00543640(g, board, x1, y1);
                let (x2, y2) = c(g, h2);
                let board = board_of(g, this);
                crate::game::board_level::FUN_00543640(g, board, x2, y2);
                let (w1, hh1, px1, py1) = { let wc = g.wc(h1); (wc.offset_0x34, wc.offset_0x38, wc.offset_0x2c, wc.offset_0x30) };
                let (w2, hh2, px2, py2) = { let wc = g.wc(h2); (wc.offset_0x34, wc.offset_0x38, wc.offset_0x2c, wc.offset_0x30) };
                ((w2 / 2 + w1 / 2 + px2 + px1) / 2 - 0x28, (hh2 / 2 + hh1 / 2 + py2 + py1) / 2 - 0x28)
            };
            let board = board_of(g, this);
            crate::game::board_level::FUN_00543640(g, board, x, y);
        }
        sound(g, this, 0x136);
        crate::game::alien::FUN_004d6830(g, this, true);
        return;
    }
    crate::game::game_object::FUN_004f22c0(g, this);
    let delay = g.bilaterus(this).offset_0x18;
    if delay != 0 {
        g.bilaterus(this).offset_0x18 = delay - 1;
        return;
    }
    g.bilaterus(this).offset_0x1c += 1;
    if !g.bilaterus(this).offset_0x10 && 999 < g.bilaterus(this).offset_0x1c {
        FUN_004fb1c0(g, this);
        g.bilaterus(this).offset_0x1c = 0;
    }
    let h1 = g.bilaterus(this).offset_0x4;
    if h1 != NULL {
        FUN_004f0ea0(g, h1);
    }
    for i in 0..6 {
        let b = g.bilaterus(this).offset_0xc[i];
        FUN_004edea0(g, b);
    }
    let h2 = g.bilaterus(this).offset_0x8;
    if h2 != NULL {
        FUN_004f0ea0(g, h2);
    }
    let h1 = g.bilaterus(this).offset_0x4;
    if h1 != NULL && g.bil_head(h1).offset_0x84 <= 0.0 {
        FUN_004fb4c0(g, h1, true);
    }
}

/// port: 004f0ea0 FUN_004f0ea0
/// `BilaterusHead::Update()` (not while paused). The leading head hunts, or wanders by its
/// mode (re-rolled now and then); the trailing head drifts toward the last bone's motion
/// (sampled every 12 updates) and stays within 40 / 20 of it. Then the turning animation,
/// the tank bounds (turning back at the sides), moving, the timers, and the widget (the
/// leading head moves the body widget along).
pub fn FUN_004f0ea0(g: &mut G, this: Ptr) {
    if paused(g, this) {
        return;
    }
    'move_: {
        if !g.bil_head(this).offset_0xa {
            let body = g.bil_head(this).offset_0x4;
            if body == NULL {
                return;
            }
            g.bil_head(this).offset_0xac += 1;
            if 0xb < g.bil_head(this).offset_0xac {
                g.bil_head(this).offset_0xac = 0;
                let b5 = FUN_004da730(&g.bilaterus(body).offset_0xc, 5);
                let (vx, vy) = (g.bil_bone(b5).offset_0x1c, g.bil_bone(b5).offset_0x24);
                let d = g.bil_head(this);
                d.offset_0x9c = vx;
                d.offset_0xa4 = vy;
            }
            let b5 = FUN_004da730(&g.bilaterus(body).offset_0xc, 5);
            let (bx, by) = (g.bil_bone(b5).offset_0xc, g.bil_bone(b5).offset_0x14);
            let d = g.bil_head(this);
            d.offset_0x8c = bx;
            d.offset_0x94 = by;
            if d.offset_0x24 < d.offset_0x9c {
                d.offset_0x24 += 0.02;
            }
            if d.offset_0x9c < d.offset_0x24 {
                d.offset_0x24 -= 0.02;
            }
            if d.offset_0x2c < d.offset_0xa4 {
                d.offset_0x2c += 0.02;
            }
            if d.offset_0xa4 < d.offset_0x2c {
                d.offset_0x2c -= 0.02;
            }
            if d.offset_0x14 < d.offset_0x8c - 40.0 {
                d.offset_0x14 = d.offset_0x8c - 40.0;
            }
            if d.offset_0x8c + 40.0 < d.offset_0x14 {
                d.offset_0x14 = d.offset_0x8c + 40.0;
            }
            if d.offset_0x94 + 20.0 < d.offset_0x1c {
                d.offset_0x1c = d.offset_0x94 + 20.0;
            }
            if d.offset_0x1c < d.offset_0x94 - 20.0 {
                d.offset_0x1c = d.offset_0x94 - 20.0;
            }
            break 'move_;
        }
        if FUN_004f0800(g, this) {
            break 'move_;
        }
        {
            let d = g.bil_head(this);
            match d.offset_0x64 {
                0 => d.offset_0x34 = -1.5,
                1 => d.offset_0x34 = 1.5,
                2 => d.offset_0x3c = -1.5,
                3 => d.offset_0x3c = 1.5,
                _ => {}
            }
            if d.offset_0x34 < d.offset_0x24 {
                d.offset_0x24 -= 0.1;
            }
            if d.offset_0x24 < d.offset_0x34 {
                d.offset_0x24 += 0.1;
            }
            if d.offset_0x3c < d.offset_0x2c {
                d.offset_0x2c -= 0.1;
            }
            if d.offset_0x2c < d.offset_0x3c {
                d.offset_0x2c += 0.1;
            }
            d.offset_0x68 += 1;
            d.offset_0x6c += 1;
        }
        if 0x14 < g.bil_head(this).offset_0x6c {
            g.bil_head(this).offset_0x6c = 0;
            if rand(g, this) % 5 == 0 {
                let m = (rand(g, this) & 3) as i32;
                g.bil_head(this).offset_0x64 = m;
            }
        }
    }
    FUN_004d4fd0(g, this);
    let (x, y) = {
        let d = g.bil_head(this);
        if d.offset_0x14 < d.offset_0x58 as f64 {
            d.offset_0x14 = d.offset_0x58 as f64;
            d.offset_0x34 = 1.5;
            d.offset_0x24 = 0.0;
        }
        if (d.offset_0x60 as f64) < d.offset_0x14 {
            d.offset_0x14 = d.offset_0x60 as f64;
            d.offset_0x34 = -1.5;
            d.offset_0x24 = 0.0;
        }
        if (d.offset_0x54 as f64) < d.offset_0x1c {
            d.offset_0x1c = d.offset_0x54 as f64;
            d.offset_0x2c = 0.0;
        }
        if d.offset_0x1c < d.offset_0x5c as f64 {
            d.offset_0x1c = d.offset_0x5c as f64;
            d.offset_0x2c = 0.0;
        }
        d.offset_0x14 = d.offset_0x24 / d.offset_0x44 + d.offset_0x14;
        d.offset_0x1c = d.offset_0x2c / d.offset_0x44 + d.offset_0x1c;
        if 0 < d.offset_0x7c {
            d.offset_0x7c -= 1;
        }
        if 0 < d.offset_0x80 {
            d.offset_0x80 -= 1;
        }
        (d.offset_0x14, d.offset_0x1c)
    };
    vcall!(g, this, w.vfunction42, ftol(x) as i32, ftol(y) as i32);
    if g.bil_head(this).offset_0xa {
        let body = g.bil_head(this).offset_0x4;
        let (x, y) = (g.bil_head(this).offset_0x14, g.bil_head(this).offset_0x1c);
        vcall!(g, body, w.vfunction42, ftol(x) as i32, ftol(y) as i32);
    }
    if g.bil_head(this).offset_0x9 {
        g.bil_head(this).offset_0x9 = false;
    }
}

/// port: 004f0800 FUN_004f0800
/// `Hunt()`: when hunting is allowed and there is prey (or the boss is up), chases the
/// nearest; true when it did.
pub fn FUN_004f0800(g: &mut G, this: Ptr) -> bool {
    if crate::game::alien::FUN_00503580(g) {
        let board = board_of(g, this);
        if crate::game::board_update::FUN_005392a0(g, board) || g.board(board).field_0x84 != NULL {
            if FUN_004e9470(g, this) != NULL {
                FUN_004ee3d0(g, this);
                return true;
            }
        }
    }
    false
}

/// port: 004e9470 FUN_004e9470
/// `FindTarget()`: the nearest huntable creature (null when hunting is not allowed).
pub fn FUN_004e9470(g: &mut G, this: Ptr) -> Ptr {
    let (cx, cy) = crate::game::alien::center(g, this);
    let mut best = 100000000;
    let mut target = NULL;
    if !crate::game::alien::FUN_00503580(g) {
        return NULL;
    }
    let board = board_of(g, this);
    let objs: Vec<Ptr> = g.board(board).offset_0x7c.iter().copied().collect();
    for o in objs {
        if crate::game::alien::huntable(g, this, o) {
            let (ox, oy) = crate::game::alien::center(g, o);
            let (dx, dy) = (cx - ox, cy - oy);
            let d = dy * dy + dx * dx;
            if d < best {
                target = o;
                best = d;
            }
        }
    }
    target
}

/// port: 004ee3d0 FUN_004ee3d0
/// `Chase()`: steers (by 0.1, up to 1.8) toward the target's corner, then bites once its
/// cooldown is over. True when there is a target.
pub fn FUN_004ee3d0(g: &mut G, this: Ptr) -> bool {
    let t = FUN_004e9470(g, this);
    if t != NULL {
        let (tx, ty) = (g.wc(t).offset_0x2c, g.wc(t).offset_0x30);
        let d = g.bil_head(this);
        let a = d.offset_0x14 + 40.0;
        let b = (tx + 0x28) as f64;
        if a <= b {
            if a < b && d.offset_0x24 < 1.8 {
                d.offset_0x24 += 0.1;
            }
        } else if -1.8 < d.offset_0x24 {
            d.offset_0x24 -= 0.1;
        }
        let a = d.offset_0x1c + 40.0;
        let b = (ty + 0x28) as f64;
        if a <= b {
            if a < b && d.offset_0x2c < 1.8 {
                d.offset_0x2c += 0.1;
            }
        } else if -1.8 < d.offset_0x2c {
            d.offset_0x2c -= 0.1;
        }
    }
    if g.bil_head(this).offset_0x80 < 1 && t != NULL {
        FUN_004e95f0(g, this);
    }
    t != NULL
}

/// port: 004e95f0 FUN_004e95f0
/// `TryEat()` (a head or a bone): eats the first huntable creature whose centre is within
/// 30 of its own (70 for the big ones of kind 6).
pub fn FUN_004e95f0(g: &mut G, this: Ptr) {
    if !crate::game::alien::FUN_00503580(g) {
        return;
    }
    let (cx, cy) = crate::game::alien::center(g, this);
    let board = board_of(g, this);
    let objs: Vec<Ptr> = g.board(board).offset_0x7c.iter().copied().collect();
    for o in objs {
        if !crate::game::alien::huntable(g, this, o) {
            continue;
        }
        let r = if g.go(o).offset_0x4 == 6 { 0x46 } else { 0x1e };
        let (ox, oy) = crate::game::alien::center(g, o);
        let (dx, dy) = (cx - ox, cy - oy);
        if -r < dx && dx < r && -r < dy && dy < r {
            vcall!(g, this, go.vfunction78, o as i32);
            vcall!(g, o, go.vfunction76);
            return;
        }
    }
}

/// port: 004d4fd0 FUN_004d4fd0
/// A head's animation: reversing direction starts a 20-update turn (not right after a
/// turn-around); otherwise the swim cels (the first leader 0..9, the other ping-ponging
/// 9..0..9); the facing follows the x speed.
pub fn FUN_004d4fd0(g: &mut G, this: Ptr) {
    let d = g.bil_head(this);
    if d.offset_0x4c <= 0.0 || 0.0 <= d.offset_0x24 || d.offset_0x78 != 0 || d.offset_0x9 {
        if d.offset_0x4c < 0.0 && 0.0 < d.offset_0x24 && d.offset_0x78 == 0 && !d.offset_0x9 {
            d.offset_0x78 = -0x14;
        }
    } else {
        d.offset_0x78 = 0x14;
    }
    if 0 < d.offset_0x78 {
        d.offset_0x78 -= 1;
    } else if d.offset_0x78 < 0 {
        d.offset_0x78 += 1;
    }
    let t = d.offset_0x78;
    if t == 0 {
        d.offset_0x70 += 1;
        if 0x13 < d.offset_0x70 {
            d.offset_0x70 = 0;
        }
        let c = d.offset_0x70;
        if d.offset_0xc == 0 {
            d.offset_0x74 = c / 2;
        } else {
            d.offset_0x74 = c - 10;
            if c - 10 < 0 {
                d.offset_0x74 = c - 9;
            }
            d.offset_0x74 = d.offset_0x74.wrapping_abs();
        }
    } else if 0 < t {
        d.offset_0x74 = 9 - t / 2;
    } else {
        d.offset_0x74 = t / 2 + 9;
    }
    if d.offset_0x24 != d.offset_0x4c && d.offset_0x24 != 0.0 && d.offset_0x4c != 0.0 {
        d.offset_0x4c = d.offset_0x24;
    }
}

/// port: 004d4f90 FUN_004d4f90
/// A bone's facing follows its x speed.
pub fn FUN_004d4f90(g: &mut G, this: Ptr) {
    let d = g.bil_bone(this);
    if d.offset_0x1c != d.offset_0x44 && d.offset_0x1c != 0.0 && d.offset_0x44 != 0.0 {
        d.offset_0x44 = d.offset_0x1c;
    }
}

/// port: 004edea0 FUN_004edea0
/// `BilaterusBone::Update()` (not while paused, with a body and a leading head): drifts
/// toward the link ahead's motion (sampled every 12 updates) and stays within 40 (30 behind
/// bone 0) / 20 of it and inside the tank; moves; the timers; bites once its cooldown is
/// over.
pub fn FUN_004edea0(g: &mut G, this: Ptr) {
    if paused(g, this) {
        return;
    }
    let body = g.bil_bone(this).offset_0x4;
    if body == NULL || g.bilaterus(body).offset_0x4 == NULL {
        return;
    }
    let i = g.bil_bone(this).offset_0x8;
    g.bil_bone(this).offset_0x94 += 1;
    if 0xb < g.bil_bone(this).offset_0x94 {
        g.bil_bone(this).offset_0x94 = 0;
        let (vx, vy) = if i == 0 {
            let h = g.bilaterus(body).offset_0x4;
            (g.bil_head(h).offset_0x24, g.bil_head(h).offset_0x2c)
        } else {
            let p = FUN_004da730(&g.bilaterus(body).offset_0xc, (i - 1) as u32);
            (g.bil_bone(p).offset_0x1c, g.bil_bone(p).offset_0x24)
        };
        let d = g.bil_bone(this);
        d.offset_0x84 = vx;
        d.offset_0x8c = vy;
    }
    let (px, py) = if i == 0 {
        let h = g.bilaterus(body).offset_0x4;
        (g.bil_head(h).offset_0x14, g.bil_head(h).offset_0x1c)
    } else {
        let p = FUN_004da730(&g.bilaterus(body).offset_0xc, (i - 1) as u32);
        (g.bil_bone(p).offset_0xc, g.bil_bone(p).offset_0x14)
    };
    {
        let d = g.bil_bone(this);
        d.offset_0x74 = px;
        d.offset_0x7c = py;
        if d.offset_0x1c < d.offset_0x84 {
            d.offset_0x1c += 0.02;
        }
        if d.offset_0x84 < d.offset_0x1c {
            d.offset_0x1c -= 0.02;
        }
        if d.offset_0x24 < d.offset_0x8c {
            d.offset_0x24 += 0.02;
        }
        if d.offset_0x8c < d.offset_0x24 {
            d.offset_0x24 -= 0.02;
        }
    }
    FUN_004d4f90(g, this);
    let (x, y) = {
        let d = g.bil_bone(this);
        let gap = if i == 0 { 40.0 } else { 30.0 };
        if d.offset_0xc < d.offset_0x74 - gap {
            d.offset_0xc = d.offset_0x74 - gap;
        }
        if gap + d.offset_0x74 < d.offset_0xc {
            d.offset_0xc = gap + d.offset_0x74;
        }
        if d.offset_0x7c + 20.0 < d.offset_0x14 {
            d.offset_0x14 = d.offset_0x7c + 20.0;
        }
        if d.offset_0x14 < d.offset_0x7c - 20.0 {
            d.offset_0x14 = d.offset_0x7c - 20.0;
        }
        if d.offset_0xc < d.offset_0x50 as f64 {
            d.offset_0xc = d.offset_0x50 as f64;
        }
        if (d.offset_0x58 as f64) < d.offset_0xc {
            d.offset_0xc = d.offset_0x58 as f64;
        }
        if (d.offset_0x4c as f64) < d.offset_0x14 {
            d.offset_0x14 = d.offset_0x4c as f64;
        }
        if d.offset_0x14 < d.offset_0x54 as f64 {
            d.offset_0x14 = d.offset_0x54 as f64;
        }
        d.offset_0xc = d.offset_0x1c / d.offset_0x3c + d.offset_0xc;
        d.offset_0x14 = d.offset_0x24 / d.offset_0x3c + d.offset_0x14;
        if 0 < d.offset_0x68 {
            d.offset_0x68 -= 1;
        }
        if 0 < d.offset_0x6c {
            d.offset_0x6c -= 1;
        }
        (d.offset_0xc, d.offset_0x14)
    };
    vcall!(g, this, w.vfunction42, ftol(x) as i32, ftol(y) as i32);
    if g.bil_bone(this).offset_0x6c < 1 {
        FUN_004e95f0(g, this);
    }
}

/// port: 004fb4c0 FUN_004fb4c0
/// A head's death. The leading head of a whole worm flies off as a missile (kind 4, or 5
/// for the second-made head), the worm turns around so the other head leads, the dead head
/// leaves its manager and the app, the worm is marked dying (and moves at once), the
/// shadow goes, and the death cries play. Once dying, the body dies instead.
pub fn FUN_004fb4c0(g: &mut G, this: Ptr, param_1: bool) {
    let body = g.bil_head(this).offset_0x4;
    if g.bilaterus(body).offset_0x10 {
        FUN_004dbfb0(g, body, param_1);
        return;
    }
    let kind = if g.bil_head(this).offset_0xc == 0 { 4 } else { 5 };
    let (x, y) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
    let board = board_of(g, this);
    crate::game::board_level::FUN_00544130(g, board, x, y, NULL, kind);
    FUN_004fb1c0(g, body);
    let mgr = g.wc(this).offset_0xc;
    vcall!(g, mgr, w.vfunction5, this);
    let app = g.go(this).offset_0x0;
    crate::sexy::sexy_app_base::vfunction35(g, app, this);
    let body = g.bil_head(this).offset_0x4;
    g.bilaterus(body).offset_0x10 = true;
    g.bilaterus(body).offset_0x8 = NULL;
    let shadow = g.go(this).offset_0xc;
    if shadow != NULL {
        crate::game::shadow::vfunction76(g, shadow);
    }
    sound(g, this, 0x11d);
    sound(g, this, 0x11e);
}

/// port: 004dbfb0 FUN_004dbfb0
/// `Die(bool effects)`: drops what it carries; with effects (and no boss up) the treasure
/// where it was (a diamond when the level allows), the bones and the remaining head fly
/// off as missiles; leaves the board's manager, the app and the board, with its shadow;
/// the aliens-gone check; with effects shells and bubbles and the death cries.
pub fn FUN_004dbfb0(g: &mut G, this: Ptr, param_1: bool) {
    let carried = g.go(this).offset_0x10;
    if carried != NULL {
        crate::game::missle::vfunction76(g, carried);
    }
    let board = board_of(g, this);
    'effects: {
        if g.board(board).field_0x84 == NULL {
            if !param_1 {
                break 'effects;
            }
            if crate::game::alien::FUN_004d70b0(g, this) {
                let (x, y) = (g.wc(this).offset_0x2c + 0x19, g.wc(this).offset_0x30 + 0x19);
                crate::game::board_level::FUN_00544430(g, board, x, y, 4, NULL, -1.0, 0);
            }
        }
        if param_1 {
            for i in 0..6 {
                let b = g.bilaterus(this).offset_0xc[i];
                let (x, y) = (g.wc(b).offset_0x2c, g.wc(b).offset_0x30);
                let board = board_of(g, this);
                crate::game::board_level::FUN_00544130(g, board, x, y, NULL, 3);
            }
            let h = g.bilaterus(this).offset_0x4;
            let kind = if g.bil_head(h).offset_0xc == 0 { 4 } else { 5 };
            let (x, y) = (g.wc(h).offset_0x2c, g.wc(h).offset_0x30);
            let board = board_of(g, this);
            crate::game::board_level::FUN_00544130(g, board, x, y, NULL, kind);
        }
    }
    let board = board_of(g, this);
    let mgr = g.wc(board).offset_0xc;
    vcall!(g, mgr, w.vfunction5, this);
    let app = g.go(this).offset_0x0;
    crate::sexy::sexy_app_base::vfunction35(g, app, this);
    let board = board_of(g, this);
    crate::game::board_level::FUN_00541d90(g, board, this, false);
    let shadow = g.go(this).offset_0xc;
    if shadow != NULL {
        crate::game::shadow::vfunction76(g, shadow);
    }
    let board = board_of(g, this);
    crate::game::alien::FUN_00539c90(g, board);
    if param_1 {
        let mut n = (rand(g, this) & 1) + 2;
        while n != 0 {
            let y = (rand(g, this) % 0x28) as i32 + 10 + g.wc(this).offset_0x30;
            let x = (rand(g, this) % 0x28) as i32 + 10 + g.wc(this).offset_0x2c;
            let board = board_of(g, this);
            crate::game::board_update::FUN_00538ac0(g, board, x, y);
            let y = (rand(g, this) % 0x14) as i32 + 0x14 + g.wc(this).offset_0x30;
            let x = (rand(g, this) % 0x14) as i32 + 0x14 + g.wc(this).offset_0x2c;
            let board = board_of(g, this);
            crate::game::board_update::FUN_00538ac0(g, board, x, y);
            n -= 1;
        }
        let mut n = rand(g, this) % 3 + 4;
        while n != 0 {
            let k = (rand(g, this) % 3) as i32 + 3;
            let y = (rand(g, this) % 0x28) as i32 + 10 + g.wc(this).offset_0x30;
            let x = (rand(g, this) % 0x28) as i32 + 10 + g.wc(this).offset_0x2c;
            let board = board_of(g, this);
            crate::game::board_level::FUN_005436f0(g, board, x, y, k);
            let k = (rand(g, this) % 3) as i32 + 3;
            let y = (rand(g, this) % 0x14) as i32 + 0x14 + g.wc(this).offset_0x30;
            let x = (rand(g, this) % 0x14) as i32 + 0x14 + g.wc(this).offset_0x2c;
            let board = board_of(g, this);
            crate::game::board_level::FUN_005436f0(g, board, x, y, k);
            n -= 1;
        }
        sound(g, this, 0x11d);
        sound(g, this, 0x11e);
    }
}

/// port: 004fdff0 Sexy::BilaterusHead::vfunction76
/// `Remove()`: dies.
pub fn vfunction76__004fdff0(g: &mut G, this: Ptr) {
    FUN_004fb4c0(g, this, true);
}

/// port: 004d5120 Sexy::BilaterusBone::vfunction78
/// `Ate(GameObject* prey)` (a head or a bone): remembers the time (virtual tank), the chomp,
/// and with +0x882 bubbles where the prey was.
pub fn vfunction78(g: &mut G, this: Ptr, param_1: i32) {
    let prey = param_1 as Ptr;
    let app = g.go(this).offset_0x0;
    g.globals.DAT_005e8f18 = g.sab(app).field_0x47c;
    let board = g.wfa(app).offset_0x4;
    crate::game::board_level::FUN_00538560(g, board, false);
    if g.wfa(app).offset_0x156 {
        let mut n = rand(g, this) % 3 + 2;
        while n != 0 {
            let off = if g.go(prey).offset_0x4 == 6 { 0x37 } else { 0xf };
            let y = (rand(g, this) % 0x14) as i32 + off + g.wc(prey).offset_0x30;
            let x = (rand(g, this) % 0x14) as i32 + off + g.wc(prey).offset_0x2c;
            let board = g.wfa(app).offset_0x4;
            crate::game::board_level::FUN_005436f0(g, board, x, y, 1);
            n -= 1;
        }
    }
}

/// Which way a part's sprite faces (`vfunction27` of both parts): left when moving left,
/// or when barely moving (speed truncating to 0) while facing left; right when moving right
/// or barely moving while facing right; neither (no draw) when still and facing nowhere.
fn facing(vx: f64, dir: f64) -> Option<bool> {
    let slow = ftol(vx) == 0;
    if vx < 0.0 || (slow && dir < 0.0) {
        Some(false)
    } else if 0.0 < vx {
        Some(true)
    } else if slow && 0.0 < dir {
        Some(true)
    } else {
        None
    }
}

/// port: 004e4220 Sexy::BilaterusBone::vfunction27
/// `Draw(Graphics*)`.
pub fn vfunction27__004e4220(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    crate::game::food::FUN_004d6ae0(g, this);
    let d = g.bil_bone(this).clone();
    if let Some(right) = facing(d.offset_0x1c, d.offset_0x44) {
        FUN_004dc4e0(g, this, gfx, right);
    }
}

/// port: 004dc4e0 FUN_004dc4e0
/// Draws a bone (once the worm moves): the cel by how far it sits from the link ahead
/// (side-on at 30..36 left of it, edge-on beyond), the row by its index's parity. The
/// facing argument is not used.
pub fn FUN_004dc4e0(g: &mut G, this: Ptr, gfx: &mut Graphics, _param_2: bool) {
    let body = g.bil_bone(this).offset_0x4;
    if g.bilaterus(body).offset_0x18 != 0 {
        return;
    }
    let i = g.bil_bone(this).offset_0x8;
    let parity = i % 2;
    let ahead = if i == 0 {
        let h = g.bilaterus(body).offset_0x4;
        g.bil_head(h).offset_0x14
    } else {
        let p = g.bilaterus(body).offset_0xc[(i - 1) as usize];
        g.bil_bone(p).offset_0xc
    };
    let d = g.bil_bone(this).offset_0xc - ahead;
    let cel = if 36.0 <= d || d <= -30.0 { 0 } else { 5 - ftol(d / 6.0) as i32 };
    let img = g.res.DAT_005e8f00;
    FUN_004d4f00(g, this, gfx, img, cel * -0x50, (-6 - parity) * 0x50);
}

/// port: 004d4f00 FUN_004d4f00
/// Draws an image at (x, y) with the bone's hit flash (+0x1bc) on top.
pub fn FUN_004d4f00(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: Ptr, param_3: i32, param_4: i32) {
    FUN_00455d20(gfx, g, param_2, param_3, param_4);
    let flash = g.bil_bone(this).offset_0x68;
    if 0 < flash {
        FUN_004558c0(gfx, 1);
        FUN_004558e0(gfx, true);
        FUN_00455890(gfx, CRect(0xff, 0xff, 0xff, flash * 0x19));
        FUN_00455d20(gfx, g, param_2, param_3, param_4);
        FUN_004558c0(gfx, 0);
        FUN_004558e0(gfx, false);
    }
}

/// port: 004e50b0 Sexy::BilaterusHead::vfunction27
/// `Draw(Graphics*)`: the head, and over the leading head the health bar while the level
/// shows alien health (+0x178) outside the virtual tank.
pub fn vfunction27__004e50b0(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    crate::game::food::FUN_004d6ae0(g, this);
    let d = g.bil_head(this).clone();
    if let Some(right) = facing(d.offset_0x24, d.offset_0x4c) {
        FUN_004e42c0(g, this, gfx, right);
    }
    if !g.bil_head(this).offset_0xa {
        return;
    }
    let board = board_of(g, this);
    if g.board(board).field_0xb8[13] == 0 || g.board(board).field_0x33c == 5 {
        return;
    }
    let bar = g.res.DAT_005e8e3c;
    let bw = g.image(bar).offset_0x20;
    let y = g.wc(this).offset_0x38 - 3;
    let hp = g.bil_head(this).offset_0x84;
    let w = ftol(bw as f64 * hp / 100.0) as i32;
    let mut w = if w < 0 { 0 } else if bw < w { bw } else { w };
    if g.bil_head(this).offset_0xc == 4 {
        w = bw - w;
    }
    let bh = g.image(bar).offset_0x24;
    FUN_00455e40(gfx, g, bar, -0x14, y, &Rect::new(0, 0, w, bh));
    let frame = g.res.DAT_005e8eb8;
    FUN_00455d20(gfx, g, frame, -0x14, y);
}

/// port: 004e42c0 FUN_004e42c0
/// Draws a head. The leading head: the swim cel, or while turning the turning cel facing
/// the turn's way. The trailing head: by how far it sits from the last bone (side-on at
/// 30..36 behind it, facing it; edge-on beyond, the way it faced). The first leader uses
/// rows 3..5, the other rows 0..2.
pub fn FUN_004e42c0(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: bool) {
    let (r0, r1, r2) = if g.bil_head(this).offset_0xc == 1 { (0, 1, 2) } else { (3, 4, 5) };
    let img = g.res.DAT_005e8f00;
    let d = g.bil_head(this).clone();
    if d.offset_0xa {
        if d.offset_0x78 != 0 {
            let src = Rect::new(d.offset_0x74 * 0x50, r1 * 0x50, 0x50, 0x50);
            FUN_004dc8f0(g, this, gfx, img, &src, 0 < d.offset_0x78);
        } else {
            let src = Rect::new(d.offset_0x74 * 0x50, r2 * 0x50, 0x50, 0x50);
            FUN_004dc8f0(g, this, gfx, img, &src, param_2);
        }
        return;
    }
    if d.offset_0x4 == NULL {
        return;
    }
    let b5 = g.bilaterus(d.offset_0x4).offset_0xc[5];
    let gap = d.offset_0x14 - g.bil_bone(b5).offset_0xc;
    if gap < 36.0 && -30.0 < gap {
        let src = Rect::new((5 - ftol(gap / 6.0) as i32) * 0x50, r0 * 0x50, 0x50, 0x50);
        FUN_004dc8f0(g, this, gfx, img, &src, false);
        g.bil_head(this).offset_0x8 = !(0.0 <= gap);
        return;
    }
    let src = Rect::new(0x140, r2 * 0x50, 0x50, 0x50);
    FUN_004dc8f0(g, this, gfx, img, &src, !d.offset_0x8);
}

/// port: 004dc8f0 FUN_004dc8f0
/// Draws a head cel with its hit flash; while the worm waits to start the part grows in
/// (the leading head from 80% of the shrink).
pub fn FUN_004dc8f0(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: Ptr, param_3: &Rect, param_4: bool) {
    let body = g.bil_head(this).offset_0x4;
    let delay = if body == NULL { 0 } else { g.bilaterus(body).offset_0x18 };
    if delay == 0 {
        FUN_004560a0(gfx, g, param_2, 0, 0, param_3, param_4);
        let flash = g.bil_head(this).offset_0x7c;
        if 0 < flash {
            FUN_004558c0(gfx, 1);
            FUN_004558e0(gfx, true);
            FUN_00455890(gfx, CRect(0xff, 0xff, 0xff, flash * 0x19));
            FUN_004560a0(gfx, g, param_2, 0, 0, param_3, param_4);
            FUN_004558c0(gfx, 0);
            FUN_004558e0(gfx, false);
        }
        return;
    }
    if 7 <= delay {
        return;
    }
    let mut s = (delay as f64 / 7.0) as f32;
    if s <= 0.0 {
        return;
    }
    if g.bil_head(this).offset_0xa {
        let k = (f64::from_bits(0x3fe99999a0000000) * s as f64) as f64;
        s = (k * s as f64) as f32;
    }
    let (w, h) = (param_3.mWidth, param_3.mHeight);
    let a = ftol(w as f64 * s as f64) as i32;
    let b = ftol(s as f64 * h as f64) as i32;
    let dest = Rect::new(a / 2, b / 2, w - a, h - b);
    let app = g.go(this).offset_0x0;
    let fast = !crate::sexy::sexy_app_base::dtor_MemoryImage__00489a20(g, app);
    FUN_00455900(gfx, fast);
    FUN_004561c0(gfx, param_2, &dest, param_3, param_4);
}

/// port: 004ffae0 FUN_004ffae0
/// A laser shot at (x, y) on the worm: when it hits the leading head, bubbles at the
/// shot; true on a hit.
pub fn FUN_004ffae0(g: &mut G, this: Ptr, param_1: i32, param_2: i32) -> bool {
    let h = g.bilaterus(this).offset_0x4;
    if h == NULL {
        return false;
    }
    if !FUN_004fe480(g, h, param_1, param_2) {
        return false;
    }
    let board = board_of(g, this);
    crate::game::board_level::FUN_005436f0(g, board, param_1 - 0x28, param_2 - 0x28, 2);
    true
}

/// port: 004fe480 FUN_004fe480
/// A laser shot at (x, y) on a head (not while it flashes): knocked away from where it was
/// hit (by 3.5 diagonally from the corners, 4 straight from the edges' middles, not at all
/// from the centre, all scaled by its facing), hurt by twice the weapon level plus 2, the
/// hit sound and flash; at no health it dies. True on a hit.
pub fn FUN_004fe480(g: &mut G, this: Ptr, param_1: i32, param_2: i32) -> bool {
    let (x, y) = (param_1 as f64, param_2 as f64);
    let d = g.bil_head(this).clone();
    let (hx, hy, f) = (d.offset_0x14, d.offset_0x1c, d.offset_0x44);
    if !(hx < x && x < hx + 80.0 && hy < y && y < hy + 80.0) || 0 < d.offset_0x7c {
        return false;
    }
    {
        let d = g.bil_head(this);
        if x < hx + 30.0 {
            if y < hy + 30.0 {
                d.offset_0x2c = f * 3.5;
                d.offset_0x24 = f * 3.5;
            } else if y < hy + 50.0 {
                d.offset_0x24 = f * 4.0;
            } else {
                d.offset_0x24 = f * 3.5;
                d.offset_0x2c = f * -3.5;
            }
        } else if x < hx + 50.0 && y < hy + 30.0 {
            d.offset_0x2c = f * 4.0;
        } else if hx + 50.0 < x && hy + 50.0 < y {
            d.offset_0x24 = f * -3.5;
            d.offset_0x2c = f * -3.5;
        } else if hx + 50.0 < x && y < hy + 30.0 {
            d.offset_0x24 = f * -3.5;
            d.offset_0x2c = f * 3.5;
        } else if hx + 50.0 < x {
            d.offset_0x24 = f * -4.0;
        } else if hy + 50.0 < y {
            d.offset_0x2c = f * -4.0;
        }
    }
    let board = board_of(g, this);
    let lvl = g.board(board).field_0x358;
    g.bil_head(this).offset_0x84 -= (lvl * 2 + 2) as f64;
    sound(g, this, 0x126);
    g.bil_head(this).offset_0x7c = 10;
    if g.bil_head(this).offset_0x84 <= 0.0 {
        FUN_004fb4c0(g, this, true);
    }
    true
}
