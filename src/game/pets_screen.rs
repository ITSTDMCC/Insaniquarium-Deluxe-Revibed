//! `Sexy::PetsScreen`: "Choose Your Pets" before an adventure level (when the player has
//! more pets than may come along), and the virtual tank's "Add/Remove Pets": 24
//! `Sexy::PetButtonWidget` slots (pets not earned yet are disabled), the hovered or last
//! chosen pet's portrait and description, and "Click Here To Continue" / "Return to Tank".
//! Plus the app functions that open and close it.
//!
//! `PetsScreen_data` starts at object offset 0x8c (after the ButtonListener vftable at
//! 0x88); the object is 0x138 bytes, past the database's 0x100 (fields `ext_0x1NN`). The
//! database names the constructor `PetScreenOverlay::PetScreenOverlay` (the overlay's
//! constructor is inlined in it); the overlay is the shared overlay class.

use crate::sexy::button_widget::{BtnSub, ButtonExt};
use crate::sexy::graphics::{FUN_00455870, FUN_00455880, FUN_00455890, FUN_004558e0, FUN_00455cf0, FUN_00455d20, FUN_004563f0, FUN_00456980, FUN_00456a00};
use crate::sexy::image_font as font;
use crate::sexy::prelude::*;
use crate::sexy::types::CRect;

/// `PetsScreen_data` (object offset 0x8c) plus the bytes past the database's class size.
#[derive(Debug, Clone, Default)]
pub struct PetsScreen_data {
    /// +0x8c the app.
    pub field_0x0: Ptr,
    /// +0x90 "Click Here To Continue" / "Return to Tank" (id 99).
    pub offset_0x4: Ptr,
    /// +0x94 "Menu" (id 100; not in the virtual tank).
    pub offset_0x8: Ptr,
    /// +0x98 the overlay widget.
    pub offset_0xc: Ptr,
    /// +0x9c..+0xf8 the 24 pet slots.
    pub offset_0x10: [Ptr; 24],
    /// +0xfc the pre-drawn background (a 640x480 MemoryImage with the slots cut out).
    pub offset_0x70: Ptr,
    /// +0x100..+0x117 which pets were in the virtual tank when the screen opened.
    pub ext_0x100: [bool; 24],
    /// +0x118 first slot column x (0x19).
    pub ext_0x118: i32,
    /// +0x11c slot column spacing (0x5e).
    pub ext_0x11c: i32,
    /// +0x120 update counter (animates the portrait).
    pub ext_0x120: i32,
    /// +0x124 pets chosen.
    pub ext_0x124: i32,
    /// +0x128 the pet described (hovered; -1 none).
    pub ext_0x128: i32,
    /// +0x12c the last chosen pet (-1 none), described again once the hover fades.
    pub ext_0x12c: i32,
    /// +0x130 how many pets may be chosen.
    pub ext_0x130: i32,
    /// +0x134 the description cross-fade countdown (7..1).
    pub ext_0x134: i32,
}

/// `PetButtonWidget_data` (object offset 0x120).
#[derive(Debug, Clone, Default)]
pub struct PetButtonWidget_data {
    /// +0x120 the over cel while chosen (empty).
    pub offset_0x0: Rect,
    /// +0x130 chosen (normal and down cels swapped).
    pub offset_0x10: bool,
    /// +0x134 the app (`gApp`).
    pub offset_0x14: Ptr,
    /// +0x138 the pet's portrait resource id.
    pub offset_0x18: i32,
}

impl G {
    pub fn pets_screen(&mut self, p: Ptr) -> &mut PetsScreen_data {
        match &mut self.widget(p).ext {
            WExt::PetsScreen(d) => d,
            e => panic!("{p} is not a PetsScreen: {e:?}"),
        }
    }
    pub fn pet_button(&mut self, p: Ptr) -> &mut PetButtonWidget_data {
        match &mut self.widget(p).ext {
            WExt::Button(e) => match &mut e.sub {
                BtnSub::PetButton(d) => d,
                s => panic!("{p} is not a PetButtonWidget: {s:?}"),
            },
            e => panic!("{p} is not a ButtonWidget: {e:?}"),
        }
    }
}

/// port: 00527820 PetScreenOverlay::PetScreenOverlay
/// `PetsScreen(WinFishApp*)`: full screen; the continue button ("Return to Tank" with the
/// store music in the virtual tank) and "Menu"; the background (frame, title bar, the Menu
/// frame outside the virtual tank, the portrait frame and plate); the 24 slots in a grid
/// (outside the virtual tank, no pet starts chosen; pets not earned are disabled); in the
/// virtual tank, the pets already in the tank chosen; how many may be chosen (at most 4
/// outside the virtual tank); the overlay.
pub fn PetScreenOverlay(g: &mut G, param_1: Ptr) -> Ptr {
    use crate::game::game_selector::FUN_00506820;
    let (wc, w) = crate::sexy::widget::Widget();
    let d = PetsScreen_data { field_0x0: param_1, ..Default::default() };
    let this = g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__PetsScreen_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::PetsScreen(Box::new(d)) }),
    });
    let (aw, ah) = (g.sab(param_1).field_0xb8, g.sab(param_1).field_0xbc);
    let wc = g.wc(this);
    wc.offset_0x2c = 0;
    wc.offset_0x30 = 0;
    wc.offset_0x34 = aw;
    wc.offset_0x38 = ah;
    let d = g.pets_screen(this);
    d.ext_0x118 = 0x19;
    d.ext_0x11c = 0x5e;
    d.ext_0x124 = 0;
    d.ext_0x128 = -1;
    d.ext_0x12c = -1;
    d.ext_0x120 = 0;
    let virt = g.wfa(param_1).offset_0x150 == 5;
    let img = g.res.DAT_005e8c1c;
    let cont = FUN_00506820(g, 99, this, b"Click Here To Continue", img);
    g.pets_screen(this).offset_0x4 = cont;
    if virt {
        g.btn(cont).field_0x4 = b"Return to Tank".to_vec();
        crate::game::win_fish_app::FUN_0054b020(g, param_1);
        crate::game::win_fish_app::FUN_0054b1a0(g, param_1, 2, 0, false);
    }
    let h = g.wc(cont).offset_0x38;
    vcall!(g, cont, w.vfunction41, 0xe1, 0xfa, 0xba, h);
    let menu = FUN_00506820(g, 100, this, b"Menu", img);
    g.pets_screen(this).offset_0x8 = menu;
    let h = g.wc(menu).offset_0x38;
    vcall!(g, menu, w.vfunction41, 0x20d, 4, 0x50, h);

    let bg = crate::sexy::blit::new_memory_image(g, 0x280, 0x1e0);
    g.pets_screen(this).offset_0x70 = bg;
    crate::sexy::blit::render_offscreen(g, bg, |g, gfx| {
        let frame = g.res.DAT_005e8c70;
        FUN_004563f0(gfx, g, &Rect::new(-5, -5, 0x28a, 0x1ea), frame);
        let bar = g.res.DAT_005e8d3c;
        let h = g.image(bar).offset_0x24;
        FUN_004563f0(gfx, g, &Rect::new(0x14, 0, 600, h), bar);
        if !virt {
            let mf = g.res.DAT_005e8e6c;
            let h = g.image(mf).offset_0x24;
            let m = g.wc(menu).clone();
            FUN_004563f0(gfx, g, &Rect::new(m.offset_0x2c - 1, m.offset_0x30 - 1, m.offset_0x34 + 2, h), mf);
        }
        let pf = g.res.DAT_005e8c58;
        FUN_00455d20(gfx, g, pf, 0xd8, 0x30);
        let plate = g.res.DAT_005e8e7c;
        FUN_00455d20(gfx, g, plate, 0xdd, 0xf3);
    });
    let profile = g.wfa(param_1).offset_0x18c;
    for i in 0..24i32 {
        if !virt {
            g.profile(profile).field_0x5a[i as usize] = false;
        }
        let b = PetButtonWidget(g, 0xe8 + i, i, this);
        g.pets_screen(this).offset_0x10[i as usize] = b;
        let (x0, dx) = (g.pets_screen(this).ext_0x118, g.pets_screen(this).ext_0x11c);
        let y = -0x176 + i * 0x53;
        let (x, y) = if i < 5 {
            (x0, y + 0x19f)
        } else if i < 10 {
            (x0 + dx, y)
        } else if i < 0xf {
            (x0 + 0x18 + dx * 4, y - 0x19f)
        } else if i < 0x14 {
            (dx * 5 + 0x18 + x0, y - 0x33e)
        } else if i < 0x16 {
            (x0 + 0xd + dx * 2, y - 0x3e4)
        } else {
            (dx * 3 + 0x10 + x0, y - 0x48a)
        };
        vcall!(g, b, w.vfunction41, x, y, 0x5a, 0x53);
        let mask = g.res.DAT_005e8e4c;
        let (bx, by) = (g.wc(b).offset_0x2c, g.wc(b).offset_0x30);
        crate::game::sim_fish::FUN_005032a0(g, bg, mask, bx, by);
        if !crate::game::profile::FUN_00501410(g, profile, i as usize) {
            vcall!(g, b, w.vfunction38, true);
        }
        g.pets_screen(this).ext_0x100[i as usize] = false;
    }
    let board = g.wfa(param_1).offset_0x4;
    if virt && board != NULL {
        let objs = g.board(board).offset_0x7c.clone();
        let mut it = objs.first().copied();
        while let Some(o) = it {
            let kind = g.go(o).offset_0x24;
            let idx = kind.wrapping_sub(1000) as u32;
            if idx < 0x18 {
                let b = g.pets_screen(this).offset_0x10[idx as usize];
                if !g.w(b).offset_0x2 {
                    let d = g.pets_screen(this);
                    d.ext_0x124 += 1;
                    d.ext_0x12c = idx as i32;
                    d.ext_0x128 = idx as i32;
                    FUN_00532dd0(g, b, true);
                }
                g.pets_screen(this).ext_0x100[idx as usize] = true;
            }
            it = crate::game::store::FUN_004e3e30(&objs, o);
        }
    }
    let p = g.profile(profile).clone();
    let mut max = if p.field_0xb4 < p.field_0x18 { p.field_0xb4 } else { p.field_0x18 };
    if !virt && 4 < max {
        max = 4;
    }
    g.pets_screen(this).ext_0x130 = max;

    // The overlay: a plain Widget with PetScreenOverlay's vftable and the screen at +0x88.
    let (mut owc, mut ow) = crate::sexy::widget::Widget();
    ow.offset_0x1 = false;
    owc.offset_0x3c = true;
    owc.offset_0x34 = 0x280;
    owc.offset_0x38 = 0x1e0;
    let overlay = g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::PetScreenOverlay_vftable),
        node: Node::Widget(WidgetObj { wc: owc, w: ow, ext: WExt::GameSelectorOverlay(this) }),
    });
    g.pets_screen(this).offset_0xc = overlay;
    g.pets_screen(this).ext_0x134 = 0;
    this
}

/// port: 00518930 Sexy::PetsScreen::~PetsScreen
pub fn dtor_PetsScreen(g: &mut G, this: Ptr) {
    let d = g.pets_screen(this).clone();
    for b in d.offset_0x10 {
        if b != NULL {
            vcall!(g, b, w.vfunction1, 1);
        }
    }
    for p in [d.offset_0x4, d.offset_0x8] {
        if p != NULL {
            vcall!(g, p, w.vfunction1, 1);
        }
    }
    if d.offset_0x70 != NULL {
        g.free(d.offset_0x70);
    }
    if d.offset_0xc != NULL {
        vcall!(g, d.offset_0xc, w.vfunction1, 1);
    }
    crate::sexy::widget::dtor_Widget(g, this);
}

/// port: 0051b1a0 Sexy::PetsScreen::deleting_destructor
pub fn deleting_destructor(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_PetsScreen(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 00518a00 Sexy::PetsScreen::vfunction21
/// `AddedToManager(WidgetManager*)`: the slots, the overlay, the continue button, and
/// "Menu" outside the virtual tank.
pub fn vfunction21(g: &mut G, this: Ptr, param_1: Ptr) {
    crate::sexy::widget_container::vfunction21(g, this, param_1);
    let d = g.pets_screen(this).clone();
    for b in d.offset_0x10 {
        vcall!(g, param_1, w.vfunction4, b);
    }
    vcall!(g, param_1, w.vfunction4, d.offset_0xc);
    vcall!(g, param_1, w.vfunction4, d.offset_0x4);
    if g.wfa(d.field_0x0).offset_0x150 != 5 {
        vcall!(g, param_1, w.vfunction4, d.offset_0x8);
    }
}

/// port: 00518a80 Sexy::PetsScreen::vfunction22
/// `RemovedFromManager(WidgetManager*)`.
pub fn vfunction22(g: &mut G, this: Ptr, param_1: Ptr) {
    crate::sexy::widget_container::vfunction22(g, this, param_1);
    let d = g.pets_screen(this).clone();
    for b in d.offset_0x10 {
        vcall!(g, param_1, w.vfunction5, b);
    }
    vcall!(g, param_1, w.vfunction5, d.offset_0xc);
    vcall!(g, param_1, w.vfunction5, d.offset_0x4);
    if g.wfa(d.field_0x0).offset_0x150 != 5 {
        vcall!(g, param_1, w.vfunction5, d.offset_0x8);
    }
}

/// port: 00518b00 Sexy::PetsScreen::vfunction23
/// `Update()`: the cross-fade back to the last chosen pet ends; the counter; redraw.
pub fn vfunction23(g: &mut G, this: Ptr) {
    let d = g.pets_screen(this);
    if d.ext_0x134 != 0 {
        d.ext_0x134 -= 1;
        if d.ext_0x134 == 0 {
            d.ext_0x128 = d.ext_0x12c;
        }
    }
    d.ext_0x120 += 1;
    vcall!(g, this, w.vfunction18);
}

/// port: 00518b30 Sexy::PetsScreen::vfunction5
/// `ButtonMouseEnter(int id)`: a slot is described at once.
pub fn vfunction5(g: &mut G, this: Ptr, param_1: i32) {
    if (param_1 as u32) < 0x18 {
        let d = g.pets_screen(this);
        d.ext_0x128 = param_1;
        d.ext_0x134 = 0;
    }
}

/// port: 00518b50 Sexy::PetsScreen::vfunction6
/// `ButtonMouseLeave(int id)`: leaving an unchosen slot fades back to the last chosen pet.
pub fn vfunction6(g: &mut G, this: Ptr, param_1: i32) {
    if (param_1 as u32) < 0x18 {
        let b = g.pets_screen(this).offset_0x10[param_1 as usize];
        if !g.pet_button(b).offset_0x10 {
            g.pets_screen(this).ext_0x134 = 7;
        }
    }
}

/// port: 00500e80 FUN_00500e80
/// Pet `param_1`'s name ("" past the last).
pub fn FUN_00500e80(param_1: i32) -> &'static [u8] {
    match param_1 {
        0 => b"Stinky",
        1 => b"Niko",
        2 => b"Itchy",
        3 => b"Prego",
        4 => b"Zorf",
        5 => b"Clyde",
        6 => b"Vert",
        7 => b"Rufus",
        8 => b"Meryl",
        9 => b"Wadsworth",
        10 => b"Seymour",
        0xb => b"Shrapnel",
        0xc => b"Gumbo",
        0xd => b"Blip",
        0xe => b"Rhubarb",
        0xf => b"Nimbus",
        0x10 => b"Amp",
        0x11 => b"Gash",
        0x12 => b"Angie",
        0x13 => b"Presto",
        0x14 => b"Brinkley",
        0x15 => b"Nostradamus",
        0x16 => b"Stanley",
        0x17 => b"Walter",
        _ => b"",
    }
}

/// port: 00527ec0 FUN_00527ec0
/// The description of pet `param_2` at opacity `param_3`: its animated portrait (Niko and
/// Vert ping-pong, the drifters and Rhubarb in quarter steps, the rest two updates a cel),
/// its name, and three lines.
pub fn FUN_00527ec0(g: &mut G, this: Ptr, param_1: &mut Graphics, param_2: i32, param_3: i32) {
    let name = FUN_00500e80(param_2).to_vec();
    let (mut x, mut y) = (0x118, 0x5a);
    let lines: [&[u8]; 3] = match param_2 {
        0 => [b"STINKY roams around the", b"bottom of your tank, catching", b"any coins you may have missed."],
        1 => {
            y = 0x55;
            [b"NIKO produces pearls that", b"you can click on for a", b"hefty sum of money."]
        }
        2 => [b"ITCHY helps you by attacking", b"aliens when they appear.", b""],
        3 => [b"PREGO helps populate your", b"tank by giving birth to a new", b"baby guppy every so often."],
        4 => [b"ZORF gives you a hand in", b"keeping your fish fed.", b""],
        5 => [b"CLYDE drifts slowly through", b"your tank, collecting any", b"coins it passes by."],
        6 => [b"VERT drops gold coins just like", b"a large guppy, but doesn't need", b"fish food to survive."],
        7 => [b"RUFUS does heavy damage to", b"enemies you've lured to the", b"bottom of the tank."],
        8 => [b"MERYL's song cheers up all the", b"guppies in the tank, making", b"them drop coins faster."],
        9 => [b"WADSWORTH helps by sheltering", b"your baby and medium guppies", b"from hungry aliens."],
        10 => [b"SEYMOUR's presence makes all", b"coins and diamonds drift", b"at a slower rate."],
        0xb => [b"SHRAPNEL drops bombs that", b"blow up fish on contact but", b"give lots of cash when clicked."],
        0xc => [b"GUMBO attracts guppies using", b"the lantern on his head,", b"luring them away from aliens."],
        0xd => [b"BLIP provides you with info", b"that helps you better combat", b"aliens and keep your fish fed."],
        0xe => [b"RHUBARB snaps his claws at", b"fish, keeping them off the", b"bottom of your tank."],
        0xf => [b"NIMBUS tosses any coins or", b"food he catches back up", b"toward the top of the tank."],
        0x10 => {
            x = 0xf0;
            [b"AMP can electrocute your", b"entire tank, killing your fish", b"and turning them into diamonds."]
        }
        0x11 => [b"GASH viciously attacks aliens,", b"but will snack on one of", b"your guppies from time to time."],
        0x12 => [b"ANGIE has the ability to", b"resurrect dead fish.", b""],
        0x13 => [b"Change PRESTO into", b"any of your other pets", b"by right-clicking on him."],
        0x14 => [b"Likes: peach muffins, all", b"things brown and sticky", b"Dislikes: arugula"],
        0x15 => [b"Little known fact:  NOSTRADAMUS", b"is the nose of ex-president", b"Rutherford B. Hayes"],
        0x16 => [b"STANLEY knows no fear.. except", b"that of badgers, aprons,", b"and badgers wearing aprons."],
        0x17 => [b"Choosing this pet donates all", b"proceeds to the Falafel", b"Foundation. Free the falafels!"],
        _ => [b"", b"", b""],
    };
    if 0 <= param_2 {
        if param_3 != 0xff {
            FUN_004558e0(param_1, true);
            FUN_00455890(param_1, CRect(0xff, 0xff, 0xff, param_3));
        }
        let t = g.pets_screen(this).ext_0x120;
        let cel = if param_2 == 1 || param_2 == 6 {
            let n = t % 0x12;
            if 9 < n { 0x12 - n } else { n }
        } else if param_2 == 10 || param_2 == 5 || param_2 == 0xb || param_2 == 0xe {
            (t % 0x28) / 4
        } else {
            (t % 0x14) / 2
        };
        let img = crate::sexy::res::FUN_005016a0(g, param_2 + 0xca) as Ptr;
        FUN_00456980(param_1, g, img, x, y, cel, 0);
        FUN_004558e0(param_1, false);
        let f = g.res.DAT_005e8e18;
        FUN_00455880(param_1, f);
        FUN_00455890(param_1, CRect(0xff, 200, 0, param_3));
        let fnt = FUN_00455870(param_1);
        let w = font::string_width(g, fnt, &name);
        FUN_00455cf0(param_1, g, &name, ((0x28a - w) >> 1) - 3, 0xb4);
    }
    let f = g.res.DAT_005e8c20;
    FUN_00455880(param_1, f);
    FUN_00455890(param_1, CRect(0xff, 0xff, 0xff, param_3));
    for (l, yy) in lines.iter().zip([200, 0xd7, 0xe6]) {
        let fnt = FUN_00455870(param_1);
        let w = font::string_width(g, fnt, l);
        FUN_00455cf0(param_1, g, l, ((0x28a - w) >> 1) - 3, yy);
    }
}

/// port: 00528690 Sexy::PetsScreen::vfunction45
/// `DrawOverlay(Graphics*)`: the background, the slot frames, the title, the description
/// (cross-fading back to the last chosen pet), and how many more pets may be chosen.
pub fn vfunction45(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    let d = g.pets_screen(this).clone();
    FUN_00455d20(gfx, g, d.offset_0x70, 0, 0);
    for b in d.offset_0x10 {
        let (x, y) = (g.wc(b).offset_0x2c, g.wc(b).offset_0x30);
        let back = g.res.DAT_005e8a88;
        FUN_00455d20(gfx, g, back, x, y);
        if !g.w(b).offset_0x2 {
            let rim = g.res.DAT_005e8a38;
            FUN_00455d20(gfx, g, rim, x, y);
        }
    }
    let f = g.res.DAT_005e8e40;
    FUN_00455880(gfx, f);
    FUN_00455890(gfx, CRect(0xff, 200, 0, 0xff));
    let virt = g.wfa(d.field_0x0).offset_0x150 == 5;
    let title: &[u8] = if virt { b"Add/Remove Pets" } else { b"Choose Your Pets" };
    FUN_00455cf0(gfx, g, title, 0xd7, 0x19);
    let n = d.ext_0x134;
    if n < 1 || 4 < n {
        FUN_00527ec0(g, this, gfx, d.ext_0x128, 0xff);
    } else {
        let a = (n * 0xff) / 5;
        FUN_00527ec0(g, this, gfx, d.ext_0x128, a);
        if d.ext_0x12c != -1 {
            FUN_00527ec0(g, this, gfx, d.ext_0x12c, 0xff - a);
        }
    }
    let line2: &[u8] = if virt { b"to display in your tank" } else { b"to take to the next level" };
    let left = d.ext_0x130 - d.ext_0x124;
    let line1: Vec<u8> = if left < 1 {
        Vec::new()
    } else if 1 < left {
        format!("Choose {left} more pets").into_bytes()
    } else {
        b"Choose 1 more pet".to_vec()
    };
    let f = g.res.DAT_005e8c20;
    FUN_00455880(gfx, f);
    FUN_00455890(gfx, CRect(0xff, 0xff, 0xff, 0xff));
    let fnt = FUN_00455870(gfx);
    let w = font::string_width(g, fnt, &line1);
    FUN_00455cf0(gfx, g, &line1, ((0x28a - w) >> 1) - 3, 0x4b);
    let fnt = FUN_00455870(gfx);
    let w = font::string_width(g, fnt, line2);
    FUN_00455cf0(gfx, g, line2, ((0x28a - w) >> 1) - 3, 0x5a);
}

/// port: 005289f0 Sexy::PetsScreen::vfunction3
/// `ButtonDepress(int id)`: a chosen slot becomes the last chosen; an unchosen one that was
/// the last chosen passes that to the last chosen slot in order. Continue (99): in the
/// virtual tank, pets added or taken away; otherwise the choice saved to the profile; with
/// three or more chosen (always in the virtual tank) the screen closes and the level starts
/// (or the tank resumes, saved); with fewer, the player is asked to confirm. "Menu" (100)
/// back to the main menu.
pub fn vfunction3(g: &mut G, this: Ptr, param_1: i32) {
    let app = g.pets_screen(this).field_0x0;
    if (param_1 as u32) < 0x18 {
        let b = g.pets_screen(this).offset_0x10[param_1 as usize];
        if g.pet_button(b).offset_0x10 {
            g.pets_screen(this).ext_0x12c = param_1;
            return;
        }
        if g.pets_screen(this).ext_0x12c == param_1 {
            g.pets_screen(this).ext_0x12c = -1;
            let slots = g.pets_screen(this).offset_0x10;
            for (i, s) in slots.iter().enumerate() {
                if g.pet_button(*s).offset_0x10 {
                    g.pets_screen(this).ext_0x12c = i as i32;
                }
            }
        }
        return;
    }
    if param_1 == 99 {
        let mut changed = false;
        let d = g.pets_screen(this).clone();
        if g.wfa(app).offset_0x150 == 5 {
            for i in 0..0x18 {
                let chosen = g.pet_button(d.offset_0x10[i]).offset_0x10;
                if d.ext_0x100[i] != chosen {
                    changed = true;
                    let board = g.wfa(app).offset_0x4;
                    if !chosen {
                        FUN_0053a770(g, board, i as i32 + 1000);
                    } else {
                        crate::game::board_level::FUN_00544a90(g, board, i as i32, -1, -1, false, false);
                    }
                }
            }
        } else {
            let profile = g.wfa(app).offset_0x18c;
            for i in 0..0x18 {
                let chosen = g.pet_button(d.offset_0x10[i]).offset_0x10;
                g.profile(profile).field_0x5a[i] = chosen;
            }
        }
        if 2 < d.ext_0x124 || g.wfa(app).offset_0x150 == 5 {
            FUN_0054af90(g, app);
            if changed {
                crate::game::store::FUN_0054aff0(g, app);
            }
            let board = g.wfa(app).offset_0x4;
            if board == NULL {
                crate::game::win_fish_app::FUN_0054cbf0(g, app);
                return;
            }
            crate::game::board_save::FUN_00538940(g, board);
            crate::game::board_update::FUN_0053db80(g, board, false);
            return;
        }
        let lines: &[u8] = if d.ext_0x130 < 4 {
            b"Are you sure you want to continue to the next level without selecting 3 pets?"
        } else {
            b"Are you sure you want to continue to the next level without selecting at least 3 pets?"
        };
        crate::game::win_fish_app::vfunction73(g, app, 0x14, true, b"Select More Pets?", lines, b"", 1);
    } else if param_1 == 100 {
        FUN_0054af90(g, app);
        crate::game::win_fish_app::FUN_00552100(g, app);
    }
}

/// port: 0053a770 FUN_0053a770
/// Takes the virtual-tank creature of kind `param_1` (+0xac) off the board and deletes it.
pub fn FUN_0053a770(g: &mut G, this: Ptr, param_1: i32) {
    let objs = g.board(this).offset_0x7c.clone();
    let mut it = objs.first().copied();
    while let Some(o) = it {
        if g.go(o).offset_0x24 == param_1 {
            crate::game::alien::FUN_004d6830(g, o, false);
            let app = g.board(this).field_0x0;
            crate::sexy::sexy_app_base::vfunction35(g, app, o);
            return;
        }
        it = crate::game::store::FUN_004e3e30(&objs, o);
    }
}

/// port: 005328a0 Sexy::PetButtonWidget::PetButtonWidget
/// `PetButtonWidget(int portrait, int theId, ButtonListener*)`: cels from quarters of the
/// slot image (normal the second, down the third, over the fourth, disabled the first).
pub fn PetButtonWidget(g: &mut G, param_1: i32, param_2: i32, param_3: Ptr) -> Ptr {
    let (wc, mut w, mut b) = crate::sexy::button_widget::ButtonWidget(param_2, param_3);
    let app = g.globals.DAT_005eb6a4;
    w.offset_0x28 = true;
    let img = g.res.DAT_005e8e78;
    b.offset_0x28 = img;
    let (iw, ih) = (g.image(img).offset_0x20, g.image(img).offset_0x24);
    let q = iw / 4;
    b.offset_0x38 = Rect::new(q, 0, q, ih);
    b.offset_0x48 = Rect::new(q * 3, 0, q, ih);
    b.offset_0x58 = Rect::new(q * 2, 0, q, ih);
    b.offset_0x68 = Rect::new(0, 0, q, ih);
    let d = PetButtonWidget_data { offset_0x0: Rect::new(0, 0, 0, 0), offset_0x10: false, offset_0x14: app, offset_0x18: param_1 };
    g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__PetButtonWidget_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::Button(Box::new(ButtonExt { b, sub: BtnSub::PetButton(d) })) }),
    })
}

/// port: 005312e0 Sexy::PetButtonWidget::~PetButtonWidget
pub fn dtor_PetButtonWidget(g: &mut G, this: Ptr) {
    crate::sexy::button_widget::dtor_ButtonWidget(g, this);
}

/// port: 00532990 Sexy::PetButtonWidget::deleting_destructor
pub fn deleting_destructor__00532990(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_PetButtonWidget(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 00531300 Sexy::PetButtonWidget::vfunction27
/// `Draw(Graphics*)`: the button, then (when enabled) the pet's animated portrait (the
/// `DAT_005e8dfc` portrait placed lower).
pub fn vfunction27__00531300(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    crate::sexy::button_widget::vfunction27(g, this, gfx);
    if g.w(this).offset_0x2 {
        return;
    }
    let id = g.pet_button(this).offset_0x18;
    let img = crate::sexy::res::FUN_005016a0(g, id) as Ptr;
    let t = g.wc(this).offset_0x24;
    // `FUN_004578c0`: the cel rect at that time (computed and not used).
    let _ = crate::sexy::image::FUN_004578c0(g, img, t);
    let (x, y) = if img == g.res.DAT_005e8dfc { (0xf, 0x18) } else { (0xe, 10) };
    FUN_00456a00(gfx, g, img, x, y, t);
}

/// port: 00532f70 Sexy::PetButtonWidget::vfunction57
/// `MouseUp(int x, int y, int clicks)` over an enabled slot: a chosen pet is unchosen; an
/// unchosen one is chosen while fewer than the allowed number are. (Then the empty base.)
pub fn vfunction57__00532f70(g: &mut G, this: Ptr, _x: i32, _y: i32, _clicks: i32) {
    let w = g.w(this).clone();
    if w.offset_0x5 && !w.offset_0x2 {
        let app = g.pet_button(this).offset_0x14;
        let screen = g.wfa(app).offset_0xe0;
        if g.pet_button(this).offset_0x10 && screen != NULL {
            FUN_00532dd0(g, this, false);
            g.pets_screen(screen).ext_0x124 -= 1;
            crate::sexy::trivial::vfunction1__0046eb50();
            return;
        }
        if screen != NULL && g.pets_screen(screen).ext_0x124 < g.pets_screen(screen).ext_0x130 {
            FUN_00532dd0(g, this, true);
            g.pets_screen(screen).ext_0x124 += 1;
        }
    }
    crate::sexy::trivial::vfunction1__0046eb50();
}

/// port: 00532dd0 FUN_00532dd0
/// `SetChosen(bool)`: swaps the normal and down cels and the over cel with the empty one,
/// and redraws.
pub fn FUN_00532dd0(g: &mut G, this: Ptr, param_1: bool) {
    if param_1 == g.pet_button(this).offset_0x10 {
        return;
    }
    g.pet_button(this).offset_0x10 = param_1;
    let alt = g.pet_button(this).offset_0x0;
    let b = g.btn(this);
    std::mem::swap(&mut b.offset_0x38, &mut b.offset_0x58);
    let over = b.offset_0x48;
    b.offset_0x48 = alt;
    g.pet_button(this).offset_0x0 = over;
    vcall!(g, this, w.vfunction18);
}

/// port: 0054c2d0 FUN_0054c2d0
/// `ShowPetsScreen()`: closes the dialogs and any pets screen, pauses the board, and opens
/// it full-screen (+0x80c).
pub fn FUN_0054c2d0(g: &mut G, this: Ptr) {
    crate::game::win_fish_app::FUN_0054b4c0(g, this);
    FUN_0054af90(g, this);
    let board = g.wfa(this).offset_0x4;
    if board != NULL {
        crate::game::board_update::FUN_0053db80(g, board, true);
    }
    let s = PetScreenOverlay(g, this);
    g.wfa(this).offset_0xe0 = s;
    let (w, h) = (g.sab(this).field_0xb8, g.sab(this).field_0xbc);
    vcall!(g, s, w.vfunction41, 0, 0, w, h);
    let wm = g.sab(this).offset_0x318;
    vcall!(g, wm, w.vfunction4, s);
}

/// port: 0054af90 FUN_0054af90
/// `RemovePetsScreen()` (+0x80c).
pub fn FUN_0054af90(g: &mut G, this: Ptr) {
    let s = g.wfa(this).offset_0xe0;
    if s != NULL {
        let wm = g.sab(this).offset_0x318;
        vcall!(g, wm, w.vfunction5, s);
        crate::sexy::sexy_app_base::vfunction35(g, this, s);
        g.wfa(this).offset_0xe0 = NULL;
    }
}
