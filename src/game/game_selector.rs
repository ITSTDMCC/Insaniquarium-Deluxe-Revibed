//! `Sexy::GameSelector`: the main menu (Adventure, Time Trial, Challenge, Virtual Tank,
//! Options, Help, Hall of Fame, Quit) with Meryl blinking, the speech bubble describing the
//! hovered mode, the trophy and the "If this is not you" link; plus `GameSelectorOverlay`,
//! a full-screen transparent widget that draws the selector's overlay text above its buttons.
//!
//! `GameSelector_data` starts at object offset 0x8c; the object is 0xe8 bytes, larger than
//! the database's 0xd0 (fields `ext_0xNN`). The database names the selector's constructor
//! `GameSelectorOverlay::GameSelectorOverlay` (the overlay's constructor is inlined in it).

use crate::game::cheat_code::{self, CheatCode};
use crate::sexy::graphics::{
    FUN_00455880, FUN_00455890, FUN_004558c0, FUN_004558e0, FUN_00455870, FUN_00455cf0, FUN_00455d20, FUN_00456950, FUN_00456a00,
    FUN_004574a0, FUN_004574b0,
};
use crate::sexy::image_font as font;
use crate::sexy::prelude::*;

/// `GameSelector_data` (object offset 0x8c) plus the bytes past the database's class size.
#[derive(Debug, Clone, Default)]
pub struct GameSelector_data {
    /// +0x8c: the app.
    pub field_0x0: Ptr,
    /// +0x90: Adventure button (id 1).
    pub offset_0x4: Ptr,
    /// +0x94: Virtual Tank button (id 2).
    pub offset_0x8: Ptr,
    /// +0x98: Options button (id 3).
    pub offset_0xc: Ptr,
    /// +0x9c: Quit button (id 4).
    pub offset_0x10: Ptr,
    /// +0xa0: Challenge button (id 9).
    pub offset_0x14: Ptr,
    /// +0xa4: Time Trial button (id 8).
    pub offset_0x18: Ptr,
    /// +0xa8: Hall of Fame button (id 6).
    pub offset_0x1c: Ptr,
    /// +0xac: Help button (id 7).
    pub offset_0x20: Ptr,
    /// +0xb0: the "If this is not you, click here." link (id 9999).
    pub offset_0x24: Ptr,
    /// +0xb4: speech bubble shown (0 = welcome, 1..6 = hovered item).
    pub field_0x28: i32,
    /// +0xb8: cross-fade frames left after the bubble changed.
    pub field_0x2c: i32,
    /// +0xbc: the previous bubble (fading out).
    pub field_0x30: i32,
    /// +0xc0: updates the hovered item has differed from the bubble.
    pub field_0x34: i32,
    /// +0xc4: the overlay widget.
    pub offset_0x38: Ptr,
    /// +0xc8: the Konami code (up up down down left right left right b a).
    pub offset_0x3c: Option<Box<CheatCode>>,
    /// +0xcc: the "give" code.
    pub offset_0x40: Option<Box<CheatCode>>,
    /// +0xd0: trophy sparkle x.
    pub ext_0xd0: i32,
    /// +0xd4: trophy sparkle y.
    pub ext_0xd4: i32,
    /// +0xd8: trophy sparkle frame (0 = none).
    pub ext_0xd8: i32,
    /// +0xdc: Meryl's blink timer.
    pub ext_0xdc: i32,
    /// +0xe0: Meryl's tail timer.
    pub ext_0xe0: i32,
}

impl G {
    pub fn gs(&mut self, p: Ptr) -> &mut GameSelector_data {
        match &mut self.widget(p).ext {
            WExt::GameSelector(d) => d,
            e => panic!("{p} is not a GameSelector: {e:?}"),
        }
    }
}

/// `DAT_005e10a4`: bubble cross-fade length in updates.
pub const DAT_005e10a4: i32 = 5;

fn app_rand(g: &mut G, app: Ptr) -> u32 {
    let r = g.wfa(app).offset_0x84;
    crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(r))
}

/// One of the image buttons: id, images (normal is IMAGE_BLANK), label font and the two
/// label colors set in the constructor.
fn image_button(g: &mut G, id: i32, listener: Ptr, over: Ptr, down: Ptr, label: &[u8], c1: Color, c2: Color) -> Ptr {
    let b = crate::sexy::button_widget::new_button_widget(g, id, listener);
    g.w(b).offset_0x28 = true;
    let blank = g.res.DAT_005e8d80;
    let bd = g.btn(b);
    bd.offset_0x28 = blank;
    bd.offset_0x2c = over;
    bd.offset_0x30 = down;
    let f = g.res.DAT_005e8e18;
    vcall!(g, b, btn.vfunction72, f);
    if !label.is_empty() {
        g.btn(b).field_0x4 = label.to_vec();
    }
    let colors = &mut g.w(b).offset_0xc;
    colors[0] = c1;
    colors[1] = c2;
    b
}

/// port: 0052e070 GameSelectorOverlay::GameSelectorOverlay
/// `GameSelector(WinFishApp*)`.
pub fn GameSelectorOverlay(g: &mut G, param_1: Ptr) -> Ptr {
    let (wc, w) = crate::sexy::widget::Widget();
    let d = GameSelector_data { field_0x0: param_1, ..Default::default() };
    let this = g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__GameSelector_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::GameSelector(Box::new(d)) }),
    });
    let c1 = FUN_00433360(0xff, 0xf0, 0);
    let c2 = FUN_00433360(200, 200, 0xff);
    let (tank, tank_d) = (g.res.DAT_005e8cf8, g.res.DAT_005e8e00);
    let (mid, mid_d) = (g.res.DAT_005e8b88, g.res.DAT_005e8e9c);

    let b = image_button(g, 1, this, tank, tank_d, b"", c1, c2);
    g.gs(this).offset_0x4 = b;
    vcall!(g, b, w.vfunction41, 0x165, 0x30, 0xd9, 0x42);

    let b = image_button(g, 2, this, tank, tank_d, b"Virtual Tank", c1, c2);
    g.gs(this).offset_0x8 = b;
    vcall!(g, b, w.vfunction41, 0x165, 0x11f, 0xd9, 0x42);

    let b = image_button(g, 9, this, mid, mid_d, b"Challenge", c1, c2);
    g.gs(this).offset_0x14 = b;
    vcall!(g, b, w.vfunction41, 0x167, 0xd4, 0xd5, 0x30);

    let b = image_button(g, 8, this, mid, mid_d, b"Time Trial", c1, c2);
    g.gs(this).offset_0x18 = b;
    vcall!(g, b, w.vfunction41, 0x167, 0x8e, 0xd5, 0x30);

    let img = g.res.DAT_005e8ad8;
    let b = FUN_00506820(g, 3, this, b"Options", img);
    g.gs(this).offset_0xc = b;
    let h = g.wc(b).offset_0x38;
    vcall!(g, b, w.vfunction41, 0x145, 0x19c, 0x5c, h);

    let img = g.res.DAT_005e8dec;
    let b = FUN_00506820(g, 4, this, b"Quit", img);
    g.gs(this).offset_0x10 = b;
    let h = g.wc(b).offset_0x38;
    vcall!(g, b, w.vfunction41, 0x202, 0x19c, 0x59, h);

    let img = g.res.DAT_005e8c1c;
    let b = FUN_00506820(g, 6, this, b"Hall of Fame", img);
    g.gs(this).offset_0x1c = b;
    let h = g.wc(b).offset_0x38;
    vcall!(g, b, w.vfunction41, 0x191, 0x17c, 0x7f, h);

    let img = g.res.DAT_005e8d28;
    let b = FUN_00506820(g, 7, this, b"Help", img);
    g.gs(this).offset_0x20 = b;
    let h = g.wc(b).offset_0x38;
    vcall!(g, b, w.vfunction41, 0x1a3, 0x19c, 0x5d, h);

    // The overlay: a plain Widget with GameSelectorOverlay's vftable and the selector at +0x88.
    let (owc, mut ow) = crate::sexy::widget::Widget();
    ow.offset_0x1 = false;
    let overlay = g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::GameSelectorOverlay_vftable),
        node: Node::Widget(WidgetObj { wc: owc, w: ow, ext: WExt::GameSelectorOverlay(this) }),
    });
    g.wc(overlay).offset_0x3c = true;
    g.gs(this).offset_0x38 = overlay;
    vcall!(g, overlay, w.vfunction41, 0, 0, 0x280, 0x1e0);

    let r = app_rand(g, param_1);
    g.gs(this).ext_0xdc = (r % 0x96 + 100) as i32;
    let r = app_rand(g, param_1);
    g.gs(this).offset_0x24 = NULL;
    g.gs(this).ext_0xe0 = (r % 500 + 200) as i32;
    if !g.wfa(param_1).offset_0x157 && !crate::game::win_fish_app::FUN_0054b170(g, param_1, 2, 0) {
        crate::game::win_fish_app::FUN_0054b020(g, param_1);
        crate::game::win_fish_app::FUN_0054b1a0(g, param_1, 2, 0, false);
    }
    let profile = g.wfa(param_1).offset_0x18c;
    if profile != NULL {
        crate::game::profile::FUN_005013f0(g, profile);
    }
    FUN_0051eb70(g, this);
    let d = g.gs(this);
    d.field_0x28 = 0;
    d.field_0x2c = 0;
    d.field_0x30 = 0;
    d.field_0x34 = 0;
    let mut konami = cheat_code::FUN_00504d30();
    for k in [0x26, 0x26, 0x28, 0x28, 0x25, 0x27, 0x25, 0x27] {
        cheat_code::FUN_00505ba0(&mut konami, k);
    }
    cheat_code::FUN_00505bd0(&mut konami, b'b');
    cheat_code::FUN_00505bd0(&mut konami, b'a');
    let d = g.gs(this);
    d.offset_0x3c = Some(Box::new(konami));
    d.offset_0x40 = Some(Box::new(cheat_code::FUN_0050ff90(b"give")));
    d.ext_0xd8 = 0;
    this
}

/// port: 00506760 FUN_00506760
/// `MakeDialogButton(int theId, ButtonListener*, const string& theLabel, Font*)`: a
/// DialogButton on IMAGE_DIALOG_BUTTON... (`DAT_005e8c54`), as tall as the image, in the
/// given font (FONT_JUNGLEFEVER... `DAT_005e8cf0` when null), cels set up by `FUN_005034e0`.
pub fn FUN_00506760(g: &mut G, param_1: i32, param_2: Ptr, param_3: &[u8], param_4: Ptr) -> Ptr {
    let img = g.res.DAT_005e8c54;
    let b = crate::sexy::dialog_button::DialogButton(g, img, param_1, param_2);
    let f = if param_4 == NULL { g.res.DAT_005e8cf0 } else { param_4 };
    vcall!(g, b, btn.vfunction72, f);
    g.btn(b).field_0x4 = param_3.to_vec();
    let bimg = g.dialog_button(b).offset_0x0;
    g.wc(b).offset_0x38 = g.image(bimg).offset_0x24;
    FUN_005034e0(g, b);
    b
}

/// port: 00506820 FUN_00506820
/// `MakeDialogButton(int theId, ButtonListener*, const string& theLabel, Image*)`: a
/// DialogButton in FONT_JUNGLEFEVER... (`DAT_005e8cf0`), yellow label turning white on hover,
/// as tall as the image, its three cels set up by `FUN_005034e0`.
pub fn FUN_00506820(g: &mut G, param_1: i32, param_2: Ptr, param_3: &[u8], param_4: Ptr) -> Ptr {
    let b = crate::sexy::dialog_button::DialogButton(g, param_4, param_1, param_2);
    let f = g.res.DAT_005e8cf0;
    vcall!(g, b, btn.vfunction72, f);
    g.btn(b).field_0x4 = param_3.to_vec();
    let c = FUN_00433360(0xff, 0xf0, 0);
    vcall!(g, b, w.vfunction35, 0, c);
    let c = FUN_00433360(0xff, 0xff, 0xff);
    vcall!(g, b, w.vfunction35, 1, c);
    g.w(b).offset_0x28 = true;
    g.wc(b).offset_0x38 = g.image(param_4).offset_0x24;
    FUN_005034e0(g, b);
    b
}

/// port: 005034e0 FUN_005034e0
/// Sets a DialogButton up for a three-cel component image (normal, over, down side by
/// side): cel rects, 1-pixel press shift, label 2 pixels left, transparent.
pub fn FUN_005034e0(g: &mut G, param_1: Ptr) {
    let img = {
        let d = g.dialog_button(param_1);
        d.offset_0x4 = 1;
        d.offset_0x8 = 1;
        d.offset_0x0
    };
    g.w(param_1).offset_0x6 = true;
    g.wc(param_1).offset_0x3c = true;
    let cw = g.image(img).offset_0x20 / 3;
    let h = g.image(img).offset_0x24;
    let b = g.btn(param_1);
    b.offset_0x38 = Rect::new(0, 0, cw, h);
    b.offset_0x48 = Rect::new(cw, 0, cw, h);
    b.offset_0x58 = Rect::new(cw * 2, 0, cw, h);
    let d = g.dialog_button(param_1);
    d.offset_0x10 = 0;
    d.offset_0xc = -2;
}

/// port: 0051c2d0 Sexy::GameSelector::~GameSelector
pub fn dtor_GameSelector(g: &mut G, this: Ptr) {
    let d = g.gs(this).clone();
    for b in [d.offset_0x4, d.offset_0x8, d.offset_0xc, d.offset_0x10, d.offset_0x14, d.offset_0x18, d.offset_0x1c, d.offset_0x20, d.offset_0x38] {
        if b != NULL {
            vcall!(g, b, w.vfunction1, 1);
        }
    }
    g.gs(this).offset_0x3c = None;
    g.gs(this).offset_0x40 = None;
    let link = g.gs(this).offset_0x24;
    if link != NULL {
        vcall!(g, link, w.vfunction1, 1);
        g.gs(this).offset_0x24 = NULL;
    }
    crate::sexy::widget::dtor_Widget(g, this);
}

/// port: 0051e250 Sexy::GameSelector::deleting_destructor
pub fn deleting_destructor(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_GameSelector(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 00517910 Sexy::GameSelector::vfunction1
/// `ButtonPress(int theId, int theClickCount)`: the click sound (app vtable +0xd8).
pub fn vfunction1(g: &mut G, this: Ptr, _id: i32, _clicks: i32) {
    let app = g.gs(this).field_0x0;
    let s = g.res.DAT_005e8c40;
    crate::sexy::sexy_app_base::vfunction55(g, app, s);
}

/// "Not Yet!" message box (app vtable +0x120 `DoDialog`, id 0xe, modal, OK button).
fn not_yet(g: &mut G, app: Ptr, lines: &[u8]) {
    crate::game::win_fish_app::vfunction73(g, app, 0xe, true, b"Not Yet!", lines, b"OK", 3);
}

/// port: 0051f160 Sexy::GameSelector::vfunction3
/// `ButtonDepress(int theId)`.
pub fn vfunction3(g: &mut G, this: Ptr, param_1: i32) {
    let app = g.gs(this).field_0x0;
    g.wfa(app).offset_0x154 = false;
    match param_1 {
        1 => {
            g.wfa(app).offset_0x150 = 0;
            crate::game::win_fish_app::FUN_0054b360(g, app);
            crate::game::win_fish_app::FUN_00552380(g, app, true, true);
            return;
        }
        2 => {
            g.wfa(app).offset_0x150 = 5;
            crate::game::win_fish_app::FUN_0054b360(g, app);
            crate::game::win_fish_app::FUN_00552380(g, app, true, true);
            return;
        }
        3 => {
            crate::game::win_fish_app::FUN_0054c620(g, app, true);
            return;
        }
        4 => {
            crate::game::win_fish_app::FUN_005517f0(g, app);
            return;
        }
        6 => {
            crate::game::high_score_screen::FUN_005522d0(g, app);
            return;
        }
        7 => {
            crate::game::win_fish_app::FUN_0054c220(g, app, false);
            return;
        }
        8 => {
            let p = g.wfa(app).offset_0x18c;
            let pr = g.profile(p);
            if pr.field_0x1c < 2 && !pr.field_0x59 {
                not_yet(g, app, b"You'll need to complete a tank in Adventure Mode before this option becomes available.");
                return;
            }
            g.wfa(app).offset_0x150 = 1;
        }
        9 => {
            let p = g.wfa(app).offset_0x18c;
            if !g.profile(p).field_0x59 {
                not_yet(g, app, b"You'll need to beat Adventure Mode before this option becomes available.");
                return;
            }
            g.wfa(app).offset_0x150 = 4;
        }
        9999 => {
            crate::game::user_dialog::FUN_0054b5f0(g, app);
            return;
        }
        _ => return,
    }
    crate::game::win_fish_app::FUN_0054b360(g, app);
    if crate::game::win_fish_app::FUN_0054cb20(g, app) {
        return;
    }
    crate::game::tank_screen::FUN_0054c130(g, app);
}

/// port: 0051ef10 Sexy::GameSelector::vfunction21
/// `AddedToManager(WidgetManager*)`: adds the buttons, creates the "not you" link the first
/// time (sized to its text, which is then cleared: the selector draws that text itself), and
/// the overlay last.
pub fn vfunction21(g: &mut G, this: Ptr, manager: Ptr) {
    crate::sexy::widget_container::vfunction21(g, this, manager);
    let d = g.gs(this).clone();
    for b in [d.offset_0x4, d.offset_0x8, d.offset_0xc, d.offset_0x10, d.offset_0x14, d.offset_0x18, d.offset_0x1c, d.offset_0x20] {
        vcall!(g, manager, w.vfunction4, b);
    }
    if d.offset_0x24 == NULL {
        let f = g.res.DAT_005e8c20;
        let link = crate::sexy::button_widget::HyperlinkWidget(g, 9999, this);
        g.gs(this).offset_0x24 = link;
        vcall!(g, link, btn.vfunction72, f);
        g.hyperlink(link).offset_0x0 = FUN_00433360(0, 0, 0x78);
        g.hyperlink(link).field_0x10 = FUN_00433360(100, 100, 0xdc);
        g.w(link).offset_0x28 = true;
        g.btn(link).field_0x4 = b"If this is not you, click here.".to_vec();
        g.hyperlink(link).offset_0x20 = 1;
        vcall!(g, link, w.vfunction32, true);
        let label = g.btn(link).field_0x4.clone();
        let w = font::string_width(g, f, &label);
        let h = font::get_height(g, f);
        vcall!(g, link, w.vfunction41, 0x61, 0x5a, w, h);
        g.btn(link).field_0x4.clear();
    }
    let (link, overlay) = (g.gs(this).offset_0x24, g.gs(this).offset_0x38);
    vcall!(g, manager, w.vfunction4, link);
    vcall!(g, manager, w.vfunction4, overlay);
}

/// port: 00517660 Sexy::GameSelector::vfunction22
/// `RemovedFromManager(WidgetManager*)`.
pub fn vfunction22(g: &mut G, this: Ptr, manager: Ptr) {
    crate::sexy::widget_container::vfunction22(g, this, manager);
    let d = g.gs(this).clone();
    for b in [d.offset_0x4, d.offset_0x8, d.offset_0xc, d.offset_0x10, d.offset_0x14, d.offset_0x18, d.offset_0x1c, d.offset_0x20] {
        vcall!(g, manager, w.vfunction5, b);
    }
    if d.offset_0x24 != NULL {
        vcall!(g, manager, w.vfunction5, d.offset_0x24);
    }
    vcall!(g, manager, w.vfunction5, d.offset_0x38);
}

/// port: 00517610 FUN_00517610
/// `SetBubble(int)`: switches the speech bubble once the hovered item has differed from it
/// for more than five updates, starting the cross-fade.
pub fn FUN_00517610(g: &mut G, this: Ptr, param_1: i32) {
    let d = g.gs(this);
    if d.field_0x28 != param_1 {
        let n = d.field_0x34;
        d.field_0x34 = n + 1;
        if 4 < n {
            d.field_0x2c = DAT_005e10a4;
            d.field_0x30 = d.field_0x28;
            d.field_0x28 = param_1;
            d.field_0x34 = 0;
        }
    }
}

/// port: 0051ebe0 Sexy::GameSelector::vfunction23
/// `Update()`: Meryl's blink/tail timers, the trophy sparkle (placed on an opaque pixel of
/// the trophy), the hovered item's bubble, the fade, and the "not you" link's visibility.
pub fn vfunction23(g: &mut G, this: Ptr) {
    crate::sexy::widget_container::vfunction23(g, this);
    let app = g.gs(this).field_0x0;
    g.gs(this).ext_0xdc -= 1;
    if g.gs(this).ext_0xdc < 1 {
        let v = if app_rand(g, app) % 7 == 0 {
            app_rand(g, app) % 0xf + 0xf
        } else {
            app_rand(g, app) % 0x96 + 100
        };
        g.gs(this).ext_0xdc = v as i32;
    }
    g.gs(this).ext_0xe0 -= 1;
    if g.gs(this).ext_0xe0 < 1 {
        let v = app_rand(g, app) % 500 + 200;
        g.gs(this).ext_0xe0 = v as i32;
    }
    let frame = g.gs(this).ext_0xd8;
    if frame < 1 {
        let profile = g.wfa(app).offset_0x18c;
        if frame == 0 && profile != NULL && g.profile(profile).field_0x80 != 0 && 0x28 < g.wc(this).offset_0x24 {
            let d = g.gs(this);
            d.ext_0xd8 = 1;
            d.ext_0xd0 = 0xaa;
            d.ext_0xd4 = 0x136;
            let award = g.res.DAT_005e8ea0;
            let (mut x, mut y);
            let mut tries = 0;
            loop {
                x = (crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32) % g.image(award).offset_0x20;
                y = (crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32) % g.image(award).offset_0x24;
                if FUN_00500650(g, award, x, y) & 0xff00_0000 == 0xff00_0000 {
                    break;
                }
                tries += 1;
                if 100 <= tries {
                    break;
                }
            }
            g.gs(this).ext_0xd0 += x;
            g.gs(this).ext_0xd4 += y;
            let sparkle = g.res.DAT_005e8bb4;
            let cw = FUN_004574b0(g, sparkle);
            g.gs(this).ext_0xd0 += -(cw / 2);
            let ch = FUN_004574a0(g, sparkle);
            g.gs(this).ext_0xd4 += -(ch / 2);
        }
    } else {
        g.gs(this).ext_0xd8 = frame + 1;
        if 0x3b < frame + 1 {
            g.gs(this).ext_0xd8 = 0;
        }
    }
    vcall!(g, this, w.vfunction18);
    let d = g.gs(this).clone();
    let hot = |g: &mut G, b: Ptr| g.w(b).offset_0x5 || g.w(b).offset_0x4;
    let bubble = if hot(g, d.offset_0x4) {
        1
    } else if hot(g, d.offset_0x18) {
        2
    } else if hot(g, d.offset_0x14) {
        3
    } else if hot(g, d.offset_0x8) {
        4
    } else {
        let profile = g.wfa(app).offset_0x18c;
        let wm = g.wc(this).offset_0xc;
        let (mx, my) = {
            let m = crate::sexy::widget_manager::wm(g, wm);
            (m.offset_0x8c, m.offset_0x90)
        };
        if profile != NULL
            && 6 <= g.profile(profile).field_0xb0
            && FUN_004773f0(&Rect::new(0xdc, 0x154, 0x32, 0x32), mx, my)
            && g.sab(app).offset_0x320.is_empty()
        {
            if 0 < g.profile(profile).field_0x80 as u32 { 6 } else { 5 }
        } else {
            0
        }
    };
    FUN_00517610(g, this, bubble);
    if 0 < g.gs(this).field_0x2c {
        g.gs(this).field_0x2c -= 1;
    }
    let link = g.gs(this).offset_0x24;
    let profile = g.wfa(app).offset_0x18c;
    if g.w(link).offset_0x0 {
        if profile == NULL || g.gs(this).field_0x28 != 0 {
            vcall!(g, link, w.vfunction32, false);
            vcall!(g, this, w.vfunction18);
            FUN_0051eb70(g, this);
            return;
        }
    } else if profile != NULL && g.gs(this).field_0x28 == 0 {
        vcall!(g, link, w.vfunction32, true);
        vcall!(g, this, w.vfunction18);
    }
    FUN_0051eb70(g, this);
}

/// port: 004773f0 FUN_004773f0
/// `TRect<int>::Contains(int x, int y)`.
pub fn FUN_004773f0(this: &Rect, param_1: i32, param_2: i32) -> bool {
    this.mX <= param_1 && param_1 < this.mWidth + this.mX && this.mY <= param_2 && param_2 < this.mHeight + this.mY
}

/// port: 00500650 FUN_00500650
/// `GetPixel(MemoryImage*, int x, int y)`: 0 outside the image. (Palette images, +0x4c,
/// do not occur in the port: every loaded image has 32-bit bits.)
pub fn FUN_00500650(g: &mut G, param_1: Ptr, param_2: i32, param_3: i32) -> u32 {
    let img = g.image(param_1);
    if -1 < param_2 && -1 < param_3 && param_2 < img.offset_0x20 && param_3 < img.offset_0x24 {
        let i = (img.offset_0x20 * param_3 + param_2) as usize;
        return img.mBits[i];
    }
    0
}

/// port: 0051eb70 FUN_0051eb70
/// The Adventure button's label: "Bonus Adventure" once adventure is beaten, "Start
/// Adventure" at tank 1-1, else "Adventure".
pub fn FUN_0051eb70(g: &mut G, this: Ptr) {
    let app = g.gs(this).field_0x0;
    let profile = g.wfa(app).offset_0x18c;
    let b = g.gs(this).offset_0x4;
    if profile != NULL {
        let p = g.profile(profile).clone();
        if p.field_0x59 {
            g.btn(b).field_0x4 = b"Bonus Adventure".to_vec();
            return;
        }
        if p.field_0x1c == 1 && p.field_0x20 == 1 {
            g.btn(b).field_0x4 = b"Start Adventure".to_vec();
            return;
        }
    }
    g.btn(b).field_0x4 = b"Adventure".to_vec();
}

/// port: 0051e690 Sexy::GameSelector::vfunction27
/// `Draw(Graphics*)`: background, trophy with sparkle, Meryl's animations, then the speech
/// bubble (cross-faded while it changes).
pub fn vfunction27(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    let bg = g.res.DAT_005e8ba8;
    FUN_00455d20(gfx, g, bg, 0, 0);
    let app = g.gs(this).field_0x0;
    let profile = g.wfa(app).offset_0x18c;
    if profile != NULL && 5 < g.profile(profile).field_0xb0 {
        let img = if g.profile(profile).field_0x80 != 0 { g.res.DAT_005e8dd4 } else { g.res.DAT_005e8ea0 };
        FUN_00455d20(gfx, g, img, 0xaa, 0x136);
        let frame = g.gs(this).ext_0xd8;
        let sparkle = g.res.DAT_005e8bb4;
        if 0 < frame && frame <= g.image(sparkle).offset_0x2c {
            FUN_004558c0(gfx, 1);
            FUN_004558e0(gfx, true);
            FUN_00455890(gfx, FUN_00433320(0xffffff));
            let (x, y) = (g.gs(this).ext_0xd0, g.gs(this).ext_0xd4);
            FUN_00456950(gfx, g, sparkle, x, y, frame - 1);
            FUN_004558c0(gfx, 0);
        }
    }
    let (blink, tail) = (g.res.DAT_005e8ad4, g.res.DAT_005e8a40);
    let (tb, tt) = (g.gs(this).ext_0xdc, g.gs(this).ext_0xe0);
    FUN_00456a00(gfx, g, blink, 0x8b, 0xc4, tb);
    FUN_00456a00(gfx, g, tail, 0x27, 0x12d, tt);
    let d = g.gs(this).clone();
    if 0 < d.field_0x2c {
        let a = (d.field_0x2c * 0xff) / DAT_005e10a4;
        FUN_0051e280(g, this, gfx, d.field_0x28, 0xff - a, true);
        FUN_0051e280(g, this, gfx, d.field_0x30, a, false);
        return;
    }
    FUN_0051e280(g, this, gfx, d.field_0x28, 0xff, true);
}

/// port: 0051e280 FUN_0051e280
/// `DrawBubble(Graphics*, int theBubble, int theAlpha, bool drawBubbleImage)`: 1..6 = the
/// hovered item's description word-wrapped in the bubble; 0 = "Welcome to/back" with the
/// player's name and the "not you" text over the link.
pub fn FUN_0051e280(g: &mut G, this: Ptr, gfx: &mut Graphics, param_2: i32, param_3: i32, param_4: bool) {
    let app = g.gs(this).field_0x0;
    let profile = g.wfa(app).offset_0x18c;
    let mut name: Vec<u8> = Vec::new();
    let (mut tank, mut level, mut beaten) = (1, 1, false);
    if profile != NULL {
        let p = g.profile(profile).clone();
        name = p.field_0x24.clone();
        level = p.field_0x20;
        tank = p.field_0x1c;
        beaten = p.field_0x59;
    }
    if param_4 {
        if name.is_empty() && param_2 < 1 {
            return;
        }
        let bubble = g.res.DAT_005e8eac;
        FUN_00455d20(gfx, g, bubble, 0x2d, 0x14);
    }
    if 0 < param_2 {
        let mut rect = Rect::new(0x40, 0x28, 0xd6, 0x3c);
        let f = g.res.DAT_005e8ea8;
        FUN_00455880(gfx, f);
        FUN_00455890(gfx, CRect(0, 0, 100, param_3));
        let text: &[u8] = match param_2 {
            1 => {
                if tank == 1 && level == 1 && !beaten {
                    b"New to Insaniquarium?\nClick here to start your aquatic adventure!"
                } else {
                    b"Feed fish and fight aliens!\nClick here to continue\nyour adventure..."
                }
            }
            2 => b"How much money can you earn before time runs out?",
            3 => b"Can you fend off the increasingly difficult aliens?",
            4 => b"Buy and raise your own\ncustom fish, then use them\nas a screensaver!",
            5 => b"Wow!  Nice trophy!",
            6 => b"The pets wanted you to have this solid gold trophy since you are now the undisputed champion of all Insaniquarium!",
            _ => b"",
        };
        let h = vcall!(g, this, w.vfunction66, gfx, rect.mWidth, text, 0x14);
        rect.mY += (rect.mHeight - h) / 2;
        vcall!(g, this, w.vfunction65, gfx, rect, text, 0x14, 0);
        return;
    }
    if name.is_empty() {
        return;
    }
    FUN_004558e0(gfx, true);
    FUN_00455890(gfx, CRect(0xff, 0xff, 0xff, param_3));
    let welcome_to = g.res.DAT_005e8ef8;
    let x = (0xfa - g.image(welcome_to).offset_0x20) / 2 + 0x2d;
    let img = if profile != NULL && g.profile(profile).field_0x4c == 0 { welcome_to } else { g.res.DAT_005e8d98 };
    FUN_00455d20(gfx, g, img, x, 0x21);
    FUN_004558e0(gfx, false);
    let mut line = name.clone();
    line.extend_from_slice(b"!");
    let f = g.res.DAT_005e8a28;
    let w = font::string_width(g, f, &line);
    let x = (0xfa - w) / 2 + 0x2d;
    FUN_00455880(gfx, f);
    FUN_00455890(gfx, CRect(0, 0, 100, param_3));
    FUN_00455cf0(gfx, g, &line, x, 0x4d);
    let link = g.gs(this).offset_0x24;
    if g.w(link).offset_0x0 {
        let lf = g.btn(link).offset_0x24;
        FUN_00455880(gfx, lf);
        let c = if g.w(link).offset_0x5 { g.hyperlink(link).field_0x10 } else { g.hyperlink(link).offset_0x0 };
        FUN_00455890(gfx, CRect(c.mRed, c.mGreen, c.mBlue, param_3));
        let f = FUN_00455870(gfx);
        let asc = font::get_ascent(g, f);
        let (lx, ly, lh) = (g.wc(link).offset_0x2c, g.wc(link).offset_0x30, g.wc(link).offset_0x38);
        let y = (asc + lh) / 2 - 1 + ly;
        FUN_00455cf0(gfx, g, b"If this is not you, click here.", lx, y);
    }
}

/// port: 00517720 Sexy::GameSelector::vfunction31
/// `OrderInManagerChanged()`: keeps the overlay, the link and the buttons just above the
/// selector (`mWidgetManager->PutInfront(w, this)`).
pub fn vfunction31(g: &mut G, this: Ptr) {
    let wm = g.wc(this).offset_0xc;
    let d = g.gs(this).clone();
    vcall!(g, wm, w.vfunction15, d.offset_0x38, this);
    if d.offset_0x24 != NULL {
        vcall!(g, wm, w.vfunction15, d.offset_0x24, this);
    }
    for b in [d.offset_0x20, d.offset_0x1c, d.offset_0x18, d.offset_0x14, d.offset_0x10, d.offset_0xc, d.offset_0x8, d.offset_0x4] {
        vcall!(g, wm, w.vfunction15, b, this);
    }
}

/// port: 005175b0 FUN_005175b0
/// Label color for the button text drawn over a button (ESI): pale cyan, shifted one pixel
/// while the button is held down (outputs in EBX/EDI).
pub fn FUN_005175b0(g: &mut G, gfx: &mut Graphics, button: Ptr) -> (i32, i32) {
    FUN_00455890(gfx, CRect(0xe1, 0xfa, 0xfa, 0xff));
    if g.w(button).offset_0x4 && g.w(button).offset_0x5 {
        return (1, 1);
    }
    (0, 0)
}

/// port: 0051e810 Sexy::GameSelector::vfunction45
/// `DrawOverlay(Graphics*)`: under the Adventure button, the player's tank ("Tank 2-3",
/// "Bonus Level 4", or the final-boss name); under Virtual Tank, the shell count.
pub fn vfunction45(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    let app = g.gs(this).field_0x0;
    let profile = g.wfa(app).offset_0x18c;
    if profile == NULL {
        return;
    }
    let f = g.res.DAT_005e8cf0;
    FUN_00455880(gfx, f);
    let mut label: Vec<u8> = Vec::new();
    let adv = g.gs(this).offset_0x4;
    let (dx, dy) = FUN_005175b0(g, gfx, adv);
    let p = g.profile(profile).clone();
    if p.field_0x1c == 5 && p.field_0x20 == 1 {
        label = FUN_00506a50(g, p.field_0x50 + 1);
    } else if p.field_0x20 == 6 {
        label = format!("Bonus Level {}", p.field_0x1c).into_bytes();
    } else if !(p.field_0x1c == 1 && p.field_0x20 == 1) {
        label = format!("Tank {}-{}", p.field_0x1c, p.field_0x20).into_bytes();
    }
    if !label.is_empty() {
        let f = FUN_00455870(gfx);
        let w = font::string_width(g, f, &label);
        FUN_00455cf0(gfx, g, &label, (dx - w / 2) + 0x1d1, dy + 0x6b);
    }
    let vt = g.gs(this).offset_0x8;
    let (dx, dy) = FUN_005175b0(g, gfx, vt);
    let shells = g.profile(profile).field_0x48;
    if 0 < shells {
        let s = format!("{shells} Shells").into_bytes();
        let f = g.res.DAT_005e8cf0;
        let w = font::string_width(g, f, &s);
        FUN_00455cf0(gfx, g, &s, (dx - w / 2) + 0x1d1, dy + 0x15a);
    }
}

/// port: 00506a50 FUN_00506a50
/// The final boss's name for the n-th time: "FINAL BOSS", then "2nd FINAL BOSS", ...
pub fn FUN_00506a50(g: &mut G, param_1: i32) -> Vec<u8> {
    if param_1 < 2 {
        return b"FINAL BOSS".to_vec();
    }
    let suffix = crate::game::game_selector::FUN_00500e10(g, param_1);
    let mut s = format!("{param_1}").into_bytes();
    s.extend_from_slice(&suffix);
    s.extend_from_slice(b" FINAL BOSS");
    s
}

/// port: 005178b0 Sexy::GameSelector::vfunction48
/// `KeyChar(char)`: feeds the Konami and "give" codes.
pub fn vfunction48(g: &mut G, this: Ptr, c: u8) {
    let app = g.gs(this).field_0x0;
    if g.sab(app).field_0x5a4 {
        return;
    }
    if cheat_code::FUN_00504180(g.gs(this).offset_0x3c.as_mut().unwrap(), c) {
        FUN_005177e0(g, this);
        return;
    }
    if cheat_code::FUN_00504180(g.gs(this).offset_0x40.as_mut().unwrap(), c) {
        crate::game::user_dialog::FUN_0054f0e0(g, app);
    }
}

/// port: 00517850 Sexy::GameSelector::vfunction49
/// `KeyDown(KeyCode)`: feeds the Konami and "give" codes.
pub fn vfunction49(g: &mut G, this: Ptr, key: i32) {
    let app = g.gs(this).field_0x0;
    if g.sab(app).field_0x5a4 {
        return;
    }
    if cheat_code::FUN_005041f0(g.gs(this).offset_0x3c.as_mut().unwrap(), key as u8) {
        FUN_005177e0(g, this);
        return;
    }
    if cheat_code::FUN_005041f0(g.gs(this).offset_0x40.as_mut().unwrap(), key as u8) {
        crate::game::user_dialog::FUN_0054f0e0(g, app);
    }
}

/// port: 005177e0 FUN_005177e0
/// The Konami code: with a trophy (progress over 5), starts mode 3 with the virtual-tank
/// unlock level raised to at least 11.
pub fn FUN_005177e0(g: &mut G, this: Ptr) {
    let app = g.gs(this).field_0x0;
    let profile = g.wfa(app).offset_0x18c;
    if profile != NULL && 5 < g.profile(profile).field_0xb0 {
        g.wfa(app).offset_0x150 = 3;
        if g.profile(profile).field_0xb4 < 0xb {
            g.profile(profile).field_0xb4 = 0xb;
        }
        crate::game::win_fish_app::FUN_0054b360(g, app);
        crate::game::win_fish_app::FUN_0054cbf0(g, app);
    }
}

/// port: 0051ada0 Sexy::GameSelector::vfunction55
/// `MouseDown(int x, int y, int theClickCount)`: a click on Meryl makes her wag soon.
pub fn vfunction55(g: &mut G, this: Ptr, x: i32, y: i32, _clicks: i32) {
    if 0x13 < x && x < 0x136 && 0x95 < y && y < 0x1fe && 0xf < g.gs(this).ext_0xe0 {
        g.gs(this).ext_0xe0 = 0xf;
    }
}

/// port: 0051ad70 BoardOverlay::deleting_destructor
/// The overlay's deleting destructor (shared by the identical overlay classes).
pub fn deleting_destructor__0051ad70(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    crate::sexy::widget::dtor_Widget(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 00518910 BonusScreenOverlay::vfunction27
/// The overlay's `Draw(Graphics*)`: its owner's `DrawOverlay(g)` (vtable +0xb0); shared by
/// the identical overlay classes.
pub fn vfunction27__00518910(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    let owner = match &g.widget(this).ext {
        WExt::GameSelectorOverlay(p) => *p,
        e => panic!("{this} is not an overlay: {e:?}"),
    };
    vcall!(g, owner, w.vfunction45, gfx);
}

/// port: 00500e10 FUN_00500e10
/// The English ordinal suffix of `param_1` ("st", "nd", "rd", "th"; 11..13 take "th").
pub fn FUN_00500e10(_g: &mut G, param_1: i32) -> Vec<u8> {
    if (param_1 / 10) % 10 != 1 {
        match param_1 % 10 {
            1 => return b"st".to_vec(),
            2 => return b"nd".to_vec(),
            3 => return b"rd".to_vec(),
            _ => {}
        }
    }
    b"th".to_vec()
}
