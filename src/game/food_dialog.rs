//! `Sexy::FoodDialog`: "FOOD SELECTOR" (dialog 0x20), the virtual tank's feeder strip: a
//! button for each kind of creature food (guppy, star, larva, Oscar, Ultra, pizza, ice
//! cream, chicken) that the tank has eaters for; a click drops one in if fewer are out
//! than there are eaters. Plus the board and app functions that open it or drop the food.
//!
//! The object is 0x220 bytes: MoneyDialog (0x158), then `FoodDialog_data` (hover, press,
//! eight 0x18-byte buttons).

use crate::game::money_dialog::MoneyDialog_data;
use crate::sexy::dialog::{alloc_dialog, DlgSub, MoneySub};
use crate::sexy::graphics::{FUN_00455880, FUN_00455890, FUN_00455920, FUN_004559b0, FUN_00455cf0, FUN_00455d20, FUN_00455e40, FUN_00456980};
use crate::sexy::prelude::*;
use crate::sexy::types::FUN_00433320;

/// One food button (0x18 bytes).
#[derive(Debug, Clone, Copy, Default)]
pub struct FoodButton {
    /// +0x0 the rect.
    pub rect: Rect,
    /// +0x10 the food index.
    pub index: i32,
    /// +0x14 shown (the tank has eaters of it).
    pub enabled: bool,
}

/// `FoodDialog_data` (object offset 0x158).
#[derive(Debug, Clone, Default)]
pub struct FoodDialog_data {
    /// +0x158 the button under the mouse (-1 none).
    pub offset_0x0: i32,
    /// +0x15c the button pressed (-1 none).
    pub offset_0x4: i32,
    /// +0x160..+0x220 the buttons.
    pub offset_0x8: [FoodButton; 8],
}

impl G {
    pub fn food_dialog(&mut self, p: Ptr) -> &mut FoodDialog_data {
        match &mut self.widget(p).ext {
            WExt::Dialog(e) => match &mut e.sub {
                DlgSub::Money(_, MoneySub::Food(d)) => d,
                s => panic!("{p} is not a FoodDialog: {s:?}"),
            },
            e => panic!("{p} is not a Dialog: {e:?}"),
        }
    }
}

/// port: 00533d80 FUN_00533d80
/// `FoodDialog(WinFishApp*)`: modeless, CLOSE, a 50-pixel strip at y 65 across the screen;
/// one 50x50 button per food the tank has eaters for (all of them with no board), centred
/// on x 300, 56 pixels apart.
pub fn FUN_00533d80(g: &mut G, param_1: Ptr) -> Ptr {
    let this = alloc_dialog(
        g,
        &crate::sexy::vtables_gen::Sexy__FoodDialog_vftable,
        DlgSub::Money(MoneyDialog_data::default(), MoneySub::Food(FoodDialog_data::default())),
    );
    let (comp, btn) = (g.res.DAT_005e8cc0, g.res.DAT_005e8c54);
    crate::game::money_dialog::MoneyDialog(g, this, param_1, comp, btn, 0x20, false, b"FOOD SELECTOR", b"", b"CLOSE", 3);
    {
        let d = g.dialog(this);
        d.ext_0x11c = 0x1e;
        d.ext_0x124 = 0x1e;
        d.ext_0x128 = 0xf;
    }
    vfunction41(g, this, 0, 0x41, 0x280, 0x32);
    let d = g.food_dialog(this);
    d.offset_0x0 = -1;
    d.offset_0x4 = -1;
    let board = g.wfa(param_1).offset_0x4;
    let mut need = [1; 8];
    if board != NULL {
        need = crate::game::virtual_tank::FUN_0053a3c0(g, board);
        let _ = crate::game::virtual_tank::FUN_005397a0(g, board);
    }
    let mut n = 0;
    for i in 0..8 {
        let on = board == NULL || 0 < need[i];
        g.food_dialog(this).offset_0x8[i].enabled = on;
        if on {
            n += 1;
        }
    }
    let mut x = 300 - (n * 0x38) / 2;
    let d = g.food_dialog(this);
    for i in 0..8 {
        let b = &mut d.offset_0x8[i];
        b.rect = Rect::new(x, 2, 0x32, 0x32);
        b.index = i as i32;
        if b.enabled {
            x += 0x38;
        }
    }
    this
}

/// port: 00532620 Sexy::FoodDialog::~FoodDialog
pub fn dtor_FoodDialog(g: &mut G, this: Ptr) {
    crate::game::money_dialog::dtor_MoneyDialog(g, this);
}

/// port: 00532da0 Sexy::FoodDialog::deleting_destructor
pub fn deleting_destructor(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_FoodDialog(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 00530310 Sexy::FoodDialog::vfunction41
/// `Resize(...)`: CLOSE at the right end of the strip.
pub fn vfunction41(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32, param_4: i32) {
    crate::sexy::dialog::vfunction41_for_Widget(g, this, param_1, param_2, param_3, param_4);
    let yes = g.dialog(this).offset_0x8;
    let h = g.wc(yes).offset_0x38;
    vcall!(g, yes, w.vfunction41, 0x212, param_2 + 10, 100, h);
}

/// port: 00532640 FUN_00532640
/// The shown button at a point, or -1.
pub fn FUN_00532640(g: &mut G, this: Ptr, param_1: i32, param_2: i32) -> i32 {
    let d = g.food_dialog(this);
    for (i, b) in d.offset_0x8.iter().enumerate() {
        let r = b.rect;
        if b.enabled && r.mX <= param_1 && param_1 < r.mWidth + r.mX && r.mY <= param_2 && param_2 < r.mHeight + r.mY {
            return i as i32;
        }
    }
    -1
}

/// port: 00532780 Sexy::FoodDialog::vfunction53
/// `MouseMove(x, y)`: lights the button under the mouse.
pub fn vfunction53(g: &mut G, this: Ptr, param_1: i32, param_2: i32) {
    crate::sexy::trivial::vfunction1__0046eb50();
    let i = FUN_00532640(g, this, param_1, param_2);
    if i != g.food_dialog(this).offset_0x0 {
        g.food_dialog(this).offset_0x0 = i;
        vcall!(g, this, w.vfunction18);
    }
}

/// port: 00532690 Sexy::FoodDialog::vfunction55
/// `MouseDown(x, y, clicks)`: presses the button under the mouse.
pub fn vfunction55(g: &mut G, this: Ptr, param_1: i32, param_2: i32, _clicks: i32) {
    let i = FUN_00532640(g, this, param_1, param_2);
    let d = g.food_dialog(this);
    d.offset_0x4 = i;
    d.offset_0x0 = i;
    vcall!(g, this, w.vfunction18);
}

/// port: 005326c0 Sexy::FoodDialog::vfunction57
/// `MouseUp(x, y, clicks)`: released over the pressed button with a board up, drops that
/// food if there are more eaters than pieces out, else buzzes.
pub fn vfunction57(g: &mut G, this: Ptr, param_1: i32, param_2: i32, _clicks: i32) {
    crate::sexy::trivial::vfunction1__0046eb50();
    let i = FUN_00532640(g, this, param_1, param_2);
    let pressed = g.food_dialog(this).offset_0x4;
    g.food_dialog(this).offset_0x0 = i;
    let app = g.money(this).offset_0x50;
    if i == pressed && pressed != -1 && g.wfa(app).offset_0x4 != NULL {
        let board = g.wfa(app).offset_0x4;
        let need = crate::game::virtual_tank::FUN_0053a3c0(g, board);
        let have = crate::game::virtual_tank::FUN_005397a0(g, board);
        let k = g.food_dialog(this).offset_0x4 as usize;
        if have[k] < need[k] {
            let board = g.wfa(app).offset_0x4;
            FUN_00547530(g, board, k as i32);
        } else {
            let s = g.res.DAT_005e8c40;
            crate::sexy::sexy_app_base::vfunction55(g, app, s);
        }
    }
    g.food_dialog(this).offset_0x4 = -1;
    vcall!(g, this, w.vfunction18);
}

/// port: 00534030 Sexy::FoodDialog::vfunction27
/// `Draw(Graphics*)`: dimmed and framed, "SELECT" / "FOOD" at the left, and each shown
/// button (lit when hovered, pressed in when pressed) with its food's picture and the
/// shine over it.
pub fn vfunction27(g: &mut G, this: Ptr, param_1: &mut Graphics) {
    let (w, h) = (g.wc(this).offset_0x34, g.wc(this).offset_0x38);
    FUN_00455890(param_1, FUN_00433320(0xb0000000));
    FUN_00455920(param_1, 0, 0, w, h);
    FUN_00455890(param_1, FUN_00433320(0xffffff));
    FUN_004559b0(param_1, 0, 0, w - 1, h - 1);
    let f = g.res.DAT_005e8a5c;
    FUN_00455890(param_1, FUN_00433320(0xffff00));
    FUN_00455880(param_1, f);
    let a = crate::sexy::image_font::get_ascent(g, f) + 5;
    FUN_00455cf0(param_1, g, b"SELECT", 10, a);
    let fh = crate::sexy::image_font::get_height(g, f);
    FUN_00455cf0(param_1, g, b"FOOD", 10, fh + a - 5);
    let d = g.food_dialog(this).clone();
    for i in 0..8usize {
        let b = d.offset_0x8[i];
        if !b.enabled {
            continue;
        }
        let (x0, y0) = (b.rect.mX, b.rect.mY);
        let (mut x, mut y) = (x0, y0);
        let mut id = 0x3c;
        if i as i32 == d.offset_0x4 {
            id = 0x3b;
            y = y0 + 1;
            x = x0 + 1;
        } else if i as i32 == d.offset_0x0 {
            id = 0x3d;
        }
        let bg = crate::sexy::res::FUN_005016a0(g, id) as Ptr;
        FUN_00455d20(param_1, g, bg, x0, y0);
        let r = &g.res;
        let (ab8, e60, b80, dc8, b64, d0c, e0c) =
            (r.DAT_005e8ab8, r.DAT_005e8e60, r.DAT_005e8b80, r.DAT_005e8dc8, r.DAT_005e8b64, r.DAT_005e8d0c, r.DAT_005e8e0c);
        match i {
            0 => FUN_00456980(param_1, g, ab8 as Ptr, x - 10, y - 0x11, 0, 0),
            1 => FUN_00456980(param_1, g, e60 as Ptr, x - 8, y - 0xc, 0, 2),
            2 => FUN_00456980(param_1, g, e60 as Ptr, x - 8, y - 0xf, 0, 5),
            3 => FUN_00456980(param_1, g, b80 as Ptr, x + 8, y + 3, 0, 0),
            4 => FUN_00456980(param_1, g, dc8 as Ptr, x + 8, y + 3, 0, 0),
            5 => FUN_00455d20(param_1, g, b64 as Ptr, x + 6, y),
            6 => FUN_00455d20(param_1, g, d0c as Ptr, x + 8, y + 2),
            _ => FUN_00455d20(param_1, g, e0c as Ptr, x + 0xd, y),
        }
        let shine = g.res.DAT_005e8e2c as Ptr;
        let sw = crate::sexy::graphics::FUN_00457480(g, shine);
        FUN_00455e40(param_1, g, shine, x0, y0, &Rect::new(0, 0, sw, 0x28));
    }
}

/// port: 0054c780 FUN_0054c780
/// `ShowFoodDialog()`: closes the dialogs and opens dialog 0x20.
pub fn FUN_0054c780(g: &mut G, this: Ptr) {
    crate::game::win_fish_app::FUN_0054b4c0(g, this);
    let d = FUN_00533d80(g, this);
    crate::sexy::sexy_app_base::vfunction76(g, this, 0x20, d);
}

/// port: 005492b0 FUN_005492b0
/// The virtual tank's Feed button: when some food has more eaters than pieces out, opens
/// the food selector if the tank has eaters of more than one food, else drops that food.
pub fn FUN_005492b0(g: &mut G, this: Ptr) {
    let need = crate::game::virtual_tank::FUN_0053a3c0(g, this);
    let have = crate::game::virtual_tank::FUN_005397a0(g, this);
    let mut short = 0;
    let mut kinds = 0;
    for i in 0..8 {
        if 0 < need[i] {
            kinds += 1;
        }
        if need[i] != have[i] && -1 < need[i] - have[i] {
            short += 1;
        }
    }
    if short == 0 {
        return;
    }
    if 1 < kinds {
        let app = g.board(this).field_0x0;
        FUN_0054c780(g, app);
        return;
    }
    for i in 0..8 {
        if need[i] != have[i] && -1 < need[i] - have[i] {
            FUN_00547530(g, this, i as i32);
        }
    }
}

/// port: 00547530 FUN_00547530
/// `DropFood(int kind)`: a guppy, star, larva, Oscar or Ultra (0..4) or a chicken, pizza
/// or ice cream (5..7), not to be eaten for 40 updates (+0x120).
pub fn FUN_00547530(g: &mut G, this: Ptr, param_1: i32) {
    let o = match param_1 {
        0 => FUN_00547490(g, this),
        1 => FUN_00544630(g, this),
        2 => FUN_00544720(g, this),
        3 => FUN_005474c0(g, this),
        4 => FUN_005474f0(g, this),
        5 => FUN_00543570(g, this, 5),
        6 => FUN_00543570(g, this, 3),
        7 => FUN_00543570(g, this, 4),
        _ => return,
    };
    if o != NULL {
        g.go(o).offset_0x98 = 0x28;
    }
}

/// The app's Mersenne Twister (`app+0x7b0`), as the board reaches it through +0x8c.
fn app_rand(g: &mut G, this: Ptr) -> u32 {
    let app = g.board(this).field_0x0;
    let r = g.wfa(app).offset_0x84;
    crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(r))
}

/// The shared tail: on the board's lists, in the widget manager, re-sorted.
fn add(g: &mut G, this: Ptr, o: Ptr, kind: i32) {
    crate::game::board_level::FUN_00542ee0(g, this, o, kind);
    let mgr = g.wc(this).offset_0xc;
    vcall!(g, mgr, w.vfunction4, o);
    crate::game::board_level::FUN_0053aa40(g, this);
}

/// port: 00543570 FUN_00543570
/// A virtual-tank food of `kind` (3 pizza, 4 ice cream, 5 chicken) at a random x
/// (50..499), y 115.
pub fn FUN_00543570(g: &mut G, this: Ptr, param_1: i32) -> Ptr {
    crate::game::board::FUN_00538230(g, this, 0x122, 3, 1.0);
    let x = app_rand(g, this) % 0x1c2 + 0x32;
    let f = crate::game::food::Food__004ef840(g, x as i32, 0x73, 0, false, param_1);
    add(g, this, f, 1);
    f
}

/// port: 00544630 FUN_00544630
/// A star (coin kind 3) for the starcatchers at a random spot (x 20..539, y 105..154),
/// not clickable.
pub fn FUN_00544630(g: &mut G, this: Ptr) -> Ptr {
    crate::game::board::FUN_00538230(g, this, 0x122, 3, 1.0);
    let x = app_rand(g, this) % 0x208 + 0x14;
    let y = crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32 % 0x32 + 0x69;
    let c = crate::game::coin::Coin__004eead0(g, x as i32, y, 3, NULL, -1.0);
    g.w(c).offset_0x1 = false;
    add(g, this, c, 0);
    c
}

/// port: 00544720 FUN_00544720
/// A larva for the beetlemunchers at a random spot (x 20..539, y 311..360), never
/// clickable.
pub fn FUN_00544720(g: &mut G, this: Ptr) -> Ptr {
    crate::game::board::FUN_00538230(g, this, 0x122, 3, 1.0);
    let x = app_rand(g, this) % 0x208 + 0x14;
    let y = 0x168 - crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32 % 0x32;
    let o = crate::game::larva::Larva__004ead40(g, x as i32, y);
    g.larva(o).offset_0x21 = true;
    add(g, this, o, 0);
    o
}

/// port: 00547490 FUN_00547490
/// A guppy for the carnivores.
pub fn FUN_00547490(g: &mut G, this: Ptr) -> Ptr {
    crate::game::board::FUN_00538230(g, this, 0x122, 3, 1.0);
    crate::game::board_level::FUN_00538360(g, this);
    crate::game::board_level::FUN_00546d70(g, this)
}

/// port: 005474c0 FUN_005474c0
/// An Oscar for the ultravores.
pub fn FUN_005474c0(g: &mut G, this: Ptr) -> Ptr {
    crate::game::board::FUN_00538230(g, this, 0x122, 3, 1.0);
    crate::game::board_level::FUN_00538360(g, this);
    crate::game::board_level::FUN_00544c00(g, this)
}

/// port: 005474f0 FUN_005474f0
/// An Ultra for the Sylvesters.
pub fn FUN_005474f0(g: &mut G, this: Ptr) -> Ptr {
    crate::game::board::FUN_00538230(g, this, 0x122, 3, 1.0);
    crate::game::board::FUN_00538230(g, this, 0x140, 3, 1.0);
    crate::game::board_level::FUN_00544e60(g, this)
}
