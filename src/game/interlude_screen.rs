//! `Sexy::InterludeScreen`: the ending after the final tank. A deep-blue backdrop with
//! bubbles and fish drifting by scrolls up through the closing story, Angie bringing back
//! the pets lost in the final battle, a word on how many were lost, the cast (each with its
//! picture) and the credits, with "Back To Main Menu" in the corner; a click once the roll
//! has ended (or the link) goes back to the menu. Plus the app functions that open and
//! close it.
//!
//! `InterludeScreen_data` starts at object offset 0x8c (after the ButtonListener vftable at
//! 0x88); the object is 0xb8 bytes, past the database's 0xa0 (fields `ext_0xNN`). The
//! database names the constructor `InterludeScreenOverlay::InterludeScreenOverlay` (the
//! overlay's constructor is inlined in it); the overlay is the shared overlay class.
//!
//! The scroll state lives in globals, as in the original: the lost pets (`DAT_005e8fe0`,
//! `DAT_005e9080`, filled when the final tank ends), the scroll positions at which each
//! comes back (`DAT_005e9040`, worked out on the first draw while `DAT_005e1600` is set),
//! where the credits start (`DAT_005e9084`, also found on the first draw) and the length of
//! the roll (`DAT_005e907c`).

use crate::sexy::graphics::{FUN_004558c0, FUN_004558e0, FUN_00455870, FUN_00455880, FUN_00455890, FUN_00455920, FUN_00455cf0, FUN_00455d20, FUN_004560a0, FUN_004563d0, FUN_00456980};
use crate::sexy::prelude::*;
use crate::sexy::types::{CRect, FUN_00433320};

/// `DAT_005e19ec`: how far above the text line each returning pet floats (three rows of
/// five).
const DAT_005e19ec: [i32; 15] = [90, 70, 80, 90, 70, 70, 90, 80, 70, 90, 90, 70, 80, 90, 70];
/// `DAT_005e1a28`: each returning pet's x (the rows run right to left, left to right,
/// right to left, following Angie).
const DAT_005e1a28: [i32; 15] = [480, 360, 240, 120, 0, 10, 130, 250, 370, 490, 480, 360, 240, 120, 0];

/// `InterludeScreen_data` (object offset 0x8c) plus the bytes past the database's class size.
#[derive(Debug, Clone, Default)]
pub struct InterludeScreen_data {
    /// +0x8c the app.
    pub offset_0x0: Ptr,
    /// +0x90 "Back To Main Menu" (HyperlinkWidget, id 0).
    pub offset_0x4: Ptr,
    /// +0x94 the overlay widget.
    pub offset_0x8: Ptr,
    /// +0x98 the bubbles in front (drawn by the overlay).
    pub offset_0xc: Ptr,
    /// +0x9c the bubbles and fish behind.
    pub offset_0x10: Ptr,
    /// +0xa0 the constructor's second argument (always 0).
    pub ext_0xa0: i32,
    /// +0xa4 how far the roll has scrolled (one pixel per update, up to `DAT_005e907c`).
    pub ext_0xa4: i32,
    /// +0xa8 the x of the text line being drawn.
    pub ext_0xa8: i32,
    /// +0xac the y of the text line being drawn (0x21c at the top of the roll).
    pub ext_0xac: i32,
    /// +0xb0 the line spacing (0x23).
    pub ext_0xb0: i32,
    /// +0xb4 Angie's and Amp's glow, -1..1 (its magnitude is used).
    pub ext_0xb4: f32,
}

impl G {
    pub fn interlude(&mut self, p: Ptr) -> &mut InterludeScreen_data {
        match &mut self.widget(p).ext {
            WExt::InterludeScreen(d) => d,
            e => panic!("{p} is not an InterludeScreen: {e:?}"),
        }
    }
}

/// port: 005256e0 InterludeScreenOverlay::InterludeScreenOverlay
/// `InterludeScreen(WinFishApp*, int)`: full screen; two bubble layers over the whole
/// screen (10 bubbles, the back one with fish swimming up and no clipping), both run ahead
/// 500 updates; "Back To Main Menu" (grey, white on hover) at the bottom right; the
/// overlay; the lost pets laid out (`FUN_00518650`); the roll's length.
pub fn InterludeScreenOverlay(g: &mut G, param_1: Ptr, param_2: i32) -> Ptr {
    use crate::game::board_parts::{BubbleMgr, FUN_00500120, FUN_00500180, FUN_00504b00, FUN_00512080};
    let (wc, w) = crate::sexy::widget::Widget();
    let d = InterludeScreen_data { offset_0x0: param_1, ext_0xa0: param_2, ..Default::default() };
    let this = g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__InterludeScreen_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::InterludeScreen(Box::new(d)) }),
    });
    let area = Rect::new(0, -0x14, 0x280, 0x208);
    let front = BubbleMgr(g);
    g.interlude(this).offset_0xc = front;
    FUN_00500120(g, front, &area);
    FUN_00500180(g, front, 10, 3);
    FUN_00512080(g, front);
    let back = BubbleMgr(g);
    g.interlude(this).offset_0x10 = back;
    FUN_00500120(g, back, &area);
    FUN_00500180(g, back, 10, 3);
    FUN_00504b00(g, back, -1.0);
    let b = g.bubble_mgr(back);
    b.offset_0x38 &= !0xff;
    FUN_00512080(g, back);
    let (aw, ah) = (g.sab(param_1).field_0xb8, g.sab(param_1).field_0xbc);
    let wc = g.wc(this);
    wc.offset_0x2c = 0;
    wc.offset_0x30 = 0;
    wc.offset_0x34 = aw;
    let d = g.interlude(this);
    d.ext_0xb4 = 0.0;
    g.wc(this).offset_0x38 = ah;
    let d = g.interlude(this);
    d.ext_0xac = 0x21c;
    d.ext_0xa8 = 0;
    d.ext_0xb0 = 0x23;

    let link = crate::sexy::button_widget::HyperlinkWidget(g, 0, this);
    g.interlude(this).offset_0x4 = link;
    g.btn(link).field_0x4 = b"Back To Main Menu".to_vec();
    let f = g.res.DAT_005e8cf0;
    vcall!(g, link, btn.vfunction72, f);
    g.hyperlink(link).offset_0x0 = FUN_00433320(0x808080);
    g.hyperlink(link).field_0x10 = FUN_00433320(0xffffff);
    g.hyperlink(link).offset_0x20 = 0;
    vcall!(g, link, w.vfunction41, 0x1ea, 0x1bb, 0x96, 0x21);
    let label = g.btn(link).field_0x4.clone();
    let lw = crate::sexy::image_font::string_width(g, f, &label);
    let lh = crate::sexy::image_font::get_height(g, f);
    vcall!(g, link, w.vfunction41, 0x276 - (lw + 0x14), 0x1cc - lh / 2, lw + 0x14, lh);

    // The overlay: a plain Widget with InterludeScreenOverlay's vftable and the screen at +0x88.
    let (mut owc, mut ow) = crate::sexy::widget::Widget();
    owc.offset_0x3c = true;
    ow.offset_0x1 = false;
    let overlay = g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::InterludeScreenOverlay_vftable),
        node: Node::Widget(WidgetObj { wc: owc, w: ow, ext: WExt::GameSelectorOverlay(this) }),
    });
    g.interlude(this).offset_0x8 = overlay;
    vcall!(g, overlay, w.vfunction41, 0, 0, 0x280, 0x1e0);
    FUN_00518650(g);
    g.globals.DAT_005e9084 = 0;
    let sp = g.interlude(this).ext_0xb0;
    g.globals.DAT_005e907c = sp * 6 + 0x1c39 + sp * 6;
    this
}

/// port: 00518650 FUN_00518650
/// Lays out the pets lost in the final battle for the roll: the unused slots of the 24 are
/// -1, and with fewer than four lost they move one place along so the first row of the
/// ending isn't crowded at its start; the return times are to be worked out again.
pub fn FUN_00518650(g: &mut G) {
    if g.globals.DAT_005e8fe0.len() < 0x28 {
        g.globals.DAT_005e8fe0.resize(0x28, 0);
    }
    let n = g.globals.DAT_005e9080;
    if n < 0x18 {
        for i in n.max(0)..0x18 {
            g.globals.DAT_005e8fe0[i as usize] = -1;
        }
    }
    g.globals.DAT_005e1600 = true;
    if n < 4 {
        let p = &mut g.globals.DAT_005e8fe0;
        p[3] = p[2];
        p[2] = p[1];
        p[1] = p[0];
        p[0] = -1;
    }
}

/// port: 005185a0 Sexy::InterludeScreen::~InterludeScreen
pub fn dtor_InterludeScreen(g: &mut G, this: Ptr) {
    let d = g.interlude(this).clone();
    for b in [d.offset_0xc, d.offset_0x10] {
        if b != NULL {
            crate::game::board_parts::deleting_destructor__00505420(g, b, 1);
        }
    }
    for p in [d.offset_0x4, d.offset_0x8] {
        if p != NULL {
            vcall!(g, p, w.vfunction1, 1);
        }
    }
    crate::sexy::widget::dtor_Widget(g, this);
}

/// port: 0051aee0 Sexy::InterludeScreen::deleting_destructor
pub fn deleting_destructor(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_InterludeScreen(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 005186b0 Sexy::InterludeScreen::vfunction21
/// `AddedToManager(WidgetManager*)`: the overlay and the link.
pub fn vfunction21(g: &mut G, this: Ptr, param_1: Ptr) {
    crate::sexy::widget_container::vfunction21(g, this, param_1);
    let d = g.interlude(this).clone();
    vcall!(g, param_1, w.vfunction4, d.offset_0x8);
    vcall!(g, param_1, w.vfunction4, d.offset_0x4);
}

/// port: 005186f0 Sexy::InterludeScreen::vfunction22
/// `RemovedFromManager(WidgetManager*)`.
pub fn vfunction22(g: &mut G, this: Ptr, param_1: Ptr) {
    crate::sexy::widget_container::vfunction22(g, this, param_1);
    let d = g.interlude(this).clone();
    vcall!(g, param_1, w.vfunction5, d.offset_0x8);
    vcall!(g, param_1, w.vfunction5, d.offset_0x4);
}

/// port: 00518730 Sexy::InterludeScreen::vfunction45
/// `DrawOverlay(Graphics*)`: the front bubbles, and black bars 40 pixels deep along the top
/// and bottom.
pub fn vfunction45(g: &mut G, this: Ptr, param_1: &mut Graphics) {
    let front = g.interlude(this).offset_0xc;
    crate::game::board_parts::FUN_00504b60(g, front, param_1);
    FUN_00455890(param_1, FUN_00433320(0));
    let (w, h) = (g.wc(this).offset_0x34, g.wc(this).offset_0x38);
    FUN_00455920(param_1, 0, 0, w, 0x28);
    FUN_00455920(param_1, 0, h - 0x28, w, 0x28);
}

/// port: 00518790 Sexy::InterludeScreen::vfunction55
/// `MouseDown(x, y, clicks)`: once the roll has ended, a click goes back to the menu (the
/// ButtonListener's `ButtonDepress(0)`).
pub fn vfunction55(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32) {
    crate::sexy::widget::vfunction55(g, this, param_1, param_2, param_3);
    if g.globals.DAT_005e907c <= g.interlude(this).ext_0xa4 {
        vcall!(g, this, bl.vfunction3, 0);
    }
}

/// port: 005187d0 Sexy::InterludeScreen::vfunction3
/// `ButtonDepress(int)`: the music stops, the screen closes and the menu opens.
pub fn vfunction3(g: &mut G, this: Ptr, _param_1: i32) {
    let app = g.interlude(this).offset_0x0;
    crate::game::win_fish_app::FUN_0054b020(g, app);
    FUN_0054ad70(g, app);
    crate::game::win_fish_app::FUN_00552100(g, app);
}

/// port: 0051afd0 Sexy::InterludeScreen::vfunction23
/// `Update()`: the ending music starts on the first update. While the app is active: the
/// bubbles move, the screen is redrawn; at 3000 the music fades into the credits theme;
/// the glow steps by 0.01 (wrapping from 1 back to -1); the roll scrolls a pixel with the
/// fish swimming up through the middle of the screen, then (at the end) the fish keep to
/// the whole screen and stop rising; each pet chimes as it comes back; from the credits on
/// three fish swim at a time (none before).
pub fn vfunction23(g: &mut G, this: Ptr) {
    use crate::game::board_parts::{FUN_00500150, FUN_005001a0, FUN_00504b00, FUN_005110e0};
    crate::sexy::widget_container::vfunction23(g, this);
    let app = g.interlude(this).offset_0x0;
    if g.wc(this).offset_0x24 == 1 {
        crate::game::win_fish_app::FUN_0054b020(g, app);
        crate::game::win_fish_app::FUN_0054b1a0(g, app, 2, 5, false);
    }
    if !g.sab(app).field_0x4ca {
        return;
    }
    let d = g.interlude(this).clone();
    FUN_005110e0(g, d.offset_0xc);
    FUN_005110e0(g, d.offset_0x10);
    vcall!(g, this, w.vfunction18);
    if g.interlude(this).ext_0xa4 == 3000 {
        crate::game::win_fish_app::FUN_0054b230(g, app, 2, 0, 0.002);
        crate::game::win_fish_app::FUN_0054b1e0(g, app, 3, 0, 0.008, false);
    }
    let f = (g.interlude(this).ext_0xb4 as f64 + 0.009999999776482582) as f32;
    g.interlude(this).ext_0xb4 = f;
    if 1.0 <= f {
        g.interlude(this).ext_0xb4 = -1.0;
    }
    let back = d.offset_0x10;
    if g.interlude(this).ext_0xa4 < g.globals.DAT_005e907c {
        FUN_00500150(g, back, &Rect::new(0, 0xc8, 0x280, 0xf0));
        FUN_00504b00(g, back, -1.0);
        g.interlude(this).ext_0xa4 += 1;
    } else {
        FUN_00500150(g, back, &Rect::new(0, 0x1e, 0x280, 0x1a4));
        FUN_00504b00(g, back, 0.0);
    }
    if !g.globals.DAT_005e1600 {
        let at = g.interlude(this).ext_0xa4;
        for i in 0..15 {
            if at == g.globals.DAT_005e9040[i] {
                let s = g.res.DAT_005e8bf8;
                crate::sexy::sexy_app_base::vfunction55(g, app, s);
            }
        }
    }
    let credits = g.globals.DAT_005e9084;
    if 0 < credits {
        if credits <= g.interlude(this).ext_0xa4 {
            FUN_005001a0(g, back, 3, 3);
            return;
        }
        FUN_005001a0(g, back, 0, 0);
    }
}

/// port: 0051bf60 FUN_0051bf60
/// `DrawLine(Graphics*, std::string text, int linesDown, int small, int align)`: moves the
/// line down by `linesDown` line spacings and draws the text there, in the heading font
/// (pale cyan) or the small one (`small`, aqua), centred on the screen (align 0), the left
/// column (1) or the right column (2); any other align keeps the last line's x.
pub fn FUN_0051bf60(g: &mut G, this: Ptr, param_1: &mut Graphics, param_2: &[u8], param_3: i32, param_4: i32, param_5: i32) {
    let d = g.interlude(this);
    d.ext_0xac += d.ext_0xb0 * param_3;
    let (r, gg, b) = if param_4 == 0 {
        let f = g.res.DAT_005e8a9c;
        FUN_00455880(param_1, f);
        (0xd7, 0xff, 0xff)
    } else {
        let f = g.res.DAT_005e8bbc;
        FUN_00455880(param_1, f);
        (0x96, 0xfa, 0xfa)
    };
    FUN_00455890(param_1, CRect(r, gg, b, 0xff));
    let span = match param_5 {
        0 => Some(0x28a),
        1 => Some(0x18b),
        2 => Some(0x375),
        _ => None,
    };
    if let Some(span) = span {
        let f = FUN_00455870(param_1);
        let w = crate::sexy::image_font::string_width(g, f, param_2);
        g.interlude(this).ext_0xa8 = (span - w) / 2;
    }
    let (x, y) = (g.interlude(this).ext_0xa8, g.interlude(this).ext_0xac);
    FUN_00455cf0(param_1, g, param_2, x, y);
}

/// port: 005187f0 FUN_005187f0
/// One lost pet coming back: from its return time (`param_2` > 0, scroll position reached,
/// and before 1700) it fades in at (x, y) over 51 updates, on the animation frame that
/// suits it (most use the 2-update cycle; Itchy, Rhubarb, Seymour and Shrapnel the
/// 4-update one; Stinky and Niko the ping-pong); Itchy sits 20 higher, Brinkley 40 further
/// left and lower.
pub fn FUN_005187f0(g: &mut G, this: Ptr, param_1: &mut Graphics, param_2: i32, param_3: i32, param_4: i32, param_5: i32, param_6: i32, param_7: i32, param_8: i32) {
    let at = g.interlude(this).ext_0xa4;
    if !(0 < param_2 && (param_3 as u32) < 0x18 && param_2 <= at && at < 0x6a4) {
        return;
    }
    let cel = match param_3 {
        1 | 6 => param_8,
        5 | 10 | 11 | 14 => param_7,
        _ => param_6,
    };
    let (mut x, mut y) = (param_4, param_5);
    if param_3 == 1 {
        y -= 0x14;
    } else if param_3 == 0x10 {
        x -= 0x28;
        y += 0x28;
    }
    let img = crate::sexy::res::FUN_005016a0(g, param_3 + 0xca) as Ptr;
    let mut a = (at - param_2) * 5;
    if 0xff < a {
        a = 0xff;
    }
    FUN_004558e0(param_1, true);
    FUN_00455890(param_1, CRect(0xff, 0xff, 0xff, a));
    FUN_00456980(param_1, g, img, x, y, cel, 0);
    FUN_004558e0(param_1, false);
}

/// port: 0051af10 FUN_0051af10
/// Angie (register arguments: the frame in EAX, y in EBX): her 80x80 frame `param_1` at
/// (`param_2`, `param_3`), mirrored when asked, then her glow added over it at the glow's
/// strength.
pub fn FUN_0051af10(g: &mut G, gfx: &mut Graphics, param_1: i32, param_2: i32, param_3: i32, param_4: f32, param_5: bool) {
    let src = Rect::new(param_1 * 0x50, 0, 0x50, 0x50);
    let angie = g.res.DAT_005e8e8c as Ptr;
    FUN_004560a0(gfx, g, angie, param_3, param_2, &src, param_5);
    FUN_004558c0(gfx, 1);
    FUN_004558e0(gfx, true);
    let a = crate::sexy::crt::ftol(param_4.abs() as f64 * 255.0) as i32;
    FUN_00455890(gfx, CRect(0xff, 0xff, 0xff, a));
    let glow = g.res.DAT_005e8c7c as Ptr;
    FUN_004560a0(gfx, g, glow, param_3, param_2, &src, param_5);
    FUN_004558c0(gfx, 0);
    FUN_004558e0(gfx, false);
}

/// `Graphics::DrawImageCel(img, x, y, col, row)` on a resource image.
fn cel(g: &mut G, gfx: &mut Graphics, img: Ptr, x: i32, y: i32, col: i32, row: i32) {
    FUN_00456980(gfx, g, img, x, y, col, row);
}

/// port: 00525a60 Sexy::InterludeScreen::vfunction27
/// `Draw(Graphics*)`: the deep-blue backdrop and the back bubbles, then the roll scrolled up
/// by the scroll position: the closing story; Angie's three passes bringing back the lost
/// pets (the first draw works out when each comes back; those not lost never do); a word on
/// how many were lost; the cast with their pictures (Amp glowing, Angie with her glow; the
/// four secret pets as "?" until earned); the credits (the first draw notes where they
/// start).
pub fn vfunction27(g: &mut G, this: Ptr, param_1: &mut Graphics) {
    let gfx = param_1;
    let t = |g: &mut G, this: Ptr, gfx: &mut Graphics, s: &[u8], adv: i32, small: i32, align: i32| {
        FUN_0051bf60(g, this, gfx, s, adv, small, align)
    };
    FUN_00455890(gfx, CRect(0x19, 0x69, 0xc3, 0xff));
    let (w, h) = (g.wc(this).offset_0x34, g.wc(this).offset_0x38);
    FUN_00455920(gfx, 0, 0, w, h);
    let back = g.interlude(this).offset_0x10;
    crate::game::board_parts::FUN_00504b60(g, back, gfx);
    let cnt = g.interlude(this).ext_0xa4;
    FUN_004563d0(gfx, 0, -cnt);
    let c20 = cnt % 0x14;
    let f2 = c20 / 2;
    let f4 = (cnt % 0x28) / 4;
    let tri = if 9 < c20 { 0x13 - c20 } else { c20 };
    {
        let d = g.interlude(this);
        d.ext_0xac = 0x21c;
        d.ext_0xa8 = 0;
    }
    t(g, this, gfx, b"Congratulations!", 0, 0, 0);
    t(g, this, gfx, b"You've successfully vanquished the alien hordes!", 2, 1, 0);
    t(g, this, gfx, b"The epic conflict between fish and evil has", 2, 1, 0);
    t(g, this, gfx, b"finally come to a close.", 1, 1, 0);
    t(g, this, gfx, b"Thanks to you, aquarium owners can", 2, 1, 0);
    t(g, this, gfx, b"sleep easy tonight!", 1, 1, 0);
    t(g, this, gfx, b"Many a pet gave their lives in the final battle,", 3, 1, 0);
    t(g, this, gfx, b"but do not grieve, little one!", 1, 1, 0);
    t(g, this, gfx, b"Angie purposefully sat out of the fight, and", 2, 1, 0);
    t(g, this, gfx, b"word is she's got a few tricks up her sleeves...", 1, 1, 0);

    // Angie's three passes, starting 535, 280 and 25 pixels above the new line.
    g.interlude(this).ext_0xac += 0x96;
    let y = g.interlude(this).ext_0xac;
    let (a, b, c) = (y - 0x217, y - 0x118, y - 0x19);
    let x1 = crate::sexy::crt::ftol(640.0 - 3.0 * (cnt - a) as f64) as i32;
    let x2 = crate::sexy::crt::ftol(3.0 * (cnt - b) as f64 - 80.0) as i32;
    let x3 = crate::sexy::crt::ftol(640.0 - 3.0 * (cnt - c) as f64) as i32;
    if g.globals.DAT_005e1600 {
        if g.globals.DAT_005e8fe0.len() < 0x28 {
            g.globals.DAT_005e8fe0.resize(0x28, 0);
        }
        // When Angie (3 pixels per update) reaches each spot.
        let af = a as f32 as f64;
        for i in 0..15 {
            g.globals.DAT_005e9040[i] = crate::sexy::crt::ftol((0x280 - DAT_005e1a28[i]) as f64 / 3.0 + af) as i32;
        }
        let bf = b as f32 as f64;
        for i in 0..5 {
            g.globals.DAT_005e9040[5 + i] = crate::sexy::crt::ftol((DAT_005e1a28[5 + i] + 0x50) as f64 / 3.0 + bf) as i32;
        }
        let cf = c as f32 as f64;
        for i in 0..5 {
            g.globals.DAT_005e9040[10 + i] = crate::sexy::crt::ftol((0x280 - DAT_005e1a28[10 + i]) as f64 / 3.0 + cf) as i32;
        }
        for i in 0..15 {
            if g.globals.DAT_005e8fe0[i] == -1 {
                g.globals.DAT_005e9040[i] = -100;
            }
        }
        g.globals.DAT_005e1600 = false;
    }
    let phase = g.interlude(this).ext_0xb4;
    let lost = g.globals.DAT_005e9080;
    let passes: [(i32, i32, i32, bool, i32); 3] = [(0, x1, -0x32, false, 0), (5, x2, 0x64, true, 0x96), (10, x3, 0xfa, false, 0x12c)];
    for (k, &(first, x, dy, mirror, rowy)) in passes.iter().enumerate() {
        let first = first as usize;
        if lost <= (k as i32) * 5 {
            break;
        }
        let y = g.interlude(this).ext_0xac;
        FUN_0051af10(g, gfx, f2, y + dy, x, phase, mirror);
        for i in first..first + 5 {
            let y = g.interlude(this).ext_0xac;
            let (trig, pet) = (g.globals.DAT_005e9040[i], g.globals.DAT_005e8fe0[i]);
            FUN_005187f0(g, this, gfx, trig, pet, DAT_005e1a28[i], y - DAT_005e19ec[i] + rowy, f2, f4, tri);
        }
    }

    let after = g.interlude(this).ext_0xac + 0x96;
    let lost = g.globals.DAT_005e9080;
    if lost == 0 {
        t(g, this, gfx, b"Wait a minute!", 0, 1, 0);
        t(g, this, gfx, b"You didn't lose any pets!!", 1, 1, 0);
        t(g, this, gfx, b"How did you do that?", 1, 1, 0);
        t(g, this, gfx, b"That's really amazing!", 1, 1, 0);
        t(g, this, gfx, b"Seriously, though...", 3, 1, 0);
        t(g, this, gfx, b"How did you do that??", 1, 1, 0);
        t(g, this, gfx, b"Fine, keep it to yourself.", 3, 1, 0);
    } else if lost == 1 {
        t(g, this, gfx, b"My goodness!  That's it?", 5, 1, 0);
        t(g, this, gfx, b"You only lost one pet?!", 1, 1, 0);
        t(g, this, gfx, b"That's unbelievable!!", 1, 1, 0);
        t(g, this, gfx, b"How the heck did that happen?!", 2, 1, 0);
        t(g, this, gfx, b"You rule this game!!", 1, 1, 0);
    } else if lost < 4 {
        t(g, this, gfx, b"Woah, hold on...", 5, 1, 0);
        let s = format!("only {lost} pets died?!");
        t(g, this, gfx, s.as_bytes(), 1, 1, 0);
        t(g, this, gfx, b"That's incredible!!", 2, 1, 0);
        t(g, this, gfx, b"Good shooting, there!", 1, 1, 0);
    } else if lost < 6 {
        t(g, this, gfx, b"Hmmm... actually it looks like", 5, 1, 0);
        t(g, this, gfx, b"not that many pets died.", 1, 1, 0);
        t(g, this, gfx, b"Great work!!", 2, 1, 0);
    } else if lost <= 10 {
        t(g, this, gfx, b"Also, you saved a fair number of pets yourself.", 7, 1, 0);
        t(g, this, gfx, b"Good job!", 1, 1, 0);
    }
    g.interlude(this).ext_0xac = after;

    t(g, this, gfx, b"Thanks for playing!", 8, 1, 0);
    t(g, this, gfx, b"We'll see you next game!", 1, 1, 0);
    t(g, this, gfx, b"I N S A N I Q U A R I U M", 4, 0, 0);
    t(g, this, gfx, b"Cast", 4, 0, 0);
    let r = g.res.clone();
    let yy = |g: &mut G, this: Ptr, dy: i32| g.interlude(this).ext_0xac - dy;
    t(g, this, gfx, b"Guppy", 4, 1, 1);
    let y = yy(g, this, 0x5a);
    cel(g, gfx, r.DAT_005e8ab8, 0xa0, y, f2, 0);
    t(g, this, gfx, b"Carnivore", 0, 1, 2);
    let y = yy(g, this, 0x5a);
    cel(g, gfx, r.DAT_005e8ab8, 0x190, y, f2, 4);
    t(g, this, gfx, b"Starcatcher", 4, 1, 1);
    let y = yy(g, this, 0x5a);
    cel(g, gfx, r.DAT_005e8e04, 0xa0, y, f2, 0);
    t(g, this, gfx, b"Guppycruncher", 0, 1, 2);
    let y = yy(g, this, 0x64);
    cel(g, gfx, r.DAT_005e8c98, 0x190, y, f2, 0);
    t(g, this, gfx, b"Beetlemuncher", 4, 1, 1);
    let y = yy(g, this, 0x5a);
    cel(g, gfx, r.DAT_005e8da8, 0xa0, y, f2, 0);
    t(g, this, gfx, b"Breeder", 0, 1, 2);
    let y = yy(g, this, 0x5a);
    cel(g, gfx, r.DAT_005e8d4c, 0x190, y, f2, 6);
    t(g, this, gfx, b"Ultravore", 5, 1, 0);
    let y = yy(g, this, 0xaa);
    cel(g, gfx, r.DAT_005e8cf4, 0xf0, y, f2, 0);
    t(g, this, gfx, b"Sylvester", 8, 1, 1);
    let y = yy(g, this, 0xb4);
    cel(g, gfx, r.DAT_005e8ebc, 0x78, y, f2, 0);
    t(g, this, gfx, b"Balrog", 0, 1, 2);
    let y = yy(g, this, 0xb4);
    cel(g, gfx, r.DAT_005e8cec, 0x168, y, f2, 0);
    t(g, this, gfx, b"Gus", 6, 1, 1);
    let y = yy(g, this, 0xb4);
    cel(g, gfx, r.DAT_005e8ae4, 0x78, y, f2, 0);
    t(g, this, gfx, b"Destructor", 0, 1, 2);
    let y = yy(g, this, 0xb4);
    cel(g, gfx, r.DAT_005e8ec8, 0x168, y, f2, 0);
    t(g, this, gfx, b"Ulysses", 6, 1, 1);
    let y = yy(g, this, 0xb4);
    cel(g, gfx, r.DAT_005e8e58, 0x78, y, f4, 0);
    t(g, this, gfx, b"Psychosquid", 0, 1, 2);
    let y = yy(g, this, 0xa5);
    cel(g, gfx, r.DAT_005e8c4c, 0x168, y, f2, 0);
    t(g, this, gfx, b"Bilaterus", 6, 1, 1);
    // Bilaterus: the head, six body segments and the tail.
    let bil = r.DAT_005e8f00;
    for &(x, dy, col, row) in &[(0xff, 0x5a, 9, 4), (0xe1, 0x5f, 2, 7), (0xc3, 0x64, 2, 6), (0xa5, 0x5f, 2, 6), (0x87, 0x55, 2, 7), (0x69, 0x50, 2, 6), (0x4b, 0x55, 2, 7)] {
        let y = yy(g, this, dy);
        cel(g, gfx, bil, x, y, col, row);
    }
    let y = yy(g, this, 0x5a);
    cel(g, gfx, bil, 0x2d, y, tri, 2);
    t(g, this, gfx, b"Cyrax", 0, 1, 2);
    let y = yy(g, this, 0xb4);
    cel(g, gfx, r.DAT_005e8cb8, 0x168, y, f2, 0);
    t(g, this, gfx, b"Stinky", 7, 1, 1);
    let y = yy(g, this, 0x5a);
    cel(g, gfx, r.DAT_005e8be0, 0xa0, y, f2, 0);
    t(g, this, gfx, b"Niko", 0, 1, 2);
    let y = yy(g, this, 0x69);
    cel(g, gfx, r.DAT_005e8e44, 0x190, y, tri, 0);
    t(g, this, gfx, b"Itchy", 4, 1, 1);
    let y = yy(g, this, 0x5a);
    cel(g, gfx, r.DAT_005e8aec, 0xa0, y, f2, 0);
    t(g, this, gfx, b"Prego", 0, 1, 2);
    let y = yy(g, this, 0x69);
    cel(g, gfx, r.DAT_005e8ca0, 0x190, y, f2, 0);
    t(g, this, gfx, b"Zorf", 4, 1, 1);
    let y = yy(g, this, 0x69);
    cel(g, gfx, r.DAT_005e8e30, 0xa0, y, f2, 0);
    t(g, this, gfx, b"Clyde", 0, 1, 2);
    let y = yy(g, this, 0x5f);
    cel(g, gfx, r.DAT_005e8bd0, 0x190, y, f4, 0);
    t(g, this, gfx, b"Vert", 4, 1, 1);
    let y = yy(g, this, 0x5a);
    cel(g, gfx, r.DAT_005e8d38, 0xa0, y, tri, 0);
    t(g, this, gfx, b"Rufus", 0, 1, 2);
    let y = yy(g, this, 0x5a);
    cel(g, gfx, r.DAT_005e8b60, 0x190, y, f2, 0);
    t(g, this, gfx, b"Meryl", 4, 1, 1);
    let y = yy(g, this, 0x69);
    cel(g, gfx, r.DAT_005e8b08, 0xa0, y, f2, 0);
    t(g, this, gfx, b"Wadsworth", 0, 1, 2);
    let y = yy(g, this, 0x5a);
    cel(g, gfx, r.DAT_005e8b30, 0x190, y, f2, 0);
    t(g, this, gfx, b"SEYMOUR", 4, 1, 1);
    let y = yy(g, this, 0x5a);
    cel(g, gfx, r.DAT_005e8b0c, 0xa0, y, f4, 0);
    t(g, this, gfx, b"SHRAPNEL", 0, 1, 2);
    let y = yy(g, this, 0x64);
    cel(g, gfx, r.DAT_005e8c2c, 0x190, y, f4, 0);
    t(g, this, gfx, b"GUMBO", 4, 1, 1);
    let y = yy(g, this, 0x5a);
    cel(g, gfx, r.DAT_005e8cb4, 0xa0, y, f2, 0);
    t(g, this, gfx, b"BLIP", 0, 1, 2);
    let y = yy(g, this, 0x5a);
    cel(g, gfx, r.DAT_005e8a14, 0x190, y, f2, 0);
    t(g, this, gfx, b"RHUBARB", 4, 1, 1);
    let y = yy(g, this, 0x64);
    cel(g, gfx, r.DAT_005e8a98, 0xa0, y, f4, 0);
    t(g, this, gfx, b"NIMBUS", 0, 1, 2);
    let y = yy(g, this, 0x5a);
    cel(g, gfx, r.DAT_005e8a08, 0x190, y, f2, 0);
    t(g, this, gfx, b"AMP", 4, 1, 1);
    let y = yy(g, this, 0x4b);
    cel(g, gfx, r.DAT_005e8de8, 0x78, y, f2, 0);
    // Amp's glow: twice the glow's strength, up to full.
    FUN_004558e0(gfx, true);
    FUN_004558c0(gfx, 1);
    let m = g.interlude(this).ext_0xb4.abs();
    let a = if 255.0 > 2.0 * m as f64 * 255.0 { crate::sexy::crt::ftol(2.0 * m as f64 * 255.0) as i32 } else { 255 };
    FUN_00455890(gfx, CRect(0xff, 0xff, 200, a));
    let y = yy(g, this, 0x4b);
    cel(g, gfx, r.DAT_005e8ce8, 0x78, y, f2, 0);
    FUN_004558c0(gfx, 0);
    FUN_004558e0(gfx, false);
    t(g, this, gfx, b"GASH", 0, 1, 2);
    let y = yy(g, this, 0x64);
    cel(g, gfx, r.DAT_005e8bfc, 0x190, y, f2, 0);
    t(g, this, gfx, b"ANGIE", 4, 1, 1);
    let y = yy(g, this, 0x64);
    let phase = g.interlude(this).ext_0xb4;
    FUN_0051af10(g, gfx, f2, y, 0xa0, phase, false);
    t(g, this, gfx, b"PRESTO", 0, 1, 2);
    let y = yy(g, this, 0x5a);
    cel(g, gfx, r.DAT_005e8d60, 0x190, y, f2, 0);

    // The four secret pets, as "?" until earned.
    let app = g.interlude(this).offset_0x0;
    let secret: [(usize, &[u8], i32, i32, Ptr, Ptr); 4] = [
        (0x14, b"BRINKLEY", 1, 0xa0, r.DAT_005e8b4c, r.DAT_005e8cac),
        (0x15, b"NOSTRADAMUS", 2, 0x190, r.DAT_005e8ed4, r.DAT_005e8ee4),
        (0x16, b"STANLEY", 1, 0xa0, r.DAT_005e8d00, r.DAT_005e8d68),
        (0x17, b"WALTER", 2, 0x190, r.DAT_005e8e50, r.DAT_005e8db0),
    ];
    for (k, &(pet, name, align, x, img, unknown)) in secret.iter().enumerate() {
        if k % 2 == 0 {
            let d = g.interlude(this);
            d.ext_0xac += d.ext_0xb0 * 6;
        }
        let profile = g.wfa(app).offset_0x18c;
        if crate::game::profile::FUN_00501410(g, profile, pet) {
            t(g, this, gfx, name, 0, 1, align);
            let y = yy(g, this, 0x64);
            cel(g, gfx, img, x, y, f2, 0);
        } else {
            t(g, this, gfx, b"?", 0, 0, align);
            let y = yy(g, this, 0x96);
            FUN_00455d20(gfx, g, unknown, if align == 1 { 0x96 } else { 0x186 }, y);
        }
    }
    if g.globals.DAT_005e9084 == 0 {
        g.globals.DAT_005e9084 = g.interlude(this).ext_0xac;
    }
    t(g, this, gfx, b"Credits", 7, 0, 0);
    t(g, this, gfx, b"Game Design", 3, 0, 0);
    t(g, this, gfx, b"George Fan", 1, 1, 0);
    t(g, this, gfx, b"Producer", 3, 0, 0);
    t(g, this, gfx, b"Jason Kapalka", 1, 1, 0);
    t(g, this, gfx, b"Sukhbir Sidhu", 1, 1, 0);
    t(g, this, gfx, b"Programming", 3, 0, 0);
    t(g, this, gfx, b"George Fan", 1, 1, 0);
    t(g, this, gfx, b"Thien Tran", 1, 1, 0);
    t(g, this, gfx, b"Brian Rothstein", 1, 1, 0);
    t(g, this, gfx, b"Art", 3, 0, 0);
    t(g, this, gfx, b"Josh Langley", 1, 1, 0);
    t(g, this, gfx, b"Walter Wilson", 1, 1, 0);
    t(g, this, gfx, b"Character Design", 3, 0, 0);
    t(g, this, gfx, b"George Fan", 1, 1, 0);
    t(g, this, gfx, b"Music", 3, 0, 0);
    t(g, this, gfx, b"Jonne Valtonen", 1, 1, 0);
    t(g, this, gfx, b"George Fan", 1, 1, 0);
    t(g, this, gfx, b"PopCap Framework", 3, 0, 0);
    t(g, this, gfx, b"Brian Fiete", 1, 1, 0);
    t(g, this, gfx, b"Biz Dev", 3, 0, 0);
    t(g, this, gfx, b"Don Walters", 1, 1, 0);
    t(g, this, gfx, b"QA", 3, 0, 0);
    t(g, this, gfx, b"Eric Harman", 1, 1, 0);
    t(g, this, gfx, b"Shawn Conard", 1, 1, 0);
    t(g, this, gfx, b"Brenna Flood", 1, 1, 0);
    t(g, this, gfx, b"Chad Zoellner", 1, 1, 0);
    t(g, this, gfx, b"Marmot Salesman", 3, 0, 0);
    t(g, this, gfx, b"David P. Wycliff", 1, 1, 0);
    t(g, this, gfx, b"Special Thanks", 3, 0, 0);
    t(g, this, gfx, b"PopCap Beta Testers", 1, 1, 0);
    t(g, this, gfx, b"Tysen Henderson", 1, 1, 0);
    t(g, this, gfx, b"Hai-Pao Fan", 1, 1, 0);
    t(g, this, gfx, b"Jan Campbell", 1, 1, 0);
    t(g, this, gfx, b"brought to you by:", 7, 1, 0);
    t(g, this, gfx, b"Flying Bear Entertainment", 1, 0, 0);
    t(g, this, gfx, b"and", 1, 1, 0);
    t(g, this, gfx, b"PopCap Games", 1, 0, 0);
    FUN_004563d0(gfx, 0, cnt);
}

/// port: 0054bd80 FUN_0054bd80
/// `ShowInterludeScreen()`: closes the dialogs and any ending, opens the ending full
/// screen (+0x81c) on top, with the focus (or behind the board, if one is up).
pub fn FUN_0054bd80(g: &mut G, this: Ptr) {
    crate::game::win_fish_app::FUN_0054b4c0(g, this);
    FUN_0054ad70(g, this);
    let s = InterludeScreenOverlay(g, this, 0);
    g.wfa(this).offset_0xf0 = s;
    let (w, h) = (g.sab(this).field_0xb8, g.sab(this).field_0xbc);
    vcall!(g, s, w.vfunction41, 0, 0, w, h);
    let wm = g.sab(this).offset_0x318;
    vcall!(g, wm, w.vfunction4, s);
    vcall!(g, wm, w.vfunction9, s);
    let board = g.wfa(this).offset_0x4;
    if board == NULL {
        vcall!(g, wm, w.vfunction13, s);
        return;
    }
    vcall!(g, wm, w.vfunction15, s, board);
}

/// port: 0054ad70 FUN_0054ad70
/// `RemoveInterludeScreen()` (+0x81c).
pub fn FUN_0054ad70(g: &mut G, this: Ptr) {
    let s = g.wfa(this).offset_0xf0;
    if s != NULL {
        let wm = g.sab(this).offset_0x318;
        vcall!(g, wm, w.vfunction5, s);
        crate::sexy::sexy_app_base::vfunction35(g, this, s);
        g.wfa(this).offset_0xf0 = NULL;
    }
}
