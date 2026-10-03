//! `Sexy::HatchScreen`: the end-of-level screen. The egg wobbles, cracks and hatches into
//! the pet the level awards ("You have found:"), which then swims in its frame with its
//! name and a description; "Please Wait..." becomes "Click Here to Continue". After the
//! last adventure level the egg is the boss's, and "Menu" is hidden.
//!
//! `HatchScreen_data` starts at object offset 0x8c (the ButtonListener part is at 0x88;
//! the port passes the object itself as the listener). The overlay widget
//! (`HatchScreenOverlay`) draws `vfunction45` above the buttons.

use crate::game::board_parts::{FUN_00500120, FUN_00500180, FUN_005110e0, FUN_00512080};
use crate::game::game_selector::FUN_00506820;
use crate::sexy::graphics::{FUN_00455800, FUN_00455870, FUN_00455880, FUN_004558e0, FUN_00455890, FUN_00455cf0, FUN_00455d20, FUN_00456340, FUN_004563f0, FUN_00456950};
use crate::sexy::image_font as font;
use crate::sexy::prelude::*;
use crate::sexy::types::CRect;

/// `HatchScreen_data` (object offset 0x8c, 44 bytes).
#[derive(Debug, Clone, Default)]
pub struct HatchScreen_data {
    /// +0x8c the app.
    pub field_0x0: Ptr,
    /// +0x90 the bubbles.
    pub offset_0x4: Ptr,
    /// +0x94 updates since the screen opened.
    pub field_0x8: i32,
    /// +0x98 the pet found (0..0x17; 999 the boss).
    pub field_0xc: i32,
    /// +0x9c the egg's x jitter (0..3).
    pub field_0x10: i32,
    /// +0xa0 the egg's y jitter (0..1).
    pub field_0x14: i32,
    /// +0xa4 the boss egg's y (rising from 480).
    pub field_0x18: i32,
    /// +0xac "Please Wait..." / "Click Here to Continue" (id 99).
    pub offset_0x20: Ptr,
    /// +0xb0 "Menu" (id 100).
    pub offset_0x24: Ptr,
    /// +0xb4 the overlay widget.
    pub offset_0x28: Ptr,
}

impl G {
    pub fn hatch(&mut self, p: Ptr) -> &mut HatchScreen_data {
        match &mut self.widget(p).ext {
            WExt::HatchScreen(d) => d,
            e => panic!("{p} is not a HatchScreen: {e:?}"),
        }
    }
}

/// Whether the profile is at tank 5, level `param_1` (after the last adventure level).
fn profile_at_5(g: &mut G, app: Ptr, param_1: i32) -> bool {
    let profile = g.wfa(app).offset_0x18c;
    let pr = g.profile(profile);
    pr.field_0x1c == 5 && pr.field_0x20 == param_1
}

/// port: 0051f450 HatchScreenOverlay::HatchScreenOverlay
/// `HatchScreen(WinFishApp*, int pet)` (the decompiler named the constructor after the
/// overlay it creates): bubbles in (190, 220, 258, 200), the two buttons and the overlay,
/// marks a play in progress, and starts song 0x36 of music 2.
pub fn HatchScreenOverlay(g: &mut G, param_1: Ptr, param_2: i32) -> Ptr {
    let (mut wc, w) = crate::sexy::widget::Widget();
    let bm = crate::game::board_parts::BubbleMgr(g);
    wc.offset_0x2c = 0;
    wc.offset_0x30 = 0;
    let (aw, ah) = (g.sab(param_1).field_0xb8, g.sab(param_1).field_0xbc);
    wc.offset_0x34 = aw;
    wc.offset_0x38 = ah;
    let d = HatchScreen_data { field_0x0: param_1, offset_0x4: bm, field_0x8: 0, field_0xc: param_2, ..Default::default() };
    let this = g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__HatchScreen_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::HatchScreen(Box::new(d)) }),
    });
    FUN_00500120(g, bm, &Rect::new(0xbe, 0xdc, 0x102, 200));
    FUN_00500180(g, bm, 10, 5);
    FUN_00512080(g, bm);
    g.wfa(param_1).offset_0x158 = true;

    let img = g.res.DAT_005e8c1c;
    let cont = FUN_00506820(g, 99, this, b"Please Wait...", img);
    g.hatch(this).offset_0x20 = cont;
    let f = g.res.DAT_005e8a5c;
    vcall!(g, cont, btn.vfunction72, f);
    g.w(cont).offset_0x1 = false;
    g.w(cont).offset_0xc[0] = CRect(0xff, 0xf0, 0, 0xff);
    let h = g.wc(cont).offset_0x38;
    vcall!(g, cont, w.vfunction41, 0xba, 0x1bd, 0x108, h);
    let wm = g.sab(param_1).offset_0x318;
    vcall!(g, wm, w.vfunction12, cont);

    let menu = FUN_00506820(g, 100, this, b"Menu", img);
    g.hatch(this).offset_0x24 = menu;
    let h = g.wc(menu).offset_0x38;
    vcall!(g, menu, w.vfunction41, 0x20d, 4, 0x50, h);
    if profile_at_5(g, param_1, 2) {
        g.w(menu).offset_0x0 = false;
    }

    let (mut owc, mut ow) = crate::sexy::widget::Widget();
    ow.offset_0x1 = false;
    owc.offset_0x3c = true;
    owc.offset_0x34 = 0x280;
    owc.offset_0x38 = 0x1e0;
    let ov = g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::HatchScreenOverlay_vftable),
        node: Node::Widget(WidgetObj { wc: owc, w: ow, ext: WExt::GameSelectorOverlay(this) }),
    });
    let d = g.hatch(this);
    d.offset_0x28 = ov;
    d.field_0x10 = 0;
    d.field_0x14 = 0;
    d.field_0x18 = 0x1e0;
    crate::game::win_fish_app::FUN_0054b020(g, param_1);
    crate::game::win_fish_app::FUN_0054b1a0(g, param_1, 2, 0x36, false);
    this
}

/// port: 00517930 Sexy::HatchScreen::~HatchScreen
/// Deletes the bubbles, the buttons and the overlay.
pub fn dtor_HatchScreen(g: &mut G, this: Ptr) {
    let d = g.hatch(this).clone();
    if d.offset_0x4 != NULL {
        crate::game::board_parts::deleting_destructor__00505420(g, d.offset_0x4, 1);
    }
    for b in [d.offset_0x20, d.offset_0x24, d.offset_0x28] {
        if b != NULL {
            vcall!(g, b, w.vfunction1, 1);
        }
    }
    crate::sexy::widget::dtor_Widget(g, this);
}

/// port: 0051ade0 Sexy::HatchScreen::deleting_destructor
pub fn deleting_destructor(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_HatchScreen(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 005179e0 Sexy::HatchScreen::vfunction21
/// `AddedToManager(WidgetManager*)`: adds the buttons and the overlay.
pub fn vfunction21(g: &mut G, this: Ptr, param_1: Ptr) {
    crate::sexy::widget_container::vfunction21(g, this, param_1);
    let d = g.hatch(this).clone();
    for b in [d.offset_0x20, d.offset_0x24, d.offset_0x28] {
        vcall!(g, param_1, w.vfunction4, b);
    }
}

/// port: 00517a30 Sexy::HatchScreen::vfunction22
/// `RemovedFromManager(WidgetManager*)`: removes them again.
pub fn vfunction22(g: &mut G, this: Ptr, param_1: Ptr) {
    crate::sexy::widget_container::vfunction22(g, this, param_1);
    let d = g.hatch(this).clone();
    for b in [d.offset_0x24, d.offset_0x20, d.offset_0x28] {
        vcall!(g, param_1, w.vfunction5, b);
    }
}

/// port: 00517c70 Sexy::HatchScreen::vfunction3
/// `ButtonDepress(int id)`: 99 continues (the next level, or the selector in modes 1 and
/// 4; the interlude after the last adventure level), 100 goes to the menu.
pub fn vfunction3(g: &mut G, this: Ptr, param_1: i32) {
    let app = g.hatch(this).field_0x0;
    if param_1 == 99 {
        crate::game::win_fish_app::FUN_0054adf0(g, app);
        let mode = g.wfa(app).offset_0x150;
        if mode == 1 || mode == 4 {
            crate::game::win_fish_app::FUN_00552230(g, app);
            return;
        }
        if !profile_at_5(g, app, 2) {
            crate::game::win_fish_app::FUN_00552380(g, app, false, false);
            return;
        }
        crate::game::interlude_screen::FUN_0054bd80(g, app);
    } else if param_1 == 100 {
        crate::game::win_fish_app::FUN_0054adf0(g, app);
        if profile_at_5(g, app, 2) {
            crate::game::interlude_screen::FUN_0054bd80(g, app);
            return;
        }
        crate::game::win_fish_app::FUN_00552100(g, app);
    }
}

/// port: 00517a80 Sexy::HatchScreen::vfunction45
/// `DrawOverlay(Graphics*)`: the egg frame's top; then the boss's egg rising in its frame,
/// or the eggshell pieces flying apart for 60 updates after the hatch.
pub fn vfunction45(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    let img = g.res.DAT_005e8bf4;
    FUN_00455d20(gfx, g, img, 0xf0, 0x3c);
    let d = g.hatch(this).clone();
    if profile_at_5(g, d.field_0x0, 1) {
        if 0x8b < d.field_0x8 {
            let mut g2 = FUN_00455800(gfx);
            FUN_00456340(&mut g2, 0xee, d.field_0x18, 0xa0, 0xa0);
            let img = g.res.DAT_005e8cb8;
            FUN_00455d20(&mut g2, g, img, ((d.field_0x8 % 0x14) / 2) * -0xa0 + 0xee, d.field_0x18);
        }
    } else if 0x8b < d.field_0x8 && d.field_0x8 < 200 {
        const DY: [i32; 8] = [-10, -15, 3, -13, 10, 5, 14, 12];
        const DX: [i32; 8] = [1, -11, -14, 12, -15, 11, 13, -1];
        let img = g.res.DAT_005e8a24;
        for i in 0..8 {
            let t = d.field_0x8 - 0x8c;
            FUN_00456950(gfx, g, img, DX[i] * t + 0x10c, DY[i] * t + 0x46, i as i32);
        }
    }
}

/// port: 00520760 Sexy::HatchScreen::vfunction23
/// `Update()`: bubbles; the egg's jitter; a click skips to the hatch; the hatch sound at
/// 141 (the boss's own after the last level); three crack sounds and a pop in mode 1; the
/// boss egg's rise and bob; "Click Here to Continue" at 170.
pub fn vfunction23(g: &mut G, this: Ptr) {
    let d = g.hatch(this).clone();
    FUN_005110e0(g, d.offset_0x4);
    vcall!(g, this, w.vfunction18);
    let app = d.field_0x0;
    let rng = g.wfa(app).offset_0x84;
    let a = crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng)) & 3;
    g.hatch(this).field_0x10 = a as i32;
    let b = crate::sexy::mt_rand::FUN_0040aeb0(g.mtrand(rng)) & 1;
    g.hatch(this).field_0x14 = b as i32;
    if g.w(this).offset_0x4 && g.hatch(this).field_0x8 < 0x8c {
        g.hatch(this).field_0x8 = 0x8c;
    } else if g.hatch(this).field_0x8 == 0x8d {
        let s = if profile_at_5(g, app, 1) { g.res.DAT_005e8c9c } else { g.res.DAT_005e8d44 };
        crate::sexy::sexy_app_base::vfunction55(g, app, s);
    }
    if g.wfa(app).offset_0x150 == 1 {
        let t = g.hatch(this).field_0x8;
        if t == 0x1c || t == 0x30 || t == 0x44 {
            let s = g.res.DAT_005e8d14;
            crate::sexy::sexy_app_base::vfunction55(g, app, s);
        }
        if g.hatch(this).field_0x8 == 0x58 {
            let s = g.res.DAT_005e8af0;
            crate::sexy::sexy_app_base::vfunction55(g, app, s);
        }
    }
    let t = g.hatch(this).field_0x8;
    let boss = profile_at_5(g, app, 1);
    let dy = if 0xdc <= t && boss {
        let r = t % 0x10;
        if r <= 3 {
            -1
        } else if 7 < r && r < 0xc {
            1
        } else {
            0
        }
    } else if 0x8c <= t && boss {
        if t < 0xa1 {
            -0xc
        } else if t < 0xab {
            -8
        } else if t < 0xb5 {
            -5
        } else if t < 0xbf {
            -2
        } else {
            -1
        }
    } else {
        0
    };
    g.hatch(this).field_0x18 += dy;
    if t == 0xaa {
        let cont = g.hatch(this).offset_0x20;
        g.btn(cont).field_0x4 = b"Click Here to Continue".to_vec();
        g.w(cont).offset_0x1 = true;
    }
    g.hatch(this).field_0x8 += 1;
}

/// The pet's name (`None`: nothing new is assigned).
fn pet_name(pet: i32) -> Option<&'static [u8]> {
    Some(match pet {
        2 => b"ITCHY the Swordfish",
        0 => b"STINKY the Snail",
        4 => b"ZORF the Sea Horse",
        3 => b"PREGO the Momma Fish",
        6 => b"VERT the Skeleton",
        9 => b"WADSWORTH the Whale",
        8 => b"MERYL the Mermaid",
        7 => b"RUFUS the Fiddler Crab",
        1 => b"NIKO the Oyster",
        5 => b"CLYDE the Jellyfish",
        0x13 => b"PRESTO the Tadpole",
        0xb => b"SHRAPNEL the Robot Fish",
        0xe => b"RHUBARB the Hermit Crab",
        10 => b"SEYMOUR the Turtle",
        0xf => b"NIMBUS the Manta Ray",
        0x10 => b"AMP the Electric Eel",
        0x12 => b"ANGIE the Angelfish",
        0xc => b"GUMBO the Angler",
        0x11 => b"GASH the Shark",
        0xd => b"BLIP the Porpoise",
        0x14 => b"BRINKLEY",
        0x15 => b"NOSTRADAMUS the Nose",
        0x17 => b"WALTER the Penguin",
        0x16 => b"STANLEY the Startlingly",
        999 => b"Evil Alien Mastermind",
        _ => return None,
    })
}

/// The pet's description lines (two-line ones leave the third string as it was).
fn pet_lines(pet: i32) -> &'static [&'static [u8]] {
    match pet {
        2 => &[b"ITCHY helps you by attacking", b"aliens when they appear."],
        0 => &[b"STINKY roams around the", b"bottom of your tank, catching", b"any coins you may have missed."],
        4 => &[b"ZORF gives you a hand in", b"keeping your fish fed."],
        3 => &[b"PREGO helps populate your", b"tank by giving birth to a new", b"baby guppy every so often."],
        6 => &[b"VERT drops gold coins just like", b"a large guppy, but doesn't need", b"fish food to survive."],
        9 => &[b"WADSWORTH helps by sheltering", b"your baby and medium guppies", b"from hungry aliens."],
        8 => &[b"MERYL's song cheers up all the", b"guppies in the tank, making", b"them drop coins faster."],
        7 => &[b"RUFUS does heavy damage to", b"enemies you've lured to the", b"bottom of the tank."],
        1 => &[b"NIKO produces pearls that", b"you can click on for a", b"hefty sum of money."],
        5 => &[b"CLYDE drifts slowly through", b"your tank, collecting any", b"coins it passes by."],
        0x13 => &[b"PRESTO has the ability to", b"metamorph into any of", b"your other pets."],
        0xb => &[b"SHRAPNEL drops bombs that", b"blow up fish on contact but", b"give lots of cash when clicked."],
        0xe => &[b"RHUBARB snaps his claws at", b"fish, keeping them off the", b"bottom of your tank."],
        10 => &[b"SEYMOUR's presence makes all", b"coins and diamonds drift", b"at a slower rate."],
        0xf => &[b"NIMBUS tosses any coins or", b"food he catches back up", b"toward the top of the tank."],
        0x10 => &[b"AMP can electrocute your", b"entire tank, killing your fish", b"and turning them into diamonds."],
        0x12 => &[b"ANGIE has the ability to", b"resurrect dead fish."],
        0xc => &[b"GUMBO attracts guppies using", b"the lantern on his head,", b"luring them away from aliens."],
        0x11 => &[b"GASH viciously attacks aliens,", b"but will snack on one of", b"your guppies from time to time."],
        0xd => &[b"BLIP provides you with info", b"that helps you better combat", b"aliens and keep your fish fed."],
        0x14 => &[b"Likes: peach muffins, all", b"things brown and sticky", b"Dislikes: arugula"],
        0x15 => &[b"Little known fact:  NOSTRADAMUS", b"is the long lost nose of ex-president", b"Rutherford B. Hayes"],
        0x17 => &[b"Choosing this pet donates all", b"proceeds to the Falafel", b"Foundation. Free the falafels!"],
        0x16 => &[b"STANLEY knows no fear.. except", b"that of badgers, aprons,", b"and badgers wearing aprons."],
        999 => &[b"EVIL ALIEN MASTERMIND actually", b"isn't very helpful at all.  Unless", b"you consider devouring the entire"],
        _ => &[],
    }
}

/// The pet's swim sheet (image, source y offset row height, frame width, frame x origin).
fn pet_sheet(g: &mut G, pet: i32) -> Option<(Ptr, i32, i32, i32)> {
    let r = &g.res;
    Some(match pet {
        2 => (r.DAT_005e8aec, 0x5a, 0x50, 0x116),
        0 => (r.DAT_005e8be0, 100, 0x50, 0x116),
        4 => (r.DAT_005e8e30, 0x5a, 0x50, 0x116),
        3 => (r.DAT_005e8ca0, 0x5a, 0x50, 0x116),
        6 => (r.DAT_005e8d38, 0x5a, 0x50, 0x116),
        9 => (r.DAT_005e8b30, 0x5a, 0x50, 0x116),
        8 => (r.DAT_005e8b08, 0x5a, 0x50, 0x116),
        7 => (r.DAT_005e8b60, 0x5a, 0x50, 0x116),
        1 => (r.DAT_005e8e44, 0x46, 0x50, 0x116),
        5 => (r.DAT_005e8bd0, 100, 0x50, 0x116),
        0x13 => (r.DAT_005e8d60, 0x5a, 0x50, 0x116),
        0xb => (r.DAT_005e8c2c, 0x5a, 0x50, 0x116),
        0xe => (r.DAT_005e8a98, 0x5a, 0x50, 0x116),
        10 => (r.DAT_005e8b0c, 0x5a, 0x50, 0x116),
        0xf => (r.DAT_005e8a08, 0x5a, 0x50, 0x116),
        0x10 => (r.DAT_005e8de8, 100, 0xa0, 0xec),
        0x12 => (r.DAT_005e8e8c, 0x5a, 0x50, 0x116),
        0xc => (r.DAT_005e8cb4, 0x5a, 0x50, 0x116),
        0x11 => (r.DAT_005e8bfc, 0x5a, 0x50, 0x116),
        0xd => (r.DAT_005e8a14, 0x5a, 0x50, 0x116),
        0x14 => (r.DAT_005e8b4c, 0x5a, 0x50, 0x116),
        0x15 => (r.DAT_005e8ed4, 0x5a, 0x50, 0x116),
        0x17 => (r.DAT_005e8e50, 0x5a, 0x50, 0x116),
        0x16 => (r.DAT_005e8d00, 0x5a, 0x50, 0x116),
        _ => return None,
    })
}

/// `font->StringWidth(s)` centered on 0x28a: the x the original draws centered lines at.
fn centered_x(g: &mut G, gfx: &mut Graphics, s: &[u8]) -> i32 {
    let f = FUN_00455870(gfx);
    ((0x28a - font::string_width(g, f, s)) >> 1) - 3
}

/// port: 0051f770 Sexy::HatchScreen::vfunction27
/// `Draw(Graphics*)`: the backdrop and button bars, "You have found:", the wobbling egg
/// (cracking from 56; the boss egg after the last level), then after the hatch the pet
/// swimming in its frame, its name and, from 171, its description and "Your game has been
/// saved." (the 5000-shell award for PRESTO).
pub fn vfunction27(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    let mut s1: Vec<u8> = Vec::new();
    let mut s2: Vec<u8> = Vec::new();
    let mut s3: Vec<u8> = Vec::new();
    let img = g.res.DAT_005e8ab4;
    FUN_00455d20(gfx, g, img, 0, 0);
    let bar = g.res.DAT_005e8d3c;
    let h = g.image(bar).offset_0x24;
    FUN_004563f0(gfx, g, &Rect::new(0x14, 0, 600, h), bar);
    let menu = g.hatch(this).offset_0x24;
    if g.w(menu).offset_0x0 {
        let bimg = g.res.DAT_005e8e6c;
        let h = g.image(bimg).offset_0x24;
        let (x, y, w) = (g.wc(menu).offset_0x2c, g.wc(menu).offset_0x30, g.wc(menu).offset_0x34);
        FUN_004563f0(gfx, g, &Rect::new(x - 1, y - 1, w + 2, h), bimg);
    }
    let f = g.res.DAT_005e8e40;
    FUN_00455880(gfx, f);
    FUN_00455890(gfx, CRect(0xff, 200, 0, 0xff));
    FUN_00455cf0(gfx, g, b"You have found:", 0xd7, 0x19);
    let d = g.hatch(this).clone();
    let r = d.field_0x8 % 0x20;
    let wob = if r < 4 {
        0
    } else if r < 8 {
        -1
    } else if r < 0xc {
        -2
    } else if r < 0x10 {
        -1
    } else if r < 0x14 {
        0
    } else if r < 0x18 {
        1
    } else if r < 0x1c {
        2
    } else if r < 0x20 {
        1
    } else {
        0
    };
    let mut g2 = FUN_00455800(gfx);
    let t = d.field_0x8;
    let app = d.field_0x0;
    if t <= 0x50 {
        let cel = if 0x37 < t { (t - 0x38) / 2 } else { 0 };
        let img = g.res.DAT_005e8c74;
        FUN_00456950(gfx, g, img, 0x102, wob + 0x3c, cel);
    } else if profile_at_5(g, app, 1) {
        let cel = if t < 0x8c {
            if 0x77 < t { (t - 0x78) / 2 } else { 0 }
        } else {
            9
        };
        if 0xa0 < t {
            let h = g.hatch(this);
            h.field_0x10 = 0;
            h.field_0x14 = 0;
        }
        let d = g.hatch(this).clone();
        let img = g.res.DAT_005e8ca8;
        FUN_00456950(gfx, g, img, d.field_0x10 + 0x102, d.field_0x14 + 0x3c + wob, cel);
    } else if t <= 0x8c {
        let cel = if 0x77 < t { (t - 0x78) / 2 } else { 0 };
        let img = g.res.DAT_005e8ca8;
        FUN_00456950(gfx, g, img, d.field_0x10 + 0x102, d.field_0x14 + 0x3c + wob, cel);
    } else {
        // The pet, clipped to its frame.
        let pet = d.field_0xc;
        let (cx, cy, cw, ch) = if pet == 0x10 {
            (0xec, 100, 0xa0, 0x3c)
        } else {
            let cy = match pet {
                5 => 100,
                1 => 0x46,
                0 => 100,
                _ => 0x5a,
            };
            (0x116, cy, 0x50, 0x50)
        };
        FUN_00456340(&mut g2, cx, cy, cw, ch);
        let frame = if matches!(pet, 10 | 0xb | 5 | 0xe | 0x14) {
            (t % 0x28) / 4
        } else if pet == 6 || pet == 1 {
            let f = t % 0x14;
            if 9 < f { 0x13 - f } else { f }
        } else {
            (t % 0x14) / 2
        };
        if let Some((img, y, fw, x0)) = pet_sheet(g, pet) {
            FUN_00455d20(&mut g2, g, img, x0 - frame * fw, y);
        }
        FUN_004558e0(&mut g2, false);
        let mut name_y = 0x104;
        if let Some(n) = pet_name(pet) {
            s1 = n.to_vec();
        }
        FUN_00455890(gfx, CRect(0xff, 200, 0, 0xff));
        let f = g.res.DAT_005e8e18;
        FUN_00455880(gfx, f);
        if matches!(pet, 0xe | 9 | 0xb | 0x15) {
            let f = g.res.DAT_005e8a5c;
            FUN_00455880(gfx, f);
        } else if pet == 0x14 || pet == 0x16 {
            let f = g.res.DAT_005e8a5c;
            FUN_00455880(gfx, f);
            if pet == 0x14 {
                s2 = b"the Scuba Diving Elephant".to_vec();
            } else {
                s2 = b"Small Sea Serpent".to_vec();
            }
            let x = centered_x(g, gfx, &s2);
            FUN_00455cf0(gfx, g, &s2, x, 0x109);
            name_y = 0xf5;
        }
        let x = centered_x(g, gfx, &s1);
        FUN_00455cf0(gfx, g, &s1, x, name_y);
    }
    if 0xaa < g.hatch(this).field_0x8 {
        let pet = g.hatch(this).field_0xc;
        FUN_00455890(gfx, CRect(0xff, 0xff, 0xff, 0xff));
        let f = g.res.DAT_005e8cf0;
        FUN_00455880(gfx, f);
        let lines = pet_lines(pet);
        if let Some(l) = lines.first() {
            s1 = l.to_vec();
        }
        if let Some(l) = lines.get(1) {
            s2 = l.to_vec();
        }
        if let Some(l) = lines.get(2) {
            s3 = l.to_vec();
        }
        if pet == 999 {
            let last: &[u8] = b"contents of your fishtank helpful.";
            FUN_00455890(gfx, CRect(0xff, 0xff, 0xff, 0xff));
            let x = centered_x(g, gfx, last);
            FUN_00455cf0(gfx, g, last, x, 0x168);
        }
        let x = centered_x(g, gfx, &s1);
        FUN_00455cf0(gfx, g, &s1, x, 300);
        let x = centered_x(g, gfx, &s2);
        FUN_00455cf0(gfx, g, &s2, x, 0x140);
        let x = centered_x(g, gfx, &s3);
        FUN_00455cf0(gfx, g, &s3, x, 0x154);
        FUN_00455890(gfx, CRect(0xff, 0xff, 100, 0xff));
        if pet == 0x13 {
            let rect = Rect::new(0xb4, 0x175, 0x11c, 0);
            vcall!(g, this, w.vfunction65, gfx, rect, b"You have been awarded 5000 shells\nfor defeating the Final Boss!", -1, 0);
        } else {
            FUN_00455cf0(gfx, g, b"Your game has been saved.", 0xdd, 0x186);
        }
        if pet == 999 {
            s1 = b"Evil Alien Mastermind".to_vec();
            FUN_00455890(gfx, CRect(0xff, 200, 0, 0xff));
            let f = g.res.DAT_005e8e18;
            FUN_00455880(gfx, f);
            let x = centered_x(g, gfx, &s1);
            FUN_00455cf0(gfx, g, &s1, x, 0x104);
        }
    }
}
