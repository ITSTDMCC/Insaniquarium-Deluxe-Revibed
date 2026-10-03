//! `Sexy::PetDialog`: "PRESTO CHANGE-O" (dialog 0x1f), Presto's pet picker: the tank dimmed
//! behind a frame of the 24 pet icons (those earned), the hovered or pressed one lit; a
//! click on an icon turns Presto into that pet. Plus the app function that opens it.
//!
//! The object is 0x2f8 bytes: MoneyDialog (0x158), `PetDialog_data` at +0x158 (0x138
//! bytes: hover, press, 24 icon rects), then the earned flags at +0x2e0 (`ext_0x2e0`).

use crate::game::money_dialog::MoneyDialog_data;
use crate::sexy::dialog::{alloc_dialog, DlgSub, MoneySub};
use crate::sexy::graphics::{FUN_00455880, FUN_00455890, FUN_00455920, FUN_004559b0, FUN_00455d20};
use crate::sexy::prelude::*;
use crate::sexy::types::FUN_00433320;

/// `PetDialog_data` (object offset 0x158) plus the earned flags past it.
#[derive(Debug, Clone, Default)]
pub struct PetDialog_data {
    /// +0x158 the icon under the mouse (-1 none).
    pub offset_0x0: i32,
    /// +0x15c the icon pressed (-1 none).
    pub offset_0x4: i32,
    /// +0x160..+0x2e0 the icons' rects: up the left side, along the bottom, down the right.
    pub offset_0x8: [Rect; 24],
    /// +0x2e0 which pets are earned.
    pub ext_0x2e0: [bool; 24],
}

impl G {
    pub fn pet_dialog(&mut self, p: Ptr) -> &mut PetDialog_data {
        match &mut self.widget(p).ext {
            WExt::Dialog(e) => match &mut e.sub {
                DlgSub::Money(_, MoneySub::Pet(d)) => d,
                s => panic!("{p} is not a PetDialog: {s:?}"),
            },
            e => panic!("{p} is not a Dialog: {e:?}"),
        }
    }
}

/// port: 005355c0 FUN_005355c0
/// `PetDialog(WinFishApp*)`: modeless, a lone CANCEL button, full screen; the icons in a U
/// around the tank (seven up the left at x 10, eleven along y 420, six down the right),
/// 50x50 each; the earned pets.
pub fn FUN_005355c0(g: &mut G, param_1: Ptr) -> Ptr {
    let this = alloc_dialog(
        g,
        &crate::sexy::vtables_gen::Sexy__PetDialog_vftable,
        DlgSub::Money(MoneyDialog_data::default(), MoneySub::Pet(Box::default())),
    );
    let (comp, btn) = (g.res.DAT_005e8cc0, g.res.DAT_005e8c54);
    crate::game::money_dialog::MoneyDialog(g, this, param_1, comp, btn, 0x1f, false, b"PRESTO CHANGE-O", b"", b"CANCEL", 3);
    {
        let d = g.dialog(this);
        d.ext_0x11c = 100;
        d.ext_0x124 = 100;
        d.ext_0x128 = 10;
    }
    vfunction41(g, this, 0, 0, 0x280, 0x1e0);
    let d = g.pet_dialog(this);
    d.offset_0x0 = -1;
    d.offset_0x4 = -1;
    let mut k = 0;
    let mut y = 0x78;
    loop {
        d.offset_0x8[k] = Rect::new(10, y, 0x32, 0x32);
        k += 1;
        if y + 0x32 >= 0x1d6 {
            break;
        }
        y += 0x32;
    }
    let mut x = 0x3e;
    loop {
        d.offset_0x8[k] = Rect::new(x, y, 0x32, 0x32);
        k += 1;
        if x + 0x34 >= 0x27a {
            break;
        }
        x += 0x34;
    }
    for _ in 0..6 {
        y -= 0x32;
        d.offset_0x8[k] = Rect::new(x, y, 0x32, 0x32);
        k += 1;
    }
    let profile = g.wfa(param_1).offset_0x18c;
    for i in 0..0x18 {
        let e = crate::game::profile::FUN_00501410(g, profile, i);
        g.pet_dialog(this).ext_0x2e0[i] = e;
    }
    this
}

/// port: 00531370 Sexy::PetDialog::~PetDialog
pub fn dtor_PetDialog(g: &mut G, this: Ptr) {
    crate::game::money_dialog::dtor_MoneyDialog(g, this);
}

/// port: 005329c0 Sexy::PetDialog::deleting_destructor
pub fn deleting_destructor(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_PetDialog(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 005313d0 Sexy::PetDialog::vfunction41
/// `Resize(...)`: CANCEL at the top middle.
pub fn vfunction41(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32, param_4: i32) {
    crate::sexy::dialog::vfunction41_for_Widget(g, this, param_1, param_2, param_3, param_4);
    let yes = g.dialog(this).offset_0x8;
    let h = g.wc(yes).offset_0x38;
    vcall!(g, yes, w.vfunction41, 0x10e, 0x1e, 100, h);
}

/// port: 00531390 Sexy::PetDialog::vfunction61
/// `IsPointVisible(x, y)`: everywhere but the tank in the middle (clicks there go to it).
pub fn vfunction61(_g: &mut G, _this: Ptr, param_1: i32, param_2: i32) -> bool {
    if param_2 < 0 {
        return false;
    }
    !(((param_1 - 0x47) as u32) < 499 && ((param_2 - 0x47) as u32) < 0x153)
}

/// port: 005329f0 FUN_005329f0
/// The earned icon at a point, or -1.
pub fn FUN_005329f0(g: &mut G, this: Ptr, param_1: i32, param_2: i32) -> i32 {
    let d = g.pet_dialog(this);
    for (i, r) in d.offset_0x8.iter().enumerate() {
        if r.mX <= param_1 && param_1 < r.mWidth + r.mX && r.mY <= param_2 && param_2 < r.mHeight + r.mY {
            return if d.ext_0x2e0[i] { i as i32 } else { -1 };
        }
    }
    -1
}

/// port: 00530360 Sexy::PetDialog::vfunction52
/// `MouseLeave()`: nothing lit.
pub fn vfunction52(g: &mut G, this: Ptr) {
    crate::sexy::trivial::vfunction2__00486bf0();
    let d = g.pet_dialog(this);
    if d.offset_0x4 == -1 && d.offset_0x0 == -1 {
        return;
    }
    d.offset_0x4 = -1;
    d.offset_0x0 = -1;
    vcall!(g, this, w.vfunction18);
}

/// port: 00532af0 Sexy::PetDialog::vfunction53
/// `MouseMove(x, y)`: lights the icon under the mouse.
pub fn vfunction53(g: &mut G, this: Ptr, param_1: i32, param_2: i32) {
    crate::sexy::trivial::vfunction1__0046eb50();
    let i = FUN_005329f0(g, this, param_1, param_2);
    if i != g.pet_dialog(this).offset_0x0 {
        g.pet_dialog(this).offset_0x0 = i;
        vcall!(g, this, w.vfunction18);
    }
}

/// port: 00532a60 Sexy::PetDialog::vfunction55
/// `MouseDown(x, y, clicks)`: presses the icon under the mouse.
pub fn vfunction55(g: &mut G, this: Ptr, param_1: i32, param_2: i32, _clicks: i32) {
    let i = FUN_005329f0(g, this, param_1, param_2);
    let d = g.pet_dialog(this);
    d.offset_0x4 = i;
    d.offset_0x0 = i;
    vcall!(g, this, w.vfunction18);
}

/// port: 00532a90 Sexy::PetDialog::vfunction57
/// `MouseUp(x, y, clicks)`: released over the pressed icon, Presto becomes that pet.
pub fn vfunction57(g: &mut G, this: Ptr, param_1: i32, param_2: i32, _clicks: i32) {
    crate::sexy::trivial::vfunction1__0046eb50();
    let i = FUN_005329f0(g, this, param_1, param_2);
    let pressed = g.pet_dialog(this).offset_0x4;
    g.pet_dialog(this).offset_0x0 = i;
    if i == pressed && pressed != -1 {
        let app = g.money(this).offset_0x50;
        crate::game::win_fish_app::FUN_0054b450(g, app, pressed);
    }
    g.pet_dialog(this).offset_0x4 = -1;
    vcall!(g, this, w.vfunction18);
}

/// port: 00535800 Sexy::PetDialog::vfunction27
/// `Draw(Graphics*)`: dims around the tank, frames the screen and the tank, the title, and
/// each earned pet's icon on its (lit when hovered or pressed) button.
pub fn vfunction27(g: &mut G, this: Ptr, param_1: &mut Graphics) {
    FUN_00455890(param_1, FUN_00433320(0xb0000000));
    FUN_00455920(param_1, 0, 0, 0x46, 0x1e0);
    FUN_00455920(param_1, 0x23a, 0, 0x46, 0x1e0);
    FUN_00455920(param_1, 0x46, 0, 500, 0x46);
    FUN_00455920(param_1, 0x46, 0x19a, 500, 0x46);
    FUN_00455890(param_1, FUN_00433320(0xffffff));
    FUN_004559b0(param_1, 0, 0, 0x27f, 0x1df);
    FUN_00455890(param_1, FUN_00433320(0xffffff));
    FUN_004559b0(param_1, 0x46, 0x46, 500, 0x154);
    let f = g.res.DAT_005e8a5c;
    FUN_00455890(param_1, FUN_00433320(0xffff00));
    FUN_00455880(param_1, f);
    let a = crate::sexy::image_font::get_ascent(g, f);
    vcall!(g, this, w.vfunction63, param_1, a + 5, b"PRESTO CHANGE-O");
    let d = g.pet_dialog(this).clone();
    let app = g.money(this).offset_0x50;
    for i in 0..0x18usize {
        if d.ext_0x2e0[i] {
            let lit = i as i32 == d.offset_0x4 || i as i32 == d.offset_0x0;
            let r = d.offset_0x8[i];
            let bg = crate::sexy::res::FUN_005016a0(g, if lit { 0x3f } else { 0x3e }) as Ptr;
            FUN_00455d20(param_1, g, bg, r.mX, r.mY);
            let icon = g.wfa(app).offset_0x10[0x328 / 4 + i] as Ptr;
            FUN_00455d20(param_1, g, icon, r.mX + 5, r.mY + 5);
        }
    }
}

/// port: 0054c6f0 FUN_0054c6f0
/// `ShowPrestoDialog(Pet* presto)`: closes the dialogs, remembers Presto (+0x8b4) and opens
/// dialog 0x1f.
pub fn FUN_0054c6f0(g: &mut G, this: Ptr, param_1: Ptr) {
    crate::game::win_fish_app::FUN_0054b4c0(g, this);
    g.wfa(this).offset_0x188 = param_1 as i32;
    let d = FUN_005355c0(g, this);
    crate::sexy::sexy_app_base::vfunction76(g, this, 0x1f, d);
}
