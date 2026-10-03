//! `Board::Draw()` (`vfunction27_for_Widget`) and the backdrop drawing it starts with: the
//! tank image (shaken while hit), the animated sand, the light layers, the tutorial arrows,
//! the mode / tank caption and clocks, the alien-portal warning sparkles.

use crate::sexy::graphics::{FUN_004558c0, FUN_004558e0, FUN_00455880, FUN_00455890, FUN_00455920, FUN_00455cf0, FUN_00455d20, FUN_00456950, FUN_004560a0, FUN_004563d0};
use crate::sexy::prelude::*;
use crate::sexy::types::CRect;

/// One "Click here to / ..." tutorial arrow with its two caption lines.
fn arrow(g: &mut G, gfx: &mut Graphics, ax: i32, line1: (i32, i32), text2: &[u8], line2: (i32, i32)) {
    let img = g.res.DAT_005e8a6c;
    FUN_00455d20(gfx, g, img, ax, 0x46);
    FUN_00455cf0(gfx, g, b"Click here to", line1.0, line1.1);
    FUN_00455cf0(gfx, g, text2, line2.0, line2.1);
}

/// port: 0053f5f0 Sexy::Board::vfunction27_for_Widget
/// `Draw(Graphics*)`.
pub fn vfunction27_for_Widget(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    FUN_00538e50(g, this, gfx);
    let app = g.board(this).field_0x0;
    if g.wfa(app).offset_0x150 == 5 {
        if !crate::game::win_fish_app::thunk_FUN_00479fc0(g, app) {
            let img = g.res.DAT_005e8dc0;
            FUN_00455d20(gfx, g, img, 0, 0);
        }
    } else {
        let img = g.res.DAT_005e8b10;
        FUN_00455d20(gfx, g, img, 0, 0);
    }
    let flash = g.board(this).ext_0x450;
    if 0 < flash && flash % 0x14 < 10 {
        let img = g.res.DAT_005e8e70;
        FUN_00455d20(gfx, g, img, 0x221, 0x27);
    }
    let f = g.res.DAT_005e8cf0;
    FUN_00455880(gfx, f);
    FUN_00455890(gfx, CRect(0x6e, 0xfa, 0x6e, 0xff));
    let profile = g.wfa(app).offset_0x18c;
    let done = g.profile(profile).field_0x59;
    let (tank, level) = (g.board(this).field_0x33c, g.board(this).field_0x340);
    let flags = g.board(this).ext_0x4b0;
    if flags[2] != 0 && level == 1 && tank == 1 && !done {
        arrow(g, gfx, 0x28, (0x41, 0x5f), b"buy fish!", (0x53, 0x6b));
    } else if flags[0x28] != 0 && level == 1 && tank == 1 && !done {
        arrow(g, gfx, 0x1cc, (0x1e5, 0x5f), b"buy egg piece!", (0x1e0, 0x6b));
    } else if flags[0xf] != 0 && level == 2 && tank == 1 && !done {
        arrow(g, gfx, 0x6e, (0x87, 0x5f), b"upgrade!", (0x98, 0x6b));
    } else if tank == 5 && g.board(this).ext_0x4f4 && !g.board(this).field_0x219 {
        arrow(g, gfx, 0x1cc, (0x1e5, 0x5f), b"end the level!", (0x1e0, 0x6b));
    } else if g.wfa(app).offset_0x150 == 5
        && !g.board(this).field_0x22c
        && crate::sexy::sexy_app_base::vfunction74(g, app, 0x27) == NULL
        && !crate::game::win_fish_app::thunk_FUN_00479fc0(g, app)
    {
        arrow(g, gfx, 100, (0x7d, 0x5f), b"buy fish!", (0x8c, 0x6b));
    }
    let mode = g.wfa(app).offset_0x150;
    if mode == 5 {
        return;
    }
    let f = g.res.DAT_005e8ce4;
    FUN_00455880(gfx, f);
    let c = match g.board(this).field_0x35c {
        2 => CRect(0xdc, 0x96, 0x96, 0xff),
        4 => CRect(0x3c, 0xb4, 0x50, 0xff),
        3 => CRect(0x7d, 200, 0xd7, 0xff),
        _ => CRect(0xa5, 0x8c, 0x50, 0xff),
    };
    FUN_00455890(gfx, c);
    if g.wfa(app).offset_0x154 {
        FUN_00455cf0(gfx, g, b"Relax", 0xf, 0x1d6);
    } else {
        let caption: Option<Vec<u8>> = if mode == 4 {
            Some(b"Challenge".to_vec())
        } else if mode == 1 {
            Some(b"Time Trial".to_vec())
        } else if mode == 3 {
            Some(b"Sandbox".to_vec())
        } else if g.board(this).field_0x219 && !done {
            Some(b"Bonus Round".to_vec())
        } else if tank != 5 {
            Some(format!("Tank {tank}-{level}").into_bytes())
        } else {
            None
        };
        if let Some(s) = caption {
            FUN_00455cf0(gfx, g, &s, 0xf, 0x1d6);
        }
    }
    let mode = g.wfa(app).offset_0x150;
    let mut bonus_hud = g.board(this).field_0x219;
    if mode == 1 {
        let left = g.board(this).field_0x330 - g.board(this).field_0x324;
        let m = (left / 0x3c).max(0);
        let s = (left % 0x3c).max(0);
        if g.board(this).ext_0x444 < 0x15b {
            let f = crate::sexy::graphics::FUN_00455870(gfx);
            let w = crate::sexy::image_font::string_width(g, f, b"Time Remaining: ");
            let t = format!("{m}:{s:02}").into_bytes();
            FUN_00455cf0(gfx, g, &t, w + 0x1d1, 0x1d6);
        } else {
            let t = format!("Time Remaining: {m}:{s:02}").into_bytes();
            FUN_00455cf0(gfx, g, &t, 0x1d1, 0x1d6);
        }
    } else {
        let profile = g.wfa(app).offset_0x18c;
        if (g.profile(profile).field_0x84 >> 7) & 1 != 0 && !g.board(this).field_0x219 {
            if mode != 5 {
                let secs = (crate::game::board::FUN_00537b60(g, this) - g.board(this).field_0x32c) / 1000;
                let m = (secs / 0x3c).max(0);
                let s = (secs % 0x3c).max(0);
                let t = format!("Time: {m}:{s:02}").into_bytes();
                FUN_00455cf0(gfx, g, &t, 0x21c, 0x1d6);
            }
            bonus_hud = false;
        }
    }
    if bonus_hud {
        FUN_0053eaf0(g, this, gfx);
    }
    let t = g.board(this).field_0x234;
    if 0 < t && t <= 0xe1 && g.board(this).field_0xb8[13] != 0 && g.board(this).field_0x33c != 5 && g.wfa(app).offset_0x150 != 5 {
        FUN_004558c0(gfx, 1);
        let cnt = g.board(this).ext_0x444;
        g.board(this).field_0x244 = 0x118;
        let img = g.res.DAT_005e8e1c;
        let x = g.board(this).field_0x240 + 0x28;
        FUN_00456950(gfx, g, img, x, 0x140, (cnt / 4) % 5);
        let kind = g.board(this).field_0x230;
        if (9..=0xc).contains(&kind) {
            if kind == 0xb {
                g.board(this).field_0x24c = 0x118;
            }
            let img = g.res.DAT_005e8e1c;
            let (x, y) = (g.board(this).field_0x248 + 0x28, g.board(this).field_0x24c + 0x28);
            FUN_00456950(gfx, g, img, x, y, (cnt / 4 + 2) % 5);
        }
        FUN_004558c0(gfx, 0);
        if g.board(this).field_0x234 == 0xe1 {
            crate::game::board::FUN_00538230(g, this, 0x13c, 3, 1.0);
        }
    }
}

/// port: 00538e50 FUN_00538e50
/// `DrawBackdrop(Graphics*)`: white with profile flag 2, the star field with flag 3;
/// otherwise the backdrop (shaken by a hit: CRT `rand`), the wavy redraw (flag 0), the
/// sand, and with 3D the drifting light layers and the backdrop's bottom overlay.
pub fn FUN_00538e50(g: &mut G, this: Ptr, param_1: &mut Graphics) {
    let ftol = crate::sexy::crt::ftol;
    let app = g.board(this).field_0x0;
    let profile = g.wfa(app).offset_0x18c;
    let flags = g.profile(profile).field_0x84;
    if (flags >> 2) & 1 != 0 {
        FUN_00455890(param_1, FUN_00433320(0xffffff));
        FUN_00455920(param_1, 0, 0, 0x280, 0x1e0);
        return;
    }
    if (flags >> 3) & 1 != 0 {
        let sf = g.board(this).offset_0xb4;
        crate::game::board_parts::FUN_00504f20(g, sf, param_1, (flags & 1) != 0);
        return;
    }
    let (mut dx, mut dy) = (0, 0);
    let shake = g.board(this).field_0x228;
    if shake != 0 && shake < 0x14 && !g.board(this).field_0x8 {
        dx = crate::sexy::crt::rand(g) % 5 - 2;
        dy = crate::sexy::crt::rand(g) % 5 - 2;
        FUN_00455890(param_1, FUN_00433320(0));
        FUN_00455920(param_1, 0, 0, 0x280, 0x1e0);
        FUN_004563d0(param_1, dx, dy);
    }
    let bd = g.board(this).field_0x35c;
    let img = crate::sexy::res::FUN_005016a0(g, bd + 0x9e) as Ptr;
    FUN_00455d20(param_1, g, img, 0, 0);
    let profile = g.wfa(app).offset_0x18c;
    if g.profile(profile).field_0x84 & 1 != 0 {
        let s = if crate::sexy::sexy_app_base::dtor_MemoryImage__00489a20(g, app) { 1.2f32 } else { 2.0f32 };
        FUN_00538cf0(g, this, param_1, img, 0, 0, s);
    }
    let cnt = g.board(this).ext_0x444;
    FUN_00500c90(g, param_1, 0x58, cnt);
    if crate::sexy::sexy_app_base::dtor_MemoryImage__00489a20(g, app) {
        FUN_004558c0(param_1, 1);
        FUN_004558e0(param_1, true);
        let rays = g.res.DAT_005e8ae8;
        let st = g.board(this).field_0x34c;
        FUN_00455890(param_1, CRect(0xff, 0xff, 0xff, 0x28));
        FUN_00455d20(param_1, g, rays, ftol(st[0] as f64) as i32, 0x16d);
        FUN_00455d20(param_1, g, rays, ftol(st[0] as f64 - 640.0) as i32, 0x16d);
        FUN_00455890(param_1, CRect(0xff, 0xff, 0xff, 100));
        FUN_00455d20(param_1, g, rays, ftol(st[1] as f64) as i32, 0x16d);
        FUN_00455d20(param_1, g, rays, ftol(st[1] as f64 - 640.0) as i32, 0x16d);
        FUN_00455890(param_1, CRect(0xff, 0xff, 0xff, 0x46));
        crate::sexy::graphics::FUN_00456060(param_1, g, rays, ftol(st[2] as f64) as i32, 0x16d, true);
        crate::sexy::graphics::FUN_00456060(param_1, g, rays, ftol(st[2] as f64 - 640.0) as i32, 0x16d, true);
        FUN_004558e0(param_1, false);
        FUN_004558c0(param_1, 0);
        let bottom = match g.board(this).field_0x35c {
            1 => Some(g.res.DAT_005e8bc0),
            2 => Some(g.res.DAT_005e8f04),
            3 => Some(g.res.DAT_005e8a30),
            4 => Some(g.res.DAT_005e8ad0),
            5 => Some(g.res.DAT_005e8af8),
            6 => Some(g.res.DAT_005e8a74),
            _ => None,
        };
        if let Some(b) = bottom {
            FUN_00455d20(param_1, g, b, 0, 0x16d);
        }
    }
    FUN_004563d0(param_1, -dx, -dy);
}

/// port: 00500c90 FUN_00500c90
/// The animated sand strip across the tank at `param_2` (additive): end pieces mirrored.
pub fn FUN_00500c90(g: &mut G, param_1: &mut Graphics, param_2: i32, param_3: i32) {
    FUN_004558c0(param_1, 1);
    let mid = g.res.DAT_005e8af4;
    let cel = crate::sexy::image::FUN_004578a0(g, mid, param_3);
    let end = g.res.DAT_005e8a50;
    let r = crate::sexy::image::FUN_004574c0(g, end, cel);
    FUN_004560a0(param_1, g, end, 0, param_2, &r, false);
    FUN_00456950(param_1, g, mid, 0xa0, param_2, cel);
    FUN_00456950(param_1, g, mid, 0x140, param_2, cel);
    FUN_004560a0(param_1, g, end, 0x1e0, param_2, &r, true);
    FUN_004558c0(param_1, 0);
}

/// port: 0053bd90 FUN_0053bd90
/// `DrawOverlay0(Graphics*)`: the bubble vent (+0x4fd), the alien portal beam (+0x4fe),
/// and (+0x4fc) the name labels of objects being dropped in.
pub fn FUN_0053bd90(g: &mut G, this: Ptr, param_1: &mut Graphics) {
    use crate::sexy::graphics::{FUN_004558e0, FUN_00455890, FUN_00455d20, FUN_00456950, FUN_004563d0, FUN_004563f0};
    if g.board(this).ext_0x4fd {
        let (x, y) = crate::game::board_update::FUN_00538b60(g, this);
        FUN_004558e0(param_1, true);
        FUN_00455890(param_1, FUN_00433320(0));
        let sh = g.res.DAT_005e8da4;
        FUN_00455d20(param_1, g, sh, x + 10, y + 0x40);
        FUN_004558e0(param_1, false);
        let vent = g.res.DAT_005e8d70;
        let cel = g.board(this).ext_0x44c;
        FUN_00456950(param_1, g, vent, x, y, cel);
    }
    if g.board(this).ext_0x4fe {
        let (x, y) = crate::game::virtual_tank::FUN_00538c10(g, this);
        FUN_00455890(param_1, crate::sexy::types::CRect(0xff, 0xff, 0x80, 0xa0));
        let (t, d) = (g.board(this).field_0x234, g.board(this).field_0x238);
        if t < 0x1e || 0 < d {
            let mut k = d;
            if k < 1 {
                k = 0x1e - t;
            }
            if 5 < k {
                k = 5;
            }
            let img = g.res.DAT_005e8a64;
            let w = g.image(img).offset_0x20;
            let h = (k * 0x1c2) / 5;
            let r = Rect::new((x - w / 2) + 0x28, (y - h) + 0x28, w, h);
            FUN_004563f0(param_1, g, &r, img);
        }
        FUN_00500940(g, param_1, x, y, t, d);
    }
    if g.board(this).ext_0x4fc {
        for o in g.board(this).offset_0x7c.clone() {
            let go = g.go(o).clone();
            if go.offset_0x6c && go.offset_0x74 && go.offset_0x78 == 0 {
                let (ox, oy) = (g.wc(o).offset_0x2c, g.wc(o).offset_0x30);
                FUN_004563d0(param_1, ox, oy);
                crate::game::game_object::FUN_004e1400(g, o, param_1, true);
                FUN_004563d0(param_1, -ox, -oy);
            }
        }
    }
}

/// port: 005400f0 FUN_005400f0
/// `DrawOverlay1(Graphics*)`: the bubbles; in the screensaver also the black bars, the
/// "running" warning, the shells collected and the periodic dimming.
pub fn FUN_005400f0(g: &mut G, this: Ptr, param_1: &mut Graphics) {
    use crate::sexy::graphics::{FUN_00455870, FUN_00455880, FUN_00455890, FUN_00455920};
    let bm = g.board(this).offset_0x90;
    crate::game::board_parts::FUN_00504b60(g, bm, param_1);
    let app = g.board(this).field_0x0;
    if !crate::game::win_fish_app::thunk_FUN_00479fc0(g, app) {
        return;
    }
    FUN_00455890(param_1, FUN_00433320(0));
    FUN_00455920(param_1, 0, 0, 0x280, 0x3c);
    FUN_00455920(param_1, 0, 0x1bd, 0x280, 0x23);
    let warn: &[u8] = b"Warning: Insaniquarium is running.  Shells collected here may not be saved.";
    let f = g.res.DAT_005e8b04;
    FUN_00455880(param_1, f);
    FUN_00455890(param_1, FUN_00433320(0xffffff));
    let font = FUN_00455870(param_1);
    let w = crate::sexy::image_font::string_width(g, font, warn);
    let cnt = g.wc(this).offset_0x24;
    let outline = g.res.DAT_005e8d1c;
    if g.wfa(app).offset_0x111 {
        let y = if (cnt / 1000) & 1 == 0 { 0x28 } else { 0x1d1 };
        crate::game::help_screen::FUN_00500bf0(g, param_1, warn, 0x140 - w / 2, y, outline, 0x29558c);
    }
    let f = g.res.DAT_005e8b04;
    FUN_00455880(param_1, f);
    let shells = g.board(this).field_0x374;
    let s = format!("{shells} Shells Collected").into_bytes();
    let font = FUN_00455870(param_1);
    let w = crate::sexy::image_font::string_width(g, font, &s);
    let (x, y) = match (cnt / 1000) % 4 {
        0 => (0x276 - w, 0x1d4),
        1 => (0x276 - w, 0x28),
        2 => (10, 0x1d4),
        3 => (10, 0x28),
        // A negative update count leaves both unset in the original.
        _ => (0, 0),
    };
    if g.wfa(app).offset_0x110 {
        let f = g.res.DAT_005e8b04;
        FUN_00455880(param_1, f);
        FUN_00455890(param_1, FUN_00433320(0xffffff));
        crate::game::help_screen::FUN_00500bf0(g, param_1, &s, x, y, outline, 0x29558c);
    }
    if g.wfa(app).offset_0x10f {
        let t = cnt % 3000;
        if t < 0x528 || 0x690 < t {
            return;
        }
        let a = if t < 0x547 {
            crate::game::help_screen::FUN_005036e0(0, 200, t - 0x528, 0x1e, false)
        } else if 0x671 < t {
            crate::game::help_screen::FUN_005036e0(200, 0, t - 0x672, 0x1e, false)
        } else {
            200
        };
        FUN_00455890(param_1, crate::sexy::types::CRect(0, 0, 0, a));
        FUN_00455920(param_1, 0, 0x3c, 0x280, 0x181);
    }
}

/// port: 0053dc50 FUN_0053dc50
/// `DrawWavyString(const char*, int x, int y)` (Graphics in EDI): each character on a sine
/// wave (5 pixels, 35 degrees per character) that moves a quarter character per update.
pub fn FUN_0053dc50(g: &mut G, this: Ptr, gfx: &mut Graphics, param_1: &[u8], mut param_2: i32, param_3: i32) {
    let base = g.board(this).ext_0x444 as f64 * 0.25;
    let yf = param_3 as f32;
    let mut prev = 0u8;
    for (i, &c) in param_1.iter().enumerate() {
        let a = (((i as f64 + base) * 35.0 * crate::game::board_parts::DAT_0059be10) / 180.0) as f32;
        let s = (a as f64).sin() as f32;
        let y = crate::sexy::crt::ftol(s as f64 * 5.0 + yf as f64) as i32;
        FUN_00455cf0(gfx, g, &[c], param_2, y);
        let f = crate::sexy::graphics::FUN_00455870(gfx);
        param_2 += crate::sexy::image_font::char_width_kern(g, f, c, prev);
        prev = c;
    }
}

/// port: 0053eaf0 FUN_0053eaf0
/// The bonus round's HUD. Before it starts: the shell counter drops in (updates 130..151),
/// "BONUS ROUND" (wavy) and "Collect as many shells as you can!" slide in from either side
/// and fade in, the shell values (1, 2, 5, 10, 20) rise from below, and the 3-2-1 countdown
/// pops (an additive glow as each number lands). Once running: the counter with the shells
/// collected (blinking in the last 60 updates' tenths), the captions fading out over 20
/// updates. Always the time remaining.
pub fn FUN_0053eaf0(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    use crate::sexy::crt::ftol;
    use crate::sexy::graphics::{FUN_004558b0, FUN_00455870, FUN_00455900};
    use crate::sexy::image_font::string_width;
    let t = g.board(this).ext_0x444 - g.board(this).field_0x21c;
    let saved = FUN_004558b0(gfx);
    let (x, y) = crate::game::board_level::FUN_00538cd0(g, this);
    let counter = g.res.DAT_005e8cc4;
    let active = g.board(this).field_0x218;
    if !active {
        if 0x81 < t {
            if t < 0x96 {
                let mut v = ((t - 0x82) as f64 / 20.0) as f32;
                v = v * v;
                let yy = ftol((y + 5) as f64 * v as f64) as i32;
                FUN_00455d20(gfx, g, counter, x, yy);
            } else {
                let mut yy = y;
                if t < 0x98 {
                    let a = ((t - 0x96) as f64 * 0.5) as f32;
                    yy = ftol((y + 5) as f64 - a as f64 * 5.0) as i32;
                }
                FUN_00455d20(gfx, g, counter, x, yy);
            }
        }
    } else {
        FUN_00455d20(gfx, g, counter, x, y);
        let blink = g.board(this).field_0x36c;
        if 0x3b < blink || (blink / 10) & 1 == 0 {
            let f = g.res.DAT_005e8b04;
            FUN_00455880(gfx, f);
            let s = format!("{}", g.board(this).field_0x368).into_bytes();
            let fnt = FUN_00455870(gfx);
            let w = string_width(g, fnt, &s);
            FUN_00455890(gfx, crate::sexy::types::FUN_00433320(0xf0a33b));
            let outline = g.res.DAT_005e8d1c;
            crate::game::help_screen::FUN_00500bf0(g, gfx, &s, (x - w / 2) + 0x37, y + 0x97, outline, 0x7b2000);
        }
    }
    if !active || t < 0x14 {
        let mut alpha = 0xff;
        if active {
            alpha = 0xff - (t * 0xff) / 0x14;
        } else if t < 10 {
            alpha = (t * 0xff) / 10;
        }
        let width = g.wc(this).offset_0x34;
        let f = g.res.DAT_005e8e40;
        FUN_00455880(gfx, f);
        FUN_00455890(gfx, CRect(0xff, 0xff, 0, alpha));
        let fnt = FUN_00455870(gfx);
        let tw = string_width(g, fnt, b"BONUS ROUND");
        let mut tx = (width - tw) / 2;
        if !active && t < 8 {
            tx += ((8 - t) * tw * -2) / 8;
        }
        FUN_0053dc50(g, this, gfx, b"BONUS ROUND", tx, 0xeb);
        let f = g.res.DAT_005e8b04;
        FUN_00455880(gfx, f);
        FUN_00455890(gfx, CRect(0xb4, 0xfa, 0x5a, alpha));
        let text: &[u8] = b"Collect as many shells as you can!";
        let fnt = FUN_00455870(gfx);
        let cw = string_width(g, fnt, text);
        let cx = (width - cw) / 2;
        let mut slide = 0;
        if !active && t < 8 {
            slide = ((8 - t) * cw * 2) / 8;
        }
        let outline = g.res.DAT_005e8d1c;
        let oc = crate::sexy::types::FUN_004333e0(&CRect(0, 0x4b, 0, alpha));
        crate::game::help_screen::FUN_00500bf0(g, gfx, text, slide + cx, 0x104, outline, oc);
        let rise = if active { 0 } else { crate::game::help_screen::FUN_005036e0(0x208, 0, t, 0x10, false) };
        let y0 = rise + 0x118;
        FUN_004558e0(gfx, true);
        const VALUES: [i32; 5] = [1, 2, 5, 10, 20];
        const DY: [i32; 5] = [0, 0, 0, 0, -12];
        let f = g.res.DAT_005e8cd0;
        FUN_00455880(gfx, f);
        for i in 0..5 {
            let (img, cel) = if i == 4 { (g.res.DAT_005e8d88, 0) } else { (g.res.DAT_005e8d7c, i as i32) };
            let s = format!("{}", VALUES[i]).into_bytes();
            FUN_00455890(gfx, CRect(0xff, 0xff, 0xff, alpha));
            crate::sexy::graphics::FUN_00456980(gfx, g, img, i as i32 * 0x3c + cx, DY[i] + y0, 0, cel);
            FUN_00455890(gfx, CRect(0xff, 0xff, 0xff, alpha));
            let fnt = FUN_00455870(gfx);
            let w = string_width(g, fnt, &s);
            FUN_00455cf0(gfx, g, &s, (0x1c - w) / 2 + i as i32 * 0x3c + cx, y0 + 0x32);
        }
        let q = (0x94 - t) / 0x24;
        let r = (0x94 - t) % 0x24;
        if !active && t < 0x94 && (q as u32) < 3 {
            let img = crate::sexy::res::FUN_005016a0(g, q + 0x59) as Ptr;
            let mut grow = 0x24 - r;
            let fade = if 0x14 < r { 0x14 } else { r };
            let glow = (grow - 5).clamp(0, 10);
            if 5 < grow {
                grow = 5;
            }
            let s = ((grow as f64 * 0.800000011920929) / 5.0 + 0.20000000298023224) as f32;
            let (iw, ih) = (g.image(img).offset_0x20, g.image(img).offset_0x24);
            let sw = ftol(iw as f64 * s as f64) as i32;
            let hf = ih as f32;
            let sh = ftol(hf as f64 * s as f64) as i32;
            let dy = ftol(hf as f64 * (s as f64 - 1.0) * 0.5) as i32;
            let app = g.board(this).field_0x0;
            let is3d = crate::sexy::sexy_app_base::dtor_MemoryImage__00489a20(g, app);
            FUN_00455900(gfx, !is3d);
            let src = Rect::new(0, 0, iw, ih);
            let dest = Rect::new((width - sw) / 2, 0x5a - dy, sw, sh);
            FUN_004558e0(gfx, true);
            FUN_00455890(gfx, CRect(0xff, 0xff, 0xff, (fade * 0xff) / 0x14));
            crate::game::shadow::FUN_005008f0(g, gfx, img, &dest, &src, false);
            FUN_004558c0(gfx, 1);
            FUN_00455890(gfx, CRect(0xff, 0xff, 0xff, ((glow * fade * 0xff) / 200) / 2));
            crate::game::shadow::FUN_005008f0(g, gfx, img, &dest, &src, false);
            FUN_004558c0(gfx, 0);
            FUN_004558e0(gfx, false);
        }
    }
    let f = g.res.DAT_005e8ce4;
    FUN_00455880(gfx, f);
    FUN_00455890(gfx, saved);
    let mut secs = g.board(this).field_0x330;
    if active {
        secs -= g.board(this).field_0x324;
    }
    let m = (secs / 0x3c).max(0);
    let s = (secs % 0x3c).max(0);
    let text = format!("Time Remaining: {m}:{s:02}").into_bytes();
    FUN_00455cf0(gfx, g, &text, 0x1d1, 0x1d6);
}

/// port: 00538cf0 FUN_00538cf0
/// `DrawWavy(Graphics*, Image*, int x, int y, float amplitude)`: the image in 24-pixel
/// strips, each shifted down by `amplitude * sin` of an angle that steps 45 degrees per
/// strip and 5 degrees per board update; only strips 81..439 pixels below `y`. The 3D
/// path places strips at float positions, the software one truncates.
pub fn FUN_00538cf0(g: &mut G, this: Ptr, param_1: &mut Graphics, param_2: Ptr, param_3: i32, param_4: i32, param_5: f32) {
    let w = g.image(param_2).offset_0x20;
    let base = -0x51 - param_4;
    let mut y = param_4;
    let mut a = 0;
    let mut sy = 0;
    while a < 0xb4 {
        if ((base + y) as u32) < 0x167 {
            let app = g.board(this).field_0x0;
            let t = g.board(this).ext_0x444;
            let src = Rect::new(0, sy, w, 0x18);
            let ang = (((t + a) * 5) as f64 * 3.141590118408203 / 180.0) as f32;
            let s = ((ang as f64).sin() as f32 * param_5) as f32;
            let dy = s + y as f32;
            if crate::sexy::sexy_app_base::dtor_MemoryImage__00489a20(g, app) {
                crate::sexy::graphics::FUN_004562e0(param_1, g, param_2, param_3 as f32, dy, &src);
            } else {
                let iy = crate::sexy::crt::ftol(dy as f64) as i32;
                crate::sexy::graphics::FUN_00455e40(param_1, g, param_2, param_3, iy, &src);
            }
        }
        y += 0x18;
        a += 9;
        sy += 0x18;
    }
}

/// port: 00500940 FUN_00500940
/// The alien portal at (x, y) `t` updates into its opening (always opening with `param_5`
/// set): the ring, then the swirl growing in (to 30), a cross-fade from the swirl to the
/// open portal (30..39), the open portal pulsing (40..299), then just the open portal.
pub fn FUN_00500940(g: &mut G, param_1: &mut Graphics, param_2: i32, param_3: i32, param_4: i32, param_5: i32) {
    let t = if param_5 != 0 { 0 } else { param_4 };
    let res = &g.res;
    let r = (res.DAT_005e8b90 as Ptr, res.DAT_005e8d90 as Ptr, res.DAT_005e8b8c as Ptr, res.DAT_005e8cfc as Ptr);
    FUN_00455d20(param_1, g, r.0, param_2, param_3);
    let y1 = param_3 - 0x1a;
    let x0 = param_2 - 0xb;
    let y0 = param_3 - 0x32;
    let x1 = param_2 + 0x19;
    if 299 < t {
        FUN_00455d20(param_1, g, r.3, x1, y1);
        return;
    }
    if t < 0x28 {
        if t < 0x1e {
            FUN_00455d20(param_1, g, r.1, x0, y0);
            return;
        }
        FUN_00455d20(param_1, g, r.2, x1, y1);
        FUN_004558e0(param_1, true);
        FUN_00455890(param_1, CRect(0xff, 0xff, 0xff, ((0x1e - t) * 0xff) / 0x14 + 0xff));
        FUN_00455d20(param_1, g, r.1, x0, y0);
    } else {
        FUN_00455d20(param_1, g, r.3, x1, y1);
        let mut k = (300 - t) % 0x28;
        if 0x14 < k {
            k = 0x28 - k;
        }
        FUN_004558e0(param_1, true);
        FUN_00455890(param_1, CRect(0xff, 0xff, 0xff, (k * 0xff) / 0x14));
        FUN_00455d20(param_1, g, r.2, x1, y1);
    }
    FUN_004558e0(param_1, false);
}
