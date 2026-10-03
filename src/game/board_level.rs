//! Board methods that set a level up: the per-level configuration (shop slots and prices,
//! alien schedule, backdrop), the starting fish, pets and bubbles, the welcome message,
//! adding objects to the board's per-kind vectors and re-stacking them.

use crate::sexy::prelude::*;
use crate::sexy::types::CRect;

/// `std::vector<GameObject*>` index of the board vector at object offset `off`
/// (+0x98 .. +0xf8).
pub fn vec_index(off: usize) -> usize {
    (off - 0x98) / 4
}

/// The app's own `MTRand` (`WinFishApp +0x7b0`), the generator these functions use.
fn app_rand(g: &mut G, app: Ptr) -> u32 {
    let r = g.wfa(app).offset_0x84;
    crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(r))
}

/// port: 005420e0 FUN_005420e0
/// `std::vector<GameObject*>::push_back` (an STL instance; `Vec::push`).
pub fn FUN_005420e0(this: &mut Vec<Ptr>, param_1: Ptr) {
    this.push(param_1);
}

/// port: 00540ec0 FUN_00540ec0
/// `std::set<GameObject*>::insert` (an STL instance; `BTreeSet::insert`): true when newly
/// inserted.
pub fn FUN_00540ec0(this: &mut std::collections::BTreeSet<Ptr>, param_1: Ptr) -> bool {
    this.insert(param_1)
}

/// port: 00541a20 FUN_00541a20
/// `Board::InitLevel()`: empties the shop slots, takes tank and level from the profile
/// (adventure) or the app (other modes), resets money, alien timers and the level
/// statistics, adds the Menu button, money label and message line, applies the level
/// configuration and prices.
pub fn FUN_00541a20(g: &mut G, this: Ptr) {
    let app = g.board(this).field_0x0;
    {
        let b = g.board(this);
        b.ext_0x444 = 0;
        for i in 0..0xc {
            b.field_0x258[i] = -1;
            b.field_0x288[i] = 0;
            b.field_0x2e8[i] = -1;
            b.field_0x318[i] = false;
        }
    }
    if g.wfa(app).offset_0x150 == 0 {
        let profile = g.wfa(app).offset_0x18c;
        let (t, l) = (g.profile(profile).field_0x1c, g.profile(profile).field_0x20);
        let b = g.board(this);
        b.field_0x33c = t;
        b.field_0x340 = l;
    } else {
        let t = g.wfa(app).offset_0x15c;
        let b = g.board(this);
        b.field_0x33c = t;
        b.field_0x340 = 5;
    }
    let mode = g.wfa(app).offset_0x150;
    {
        let b = g.board(this);
        b.field_0x364 = if mode == 3 { 999999 } else { 200 };
        b.ext_0x4ac = 5;
        b.field_0x234 = 3000;
        b.field_0x218 = false;
        b.field_0x23c = 0;
        b.field_0x21c = 0;
        b.ext_0x43c = 1;
        b.field_0x358 = 2;
        b.field_0x230 = 2;
    }
    g.globals.DAT_005e89dc = 0;
    g.globals.DAT_005df58c = 1;
    g.globals.DAT_005e89cc = false;
    g.globals.DAT_005e89d0 = 0;
    g.globals.DAT_005e89c0 = 0;
    g.board(this).ext_0x440 = false;
    if g.wfa(app).offset_0x154 {
        g.globals.DAT_005e89dc = 0;
        g.globals.DAT_005df58c = 3;
        let b = g.board(this);
        b.field_0x358 = 8;
        b.field_0x364 = 200;
        g.globals.DAT_005e8f18 = 0;
    }
    g.board(this).ext_0x460 = [0; 19];
    let mgr = g.wc(this).offset_0xc;
    let (menu, label, msg) = (g.board(this).offset_0x88, g.board(this).offset_0x3ac, g.board(this).offset_0x8c);
    vcall!(g, mgr, w.vfunction4, menu);
    vcall!(g, mgr, w.vfunction4, label);
    vcall!(g, mgr, w.vfunction4, msg);
    FUN_0053a360(g, this);
    g.board(this).ext_0x4b0 = [0; 0x36];
    FUN_00541200(g, this);
    let b = g.board(this);
    b.field_0x250 = 0;
    b.field_0x254 = 0;
    for i in 0..0xc {
        if -1 < b.field_0x258[i] {
            b.field_0x250 += 1;
        }
        if b.field_0x2e8[i] == -1 {
            b.field_0x2e8[i] = b.field_0x288[i] / 100;
        }
        b.field_0x2b8[i] = b.field_0x288[i];
    }
    if g.wfa(app).offset_0x150 == 5 {
        crate::game::board_save::FUN_0053e280(g, this);
    }
    vcall!(g, mgr, w.vfunction11);
}

/// port: 0053a360 FUN_0053a360
/// `UpdateMoneyLabel()`: the shells (virtual tank) or the money, as "%d".
pub fn FUN_0053a360(g: &mut G, this: Ptr) {
    let label = g.board(this).offset_0x3ac;
    if label != NULL {
        let app = g.board(this).field_0x0;
        let v = if g.wfa(app).offset_0x150 == 5 {
            let profile = g.wfa(app).offset_0x18c;
            g.profile(profile).field_0x48
        } else {
            g.board(this).field_0x364
        };
        let s = format!("{v}").into_bytes();
        crate::game::board_parts::FUN_005346d0(g, label, &s);
    }
}

/// port: 00541200 FUN_00541200
/// `SetupLevel()`: the shop slots, prices, alien schedule (+0x2bc) and egg price (slot 11)
/// for the tank and level, then the backdrop.
pub fn FUN_00541200(g: &mut G, this: Ptr) {
    let app = g.board(this).field_0x0;
    if !g.wfa(app).offset_0x154 {
        let tank = g.board(this).field_0x33c;
        if tank == 1 {
            {
                let b = g.board(this);
                b.field_0x288[4] = 1000; // +0x324
                b.field_0x288[10] = 1000; // +0x33c
                b.field_0x258[0] = 0;
                b.field_0x258[2] = 1;
                b.field_0x258[3] = 2;
                b.field_0x258[4] = 3; // +0x2f4
                b.field_0x258[10] = 5; // +0x30c
                b.field_0x258[11] = 6; // +0x310
                b.field_0x288[0] = 100;
                b.field_0x288[2] = 200;
                b.field_0x288[3] = 300;
            }
            let b = g.board(this);
            match b.field_0x340 {
                1 => {
                    b.field_0x230 = 0;
                    b.field_0x288[11] = 0x96;
                    b.field_0x258[2] = -1;
                    b.field_0x258[3] = -1;
                    b.field_0x258[4] = -1;
                    b.field_0x258[10] = -1;
                }
                2 => {
                    b.field_0x288[11] = 500;
                    b.field_0x230 = 1;
                    b.field_0x234 = 0x6d6;
                    b.field_0x258[4] = -1;
                    b.field_0x258[10] = -1;
                }
                3 => {
                    b.field_0x288[11] = 2000;
                    b.field_0x230 = 2;
                }
                4 => {
                    b.field_0x288[11] = 3000;
                    b.field_0x230 = 3;
                }
                5 => {
                    b.field_0x288[11] = 5000;
                    b.field_0x230 = 3;
                }
                _ => {}
            }
        } else if tank == 2 {
            {
                let b = g.board(this);
                b.field_0x258[0] = 0;
                b.field_0x258[2] = 1;
                b.field_0x258[3] = 2;
                b.field_0x258[5] = 3;
                b.field_0x258[6] = 4;
                b.field_0x258[10] = 5;
                b.field_0x258[11] = 6;
                b.field_0x288[0] = 100;
                b.field_0x288[2] = 200;
                b.field_0x288[3] = 300;
                b.field_0x288[5] = 0xfa;
                b.field_0x288[6] = 0x2ee;
                b.field_0x288[10] = 1000;
            }
            let level = g.board(this).field_0x340;
            match level {
                1 => {
                    let b = g.board(this);
                    b.field_0x288[11] = 0x2ee;
                    b.field_0x230 = 2;
                    b.field_0x258[10] = -1;
                    b.field_0x258[6] = -1;
                }
                2 => {
                    let b = g.board(this);
                    b.field_0x288[11] = 3000;
                    b.field_0x230 = 3;
                }
                3 => {
                    let b = g.board(this);
                    b.field_0x288[11] = 5000;
                    b.field_0x230 = 4;
                }
                4 => {
                    let b = g.board(this);
                    b.field_0x288[11] = 0x1d4c;
                    b.field_0x230 = 5;
                }
                5 => {
                    g.board(this).field_0x288[11] = 10000;
                    let r = app_rand(g, app);
                    g.board(this).field_0x230 = (r & 1 | 4) as i32;
                }
                _ => {}
            }
        } else if tank == 3 {
            {
                let b = g.board(this);
                b.field_0x258[0] = 0;
                b.field_0x288[8] = 2000; // +0x334
                b.field_0x288[10] = 2000;
                b.field_0x258[2] = 1;
                b.field_0x258[3] = 2;
                b.field_0x258[7] = 3;
                b.field_0x258[8] = 4;
                b.field_0x258[10] = 5;
                b.field_0x258[11] = 6;
                b.field_0x288[0] = 100;
                b.field_0x288[2] = 200;
                b.field_0x288[3] = 300;
                b.field_0x288[7] = 0x2ee;
            }
            match g.board(this).field_0x340 {
                1 => {
                    let b = g.board(this);
                    b.field_0x288[11] = 1000;
                    b.field_0x230 = 3;
                    b.field_0x258[10] = -1;
                    b.field_0x258[8] = -1;
                }
                2 => {
                    g.board(this).field_0x288[11] = 5000;
                    let r = app_rand(g, app);
                    g.board(this).field_0x230 = (r & 1 | 4) as i32;
                }
                3 => {
                    let b = g.board(this);
                    b.field_0x288[11] = 0x1d4c;
                    b.field_0x230 = 7;
                }
                4 => {
                    let b = g.board(this);
                    b.field_0x288[11] = 10000;
                    b.field_0x230 = 6;
                }
                5 => {
                    g.board(this).field_0x288[11] = 15000;
                    let r = app_rand(g, app);
                    g.board(this).field_0x230 = (r & 1 | 6) as i32;
                }
                _ => {}
            }
        } else if tank == 4 {
            let b = g.board(this);
            b.field_0x258[1] = 0; // +0x2e8
            b.field_0x288[1] = 200; // +0x318
            b.field_0x288[2] = 200;
            b.field_0x258[2] = 1;
            b.field_0x258[3] = 2;
            b.field_0x258[4] = 3;
            b.field_0x258[9] = 4;
            b.field_0x258[10] = 5;
            b.field_0x258[11] = 6;
            b.field_0x288[3] = 300;
            b.field_0x288[4] = 1000;
            b.field_0x288[9] = 10000;
            b.field_0x288[10] = 5000;
            match b.field_0x340 {
                1 => {
                    b.field_0x288[11] = 3000;
                    b.field_0x230 = 3;
                    b.field_0x258[10] = -1;
                    b.field_0x258[9] = -1;
                }
                2 => {
                    b.field_0x288[11] = 25000;
                    b.field_0x230 = 8;
                }
                3 => {
                    b.field_0x288[11] = 50000;
                    b.field_0x230 = 4;
                }
                4 => {
                    b.field_0x288[11] = 75000;
                    b.field_0x230 = 8;
                }
                5 => {
                    b.field_0x288[11] = 99999;
                    b.field_0x230 = 8;
                }
                _ => {}
            }
        } else if tank == 5 {
            let b = g.board(this);
            b.field_0x258[11] = 6;
            b.field_0x288[11] = 0;
            b.ext_0x43c = 3;
            b.field_0x230 = 0x15;
        }
    } else {
        {
            let b = g.board(this);
            b.field_0x288[0] = 100;
            b.field_0x288[4] = 1000;
            b.field_0x288[9] = 10000;
            b.field_0x288[6] = 0x2ee;
            b.field_0x288[7] = 0x2ee;
            b.field_0x288[8] = 2000;
            b.field_0x258[11] = 6;
        }
        let set_slot0 = {
            let b = g.board(this);
            match b.field_0x33c {
                1 => {
                    b.field_0x288[11] = 5000;
                    b.field_0x258[4] = 3;
                    true
                }
                2 => {
                    b.field_0x258[6] = 3;
                    b.field_0x288[11] = 5000;
                    true
                }
                3 => {
                    b.field_0x258[7] = 3;
                    b.field_0x258[8] = 5;
                    b.field_0x288[11] = 10000;
                    true
                }
                4 => {
                    b.field_0x258[9] = 5;
                    b.field_0x288[11] = 25000;
                    b.field_0x258[4] = 3;
                    true
                }
                _ => false,
            }
        };
        if set_slot0 {
            g.board(this).field_0x258[0] = 0;
        }
        for i in 0..0xc {
            FUN_005409b0(g, this, i, true);
        }
        FUN_00537c20(g, this);
    }
    let mode = g.wfa(app).offset_0x150;
    if mode == 4 {
        let b = g.board(this);
        match b.field_0x33c {
            2 => b.field_0x288[11] = 5000,
            3 => b.field_0x288[11] = 15000,
            4 => b.field_0x288[11] = 25000,
            _ => {}
        }
    }
    if mode == 1 {
        g.board(this).field_0x288[11] = 100;
    }
    if mode == 5 {
        let profile = g.wfa(app).offset_0x18c;
        let n = g.profile(profile).field_0x8c + 1;
        FUN_00538a10(g, this, n);
        return;
    }
    match g.board(this).field_0x33c {
        1 => FUN_00538a10(g, this, 1),
        2 => FUN_00538a10(g, this, 2),
        3 => FUN_00538a10(g, this, 4),
        4 => FUN_00538a10(g, this, 5),
        5 => FUN_00538a10(g, this, 6),
        _ => {}
    }
}

/// port: 00537c20 FUN_00537c20
/// Picks the alien schedule at random for the tank (the sandbox-style setup). Tank 4 reads
/// its table after overwriting entry 1 with `rand / 6`, as the original does.
pub fn FUN_00537c20(g: &mut G, this: Ptr) -> u32 {
    let mut t = [0u32; 6];
    t[1] = g.board(this).field_0x33c as u32;
    if t[1] == 1 {
        t[4] = 2;
        t[5] = 3;
        let r = crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32 % 2;
        let v = t[(r + 4) as usize];
        g.board(this).field_0x230 = v as i32;
        return v;
    }
    if t[1] == 2 {
        t[3] = 3;
        t[4] = 4;
        t[5] = 5;
        t[2] = t[1];
        let r = crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32 % 4;
        g.board(this).field_0x230 = t[(r + 2) as usize] as i32;
        return r as u32;
    }
    if t[1] == 3 {
        t[1] = 3;
        t[2] = 4;
        t[3] = 5;
        t[4] = 7;
        t[5] = 6;
        let r = crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32;
        g.board(this).field_0x230 = t[(r % 5 + 1) as usize] as i32;
        return (r / 5) as u32;
    }
    if t[1] == 4 {
        t[0] = 3;
        t[2] = 5;
        t[3] = 7;
        t[4] = 6;
        t[5] = 8;
        let r = crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32;
        t[1] = (r / 6) as u32;
        g.board(this).field_0x230 = t[(r % 6) as usize] as i32;
    }
    t[1]
}

/// port: 00538a10 FUN_00538a10
/// `SetBackdrop(int)` (clamped to 1..6), then its tint.
pub fn FUN_00538a10(g: &mut G, this: Ptr, param_1: i32) {
    if param_1 < 1 {
        g.board(this).field_0x35c = 1;
        FUN_00537d30(g, this);
        return;
    }
    g.board(this).field_0x35c = param_1.min(6);
    FUN_00537d30(g, this);
}

/// port: 00537d30 FUN_00537d30
/// The backdrop's tint (`DAT_005e89e8`, used for shadows); dark gray with profile flag 2.
pub fn FUN_00537d30(g: &mut G, this: Ptr) {
    let c = match g.board(this).field_0x35c {
        1 => Some((0x42, 0x30, 0x29)),
        2 => Some((0x9a, 0x4d, 0x6f)),
        3 => Some((0x2f, 0x27, 0x7c)),
        4 => Some((0xf, 0x49, 0x3a)),
        5 => Some((0x6e, 0x53, 0x48)),
        6 => Some((0x32, 0, 0x3b)),
        _ => None,
    };
    if let Some((r, gr, b)) = c {
        g.globals.DAT_005e89e8 = FUN_00433360(r, gr, b);
    }
    let app = g.board(this).field_0x0;
    let profile = g.wfa(app).offset_0x18c;
    if (g.profile(profile).field_0x84 >> 2) & 1 != 0 {
        g.globals.DAT_005e89e8 = FUN_00433320(0x333333);
    }
}

/// port: 005409b0 FUN_005409b0
/// `MakeShopButton(int slot, bool)`: the button for the slot's item, created once per item
/// at its fixed x (images IMAGE_BUTTONS...), counted in +0x2e0.
pub fn FUN_005409b0(g: &mut G, this: Ptr, param_1: usize, param_2: bool) -> Ptr {
    let item = g.board(this).field_0x258[param_1];
    if item < 0 {
        return NULL;
    }
    let mut b = g.board(this).field_0x378[item as usize];
    if b == NULL {
        const XS: [i32; 7] = [0x12, 0x57, 0x90, 0xd9, 0x122, 0x16b, 0x1b4];
        let label = FUN_00538100(g, this, param_1 as i32);
        let mgr = g.wc(this).offset_0xc;
        b = crate::game::menu_button::MenuButtonWidget(g, mgr, param_1 as i32, this, label);
        g.board(this).field_0x378[item as usize] = b;
        vcall!(g, mgr, w.vfunction4, b);
        let (i0, i1, i2) = (g.res.DAT_005e8c60, g.res.DAT_005e8d64, g.res.DAT_005e8e74);
        let bd = g.btn(b);
        bd.offset_0x28 = i0;
        bd.offset_0x30 = i1;
        bd.offset_0x2c = i2;
        vcall!(g, b, w.vfunction41, XS[item as usize], 3, 0x3a, 0x3c);
        FUN_005406a0(g, this, param_1, param_2);
        let d = g.board(this);
        d.field_0x318[param_1] = true;
        d.field_0x254 += 1;
    }
    b
}

/// port: 005498b0 FUN_005498b0
/// `StartLevel()`: the start sound, 3..6 bubbles, the pets the player has earned (24 pet
/// flags once adventure is finished or the fourth tank is reached, else one per finished
/// level), the starting fish (two guppies; a carnivore in tank 4; pets only in tank 5), the
/// time limit (time trial), the welcome message, and the level clocks.
pub fn FUN_005498b0(g: &mut G, this: Ptr) {
    g.board(this).field_0x8 = false;
    crate::game::board::FUN_00538230(g, this, 0x110, 3, 1.0);
    let app = g.board(this).field_0x0;
    let mut n = (app_rand(g, app) & 3) as i32 + 2;
    while -1 < n {
        FUN_00538a50(g, this);
        n -= 1;
    }
    let profile = g.wfa(app).offset_0x18c;
    let late = 3 < g.profile(profile).field_0x18;
    let mode = g.wfa(app).offset_0x150;
    if (g.profile(profile).field_0x59 && (mode == 1 || mode == 4)) || late {
        for i in 0..0x18 {
            let profile = g.wfa(app).offset_0x18c;
            if g.profile(profile).field_0x5a[i] {
                crate::game::board_level::FUN_00544a90(g, this, i as i32, -1, -1, false, false);
            }
        }
    } else {
        for i in 0..0x13 {
            let profile = g.wfa(app).offset_0x18c;
            if crate::game::profile::FUN_00501410(g, profile, i) {
                crate::game::board_level::FUN_00544a90(g, this, i as i32, -1, -1, false, false);
            }
        }
    }
    let guppies = vec_index(0xa0);
    if g.wfa(app).offset_0x154 {
        for _ in 0..2 {
            let x = (app_rand(g, app) % 0x208) as i32 + 0x14;
            let y = (app_rand(g, app) % 0x109) as i32 + 0x69;
            FUN_00546ef0(g, this, x, y);
        }
    } else if g.board(this).field_0x33c == 4 {
        let x = (app_rand(g, app) % 0x208) as i32 + 0x14;
        let y = (app_rand(g, app) % 0x109) as i32 + 0x69;
        FUN_005471e0(g, this, x, y);
        let first = g.board(this).offset_0xc[vec_index(0xc0)][0];
        g.breeder(first).field_0x50 = 2;
    } else if g.board(this).field_0x33c == 5 {
        g.board(this).field_0x234 = 300;
        for i in 0..0x11 {
            crate::game::board_level::FUN_00544a90(g, this, i, -1, -1, false, false);
        }
        crate::game::board_level::FUN_00544a90(g, this, 0x11, -1, -1, false, false);
    } else {
        for _ in 0..2 {
            let x = (app_rand(g, app) % 0x208) as i32 + 0x14;
            let y = (app_rand(g, app) % 0x109) as i32 + 0x69;
            FUN_00546ef0(g, this, x, y);
        }
        let first_level = g.board(this).field_0x340 == 1 && g.board(this).field_0x33c == 1;
        for k in 0..2 {
            let f = g.board(this).offset_0xc[guppies][k];
            crate::game::fish::fish_data(g, f).field_0x50 = 2;
            if first_level {
                crate::game::fish::fish_data(g, f).field_0xd4 = true;
            }
        }
    }
    if g.wfa(app).offset_0x150 == 1 {
        let b = g.board(this);
        match b.field_0x33c {
            1 => b.field_0x330 = 300,
            2 | 3 | 4 => b.field_0x330 = 600,
            _ => {}
        }
    }
    let mode = g.wfa(app).offset_0x150;
    let profile = g.wfa(app).offset_0x18c;
    let done = g.profile(profile).field_0x59;
    let (tank, level) = (g.board(this).field_0x33c, g.board(this).field_0x340);
    let msg: Option<(Vec<u8>, i32)> = if mode == 0 && level == 1 && tank == 1 {
        if done {
            if g.profile(profile).field_0x50 != 1 {
                None
            } else {
                Some((b"Welcome to the Bonus Adventure!".to_vec(), 0))
            }
        } else {
            Some((b"Welcome to the Insaniquarium!".to_vec(), 0))
        }
    } else if mode == 0 && level == 2 && tank == 1 && !done {
        Some((b"Welcome to the next level!".to_vec(), 0x10))
    } else if mode == 0 && level == 1 && tank == 2 && !done {
        Some((b"Welcome to the second tank! Now things get tricky!".to_vec(), 0x13))
    } else if mode == 4 {
        Some((b"Welcome to Challenge mode!".to_vec(), 0x32))
    } else if mode == 1 {
        // The format has no conversion; the time limit in minutes is passed and unused.
        Some((b"Welcome to Time Trial mode!".to_vec(), 0x34))
    } else if mode == 3 {
        Some((b"You've discovered the top-secret SANDBOX mode!".to_vec(), 0x2f))
    } else {
        None
    };
    if let Some((text, id)) = msg {
        FUN_0053e950(g, this, &text, false, id);
    }
    let b = g.board(this);
    b.ext_0x444 = 0;
    b.ext_0x450 = 0;
    b.ext_0x454 = 0;
    b.ext_0x45c = 0;
    b.ext_0x458 = 0;
    let t = crate::game::board::FUN_00537b60(g, this);
    let b = g.board(this);
    b.ext_0x4e8 = 0;
    b.field_0x328 = t;
    b.field_0x32c = t;
    b.ext_0x4e6 = false;
}

/// port: 00538a50 FUN_00538a50
/// One more bubble near the bottom (fewer than 50 only).
pub fn FUN_00538a50(g: &mut G, this: Ptr) {
    let bm = g.board(this).offset_0x90;
    if (g.bubble_mgr(bm).offset_0x40.len() as i32) < 0x32 {
        let app = g.board(this).field_0x0;
        let x = (app_rand(g, app) % 0x16) as i32 + 0x96;
        let y = (app_rand(g, app) % 6) as i32 + 400;
        crate::game::board_parts::FUN_00510f70(g, bm, x, y);
    }
}

/// port: 00546ef0 FUN_00546ef0
/// `AddGuppy(int x, int y)`: a new guppy on the board (and in the most-guppies statistic),
/// with its shadow, re-stacked.
pub fn FUN_00546ef0(g: &mut G, this: Ptr, param_1: i32, param_2: i32) -> Ptr {
    let f = crate::game::fish::Fish__004eef00(g, param_1, param_2);
    FUN_00542ee0(g, this, f, 1);
    let mgr = g.wc(this).offset_0xc;
    vcall!(g, mgr, w.vfunction4, f);
    let n = g.board(this).offset_0xc[vec_index(0xa0)].len() as i32;
    let b = g.board(this);
    if b.ext_0x460[(0x48c - 0x460) / 4] < n {
        b.ext_0x460[(0x48c - 0x460) / 4] = n;
    }
    FUN_00544800(g, this, f);
    FUN_0053aa40(g, this);
    f
}

/// port: 00542ee0 FUN_00542ee0
/// `AddObject(GameObject*, bool)`: files the object in its kind's vector (by type id at
/// +0x8c; pets also counted per pet id), inserts it into the object set and, except for
/// kinds 0x1e..0x21, marks +0x2a7.
pub fn FUN_00542ee0(g: &mut G, this: Ptr, param_1: Ptr, _param_2: i32) {
    let ty = g.go(param_1).offset_0x4;
    let mut mark = true;
    let push = |g: &mut G, off: usize| FUN_005420e0(&mut g.board(this).offset_0xc[vec_index(off)], param_1);
    match ty {
        0 => push(g, 0xa0),
        5 => push(g, 0xa4),
        6 => push(g, 0xd4),
        7 => push(g, 0xcc),
        8 => push(g, 0xd0),
        9 => push(g, 0xc4),
        0xa => push(g, 0xc0),
        0x14 => {
            let id = g.other_pet(param_1).offset_0x70;
            if -1 < id && id < 0x18 {
                g.board(this).field_0xb8[id as usize] += 1;
            }
            push(g, 0xb0);
        }
        0x15 => {
            let id = crate::game::fish_type_pet::pet_id(g, param_1);
            if -1 < id && id < 0x18 {
                g.board(this).field_0xb8[id as usize] += 1;
            }
            push(g, 0xb4);
        }
        0x16 => {
            let off = if g.alien(param_1).offset_0x98 == 0x14 { 0xbc } else { 0xb8 };
            push(g, off);
        }
        0x17 => push(g, 0xf4),
        0x19 => {
            let off = if 1 < g.coin(param_1).offset_0x4c {
                0xec
            } else {
                match g.coin(param_1).offset_0x40 {
                    0xf => 0xec,
                    0x10 => 0xe8,
                    _ => 0xa8,
                }
            };
            push(g, off);
        }
        0x1a => push(g, 0x9c),
        0x1b => push(g, 0x98),
        0x1c => push(g, 0xac),
        0x1d => push(g, 0xc8),
        0x1e => {
            let off = if crate::game::missle::FUN_004d80c0(g, param_1) { 0xe0 } else { 0xdc };
            push(g, off);
            mark = false;
        }
        0x1f => {
            push(g, 0xe4);
            mark = false;
        }
        0x20 => {
            push(g, 0xd8);
            mark = false;
        }
        0x21 => {
            push(g, 0xf0);
            mark = false;
        }
        0x22 | 0x23 | 0x24 => push(g, 0xf8),
        _ => {}
    }
    FUN_00540ec0(&mut g.board(this).offset_0x7c, param_1);
    if mark {
        g.board(this).field_0x21b = true;
    }
}

/// port: 00544800 FUN_00544800
/// Gives a new object its shadow (kind 0 normal, 1 big, 2 offset), unless it has one;
/// some kinds get none.
pub fn FUN_00544800(g: &mut G, this: Ptr, param_1: Ptr) {
    if g.go(param_1).offset_0xc != NULL {
        return;
    }
    let ty = g.go(param_1).offset_0x4;
    let kind = match ty {
        0 | 5 | 7 | 0xa | 0x1b | 0x23 | 0x24 => Some(0),
        6 => Some(2),
        8 | 9 => Some(1),
        0x14 => match g.other_pet(param_1).offset_0x70 {
            5 => Some(0),
            1 => None,
            _ => Some(1),
        },
        0x15 => {
            let id = crate::game::fish_type_pet::pet_id(g, param_1);
            if id == 0x10 {
                Some(2)
            } else {
                Some((id == 0x14) as i32)
            }
        }
        0x16 => Some(if g.alien(param_1).offset_0x98 != 0x14 { 2 } else { 0 }),
        0x17 | 0x19 | 0x1a | 0x1c | 0x1d | 0x1e | 0x1f | 0x20 | 0x21 => None,
        0x22 => Some(if g.fish(param_1).offset_0x4c != 1 { 2 } else { 0 }),
        _ => Some(0),
    };
    if let Some(k) = kind {
        FUN_00544370(g, this, k, param_1);
    }
}

/// port: 00544370 FUN_00544370
/// `AddShadow(int kind, GameObject* owner)`: at most 40 shadows; visible only with 3D
/// acceleration (`SexyAppBase::Is3DAccelerated`).
pub fn FUN_00544370(g: &mut G, this: Ptr, param_1: i32, param_2: Ptr) {
    if (g.board(this).offset_0xc[vec_index(0xe4)].len() as u32) < 0x28 {
        let s = crate::game::shadow::Shadow__004ec4f0(g, param_1, param_2);
        let app = g.board(this).field_0x0;
        let vis = crate::sexy::sexy_app_base::dtor_MemoryImage__00489a20(g, app);
        g.w(s).offset_0x0 = vis;
        FUN_00542ee0(g, this, s, 0);
        let mgr = g.wc(this).offset_0xc;
        vcall!(g, mgr, w.vfunction4, s);
        FUN_0053aa40(g, this);
    }
}

/// Brings every object of one vector to the front, except objects in the middle of being
/// dropped in (+0xfc set with +0x100 zero).
fn front_all(g: &mut G, this: Ptr, mgr: Ptr, off: usize) {
    let objs = g.board(this).offset_0xc[vec_index(off)].clone();
    for o in objs {
        if !g.go(o).offset_0x74 || g.go(o).offset_0x78 != 0 {
            vcall!(g, mgr, w.vfunction12, o);
        }
    }
}

/// port: 0053aa40 FUN_0053aa40
/// `RestackObjects()`: rebuilds the widget order bottom to top: the overlay layer 0, every
/// object vector in drawing order (special cases for the pets, the "boss" vectors and the
/// selected item), the message line, overlay layer 1, then the open dialogs.
pub fn FUN_0053aa40(g: &mut G, this: Ptr) {
    let mgr = g.wc(this).offset_0xc;
    if mgr != NULL {
        vcall!(g, mgr, w.vfunction11);
        let o0 = g.board(this).offset_0x70;
        vcall!(g, mgr, w.vfunction12, o0);
    }
    for off in [0xe4, 0x98, 0x9c, 0xe8, 0xf0, 0xd0, 0xc4, 0xac, 0xa0, 0xa4, 0xc8, 0xcc, 0xd4, 0xc0, 0xf8] {
        front_all(g, this, mgr, off);
    }
    let guppies = g.board(this).offset_0xc[vec_index(0xa0)].clone();
    for o in guppies {
        if crate::game::fish::fish_data(g, o).offset_0x4c == 4 {
            vcall!(g, mgr, w.vfunction12, o);
        }
    }
    if g.board(this).field_0x84 == NULL {
        front_all(g, this, mgr, 0xb8);
        front_all(g, this, mgr, 0xf4);
    }
    let pets = g.board(this).offset_0xc[vec_index(0xb0)].clone();
    for o in pets {
        if g.other_pet(o).offset_0x70 == 1 && !g.other_pet(o).offset_0x74 {
            let o0 = g.board(this).offset_0x70;
            vcall!(g, mgr, w.vfunction14, o, o0);
        } else {
            vcall!(g, mgr, w.vfunction12, o);
        }
        vcall!(g, o, w.vfunction18);
    }
    for off in [0xb4, 0xdc, 0xe0] {
        front_all(g, this, mgr, off);
    }
    let sel = g.board(this).field_0x84;
    if sel != NULL {
        vcall!(g, mgr, w.vfunction12, sel);
        vcall!(g, sel, w.vfunction18);
        front_all(g, this, mgr, 0xb8);
        front_all(g, this, mgr, 0xf4);
    }
    for off in [0xbc, 0xd8, 0xa8] {
        front_all(g, this, mgr, off);
    }
    let pick = g.board(this).field_0x220;
    if pick != -1 {
        let objs = g.board(this).offset_0xc[vec_index(0xa8)].clone();
        for o in objs {
            if g.coin(o).offset_0x40 == pick {
                vcall!(g, mgr, w.vfunction12, o);
            }
        }
    }
    front_all(g, this, mgr, 0xec);
    let msg = g.board(this).offset_0x8c;
    if msg != NULL {
        vcall!(g, mgr, w.vfunction12, msg);
    }
    let o1 = g.board(this).offset_0x74;
    vcall!(g, mgr, w.vfunction12, o1);
    let app = g.board(this).field_0x0;
    let dialogs = g.sab(app).offset_0x32c.clone();
    for d in dialogs {
        vcall!(g, mgr, w.vfunction12, d);
        vcall!(g, d, w.vfunction18);
    }
}

/// port: 0053e950 FUN_0053e950
/// `ShowMessage(const string&, bool blink, int id)`: id -1 always shows (100 updates, or
/// 500 blinking); other ids show once per level (185 updates).
pub fn FUN_0053e950(g: &mut G, this: Ptr, param_1: &[u8], param_2: bool, param_3: i32) {
    let msg = g.board(this).offset_0x8c;
    if param_3 == -1 {
        let m = g.message_widget(msg);
        m.offset_0x50 = -1;
        m.field_0x4 = param_1.to_vec();
        m.offset_0x4c = param_2;
        m.offset_0x48 = if param_2 { 500 } else { 100 };
    } else if g.board(this).ext_0x4b0[param_3 as usize] == 0 {
        let m = g.message_widget(msg);
        m.offset_0x50 = param_3;
        m.field_0x4 = param_1.to_vec();
        m.offset_0x4c = param_2;
        if param_2 {
            m.offset_0x48 = 500;
        }
        g.board(this).ext_0x4b0[param_3 as usize] = 1;
        g.message_widget(msg).offset_0x48 = 0xb9;
    }
    let m = g.message_widget(msg);
    m.offset_0x20 = CRect(0xb4, 0xfa, 0x5a, 0xff);
    m.offset_0x30 = CRect(0, 0x4b, 0, 0xff);
}

/// port: 00540cc0 FUN_00540cc0
/// `SingSong(int, int kind)`: unless a melody is already playing, picks a song of `kind`
/// (see `FUN_0050fd00`), shows its title (" (extended)" for a long version) for 180
/// updates and starts it; a long normal song ends in applause. The melody id, or -1.
pub fn FUN_00540cc0(g: &mut G, this: Ptr, _param_1: i32, param_2: i32) -> i32 {
    let tm = g.board(this).offset_0x4;
    if !g.timed_messages(tm).field_0x4.is_empty() {
        return -1;
    }
    let id = g.timed_messages(tm).field_0x1c;
    g.timed_messages(tm).field_0x1c = id + 1;
    let Some(song) = crate::game::fish_songs::FUN_0050fd00(g, param_2) else {
        return -1;
    };
    let song = g.globals.songs.DAT_005e8f68[song].clone();
    let mut title = Vec::new();
    if crate::game::fish_songs::FUN_0050ecc0(&song, b"title", Some(&mut title)) {
        if crate::game::fish_songs::FUN_0050ecc0(&song, b"long", None) {
            title.extend_from_slice(b" (extended)");
        }
        FUN_0053e950(g, this, &title, false, -1);
        let msg = g.board(this).offset_0x8c;
        g.message_widget(msg).offset_0x48 = 0xb4;
    }
    g.globals.DAT_005e89cd = true;
    let m = crate::game::timed_messages::FUN_00515c70(g, tm, &song, id);
    if let Some(m) = m {
        if param_2 != 4 && param_2 != 5 && crate::game::fish_songs::FUN_0050ecc0(&song, b"long", None) {
            let applause = g.res.DAT_005e8c38;
            g.timed_messages(tm).field_0x4[m].field_0x2c = applause;
        }
    }
    id
}

/// port: 00537f40 FUN_00537f40
/// The bonus round begins (after its countdown): the level music unless this is the
/// adventure bonus level, +0x2a4 set, the clocks restarted, the HUD's start count.
pub fn FUN_00537f40(g: &mut G, this: Ptr) {
    if g.board(this).field_0x340 != 6 {
        let app = g.board(this).field_0x0;
        crate::game::win_fish_app::FUN_0054b2d0(g, app);
    }
    g.board(this).field_0x218 = true;
    let t = crate::game::board::FUN_00537b60(g, this);
    let b = g.board(this);
    b.field_0x328 = t;
    b.field_0x32c = t;
    b.field_0x324 = 0;
    b.field_0x21c = b.ext_0x444;
}

/// port: 00537e20 FUN_00537e20
/// `StartBonusRound()`: clears the message and the alien schedule, marks +0x2a5, sets the
/// bonus time from tank and level, resets the level clocks.
pub fn FUN_00537e20(g: &mut G, this: Ptr) {
    let msg = g.board(this).offset_0x8c;
    g.message_widget(msg).offset_0x48 = 0;
    {
        let b = g.board(this);
        b.field_0x21c = b.ext_0x444;
        b.field_0x230 = 0;
        b.field_0x8 = false;
        b.field_0x219 = true;
        b.field_0x218 = false;
    }
    g.globals.DAT_005e89dc = 0;
    g.globals.DAT_005df58c = 1;
    g.globals.DAT_005e89cc = false;
    g.globals.DAT_005e89d0 = 0;
    g.globals.DAT_005e89c0 = 0;
    let b = g.board(this);
    b.ext_0x440 = false;
    b.ext_0x450 = 0;
    b.ext_0x454 = 0;
    b.ext_0x45c = 0;
    b.ext_0x458 = 0;
    b.field_0x330 = match b.field_0x33c {
        2 => 0xf,
        3 => 0x14,
        4 => 0x19,
        _ => 10,
    };
    let lvl = b.field_0x340.clamp(1, 6);
    b.field_0x330 = b.field_0x330 + lvl - 1;
    b.field_0x368 = 0;
    b.field_0x36c = 0;
    b.field_0x220 = -1;
    b.field_0x224 = 0;
    let t = crate::game::board::FUN_00537b60(g, this);
    let b = g.board(this);
    b.field_0x328 = t;
    b.field_0x32c = t;
}

/// port: 00537b80 FUN_00537b80
/// The very first level of a fresh adventure (tank 1, level 1, adventure not finished).
pub fn FUN_00537b80(g: &mut G, this: Ptr) -> bool {
    let app = g.board(this).field_0x0;
    let profile = g.wfa(app).offset_0x18c;
    !(g.board(this).field_0x33c != 1 || g.board(this).field_0x340 != 1 || g.profile(profile).field_0x59)
}

/// port: 0053c320 FUN_0053c320
/// `std::vector<GameObject*>` erase of the first `param_1` (an STL instance).
pub fn FUN_0053c320(this: &mut Vec<Ptr>, param_1: Ptr) {
    if let Some(i) = this.iter().position(|&o| o == param_1) {
        this.remove(i);
    }
}

/// port: 005418e0 FUN_005418e0
/// `std::set<GameObject*>::erase(key)` (an STL instance): the number erased.
pub fn FUN_005418e0(this: &mut std::collections::BTreeSet<Ptr>, param_1: Ptr) -> i32 {
    this.remove(&param_1) as i32
}

/// port: 00541d90 FUN_00541d90
/// `RemoveObject(GameObject*, bool mustBeKnown)`: out of its kind's vector (pet counts
/// down) and the object set; marks +0x2a7 except for kinds 0x1e..0x21. The selected
/// object (+0x110) is never removed; with `param_2`, false unless it was in the set.
pub fn FUN_00541d90(g: &mut G, this: Ptr, param_1: Ptr, param_2: bool) -> bool {
    let ty = g.go(param_1).offset_0x4;
    let mut mark = true;
    let erase = |g: &mut G, off: usize| FUN_0053c320(&mut g.board(this).offset_0xc[vec_index(off)], param_1);
    match ty {
        0 => erase(g, 0xa0),
        5 => erase(g, 0xa4),
        6 => erase(g, 0xd4),
        7 => erase(g, 0xcc),
        8 => erase(g, 0xd0),
        9 => erase(g, 0xc4),
        10 => erase(g, 0xc0),
        0x14 => {
            let id = g.other_pet(param_1).offset_0x70;
            if -1 < id && id < 0x18 {
                g.board(this).field_0xb8[id as usize] -= 1;
            }
            erase(g, 0xb0);
        }
        0x15 => {
            let id = crate::game::fish_type_pet::pet_id(g, param_1);
            if -1 < id && id < 0x18 {
                g.board(this).field_0xb8[id as usize] -= 1;
            }
            erase(g, 0xb4);
        }
        0x16 => {
            if param_1 == g.board(this).field_0x84 {
                return false;
            }
            let off = if g.alien(param_1).offset_0x98 == 0x14 { 0xbc } else { 0xb8 };
            erase(g, off);
        }
        0x17 => erase(g, 0xf4),
        0x19 => {
            if 1 < g.coin(param_1).offset_0x4c || g.coin(param_1).offset_0x40 == 0xf {
                erase(g, 0xec);
            }
            let off = if g.coin(param_1).offset_0x40 == 0x10 { 0xe8 } else { 0xa8 };
            erase(g, off);
        }
        0x1a => erase(g, 0x9c),
        0x1b => erase(g, 0x98),
        0x1c => erase(g, 0xac),
        0x1d => erase(g, 0xc8),
        0x1e => {
            let off = if crate::game::missle::FUN_004d80c0(g, param_1) { 0xe0 } else { 0xdc };
            erase(g, off);
            mark = false;
        }
        0x1f => {
            erase(g, 0xe4);
            mark = false;
        }
        0x20 => {
            erase(g, 0xd8);
            mark = false;
        }
        0x21 => {
            erase(g, 0xf0);
            mark = false;
        }
        0x22 | 0x23 | 0x24 => erase(g, 0xf8),
        _ => {}
    }
    let n = FUN_005418e0(&mut g.board(this).offset_0x7c, param_1);
    if n != 1 && param_2 {
        return false;
    }
    if mark {
        g.board(this).field_0x21b = true;
    }
    true
}

/// port: 00538610 FUN_00538610
/// The death sound (at most every 3 updates), pitched by the kind that died.
pub fn FUN_00538610(g: &mut G, this: Ptr, param_1: i32) {
    if crate::game::board::FUN_005381f0(g, this, 0x117, 3) {
        let pitch = match param_1 {
            5 | 9 => -6,
            6 => -0xc,
            7 | 0x22 | 0x23 | 0x24 => -3,
            8 => 4,
            10 => 5,
            _ => 0,
        };
        // SoundManager::GetSoundInstance(SOUND_DIE); AdjustPitch; Play(false, true).
        let id = g.res.DAT_005e8a70;
        g.sound_requests.push(crate::sexy::sexy_app_base::SoundRequest { id, volume: 1.0, pan: 0, pitch: pitch as f64 });
    }
}

/// port: 005441f0 FUN_005441f0
/// `AddDeadFish(x, y, vx, vy, speed, size, facingRight, shadow)`: the dead fish takes over
/// the shadow.
pub fn FUN_005441f0(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: f64, param_4: f64, param_5: f64, param_6: i32, param_7: bool, param_8: Ptr) {
    let d = crate::game::dead_fish::DeadFish__004eed20(g, param_1 as f64, param_2 as f64, param_3, param_4, param_5, param_6, param_7);
    FUN_00542ee0(g, this, d, 0);
    let mgr = g.wc(this).offset_0xc;
    vcall!(g, mgr, w.vfunction4, d);
    g.go(d).offset_0xc = param_8;
    if param_8 != NULL {
        g.shadow(param_8).offset_0x4 = d;
    }
    FUN_0053aa40(g, this);
}

/// port: 00543920 Sexy::Board::vfunction55_for_Widget
/// `MouseDown(x, y, clicks)`: right clicks (and clicks during a won bonus round) go to
/// `FUN_0053bfb0`; a click during the bonus round ends it. With a weapon selected or aliens
/// around, the laser fires at the click (hitting the first alien under it); otherwise a
/// click in the tank buys and drops a food pellet and starts the hold-to-feed timer.
pub fn vfunction55_for_Widget(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32) {
    let (x, y) = (param_1, param_2);
    crate::sexy::widget::vfunction55(g, this, x, y, param_3);
    let right = |g: &mut G| {
        if 0x28 < y {
            FUN_0053bfb0(g, this, x, y);
        }
    };
    if g.board(this).field_0x219 {
        if param_3 < 0 {
            right(g);
            return;
        }
        if g.board(this).field_0x218 {
            return;
        }
        crate::game::board_level::FUN_00537f40(g, this);
        return;
    }
    if param_3 < 0 {
        right(g);
        return;
    }
    if 0 < g.board(this).field_0x23c {
        let (dx, dy) = (g.board(this).field_0x344 - x, g.board(this).field_0x348 - y);
        if 0x9c4 < dy * dy + dx * dx {
            g.board(this).field_0x23c = 0;
        }
    }
    let mut shot = false;
    let sel = g.board(this).field_0x84;
    if !g.board(this).offset_0xc[vec_index(0xbc)].is_empty() && sel != NULL && 0x28 < y {
        for o in g.board(this).offset_0xc[vec_index(0xbc)].clone() {
            if crate::game::alien::FUN_004fa6f0(g, o, x, y) {
                break;
            }
        }
        shot = true;
        FUN_00543640(g, this, x - 0x28, y - 0x28);
    }
    let sel = g.board(this).field_0x84;
    if sel != NULL {
        crate::game::alien::FUN_004fa6f0(g, sel, x, y);
        shot = true;
        FUN_00543640(g, this, x - 0x28, y - 0x28);
    }
    let aliens = !g.board(this).offset_0xc[vec_index(0xb8)].is_empty() || !g.board(this).offset_0xc[vec_index(0xf4)].is_empty() || {
        let sel = g.board(this).field_0x84;
        sel != NULL && !FUN_004e3ea0(g, this)
    };
    if !aliens {
        let sel = g.board(this).field_0x84;
        if !g.board(this).offset_0xc[vec_index(0xdc)].is_empty() || sel != NULL {
            if 0x28 < y {
                for o in g.board(this).offset_0xc[vec_index(0xdc)].clone() {
                    if crate::game::missle::FUN_004d82a0(g, o, x, y) {
                        break;
                    }
                }
                laser_hold(g, this, x, y);
                return;
            }
        } else if !(g.board(this).field_0x33c == 5 && {
            let app = g.board(this).field_0x0;
            g.wfa(app).offset_0x150 != 5
        })
            && ((x - 0x1f) as u32) <= 0x22b
            && ((y - 0x3d) as u32) <= 0x152
            && g.board(this).field_0x23c <= 0
        {
            let cost = g.board(this).ext_0x4ac;
            if g.board(this).ext_0x440 || crate::game::board_update::FUN_00540b30(g, this, cost, true) {
                FUN_00543280(g, this, x - 10, y - 10, 0, false, 0, -1);
            }
            g.board(this).ext_0x4ec = true;
            let t = crate::game::board::FUN_00537b60(g, this);
            g.board(this).field_0x334 = t;
        }
    } else {
        if !g.board(this).offset_0xc[vec_index(0xf4)].is_empty() && 0x28 < y {
            for o in g.board(this).offset_0xc[vec_index(0xf4)].clone() {
                if crate::game::bilaterus::FUN_004ffae0(g, o, x, y) {
                    break;
                }
            }
            shot = true;
            FUN_00543640(g, this, x - 0x28, y - 0x28);
            g.board(this).ext_0x4ed = true;
            let t = crate::game::board::FUN_00537b60(g, this);
            g.board(this).field_0x338 = t;
        }
        if !g.board(this).offset_0xc[vec_index(0xb8)].is_empty() {
            let first = g.board(this).offset_0xc[vec_index(0xb8)][0];
            let sel = g.board(this).field_0x84 != NULL;
            if g.alien(first).offset_0x98 == 4 && !sel {
                if ((x - 0x1f) as u32) <= 0x22b && ((y - 0x3d) as u32) <= 0x13e {
                    let r = crate::game::alien::FUN_004d46f0(g, first, x, y);
                    FUN_00543280(g, this, x - 10, y - 10, 0, false, if r { 0x14 } else { 0 }, -1);
                    g.board(this).ext_0x4ec = true;
                    let t = crate::game::board::FUN_00537b60(g, this);
                    g.board(this).field_0x334 = t;
                }
            } else if 0x28 < y {
                for o in g.board(this).offset_0xc[vec_index(0xb8)].clone() {
                    if crate::game::alien::FUN_004fa6f0(g, o, x, y) {
                        break;
                    }
                }
                for o in g.board(this).offset_0xc[vec_index(0xdc)].clone() {
                    if crate::game::missle::FUN_004d82a0(g, o, x, y) {
                        break;
                    }
                }
                laser_hold(g, this, x, y);
                return;
            }
        }
    }
    if shot {
        crate::game::board_update::FUN_00538ae0(g, this);
    }
}

/// The shared tail of a laser click: the shot, the hold-to-fire timer and the laser sound.
fn laser_hold(g: &mut G, this: Ptr, x: i32, y: i32) {
    FUN_00543640(g, this, x - 0x28, y - 0x28);
    g.board(this).ext_0x4ed = true;
    let t = crate::game::board::FUN_00537b60(g, this);
    g.board(this).field_0x338 = t;
    crate::game::board_update::FUN_00538ae0(g, this);
}

/// port: 00543640 FUN_00543640
/// `AddShot(x, y)`: a laser shot on top, remembered as the last click (+0x3d0).
pub fn FUN_00543640(g: &mut G, this: Ptr, param_1: i32, param_2: i32) {
    let s = crate::game::shot::Shot__004ec600(g, param_1, param_2);
    FUN_00542ee0(g, this, s, 0);
    let mgr = g.wc(this).offset_0xc;
    vcall!(g, mgr, w.vfunction4, s);
    vcall!(g, mgr, w.vfunction12, s);
    FUN_0053aa40(g, this);
    let b = g.board(this);
    b.field_0x344 = param_1 + 0x28;
    b.field_0x348 = param_2 + 0x28;
}

/// port: 004e3ea0 FUN_004e3ea0
/// The +0xdc vector is not empty.
pub fn FUN_004e3ea0(g: &mut G, this: Ptr) -> bool {
    !g.board(this).offset_0xc[vec_index(0xdc)].is_empty()
}

/// port: 0053c280 FUN_0053c280
/// Refunds a food pellet (not while aliens attack or with free food) and updates the label.
pub fn FUN_0053c280(g: &mut G, this: Ptr) {
    let app = g.board(this).field_0x0;
    if g.wfa(app).offset_0x150 == 5 {
        return;
    }
    if !g.board(this).ext_0x440 && !crate::game::board_update::FUN_004da780(g, this) {
        let b = g.board(this);
        b.field_0x364 += b.ext_0x4ac;
    }
    let b = g.board(this);
    if 9999999 < b.field_0x364 {
        b.field_0x364 = 9999999;
    }
    FUN_0053a360(g, this);
}

/// port: 00543280 FUN_00543280
/// `DropFood(x, y, dropDir, fromTop, settle, grade)`: a pellet of the current grade
/// (`DAT_005e89dc`; 1 when thrown from the side; the free +0x440 pellet is grade 3), at most
/// `DAT_005df58c` plain pellets at once (refunded, with first-levels hints, otherwise).
pub fn FUN_00543280(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32, param_4: bool, param_5: i32, param_6: i32) {
    if param_3 == 0 {
        let foods = g.board(this).offset_0xc[vec_index(0xac)].clone();
        let mut special = 0;
        for &f in &foods {
            let d = g.food(f).clone();
            if d.offset_0x38 != 0 || d.offset_0x40 != 0 {
                special += 1;
            }
        }
        if g.globals.DAT_005df58c + special <= foods.len() as i32 {
            FUN_0053c280(g, this);
            let msg = g.board(this).offset_0x8c;
            if crate::game::board_update::FUN_00537bb0(g, this, 1, 1) && g.message_widget(msg).offset_0x48 <= 0 {
                FUN_0053e950(g, this, b"You can only drop 1 food pellet at a time for now", false, 6);
            } else if crate::game::board_update::FUN_00537bb0(g, this, 1, 2) && g.message_widget(msg).offset_0x48 <= 0 && g.board(this).field_0x318[3] {
                FUN_0053e950(g, this, b"Upgrade Food Quantity to drop more food at once!", false, 0xd);
            }
            FUN_0053aa40(g, this);
            return;
        }
    }
    let f = crate::game::food::Food__004ef840(g, param_1, param_2, param_3, param_4, 0);
    g.food(f).offset_0x34 = g.globals.DAT_005e89dc;
    if param_3 != 0 {
        g.food(f).offset_0x34 = 1;
    }
    if g.board(this).ext_0x440 {
        if param_3 == 0 {
            if param_4 {
                crate::game::board::FUN_00538230(g, this, 0x118, 3, 1.0);
            } else {
                g.food(f).offset_0x34 = 3;
                g.board(this).ext_0x440 = false;
            }
        }
    } else if param_3 == 0 {
        crate::game::board::FUN_00538230(g, this, 0x118, 3, 1.0);
    }
    if param_6 != -1 {
        g.food(f).offset_0x34 = param_6;
    }
    if -1 < param_5 {
        g.food(f).offset_0x44 = param_5;
    }
    FUN_00542ee0(g, this, f, 1);
    let mgr = g.wc(this).offset_0xc;
    vcall!(g, mgr, w.vfunction4, f);
    FUN_0053aa40(g, this);
}

/// port: 005384c0 FUN_005384c0
/// The eating sound (one of three, at most every 3 updates), pitched down 14 semitones
/// for `param_1`.
pub fn FUN_005384c0(g: &mut G, this: Ptr, param_1: bool) {
    if crate::game::board::FUN_005381f0(g, this, 0x138, 3) {
        let app = g.board(this).field_0x0;
        let r = app_rand(g, app) % 5;
        let id = if r == 0 {
            g.res.DAT_005e8de0
        } else if r == 1 {
            g.res.DAT_005e8bdc
        } else {
            g.res.DAT_005e8de4
        };
        let pitch = if param_1 { -14.0 } else { 0.0 };
        g.sound_requests.push(crate::sexy::sexy_app_base::SoundRequest { id, volume: 1.0, pan: 0, pitch });
    }
}

/// port: 00539f30 FUN_00539f30
/// `ClickBonusFood(x, y)`: picks up a clickable (kind 2) food under the cursor.
pub fn FUN_00539f30(g: &mut G, this: Ptr, param_1: i32, param_2: i32) -> bool {
    for f in g.board(this).offset_0xc[vec_index(0xac)].clone() {
        if g.food(f).offset_0x40 == 2 && !g.food(f).offset_0x31 && vcall!(g, f, w.vfunction69, param_1, param_2) {
            crate::game::food::FUN_004d6290(g, f);
            return true;
        }
    }
    false
}

/// port: 00538100 FUN_00538100
/// The tooltip of shop slot `param_1` (slot 11: random pet in mode 1, "end level" in mode 3,
/// else the egg piece).
pub fn FUN_00538100(g: &mut G, this: Ptr, param_1: i32) -> &'static [u8] {
    match param_1 {
        0 => b"buy guppy",
        1 => b"buy breeder",
        2 => b"upgrade food quality",
        3 => b"upgrade food quantity",
        4 => b"buy carnivore",
        5 => b"buy star potion",
        6 => b"buy starcatcher",
        7 => b"buy guppycruncher",
        8 => b"buy beetlemuncher",
        9 => b"buy ultravore",
        10 => b"upgrade weapon",
        0xb => {
            let app = g.board(this).field_0x0;
            match g.wfa(app).offset_0x150 {
                1 => b"buy random pet",
                3 => b"end level",
                _ => b"buy egg piece",
            }
        }
        _ => b"",
    }
}

/// port: 005406a0 FUN_005406a0
/// `SetupShopButton(int slot, bool flash)`: the price and the slot's icon (food grade, food
/// quantity caption, weapon level, egg piece / pet / end level); maxed-out items get "MAX".
/// Flashes the button when asked.
pub fn FUN_005406a0(g: &mut G, this: Ptr, param_1: usize, param_2: bool) {
    use crate::game::menu_button::{FUN_00530410, FUN_00530450, FUN_00534360, FUN_00534450};
    let b = crate::game::board_update::FUN_00538090(g, this, param_1);
    if b == NULL {
        return;
    }
    let price = g.board(this).field_0x288[param_1];
    crate::game::board_update::FUN_00534380(g, b, price);
    // (image, x, y, row, col) for the plain icon cases.
    let icon: Option<(Ptr, i32, i32, i32, i32)> = match param_1 {
        0 => Some((g.res.DAT_005e8ab8, -0xc, -0x12, 0, -1)),
        1 => Some((g.res.DAT_005e8c90, 8, 2, 0, -1)),
        2 => match g.globals.DAT_005e89dc {
            0 => Some((g.res.DAT_005e8aa0, 9, 3, 1, 0)),
            1 => Some((g.res.DAT_005e8aa0, 9, 3, 2, 0)),
            _ => {
                FUN_00534450(g, b);
                None
            }
        },
        3 => {
            let n = g.globals.DAT_005df58c;
            if n < 9 {
                let s = format!("{}", (n + 1).min(9)).into_bytes();
                FUN_00534360(g, b, &s);
            } else {
                FUN_00534450(g, b);
            }
            None
        }
        4 => Some((g.res.DAT_005e8b80, 8, 3, 0, -1)),
        5 => Some((g.res.DAT_005e8aa0, 10, 1, 3, 0)),
        6 => Some((g.res.DAT_005e8ac0, 8, 2, 0, -1)),
        7 => Some((g.res.DAT_005e8bac, 8, 1, 0, -1)),
        8 => Some((g.res.DAT_005e8ce0, 10, 2, 0, -1)),
        9 => Some((g.res.DAT_005e8dc8, 10, 2, 0, -1)),
        10 => {
            let lvl = g.board(this).field_0x358;
            let img = g.res.DAT_005e8a10;
            FUN_00530410(g, b, img, 7, 3, 0, lvl - 2);
            if 0xb < g.board(this).field_0x358 {
                if param_2 {
                    FUN_0053e950(g, this, b"Hold down mouse button for Rapid Fire!", false, -1);
                }
                FUN_00534450(g, b);
            }
            None
        }
        0xb => {
            let app = g.board(this).field_0x0;
            let mode = g.wfa(app).offset_0x150;
            let img = g.res.DAT_005e8c00;
            if mode == 1 {
                FUN_00530410(g, b, img, 7, 3, 0, 2);
                FUN_00534360(g, b, b"?");
                let profile = g.wfa(app).offset_0x18c;
                let mut i = 0;
                while i < 0x18 {
                    if g.board(this).field_0xb8[i] == 0 && i != 0x13 && crate::game::profile::FUN_00501410(g, profile, i) {
                        break;
                    }
                    i += 1;
                }
                if i == 0x18 {
                    FUN_00534450(g, b);
                }
                None
            } else if mode == 3 {
                Some((img, 7, mode, 0, 2))
            } else {
                let n = g.board(this).ext_0x43c;
                if 0 < n && n <= 3 { Some((img, 7, 3, 0, n - 1)) } else { None }
            }
        }
        _ => None,
    };
    if let Some((img, x, y, row, col)) = icon {
        FUN_00530410(g, b, img, x, y, row, col);
    }
    if param_2 {
        FUN_00530450(g, b);
    }
}

/// port: 005460f0 FUN_005460f0
/// A bonus-round shell shower: one or two shells across the top (kind 2, 4, 6 or 7 by a
/// 20/30/20/10% draw, else 1) at speed 1.0..3.7, then the next shower in 500 updates.
pub fn FUN_005460f0(g: &mut G, this: Ptr) {
    let app = g.board(this).field_0x0;
    let mut n = (app_rand(g, app) & 1) + 1;
    const CHANCE: [i32; 5] = [0x14, 0x1e, 0x14, 10, 0];
    while n != 0 {
        let ry = app_rand(g, app);
        let rx = app_rand(g, app);
        let r = crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32;
        let mut k = 0;
        let mut left = 100;
        while k < 4 {
            left -= CHANCE[k];
            if left <= r % 100 {
                break;
            }
            k += 1;
        }
        let kind = match k {
            0 => 2,
            1 => 4,
            2 => 6,
            3 => 7,
            _ => 1,
        };
        let s = (crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32) % 10;
        let speed = (s as f64 / 10.0) * 3.0 + 1.0;
        FUN_00544430(g, this, (rx % 0x208) as i32 + 0x14, (ry % 10) as i32 + 0x32, kind, NULL, speed, 0);
        n -= 1;
    }
    g.board(this).ext_0x454 = 500;
}

/// port: 00545e20 FUN_00545e20
/// Bonus round (+0x2a5): a collected shell continues the chain of its kind (another kind
/// starts a new one): its value is multiplied by the chain length, shown as "+N", with the
/// kind's sound pitched up a semitone per link; the tenth link pays 25 times ("+N (MAX
/// CHAIN!!)"), plays five high notes and resets the chain. False (no chain) outside the
/// bonus round or for other kinds.
pub fn FUN_00545e20(g: &mut G, this: Ptr, param_1: Ptr) -> bool {
    use crate::game::timed_messages::{FUN_00511770, FUN_00511830, FUN_00515e30, Melody, Note};
    if !g.board(this).field_0x219 {
        return false;
    }
    let kind = g.coin(param_1).offset_0x40;
    if kind != g.board(this).field_0x220 {
        let b = g.board(this);
        b.field_0x220 = kind;
        b.field_0x224 = 0;
    }
    let (snd, mul, style) = match kind {
        1 => (g.res.DAT_005e8d74, 1, 0),
        2 => (g.res.DAT_005e8d74, 2, 1),
        4 => (g.res.DAT_005e8d08, 5, 2),
        6 => (g.res.DAT_005e8e54, 10, 3),
        7 => (g.res.DAT_005e8e24, 0x14, 4),
        _ => return false,
    };
    g.board(this).field_0x224 += 1;
    let n = g.board(this).field_0x224;
    let (x, y) = (g.wc(param_1).offset_0x2c, g.wc(param_1).offset_0x30);
    if n < 10 {
        g.coin(param_1).offset_0x6c = n;
        let text = format!("+{}", n * mul).into_bytes();
        FUN_005444e0(g, this, x, y, style + 3, &text);
        // mSoundManager->GetSoundInstance(snd); SetVolume(1.0); AdjustPitch(n - 1); Play(false, true).
        g.sound_requests.push(crate::sexy::sexy_app_base::SoundRequest { id: snd, volume: 1.0, pan: 0, pitch: (n - 1) as f64 });
    } else {
        g.coin(param_1).offset_0x6c = 0x19;
        let text = format!("+{} (MAX CHAIN!!)", mul * 0x19).into_bytes();
        FUN_005444e0(g, this, x, y, style + 3, &text);
        let mut m = Melody::default();
        FUN_00511770(&mut m);
        m.field_0x1c = snd;
        for _ in 0..5 {
            FUN_00511830(&mut m.field_0x4, Note { field_0x0: 9.0, field_0x8: 1.0, field_0x10: 200 });
        }
        let tm = g.board(this).offset_0x4;
        FUN_00515e30(g, tm, m);
        let b = g.board(this);
        b.field_0x224 = 1;
        b.field_0x220 = -1;
    }
    true
}

/// port: 00544430 FUN_00544430
/// `AddCoin(x, y, kind, from, speed, updateCnt)`: a coin on the board and the widget manager.
pub fn FUN_00544430(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32, param_4: Ptr, param_5: f64, param_6: i32) {
    let c = crate::game::coin::Coin__004eead0(g, param_1, param_2, param_3, param_4, param_5);
    g.wc(c).offset_0x24 = param_6;
    FUN_00542ee0(g, this, c, 0);
    let mgr = g.wc(this).offset_0xc;
    vcall!(g, mgr, w.vfunction4, c);
    FUN_0053aa40(g, this);
}

/// port: 005444e0 FUN_005444e0
/// `AddText(x, y, style, text)`: a floating text (coin kind 0x10).
pub fn FUN_005444e0(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32, param_4: &[u8]) {
    let c = crate::game::coin::Coin__004eead0(g, param_1, param_2, 0x10, NULL, -1.0);
    let d = g.coin(c);
    d.offset_0x4c = param_3;
    d.field_0x50 = param_4.to_vec();
    FUN_00542ee0(g, this, c, 0);
    let mgr = g.wc(this).offset_0xc;
    vcall!(g, mgr, w.vfunction4, c);
    FUN_0053aa40(g, this);
}

/// port: 005436f0 FUN_005436f0
/// `AddSparkle(x, y, kind)`: a `Shot` effect on top, while fewer than 20 are around.
pub fn FUN_005436f0(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32) {
    if g.board(this).offset_0xc[vec_index(0xd8)].len() < 0x14 {
        let s = crate::game::shot::Shot__004ec660(g, param_1, param_2, param_3);
        FUN_00542ee0(g, this, s, 0);
        let mgr = g.wc(this).offset_0xc;
        vcall!(g, mgr, w.vfunction4, s);
        vcall!(g, mgr, w.vfunction12, s);
        FUN_0053aa40(g, this);
    }
}

/// port: 00538cd0 FUN_00538cd0
/// Where collected items fly in the virtual tank: (0x104, 0x109).
pub fn FUN_00538cd0(g: &mut G, this: Ptr) -> (i32, i32) {
    let _ = (g, this);
    (0x104, 0x109)
}

/// port: 00538470 FUN_00538470
/// The treasure sound (0x116).
pub fn FUN_00538470(g: &mut G, this: Ptr) {
    crate::game::board::FUN_00538230(g, this, 0x116, 3, 1.0);
}

/// port: 00538560 FUN_00538560
/// The beetle sound (0x113 or, one time in three, 0x114; 0x138 lowered 18 semitones when
/// `param_1`), at most every 3 updates.
pub fn FUN_00538560(g: &mut G, this: Ptr, param_1: bool) {
    if crate::game::board::FUN_005381f0(g, this, 0x113, 3) {
        let app = g.board(this).field_0x0;
        let rng = g.wfa(app).offset_0x84;
        let r = crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng)) % 3;
        let snd = if param_1 { 0x138 } else if r != 0 { 0x113 } else { 0x114 };
        let id = crate::sexy::res::FUN_005016a0(g, snd);
        // SoundManager::GetSoundInstance(id); AdjustPitch(-18) when param_1; Play(false, true).
        let pitch = if param_1 { -18.0 } else { 0.0 };
        g.sound_requests.push(crate::sexy::sexy_app_base::SoundRequest { id, volume: 1.0, pan: 0, pitch });
    }
}

/// port: 00546fc0 FUN_00546fc0
/// `AddGuppy(x, y, size, facing)`: a guppy of the given size (a revived one), counted in the
/// most-guppies statistic, with its shadow.
pub fn FUN_00546fc0(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32, param_4: bool) -> Ptr {
    let f = crate::game::fish::Fish__004eef90(g, param_1, param_2, param_3, param_4);
    FUN_00542ee0(g, this, f, 1);
    let mgr = g.wc(this).offset_0xc;
    vcall!(g, mgr, w.vfunction4, f);
    let n = g.board(this).offset_0xc[vec_index(0xa0)].len() as i32;
    let b = g.board(this);
    if b.ext_0x460[(0x48c - 0x460) / 4] < n {
        b.ext_0x460[(0x48c - 0x460) / 4] = n;
    }
    FUN_00544800(g, this, f);
    FUN_0053aa40(g, this);
    f
}

/// port: 00549380 FUN_00549380
/// `ShopButtonClicked(int slot)`: pays the slot's price and delivers: a guppy (with a hint
/// when broke on the first level), a breeder, the next food grade (up to 2; on 1-2 also
/// opens quantity and the egg), one more pellet at once (up to 9), a carnivore, the star
/// potion (dropped by the next click), starcatcher, guppycruncher, beetlemuncher,
/// ultravore, the next weapon level (up to 12), or the egg piece / pet / level end.
pub fn FUN_00549380(g: &mut G, this: Ptr, param_1: usize) {
    use crate::game::board_update::FUN_00540b30;
    if crate::game::board_update::FUN_00538090(g, this, param_1) == NULL || 0xb < param_1 {
        return;
    }
    let price = g.board(this).field_0x288[param_1];
    let buy_sound = |g: &mut G| {
        crate::game::board::FUN_00538230(g, this, 0x122, 3, 1.0);
        FUN_00538360(g, this);
    };
    let upgrade_sound = |g: &mut G| crate::game::board::FUN_00538230(g, this, 0x111, 3, 1.0);
    match param_1 {
        0 => {
            if crate::game::board_update::FUN_00537bb0(g, this, 1, 1) && g.board(this).field_0x364 < price {
                FUN_0053e950(g, this, b"You can't afford new fish yet! Collect more money!", false, 0xe);
            }
            if !FUN_00540b30(g, this, price, true) {
                return;
            }
            FUN_00546d70(g, this);
            if g.board(this).ext_0x4b0[2] != 0 {
                g.board(this).ext_0x4b0[2] = 0;
            }
            buy_sound(g);
        }
        1 => {
            if !FUN_00540b30(g, this, price, true) {
                return;
            }
            FUN_005470a0(g, this);
            buy_sound(g);
        }
        2 => {
            if 2 <= g.globals.DAT_005e89dc || !FUN_00540b30(g, this, price, true) {
                return;
            }
            if g.board(this).ext_0x4b0[0xf] != 0 {
                g.board(this).ext_0x4b0[0xf] = 0;
            }
            g.globals.DAT_005e89dc += 1;
            FUN_005406a0(g, this, 2, true);
            let b = g.board(this);
            if !b.field_0x318[3] && b.field_0x33c == 1 && b.field_0x340 == 2 {
                FUN_005409b0(g, this, 3, true);
                FUN_005409b0(g, this, 0xb, true);
                let app = g.board(this).field_0x0;
                let profile = g.wfa(app).offset_0x18c;
                if !g.profile(profile).field_0x59 {
                    FUN_0053e950(g, this, b"Upgrade Food Quantity to drop more food at once!", false, 0xc);
                }
            }
            upgrade_sound(g);
        }
        3 => {
            if 9 <= g.globals.DAT_005df58c || !FUN_00540b30(g, this, price, true) {
                return;
            }
            let app = g.board(this).field_0x0;
            let profile = g.wfa(app).offset_0x18c;
            if g.board(this).field_0x33c == 1 && g.globals.DAT_005df58c == 1 && !g.profile(profile).field_0x59 {
                FUN_0053e950(g, this, b"Hold down mouse button to Auto-Feed!", false, -1);
            }
            g.globals.DAT_005df58c += 1;
            FUN_005406a0(g, this, 3, true);
            upgrade_sound(g);
        }
        4 => {
            if !FUN_00540b30(g, this, price, true) {
                return;
            }
            FUN_00544c00(g, this);
            let b = g.board(this);
            if b.field_0x33c == 4 {
                if 1 < b.field_0x340 {
                    FUN_005409b0(g, this, 9, true);
                    FUN_005409b0(g, this, 10, true);
                }
            } else {
                FUN_005409b0(g, this, 10, true);
            }
            FUN_005409b0(g, this, 0xb, true);
            buy_sound(g);
        }
        5 => {
            if g.board(this).ext_0x440 || !FUN_00540b30(g, this, price, true) {
                return;
            }
            if !g.board(this).ext_0x441 {
                g.board(this).ext_0x441 = true;
                if g.board(this).field_0x340 < 3 {
                    FUN_0053e950(g, this, b"Click on tank to drop star potion!", false, -1);
                    let msg = g.board(this).offset_0x8c;
                    g.message_widget(msg).offset_0x48 = 0xb4;
                }
            }
            g.board(this).ext_0x440 = true;
            upgrade_sound(g);
        }
        6 => {
            if !FUN_00540b30(g, this, price, true) {
                return;
            }
            FUN_00545260(g, this);
            FUN_005409b0(g, this, 10, true);
            FUN_005409b0(g, this, 0xb, true);
            buy_sound(g);
        }
        7 => {
            if !FUN_00540b30(g, this, price, true) {
                return;
            }
            FUN_00545430(g, this);
            buy_sound(g);
            let slot = if g.board(this).field_0x340 != 1 { 8 } else { 0xb };
            FUN_005409b0(g, this, slot, true);
        }
        8 => {
            if !FUN_00540b30(g, this, price, true) {
                return;
            }
            g.board(this).ext_0x460[(0x4a4 - 0x460) / 4] += 1;
            FUN_00545080(g, this);
            FUN_005409b0(g, this, 10, true);
            FUN_005409b0(g, this, 0xb, true);
            buy_sound(g);
        }
        9 => {
            if !FUN_00540b30(g, this, price, true) {
                return;
            }
            FUN_00544e60(g, this);
            FUN_005409b0(g, this, 10, true);
            FUN_005409b0(g, this, 0xb, true);
            crate::game::board::FUN_00538230(g, this, 0x122, 3, 1.0);
            crate::game::board::FUN_00538230(g, this, 0x140, 3, 1.0);
        }
        10 => {
            if 0xc <= g.board(this).field_0x358 || !FUN_00540b30(g, this, price, true) {
                return;
            }
            g.board(this).field_0x358 += 1;
            FUN_005406a0(g, this, 10, true);
            upgrade_sound(g);
        }
        _ => FUN_00545960(g, this),
    }
}

/// port: 00546d70 FUN_00546d70
/// `BuyGuppy()`: a small guppy dropped in at a random x (20..539) from y 30, sinking at
/// 18..22 and slowing over 45..54 updates.
pub fn FUN_00546d70(g: &mut G, this: Ptr) -> Ptr {
    let app = g.board(this).field_0x0;
    let x = app_rand(g, app) % 0x208 + 0x14;
    let y = app_rand(g, app) % 0x109 + 0x69;
    let f = crate::game::fish::Fish__004eef00(g, x as i32, y as i32);
    let vy = app_rand(g, app) % 5 + 0x12;
    g.fish(f).field_0x1c = vy as f64;
    let damp = app_rand(g, app) % 10 + 0x2d;
    g.fish(f).field_0xc = 30.0;
    g.wc(f).offset_0x30 = 0x1e;
    g.fish(f).field_0xb4 = damp as i32;
    FUN_00542ee0(g, this, f, 1);
    let mgr = g.wc(this).offset_0xc;
    vcall!(g, mgr, w.vfunction4, f);
    let n = g.board(this).offset_0xc[vec_index(0xa0)].len() as i32;
    let b = g.board(this);
    if b.ext_0x460[(0x48c - 0x460) / 4] < n {
        b.ext_0x460[(0x48c - 0x460) / 4] = n;
    }
    FUN_00544800(g, this, f);
    FUN_0053aa40(g, this);
    f
}

/// port: 00547310 FUN_00547310
/// The debug '3' key: a king guppy (size 3) dropped in as `BuyGuppy` drops a small one.
pub fn FUN_00547310(g: &mut G, this: Ptr) {
    let app = g.board(this).field_0x0;
    let x = app_rand(g, app) % 0x208 + 0x14;
    let y = app_rand(g, app) % 0x109 + 0x69;
    let f = crate::game::fish::Fish__004eef00(g, x as i32, y as i32);
    let vy = app_rand(g, app) % 5 + 0x12;
    g.fish(f).field_0x1c = vy as f64;
    let damp = app_rand(g, app) % 10 + 0x2d;
    g.fish(f).field_0xc = 30.0;
    g.wc(f).offset_0x30 = 0x1e;
    g.fish(f).field_0xb4 = damp as i32;
    FUN_00542ee0(g, this, f, 1);
    let mgr = g.wc(this).offset_0xc;
    vcall!(g, mgr, w.vfunction4, f);
    let n = g.board(this).offset_0xc[vec_index(0xa0)].len() as i32;
    let b = g.board(this);
    if b.ext_0x460[(0x48c - 0x460) / 4] < n {
        b.ext_0x460[(0x48c - 0x460) / 4] = n;
    }
    g.fish(f).offset_0x4c = 3;
    FUN_00544800(g, this, f);
    FUN_0053aa40(g, this);
}

/// port: 00538360 FUN_00538360
/// One of three splash sounds for a new creature.
pub fn FUN_00538360(g: &mut G, this: Ptr) {
    let app = g.board(this).field_0x0;
    let r = app_rand(g, app) % 3;
    let id = match r {
        0 => g.res.DAT_005e8e64,
        1 => g.res.DAT_005e8d2c,
        _ => g.res.DAT_005e8cd4,
    };
    crate::sexy::sexy_app_base::vfunction55(g, app, id);
}

/// port: 005380c0 FUN_005380c0
/// `SetPrice(slot, price)` (capped at 99999), shown on the slot's button.
pub fn FUN_005380c0(g: &mut G, this: Ptr, param_1: usize, param_2: i32) {
    let p = param_2.min(99999);
    g.board(this).field_0x288[param_1] = p;
    let b = crate::game::board_update::FUN_00538090(g, this, param_1);
    if b != NULL {
        crate::game::board_update::FUN_00534380(g, b, p);
    }
}

/// port: 00545960 FUN_00545960
/// `BuyEggPiece()` (shop slot 11): ends the level in mode 3; in mode 1 a random pet the
/// player has earned but not got (doubling the price; "MAX" once none is left); otherwise an
/// egg piece (time trial resets the prices). The fourth piece finishes the level: time and
/// statistics, the high score, then the next level's progress, its pet (the last level of
/// tank 4 gives 999, tank 5 the 5000 bonus) and the interlude.
pub fn FUN_00545960(g: &mut G, this: Ptr) {
    let b = crate::game::board_update::FUN_00538090(g, this, 0xb);
    if b == NULL {
        return;
    }
    let app = g.board(this).field_0x0;
    if g.wfa(app).offset_0x150 == 3 {
        crate::game::win_fish_app::FUN_00552230(g, app);
        return;
    }
    let price = g.board(this).field_0x288[11];
    if !crate::game::board_update::FUN_00540b30(g, this, price, true) {
        return;
    }
    if g.wfa(app).offset_0x150 == 1 {
        let profile = g.wfa(app).offset_0x18c;
        let mut pets: Vec<i32> = Vec::new();
        for i in 0..0x18 {
            if g.board(this).field_0xb8[i] == 0 && i != 0x13 && crate::game::profile::FUN_00501410(g, profile, i) {
                pets.push(i as i32);
            }
        }
        if !pets.is_empty() {
            let r = crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32 % pets.len() as i32;
            let pet = pets[r as usize];
            crate::game::board_level::FUN_00544a90(g, this, pet, -1, -1, false, false);
            g.board(this).ext_0x460[(0x498 - 0x460) / 4] += 1;
            let np = g.board(this).field_0x288[11] * 2;
            FUN_005380c0(g, this, 0xb, np);
        }
        if pets.len() <= 1 {
            crate::game::menu_button::FUN_00534450(g, b);
        }
        crate::game::board::FUN_00538230(g, this, 0x111, 3, 1.0);
        return;
    }
    if g.board(this).ext_0x4b0[0x28] != 0 {
        let bd = g.board(this);
        bd.ext_0x4b0[0x28] = 0;
        bd.ext_0x4e6 = true;
        bd.ext_0x4e8 = 0;
    }
    if crate::game::board_update::FUN_00537bb0(g, this, 1, 1) {
        let msg = g.board(this).offset_0x8c;
        if g.message_widget(msg).offset_0x48 <= 0 && g.board(this).ext_0x43c == 1 {
            FUN_0053e950(g, this, b"Collect 2 more egg pieces to finish level!", false, 10);
        }
    }
    g.board(this).ext_0x43c += 1;
    if g.wfa(app).offset_0x150 == 4 {
        g.board(this).ext_0x454 = 500;
        for i in 0..0xc {
            if i != 0xb {
                let base = g.board(this).field_0x2b8[i];
                FUN_005380c0(g, this, i, base);
            }
        }
        if g.board(this).ext_0x43c <= 3 {
            FUN_0053e950(g, this, b"Prices have been reset!", false, -1);
        }
    }
    FUN_005406a0(g, this, 0xb, true);
    crate::game::board::FUN_00538230(g, this, 0x111, 3, 1.0);
    if g.board(this).ext_0x43c <= 3 {
        return;
    }
    // The level is finished.
    let t = crate::game::board::FUN_00538040(g, this);
    g.board(this).field_0x370 = t;
    g.board(this).ext_0x4ee = false;
    crate::game::board_update::FUN_0053c2d0(g, this);
    let profile = g.wfa(app).offset_0x18c;
    let hsm = g.wfa(app).offset_0x180;
    let tank = g.board(this).field_0x33c;
    if g.wfa(app).offset_0x150 == 4 {
        if (1..=4).contains(&tank) {
            g.profile(profile).field_0x78[(tank - 1) as usize] = true;
        }
        let t = g.board(this).field_0x370;
        crate::game::high_score::FUN_005144c0(g, hsm, tank, profile, t);
        crate::game::bonus_screen::FUN_0054bf60(g, app);
        return;
    }
    let done = g.profile(profile).field_0x59;
    let mut score = g.board(this).field_0x370;
    if tank == 5 {
        g.globals.DAT_005e9080 = 0;
        if g.globals.DAT_005e8fe0.len() < 0x28 {
            g.globals.DAT_005e8fe0.resize(0x28, 0);
        }
        for i in 0..0x12 {
            if g.board(this).field_0xb8[i] == 0 && i != 0x12 {
                let n = g.globals.DAT_005e9080 as usize;
                g.globals.DAT_005e8fe0[n] = i as i32;
                g.globals.DAT_005e9080 += 1;
            }
        }
        score = (g.board(this).offset_0xc[vec_index(0xb4)].len() + g.board(this).offset_0xc[vec_index(0xb0)].len()) as i32;
    }
    let level = g.board(this).field_0x340;
    crate::game::high_score::FUN_00514510(g, hsm, tank, level, profile, score);
    if done {
        if tank == 5 && level == 1 {
            crate::game::profile::FUN_00501370(g.profile(profile));
            crate::game::bonus_screen::FUN_0054bf60(g, app);
            return;
        }
        FUN_0053dc10(g, this);
        return;
    }
    crate::game::profile::FUN_00501370(g.profile(profile));
    let pet = if tank == 4 && level == 5 {
        999
    } else {
        let pet = if tank == 5 { 0x13 } else { tank * 5 + level - 6 };
        if (0..0x18).contains(&pet) {
            crate::game::profile::FUN_00501420(g.profile(profile), pet as usize, true);
        }
        if pet == 0x13 {
            crate::game::profile::FUN_00501200(g, profile, 5000);
        }
        pet
    };
    crate::game::win_fish_app::FUN_0054be80(g, app, pet);
}

/// port: 00545620 FUN_00545620
/// `AddAlien(kind, x, y, announce)`: the combined waves (9..0xc) add two aliens, the second
/// at the second spot (+0x2d4); 8 is Bilaterus; the boss (0x15) is remembered (+0x110, a
/// game in progress, weapon level 10); the others arrive through a warp with a shadow.
/// Remembers the time (virtual tank), updates the music state, and plays the arrival cry.
pub fn FUN_00545620(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32, param_4: bool) {
    let app = g.board(this).field_0x0;
    g.globals.DAT_005e8f18 = g.sab(app).field_0x47c - 0xd8;
    let (x2, y2) = (g.board(this).field_0x248, g.board(this).field_0x24c);
    match param_1 {
        9 => {
            FUN_00545620(g, this, 1, param_2, param_3, param_4);
            FUN_00545620(g, this, 3, x2, y2, param_4);
        }
        10 => {
            FUN_00545620(g, this, 7, param_2, param_3, param_4);
            FUN_00545620(g, this, 3, x2, y2, param_4);
        }
        0xb => {
            FUN_00545620(g, this, 5, param_2, param_3, param_4);
            FUN_00545620(g, this, 6, x2, y2, param_4);
        }
        0xc => {
            FUN_00545620(g, this, 3, param_2, param_3, param_4);
            FUN_00545620(g, this, 8, x2, y2, param_4);
        }
        8 => {
            let b = crate::game::bilaterus::Bilaterus__004fe000(g, param_2, param_3);
            crate::game::bilaterus::FUN_004edc70(g, b);
            let mgr = g.wc(this).offset_0xc;
            vcall!(g, mgr, w.vfunction4, b);
            FUN_00542ee0(g, this, b, 1);
        }
        0x15 => {
            g.board(this).ext_0x4ee = true;
            let a = crate::game::alien::Alien__004ecfa0(g, param_2, param_3, 0x15);
            crate::game::alien::FUN_004ed370(g, a);
            g.board(this).field_0x84 = a;
            let mgr = g.wc(this).offset_0xc;
            vcall!(g, mgr, w.vfunction4, a);
            FUN_00544800(g, this, a);
            g.board(this).field_0x358 = 10;
        }
        _ => {
            let a = crate::game::alien::Alien__004ecfa0(g, param_2, param_3, param_1);
            crate::game::alien::FUN_004ed370(g, a);
            FUN_00542ee0(g, this, a, 1);
            let mgr = g.wc(this).offset_0xc;
            vcall!(g, mgr, w.vfunction4, a);
            FUN_00544800(g, this, a);
        }
    }
    crate::game::board_update::FUN_00539ad0(g, this, false);
    FUN_0053aa40(g, this);
    if param_4 {
        let s = match param_1 {
            4 => g.res.DAT_005e8e98,
            5 => g.res.DAT_005e8bc8,
            6 => g.res.DAT_005e8b6c,
            7 => g.res.DAT_005e89fc,
            0x15 => g.res.DAT_005e8c9c,
            1 | 2 | 3 => g.res.DAT_005e8c6c,
            _ => return,
        };
        crate::sexy::sexy_app_base::vfunction55(g, app, s);
    }
}

/// port: 005442c0 FUN_005442c0
/// `AddDeadAlien(x, y, cel, kind, facingRight)`.
pub fn FUN_005442c0(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: bool) {
    let d = crate::game::dead_alien::DeadAlien__004eec10(g, param_1 as f64, param_2 as f64, param_3, param_4, param_5);
    FUN_00542ee0(g, this, d, 0);
    let mgr = g.wc(this).offset_0xc;
    vcall!(g, mgr, w.vfunction4, d);
    FUN_0053aa40(g, this);
}

/// port: 00537f90 FUN_00537f90
/// Clears the message line when it shows message `param_1`.
pub fn FUN_00537f90(g: &mut G, this: Ptr, param_1: i32) {
    let m = g.board(this).offset_0x8c;
    if m != NULL && g.message_widget(m).offset_0x50 == param_1 {
        g.message_widget(m).offset_0x48 = 0;
    }
}

/// port: 00544a90 FUN_00544a90
/// `AddPet(pet, x, y, presto, keep)`: Presto (0x13) is always Presto; x -1 picks a random
/// spot; the crawlers and Niko/Clyde are `OtherTypePet`s, the rest `FishTypePet`s; virtual
/// tank pets (unless `keep`) remember their kind (+0xac = 1000 + pet, 1019 for Presto).
pub fn FUN_00544a90(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32, param_4: bool, param_5: bool) -> Ptr {
    let app = g.board(this).field_0x0;
    let presto = param_4 || param_1 == 0x13;
    let (mut a, mut b) = (param_2, param_3);
    if param_2 == -1 {
        a = (app_rand(g, app) % 0x109) as i32 + 0x69;
        b = (app_rand(g, app) % 0x208) as i32 + 0x14;
    }
    let p = if matches!(param_1, 0 | 1 | 7 | 5 | 0xe) {
        let backdrop = g.board(this).field_0x35c;
        crate::game::other_pet::OtherTypePet__004eb5c0(g, a, b, param_1, backdrop, presto)
    } else {
        crate::game::fish_type_pet::FishTypePet__004ef420(g, a, b, param_1, presto)
    };
    if g.wfa(app).offset_0x150 == 5 && !param_5 {
        g.go(p).offset_0x24 = if presto { 0x13 } else { param_1 } + 1000;
    }
    FUN_00542ee0(g, this, p, 1);
    let mgr = g.wc(this).offset_0xc;
    vcall!(g, mgr, w.vfunction4, p);
    FUN_00544800(g, this, p);
    FUN_0053aa40(g, this);
    p
}

/// port: 00538340 FUN_00538340
/// The pinch sound (0x130), at most every `param_1` updates.
pub fn FUN_00538340(g: &mut G, this: Ptr, param_1: i32) {
    crate::game::board::FUN_00538230(g, this, 0x130, param_1, 1.0);
}

/// port: 005382c0 FUN_005382c0
/// The birth / drop sound: 0x121 with profile flag 6, else 0x136 (`param_1`, unless flag
/// 5) or 0x108 (unless flag 1), else 0x120.
pub fn FUN_005382c0(g: &mut G, this: Ptr, param_1: bool) {
    let app = g.board(this).field_0x0;
    let profile = g.wfa(app).offset_0x18c;
    let flags = g.profile(profile).field_0x84;
    if (flags >> 6) & 1 != 0 {
        crate::game::board::FUN_00538230(g, this, 0x121, 3, 1.0);
        return;
    }
    if !param_1 {
        if (flags >> 1) & 1 == 0 {
            crate::game::board::FUN_00538230(g, this, 0x108, 3, 1.0);
            return;
        }
    } else if (flags >> 5) & 1 == 0 {
        crate::game::board::FUN_00538230(g, this, 0x136, 3, 1.0);
        return;
    }
    crate::game::board::FUN_00538230(g, this, 0x120, 3, 1.0);
}

/// port: 005434e0 FUN_005434e0
/// `AddFood(x, y)`: a pellet of grade 2 dropped in place (Nostradamus), on the board and the
/// widget manager.
pub fn FUN_005434e0(g: &mut G, this: Ptr, param_1: i32, param_2: i32) {
    let f = crate::game::food::Food__004ef840(g, param_1, param_2, 0, false, 2);
    FUN_00542ee0(g, this, f, 1);
    let mgr = g.wc(this).offset_0xc;
    vcall!(g, mgr, w.vfunction4, f);
    FUN_0053aa40(g, this);
}

/// The tail both carnivore adders share: on the board and the widget manager, the most
/// carnivores statistic (+0x46c from the +0xa4 list), the shadow, re-stacked.
fn add_carnivore_tail(g: &mut G, this: Ptr, o: Ptr) {
    FUN_00542ee0(g, this, o, 1);
    let mgr = g.wc(this).offset_0xc;
    vcall!(g, mgr, w.vfunction4, o);
    let n = g.board(this).offset_0xc[vec_index(0xa4)].len() as i32;
    let b = g.board(this);
    if b.ext_0x460[(0x46c - 0x460) / 4] < n {
        b.ext_0x460[(0x46c - 0x460) / 4] = n;
    }
    FUN_00544800(g, this, o);
    FUN_0053aa40(g, this);
}

/// port: 00544c00 FUN_00544c00
/// `BuyCarnivore()`: an Oscar dropped in from the top (y 40, falling at 23..27) at a random
/// spot, with 45..54 updates of slowing.
pub fn FUN_00544c00(g: &mut G, this: Ptr) -> Ptr {
    let app = g.board(this).field_0x0;
    let x = (app_rand(g, app) % 0x208) as i32 + 0x14;
    let y = (app_rand(g, app) % 0x109) as i32 + 0x69;
    let o = crate::game::oscar::Oscar__004efda0(g, x, y);
    let r = app_rand(g, app);
    g.fish(o).field_0x1c = (r % 5 + 0x17) as f64;
    let r = app_rand(g, app);
    g.fish(o).field_0xc = 40.0;
    g.wc(o).offset_0x30 = 0x28;
    g.fish(o).field_0xb4 = (r % 10) as i32 + 0x2d;
    add_carnivore_tail(g, this, o);
    o
}

/// port: 00544d80 FUN_00544d80
/// `AddCarnivore(x, y, facingRight)` (a revived one).
pub fn FUN_00544d80(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: bool) {
    let o = crate::game::oscar::Oscar__004efe00(g, param_1, param_2, param_3);
    add_carnivore_tail(g, this, o);
}

/// port: 00545260 FUN_00545260
/// `BuyStarcatcher()`: a Penta dropped in at a random x (y 65), on the board and the widget
/// manager, the most starcatchers statistic (+0x494 from the +0xd0 list), the shadow.
pub fn FUN_00545260(g: &mut G, this: Ptr) {
    let app = g.board(this).field_0x0;
    let x = (app_rand(g, app) % 0x208) as i32 + 0x14;
    let o = crate::game::penta::Penta__004ec080(g, x);
    g.penta(o).offset_0xc = 65.0;
    g.wc(o).offset_0x30 = 0x41;
    FUN_00542ee0(g, this, o, 1);
    let mgr = g.wc(this).offset_0xc;
    vcall!(g, mgr, w.vfunction4, o);
    let n = g.board(this).offset_0xc[vec_index(0xd0)].len() as i32;
    let b = g.board(this);
    if b.ext_0x460[(0x494 - 0x460) / 4] < n {
        b.ext_0x460[(0x494 - 0x460) / 4] = n;
    }
    FUN_00544800(g, this, o);
    FUN_0053aa40(g, this);
}

/// port: 00545430 FUN_00545430
/// `BuyGuppycruncher()`: a Grubber dropped in at a random x (y 65), on the board and the
/// widget manager, its shadow.
pub fn FUN_00545430(g: &mut G, this: Ptr) {
    let app = g.board(this).field_0x0;
    let x = (app_rand(g, app) % 0x208) as i32 + 0x14;
    let o = crate::game::grubber::Grubber__004ea740(g, x);
    g.grubber(o).offset_0xc = 65.0;
    g.wc(o).offset_0x30 = 0x41;
    FUN_00542ee0(g, this, o, 1);
    let mgr = g.wc(this).offset_0xc;
    vcall!(g, mgr, w.vfunction4, o);
    FUN_00544800(g, this, o);
    FUN_0053aa40(g, this);
}

/// port: 005445a0 FUN_005445a0
/// `AddBeetle(x, y)`: a `Larva` on the board and the widget manager.
pub fn FUN_005445a0(g: &mut G, this: Ptr, param_1: i32, param_2: i32) {
    let o = crate::game::larva::Larva__004ead40(g, param_1, param_2);
    FUN_00542ee0(g, this, o, 0);
    let mgr = g.wc(this).offset_0xc;
    vcall!(g, mgr, w.vfunction4, o);
    FUN_0053aa40(g, this);
}

/// port: 00545080 FUN_00545080
/// `BuyBeetlemuncher()`: a Gekko dropped in from the top (y 40, falling at 23..27) at a
/// random spot, with 45..54 updates of slowing.
pub fn FUN_00545080(g: &mut G, this: Ptr) {
    let app = g.board(this).field_0x0;
    let x = (app_rand(g, app) % 0x208) as i32 + 0x14;
    let y = (app_rand(g, app) % 0x109) as i32 + 0x69;
    let o = crate::game::gekko::Gekko__004ef9a0(g, x, y);
    let r = app_rand(g, app);
    g.fish(o).field_0x1c = (r % 5 + 0x17) as f64;
    let r = app_rand(g, app);
    g.fish(o).field_0xc = 40.0;
    g.wc(o).offset_0x30 = 0x28;
    g.fish(o).field_0xb4 = (r % 10) as i32 + 0x2d;
    FUN_00542ee0(g, this, o, 1);
    let mgr = g.wc(this).offset_0xc;
    vcall!(g, mgr, w.vfunction4, o);
    FUN_00544800(g, this, o);
    FUN_0053aa40(g, this);
}

/// port: 005451c0 FUN_005451c0
/// `AddBeetlemuncher(x, y, facingRight)` (a revived one).
pub fn FUN_005451c0(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: bool) {
    let o = crate::game::gekko::Gekko__004efa10(g, param_1, param_2, param_3);
    FUN_00542ee0(g, this, o, 1);
    let mgr = g.wc(this).offset_0xc;
    vcall!(g, mgr, w.vfunction4, o);
    FUN_00544800(g, this, o);
    FUN_0053aa40(g, this);
}

/// port: 00544e60 FUN_00544e60
/// `BuyUltravore()`: an Ultra dropped in from the top (y 40, falling at 25..29) at a random
/// spot, with 45..54 updates of slowing; the most ultravores statistic (+0x470 from the
/// +0xd4 list).
pub fn FUN_00544e60(g: &mut G, this: Ptr) -> Ptr {
    let app = g.board(this).field_0x0;
    let x = (app_rand(g, app) % 0x208) as i32 + 0x14;
    let y = (app_rand(g, app) % 0x109) as i32 + 0x69;
    let o = crate::game::ultra::Ultra__004f02a0(g, x, y);
    let r = app_rand(g, app);
    g.fish(o).field_0x1c = (r % 5 + 0x19) as f64;
    let r = app_rand(g, app);
    g.fish(o).field_0xc = 40.0;
    g.wc(o).offset_0x30 = 0x28;
    g.fish(o).field_0xb4 = (r % 10) as i32 + 0x2d;
    FUN_00542ee0(g, this, o, 1);
    let mgr = g.wc(this).offset_0xc;
    vcall!(g, mgr, w.vfunction4, o);
    let n = g.board(this).offset_0xc[vec_index(0xd4)].len() as i32;
    let b = g.board(this);
    if b.ext_0x460[(0x470 - 0x460) / 4] < n {
        b.ext_0x460[(0x470 - 0x460) / 4] = n;
    }
    FUN_00544800(g, this, o);
    FUN_0053aa40(g, this);
    o
}

/// port: 00544fe0 FUN_00544fe0
/// `AddUltravore(x, y, facingRight)` (a revived one).
pub fn FUN_00544fe0(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: bool) {
    let o = crate::game::ultra::Ultra__004f0310(g, param_1, param_2, param_3);
    FUN_00542ee0(g, this, o, 1);
    let mgr = g.wc(this).offset_0xc;
    vcall!(g, mgr, w.vfunction4, o);
    FUN_00544800(g, this, o);
    FUN_0053aa40(g, this);
}

/// port: 005470a0 FUN_005470a0
/// `BuyBreeder()`: a Breeder dropped in from the top (y 30, falling at 18..22) at a random
/// spot, with 45..54 updates of slowing.
pub fn FUN_005470a0(g: &mut G, this: Ptr) {
    let app = g.board(this).field_0x0;
    let x = (app_rand(g, app) % 0x208) as i32 + 0x14;
    let y = (app_rand(g, app) % 0x109) as i32 + 0x69;
    let o = crate::game::breeder::Breeder__004ee530(g, x, y);
    let r = app_rand(g, app);
    g.breeder(o).field_0x1c = (r % 5 + 0x12) as f64;
    let r = app_rand(g, app);
    g.breeder(o).field_0xc = 30.0;
    g.wc(o).offset_0x30 = 0x1e;
    g.breeder(o).offset_0x84 = (r % 10) as i32 + 0x2d;
    FUN_00542ee0(g, this, o, 1);
    let mgr = g.wc(this).offset_0xc;
    vcall!(g, mgr, w.vfunction4, o);
    FUN_00544800(g, this, o);
    FUN_0053aa40(g, this);
}

/// port: 00547270 FUN_00547270
/// `AddBreeder(x, y, stage, facingRight)` (a revived one).
pub fn FUN_00547270(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32, param_4: bool) {
    let o = crate::game::breeder::Breeder__004ee5c0(g, param_1, param_2, param_3, param_4);
    FUN_00542ee0(g, this, o, 1);
    let mgr = g.wc(this).offset_0xc;
    vcall!(g, mgr, w.vfunction4, o);
    FUN_00544800(g, this, o);
    FUN_0053aa40(g, this);
}

/// port: 005454f0 FUN_005454f0
/// `AddGuppycruncher(x)` (a revived one) on the floor at x.
pub fn FUN_005454f0(g: &mut G, this: Ptr, param_1: i32) {
    let o = crate::game::grubber::Grubber__004ea740(g, param_1);
    FUN_00542ee0(g, this, o, 1);
    let mgr = g.wc(this).offset_0xc;
    vcall!(g, mgr, w.vfunction4, o);
    FUN_00544800(g, this, o);
    FUN_0053aa40(g, this);
}

/// port: 00545360 FUN_00545360
/// `AddStarcatcher(x)` (a revived one) on the floor at x, with the most starcatchers
/// statistic (+0x494).
pub fn FUN_00545360(g: &mut G, this: Ptr, param_1: i32) {
    let o = crate::game::penta::Penta__004ec080(g, param_1);
    FUN_00542ee0(g, this, o, 1);
    let mgr = g.wc(this).offset_0xc;
    vcall!(g, mgr, w.vfunction4, o);
    let n = g.board(this).offset_0xc[vec_index(0xd0)].len() as i32;
    let b = g.board(this);
    if b.ext_0x460[(0x494 - 0x460) / 4] < n {
        b.ext_0x460[(0x494 - 0x460) / 4] = n;
    }
    FUN_00544800(g, this, o);
    FUN_0053aa40(g, this);
}

/// port: 00544130 FUN_00544130
/// `AddMissile(x, y, target, kind)`: on the board and the widget manager, on top, below
/// the layer-1 overlay; a homing missile is what its target carries.
pub fn FUN_00544130(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: Ptr, param_4: i32) {
    let m = crate::game::missle::Missle__004eae40(g, param_1, param_2, param_3, param_4);
    FUN_00542ee0(g, this, m, 0);
    let mgr = g.wc(this).offset_0xc;
    vcall!(g, mgr, w.vfunction4, m);
    vcall!(g, mgr, w.vfunction12, m);
    let overlay = g.board(this).offset_0x74;
    vcall!(g, mgr, w.vfunction12, overlay);
    if !crate::game::missle::FUN_004d80c0(g, m) {
        g.go(param_3).offset_0x10 = m;
    }
}

/// port: 00539fe0 FUN_00539fe0
/// Shows the +0xe4 decorations with 3D (or profile flag 2) unless flag 3 (void mode).
pub fn FUN_00539fe0(g: &mut G, this: Ptr) {
    let app = g.board(this).field_0x0;
    let profile = g.wfa(app).offset_0x18c;
    let mut show = crate::sexy::sexy_app_base::dtor_MemoryImage__00489a20(g, app) || (g.profile(profile).field_0x84 >> 2) & 1 != 0;
    if (g.profile(profile).field_0x84 >> 3) & 1 != 0 {
        show = false;
    }
    for o in g.board(this).offset_0xc[vec_index(0xe4)].clone() {
        g.w(o).offset_0x0 = show;
    }
}

/// port: 005404a0 FUN_005404a0
/// A cheat code typed (`param_1`): toggles its profile flag and says so: wavy mode; the
/// original Prego, breeder and ultra Prego sounds; void and space mode (exclusive, the
/// backdrop redone; space starts the star field); zombie mode (`DAT_005e89ce`).
pub fn FUN_005404a0(g: &mut G, this: Ptr, param_1: i32) -> bool {
    let app = g.board(this).field_0x0;
    let profile = g.wfa(app).offset_0x18c;
    let on = crate::game::profile::FUN_00501500(g.profile(profile), param_1 as u8);
    let text: &[u8] = match param_1 {
        0 => {
            let t: &[u8] = if on { b"Wavy Mode Enabled" } else { b"Wavy Mode Disabled" };
            FUN_0053e950(g, this, t, false, -1);
            return true;
        }
        1 => if on { b"Original Prego Sound Enabled" } else { b"Original Prego Sound Disabled" },
        5 => if on { b"Original Breeder Sound Enabled" } else { b"Original Breeder Sound Disabled" },
        6 => if on { b"Ultra Prego Sound Enabled" } else { b"Ultra Prego Sound Disabled" },
        2 => {
            crate::game::profile::FUN_005014d0(g.profile(profile), 3, false);
            FUN_00539fe0(g, this);
            FUN_00537d30(g, this);
            if on { b"Void Mode Enabled" } else { b"Void Mode Disabled" }
        }
        3 => {
            crate::game::profile::FUN_005014d0(g.profile(profile), 2, false);
            FUN_00539fe0(g, this);
            FUN_00537d30(g, this);
            let t: &[u8] = if on { b"Space Mode Enabled" } else { b"Space Mode Disabled" };
            FUN_0053e950(g, this, t, false, -1);
            if on {
                let sf = g.board(this).offset_0xb4;
                crate::game::board_parts::FUN_00511580(g, sf, 1000);
            }
            return true;
        }
        4 => {
            g.globals.DAT_005e89ce = on;
            if on { b"Zombie Mode Enabled" } else { b"Zombie Mode Disabled" }
        }
        _ => return true,
    };
    FUN_0053e950(g, this, text, false, -1);
    true
}

/// port: 005475b0 FUN_005475b0
/// `AddAlienAnywhere(kind, announce)`: an alien of `kind` at a random spot (x 20..469,
/// y 105..299).
pub fn FUN_005475b0(g: &mut G, this: Ptr, param_1: i32, param_2: bool) {
    let app = g.board(this).field_0x0;
    let r = g.wfa(app).offset_0x84;
    let x = crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(r));
    let y = crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(r));
    FUN_00545620(g, this, param_1, (x % 0x1c2) as i32 + 0x14, (y % 0xc3) as i32 + 0x69, param_2);
}

/// port: 00545580 FUN_00545580
/// `AddBossMinion(x, y)`: a small alien (kind 0x14) at (x+40, y+40), in the board's
/// manager, with a shadow; the alien state is refreshed.
pub fn FUN_00545580(g: &mut G, this: Ptr, param_1: i32, param_2: i32) {
    let a = crate::game::alien::Alien__004ecfa0(g, param_1 + 0x28, param_2 + 0x28, 0x14);
    FUN_00542ee0(g, this, a, 1);
    let mgr = g.wc(this).offset_0xc;
    vcall!(g, mgr, w.vfunction4, a);
    FUN_00544800(g, this, a);
    FUN_0053aa40(g, this);
}

/// port: 0053bfb0 FUN_0053bfb0
/// `ClickThrough(x, y)`: a click no object took. A pet that wants clicks (FishTypePet
/// +0x230, OtherTypePet +0x1c8) under it gets it; otherwise the glass tap, and in the
/// virtual tank the fish near it are startled, and a click on an object with a speech
/// bubble clears the melody list.
pub fn FUN_0053bfb0(g: &mut G, this: Ptr, param_1: i32, param_2: i32) {
    if FUN_00539630(g, this, param_1, param_2) {
        return;
    }
    let app = g.board(this).field_0x0;
    let s = g.res.DAT_005e8b68;
    crate::sexy::sexy_app_base::vfunction55(g, app, s);
    if g.wfa(app).offset_0x150 != 5 {
        return;
    }
    for off in [0xa0, 0xa4, 0xcc] {
        for f in g.board(this).offset_0xc[vec_index(off)].clone() {
            crate::game::fish::FUN_004d6040(g, f, param_1, param_2);
        }
    }
    let objs: Vec<Ptr> = g.board(this).offset_0x7c.iter().copied().collect();
    for o in objs {
        if g.go(o).offset_0x94 != -1 && vcall!(g, o, w.vfunction69, param_1, param_2) {
            let m = g.board(this).offset_0x4;
            crate::game::timed_messages::FUN_00513c20(g, m);
            return;
        }
    }
}

/// port: 00539630 FUN_00539630
/// Passes a click to the first pet under it that takes clicks (the fish-type pets first),
/// as a right click relative to the pet; true when one did. Not re-entrant.
pub fn FUN_00539630(g: &mut G, this: Ptr, param_1: i32, param_2: i32) -> bool {
    if g.globals.DAT_005e9630 {
        return false;
    }
    g.globals.DAT_005e9630 = true;
    let mut hit = NULL;
    for p in g.board(this).offset_0xc[vec_index(0xb4)].clone() {
        if g.fish_type_pet(p).offset_0x4 && vcall!(g, p, w.vfunction69, param_1, param_2) {
            hit = p;
            break;
        }
    }
    if hit == NULL {
        for p in g.board(this).offset_0xc[vec_index(0xb0)].clone() {
            if g.other_pet(p).offset_0x74 && vcall!(g, p, w.vfunction69, param_1, param_2) {
                hit = p;
                break;
            }
        }
    }
    if hit != NULL {
        let (x, y) = (g.wc(hit).offset_0x2c, g.wc(hit).offset_0x30);
        vcall!(g, hit, w.vfunction55, param_1 - x, param_2 - y, -1);
    }
    g.globals.DAT_005e9630 = false;
    hit != NULL
}

/// port: 005471e0 FUN_005471e0
/// `AddBreeder(x, y)`: a newborn breeder in the board's manager, with a shadow; the alien
/// state is refreshed.
pub fn FUN_005471e0(g: &mut G, this: Ptr, param_1: i32, param_2: i32) {
    let b = crate::game::breeder::Breeder__004ee530(g, param_1, param_2);
    FUN_00542ee0(g, this, b, 1);
    let mgr = g.wc(this).offset_0xc;
    vcall!(g, mgr, w.vfunction4, b);
    FUN_00544800(g, this, b);
    FUN_0053aa40(g, this);
}

/// port: 0053dc10 FUN_0053dc10
/// A finished adventure's level replayed as its bonus round: the level-end state (+0x4ee),
/// the board reset (`FUN_0053c480`), the shop song, the bonus round.
pub fn FUN_0053dc10(g: &mut G, this: Ptr) {
    g.board(this).ext_0x4ee = true;
    let wm = g.wc(this).offset_0xc;
    crate::game::board::FUN_0053c480(g, this, wm);
    let app = g.board(this).field_0x0;
    crate::game::win_fish_app::FUN_0054b020(g, app);
    crate::game::win_fish_app::FUN_0054b1a0(g, app, 2, 0x36, false);
    FUN_00537e20(g, this);
}

/// port: 00539540 FUN_00539540
/// Presto, whichever pet he is disguised as: the first pet of the +0xb4 kind with +0x230
/// set, else the first of the +0xb0 kind with +0x1c8 set; null when absent.
pub fn FUN_00539540(g: &mut G, this: Ptr) -> Ptr {
    for p in g.board(this).offset_0xc[vec_index(0xb4)].clone() {
        if g.fish_type_pet(p).offset_0x4 {
            return p;
        }
    }
    for p in g.board(this).offset_0xc[vec_index(0xb0)].clone() {
        if g.other_pet(p).offset_0x74 {
            return p;
        }
    }
    NULL
}
