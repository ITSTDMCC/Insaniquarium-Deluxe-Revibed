//! `Sexy::TitleScreen`: the loading screen with the progress bar, Stinky crossing it, and the
//! "click to play" link. `TitleScreen_data` starts at object offset 0x8c; the class is larger
//! than the database's 0xc0 bytes (fields `ext_0xNN`).

use crate::sexy::graphics::{FUN_00455880, FUN_00455890, FUN_004558e0, FUN_00455870, FUN_00455cf0, FUN_00455d20, FUN_004560a0, FUN_00456340};
use crate::sexy::prelude::*;

/// `TitleScreen_data` (object offset 0x8c) plus the bytes past the database's class size.
#[derive(Debug, Clone, Default)]
pub struct TitleScreen_data {
    /// +0x8c: the "click to play" hyperlink over the bar.
    pub offset_0x0: Ptr,
    /// +0x90: the "Click Here To Register!" hyperlink.
    pub offset_0x4: Ptr,
    /// +0x94: drawn progress-bar width.
    pub field_0x8: i32,
    /// +0x98: full bar width (IMAGE_LOADERBAR's width).
    pub field_0xc: i32,
    /// +0x9c: bar growth per update (accelerates up to 3).
    pub field_0x10: i32,
    /// +0xa0: open the registration dialog when possible.
    pub field_0x14: bool,
    /// +0xa1: loading finished (link shown).
    pub field_0x15: bool,
    /// +0xa4: the trial message (std::string 0xa4..0xc0).
    pub field_0x18: Vec<u8>,
    /// +0xc0: the trial has run out.
    pub ext_0xc0: bool,
    /// +0xc1: show the "trial expired" dialog when possible.
    pub ext_0xc1: bool,
    /// +0xc4: Stinky's x.
    pub ext_0xc4: i32,
    /// +0xcc: frame counter.
    pub ext_0xcc: i32,
    /// +0xd0: fade between the "loading" and "play" bar images.
    pub ext_0xd0: i32,
    /// +0xd4: the app.
    pub ext_0xd4: Ptr,
}

impl G {
    pub fn title(&mut self, p: Ptr) -> &mut TitleScreen_data {
        match &mut self.widget(p).ext {
            WExt::TitleScreen(t) => t,
            e => panic!("{p} is not a TitleScreen: {e:?}"),
        }
    }
}

/// `DAT_005e160c`: Stinky's starting x (160).
pub const DAT_005e160c: i32 = 160;

/// port: 0052eb50 Sexy::TitleScreen::TitleScreen
/// `TitleScreen(WinFishApp*)`.
pub fn TitleScreen(g: &mut G, param_1: Ptr) -> Ptr {
    let (wc, w) = crate::sexy::widget::Widget();
    let bar = g.res.DAT_005e8b00;
    let d = TitleScreen_data { ext_0xd4: param_1, field_0xc: g.image(bar).offset_0x20, ..Default::default() };
    let this = g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__TitleScreen_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::TitleScreen(Box::new(d)) }),
    });
    let font = g.res.DAT_005e8cf0;
    let play = crate::sexy::button_widget::HyperlinkWidget(g, 0, this);
    g.title(this).offset_0x0 = play;
    vcall!(g, play, btn.vfunction72, font);
    g.hyperlink(play).offset_0x0 = FUN_00433320(0xffec91);
    g.hyperlink(play).field_0x10 = FUN_00433320(0xffffff);
    g.hyperlink(play).offset_0x20 = 2;
    vcall!(g, play, w.vfunction32, false);
    let reg = crate::sexy::button_widget::HyperlinkWidget(g, 1, this);
    g.title(this).offset_0x4 = reg;
    g.btn(reg).field_0x4 = b"Click Here To Register!".to_vec();
    vcall!(g, reg, btn.vfunction72, font);
    g.hyperlink(reg).offset_0x0 = FUN_00433320(0xffec91);
    g.hyperlink(reg).field_0x10 = FUN_00433320(0xffffff);
    g.hyperlink(reg).offset_0x20 = 2;
    vcall!(g, reg, w.vfunction32, true);
    let (bw, bh) = (g.image(bar).offset_0x20, g.image(bar).offset_0x24);
    vcall!(g, play, w.vfunction41, 0xb4, 0x19f, bw, bh);
    let label = g.btn(reg).field_0x4.clone();
    let sw = crate::sexy::image_font::string_width(g, font, &label);
    let fh = crate::sexy::image_font::get_height(g, font);
    vcall!(g, reg, w.vfunction41, 0, 0, sw + 0x14, fh + 0x14);
    let app = g.globals.DAT_005eb6a4;
    let registered = g.app_obj(app).sa.offset_0xe4;
    if !registered {
        if !crate::game::win_fish_app::FUN_00479fc0(g, param_1) {
            if !g.app_obj(app).sa.offset_0xe5 {
                FUN_0052d2e0(g, this);
            }
            g.title(this).ext_0xc4 = DAT_005e160c;
            return this;
        }
    }
    vcall!(g, reg, w.vfunction32, false);
    let t = g.title(this);
    t.ext_0xc4 = DAT_005e160c;
    t.ext_0xcc = 0;
    t.ext_0xd0 = 0;
    this
}

/// port: 0051c140 Sexy::TitleScreen::~TitleScreen
pub fn dtor_TitleScreen(g: &mut G, this: Ptr) {
    for link in [g.title(this).offset_0x0, g.title(this).offset_0x4] {
        if link != NULL {
            vcall!(g, link, w.vfunction1, 1);
        }
    }
    g.title(this).field_0x18.clear();
    crate::sexy::widget::dtor_Widget(g, this);
}

/// port: 0051c2a0 Sexy::TitleScreen::deleting_destructor_for_Widget
pub fn deleting_destructor_for_Widget(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_TitleScreen(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 0051a7a0 Sexy::TitleScreen::vfunction41_for_Widget
/// `Resize(x, y, w, h)`: keeps the register link centered at y 350.
pub fn vfunction41_for_Widget(g: &mut G, this: Ptr, x: i32, y: i32, w: i32, h: i32) {
    crate::sexy::widget::vfunction41(g, this, x, y, w, h);
    let reg = g.title(this).offset_0x4;
    let (lw, lh) = (g.wc(reg).offset_0x34, g.wc(reg).offset_0x38);
    let tw = g.wc(this).offset_0x34;
    vcall!(g, reg, w.vfunction41, (tw - lw) / 2 + x, 0x15e, lw, lh);
}

/// port: 0051a9b0 Sexy::TitleScreen::vfunction21_for_Widget
/// `AddedToManager(WidgetManager*)`: adds both links.
pub fn vfunction21_for_Widget(g: &mut G, this: Ptr, manager: Ptr) {
    crate::sexy::widget_container::vfunction21(g, this, manager);
    let (reg, play) = (g.title(this).offset_0x4, g.title(this).offset_0x0);
    vcall!(g, manager, w.vfunction4, reg);
    vcall!(g, manager, w.vfunction4, play);
}

/// port: 0051a9f0 Sexy::TitleScreen::vfunction22_for_Widget
/// `RemovedFromManager(WidgetManager*)`.
pub fn vfunction22_for_Widget(g: &mut G, this: Ptr, manager: Ptr) {
    crate::sexy::widget_container::vfunction22(g, this, manager);
    let (reg, play) = (g.title(this).offset_0x4, g.title(this).offset_0x0);
    vcall!(g, manager, w.vfunction5, reg);
    vcall!(g, manager, w.vfunction5, play);
}

/// port: 0051aa30 Sexy::TitleScreen::vfunction2_for_ButtonListener
/// `ButtonPress(int)`: the click sound (app vtable +0xd8 `PlaySample`).
pub fn vfunction2_for_ButtonListener(g: &mut G, _this: Ptr, _id: i32) {
    let app = g.globals.DAT_005eb6a4;
    let s = g.res.DAT_005e8c40;
    crate::sexy::sexy_app_base::vfunction55(g, app, s);
}

/// port: 0051aa50 Sexy::TitleScreen::vfunction3_for_ButtonListener
/// `ButtonDepress(int id)`: 0 = play (app vtable +0x1a0 `TitleScreenIsFinished`), 1 = register.
pub fn vfunction3_for_ButtonListener(g: &mut G, this: Ptr, id: i32) {
    if id == 0 {
        let app = g.globals.DAT_005eb6a4;
        crate::game::win_fish_app::vfunction105(g, app);
    } else if id == 1 {
        g.title(this).field_0x14 = true;
    }
}

/// port: 0051a7f0 Sexy::TitleScreen::vfunction23_for_Widget
/// `Update()`: grows the bar toward the loading progress, then reveals the play link and
/// walks Stinky across.
pub fn vfunction23_for_Widget(g: &mut G, this: Ptr) {
    crate::sexy::widget_container::vfunction23(g, this);
    if 0 < g.title(this).ext_0xd0 {
        g.title(this).ext_0xd0 -= 1;
    }
    let full = g.title(this).field_0xc;
    let app = g.globals.DAT_005eb6a4;
    // app->GetLoadingThreadProgress() (app vtable +0x100), times the bar width, truncated.
    let progress = crate::sexy::sexy_app_base::vfunction65(g, app);
    let target = crate::sexy::crt::ftol(progress * full as f64) as i32;
    if g.title(this).field_0x8 < target {
        let t = g.title(this);
        if t.field_0x10 < 3 {
            t.field_0x10 += 1;
        }
        let speed = t.field_0x10;
        t.field_0x8 += speed;
        t.ext_0xcc += 1;
        t.ext_0xc4 = DAT_005e160c + t.field_0x8;
        if full < t.field_0x8 {
            t.field_0x8 = full;
            t.field_0x10 = speed - 1;
        }
        vcall!(g, this, w.vfunction18);
    } else if !g.title(this).field_0x15 {
        if g.sab(app).field_0x4fa {
            g.title(this).field_0x15 = true;
            if !crate::game::win_fish_app::FUN_00479fc0(g, app) {
                let play = g.title(this).offset_0x0;
                g.title(this).ext_0xd0 = 8;
                vcall!(g, play, w.vfunction32, true);
                vcall!(g, this, w.vfunction18);
            }
        }
    } else {
        let x = g.title(this).ext_0xc4;
        if x < g.wc(this).offset_0x34 {
            let t = g.title(this);
            if t.field_0x10 < 3 {
                t.field_0x10 += 1;
            }
            t.ext_0xcc += 1;
            t.ext_0xc4 = t.field_0x10 + x;
            vcall!(g, this, w.vfunction18);
        }
        if crate::game::win_fish_app::FUN_00479fc0(g, app) && 0x1fe < g.title(this).ext_0xc4 {
            crate::game::win_fish_app::vfunction105(g, app);
        }
    }
    if g.title(this).field_0x14 && crate::game::win_fish_app::FUN_0054b590(g, app) {
        g.title(this).field_0x14 = false;
        crate::game::register_dialog::FUN_0054e8a0(g, app);
    }
    if g.title(this).ext_0xc1 && crate::game::win_fish_app::FUN_0054b590(g, app) {
        g.title(this).ext_0xc1 = false;
        crate::game::register_dialog::FUN_0054ea40(g, app);
    }
    if g.wc(this).offset_0x24 % 4 == 0 {
        vcall!(g, this, w.vfunction18);
    }
}

/// port: 0052d490 Sexy::TitleScreen::vfunction27_for_Widget
/// `Draw(Graphics*)`.
pub fn vfunction27_for_Widget(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    let play = g.title(this).offset_0x0;
    let (mut id, mut x, mut y) = (7, 0xb4, 0x19f);
    let pw = g.w(play).clone();
    if pw.offset_0x0 {
        id = 7 + pw.offset_0x5 as i32;
        if pw.offset_0x5 && pw.offset_0x4 {
            x = 0xb5;
            y = 0x1a0;
        }
    }
    let bar = crate::sexy::res::FUN_005016a0(g, id) as Ptr;
    let title = g.res.DAT_005e8bec;
    FUN_00455d20(gfx, g, title, 0, 0);
    let mut clip = Graphics { s: gfx.s.clone(), mStateStack: Vec::new(), out: gfx.out.clone() };
    let (bw, bh) = (g.title(this).field_0x8, g.image(bar).offset_0x24);
    FUN_00456340(&mut clip, x, y, bw, bh);
    FUN_00455d20(&mut clip, g, bar, x, y);
    let (loading, play_img) = (g.res.DAT_005e8f10, g.res.DAT_005e8d6c);
    if g.w(play).offset_0x0 {
        let a = (g.title(this).ext_0xd0 * 0xff) / 8;
        if a == 0 {
            FUN_00455d20(gfx, g, play_img, x, y);
        } else {
            FUN_004558e0(gfx, true);
            FUN_00455890(gfx, CRect(0xff, 0xff, 0xff, a));
            FUN_00455d20(gfx, g, loading, 0xe6, 0x19f);
            FUN_00455890(gfx, CRect(0xff, 0xff, 0xff, 0xff - a));
            FUN_00455d20(gfx, g, play_img, x, y);
            FUN_004558e0(gfx, false);
        }
    } else {
        FUN_00455d20(&mut clip, g, loading, 0xe6, 0x19f);
    }
    let frame = (g.wc(this).offset_0x24 / 4) % 10;
    let sx = g.title(this).ext_0xc4;
    if sx < 0x1fe {
        let stinky = g.res.DAT_005e8be0;
        FUN_004560a0(gfx, g, stinky, sx, y - 0x1e, &Rect::new(frame * 0x50, 0, 0x50, 0x50), true);
    }
    let mask = g.res.DAT_005e8e80;
    FUN_00455d20(gfx, g, mask, 0x180, 0x184);
    let app = g.title(this).ext_0xd4;
    let tw = g.wc(this).offset_0x34;
    let centered = |g: &mut G, gfx: &mut Graphics, s: &[u8], yy: i32| {
        let f = FUN_00455870(gfx);
        let sw = crate::sexy::image_font::string_width(g, f, s);
        FUN_00455cf0(gfx, g, s, (tw - sw) / 2 - 0xd, yy);
    };
    if crate::game::win_fish_app::FUN_00479fc0(g, app) {
        FUN_00455880(gfx, g.res.DAT_005e8e40);
        FUN_00455890(gfx, FUN_00433320(0xffec91));
        centered(g, gfx, b"Starting Screensaver", 0x18b);
    } else {
        let gapp = g.globals.DAT_005eb6a4;
        let (reg, unlocked) = (g.app_obj(gapp).sa.offset_0xe4, g.app_obj(gapp).sa.offset_0xe5);
        if reg && !unlocked {
            FUN_00455880(gfx, g.res.DAT_005e8cf0);
            FUN_00455890(gfx, FUN_00433360(0xff, 0xff, 0xff));
            centered(g, gfx, b"THANKS FOR REGISTERING!", 0x1d6);
        }
        if !reg {
            FUN_00455880(gfx, g.res.DAT_005e8cf0);
            FUN_00455890(gfx, FUN_00433360(0xff, 0xff, 0xff));
            let msg = g.title(this).field_0x18.clone();
            centered(g, gfx, &msg, 0x1d6);
        }
    }
    FUN_00455880(gfx, g.res.DAT_005e8cf0);
    FUN_00455890(gfx, FUN_00433360(0xff, 0xff, 0xff));
    let mut ver = b"Version ".to_vec();
    ver.extend_from_slice(&g.sab(app).field_0x390);
    let f = FUN_00455870(gfx);
    let sw = crate::sexy::image_font::string_width(g, f, &ver);
    FUN_00455cf0(gfx, g, &ver, tw - sw - 0x10, 0xd3);
}

/// port: 0052d2e0 FUN_0052d2e0
/// The trial message under the bar: minutes left (`MaxTime`), else free plays left
/// (`MaxPlays`, else `MaxExecutions` counting this run); when none are left the trial has
/// run out: the expired dialog is to be shown and both links are hidden.
pub fn FUN_0052d2e0(g: &mut G, this: Ptr) {
    let app = g.globals.DAT_005eb6a4;
    let (max_time, max_plays, max_exec) = (g.wfa(app).offset_0x178, g.wfa(app).offset_0x174, g.wfa(app).offset_0x170);
    let (played, executed) = (g.app_obj(app).sa.offset_0xe8, g.app_obj(app).sa.offset_0xec);
    if 0 < max_time {
        let n = max_time - played;
        if n < 1 {
            g.title(this).ext_0xc0 = true;
        } else if n == 1 {
            g.title(this).field_0x18 = b"YOU HAVE 1 MINUTE LEFT IN THIS TRIAL!".to_vec();
        } else {
            g.title(this).field_0x18 = format!("YOU HAVE {n} MINUTES LEFT IN THIS TRIAL!").into_bytes();
        }
    } else if 0 < max_plays {
        let n = max_plays - played;
        if n < 1 {
            g.title(this).ext_0xc0 = true;
        } else if n == 1 {
            g.title(this).field_0x18 = b"YOU HAVE 1 FREE PLAY REMAINING!".to_vec();
        } else {
            g.title(this).field_0x18 = format!("YOU HAVE {n} FREE PLAYS REMAINING!").into_bytes();
        }
    } else if 0 < max_exec {
        let n = (max_exec - executed) + 1;
        if n < 1 {
            g.title(this).ext_0xc0 = true;
        } else if n == 1 {
            g.title(this).field_0x18 = b"YOU HAVE 1 FREE PLAY REMAINING!".to_vec();
        } else {
            g.title(this).field_0x18 = format!("YOU HAVE {n} FREE PLAYS REMAINING!").into_bytes();
        }
    }
    if g.title(this).ext_0xc0 {
        g.title(this).ext_0xc1 = true;
        let (play, reg) = (g.title(this).offset_0x0, g.title(this).offset_0x4);
        vcall!(g, reg, w.vfunction32, false);
        vcall!(g, play, w.vfunction32, false);
    }
}

/// port: 0051a750 FUN_0051a750
/// `Registered()`: the trial no longer counts as expired, the register link goes, and
/// (outside the screensaver) the play link shows once the bar is full; redraw.
pub fn FUN_0051a750(g: &mut G, this: Ptr) {
    g.title(this).ext_0xc0 = false;
    let reg = g.title(this).offset_0x4;
    vcall!(g, reg, w.vfunction32, false);
    let app = g.title(this).ext_0xd4;
    if !crate::game::win_fish_app::thunk_FUN_00479fc0(g, app) {
        let t = g.title(this).clone();
        vcall!(g, t.offset_0x0, w.vfunction32, t.field_0x8 == t.field_0xc);
    }
    vcall!(g, this, w.vfunction18);
}
