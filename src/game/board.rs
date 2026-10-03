//! `Sexy::Board`: the tank. Board_data starts at object offset 0x8c (a second vftable for
//! `ButtonListener` sits at 0x88). Fields are named per tools/off.py.

use crate::sexy::prelude::*;

/// `Board_data` (object offset 0x8c) plus the bytes past the database's 0x43c-byte type.
/// The original never initializes +0x3f0 and +0x460 in the constructor (set when a game
/// starts); here they start at 0.
#[derive(Debug, Clone)]
pub struct Board_data {
    /// +0x8c: the `WinFishApp*`.
    pub field_0x0: Ptr,
    /// +0x90: the board's timed-message list (`FUN_00515c40`).
    pub offset_0x4: Ptr,
    /// +0x94: paused (game objects skip their update).
    pub field_0x8: bool,
    /// +0x98..+0xf8: 25 `std::vector<GameObject*>` (one per kind of object), in field order
    /// +0x98, +0x9c, ... +0xf8.
    pub offset_0xc: [Vec<Ptr>; 25],
    /// +0xfc: the overlay widget for layer 0.
    pub offset_0x70: Ptr,
    /// +0x100: the overlay widget for layer 1.
    pub offset_0x74: Ptr,
    /// +0x104 a `std::set` of objects (head +0x108, size +0x10c). The original orders it by
    /// heap address; here by object id (also allocation order).
    pub offset_0x7c: std::collections::BTreeSet<Ptr>,
    /// +0x110: a widget of the board (removed with the objects).
    pub field_0x84: Ptr,
    /// +0x114: the Menu button (id 0x7b).
    pub offset_0x88: Ptr,
    /// +0x118: the message line.
    pub offset_0x8c: Ptr,
    /// +0x11c: the bubble manager.
    pub offset_0x90: Ptr,
    /// +0x120..+0x13c: cheat codes "wavy", "prego", "void", "space", "zombie",
    /// "welovebetatesters", "supermegaultra", "time".
    pub offset_0x94: [Option<Box<crate::game::cheat_code::CheatCode>>; 8],
    /// +0x140: the star field.
    pub offset_0xb4: Ptr,
    /// +0x144..+0x1a0.
    pub field_0xb8: [i32; 24],
    /// +0x1a4: per-sound tick of the last play (`int[0x40]`, sounds 0x106..0x145), -1000 initially.
    pub field_0x118: [i32; 0x40],
    /// +0x2a4: the level is over (won).
    pub field_0x218: bool,
    /// +0x2a5.
    pub field_0x219: bool,
    /// +0x2a6.
    pub field_0x21a: bool,
    /// +0x2a7.
    pub field_0x21b: bool,
    /// +0x2a8: board update count when the level was won.
    pub field_0x21c: i32,
    /// +0x2ac.
    pub field_0x220: i32,
    /// +0x2b0.
    pub field_0x224: i32,
    /// +0x2b4.
    pub field_0x228: i32,
    /// +0x2b8.
    pub field_0x22c: bool,
    /// +0x2bc: alien schedule (which aliens this level brings).
    pub field_0x230: i32,
    /// +0x2c0: updates until the next alien.
    pub field_0x234: i32,
    /// +0x2c4.
    pub field_0x238: i32,
    /// +0x2c8.
    pub field_0x23c: i32,
    /// +0x2cc..+0x2d8: two random spots (x, y) for the next aliens.
    pub field_0x240: i32,
    pub field_0x244: i32,
    pub field_0x248: i32,
    pub field_0x24c: i32,
    /// +0x2dc: how many shop slots hold an item.
    pub field_0x250: i32,
    /// +0x2e0: how many shop buttons exist.
    pub field_0x254: i32,
    /// +0x2e4: the item in each of the 12 shop slots (-1 = empty; 0 guppy, 1 food quality,
    /// 2 food quantity, 3 carnivore, 4 star potion, 5 weapon, 6 egg piece, ...).
    pub field_0x258: [i32; 12],
    /// +0x314: each slot's price.
    pub field_0x288: [i32; 12],
    /// +0x344: each slot's current price.
    pub field_0x2b8: [i32; 12],
    /// +0x374: each slot's price step (price / 100 when unset).
    pub field_0x2e8: [i32; 12],
    /// +0x3a4: each slot's button was created.
    pub field_0x318: [bool; 12],
    /// +0x3b0.
    pub field_0x324: i32,
    /// +0x3b4: game time at start (`FUN_00537b60`).
    pub field_0x328: i32,
    /// +0x3b8: game time at start.
    pub field_0x32c: i32,
    /// +0x3bc.
    pub field_0x330: i32,
    /// +0x3c0.
    pub field_0x334: i32,
    /// +0x3c4.
    pub field_0x338: i32,
    /// +0x3c8.
    pub field_0x33c: i32,
    /// +0x3cc.
    pub field_0x340: i32,
    /// +0x3d0: x of the last laser shot.
    pub field_0x344: i32,
    /// +0x3d4: y of the last laser shot.
    pub field_0x348: i32,
    /// +0x3d8: three x positions (`rand() % 640`).
    pub field_0x34c: [f32; 3],
    /// +0x3e4.
    pub field_0x358: i32,
    /// +0x3e8: the tank backdrop (1..6).
    pub field_0x35c: i32,
    /// +0x3ec.
    pub field_0x360: i32,
    /// +0x3f0: money.
    pub field_0x364: i32,
    /// +0x3f4.
    pub field_0x368: i32,
    /// +0x3f8.
    pub field_0x36c: i32,
    /// +0x3fc.
    pub field_0x370: i32,
    /// +0x400.
    pub field_0x374: i32,
    /// +0x404..+0x41c: seven widgets of the board (removed with the objects).
    pub field_0x378: [Ptr; 7],
    /// +0x420: the virtual tank's Visit Store button (id 10).
    pub field_0x394: Ptr,
    /// +0x424: Add/Remove Fish (id 11).
    pub field_0x398: Ptr,
    /// +0x428: Add/Remove Pets (id 12).
    pub field_0x39c: Ptr,
    /// +0x42c: Tank Options (id 13).
    pub field_0x3a0: Ptr,
    /// +0x430: Special Food (id 14).
    pub field_0x3a4: Ptr,
    /// +0x434: Back to Main Menu (id 15).
    pub field_0x3a8: Ptr,
    /// +0x438: the money label.
    pub offset_0x3ac: Ptr,
    /// +0x43c.
    pub ext_0x43c: i32,
    /// +0x440.
    pub ext_0x440: bool,
    /// +0x441.
    pub ext_0x441: bool,
    /// +0x444: board updates (read by `FUN_00537b60` before the constructor sets it).
    pub ext_0x444: i32,
    /// +0x448.
    pub ext_0x448: i32,
    /// +0x44c.
    pub ext_0x44c: i32,
    /// +0x450.
    pub ext_0x450: i32,
    /// +0x454.
    pub ext_0x454: i32,
    /// +0x458.
    pub ext_0x458: i32,
    /// +0x45c.
    pub ext_0x45c: i32,
    /// +0x460..+0x4a8: 19 per-level statistics (+0x460 money earned, +0x48c most guppies).
    pub ext_0x460: [i32; 19],
    /// +0x4ac.
    pub ext_0x4ac: i32,
    /// +0x4b0: 0x36 bytes cleared by the constructor.
    pub ext_0x4b0: [u8; 0x36],
    /// +0x4e6.
    pub ext_0x4e6: bool,
    /// +0x4e8.
    pub ext_0x4e8: i32,
    /// +0x4ec.
    pub ext_0x4ec: bool,
    /// +0x4ed.
    pub ext_0x4ed: bool,
    /// +0x4ee: a game is in progress (saved on exit).
    pub ext_0x4ee: bool,
    /// +0x4f0.
    pub ext_0x4f0: i32,
    /// +0x4f4.
    pub ext_0x4f4: bool,
    /// +0x4f8.
    pub ext_0x4f8: i32,
    /// +0x4fc.
    pub ext_0x4fc: bool,
    /// +0x4fd.
    pub ext_0x4fd: bool,
    /// +0x4fe.
    pub ext_0x4fe: bool,
    /// +0x4ff.
    pub ext_0x4ff: bool,
    /// +0x500.
    pub ext_0x500: bool,
}

impl Default for Board_data {
    fn default() -> Self {
        Board_data {
            field_0x0: NULL,
            offset_0x4: NULL,
            field_0x8: false,
            offset_0xc: Default::default(),
            offset_0x70: NULL,
            offset_0x74: NULL,
            offset_0x7c: Default::default(),
            field_0x84: NULL,
            offset_0x88: NULL,
            offset_0x8c: NULL,
            offset_0x90: NULL,
            offset_0x94: Default::default(),
            offset_0xb4: NULL,
            field_0xb8: [0; 24],
            field_0x118: [-1000; 0x40],
            field_0x218: false,
            field_0x219: false,
            field_0x21a: false,
            field_0x21b: false,
            field_0x21c: 0,
            field_0x220: 0,
            field_0x224: 0,
            field_0x228: 0,
            field_0x22c: false,
            field_0x230: 0,
            field_0x234: 0,
            field_0x238: 0,
            field_0x23c: 0,
            field_0x240: 0,
            field_0x244: 0,
            field_0x248: 0,
            field_0x24c: 0,
            field_0x250: 0,
            field_0x254: 0,
            field_0x258: [0; 12],
            field_0x288: [0; 12],
            field_0x2b8: [0; 12],
            field_0x2e8: [0; 12],
            field_0x318: [false; 12],
            field_0x324: 0,
            field_0x328: 0,
            field_0x32c: 0,
            field_0x330: 0,
            field_0x334: 0,
            field_0x338: 0,
            field_0x33c: 0,
            field_0x340: 0,
            field_0x344: 0,
            field_0x348: 0,
            field_0x34c: [0.0; 3],
            field_0x358: 0,
            field_0x35c: 0,
            field_0x360: 0,
            field_0x364: 0,
            field_0x368: 0,
            field_0x36c: 0,
            field_0x370: 0,
            field_0x374: 0,
            field_0x378: [NULL; 7],
            field_0x394: NULL,
            field_0x398: NULL,
            field_0x39c: NULL,
            field_0x3a0: NULL,
            field_0x3a4: NULL,
            field_0x3a8: NULL,
            offset_0x3ac: NULL,
            ext_0x43c: 0,
            ext_0x440: false,
            ext_0x441: false,
            ext_0x444: 0,
            ext_0x448: 0,
            ext_0x44c: 0,
            ext_0x450: 0,
            ext_0x454: 0,
            ext_0x458: 0,
            ext_0x45c: 0,
            ext_0x460: [0; 19],
            ext_0x4ac: 0,
            ext_0x4b0: [0; 0x36],
            ext_0x4e6: false,
            ext_0x4e8: 0,
            ext_0x4ec: false,
            ext_0x4ed: false,
            ext_0x4ee: false,
            ext_0x4f0: 0,
            ext_0x4f4: false,
            ext_0x4f8: 0,
            ext_0x4fc: false,
            ext_0x4fd: false,
            ext_0x4fe: false,
            ext_0x4ff: false,
            ext_0x500: false,
        }
    }
}

impl G {
    /// `Board_data` of the board.
    pub fn board(&mut self, p: Ptr) -> &mut Board_data {
        match &mut self.widget(p).ext {
            WExt::Board(b) => b,
            e => panic!("{p} is not a Board: {e:?}"),
        }
    }
}

/// port: 005381f0 FUN_005381f0
/// `Board::CanPlaySample(int sound, int minTicks)`: true (and remembers the tick) when the
/// sound has not been started in the last `param_2` updates.
pub fn FUN_005381f0(g: &mut G, this: Ptr, param_1: i32, param_2: i32) -> bool {
    let i = param_1.wrapping_sub(0x106) as u32;
    if i < 0x40 {
        let now = g.wc(this).offset_0x24;
        let last = g.board(this).field_0x118[i as usize];
        if param_2 <= now - last {
            g.board(this).field_0x118[i as usize] = now;
            return true;
        }
    }
    false
}

/// port: 00538230 FUN_00538230
/// `Board::PlaySample(int sound, int minTicks, double volume)`.
pub fn FUN_00538230(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: f64) {
    let i = param_1.wrapping_sub(0x106) as u32;
    if i < 0x40 {
        let now = g.wc(this).offset_0x24;
        if param_2 <= now - g.board(this).field_0x118[i as usize] {
            g.board(this).field_0x118[i as usize] = now;
            let id = crate::sexy::res::FUN_005016a0(g, param_1);
            // mSoundManager->GetSoundInstance(id); SetVolume(param_3); Play(false, true)
            g.sound_requests.push(crate::sexy::sexy_app_base::SoundRequest { id, volume: param_3, pan: 0, pitch: 0.0 });
        }
    }
}

/// port: 005383d0 FUN_005383d0
/// One of the four "points" sounds, at most every 3 updates (sound 300).
pub fn FUN_005383d0(g: &mut G, this: Ptr) {
    if FUN_005381f0(g, this, 300, 3) {
        let app = g.board(this).field_0x0;
        let rng = g.wfa(app).offset_0x84;
        let r = g.mtrand(rng).next() & 3;
        let id = if r != 0 && !g.board(this).field_0x219 {
            match r {
                1 => g.res.DAT_005e8b24,
                2 => g.res.DAT_005e8e94,
                _ => g.res.DAT_005e8b1c,
            }
        } else {
            g.res.DAT_005e8d74
        };
        // app->PlaySample(id) (app vtable +0xd8)
        crate::sexy::sexy_app_base::vfunction55(g, app, id);
    }
}

/// port: 00538060 FUN_00538060
/// Runs the timed-message list (`FUN_00515ec0` on `offset_0x4`).
pub fn FUN_00538060(g: &mut G, this: Ptr) {
    let m = g.board(this).offset_0x4;
    crate::game::timed_messages::FUN_00515ec0(g, m);
}

/// port: 00538490 FUN_00538490
/// Whether message `param_1` is still in the timed-message list.
pub fn FUN_00538490(g: &mut G, this: Ptr, param_1: i32) -> bool {
    let m = g.board(this).offset_0x4;
    crate::game::timed_messages::FUN_00505170(g, m, param_1)
}

/// port: 005384a0 FUN_005384a0
/// Removes message `param_1` from the timed-message list, if there is one.
pub fn FUN_005384a0(g: &mut G, this: Ptr, param_1: i32) {
    let m = g.board(this).offset_0x4;
    if m != NULL {
        crate::game::timed_messages::FUN_005156f0(g, m, param_1);
    }
}

/// port: 0053c1e0 FUN_0053c1e0
/// `Board::AddMoney(int)`: virtual-tank shells (mode 5), the alternate counter when
/// `field_0x219` is set, else money capped at 9,999,999.
pub fn FUN_0053c1e0(g: &mut G, this: Ptr, param_1: i32) {
    let app = g.board(this).field_0x0;
    if g.wfa(app).offset_0x150 == 5 {
        if !crate::game::win_fish_app::thunk_FUN_00479fc0(g, app) {
            let profile = g.wfa(app).offset_0x18c;
            crate::game::profile::FUN_00501200(g, profile, param_1);
        }
        g.board(this).field_0x374 += param_1;
        crate::game::board_level::FUN_0053a360(g, this);
        return;
    }
    if g.board(this).field_0x219 {
        g.board(this).field_0x368 += param_1;
        crate::game::board_level::FUN_0053a360(g, this);
        return;
    }
    let b = g.board(this);
    b.field_0x364 += param_1;
    b.ext_0x460[0] += param_1;
    if 9_999_999 < b.field_0x364 {
        b.field_0x364 = 9_999_999;
    }
    crate::game::board_level::FUN_0053a360(g, this);
}

/// The cheat-code words, in field order (+0x120..+0x13c).
const BOARD_CHEATS: [&[u8]; 8] = [b"wavy", b"prego", b"void", b"space", b"zombie", b"welovebetatesters", b"supermegaultra", b"time"];

/// port: 00542150 Sexy::Board::Board
/// `Board(WinFishApp*)`: the tank's helpers (star field, cheat codes, timed messages, the 25
/// object vectors, two overlays, bubbles in (0, 82, 640, 398)), the Menu button, the money
/// label (green, or pink in the virtual tank) and the message line.
pub fn Board(g: &mut G, param_1: Ptr) -> Ptr {
    let (wc, w) = crate::sexy::widget::Widget();
    let this = g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__Board_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::Board(Box::new(Board_data::default())) }),
    });
    {
        let b = g.board(this);
        b.field_0x0 = param_1;
        b.field_0x21a = true;
        b.field_0x21b = false;
    }
    let sf = crate::game::board_parts::StarField(g);
    g.board(this).offset_0xb4 = sf;
    let profile = g.wfa(param_1).offset_0x18c;
    if g.profile(profile).field_0x84 >> 3 & 1 != 0 {
        crate::game::board_parts::FUN_00511580(g, sf, 1000);
    }
    for (i, word) in BOARD_CHEATS.iter().enumerate() {
        g.board(this).offset_0x94[i] = Some(Box::new(crate::game::cheat_code::FUN_0050ff90(word)));
    }
    let list = crate::game::timed_messages::FUN_00515c40(g);
    g.board(this).offset_0x4 = list;
    {
        let (a, b2, c, d) = (g.res.DAT_005e8ec4, g.res.DAT_005e8a3c, g.res.DAT_005e8a68, g.res.DAT_005e8a7c);
        let m = g.timed_messages(list);
        m.field_0xc = a;
        m.field_0x18 = b2;
        m.field_0x10 = c;
        m.field_0x14 = d;
    }
    let o0 = crate::game::board_parts::BoardOverlay(g, this, 0);
    g.board(this).offset_0x70 = o0;
    let o1 = crate::game::board_parts::BoardOverlay(g, this, 1);
    g.board(this).offset_0x74 = o1;
    let bm = crate::game::board_parts::BubbleMgr(g);
    g.board(this).offset_0x90 = bm;
    crate::game::board_parts::FUN_00500120(g, bm, &Rect::new(0, 0x52, 0x280, 0x18e));
    crate::game::board_parts::FUN_00500180(g, bm, 0, 0);
    g.globals.DAT_005e89ce = (g.profile(profile).field_0x84 >> 4) & 1 != 0;
    {
        let b = g.board(this);
        b.field_0x84 = NULL;
        b.ext_0x4b0 = [0; 0x36];
        b.field_0xb8 = [0; 24];
        b.field_0x35c = 0;
        b.field_0x360 = -1;
    }
    let t = FUN_00537b60(g, this);
    {
        let b = g.board(this);
        b.field_0x328 = t;
        b.field_0x32c = t;
        b.field_0x330 = 0;
        b.field_0x324 = 0;
        b.field_0x334 = 0;
        b.field_0x338 = 0;
        b.field_0x8 = false;
    }
    let rng = g.wfa(param_1).offset_0x84;
    for i in 0..3 {
        let r = crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng)) % 0x280;
        g.board(this).field_0x34c[i] = r as i32 as f32;
    }
    {
        let b = g.board(this);
        b.field_0x378 = [NULL; 7];
        b.offset_0x88 = NULL;
        b.offset_0x8c = NULL;
        b.field_0x398 = NULL;
        b.field_0x394 = NULL;
        b.field_0x3a0 = NULL;
        b.field_0x39c = NULL;
        b.field_0x3a4 = NULL;
        b.field_0x3a8 = NULL;
        b.offset_0x3ac = NULL;
        b.ext_0x4ec = false;
        b.ext_0x4ed = false;
        b.ext_0x4f0 = 0;
        b.field_0x23c = 0;
        b.ext_0x4f4 = false;
        b.ext_0x4f8 = 0;
    }
    let menu = crate::sexy::button_widget::new_button_widget(g, 0x7b, this);
    g.board(this).offset_0x88 = menu;
    g.w(menu).offset_0x28 = true;
    let (blank, over, down) = (g.res.DAT_005e8d80, g.res.DAT_005e8c94, g.res.DAT_005e8a20);
    let bd = g.btn(menu);
    bd.offset_0x28 = blank;
    bd.offset_0x2c = over;
    bd.offset_0x30 = down;
    vcall!(g, menu, w.vfunction41, 0x20d, 3, 0x65, 0x1d);
    let label = crate::game::board_parts::MyLabelWidget(g);
    g.board(this).offset_0x3ac = label;
    g.my_label(label).offset_0x2c = 2;
    g.w(label).offset_0x1 = false;
    g.wc(label).offset_0x2c = 0x217;
    g.wc(label).offset_0x30 = 0x28;
    let lf = g.res.DAT_005e8ce4;
    g.my_label(label).offset_0x30 = lf;
    let fh = crate::sexy::image_font::get_height(g, lf);
    g.wc(label).offset_0x38 = fh;
    g.wc(label).offset_0x34 = 0x50;
    g.my_label(label).offset_0x1c = if g.wfa(param_1).offset_0x150 == 5 { CRect(0xfa, 0x9b, 0x96, 0xff) } else { CRect(0xb4, 0xff, 0x5a, 0xff) };
    let msg = crate::game::board_parts::MessageWidget(g, param_1, b"");
    g.board(this).offset_0x8c = msg;
    let b = g.board(this);
    b.ext_0x4ee = false;
    b.field_0x33c = 0;
    b.field_0x340 = 0;
    b.field_0x219 = false;
    b.ext_0x444 = 0;
    b.ext_0x44c = 0;
    b.ext_0x448 = 0;
    b.field_0x238 = 0;
    b.field_0x368 = 0;
    b.field_0x36c = 0;
    b.field_0x370 = 0;
    b.field_0x220 = -1;
    b.field_0x224 = 0;
    b.field_0x374 = 0;
    b.ext_0x4fc = false;
    b.ext_0x4fd = false;
    b.ext_0x4fe = false;
    b.ext_0x4ff = false;
    b.ext_0x500 = true;
    b.ext_0x441 = false;
    b.field_0x228 = 0;
    b.field_0x118 = [-1000; 0x40];
    b.field_0x22c = false;
    this
}

/// port: 0054a430 Sexy::Board::~Board
/// Deletes the label, Menu button and message line, frees the object vectors (not the
/// objects), the timed-message list, overlays, bubbles, cheat codes, star field and set.
pub fn dtor_Board(g: &mut G, this: Ptr) {
    let b = g.board(this).clone();
    for p in [b.offset_0x3ac, b.offset_0x88, b.offset_0x8c] {
        if p != NULL {
            vcall!(g, p, w.vfunction1, 1);
        }
    }
    {
        let bm = g.board(this);
        for v in bm.offset_0xc.iter_mut() {
            v.clear();
        }
    }
    if b.offset_0x4 != NULL {
        crate::game::timed_messages::FUN_00513c20(g, b.offset_0x4);
        g.free(b.offset_0x4);
    }
    for p in [b.offset_0x70, b.offset_0x74] {
        if p != NULL {
            vcall!(g, p, w.vfunction1, 1);
        }
    }
    if b.offset_0x90 != NULL {
        crate::game::board_parts::deleting_destructor__00505420(g, b.offset_0x90, 1);
    }
    for c in g.board(this).offset_0x94.iter_mut() {
        *c = None;
    }
    if b.offset_0xb4 != NULL {
        crate::game::board_parts::deleting_destructor__00505480(g, b.offset_0xb4, 1);
    }
    let bm = g.board(this);
    bm.offset_0x4 = NULL;
    bm.offset_0x7c.clear();
    crate::sexy::widget::dtor_Widget(g, this);
}

/// port: 0054aa10 Sexy::Board::deleting_destructor_for_Widget
pub fn deleting_destructor_for_Widget(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_Board(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 00537b60 FUN_00537b60
/// `GetGameTime()`: board updates times the app's frame duration (+0x454) in ms.
pub fn FUN_00537b60(g: &mut G, this: Ptr) -> i32 {
    let app = g.board(this).field_0x0;
    g.sab(app).field_0x44c.wrapping_mul(g.board(this).ext_0x444)
}

/// port: 00538040 FUN_00538040
/// Seconds played on this board.
pub fn FUN_00538040(g: &mut G, this: Ptr) -> i32 {
    let t = FUN_00537b60(g, this);
    (t - g.board(this).field_0x32c) / 1000
}

/// port: 00537bf0 FUN_00537bf0
/// Whether there is a game to save (in progress, and not mode 3).
pub fn FUN_00537bf0(g: &mut G, this: Ptr) -> bool {
    let app = g.board(this).field_0x0;
    g.board(this).ext_0x4ee && g.wfa(app).offset_0x150 != 3
}

/// port: 005497a0 FUN_005497a0
/// `SaveGame()` on leaving the board: writes `userdata\<mode><id>.dat` (the host creates the
/// folder) when there is a game in progress, else deletes it.
pub fn FUN_005497a0(g: &mut G, this: Ptr) {
    let app = g.board(this).field_0x0;
    let mode = g.wfa(app).offset_0x150;
    let profile = g.wfa(app).offset_0x18c;
    let id = g.profile(profile).field_0x44;
    let path = crate::game::win_fish_app::FUN_00505880(mode, id);
    if FUN_00537bf0(g, this) {
        crate::game::board_save::FUN_00546990(g, this, &path);
    } else {
        crate::sexy::app_host::FUN_0047fbc0(g, &path);
    }
}

/// port: 00500fb0 FUN_00500fb0
/// `RemoveAndDelete(Widget*)`: off the widget manager, then `SafeDeleteWidget`.
pub fn FUN_00500fb0(g: &mut G, param_1: Ptr) {
    if param_1 != NULL {
        let app = g.globals.DAT_005eb6a4;
        let wm = g.sab(app).offset_0x318;
        vcall!(g, wm, w.vfunction5, param_1);
        crate::sexy::sexy_app_base::vfunction35(g, app, param_1);
    }
}

/// port: 00537fc0 Sexy::Board::vfunction21_for_Widget
/// `AddedToManager(WidgetManager*)`: adds the two overlay layers.
pub fn vfunction21_for_Widget(g: &mut G, this: Ptr, manager: Ptr) {
    crate::sexy::widget_container::vfunction21(g, this, manager);
    let (o0, o1) = (g.board(this).offset_0x70, g.board(this).offset_0x74);
    vcall!(g, manager, w.vfunction4, o0);
    vcall!(g, manager, w.vfunction4, o1);
}

/// port: 0053da90 Sexy::Board::vfunction22_for_Widget
/// `RemovedFromManager(WidgetManager*)`: removes the overlays; deletes the Menu button, money
/// label, message line and the six shop buttons; then every game object.
pub fn vfunction22_for_Widget(g: &mut G, this: Ptr, manager: Ptr) {
    crate::sexy::widget_container::vfunction22(g, this, manager);
    let (o0, o1) = (g.board(this).offset_0x70, g.board(this).offset_0x74);
    vcall!(g, manager, w.vfunction5, o0);
    vcall!(g, manager, w.vfunction5, o1);
    let p = g.board(this).offset_0x88;
    FUN_00500fb0(g, p);
    g.board(this).offset_0x88 = NULL;
    let p = g.board(this).offset_0x3ac;
    FUN_00500fb0(g, p);
    g.board(this).offset_0x3ac = NULL;
    let p = g.board(this).offset_0x8c;
    FUN_00500fb0(g, p);
    g.board(this).offset_0x8c = NULL;
    let p = g.board(this).field_0x394;
    FUN_00500fb0(g, p);
    g.board(this).field_0x394 = NULL;
    let p = g.board(this).field_0x398;
    FUN_00500fb0(g, p);
    g.board(this).field_0x398 = NULL;
    let p = g.board(this).field_0x39c;
    FUN_00500fb0(g, p);
    g.board(this).field_0x39c = NULL;
    let p = g.board(this).field_0x3a0;
    FUN_00500fb0(g, p);
    g.board(this).field_0x3a0 = NULL;
    let p = g.board(this).field_0x3a4;
    FUN_00500fb0(g, p);
    g.board(this).field_0x3a4 = NULL;
    let p = g.board(this).field_0x3a8;
    FUN_00500fb0(g, p);
    g.board(this).field_0x3a8 = NULL;
    FUN_0053c480(g, this, manager);
}

/// The object vectors in the order `FUN_0053c480` empties them (by field offset).
const REMOVE_ORDER: [usize; 25] = [0xa0, 0x98, 0x9c, 0xa8, 0xa4, 0xd4, 0xcc, 0xc4, 0xd0, 0xc0, 0xd8, 0xac, 0xe4, 0xe8, 0xec, 0xf0, 0xdc, 0xe0, 0xb4, 0xb0, 0xc8, 0xb8, 0xbc, 0xf4, 0xf8];

/// port: 0053c480 FUN_0053c480
/// `RemoveAllObjects(WidgetManager*)`: clears the object set and the per-kind counters, takes
/// every object of every vector off the manager and safe-deletes it, deletes the board's
/// extra widgets.
pub fn FUN_0053c480(g: &mut G, this: Ptr, param_1: Ptr) {
    let app = g.board(this).field_0x0;
    {
        let b = g.board(this);
        b.offset_0x7c.clear();
        b.field_0xb8 = [0; 24];
    }
    for off in REMOVE_ORDER {
        let i = (off - 0x98) / 4;
        let objs = g.board(this).offset_0xc[i].clone();
        for o in objs {
            vcall!(g, param_1, w.vfunction5, o);
            crate::sexy::sexy_app_base::vfunction35(g, app, o);
        }
        g.board(this).offset_0xc[i].clear();
    }
    let p = g.board(this).field_0x84;
    FUN_00500fb0(g, p);
    g.board(this).field_0x84 = NULL;
    for k in 0..7 {
        let p = g.board(this).field_0x378[k];
        FUN_00500fb0(g, p);
        g.board(this).field_0x378[k] = NULL;
    }
    let b = g.board(this);
    b.field_0x250 = 0;
    b.field_0x254 = 0;
    b.field_0x318 = [false; 12];
}

/// port: 00538010 Sexy::Board::vfunction57_for_Widget
/// `MouseUp(int x, int y, int theClickCount)`: also ends both mouse-held flags.
pub fn vfunction57_for_Widget(g: &mut G, this: Ptr, x: i32, y: i32, clicks: i32) {
    crate::sexy::widget::vfunction57(g, this, x, y, clicks);
    let b = g.board(this);
    b.ext_0x4ec = false;
    b.ext_0x4ed = false;
}

/// port: 00538070 Sexy::Board::vfunction3_for_ButtonListener
/// `ButtonDepress(int)`: the Menu button opens the options dialog.
pub fn vfunction3_for_ButtonListener(g: &mut G, this: Ptr, id: i32) {
    if id == 0x7b {
        let app = g.board(this).field_0x0;
        crate::game::win_fish_app::FUN_0054c620(g, app, false);
    }
}

/// port: 00540470 Sexy::Board::vfunction44_for_Widget
/// `DrawOverlay(Graphics*, int layer)`: layer 0 and layer 1 drawing.
pub fn vfunction44_for_Widget(g: &mut G, this: Ptr, gfx: &mut Graphics, layer: i32) {
    if layer == 0 {
        crate::game::board_draw::FUN_0053bd90(g, this, gfx);
    } else if layer == 1 {
        crate::game::board_draw::FUN_005400f0(g, this, gfx);
    }
}

/// port: 00549d70 Sexy::Board::vfunction48_for_Widget
/// `KeyChar(char)`. In the debug mode (3): 1..8 add creatures (guppy, Oscar, king guppy,
/// star guppy, breeder, beetle, grubber, Ultra), z..m bring alien waves 2..8 (not while
/// the first alien is kind 4, except c), the letters q..l, ';' and A/S/F/D drop pets
/// 0..0x17, +/- step the backdrop. Otherwise: the eight cheat words (the first one that
/// fires ends it), and b/B lets out a burst of 3..6 bubbles (or, with the bubbles cheat
/// on, shortens its countdown to 40). Space pauses.
pub fn vfunction48_for_Widget(g: &mut G, this: Ptr, param_1: u8) {
    use crate::game::board_level as bl;
    let c = param_1;
    let app = g.board(this).field_0x0;
    if g.wfa(app).offset_0x150 == 3 {
        let aliens = &g.board(this).offset_0xc[bl::vec_index(0xb8)];
        let kind4 = match aliens.first() {
            Some(&a) => g.alien(a).offset_0x98 == 4,
            None => false,
        };
        match c {
            b'1' => {
                bl::FUN_00546d70(g, this);
                return;
            }
            b'2' => {
                bl::FUN_00544c00(g, this);
                return;
            }
            b'3' => {
                bl::FUN_00547310(g, this);
                return;
            }
            b'4' => {
                bl::FUN_00545260(g, this);
                return;
            }
            b'5' => {
                bl::FUN_00545430(g, this);
                return;
            }
            b'6' => {
                bl::FUN_00545080(g, this);
                return;
            }
            b'7' => {
                bl::FUN_005470a0(g, this);
                return;
            }
            b'8' => {
                bl::FUN_00544e60(g, this);
                return;
            }
            b'z' | b'x' | b'c' | b'v' | b'b' | b'n' | b'm' => {
                let wave = match c {
                    b'z' => 2,
                    b'x' => 3,
                    b'c' => 4,
                    b'v' => 5,
                    b'b' => 6,
                    b'n' => 7,
                    _ => 8,
                };
                if kind4 && c != b'c' {
                    return;
                }
                g.board(this).field_0x230 = wave;
                bl::FUN_005475b0(g, this, wave, true);
                return;
            }
            _ => {}
        }
        let pet = match c {
            b'q' => Some(0),
            b'w' => Some(1),
            b'e' => Some(2),
            b'r' => Some(3),
            b't' => Some(4),
            b'y' => Some(5),
            b'u' => Some(6),
            b'i' => Some(7),
            b'o' => Some(8),
            b'p' => Some(9),
            b'a' => Some(10),
            b's' => Some(0xb),
            b'd' => Some(0xc),
            b'f' => Some(0xd),
            b'g' => Some(0xe),
            b'h' => Some(0xf),
            b'j' => Some(0x10),
            b'k' => Some(0x11),
            b'l' => Some(0x12),
            b';' => Some(0x13),
            b'A' => Some(0x14),
            b'S' => Some(0x15),
            b'D' => Some(0x17),
            b'F' => Some(0x16),
            _ => None,
        };
        if let Some(p) = pet {
            bl::FUN_00544a90(g, this, p, -1, -1, false, false);
            return;
        }
        if c == b'+' || c == b'-' {
            let bd = g.board(this).field_0x35c + if c == b'+' { 1 } else { -1 };
            bl::FUN_00538a10(g, this, bd);
            crate::game::board_update::FUN_00539a50(g, this);
        }
    } else {
        for i in 0..8 {
            let hit = crate::game::cheat_code::FUN_00504180(g.board(this).offset_0x94[i].as_mut().unwrap(), c);
            if hit && bl::FUN_005404a0(g, this, i as i32) {
                return;
            }
        }
        if c == b'b' || c == b'B' {
            if g.board(this).ext_0x4fd {
                if 0x3c < g.board(this).ext_0x448 {
                    g.board(this).ext_0x448 = 0x28;
                }
            } else {
                let r = g.wfa(app).offset_0x84;
                let n = (crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(r)) & 3) as i32 + 2;
                let mut k = n;
                while -1 < k {
                    bl::FUN_00538a50(g, this);
                    k -= 1;
                }
            }
        }
    }
    if c == b' ' {
        crate::game::win_fish_app::FUN_0054eca0(g, app);
    }
}

/// port: 00540650 Sexy::Board::vfunction49_for_Widget
/// `KeyDown(KeyCode)`: feeds the eight cheat codes.
pub fn vfunction49_for_Widget(g: &mut G, this: Ptr, key: i32) {
    let app = g.board(this).field_0x0;
    if g.sab(app).field_0x5a4 {
        return;
    }
    for i in 0..8 {
        let hit = crate::game::cheat_code::FUN_005041f0(g.board(this).offset_0x94[i].as_mut().unwrap(), key as u8);
        if hit {
            crate::game::board_level::FUN_005404a0(g, this, i as i32);
        }
    }
}

/// port: 0054a370 Sexy::Board::vfunction2_for_ButtonListener
/// `ButtonPress(int)`: the click sound; shop buttons 0..11 buy (outside the virtual tank);
/// 10..15 in the virtual tank open its screens.
pub fn vfunction2_for_ButtonListener(g: &mut G, this: Ptr, id: i32) {
    let app = g.board(this).field_0x0;
    let s = g.res.DAT_005e8c40;
    crate::sexy::sexy_app_base::vfunction55(g, app, s);
    if g.wfa(app).offset_0x150 != 5 && id < 0xc {
        crate::game::board_level::FUN_00549380(g, this, id as usize);
        return;
    }
    match id {
        10 => crate::game::store::FUN_0054bca0(g, app),
        0xb => crate::game::sim_fish::FUN_0054c060(g, app),
        0xc => crate::game::pets_screen::FUN_0054c2d0(g, app),
        0xd => crate::game::sim_setup::FUN_00551c90(g, app),
        0xe => {
            if !crate::game::win_fish_app::FUN_0054b4b0(g, app) {
                crate::game::food_dialog::FUN_005492b0(g, this);
            }
        }
        0xf => crate::game::win_fish_app::FUN_00552230(g, app),
        _ => {}
    }
}
