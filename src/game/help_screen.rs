//! `Sexy::HelpScreen`: the instructions shown before the first game ("Click Here To
//! Continue", page 0: FEED ME / FEAR ME / FIND ME) and the eight-page help reached from the
//! menu (Next / Previous / Menu or "Back to Game"), over rising bubbles and passing fish.
//!
//! `HelpScreen_data` starts at object offset 0x8c; the object is 0xac bytes, one int
//! larger than the database's 0xa8 (`ext_0xa8`).

use crate::game::board_parts::{FUN_00500120, FUN_00500150, FUN_00500180, FUN_005001a0, FUN_00504a20, FUN_00504b60, FUN_005110e0, FUN_00512080, DAT_0059be10};
use crate::game::game_selector::{FUN_00506760, FUN_00506820};
use crate::sexy::graphics::{
    FUN_004558c0, FUN_004558e0, FUN_00455870, FUN_00455880, FUN_00455890, FUN_004558b0, FUN_00455920, FUN_00455990, FUN_00455ac0,
    FUN_00455c50, FUN_00455cf0, FUN_00455d20, FUN_00455e40, FUN_00456950, FUN_00456980, FUN_004560a0, FUN_004563f0,
};
use crate::sexy::image_font as font;
use crate::sexy::prelude::*;
use crate::sexy::types::CRect;

/// `HelpScreen_data` (object offset 0x8c) plus the int past the database's class size.
#[derive(Debug, Clone, Default)]
pub struct HelpScreen_data {
    /// +0x8c the app.
    pub field_0x0: Ptr,
    /// +0x90 the bubbles and fish.
    pub offset_0x4: Ptr,
    /// +0x94 "Click Here To Continue" (id 0; first time only).
    pub offset_0x8: Ptr,
    /// +0x98 "Menu" / "Back to Game" (id 1).
    pub offset_0xc: Ptr,
    /// +0x9c "Next" (id 2).
    pub offset_0x10: Ptr,
    /// +0xa0 "Previous" (id 3).
    pub offset_0x14: Ptr,
    /// +0xa4 the page: 0 = first-time instructions, 1..8 = help pages.
    pub offset_0x18: i32,
    /// +0xa8 the page the bubble manager was last set up for (-1 = none).
    pub ext_0xa8: i32,
}

impl G {
    pub fn help(&mut self, p: Ptr) -> &mut HelpScreen_data {
        match &mut self.widget(p).ext {
            WExt::HelpScreen(d) => d,
            e => panic!("{p} is not a HelpScreen: {e:?}"),
        }
    }
}

/// port: 00520950 Sexy::HelpScreen::HelpScreen
/// `HelpScreen(WinFishApp*, bool firstTime)`.
pub fn HelpScreen(g: &mut G, param_1: Ptr, param_2: bool) -> Ptr {
    let (wc, w) = crate::sexy::widget::Widget();
    let d = HelpScreen_data { field_0x0: param_1, ..Default::default() };
    let this = g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__HelpScreen_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::HelpScreen(Box::new(d)) }),
    });
    let bm = crate::game::board_parts::BubbleMgr(g);
    g.help(this).offset_0x4 = bm;

    let b = FUN_00506760(g, 0, this, b"Click Here To Continue", NULL);
    g.help(this).offset_0x8 = b;
    let h = g.wc(b).offset_0x38;
    vcall!(g, b, w.vfunction41, 0xd2, 0x19f, 0xdc, h);
    let c = FUN_00433360(0xff, 0xf0, 0);
    vcall!(g, b, w.vfunction35, 0, c);

    let img = g.res.DAT_005e8c1c;
    let menu = FUN_00506820(g, 1, this, b"Menu", img);
    g.help(this).offset_0xc = menu;
    let h = g.wc(menu).offset_0x38;
    vcall!(g, menu, w.vfunction41, 0x20d, 4, 0x50, h);

    let prev = FUN_00506820(g, 3, this, b"Previous", img);
    g.help(this).offset_0x14 = prev;
    let h = g.wc(menu).offset_0x38;
    vcall!(g, prev, w.vfunction41, 0xbe, 0x1ae, 0x78, h);

    let next = FUN_00506820(g, 2, this, b"Next", img);
    g.help(this).offset_0x10 = next;
    crate::sexy::widget::FUN_0046e880(g, next, 0x8403, prev, 0x14, 0, 0, 0);

    if !param_2 {
        g.help(this).offset_0x18 = g.globals.DAT_005e15f8;
        let cont = g.help(this).offset_0x8;
        g.w(cont).offset_0x0 = false;
        if g.wfa(param_1).offset_0x4 != NULL {
            let menu = g.help(this).offset_0xc;
            let h = g.wc(menu).offset_0x38;
            vcall!(g, menu, w.vfunction41, 0x1e0, 4, 0x7d, h);
            g.btn(menu).field_0x4 = b"Back to Game".to_vec();
        }
        FUN_00500120(g, bm, &Rect::new(0, 0x50, 0x280, 400));
        FUN_00500150(g, bm, &Rect::new(0, 0x50, 0x280, 0x15e));
    } else {
        g.help(this).offset_0x18 = 0;
        let (next, prev) = (g.help(this).offset_0x10, g.help(this).offset_0x14);
        g.w(next).offset_0x0 = false;
        g.w(prev).offset_0x0 = false;
        FUN_00500120(g, bm, &Rect::new(0, 0x3c, 0x280, 0x1a4));
    }
    g.help(this).ext_0xa8 = -1;
    FUN_00500180(g, bm, 0x14, 3);
    FUN_00512080(g, bm);
    this
}

/// port: 00517d00 Sexy::HelpScreen::~HelpScreen
/// Remembers the page for next time, then deletes the bubbles and the four buttons.
pub fn dtor_HelpScreen(g: &mut G, this: Ptr) {
    let d = g.help(this).clone();
    if 0 < d.offset_0x18 {
        g.globals.DAT_005e15f8 = d.offset_0x18;
    }
    if d.offset_0x4 != NULL {
        crate::game::board_parts::deleting_destructor__00505420(g, d.offset_0x4, 1);
    }
    for b in [d.offset_0x8, d.offset_0xc, d.offset_0x10, d.offset_0x14] {
        if b != NULL {
            vcall!(g, b, w.vfunction1, 1);
        }
    }
    crate::sexy::widget::dtor_Widget(g, this);
}

/// port: 0051ae10 Sexy::HelpScreen::deleting_destructor_for_Widget
pub fn deleting_destructor_for_Widget(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_HelpScreen(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 00517dd0 Sexy::HelpScreen::vfunction21_for_Widget
/// `AddedToManager(WidgetManager*)`: adds the four buttons.
pub fn vfunction21_for_Widget(g: &mut G, this: Ptr, param_1: Ptr) {
    crate::sexy::widget_container::vfunction21(g, this, param_1);
    let d = g.help(this).clone();
    for b in [d.offset_0xc, d.offset_0x8, d.offset_0x10, d.offset_0x14] {
        vcall!(g, param_1, w.vfunction4, b);
    }
}

/// port: 00517e30 Sexy::HelpScreen::vfunction22_for_Widget
/// `RemovedFromManager(WidgetManager*)`: removes the four buttons.
pub fn vfunction22_for_Widget(g: &mut G, this: Ptr, param_1: Ptr) {
    crate::sexy::widget_container::vfunction22(g, this, param_1);
    let d = g.help(this).clone();
    for b in [d.offset_0xc, d.offset_0x8, d.offset_0x10, d.offset_0x14] {
        vcall!(g, param_1, w.vfunction5, b);
    }
}

/// port: 00518240 Sexy::HelpScreen::vfunction23_for_Widget
/// `Update()`: on a page change the fish come (credits page, 3 at 3%) or go (scattering);
/// the bubbles update every frame.
pub fn vfunction23_for_Widget(g: &mut G, this: Ptr) {
    crate::sexy::widget_container::vfunction23(g, this);
    let d = g.help(this).clone();
    if d.ext_0xa8 != d.offset_0x18 {
        g.help(this).ext_0xa8 = d.offset_0x18;
        if d.offset_0x18 == 8 {
            FUN_005001a0(g, d.offset_0x4, 3, 3);
        } else {
            FUN_005001a0(g, d.offset_0x4, 0, 0);
            FUN_00504a20(g, d.offset_0x4);
        }
    }
    FUN_005110e0(g, d.offset_0x4);
    vcall!(g, this, w.vfunction18);
}

/// port: 00523c80 Sexy::HelpScreen::vfunction27_for_Widget
/// `Draw(Graphics*)`: the backdrop (page 0: a picture; else a blue gradient with bubbles,
/// fish, the sand strip and, except on the credits, a translucent panel ruled into three
/// rows), the page, its heading and "Page n of 8", then the title bar and the menu
/// button's frame.
pub fn vfunction27_for_Widget(g: &mut G, this: Ptr, param_1: &mut Graphics) {
    let page = g.help(this).offset_0x18;
    if page == 0 {
        let img = g.res.DAT_005e8c70;
        FUN_004563f0(param_1, g, &Rect::new(-5, -5, 0x28a, 0x1ea), img);
    } else {
        let app = g.help(this).field_0x0;
        let mut y = 0;
        let mut l = 0x6b;
        loop {
            let c = crate::sexy::sexy_app_base::FUN_00489530(g, app, 0x97, 0x8a, l);
            FUN_00455890(param_1, FUN_00433320(c));
            let w = g.wc(this).offset_0x34;
            FUN_00455920(param_1, 0, y, w, 4);
            y += 4;
            l -= 1;
            if l < 0x32 {
                l = 0x32;
            }
            if 0x1e0 <= y {
                break;
            }
        }
        let bm = g.help(this).offset_0x4;
        FUN_00504b60(g, bm, param_1);
        let cnt = g.wc(this).offset_0x24;
        let page = g.help(this).offset_0x18;
        FUN_00500d40(g, param_1, if page <= 0 { 0x3c } else { 0x50 }, cnt);
        if g.help(this).offset_0x18 != 8 {
            let r0 = FUN_0051ae40(0);
            let r1 = FUN_0051ae40(1);
            let r3 = FUN_0051ae40(3);
            let gap = ((r1.mY - r0.mHeight) - r0.mY) / 2;
            let top = r0.mY - gap;
            let panel = Rect::new(r0.mX, top, r0.mWidth, (r3.mY + r3.mHeight + gap) - top);
            FUN_00455890(param_1, CRect(0, 0, 0, 0x1e));
            FUN_00455990(param_1, &panel);
            FUN_00455890(param_1, FUN_00433320(0x88aaaa));
            FUN_00455ac0(param_1, &panel);
            for i in 0..3 {
                let r = FUN_0051ae40(i);
                FUN_00455920(param_1, r.mX, r.mHeight + r.mY + gap, r.mWidth, 1);
            }
        }
    }
    match g.help(this).offset_0x18 {
        0 => FUN_00521480(g, this, param_1),
        1 => FUN_00521660(g, this, param_1),
        2 => FUN_00521ad0(g, this, param_1),
        3 => FUN_005220a0(g, this, param_1),
        4 => FUN_00522460(g, this, param_1),
        5 => FUN_005228e0(g, this, param_1),
        6 => FUN_00522e40(g, this, param_1),
        7 => FUN_00523600(g, this, param_1),
        8 => FUN_005239d0(g, this, param_1),
        _ => {}
    }
    let page = g.help(this).offset_0x18;
    if 0 < page {
        let name: &[u8] = match page {
            1 => b"THE BASICS",
            2 => b"UPGRADES",
            3 => b"PETS",
            4 => b"GAME MODES",
            5 => b"VIRTUAL TANK",
            6 => b"VIRTUAL TANK FISH",
            7 => b"CONFIGURING VIRTUAL TANK",
            8 => b"CREDITS",
            _ => b"",
        };
        let outline = g.res.DAT_005e8d1c;
        let f = g.res.DAT_005e8b04;
        FUN_00455880(param_1, f);
        FUN_00455890(param_1, FUN_00433320(0xffffff));
        let f = FUN_00455870(param_1);
        let w = font::string_width(g, f, name);
        FUN_00500bf0(g, param_1, name, 0x140 - w / 2, 0x3c, outline, 0);
        FUN_00455890(param_1, FUN_00433320(0xffffff));
        let s = format!("Page {page} of 8").into_bytes();
        FUN_00455890(param_1, FUN_00433320(0xffffff));
        let f = FUN_00455870(param_1);
        let w = font::string_width(g, f, &s);
        FUN_00500bf0(g, param_1, &s, 0x140 - w / 2, 0x50, outline, 0);
    }
    let bar = g.res.DAT_005e8d3c;
    let h = g.image(bar).offset_0x24;
    FUN_004563f0(param_1, g, &Rect::new(0x14, 0, 600, h), bar);
    let frame = g.res.DAT_005e8e6c;
    let h = g.image(frame).offset_0x24;
    let menu = g.help(this).offset_0xc;
    let m = g.wc(menu).clone();
    FUN_004563f0(param_1, g, &Rect::new(m.offset_0x2c - 1, m.offset_0x30 - 1, m.offset_0x34 + 2, h), frame);
    let f = g.res.DAT_005e8e40;
    FUN_00455880(param_1, f);
    FUN_00455890(param_1, CRect(0xff, 200, 0, 0xff));
    let title: &[u8] = if g.help(this).offset_0x18 != 0 { b"Insaniquarium Help" } else { b"Instructions" };
    vcall!(g, this, w.vfunction63, param_1, 0x19, title);
}

/// port: 0051a6f0 Sexy::HatchScreen::vfunction2
/// `ButtonPress(int)` shared by the full-screen menus: the click sound (app vtable +0xd8
/// `PlaySample`). The listener's app pointer is the first field of each class's data.
pub fn vfunction2(g: &mut G, this: Ptr, _id: i32) {
    let app = match &g.widget(this).ext {
        WExt::HelpScreen(d) => d.field_0x0,
        WExt::HatchScreen(d) => d.field_0x0,
        WExt::Store(d) => d.offset_0x0,
        WExt::PetsScreen(d) => d.field_0x0,
        WExt::InterludeScreen(d) => d.offset_0x0,
        WExt::TankScreen(d) => d.field_0x0,
        WExt::StoryScreen(d) => d.field_0x0,
        WExt::HighScoreScreen(d) => d.field_0x0,
        e => panic!("not ported yet: ButtonPress for {e:?} @ 0051a6f0"),
    };
    let s = g.res.DAT_005e8c40;
    crate::sexy::sexy_app_base::vfunction55(g, app, s);
}

/// port: 005182a0 Sexy::HelpScreen::vfunction3_for_ButtonListener
/// `ButtonDepress(int)`: 1 Menu/"Back to Game" (back to the selector when no game is up),
/// 0 continue into the new game, 2/3 next/previous page (wrapping 1..8).
pub fn vfunction3_for_ButtonListener(g: &mut G, this: Ptr, param_1: i32) {
    let app = g.help(this).field_0x0;
    if param_1 == 1 {
        crate::game::win_fish_app::FUN_0054af10(g, app);
        if g.wfa(app).offset_0x4 == NULL {
            crate::game::win_fish_app::FUN_00552100(g, app);
        }
    } else if param_1 == 0 {
        crate::game::win_fish_app::FUN_0054cbf0(g, app);
        crate::game::win_fish_app::FUN_0054af10(g, app);
    } else if param_1 == 2 {
        let d = g.help(this);
        d.offset_0x18 += 1;
        if 8 < d.offset_0x18 {
            d.offset_0x18 = 1;
        }
    } else if param_1 == 3 {
        let d = g.help(this);
        d.offset_0x18 -= 1;
        if d.offset_0x18 < 1 {
            d.offset_0x18 = 8;
        }
    }
}

/// port: 0051ae40 FUN_0051ae40
/// The rect of help row `n` (in ECX; result through EAX).
pub fn FUN_0051ae40(param_1: i32) -> Rect {
    Rect::new(10, param_1 * 0x41 + 0xa0, 0x26c, 0x32)
}

/// port: 00500d40 FUN_00500d40
/// Draws the animated sand strip (IMAGE `DAT_005e8af4`, additive) across the screen at `y`.
pub fn FUN_00500d40(g: &mut G, param_1: &mut Graphics, param_2: i32, param_3: i32) {
    FUN_004558c0(param_1, 1);
    let img = g.res.DAT_005e8af4;
    let cel = crate::sexy::image::FUN_004578a0(g, img, param_3);
    for x in [0, 0xa0, 0x140, 0x1e0] {
        let img = g.res.DAT_005e8af4;
        FUN_00456950(param_1, g, img, x, param_2, cel);
    }
    FUN_004558c0(param_1, 0);
}

/// port: 00500bf0 FUN_00500bf0
/// `DrawStringWithOutline(Graphics*, const string&, int x, int y, Font* outline, int color)`:
/// the outline font in `color`, then the current font and color over it.
pub fn FUN_00500bf0(g: &mut G, param_1: &mut Graphics, param_2: &[u8], param_3: i32, param_4: i32, param_5: Ptr, param_6: u32) {
    let f = FUN_00455870(param_1);
    let c = FUN_004558b0(param_1);
    FUN_00455880(param_1, param_5);
    FUN_00455890(param_1, FUN_00433320(param_6));
    FUN_00455cf0(param_1, g, param_2, param_3, param_4);
    FUN_00455880(param_1, f);
    FUN_00455890(param_1, c);
    FUN_00455cf0(param_1, g, param_2, param_3, param_4);
}

/// port: 00521480 FUN_00521480
/// Page 0: the three panels (FEED ME, FEAR ME, FIND ME) with vine borders and corners.
pub fn FUN_00521480(g: &mut G, this: Ptr, param_1: &mut Graphics) {
    FUN_00520ee0(g, this, param_1, &Rect::new(0x1e, 0x46, 0xb9, 0x140));
    FUN_00521170(g, this, param_1, &Rect::new(0xe3, 0x46, 0xb9, 0x140));
    FUN_005213a0(g, this, param_1, &Rect::new(0x1a8, 0x46, 0xb9, 0x140));
    let h = g.res.DAT_005e8a58;
    FUN_00503390(g, param_1, h, 0xd, 0xe6, 0x17);
    FUN_00503390(g, param_1, h, 0xd1, 0xe6, 0x18);
    FUN_00503390(g, param_1, h, 0x196, 0xe6, 0x18);
    FUN_00503390(g, param_1, h, 0x25b, 0xe6, 0x18);
    let v = g.res.DAT_005e8cbc;
    FUN_00503400(g, param_1, v, 0x78, 0x21, 0x28);
    FUN_00503400(g, param_1, v, 0x203, 0x21, 0x28);
    FUN_00503400(g, param_1, v, 0x78, 0x182, 0x53);
    FUN_00503400(g, param_1, v, 0x203, 0x182, 0x53);
    let k = g.res.DAT_005e8b18;
    for (x, y) in [(0x69, 0x35), (0x87, 0x35), (500, 0x35), (0x212, 0x35), (0x69, 0x196), (0x87, 0x196), (500, 0x196), (0x212, 0x196)] {
        FUN_00455d20(param_1, g, k, x, y);
    }
}

/// port: 00503390 FUN_00503390
/// Tiles `image` horizontally over `width` pixels from (x, y) (the last tile cut short).
pub fn FUN_00503390(g: &mut G, param_1: &mut Graphics, param_2: Ptr, mut param_3: i32, param_4: i32, param_5: i32) {
    let mut done = 0;
    if 0 < param_5 {
        let mut iw = g.image(param_2).offset_0x20;
        loop {
            let mut w = param_5 - done;
            if iw <= param_5 - done {
                w = iw;
            }
            let src = Rect::new(0, 0, w, g.image(param_2).offset_0x24);
            FUN_00455e40(param_1, g, param_2, param_3, param_4, &src);
            iw = g.image(param_2).offset_0x20;
            param_3 += w;
            done += iw;
            if param_5 <= done {
                break;
            }
        }
    }
}

/// port: 00503400 FUN_00503400
/// Tiles `image` vertically over `height` pixels from (x, y) (the last tile cut short).
pub fn FUN_00503400(g: &mut G, param_1: &mut Graphics, param_2: Ptr, param_3: i32, mut param_4: i32, param_5: i32) {
    let mut done = 0;
    if 0 < param_5 {
        let mut ih = g.image(param_2).offset_0x24;
        loop {
            let mut h = param_5 - done;
            if ih <= param_5 - done {
                h = ih;
            }
            let src = Rect::new(0, 0, g.image(param_2).offset_0x20, h);
            FUN_00455e40(param_1, g, param_2, param_3, param_4, &src);
            ih = g.image(param_2).offset_0x24;
            param_4 += h;
            done += ih;
            if param_5 <= done {
                break;
            }
        }
    }
}

/// port: 00518080 FUN_00518080
/// A help panel's frame (IMAGE `DAT_005e8c58` stretched as a box).
pub fn FUN_00518080(g: &mut G, param_1: &mut Graphics, param_2: &Rect) {
    let img = g.res.DAT_005e8c58;
    FUN_004563f0(param_1, g, param_2, img);
}

/// port: 005036e0 FUN_005036e0
/// Linear interpolation from `a` to `b` over `n` steps at step `t` (clamped), reversed
/// when `flip` is set.
pub fn FUN_005036e0(mut param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: bool) -> i32 {
    let mut b = param_2;
    if param_5 {
        b = param_1;
        param_1 = param_2;
    }
    if 0 < param_3 {
        if param_3 < param_4 {
            return ((param_4 - param_3) * param_1 + b * param_3) / param_4;
        }
        return b;
    }
    param_1
}

/// `float` angle in radians of `deg` degrees, the way the x87 code rounds it.
fn rad_f32(deg: f64) -> f32 {
    ((deg * DAT_0059be10) / 180.0) as f32
}

/// The CRT `sin`/`cos` on a float argument, stored back to a float.
fn sin_f32(a: f32) -> f32 {
    (a as f64).sin() as f32
}
fn cos_f32(a: f32) -> f32 {
    (a as f64).cos() as f32
}

/// port: 00520ee0 FUN_00520ee0
/// FEED ME: the hand drops food that sinks while a guppy swims over, eats it (eat strip),
/// turns (turn strip) and swims back; a 115-update cycle that alternates direction.
pub fn FUN_00520ee0(g: &mut G, this: Ptr, param_1: &mut Graphics, param_2: &Rect) {
    let ftol = crate::sexy::crt::ftol;
    FUN_00518080(g, param_1, param_2);
    let food_top = param_2.mY + 0x1e;
    let left = param_2.mX + 0x32;
    let t = g.wc(this).offset_0x24;
    let right = param_2.mWidth + param_2.mX - 0x32;
    let s = sin_f32(rad_f32((t * 5) as f64));
    let fish_y = ftol((param_2.mY + 0x78) as f64 + s as f64 * 3.0) as i32;
    let m = t % 0x73;
    let flip = (t / 0x73) % 2 != 0;
    let hand_x = FUN_005036e0(left, right, m, 0x1e, flip);
    if 0x27 < m && m < 0x55 {
        let food_y = FUN_005036e0(food_top, fish_y, m - 0x28, 0x2d, false);
        let img = g.res.DAT_005e8aa0;
        FUN_00456980(param_1, g, img, hand_x - 10, food_y - 0x14, (t / 4) % 10, 0);
    }
    let fish_x = FUN_005036e0(left + 0xf, right - 10, m - 0x2d, 0x28, flip);
    let mut src = Rect::new(((t / 4) % 10) * 0x50, 0, 0x50, 0x50);
    let mut mirror = flip;
    let mut img = g.res.DAT_005e8ab8;
    if 0x2c < m {
        if m < 0x41 {
            src.mX = ((m - 0x2d) / 2) * 0x50;
            img = g.res.DAT_005e8d84;
        } else {
            mirror = !flip;
            if 0x4a < m && m < 0x5f {
                src.mX = ((m - 0x4b) / 2) * 0x50;
                img = g.res.DAT_005e8ec0;
            }
        }
    }
    FUN_004560a0(param_1, g, img, fish_x - 0x25, fish_y - 0x28, &src, mirror);
    let click = 0x27 < m && m < 0x3c;
    FUN_00517f70(g, param_1, hand_x, food_top, click);
    FUN_00520cf0(g, this, param_1, param_2, b"FEED ME", &[b"Click on the tank", b"to drop food", b"for your fish."], 3);
}

/// port: 00521170 FUN_00521170
/// FEAR ME: a bobbing alien; the hand circles and clicks it every 40 updates (the alien
/// flashes white and the click mark shows).
pub fn FUN_00521170(g: &mut G, this: Ptr, param_1: &mut Graphics, param_2: &Rect) {
    let ftol = crate::sexy::crt::ftol;
    FUN_00518080(g, param_1, param_2);
    let t = g.wc(this).offset_0x24;
    let a = ((t as f64 * 3.0 * DAT_0059be10) / 180.0) as f32;
    let phase = t % 0x28;
    let cx = param_2.mWidth / 2 + param_2.mX;
    let top = param_2.mY + 0x14;
    let s = sin_f32(a);
    let alien_y = ftol(s as f64 * 5.0 + top as f64) as i32;
    let (mut hand_x, mut hand_y) = (cx, top + 0x3c);
    let (mut mark_x, mut mark_y) = (cx, top + 0x3c);
    let v = phase - 0x14;
    FUN_005180a0(t, &mut hand_x, &mut hand_y);
    FUN_005180a0(t - v, &mut mark_x, &mut mark_y);
    let src = Rect::new(((t / 3) % 10) * 0xa0, 0, 0xa0, 0xa0);
    let img = g.res.DAT_005e8ebc;
    FUN_00455e40(param_1, g, img, cx - 0x50, alien_y, &src);
    if -1 < v {
        if v < 10 {
            FUN_004558c0(param_1, 1);
            FUN_004558e0(param_1, true);
            FUN_00455890(param_1, CRect(0xff, 0xff, 0xff, (10 - v) * 10));
            FUN_00455e40(param_1, g, img, cx - 0x50, alien_y, &src);
            FUN_004558c0(param_1, 0);
            FUN_004558e0(param_1, false);
        }
        if v < 0x14 {
            FUN_00517e90(param_1, mark_x, mark_y);
        }
    }
    FUN_00517f70(g, param_1, hand_x, hand_y, false);
    FUN_00520cf0(g, this, param_1, param_2, b"FEAR ME", &[b"Use the mouse to", b"zap Aliens!"], 2);
}

/// port: 005213a0 FUN_005213a0
/// FIND ME: the egg pieces.
pub fn FUN_005213a0(g: &mut G, this: Ptr, param_1: &mut Graphics, param_2: &Rect) {
    FUN_00518080(g, param_1, param_2);
    let img = g.res.DAT_005e8c00;
    let w = g.image(img).offset_0x20;
    FUN_00455d20(param_1, g, img, (param_2.mWidth - w) / 2 + param_2.mX, param_2.mY + 0x50);
    FUN_00520cf0(
        g,
        this,
        param_1,
        param_2,
        b"FIND ME",
        &[b"Collect all 3 pieces", b"of the egg to", b"advance a level and", b"gain a new pet!"],
        4,
    );
}

/// port: 00520cf0 FUN_00520cf0
/// A help panel's caption: the outlined yellow title 100 pixels above the panel's bottom,
/// then `n` centered white lines 15 apart below it.
pub fn FUN_00520cf0(g: &mut G, _this: Ptr, param_1: &mut Graphics, param_2: &Rect, param_3: &[u8], param_4: &[&[u8]], param_5: i32) {
    let mut y = param_2.mHeight - 100 + param_2.mY;
    let cx = param_2.mWidth / 2 + param_2.mX;
    let f = g.res.DAT_005e8b04;
    FUN_00455880(param_1, f);
    FUN_00455890(param_1, FUN_00433320(0xffff00));
    let f = FUN_00455870(param_1);
    let w = font::string_width(g, f, param_3);
    let outline = g.res.DAT_005e8d1c;
    FUN_00500bf0(g, param_1, param_3, (cx - w / 2) - 5, y, outline, 0);
    let f = g.res.DAT_005e8cf0;
    FUN_00455880(param_1, f);
    y += 0x19;
    let mut i = 0;
    while i < param_5 {
        FUN_00455890(param_1, FUN_00433320(0xffffff));
        let line = param_4[i as usize];
        let f = FUN_00455870(param_1);
        let w = font::string_width(g, f, line);
        FUN_00455cf0(param_1, g, line, cx - w / 2, y);
        y += 0xf;
        i += 1;
    }
}

/// port: 00517f70 FUN_00517f70
/// The hand cursor (IMAGE `DAT_005e8a4c`) centered on (x, y); with `click`, eight white
/// strokes around it first. Register arguments: ECX = x, EAX = y, ESI = the Graphics.
pub fn FUN_00517f70(g: &mut G, gfx: &mut Graphics, x: i32, y: i32, param_1: bool) {
    if param_1 {
        let f = g.res.DAT_005e8cf0;
        FUN_00455880(gfx, f);
        FUN_00455890(gfx, FUN_00433320(0xffffff));
        click_mark(gfx, x, y);
    }
    let img = g.res.DAT_005e8a4c;
    let (w, h) = (g.image(img).offset_0x20, g.image(img).offset_0x24);
    FUN_00455d20(gfx, g, img, x - w / 2, y - h / 2);
}

/// The eight strokes both click marks draw (diagonals, then horizontals, then verticals).
fn click_mark(gfx: &mut Graphics, x: i32, y: i32) {
    FUN_00455c50(gfx, x - 3, y - 3, x - 8, y - 8);
    FUN_00455c50(gfx, x + 3, y + 3, x + 8, y + 8);
    FUN_00455c50(gfx, x - 3, y + 3, x - 8, y + 8);
    FUN_00455c50(gfx, x + 3, y - 3, x + 8, y - 8);
    FUN_00455c50(gfx, x - 3, y, x - 8, y);
    FUN_00455c50(gfx, x + 3, y, x + 8, y);
    FUN_00455c50(gfx, x, y + 3, x, y + 8);
    FUN_00455c50(gfx, x, y - 3, x, y - 8);
}

/// port: 00517e90 FUN_00517e90
/// The white click mark at (x, y). Register arguments: EBX = x, EDI = y, ESI = Graphics.
pub fn FUN_00517e90(gfx: &mut Graphics, x: i32, y: i32) {
    FUN_00455890(gfx, FUN_00433320(0xffffff));
    click_mark(gfx, x, y);
}

/// port: 005180a0 FUN_005180a0
/// Moves (x, y) along a 10-pixel Lissajous loop at time `t`: x += sin(2t deg) * 10,
/// y += cos(5t deg) * 10. Register arguments: ESI = t, EBX = &x, EDI = &y.
pub fn FUN_005180a0(t: i32, x: &mut i32, y: &mut i32) {
    let ftol = crate::sexy::crt::ftol;
    let s = sin_f32(rad_f32((t * 2) as f64));
    *x = ftol(s as f64 * 10.0 + *x as f64) as i32;
    let c = cos_f32(rad_f32((t * 5) as f64));
    *y = ftol(c as f64 * 10.0 + *y as f64) as i32;
}

/// The page's update counter (+0x28, the widget's update count).
fn counter(g: &mut G, this: Ptr) -> i32 {
    g.wc(this).offset_0x24
}

/// port: 0051ae70 FUN_0051ae70
/// The picture box of panel `param_1` (ECX): 22 in, 75 wide, inset `param_2` (EDI) top and
/// bottom.
pub fn FUN_0051ae70(param_1: i32, param_2: i32) -> Rect {
    let r = FUN_0051ae40(param_1);
    Rect::new(0x16, r.mY + param_2, 0x4b, r.mHeight - param_2 * 2)
}

/// port: 0051be90 FUN_0051be90
/// A page's two-line introduction in pale yellow, word-wrapped across the top.
pub fn FUN_0051be90(g: &mut G, this: Ptr, param_1: &mut Graphics, param_2: &[u8], param_3: &[u8]) {
    let mut r = Rect::new(10, 0x6e, 0x26c, 0);
    let f = g.res.DAT_005e8cf0;
    FUN_00455880(param_1, f);
    FUN_00455890(param_1, FUN_00433320(0xffffaa));
    vcall!(g, this, w.vfunction65, param_1, r, param_2, -1, 0);
    r.mY += 0xf;
    vcall!(g, this, w.vfunction65, param_1, r, param_3, -1, 0);
}

/// port: 0051bd10 FUN_0051bd10
/// Panel `param_2`: its picture box (translucent black, blue outline; inset `param_5`), its
/// number ("1.") in the outlined font, and two lines of text in pulsing colors.
pub fn FUN_0051bd10(g: &mut G, this: Ptr, param_1: &mut Graphics, param_2: i32, param_3: &[u8], param_4: &[u8], param_5: i32) {
    use crate::sexy::graphics::{FUN_00455990, FUN_00455ac0};
    let r = FUN_0051ae40(param_2);
    let b = FUN_0051ae70(param_2, param_5);
    FUN_00455890(param_1, FUN_00433320(0x60000000));
    FUN_00455990(param_1, &b);
    FUN_00455890(param_1, FUN_00433320(0x316584));
    FUN_00455ac0(param_1, &b);
    FUN_00455890(param_1, FUN_00433320(0xffff88));
    let f = g.res.DAT_005e8b04;
    FUN_00455880(param_1, f);
    let n = format!("{}.", param_2 + 1).into_bytes();
    let o = g.res.DAT_005e8d1c;
    FUN_00500bf0(g, param_1, &n, -100, r.mY + 0x23, o, 0);
    let f = g.res.DAT_005e8cf0;
    FUN_00455880(param_1, f);
    FUN_005184e0(g, this, param_1, r.mY + 0x14);
    FUN_00455cf0(param_1, g, param_3, 0x78, r.mY + 0x14);
    FUN_005184e0(g, this, param_1, r.mY + 0x23);
    FUN_00455cf0(param_1, g, param_4, 0x78, r.mY + 0x23);
}

/// The app pointer at +0x8c of the screens these helpers serve (Help, Hall of Fame).
fn app_at_0x8c(g: &mut G, this: Ptr) -> Ptr {
    match &g.widget(this).ext {
        WExt::HelpScreen(d) => d.field_0x0,
        WExt::HighScoreScreen(d) => d.field_0x0,
        e => panic!("{this} has no app at +0x8c: {e:?}"),
    }
}

/// The pulse shared by the help text colors: lightness 180..250 bouncing with the counter
/// and the line's y.
fn pulse(g: &mut G, this: Ptr, y: i32) -> i32 {
    let mut v = (counter(g, this) + y) % 200;
    if 100 < v {
        v = 200 - v;
    }
    (v * 0x46) / 100 + 0xb4
}

/// port: 005184e0 FUN_005184e0
/// Sets the pulsing body-text color (hue 0x9f, saturation 0x77) for a line at `param_2`.
pub fn FUN_005184e0(g: &mut G, this: Ptr, param_1: &mut Graphics, param_2: i32) {
    let l = pulse(g, this, param_2);
    let app = app_at_0x8c(g, this);
    let c = crate::sexy::sexy_app_base::FUN_00489530(g, app, 0x9f, 0x77, l);
    FUN_00455890(param_1, FUN_00433320(c));
}

/// port: 00518120 FUN_00518120
/// A heading line in the pulsing gold color (hue 0x28, saturation 200).
pub fn FUN_00518120(g: &mut G, this: Ptr, param_1: &mut Graphics, param_2: &[u8], param_3: i32, param_4: i32) {
    let l = pulse(g, this, param_4);
    let app = app_at_0x8c(g, this);
    let c = crate::sexy::sexy_app_base::FUN_00489530(g, app, 0x28, 200, l);
    FUN_00455890(param_1, FUN_00433320(c));
    FUN_00455cf0(param_1, g, param_2, param_3, param_4);
}

/// port: 005181a0 FUN_005181a0
/// A heading in the pulsing gold color, in the outlined heading font.
pub fn FUN_005181a0(g: &mut G, this: Ptr, param_1: &mut Graphics, param_2: &[u8], param_3: i32, param_4: i32) {
    let l = pulse(g, this, param_4);
    let app = app_at_0x8c(g, this);
    let c = crate::sexy::sexy_app_base::FUN_00489530(g, app, 0x28, 200, l);
    FUN_00455890(param_1, FUN_00433320(c));
    let f = g.res.DAT_005e8b04;
    FUN_00455880(param_1, f);
    let o = g.res.DAT_005e8d1c;
    FUN_00500bf0(g, param_1, param_2, param_3, param_4, o, 0);
}

/// port: 00521660 FUN_00521660
/// Page 1, the basics: feeding (a swimming guppy), coins (a spinning coin), aliens (a
/// bobbing alien) and buying (an egg).
pub fn FUN_00521660(g: &mut G, this: Ptr, param_1: &mut Graphics) {
    use crate::sexy::graphics::{FUN_00456950, FUN_00456980};
    let ftol = crate::sexy::crt::ftol;
    FUN_0051be90(g, this, param_1, b"Drop food for your fish to make them grow, then defend them from evil aliens!", b"Collect coins to buy upgrades and Egg pieces.  Complete an Egg to level up!");
    FUN_0051bd10(g, this, param_1, 0, b"Fish turn green when hungry!  Click on the tank to drop food.", b"Well fed fish grow larger, but unfed fish may starve!", 4);
    FUN_0051bd10(g, this, param_1, 1, b"Fish will drop coins for you.  Click on the coins to earn", b"money!  Bigger fish will drop more valuable coins.", 4);
    FUN_0051bd10(g, this, param_1, 2, b"Aliens will attack your fish!  Click on them to shoot.  Clicking", b"on the right side of the alien will make it move left, and so on.", 4);
    FUN_0051bd10(g, this, param_1, 3, b"Click buttons at the top to buy new upgrades.  Buy 3 Egg", b"Pieces to finish the level and earn a new pet!", 4);
    let t = counter(g, this);
    let r = FUN_0051ae70(0, 4);
    let img = g.res.DAT_005e8e48;
    FUN_00456980(param_1, g, img, r.mWidth / 2 + r.mX - 0x28, r.mHeight / 2 + r.mY - 0x28, (t / 3) % 10, 0);
    let r = FUN_0051ae70(1, 4);
    let img = g.res.DAT_005e8e60;
    FUN_00456980(param_1, g, img, r.mWidth / 2 + r.mX - 0x24, r.mHeight / 2 + r.mY - 0x24, (t / 2) % 10, 1);
    let r = FUN_0051ae70(2, 4);
    let s = sin_f32(rad_f32((t * 4) as f64));
    let alien = g.res.DAT_005e8ca4;
    let (aw, ah) = (g.image(alien).offset_0x20, g.image(alien).offset_0x24);
    let yi = (r.mHeight - ah) / 2 + r.mY;
    // x87 order: (3*s + y) + 3.
    let y = ftol((3.0 * s as f64 + yi as f64) + 3.0) as i32;
    let xf = ((r.mWidth - aw) / 2 + r.mX) as f32;
    let c = cos_f32(rad_f32((t * 2) as f64));
    // x87 order: (x - 2*c) + 2.
    let x = ftol((xf as f64 - 2.0 * c as f64) + 2.0) as i32;
    FUN_00455d20(param_1, g, alien, x, y);
    let r = FUN_0051ae70(3, 4);
    let img = g.res.DAT_005e8c00;
    FUN_00456950(param_1, g, img, r.mWidth / 2 + r.mX - 0x17, r.mHeight / 2 + r.mY - 0x12, (t / 36) % 3);
}

/// port: 00521ad0 FUN_00521ad0
/// Page 2, upgrades: a guppy, bobbing food pellets (two rows), and an egg.
pub fn FUN_00521ad0(g: &mut G, this: Ptr, param_1: &mut Graphics) {
    use crate::sexy::graphics::{FUN_00456950, FUN_00456980};
    let ftol = crate::sexy::crt::ftol;
    FUN_0051be90(g, this, param_1, b"There are several upgrades you can purchase if you have enough money.", b"Click on the buttons at the top of the screen to buy the ones you want.");
    FUN_0051bd10(g, this, param_1, 0, b"You can buy new fish for your tank!  Click on the appropriate", b"button to buy a new Guppy or Carnivore, if you can afford it.", 4);
    FUN_0051bd10(g, this, param_1, 1, b"You can upgrade the quality of your food pellets.  Better food", b"will feed fish for a longer time and make them grow faster.", 4);
    FUN_0051bd10(g, this, param_1, 2, b"You can also increase the number of food pellets you can", b"drop at once by upgrading your food quantity.", 4);
    FUN_0051bd10(g, this, param_1, 3, b"You can also increase the power of your anti-alien weapon.", b"More powerful weapons kill aliens in fewer clicks!", 4);
    let t = counter(g, this);
    let r = FUN_0051ae70(0, 4);
    let img = g.res.DAT_005e8ab8;
    FUN_00456980(param_1, g, img, r.mWidth / 2 + r.mX - 0x28, r.mHeight / 2 + r.mY - 0x28, (t / 3) % 10, 0);
    let food = g.res.DAT_005e8aa0;
    let bob = |a: f32, base: f32| ftol(sin_f32(a) as f64 * 3.0 + base as f64) as i32;
    // Food quality: three pellets in rows 0..2.
    let r = FUN_0051ae70(1, 4);
    let a = rad_f32((t * 5) as f64);
    let base = (r.mHeight / 2 + r.mY - 0x14) as f32;
    let cx = r.mX + r.mWidth / 2;
    let y = bob(a, base);
    FUN_00456980(param_1, g, food, cx - 0x28, y, (t / 3) % 10, 0);
    let y = bob((a as f64 + f64::from_bits(0x403e0477_e0000000)) as f32, base);
    FUN_00456980(param_1, g, food, cx - 0x14, y, (t / 4) % 10, 1);
    let y = bob((a as f64 + f64::from_bits(0x404e0477_e0000000)) as f32, base);
    FUN_00456980(param_1, g, food, cx, y, (t / 3) % 10, 2);
    // Food quantity: three pellets in row 0 (the first at a fixed angle, as in the original).
    let a = ((t * 5) as f64 * f64::from_bits(0x400921fa_00000000) / 180.0 + 60.0) as f32;
    let r = FUN_0051ae70(2, 4);
    let base = (r.mHeight / 2 + r.mY - 0x14) as f32;
    let cx = r.mWidth / 2 + r.mX;
    let k = f64::from_bits(0x3fb65717_20000000);
    let y = bob(k as f32, base);
    FUN_00456980(param_1, g, food, cx - 0x28, y, (t / 3) % 10, 0);
    let y = bob((a as f64 + k) as f32, base);
    FUN_00456980(param_1, g, food, cx - 0x14, y, (t / 4 + 3) % 10, 0);
    let y = bob((a as f64 + a as f64 + k) as f32, base);
    FUN_00456980(param_1, g, food, cx, y, (t / 3 + 6) % 10, 0);
    let r = FUN_0051ae70(3, 4);
    let img = g.res.DAT_005e8a10;
    FUN_00456950(param_1, g, img, r.mWidth / 2 + r.mX - 0x17, r.mHeight / 2 + r.mY - 0x12, (t / 36) % 3);
}

/// port: 005220a0 FUN_005220a0
/// Page 3, eggs and pets: an egg, Itchy, Stinky, and the pet choice panel.
pub fn FUN_005220a0(g: &mut G, this: Ptr, param_1: &mut Graphics) {
    use crate::sexy::graphics::FUN_00456950;
    FUN_0051be90(g, this, param_1, b"You can finish a level by collecting all three pieces of an Egg.  Click on the", b"Egg Piece button in the control bar to buy an Egg Piece.");
    FUN_0051bd10(g, this, param_1, 0, b"Advancing to the next level makes your egg hatch.  This gives you", b"a new pet!  Each pet has different powers to help you out.", 4);
    FUN_0051bd10(g, this, param_1, 1, b"ITCHY the Swordfish is one of the pets that you can get.", b"He will attack aliens that infiltrate your tank!", 4);
    FUN_0051bd10(g, this, param_1, 2, b"STINKY the Snail is another pet that you can get.", b"He roams the ground picking up stray coins for you!", 4);
    FUN_0051bd10(g, this, param_1, 3, b"You can only bring 3 pets into a level. If you have more than 3", b"pets, you'll have to choose which 3 you want to have in the tank.", 4);
    let t = counter(g, this);
    let r = FUN_0051ae70(0, 4);
    let img = g.res.DAT_005e8c00;
    FUN_00456950(param_1, g, img, r.mWidth / 2 + r.mX - 0x17, r.mHeight / 2 + r.mY - 0x12, 2);
    let r = FUN_0051ae70(1, 4);
    let img = g.res.DAT_005e8cb0;
    FUN_00456950(param_1, g, img, r.mWidth / 2 + r.mX - 0x1e, r.mHeight / 2 + r.mY - 0x1e, (t / 4) % 10);
    let r = FUN_0051ae70(2, 4);
    let img = g.res.DAT_005e8a84;
    FUN_00456950(param_1, g, img, r.mWidth / 2 + r.mX - 0x1e, r.mHeight / 2 + r.mY - 0x1e, (t / 4) % 10);
    let r = FUN_0051ae70(3, 4);
    let img = g.res.DAT_005e8b34;
    let (iw, ih) = (g.image(img).offset_0x20, g.image(img).offset_0x24);
    FUN_00455d20(param_1, g, img, (r.mWidth - iw) / 2 + r.mX, (r.mHeight - ih) / 2 + r.mY + 2);
}

/// A heading centered in panel `i`'s picture box at `dy` below its top (page 4).
fn box_heading(g: &mut G, this: Ptr, gfx: &mut Graphics, i: i32, text: &[u8], dy: i32) {
    let r = FUN_0051ae70(i, 4);
    let f = FUN_00455870(gfx);
    let w = font::string_width(g, f, text);
    FUN_00518120(g, this, gfx, text, (r.mWidth - w) / 2 + r.mX, r.mY + dy);
}

/// port: 00522460 FUN_00522460
/// Page 4, the game modes: Adventure, Time Trial, Challenge and Virtual Tank, each name in
/// its picture box.
pub fn FUN_00522460(g: &mut G, this: Ptr, param_1: &mut Graphics) {
    FUN_0051be90(g, this, param_1, b"Insaniquarium has four different game modes that you can play!", b"They are Adventure, Time Trial, Challenge, and Virtual Tank.");
    FUN_0051bd10(g, this, param_1, 0, b"Adventure is the main game mode.  In this mode, you", b"progress through multiple tanks and accumulate pets.", 4);
    FUN_0051bd10(g, this, param_1, 1, b"The goal of Time Trial mode is to collect as much", b"money as you can before the time runs out.", 4);
    FUN_0051bd10(g, this, param_1, 2, b"Challenge mode is for experts.  In this mode, you must", b"deal with price inflation and increasingly difficult aliens.", 4);
    FUN_0051bd10(g, this, param_1, 3, b"Virtual Tank is a Virtual Aquarium.  Use Shells earned in", b"other modes to purchase items and fish for your tank!", 4);
    FUN_00455890(param_1, FUN_00433320(0xbbffbb));
    let f = g.res.DAT_005e8ce4;
    FUN_00455880(param_1, f);
    box_heading(g, this, param_1, 0, b"Adventure", 0x1e);
    box_heading(g, this, param_1, 1, b"Time Trial", 0x1e);
    box_heading(g, this, param_1, 2, b"Challenge", 0x1e);
    box_heading(g, this, param_1, 3, b"Virtual", 0x12);
    box_heading(g, this, param_1, 3, b"Tank", 0x25);
}

/// `min(v + 5, 255)` of a colored-guppy palette entry (constant data in the original).
fn tint(v: i32) -> i32 {
    if v + 5 > 0xff { 0xff } else { v + 5 }
}

/// The colored guppy's three layers (base, then additive tinted layers in `c2` and `c1`).
fn tinted_guppy(g: &mut G, gfx: &mut Graphics, imgs: [Ptr; 3], x: i32, y: i32, src: &Rect, c1: Color, c2: Color) {
    use crate::sexy::graphics::{FUN_004558c0, FUN_004558e0, FUN_00455e40};
    FUN_00455e40(gfx, g, imgs[0], x, y, src);
    FUN_004558c0(gfx, 1);
    FUN_004558e0(gfx, true);
    FUN_00455890(gfx, c2);
    FUN_00455e40(gfx, g, imgs[1], x, y, src);
    FUN_00455890(gfx, c1);
    FUN_00455e40(gfx, g, imgs[2], x, y, src);
    FUN_004558c0(gfx, 0);
    FUN_004558e0(gfx, false);
}

/// port: 005228e0 FUN_005228e0
/// Page 5, the virtual tank: the store icon, a shell, a colored guppy, and a rare fish.
pub fn FUN_005228e0(g: &mut G, this: Ptr, param_1: &mut Graphics) {
    use crate::sexy::graphics::{FUN_00455e40, FUN_00456950};
    FUN_0051be90(g, this, param_1, b"Virtual Tank is a Virtual Aquarium.  In it, you can buy, name,", b"take care of, and play with your own unique fish.");
    FUN_0051bd10(g, this, param_1, 0, b"Access the Virtual Tank store from within Virtual Tank.", b"You must use Shells to buy items in the store.", 4);
    FUN_0051bd10(g, this, param_1, 1, b"You can earn Shells at certain times in all four", b"game modes.  Shells are the currency of Virtual Tank.", 4);
    FUN_0051bd10(g, this, param_1, 2, b"Items in the Virtual Tank store change daily with the real", b"world date so check back every day to see what's new!", 4);
    FUN_0051bd10(g, this, param_1, 3, b"Some items in the store are quite rare so make sure to", b"check the store every day so you don't miss a rare item!", 4);
    FUN_00455890(param_1, FUN_00433320(0xbbffbb));
    let f = g.res.DAT_005e8ce4;
    FUN_00455880(param_1, f);
    let t = counter(g, this);
    let r = FUN_0051ae70(0, 4);
    let icons = g.res.DAT_005e8e28;
    let cw = crate::sexy::graphics::FUN_004574b0(g, icons);
    let ch = crate::sexy::graphics::FUN_00457490(g, icons);
    FUN_00456950(param_1, g, icons, (r.mWidth - cw) / 2 + r.mX, (r.mHeight - ch) / 2 + r.mY, 2);
    let r = FUN_0051ae70(1, 4);
    let shell = g.res.DAT_005e8b74;
    let (sw, sh) = (g.image(shell).offset_0x20, g.image(shell).offset_0x24);
    FUN_00455d20(param_1, g, shell, (r.mWidth - sw) / 2 + r.mX, (r.mHeight - sh) / 2 + r.mY);
    // `DAT_005e117c`, `DAT_005e11f4`, `DAT_005e126c` (255, 240, 60) and `DAT_005e12e4`,
    // `DAT_005e135c`, `DAT_005e13d4` (45, 95, 195): palette entries, never written.
    let c1 = CRect(tint(255), tint(240), tint(60), 0xff);
    let c2 = CRect(tint(45), tint(95), tint(195), 0xff);
    let r = FUN_0051ae70(2, 4);
    let src = Rect::new(((t / 2) % 10) * 0x50, 0, 0x50, 0x50);
    let (x, y) = ((r.mWidth - 0x50) / 2 + r.mX, (r.mHeight - 0x50) / 2 + r.mY);
    let imgs = [g.res.DAT_005e8b78, g.res.DAT_005e8eec, g.res.DAT_005e8efc];
    tinted_guppy(g, param_1, imgs, x, y, &src, c1, c2);
    let r = FUN_0051ae70(3, 4);
    let src = Rect::new(((t / 4) % 10) * 0x32, 0, 0x32, 0x32);
    let img = g.res.DAT_005e8ee8;
    FUN_00455e40(param_1, g, img, (r.mWidth - 0x32) / 2 + r.mX, (r.mHeight - 0x32) / 2 + r.mY, &src);
}

/// port: 00522e40 FUN_00522e40
/// Page 6, virtual fish: a colored guppy eating (swimming, then the eat cels), a colored
/// guppy swimming, the feed icon, and a sleeping fish with a "?".
pub fn FUN_00522e40(g: &mut G, this: Ptr, param_1: &mut Graphics) {
    use crate::sexy::graphics::{FUN_00455e40, FUN_00456950};
    FUN_0051be90(g, this, param_1, b"Virtual Tank fish behave differently than fish in other game modes.", b"Here is a list of differences.");
    FUN_0051bd10(g, this, param_1, 0, b"Virtual Tank fish want to eat three times a day.  They will", b"become unhappy if you don't feed them three times every day.", 4);
    FUN_0051bd10(g, this, param_1, 1, b"Fish in Virtual Tank will grow eventually.  The time", b"to grow is about one real world week if fed every day.", 0);
    FUN_0051bd10(g, this, param_1, 2, b"To feed fish which don't eat the normal food that you drop by", b"clicking, press the \"Feed\" button on the Virtual Tank menu.", 4);
    FUN_0051bd10(g, this, param_1, 3, b"Virtual Tank fish can not die or be killed.  That's not", b"to say that you shouldn't take care of them, though!", 4);
    FUN_00455890(param_1, FUN_00433320(0xbbffbb));
    let f = g.res.DAT_005e8ce4;
    FUN_00455880(param_1, f);
    // `DAT_005e11b4`, `DAT_005e122c`, `DAT_005e12a4` (175, 240, 175) and `DAT_005e131c`,
    // `DAT_005e1394`, `DAT_005e140c` (30, 80, 125): palette entries, never written.
    let c1 = CRect(tint(175), tint(240), tint(175), 0xff);
    let c2 = CRect(tint(30), tint(80), tint(125), 0xff);
    let t = counter(g, this);
    let r = FUN_0051ae70(0, 4);
    let mut n = (t / 2) % 0x1b;
    let base = if n < 10 {
        0xb5
    } else {
        n += 3;
        0xb8
    };
    let src = Rect::new((n % 10) * 0x50, 0, 0x50, 0x50);
    let (x, y) = ((r.mWidth - 0x50) / 2 + r.mX, (r.mHeight - 0x50) / 2 + r.mY);
    let imgs = [
        crate::sexy::res::FUN_005016a0(g, base) as Ptr,
        crate::sexy::res::FUN_005016a0(g, base + 1) as Ptr,
        crate::sexy::res::FUN_005016a0(g, base + 2) as Ptr,
    ];
    tinted_guppy(g, param_1, imgs, x, y, &src, c1, c2);
    let r = FUN_0051ae70(1, 4);
    let src = Rect::new(((t / 2) % 10) * 0x50, 0, 0x50, 0x50);
    let (x, y) = ((r.mWidth - 0x50) / 2 + r.mX, (r.mHeight - 0x50) / 2 + r.mY);
    let imgs = [
        crate::sexy::res::FUN_005016a0(g, 0xb8) as Ptr,
        crate::sexy::res::FUN_005016a0(g, 0xb9) as Ptr,
        crate::sexy::res::FUN_005016a0(g, 0xba) as Ptr,
    ];
    tinted_guppy(g, param_1, imgs, x, y, &src, c1, c2);
    let r = FUN_0051ae70(2, 4);
    let icons = g.res.DAT_005e8e28;
    let cw = crate::sexy::graphics::FUN_004574b0(g, icons);
    let ch = crate::sexy::graphics::FUN_00457490(g, icons);
    FUN_00456950(param_1, g, icons, (r.mWidth - cw) / 2 + r.mX, (r.mHeight - ch) / 2 + r.mY + 2, 1);
    let r = FUN_0051ae70(3, 4);
    let src = Rect::new(0x2d0, 0, 0x50, 0x50);
    let img = g.res.DAT_005e8aa8;
    FUN_00455e40(param_1, g, img, (r.mWidth - 0x50) / 2 + r.mX, (r.mHeight - 0x50) / 2 + r.mY, &src);
    FUN_00455890(param_1, FUN_00433320(0xffff00));
    let f = g.res.DAT_005e8b04;
    FUN_00455880(param_1, f);
    let fnt = FUN_00455870(param_1);
    let w = font::string_width(g, fnt, b"?");
    let o = g.res.DAT_005e8d1c;
    FUN_00500bf0(g, param_1, b"?", (r.mWidth - w) / 2 + r.mX, r.mY + 0x19, o, 0);
}

/// port: 00523600 FUN_00523600
/// Page 7, the virtual tank's buttons: the Fish, Pets and Tank icons and the screensaver.
pub fn FUN_00523600(g: &mut G, this: Ptr, param_1: &mut Graphics) {
    use crate::sexy::graphics::FUN_00456950;
    FUN_0051be90(g, this, param_1, b"You can configure the Virtual Tank in several ways.", b"Here is a list of the options.");
    FUN_0051bd10(g, this, param_1, 0, b"Press the \"Fish\" button to go to a screen where you can see", b"information about your fish as well as hide, show, and sell them.", 4);
    FUN_0051bd10(g, this, param_1, 1, b"Press the \"Pets\" button to select which pets you would like", b"to display in your Virtual Tank.", 4);
    FUN_0051bd10(g, this, param_1, 2, b"Press the \"Tank\" button to select which tank backdrop to display", b"and whether or not to show various items in your tank.", 4);
    FUN_0051bd10(g, this, param_1, 3, b"You can set Virtual Tank as your computer's Screensaver by", b"pressing the \"Screensaver\" button on the \"Tank\" screen.", 0);
    FUN_00455890(param_1, FUN_00433320(0xbbffbb));
    let f = g.res.DAT_005e8ce4;
    FUN_00455880(param_1, f);
    let icons = g.res.DAT_005e8e28;
    let cw = crate::sexy::graphics::FUN_004574b0(g, icons);
    let ch = crate::sexy::graphics::FUN_00457490(g, icons);
    let r = FUN_0051ae70(0, 4);
    FUN_00456950(param_1, g, icons, (r.mWidth - cw) / 2 + r.mX, (r.mHeight - ch) / 2 + r.mY + 3, 3);
    let r = FUN_0051ae70(1, 4);
    FUN_00456950(param_1, g, icons, (r.mWidth - cw) / 2 + r.mX, (r.mHeight - ch) / 2 + r.mY, 4);
    let r = FUN_0051ae70(2, 4);
    FUN_00456950(param_1, g, icons, (r.mWidth - cw) / 2 + r.mX, (r.mHeight - ch) / 2 + r.mY, 5);
    let r = FUN_0051ae70(3, 4);
    let img = g.res.DAT_005e8a1c;
    let (iw, ih) = (g.image(img).offset_0x20, g.image(img).offset_0x24);
    FUN_00455d20(param_1, g, img, (r.mWidth - iw) / 2 + r.mX, (r.mHeight - ih) / 2 + r.mY + 2);
}

/// port: 005239d0 FUN_005239d0
/// Page 8, the credits: eleven title / name pairs 25 pixels apart (an untitled line 7 higher);
/// with 3D acceleration each line bobs by 3 * sin of its y and the counter (a D3D transform;
/// here the Graphics translation, the same offset for the blits).
pub fn FUN_005239d0(g: &mut G, this: Ptr, param_1: &mut Graphics) {
    const CREDITS: [(&[u8], &[u8]); 11] = [
        (b"Game Design", b"George Fan"),
        (b"Producer", b"Jason Kapalka, Sukhbir Sidhu"),
        (b"Programming", b"George Fan, Thien Tran, Brian Rothstein"),
        (b"Technical Assistance", b"David Parton"),
        (b"Art", b"Josh Langley, Walter Wilson"),
        (b"Character Design", b"George Fan"),
        (b"Music", b"Jonne Valtonen, George Fan"),
        (b"PopCap Framework", b"Brian Fiete, David Parton"),
        (b"Biz Dev", b"Don Walters"),
        (b"QA", b"Eric Harman, Shawn Conard,"),
        (b"", b"Brenna Flood, Chad Zoellner"),
    ];
    let app = g.help(this).field_0x0;
    let is3d = crate::sexy::sexy_app_base::dtor_MemoryImage__00489a20(g, app);
    let pi = f64::from_bits(0x400921fa_00000000);
    let mut y: i32 = 0x96;
    for (title, name) in CREDITS {
        let mut pushed = 0.0f32;
        if is3d {
            let t = counter(g, this);
            let a = ((t as f64 * 3.0 * pi / 180.0) + ((y + y) as f64 * pi / 180.0)) as f32;
            pushed = (sin_f32(a) as f64 * 3.0) as f32;
            param_1.s.mTransY += pushed;
        }
        FUN_00455890(param_1, FUN_00433320(0xbbffbb));
        let f = g.res.DAT_005e8b04;
        FUN_00455880(param_1, f);
        if title.is_empty() {
            y -= 7;
        }
        FUN_005181a0(g, this, param_1, title, 0x14, y);
        let f = g.res.DAT_005e8cf0;
        FUN_00455880(param_1, f);
        FUN_005184e0(g, this, param_1, y);
        FUN_00455cf0(param_1, g, name, 300, y);
        y += 0x19;
        if is3d {
            param_1.s.mTransY -= pushed;
        }
    }
}
