//! The virtual tank (mode 5): the board in its free-play mode, where the player buys,
//! keeps and names fish. Board functions specific to it.

use crate::sexy::prelude::*;
use crate::game::board_level::vec_index;

/// port: 0053c3f0 FUN_0053c3f0
/// `StartVirtualTank()`: a fresh tank: no level state, not paused, a game in progress, the
/// shared globals (food, follow mode, breeder spot) reset, the counters cleared, both coin
/// totals at the shell rate times the time played (0 now), the screensaver flag; a random
/// alien portal backdrop, the button states, then the resume check.
pub fn FUN_0053c3f0(g: &mut G, this: Ptr) {
    let b = g.board(this);
    b.field_0x230 = 0;
    b.field_0x8 = false;
    b.ext_0x4ee = true;
    let gl = &mut g.globals;
    gl.DAT_005e89dc = 0;
    gl.DAT_005df58c = 1;
    gl.DAT_005e89cc = false;
    gl.DAT_005e89d0 = 0;
    gl.DAT_005e89c0 = 0;
    let b = g.board(this);
    b.ext_0x440 = false;
    b.ext_0x444 = 0;
    b.ext_0x450 = 0;
    b.ext_0x454 = 0;
    b.ext_0x45c = 0;
    b.ext_0x458 = 0;
    let v = crate::game::board::FUN_00537b60(g, this);
    let b = g.board(this);
    b.field_0x328 = v;
    b.field_0x32c = v;
    // (DL still holds the 1 stored to `DAT_005df58c` above.)
    b.ext_0x4fc = true;
    FUN_005394e0(g, this);
    crate::game::board_update::FUN_0053a4f0(g, this);
    crate::game::board_save::FUN_0053aa00(g, this);
}

/// port: 005394e0 FUN_005394e0
/// A random alien wave kind for the virtual tank (2, 3, 4, 5, 7, 6 or 8), then its spots.
pub fn FUN_005394e0(g: &mut G, this: Ptr) {
    let kinds = [2, 3, 4, 5, 7, 6, 8];
    let r = crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32;
    g.board(this).field_0x230 = kinds[(r % 7) as usize];
    crate::game::board_update::FUN_00539330(g, this);
}

/// port: 00538c10 FUN_00538c10
/// Where the alien portal sits on each backdrop (+0x3e8).
pub fn FUN_00538c10(g: &mut G, this: Ptr) -> (i32, i32) {
    match g.board(this).field_0x35c {
        1 => (0x10b, 0x140),
        2 => (0xff, 0x136),
        3 => (0x104, 0x136),
        4 => (0x10e, 0x159),
        5 => (0x124, 0x140),
        6 => (0xd2, 0x160),
        _ => (0x1e0, 300),
    }
}

/// port: 0053a3c0 FUN_0053a3c0
/// Counts the tank's creatures by kind for the shop (every object's `CountKill` stats).
pub fn FUN_0053a3c0(g: &mut G, this: Ptr) -> [i32; 8] {
    let mut c = [0; 8];
    let objs: Vec<Ptr> = g.board(this).offset_0x7c.iter().copied().collect();
    for o in objs {
        crate::game::game_object::FUN_004d7210(g, o, &mut c);
    }
    c
}

/// port: 005397a0 FUN_005397a0
/// Counts the creature food already out, by the shop's kinds: unbought small guppies,
/// stars (coin kind 3), larvae, unbought Oscars and Ultras, chicken, pizza and ice cream.
pub fn FUN_005397a0(g: &mut G, this: Ptr) -> [i32; 8] {
    let mut c = [0; 8];
    for o in g.board(this).offset_0xc[vec_index(0xa0)].clone() {
        if g.go(o).offset_0x24 < 0 && g.fish(o).offset_0x4c == 0 {
            c[0] += 1;
        }
    }
    for o in g.board(this).offset_0xc[vec_index(0xa4)].clone() {
        if g.go(o).offset_0x24 < 0 {
            c[3] += 1;
        }
    }
    for o in g.board(this).offset_0xc[vec_index(0xd4)].clone() {
        if g.go(o).offset_0x24 < 0 {
            c[4] += 1;
        }
    }
    for o in g.board(this).offset_0xc[vec_index(0xa8)].clone() {
        if g.coin(o).offset_0x40 == 3 {
            c[1] += 1;
        }
    }
    c[2] = g.board(this).offset_0xc[vec_index(0xc8)].len() as i32;
    for o in g.board(this).offset_0xc[vec_index(0xac)].clone() {
        match g.food(o).offset_0x40 {
            3 => c[6] += 1,
            4 => c[7] += 1,
            5 => c[5] += 1,
            _ => {}
        }
    }
    c
}

/// port: 0053a980 FUN_0053a980
/// Whether the tank holds any bought creature (+0x2b8).
pub fn FUN_0053a980(g: &mut G, this: Ptr) {
    g.board(this).field_0x22c = false;
    let objs: Vec<Ptr> = g.board(this).offset_0x7c.iter().copied().collect();
    for o in objs {
        if -1 < g.go(o).offset_0x24 {
            g.board(this).field_0x22c = true;
            return;
        }
    }
}


/// `VirtualDialog_data` (object offset 0x158): the "WELCOME TO VIRTUAL TANK!" dialog.
#[derive(Debug, Clone, Default)]
pub struct VirtualDialog_data {
    /// +0x158 the bubbles behind the text (`BubbleMgr`, owned).
    pub offset_0x0: Ptr,
    /// +0x15c..+0x168 the framed area (left, top, width, height).
    pub offset_0x4: i32,
    pub offset_0x8: i32,
    pub offset_0xc: i32,
    pub offset_0x10: i32,
}

impl G {
    pub fn virtual_dialog(&mut self, p: Ptr) -> &mut VirtualDialog_data {
        use crate::sexy::dialog::{DlgSub, MoneySub};
        match &mut self.widget(p).ext {
            WExt::Dialog(e) => match &mut e.sub {
                DlgSub::Money(_, MoneySub::Virtual(v)) => v,
                s => panic!("{p} is not a VirtualDialog: {s:?}"),
            },
            e => panic!("{p} is not a Dialog: {e:?}"),
        }
    }
}

/// port: 00536c80 FUN_00536c80
/// `VirtualDialog(WinFishApp*)`: dialog 0x27 "WELCOME TO VIRTUAL TANK!" with an OK button,
/// 550x400, a framed area with bubbles (already running for 500 updates).
pub fn FUN_00536c80(g: &mut G, param_1: Ptr) -> Ptr {
    use crate::sexy::dialog::{alloc_dialog, DlgSub, MoneySub};
    let this = alloc_dialog(
        g,
        &crate::sexy::vtables_gen::Sexy__VirtualDialog_vftable,
        DlgSub::Money(crate::game::money_dialog::MoneyDialog_data::default(), MoneySub::Virtual(VirtualDialog_data::default())),
    );
    let (comp, btn) = (g.res.DAT_005e8cc0, g.res.DAT_005e8c54);
    crate::game::money_dialog::MoneyDialog(g, this, param_1, comp, btn, 0x27, true, b"WELCOME TO VIRTUAL TANK!", b"", b"OK", 3);
    {
        let v = g.virtual_dialog(this);
        v.offset_0x4 = 0x1e;
        v.offset_0x8 = 0x55;
        v.offset_0xc = 0x1ea;
    }
    g.wc(this).offset_0x34 = 0x226;
    g.wc(this).offset_0x38 = 400;
    g.virtual_dialog(this).offset_0x10 = 0xdc;
    let bubbles = crate::game::board_parts::BubbleMgr(g);
    let v = g.virtual_dialog(this).clone();
    g.virtual_dialog(this).offset_0x0 = bubbles;
    let area = Rect::new(v.offset_0x4 + 0x14, v.offset_0x8, v.offset_0xc - 0x28, v.offset_0x10 - 10);
    crate::game::board_parts::FUN_00500120(g, bubbles, &area);
    crate::game::board_parts::FUN_00500180(g, bubbles, 10, 3);
    crate::game::board_parts::FUN_00512080(g, bubbles);
    this
}

/// port: 00532280 Sexy::VirtualDialog::~VirtualDialog
pub fn dtor_VirtualDialog(g: &mut G, this: Ptr) {
    let b = g.virtual_dialog(this).offset_0x0;
    if b != NULL {
        crate::game::board_parts::deleting_destructor__00505420(g, b, 1);
    }
    crate::game::money_dialog::dtor_MoneyDialog(g, this);
}

/// port: 00532bc0 Sexy::VirtualDialog::deleting_destructor
pub fn deleting_destructor(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_VirtualDialog(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 005322f0 Sexy::VirtualDialog::vfunction41
/// `Resize(int x, int y, int w, int h)`: the OK button 200 wide, centered.
pub fn vfunction41(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32, param_4: i32) {
    crate::sexy::dialog::vfunction41_for_Widget(g, this, param_1, param_2, param_3, param_4);
    let yes = g.dialog(this).offset_0x8;
    let (mx, mw) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x34);
    let (by, bh) = (g.wc(yes).offset_0x30, g.wc(yes).offset_0x38);
    vcall!(g, yes, w.vfunction41, (mw - 200) / 2 + mx, by, 200, bh);
}

/// port: 00532350 Sexy::VirtualDialog::vfunction23
/// `Update()`: the bubbles move, and it redraws.
pub fn vfunction23(g: &mut G, this: Ptr) {
    let b = g.virtual_dialog(this).offset_0x0;
    crate::game::board_parts::FUN_005110e0(g, b);
    vcall!(g, this, w.vfunction18);
}

/// port: 00536e70 Sexy::VirtualDialog::vfunction27
/// `Draw(Graphics*)`: the dialog, the tagline, the framed area with its bubbles, and three
/// tips beside their icons (the store icon, a shell, the tank icon) with their captions.
pub fn vfunction27(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    use crate::sexy::graphics::{FUN_00455880, FUN_00455890, FUN_00455cf0, FUN_00455d20, FUN_004563f0, FUN_00456950};
    crate::sexy::dialog::vfunction27_for_Widget(g, this, gfx);
    let f = g.res.DAT_005e8a5c;
    FUN_00455880(gfx, f);
    FUN_00455890(gfx, crate::sexy::types::FUN_00433320(0xffff88));
    let w = g.wc(this).offset_0x34;
    vcall!(g, this, w.vfunction65, gfx, Rect::new(0x1e, 0x37, w - 0x3c, 0), b"Customize your very own virtual aquarium!", -1, 0);
    let v = g.virtual_dialog(this).clone();
    let frame = g.res.DAT_005e8c58;
    FUN_004563f0(gfx, g, &Rect::new(v.offset_0x4, v.offset_0x8, v.offset_0xc, v.offset_0x10), frame);
    crate::game::board_parts::FUN_00504b60(g, v.offset_0x0, gfx);
    let f = g.res.DAT_005e8cf0;
    FUN_00455880(gfx, f);
    FUN_00455890(gfx, crate::sexy::types::FUN_00433320(0xffffff));
    let mut r = Rect::new(v.offset_0x4 + 100, v.offset_0x8 + 0x23, v.offset_0xc - 0x78, v.offset_0x10);
    let icons = g.res.DAT_005e8e28;
    FUN_00456950(gfx, g, icons, 0x3c, v.offset_0x8 + 0x17, 2);
    vcall!(g, this, w.vfunction65, gfx, r, b"Buy all sorts of fish in the Store.  Virtual fish never die, but try to take good care of them.", -1, -1);
    r.mY += 0x3c;
    let shell = g.res.DAT_005e8b74;
    FUN_00455d20(gfx, g, shell, 0x48, r.mY);
    vcall!(g, this, w.vfunction65, gfx, r, b"Use shells to buy fish -- you can earn shells in every game mode!", -1, -1);
    let icon_y = r.mY + 0x30;
    r.mY += 0x3c;
    FUN_00456950(gfx, g, icons, 0x3c, icon_y, 5);
    vcall!(g, this, w.vfunction65, gfx, r, b"Make Virtual Tank your screensaver!  Click on the Tank button to set it up.", -1, -1);
    let f = g.res.DAT_005e8c20;
    FUN_00455880(gfx, f);
    FUN_00455890(gfx, crate::sexy::types::FUN_00433320(0xaaffaa));
    FUN_00455cf0(gfx, g, b"STORE", 0x43, r.mY - 0x50);
    FUN_00455cf0(gfx, g, b"SHELLS", 0x40, r.mY - 0x1c);
    FUN_00455cf0(gfx, g, b"TANK", 0x44, r.mY + 0x23);
}
