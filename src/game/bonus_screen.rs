//! `Sexy::BonusScreen`: the results screen after an adventure tank's bonus level, a Time
//! Trial or a Challenge: the level info (time or score against the best), the shell reward
//! counted into the balance, and the bonus shop (the four bonus pets, then the 4-pet and
//! 7-pet limits) with its purchase animation. Plus the app functions that open and close it.
//!
//! `BonusScreen_data` starts at object offset 0x8c (after the ButtonListener vftable at
//! 0x88); the object is 0x18c bytes. The database names the constructor
//! `BonusScreenOverlay::BonusScreenOverlay` (the overlay's constructor is inlined in it);
//! the overlay is the shared overlay class.

use crate::game::board_parts::{FUN_00500120, FUN_00500180, FUN_00504b60, FUN_005110e0, FUN_00512080};
use crate::game::game_selector::FUN_00506820;
use crate::sexy::graphics::{FUN_00455870, FUN_00455880, FUN_00455890, FUN_00455920, FUN_00455cf0, FUN_00455d20, FUN_004563f0, FUN_00456950, FUN_00456980};
use crate::sexy::image_font as font;
use crate::sexy::prelude::*;
use crate::sexy::types::{CRect, FUN_00433320, FUN_00433360};

/// `BonusScreen_data` (object offset 0x8c).
#[derive(Debug, Clone, Default)]
pub struct BonusScreen_data {
    /// +0x8c the app.
    pub field_0x0: Ptr,
    /// +0x90 the bubbles.
    pub offset_0x4: Ptr,
    /// +0x94..+0xa0 the results box (x, y, width, height).
    pub field_0x8: i32,
    pub field_0xc: i32,
    pub field_0x10: i32,
    pub field_0x14: i32,
    /// +0xa4 the overlay widget.
    pub offset_0x18: Ptr,
    /// +0xa8 "Menu" (id 1).
    pub offset_0x1c: Ptr,
    /// +0xac "Click Here To Continue" / "Click for Story Time" (id 0).
    pub offset_0x20: Ptr,
    /// +0xb0 the bonus shop button (id 2).
    pub offset_0x24: Ptr,
    /// +0xb4 the bought pet's sparkle cels (8 columns), or null.
    pub offset_0x28: Ptr,
    /// +0xb8 `FUN_0054acf0` (the story the Challenge continues to).
    pub field_0x2c: i32,
    /// +0xbc the shell reward.
    pub field_0x30: i32,
    /// +0xc0 how much of the reward has been counted.
    pub field_0x34: i32,
    /// +0xc4 the balance shown.
    pub field_0x38: i32,
    /// +0xc8 shells counted per update.
    pub field_0x3c: i32,
    /// +0xcc the counting sound's pitch.
    pub field_0x40: i32,
    /// +0xd0 the bonus shop price (0 once all is bought).
    pub field_0x44: i32,
    /// +0xd4 the purchase animation counter (0 = none).
    pub field_0x48: i32,
    /// +0xd8, +0xdc the purchase shake.
    pub field_0x4c: i32,
    pub field_0x50: i32,
    /// +0xe0 something was bought.
    pub field_0x54: bool,
    /// +0xe1 the shop button was pressed (it stops flashing).
    pub field_0x55: bool,
    /// +0xe4 the level-info heading ("" after a bonus level).
    pub offset_0x5c: Vec<u8>,
    /// +0x100, +0x11c, +0x138, +0x154 the level-info rows: label, value, label, value.
    pub offset_0x78: Vec<u8>,
    pub offset_0x94: Vec<u8>,
    pub offset_0xb0: Vec<u8>,
    pub offset_0xcc: Vec<u8>,
    /// +0x170 the reward row's label.
    pub offset_0xe8: Vec<u8>,
}

impl G {
    pub fn bonus(&mut self, p: Ptr) -> &mut BonusScreen_data {
        match &mut self.widget(p).ext {
            WExt::BonusScreen(d) => d,
            e => panic!("{p} is not a BonusScreen: {e:?}"),
        }
    }
}

/// port: 0052dad0 BonusScreenOverlay::BonusScreenOverlay
/// `BonusScreen(WinFishApp*)`: bubbles in the results box; "Menu" (hidden after the final
/// boss level), the continue button ("Click for Story Time" in a Challenge, without "Menu")
/// and the shop button, all disabled for the first 30 updates; the level info and reward
/// (`FUN_0051c8e0`, which also pays it), counted at 15..50 updates' worth (at least 50 a
/// step); the shop price; the overlay.
pub fn BonusScreenOverlay(g: &mut G, param_1: Ptr) -> Ptr {
    let (wc, w) = crate::sexy::widget::Widget();
    let bm = crate::game::board_parts::BubbleMgr(g);
    let d = BonusScreen_data { field_0x0: param_1, offset_0x4: bm, ..Default::default() };
    let this = g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__BonusScreen_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::BonusScreen(Box::new(d)) }),
    });
    let (mut owc, mut ow) = crate::sexy::widget::Widget();
    ow.offset_0x1 = false;
    owc.offset_0x3c = true;
    let ov = g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::BonusScreenOverlay_vftable),
        node: Node::Widget(WidgetObj { wc: owc, w: ow, ext: WExt::GameSelectorOverlay(this) }),
    });
    g.bonus(this).offset_0x18 = ov;
    vcall!(g, ov, w.vfunction41, 0, 0, 0x280, 0x1e0);
    {
        let d = g.bonus(this);
        d.field_0x8 = 0xaf;
        d.field_0xc = 0xe6;
        d.field_0x10 = 300;
        d.field_0x14 = 0xe1;
    }
    let r = Rect::new(0xaf + 0x14, 0xe6 - 0xf, 300 - 0x28, 0xcd);
    FUN_00500120(g, bm, &r);
    FUN_00500180(g, bm, 10, 3);
    FUN_00512080(g, bm);

    let img = g.res.DAT_005e8c1c;
    let menu = FUN_00506820(g, 1, this, b"Menu", img);
    g.bonus(this).offset_0x1c = menu;
    let h = g.wc(menu).offset_0x38;
    vcall!(g, menu, w.vfunction41, 0x20d, 4, 0x50, h);
    let profile = g.wfa(param_1).offset_0x18c;
    if g.profile(profile).field_0x1c == 5 && g.profile(profile).field_0x20 == 2 {
        g.w(menu).offset_0x0 = false;
    }
    let cont = FUN_00506820(g, 0, this, b"Click Here To Continue", img);
    g.bonus(this).offset_0x20 = cont;
    let h = g.wc(cont).offset_0x38;
    vcall!(g, cont, w.vfunction41, 0xba, 0x1bd, 0x108, h);
    vcall!(g, cont, w.vfunction35, 0, FUN_00433360(0xff, 0xf0, 0));
    let f = g.res.DAT_005e8a5c;
    vcall!(g, cont, btn.vfunction72, f);
    if g.wfa(param_1).offset_0x150 == 4 && g.wfa(param_1).offset_0x4 != NULL {
        g.btn(cont).field_0x4 = b"Click for Story Time".to_vec();
        g.w(menu).offset_0x0 = false;
    }
    let shop_img = g.res.DAT_005e8ed8;
    let buy = FUN_00506820(g, 2, this, b"", shop_img);
    g.bonus(this).offset_0x24 = buy;
    vcall!(g, buy, w.vfunction35, 0, FUN_00433360(0xff, 0xff, 0xff));
    let h = g.wc(buy).offset_0x38;
    vcall!(g, buy, w.vfunction41, 0xd5, 200, 0xd1, h);
    vcall!(g, buy, w.vfunction32, true);

    let shells = g.profile(profile).field_0x48;
    {
        let d = g.bonus(this);
        d.field_0x54 = false;
        d.field_0x3c = 0x32;
        d.field_0x34 = 0;
        d.field_0x30 = 0;
        d.field_0x38 = shells;
    }
    FUN_0051c8e0(g, this);
    {
        let d = g.bonus(this);
        let reward = d.field_0x30;
        let steps = reward / d.field_0x3c;
        if steps < 0xf {
            d.field_0x3c = reward / 0xf;
        } else if 0x32 < steps {
            d.field_0x3c = reward / 0x32;
        }
        d.field_0x40 = 0;
        if d.field_0x3c < 0x32 {
            d.field_0x3c = 0x32;
        }
    }
    FUN_00516f70(g, this);
    for b in [menu, cont, buy] {
        vcall!(g, b, w.vfunction38, true);
    }
    let story = crate::game::win_fish_app::FUN_0054acf0(g, param_1);
    let d = g.bonus(this);
    d.offset_0x28 = NULL;
    d.field_0x4c = 0;
    d.field_0x50 = 0;
    d.field_0x48 = 0;
    d.field_0x55 = false;
    d.field_0x2c = story;
    this
}

/// port: 0051bb30 Sexy::BonusScreen::~BonusScreen
/// Deletes the bubbles, the overlay, the buttons and the sparkle cels.
pub fn dtor_BonusScreen(g: &mut G, this: Ptr) {
    let d = g.bonus(this).clone();
    if d.offset_0x4 != NULL {
        crate::game::board_parts::deleting_destructor__00505420(g, d.offset_0x4, 1);
    }
    for b in [d.offset_0x18, d.offset_0x20, d.offset_0x1c, d.offset_0x24] {
        if b != NULL {
            vcall!(g, b, w.vfunction1, 1);
        }
    }
    if d.offset_0x28 != NULL {
        g.free(d.offset_0x28);
    }
    crate::sexy::widget::dtor_Widget(g, this);
}

/// port: 0051c270 Sexy::BonusScreen::deleting_destructor
pub fn deleting_destructor(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_BonusScreen(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 00516fd0 Sexy::BonusScreen::vfunction21
/// `AddedToManager(WidgetManager*)`: the continue button, "Menu", the shop button, the
/// overlay.
pub fn vfunction21(g: &mut G, this: Ptr, param_1: Ptr) {
    crate::sexy::widget_container::vfunction21(g, this, param_1);
    let d = g.bonus(this).clone();
    for p in [d.offset_0x20, d.offset_0x1c, d.offset_0x24, d.offset_0x18] {
        vcall!(g, param_1, w.vfunction4, p);
    }
}

/// port: 00517030 Sexy::BonusScreen::vfunction22
/// `RemovedFromManager(WidgetManager*)`.
pub fn vfunction22(g: &mut G, this: Ptr, param_1: Ptr) {
    crate::sexy::widget_container::vfunction22(g, this, param_1);
    let d = g.bonus(this).clone();
    for p in [d.offset_0x20, d.offset_0x1c, d.offset_0x24, d.offset_0x18] {
        vcall!(g, param_1, w.vfunction5, p);
    }
}

/// port: 00517090 Sexy::BonusScreen::vfunction23
/// `Update()`: a held click skips the purchase animation to its end; the shop button
/// flashes while affordable (updates 11..179, then follows the mouse); at update 30 the
/// buttons are enabled. Otherwise: the purchase shake (the sparkle sound at its end), the
/// bubbles, redraw, and the reward counted into the balance, a rising tick every third
/// update.
pub fn vfunction23(g: &mut G, this: Ptr) {
    crate::sexy::widget_container::vfunction23(g, this);
    if g.w(this).offset_0x4 {
        let d = g.bonus(this);
        if 0 < d.field_0x48 && d.field_0x48 < 0x8c {
            d.field_0x48 = 0x8c;
        }
    }
    let app = g.bonus(this).field_0x0;
    let profile = g.wfa(app).offset_0x18c;
    let d = g.bonus(this).clone();
    if 0 < d.field_0x44 && d.field_0x44 <= g.profile(profile).field_0x48 && !d.field_0x55 {
        let n = g.wc(this).offset_0x24;
        if 10 < n && n < 0xb4 {
            g.w(d.offset_0x24).offset_0x5 = (n / 10) % 2 == 0;
        } else if n == 0xb4 {
            let wm = g.wc(this).offset_0xc;
            let (mx, my) = {
                let m = crate::sexy::widget_manager::wm(g, wm);
                (m.offset_0x8c, m.offset_0x90)
            };
            let over = vcall!(g, d.offset_0x24, w.vfunction69, mx, my);
            g.w(d.offset_0x24).offset_0x5 = over;
        }
    }
    if g.wc(this).offset_0x24 == 0x1e {
        vcall!(g, d.offset_0x1c, w.vfunction38, false);
        vcall!(g, d.offset_0x20, w.vfunction38, false);
        vcall!(g, d.offset_0x24, w.vfunction38, false);
        return;
    }
    let t = g.bonus(this).field_0x48;
    if 0 < t && t < 200 {
        let rx = (crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32) % 4;
        let ry = (crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32) % 2;
        let d = g.bonus(this);
        d.field_0x4c = rx;
        d.field_0x48 += 1;
        d.field_0x50 = ry;
        if d.field_0x48 == 0x8d {
            let s = g.res.DAT_005e8d44;
            crate::sexy::sexy_app_base::vfunction55(g, app, s);
        }
    }
    let bm = g.bonus(this).offset_0x4;
    FUN_005110e0(g, bm);
    vcall!(g, this, w.vfunction18);
    let n = g.wc(this).offset_0x24;
    let d = g.bonus(this).clone();
    if d.field_0x34 < d.field_0x30 && 0x1e < n && !d.field_0x54 {
        if n % 3 == 1 {
            g.bonus(this).field_0x40 += 1;
            // mSoundManager->GetSoundInstance(SOUND); AdjustPitch(+0xcc); Play(false, true).
            let id = g.res.DAT_005e8edc;
            let pitch = g.bonus(this).field_0x40 as f64;
            g.sound_requests.push(crate::sexy::sexy_app_base::SoundRequest { id, volume: 1.0, pan: 0, pitch });
        }
        let shells = g.profile(profile).field_0x48;
        let d = g.bonus(this);
        let mut step = d.field_0x3c;
        if d.field_0x30 < d.field_0x34 + step {
            step = d.field_0x30 - d.field_0x34;
        }
        d.field_0x38 += step;
        d.field_0x34 += step;
        if shells < d.field_0x38 {
            d.field_0x38 = shells;
        }
    }
}

/// port: 00517560 Sexy::BonusScreen::vfunction2
/// `ButtonPress(int id)`: the click sound; the shop button stops flashing (shows the mouse
/// state instead).
pub fn vfunction2(g: &mut G, this: Ptr, param_1: i32) {
    let app = g.bonus(this).field_0x0;
    let s = g.res.DAT_005e8c40;
    crate::sexy::sexy_app_base::vfunction55(g, app, s);
    if param_1 == 2 {
        let wm = g.wc(this).offset_0xc;
        let buy = g.bonus(this).offset_0x24;
        g.bonus(this).field_0x55 = true;
        let (mx, my) = {
            let m = crate::sexy::widget_manager::wm(g, wm);
            (m.offset_0x8c, m.offset_0x90)
        };
        let over = vcall!(g, buy, w.vfunction69, mx, my);
        g.w(buy).offset_0x5 = over;
    }
}

/// port: 00516f70 FUN_00516f70
/// The bonus shop price: 5000 x (bought + 4), at most 50000; nothing left after six (the
/// button hides).
pub fn FUN_00516f70(g: &mut G, this: Ptr) {
    let app = g.bonus(this).field_0x0;
    let profile = g.wfa(app).offset_0x18c;
    let n = g.profile(profile).field_0xb0;
    if 5 < n {
        g.bonus(this).field_0x44 = 0;
        let buy = g.bonus(this).offset_0x24;
        vcall!(g, buy, w.vfunction32, false);
        return;
    }
    let p = (n + 4) * 5000;
    g.bonus(this).field_0x44 = if 50000 < p { 50000 } else { p };
}

/// port: 0051c8e0 FUN_0051c8e0
/// The level info and the shell reward, then pays it to the profile. Adventure: the time
/// against the best (no heading after a bonus level), or after the final tank the surviving
/// pets and the playthrough score, 5000 per playthrough after the first (5000..25000).
/// Time Trial: the score against the best, 5% of it. Challenge: the time against the best,
/// 2000..20000 by tank. (Without a board: sample values, cycling.)
pub fn FUN_0051c8e0(g: &mut G, this: Ptr) {
    let app = g.bonus(this).field_0x0;
    let board = g.wfa(app).offset_0x4;
    let profile = g.wfa(app).offset_0x18c;
    g.bonus(this).offset_0x5c = b"Level Info".to_vec();
    g.bonus(this).offset_0xe8 = b"Bonus Reward".to_vec();
    if board == NULL {
        let k = g.globals.DAT_005e9628 % 4;
        let d = g.bonus(this);
        match k {
            0 => {
                d.offset_0x78 = b"Level Time".to_vec();
                d.offset_0x94 = b"23:10".to_vec();
                d.offset_0xb0 = b"Your Best Time".to_vec();
                d.offset_0xcc = b"15:23".to_vec();
            }
            1 => {
                d.offset_0x5c = b"Surviving Pets".to_vec();
                d.offset_0x78 = b"Your Score".to_vec();
                d.offset_0x94 = b"3".to_vec();
                d.offset_0xb0 = b"Your Best Score".to_vec();
                d.offset_0xcc = b"8".to_vec();
            }
            2 => {
                d.offset_0x78 = b"Your Score".to_vec();
                d.offset_0x94 = b"25,000".to_vec();
                d.offset_0xb0 = b"Your Best Score".to_vec();
                d.offset_0xcc = b"32,125".to_vec();
            }
            3 => d.offset_0x5c = Vec::new(),
            _ => {}
        }
        g.globals.DAT_005e9628 += 1;
        return;
    }
    let mode = g.wfa(app).offset_0x150;
    let (tank, level) = (g.board(board).field_0x33c, g.board(board).field_0x340);
    if mode == 0 {
        if tank != 5 {
            let reward = g.board(board).field_0x368;
            g.bonus(this).field_0x30 = reward;
            if 5 < level {
                g.bonus(this).offset_0x5c = Vec::new();
            }
            let time = g.board(board).field_0x370;
            let best = crate::game::profile::FUN_00501300(g.profile(profile), tank, level);
            let d = g.bonus(this);
            d.offset_0x78 = b"Level Time".to_vec();
            d.offset_0xb0 = b"Your Best Time".to_vec();
            d.offset_0x94 = crate::game::high_score_screen::FUN_00504c90(time);
            d.offset_0xcc = crate::game::high_score_screen::FUN_00504c90(best);
        } else {
            let mut n = g.profile(profile).field_0x50 - 1;
            if n < 1 {
                n = 1;
            }
            let reward = n * 5000;
            let pets = {
                use crate::game::board_level::vec_index;
                let b = g.board(board);
                b.offset_0xc[vec_index(0xb4)].len() as i32 + b.offset_0xc[vec_index(0xb0)].len() as i32
            };
            let best = crate::game::profile::FUN_00501300(g.profile(profile), 5, 1);
            let d = g.bonus(this);
            d.field_0x30 = if 25000 < reward { 25000 } else { reward };
            d.offset_0x5c = b"Surviving Pets".to_vec();
            d.offset_0x78 = b"Your Score".to_vec();
            d.offset_0xb0 = b"Your Best Score".to_vec();
            d.offset_0x94 = format!("{pets}").into_bytes();
            d.offset_0xcc = format!("{best}").into_bytes();
        }
    } else if mode == 1 {
        let score = g.board(board).field_0x364;
        let best = crate::game::profile::FUN_00501330(g.profile(profile), tank);
        let s = crate::sexy::app_host::FUN_004107c0(g, score);
        let b = crate::sexy::app_host::FUN_004107c0(g, best);
        let d = g.bonus(this);
        d.field_0x30 = (score * 5) / 100;
        d.offset_0xe8 = format!("{}% Bonus Award", 5).into_bytes();
        d.offset_0x78 = b"Your Score".to_vec();
        d.offset_0xb0 = b"Your Best Score".to_vec();
        d.offset_0x94 = s;
        d.offset_0xcc = b;
    } else if mode == 4 {
        let d = g.bonus(this);
        match tank {
            1 => d.field_0x30 = 2000,
            2 => d.field_0x30 = 5000,
            3 => d.field_0x30 = 10000,
            4 => d.field_0x30 = 20000,
            _ => {}
        }
        let time = g.board(board).field_0x370;
        let best = crate::game::profile::FUN_00501350(g.profile(profile), tank);
        let d = g.bonus(this);
        d.offset_0x78 = b"Level Time".to_vec();
        d.offset_0xb0 = b"Your Best Time".to_vec();
        d.offset_0x94 = crate::game::high_score_screen::FUN_00504c90(time);
        d.offset_0xcc = crate::game::high_score_screen::FUN_00504c90(best);
    }
    let reward = g.bonus(this).field_0x30;
    crate::game::profile::FUN_00501200(g, profile, reward);
}

/// port: 0051daa0 Sexy::BonusScreen::vfunction27
/// `Draw(Graphics*)`: the background, the title bar (and the Menu frame), the title by
/// mode, the bubbles, the shop window, then the purchase text or the results.
pub fn vfunction27(g: &mut G, this: Ptr, param_1: &mut Graphics) {
    let bg = g.res.DAT_005e8ab4;
    FUN_00455d20(param_1, g, bg, 0, 0);
    let bar = g.res.DAT_005e8d3c;
    let h = g.image(bar).offset_0x24;
    FUN_004563f0(param_1, g, &Rect::new(0x14, 0, 600, h), bar);
    let d = g.bonus(this).clone();
    if g.w(d.offset_0x1c).offset_0x0 {
        let mf = g.res.DAT_005e8e6c;
        let h = g.image(mf).offset_0x24;
        let m = g.wc(d.offset_0x1c).clone();
        FUN_004563f0(param_1, g, &Rect::new(m.offset_0x2c - 1, m.offset_0x30 - 1, m.offset_0x34 + 2, h), mf);
    }
    let f = g.res.DAT_005e8e40;
    FUN_00455880(param_1, f);
    FUN_00455890(param_1, CRect(0xff, 200, 0, 0xff));
    let title: &[u8] = match g.wfa(d.field_0x0).offset_0x150 {
        4 => b"CHALLENGE RESULTS",
        1 => b"TIME TRIAL RESULTS",
        _ => b"BONUS RESULTS",
    };
    vcall!(g, this, w.vfunction63, param_1, 0x19, title);
    FUN_00504b60(g, d.offset_0x4, param_1);
    FUN_0051ab80(g, this, param_1);
    if g.bonus(this).field_0x54 {
        FUN_0051cf30(g, this, param_1);
        return;
    }
    FUN_0051d560(g, this, param_1);
}

/// port: 0051ab80 FUN_0051ab80
/// The shop window: its frame (with the button), and what is for sale (or was just
/// bought): the bonus pet (shaking; gone once its sparkles start), the four or seven pet
/// tokens, or after everything the sold-out sign (the "all bought" one before any story
/// was unlocked); then the window's top.
pub fn FUN_0051ab80(g: &mut G, this: Ptr, param_1: &mut Graphics) {
    let d = g.bonus(this).clone();
    if g.w(d.offset_0x24).offset_0x0 {
        let f = g.res.DAT_005e8a04;
        FUN_00455d20(param_1, g, f, 200, 0xb2);
    }
    let token = g.res.DAT_005e8e38;
    let profile = g.wfa(d.field_0x0).offset_0x18c;
    let mut n = g.profile(profile).field_0xb0;
    if d.field_0x54 {
        n -= 1;
    }
    let top = |g: &mut G, gfx: &mut Graphics| {
        let t = g.res.DAT_005e8bf4;
        FUN_00455d20(gfx, g, t, 0xf0, 0x3c);
    };
    let (img, x, y);
    if n < 4 {
        if 0x8b < d.field_0x48 {
            top(g, param_1);
            return;
        }
        img = crate::sexy::res::FUN_005016a0(g, n + 0x6e) as Ptr;
        let (iw, ih) = (g.image(img).offset_0x20, g.image(img).offset_0x24);
        y = (0x9f - ih) / 2 + 0x35 + d.field_0x50;
        x = (0xb1 - iw) / 2 + 0xe6 + d.field_0x4c;
    } else if n < 6 {
        let (x0, w) = if n == 4 { (0x113, 0x57) } else { (0x10e, 0x61) };
        let (iw, ih) = (g.image(token).offset_0x20, g.image(token).offset_0x24);
        FUN_00455d20(param_1, g, token, x0, 0x51);
        FUN_00455d20(param_1, g, token, (w - iw) + x0, 0x51);
        FUN_00455d20(param_1, g, token, x0, 0xb4 - ih);
        FUN_00455d20(param_1, g, token, (w - iw) + x0, 0xb4 - ih);
        if n != 5 {
            top(g, param_1);
            return;
        }
        FUN_00455d20(param_1, g, token, (w - iw) / 2 - 0x3c + x0, (99 - ih) / 2 + 0x51);
        FUN_00455d20(param_1, g, token, (w - iw) / 2 + x0, (99 - ih) / 2 + 0x51);
        img = token;
        y = (99 - ih) / 2 + 0x51;
        x = (w - iw) / 2 + 0x3c + x0;
    } else {
        img = if g.profile(profile).field_0x7c == -1 { g.res.DAT_005e8dd4 } else { g.res.DAT_005e8ea0 };
        y = 0x42;
        x = 0xf0;
    }
    FUN_00455d20(param_1, g, img, x, y);
    top(g, param_1);
}

/// The shop button's text color (`FUN_00433320(0xffff00)`), the font of the "Buy" line.
fn buy_label(n: i32) -> Vec<u8> {
    if n < 4 {
        format!("Buy Bonus Pet #{}", n + 1).into_bytes()
    } else if n == 4 {
        b"Buy 4 Pet Limit".to_vec()
    } else if n == 5 {
        b"Buy 7 Pet Virtual Tank Limit".to_vec()
    } else {
        Vec::new()
    }
}

/// port: 0051dc70 Sexy::BonusScreen::vfunction45
/// `DrawOverlay(Graphics*)`: on the shop button (shifted a pixel while pressed) what it
/// buys, outlined (plainer for the 7-pet limit), and "for N Shells"; while the bought pet
/// sparkles, its eight cels flying apart.
pub fn vfunction45(g: &mut G, this: Ptr, param_1: &mut Graphics) {
    let d = g.bonus(this).clone();
    let profile = g.wfa(d.field_0x0).offset_0x18c;
    let n = g.profile(profile).field_0xb0;
    let buy = d.offset_0x24;
    if g.w(buy).offset_0x0 {
        let f = g.res.DAT_005e8b04;
        FUN_00455880(param_1, f);
        FUN_00455890(param_1, FUN_00433320(0xffff00));
        let text = buy_label(n);
        if n == 5 {
            let f = g.res.DAT_005e8cd0;
            FUN_00455880(param_1, f);
        }
        let fnt = FUN_00455870(param_1);
        let tw = font::string_width(g, fnt, &text);
        let bw = g.wc(buy).clone();
        let mut cx = bw.offset_0x34 / 2 + bw.offset_0x2c;
        let mut ty = bw.offset_0x30 + 0x14;
        let mut r = Rect::new(0xda, 0xe1, 200, 0);
        if g.w(buy).offset_0x4 && g.w(buy).offset_0x5 {
            cx += 1;
            ty = bw.offset_0x30 + 0x15;
            r = Rect::new(0xdb, 0xe2, 200, 0);
        }
        if FUN_00455870(param_1) == g.res.DAT_005e8b04 {
            let outline = g.res.DAT_005e8d1c;
            crate::game::help_screen::FUN_00500bf0(g, param_1, &text, cx - tw / 2, ty, outline, 0);
        } else {
            FUN_00455cf0(param_1, g, &text, cx - tw / 2, ty);
        }
        let f = g.res.DAT_005e8cf0;
        FUN_00455880(param_1, f);
        FUN_00455890(param_1, FUN_00433320(0xffffff));
        let price = crate::sexy::app_host::FUN_004107c0(g, d.field_0x44);
        let mut line = b"for ".to_vec();
        line.extend_from_slice(&price);
        line.extend_from_slice(b" Shells");
        vcall!(g, this, w.vfunction65, param_1, r, &line, -1, 0);
    }
    if d.offset_0x28 != NULL && 0x8b < d.field_0x48 && d.field_0x48 < 200 {
        const DX: [i32; 8] = [1, -11, -14, 12, -15, 11, 13, -1];
        const DY: [i32; 8] = [-10, -15, 3, -13, 10, 5, 14, 12];
        let t = d.field_0x48 - 0x8c;
        for i in 0..8 {
            FUN_00456950(param_1, g, d.offset_0x28, DX[i] * t + 0x10c, DY[i] * t + 0x46, i as i32);
        }
    }
}

/// port: 0051cf30 FUN_0051cf30
/// What was just bought: a bonus pet ("You Have Bought", then once it has sparkled its
/// name, animated portrait and description), or a pet limit ("Congratulations! You can now
/// use FOUR/SEVEN pets at the same time!", the 7-pet one with its fine print).
pub fn FUN_0051cf30(g: &mut G, this: Ptr, param_1: &mut Graphics) {
    let d = g.bonus(this).clone();
    let profile = g.wfa(d.field_0x0).offset_0x18c;
    let count = g.profile(profile).field_0xb0;
    let n = count - 1;
    let mut r = Rect::new(d.field_0x8 + 10, 0, d.field_0x10 - 0x14, d.field_0x14);
    let white = FUN_00433320(0xffffff);
    let gold = FUN_00433320(0xffc800);
    if n < 4 {
        let (name, desc, img, div): (&[u8], &[u8], Ptr, i32) = match n {
            0 => (
                b"BRINKLEY the Scuba Diving Elephant!",
                b"Likes: Peach muffins, all\nthings brown and sticky\nDislikes: Arugula",
                g.res.DAT_005e8b4c,
                4,
            ),
            1 => (
                b"NOSTRADAMUS the Nose!",
                b"Little known fact:  NOSTRADAMUS\nis the long lost nose of ex-president\nRutherford B. Hayes.",
                g.res.DAT_005e8ed4,
                2,
            ),
            2 => (
                b"STANLEY the Startlingly Small Sea Serpent!",
                b"STANLEY knows no fear.. except\nthat of badgers, aprons,\nand badgers wearing aprons.",
                g.res.DAT_005e8d00,
                2,
            ),
            3 => (
                b"WALTER the Penguin!",
                b"All shells used to purchase\nWALTER are donated to\nthe Falafel Foundation.\nFree the falafels!",
                g.res.DAT_005e8e50,
                2,
            ),
            _ => return,
        };
        r.mY = d.field_0xc + 0x14;
        let f = g.res.DAT_005e8cf0;
        FUN_00455880(param_1, f);
        FUN_00455890(param_1, white);
        vcall!(g, this, w.vfunction65, param_1, r, b"You Have Bought", -1, 0);
        if d.field_0x48 < 0x8d {
            return;
        }
        r.mY += 0x14;
        let f = g.res.DAT_005e8a5c;
        FUN_00455880(param_1, f);
        FUN_00455890(param_1, gold);
        vcall!(g, this, w.vfunction65, param_1, r, name, -1, 0);
        let t = g.wc(this).offset_0x24;
        FUN_00456980(param_1, g, img, 0x116, 0x5a, (t / div) % 10, 0);
        r.mY = d.field_0xc + 0x6e;
        let f = g.res.DAT_005e8cf0;
        FUN_00455880(param_1, f);
        FUN_00455890(param_1, white);
        vcall!(g, this, w.vfunction65, param_1, r, desc, -1, 0);
        return;
    }
    if 5 < n {
        return;
    }
    r.mY = d.field_0xc + 0x14;
    FUN_00455890(param_1, gold);
    let f = g.res.DAT_005e8e18;
    FUN_00455880(param_1, f);
    vcall!(g, this, w.vfunction65, param_1, r, b"Congratulations!", -1, 0);
    r.mY += 0x23;
    let f = g.res.DAT_005e8cf0;
    FUN_00455880(param_1, f);
    FUN_00455890(param_1, white);
    vcall!(g, this, w.vfunction65, param_1, r, b"You can now use", -1, 0);
    r.mY += 0x14;
    let f = g.res.DAT_005e8e40;
    FUN_00455880(param_1, f);
    FUN_00455890(param_1, gold);
    let (big, note): (&[u8], &[u8]) = if count == 5 {
        (b"FOUR", b"Four pets?!!!\nThat's INSANE!")
    } else if n == 5 {
        (b"SEVEN", b"")
    } else {
        (b"", b"")
    };
    vcall!(g, this, w.vfunction65, param_1, r, big, -1, 0);
    r.mY += 0x1e;
    let f = g.res.DAT_005e8cf0;
    FUN_00455880(param_1, f);
    FUN_00455890(param_1, white);
    vcall!(g, this, w.vfunction65, param_1, r, b"pets at the same time!", -1, 0);
    if n == 5 {
        let saved = FUN_00455870(param_1);
        let f = g.res.DAT_005e8c20;
        FUN_00455880(param_1, f);
        FUN_00455cf0(param_1, g, b"*", r.mX + 0xdf, r.mY + 5);
        r.mY += 0x37;
        FUN_00455cf0(param_1, g, b"*", r.mX + 0x3c, r.mY + 6);
        vcall!(g, this, w.vfunction65, param_1, r, b"Only applies to Virtual Tank.\nVoid where prohibited.", -1, 0);
        r.mY -= 0x23;
        FUN_00455880(param_1, saved);
    } else {
        r.mY += 0x23;
    }
    let f = g.res.DAT_005e8cf0;
    FUN_00455880(param_1, f);
    FUN_00455890(param_1, FUN_00433320(0xffffaa));
    vcall!(g, this, w.vfunction65, param_1, r, note, -1, 0);
}

/// One results row's rule: white, with a black line under it.
fn rule(g: &mut G, gfx: &mut Graphics, x: i32, y: i32, w: i32) {
    let _ = g;
    FUN_00455890(gfx, FUN_00433320(0xffffff));
    FUN_00455920(gfx, x, y, w, 1);
    FUN_00455890(gfx, FUN_00433320(0));
    FUN_00455920(gfx, x, y + 1, w, 1);
}

/// A value right-aligned at `right`, in yellow.
fn right_value(g: &mut G, gfx: &mut Graphics, s: &[u8], right: i32, y: i32) {
    FUN_00455890(gfx, FUN_00433320(0xffff00));
    let fnt = FUN_00455870(gfx);
    let w = font::string_width(g, fnt, s);
    FUN_00455cf0(gfx, g, s, right - w, y);
}

/// port: 0051d560 FUN_0051d560
/// The results: the level-info block (heading, rule, two label/value rows) when there is a
/// heading, then "Shells" with the reward counted so far and the new balance; after a bonus
/// level, "Keep playing to earn more shells!".
pub fn FUN_0051d560(g: &mut G, this: Ptr, param_1: &mut Graphics) {
    let d = g.bonus(this).clone();
    let (bx, by, bw) = (d.field_0x8, d.field_0xc, d.field_0x10);
    let mut r = Rect::new(bx + 0x14, 0, bw - 0x28, d.field_0x14);
    let lx = bx + 0x1e;
    let lw = bw - 0x46;
    let right = bw - 0x3c + (bx + 0x14);
    let white = FUN_00433320(0xffffff);
    let gold = FUN_00433320(0xffc800);
    if !d.offset_0x5c.is_empty() {
        r.mY = by + 0x14;
        let f = g.res.DAT_005e8a5c;
        FUN_00455880(param_1, f);
        FUN_00455890(param_1, gold);
        let h = vcall!(g, this, w.vfunction65, param_1, r, &d.offset_0x5c, -1, 0);
        let y = by + 0x14 + h;
        rule(g, param_1, lx, y, lw);
        let f = g.res.DAT_005e8cf0;
        FUN_00455880(param_1, f);
        FUN_00455890(param_1, white);
        FUN_00455cf0(param_1, g, &d.offset_0x78, lx, y + 0x14);
        right_value(g, param_1, &d.offset_0x94, right, y + 0x14);
        FUN_00455890(param_1, white);
        FUN_00455cf0(param_1, g, &d.offset_0xb0, lx, y + 0x28);
        right_value(g, param_1, &d.offset_0xcc, right, y + 0x28);
    }
    let top = if d.offset_0x5c.is_empty() { 0 } else { 0x50 } + 0x1e + by;
    r.mY = top;
    let f = g.res.DAT_005e8a5c;
    FUN_00455880(param_1, f);
    FUN_00455890(param_1, gold);
    let h = vcall!(g, this, w.vfunction65, param_1, r, b"Shells", -1, 0);
    let y = top + h;
    rule(g, param_1, lx, y, lw);
    FUN_00455890(param_1, white);
    let f = g.res.DAT_005e8cf0;
    FUN_00455880(param_1, f);
    FUN_00455890(param_1, white);
    FUN_00455cf0(param_1, g, &d.offset_0xe8, lx, y + 0x14);
    let counted = crate::sexy::app_host::FUN_004107c0(g, d.field_0x34);
    right_value(g, param_1, &counted, right, y + 0x14);
    FUN_00455890(param_1, white);
    FUN_00455cf0(param_1, g, b"New Balance", lx, y + 0x28);
    let balance = crate::sexy::app_host::FUN_004107c0(g, d.field_0x38);
    right_value(g, param_1, &balance, right, y + 0x28);
    if d.offset_0x5c.is_empty() {
        r.mY = 0x168;
        let f = g.res.DAT_005e8cf0;
        FUN_00455880(param_1, f);
        FUN_00455890(param_1, FUN_00433320(0xffffaa));
        vcall!(g, this, w.vfunction65, param_1, r, b"Keep playing to earn\nmore shells!", -1, 0);
    }
}

/// port: 0051e060 Sexy::BonusScreen::vfunction3
/// `ButtonDepress(int id)`: "Menu" (1) back to the main menu. Continue (0): after a
/// Challenge its story; after the final boss level the ending; else the next level (a Time
/// Trial, or a quick play, back to the menu). The shop (2): "Not Enough Shells", or asks to
/// confirm the purchase.
pub fn vfunction3(g: &mut G, this: Ptr, param_1: i32) {
    let app = g.bonus(this).field_0x0;
    match param_1 {
        1 => {
            FUN_0054ae30(g, app);
        }
        0 => {
            FUN_0054ae30(g, app);
            if g.wfa(app).offset_0x150 != 1 && g.wfa(app).offset_0xc == NULL {
                if g.wfa(app).offset_0x150 == 4 {
                    let story = g.bonus(this).field_0x2c;
                    crate::game::story_screen::FUN_00552e50(g, app, story);
                    return;
                }
                let profile = g.wfa(app).offset_0x18c;
                if g.profile(profile).field_0x1c == 5 && g.profile(profile).field_0x20 == 2 {
                    crate::game::interlude_screen::FUN_0054bd80(g, app);
                    return;
                }
                crate::game::win_fish_app::FUN_00552380(g, app, true, true);
                return;
            }
        }
        2 => {
            let profile = g.wfa(app).offset_0x18c;
            let p = g.profile(profile).clone();
            if p.field_0x48 < g.bonus(this).field_0x44 {
                let what = if 3 < p.field_0xb0 { "this Bonus Upgrade" } else { "this Bonus Pet" };
                let msg = format!("Sorry, but you need more Shells to purchase {what}.  Keep playing to earn more!");
                crate::game::win_fish_app::vfunction73(g, app, 0xe, true, b"Not Enough Shells", msg.as_bytes(), b"OK", 3);
            } else if p.field_0xb0 < 4 {
                crate::game::store::FUN_0054fdd0(g, app, b"Would you like to\nbuy this Bonus Pet?");
            } else {
                crate::game::store::FUN_0054fdd0(g, app, b"Would you like to\nbuy this Bonus Upgrade?");
            }
            return;
        }
        _ => return,
    }
    crate::game::win_fish_app::FUN_00552230(g, app);
}

/// port: 005172e0 FUN_005172e0
/// Completes a bonus purchase: the shop song; a bonus pet is earned and its eight sparkle
/// cels made (the pet drawn eight times, shaped by the sparkle mask) and animated; a limit
/// raises the virtual-tank pet limit to 4 or 7. Counts it, pays, hides the shop button and
/// saves.
pub fn FUN_005172e0(g: &mut G, this: Ptr) {
    let app = g.bonus(this).field_0x0;
    crate::game::win_fish_app::FUN_0054b020(g, app);
    crate::game::win_fish_app::FUN_0054b1a0(g, app, 2, 0x36, false);
    let old = g.bonus(this).offset_0x28;
    if old != NULL {
        g.free(old);
    }
    g.bonus(this).offset_0x28 = NULL;
    let profile = g.wfa(app).offset_0x18c;
    let n = g.profile(profile).field_0xb0;
    if (0..4).contains(&n) {
        crate::game::profile::FUN_00501420(g.profile(profile), 0x14 + n as usize, true);
        let mask = g.res.DAT_005e8a24;
        let (w, h) = (g.image(mask).offset_0x20, g.image(mask).offset_0x24);
        let img = crate::sexy::blit::new_memory_image(g, w, h);
        g.bonus(this).offset_0x28 = img;
        g.image(img).offset_0x2c = 8;
        crate::sexy::blit::render_offscreen(g, img, |g, gfx| {
            let mut x = 2;
            while x < 0x322 {
                let pet = crate::sexy::res::FUN_005016a0(g, n + 0x6e) as Ptr;
                FUN_00455d20(gfx, g, pet, x, 2);
                x += 100;
            }
        });
        let alpha = g.image(mask).mBits.clone();
        let bits = &mut g.image(img).mBits;
        let count = (w * h) as usize;
        for i in 0..count {
            bits[i] = bits[i] & 0xffffff | alpha[i] & 0xff00_0000;
        }
        g.bonus(this).field_0x48 = 1;
    }
    if 3 < n {
        let v = if n != 4 { 7 } else { 4 };
        if g.profile(profile).field_0xb4 < v {
            g.profile(profile).field_0xb4 = v;
        }
    }
    g.profile(profile).field_0xb0 += 1;
    let price = g.bonus(this).field_0x44;
    crate::game::profile::FUN_00501200(g, profile, -price);
    g.bonus(this).field_0x54 = true;
    let buy = g.bonus(this).offset_0x24;
    vcall!(g, buy, w.vfunction32, false);
    crate::game::win_fish_app::FUN_0054afd0(g, app);
}

/// port: 0054ae30 FUN_0054ae30
/// `RemoveBonusScreen()` (+0x818).
pub fn FUN_0054ae30(g: &mut G, this: Ptr) {
    let s = g.wfa(this).offset_0xec;
    if s != NULL {
        let wm = g.sab(this).offset_0x318;
        vcall!(g, wm, w.vfunction5, s);
        crate::sexy::sexy_app_base::vfunction35(g, this, s);
        g.wfa(this).offset_0xec = NULL;
    }
}

/// port: 0054bf60 FUN_0054bf60
/// `ShowBonusScreen()` after a bonus level, Time Trial or Challenge: closes the dialogs,
/// marks a play in progress, stops the board's level-end state, records the playthrough
/// (`FUN_00514b00`), opens the screen (+0x818), saves, and starts the shop song.
pub fn FUN_0054bf60(g: &mut G, this: Ptr) {
    crate::game::win_fish_app::FUN_0054b4c0(g, this);
    g.wfa(this).offset_0x158 = true;
    let board = g.wfa(this).offset_0x4;
    if board != NULL {
        g.board(board).ext_0x4ee = false;
    }
    FUN_0054ae30(g, this);
    let profile = g.wfa(this).offset_0x18c;
    if profile != NULL {
        crate::game::profile::FUN_00514b00(g, profile);
    }
    let s = BonusScreenOverlay(g, this);
    g.wfa(this).offset_0xec = s;
    let (w, h) = (g.sab(this).field_0xb8, g.sab(this).field_0xbc);
    vcall!(g, s, w.vfunction41, 0, 0, w, h);
    let wm = g.sab(this).offset_0x318;
    vcall!(g, wm, w.vfunction4, s);
    crate::game::win_fish_app::FUN_0054afd0(g, this);
    crate::game::win_fish_app::FUN_0054bc30(g, this);
    crate::game::win_fish_app::FUN_0054b020(g, this);
    crate::game::win_fish_app::FUN_0054b1a0(g, this, 2, 0x36, false);
}
