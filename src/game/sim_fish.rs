//! `Sexy::SimFishScreen`: the virtual tank's "Fish Setup" screen (the FISH button): twenty
//! `Sexy::FishButtonWidget` slots holding the tank's creatures in purchase order, the
//! selected creature's portrait and card (name, purchase date, hometown, mood, notes, and
//! the price / resale value while the Sell button is hovered), Show / Hide, Sell, Rename,
//! Hide All / Show All and "Return to Tank". Plus the app functions that open and close it.
//!
//! `SimFishScreen_data` starts at object offset 0x8c (after the ButtonListener vftable at
//! 0x88); the object is 0x11c bytes, past the database's 0x10c (fields `ext_0x1NN`). The
//! database names the constructor `SimFishScreenOverlay::SimFishScreenOverlay` (the
//! overlay's constructor is inlined in it); the overlay is the shared overlay class.

use crate::sexy::button_widget::{BtnSub, ButtonExt};
use crate::sexy::graphics::{FUN_00455880, FUN_00455870, FUN_00455890, FUN_00455cf0, FUN_00455d20, FUN_004563d0, FUN_004563f0};
use crate::sexy::image_font as font;
use crate::sexy::prelude::*;
use crate::sexy::types::{CRect, FUN_00433320, FUN_00433360};

/// `DAT_005e1604`: a hometown index forced on every creature (-1, never written: off).
pub const DAT_005e1604: i32 = -1;
/// `DAT_005e1608`: a hobby index forced on every creature (-1, never written: off).
pub const DAT_005e1608: i32 = -1;

/// `SimFishScreen_data` (object offset 0x8c) plus the bytes past the database's class size.
#[derive(Debug, Clone, Default)]
pub struct SimFishScreen_data {
    /// +0x8c the app.
    pub field_0x0: Ptr,
    /// +0x90 the bubbles in the portrait frame (`BubbleMgr`, owned).
    pub offset_0x4: Ptr,
    /// +0x94 "Return to Tank" (id 100).
    pub offset_0x8: Ptr,
    /// +0x98 "Menu" (id 0x65; nothing handles it).
    pub offset_0xc: Ptr,
    /// +0x9c Show / Hide the selected creature (id 0x66).
    pub offset_0x10: Ptr,
    /// +0xa0 "Sell" (id 0x67).
    pub offset_0x14: Ptr,
    /// +0xa4 "Rename" (id 0x68).
    pub offset_0x18: Ptr,
    /// +0xa8 "Hide All" (id 0x69; becomes "Show All", id 0x6a, while none is shown).
    pub offset_0x1c: Ptr,
    /// +0xac "Show All" (id 0x6a; hidden, never shown).
    pub offset_0x20: Ptr,
    /// +0xb0 the overlay widget.
    pub offset_0x24: Ptr,
    /// +0xb4..+0x100 the twenty creature slots.
    pub offset_0x28: [Ptr; 20],
    /// +0x104 the selected slot.
    pub field_0x78: Ptr,
    /// +0x108 the pre-drawn background (a 640x480 MemoryImage with the slots cut out).
    pub offset_0x7c: Ptr,
    /// +0x10c first slot column x (0x19).
    pub ext_0x10c: i32,
    /// +0x110 slot column spacing (0x5e).
    pub ext_0x110: i32,
    /// +0x114 the card cross-fade (0 none, 1..7 fading, counting down while the price shows).
    pub ext_0x114: i32,
    /// +0x118 the price card shows (Sell hovered or its question up).
    pub ext_0x118: bool,
    /// +0x119 the sell question is up.
    pub ext_0x119: bool,
    /// +0x11a something changed (leaving re-enters the virtual tank).
    pub ext_0x11a: bool,
}

/// `FishButtonWidget_data` (object offset 0x120).
#[derive(Debug, Clone, Default)]
pub struct FishButtonWidget_data {
    /// +0x120 the over cel while selected (empty).
    pub offset_0x0: Rect,
    /// +0x130 the normal cel while selected (the third quarter).
    pub offset_0x10: Rect,
    /// +0x140 the down cel while selected (empty).
    pub offset_0x20: Rect,
    /// +0x150 selected.
    pub offset_0x30: bool,
    /// +0x154 the app (`gApp`).
    pub offset_0x34: Ptr,
    /// +0x158 the creature (not owned).
    pub offset_0x38: Ptr,
}

impl G {
    pub fn sim_fish(&mut self, p: Ptr) -> &mut SimFishScreen_data {
        match &mut self.widget(p).ext {
            WExt::SimFish(d) => d,
            e => panic!("{p} is not a SimFishScreen: {e:?}"),
        }
    }
    pub fn fish_button(&mut self, p: Ptr) -> &mut FishButtonWidget_data {
        match &mut self.widget(p).ext {
            WExt::Button(e) => match &mut e.sub {
                BtnSub::FishButton(d) => d,
                s => panic!("{p} is not a FishButtonWidget: {s:?}"),
            },
            e => panic!("{p} is not a ButtonWidget: {e:?}"),
        }
    }
}

/// port: 0052f2c0 SimFishScreenOverlay::SimFishScreenOverlay
/// `SimFishScreen(WinFishApp*)`: full screen; bubbles; the store music; the buttons; the
/// background drawn once into a MemoryImage (frame, title bar, the Hide All frame, the
/// portrait frame and its plate); the creatures placed in their slots by purchase time
/// (empty slots disabled); the overlay; the button labels.
pub fn SimFishScreenOverlay(g: &mut G, param_1: Ptr) -> Ptr {
    use crate::game::game_selector::{FUN_00506760, FUN_00506820};
    use crate::sexy::widget::FUN_0046e880;
    let (wc, w) = crate::sexy::widget::Widget();
    let d = SimFishScreen_data { field_0x0: param_1, ..Default::default() };
    let this = g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__SimFishScreen_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::SimFish(Box::new(d)) }),
    });
    let bubbles = crate::game::board_parts::BubbleMgr(g);
    g.sim_fish(this).offset_0x4 = bubbles;
    crate::game::win_fish_app::FUN_0054b020(g, param_1);
    crate::game::win_fish_app::FUN_0054b1a0(g, param_1, 2, 0, false);
    let (aw, ah) = (g.sab(param_1).field_0xb8, g.sab(param_1).field_0xbc);
    let wc = g.wc(this);
    wc.offset_0x2c = 0;
    wc.offset_0x30 = 0;
    g.sim_fish(this).ext_0x10c = 0x19;
    g.sim_fish(this).ext_0x110 = 0x5e;
    let wc = g.wc(this);
    wc.offset_0x34 = aw;
    wc.offset_0x38 = ah;

    let img = g.res.DAT_005e8c1c;
    let b = FUN_00506820(g, 0x69, this, b"Hide All", img);
    g.sim_fish(this).offset_0x1c = b;
    let h = g.wc(b).offset_0x38;
    vcall!(g, b, w.vfunction41, 500, 4, 100, h);
    let b = FUN_00506820(g, 0x6a, this, b"Show All", img);
    g.sim_fish(this).offset_0x20 = b;
    let h = g.wc(b).offset_0x38;
    vcall!(g, b, w.vfunction41, 500, 4, 100, h);
    g.w(b).offset_0x0 = false;
    let f = g.res.DAT_005e8cf0;
    let b = FUN_00506760(g, 100, this, b"Return to Tank", f);
    g.sim_fish(this).offset_0x8 = b;
    let h = g.wc(b).offset_0x38;
    vcall!(g, b, w.vfunction41, 0xda, 0x1ac, 0xce, h);
    let b = FUN_00506820(g, 0x65, this, b"Menu", img);
    g.sim_fish(this).offset_0xc = b;
    let h = g.wc(b).offset_0x38;
    vcall!(g, b, w.vfunction41, 0x20d, 4, 0x50, h);
    let img = g.res.DAT_005e8ad8;
    let toggle = FUN_00506820(g, 0x66, this, b"", img);
    g.sim_fish(this).offset_0x10 = toggle;
    let h = g.wc(toggle).offset_0x38;
    vcall!(g, toggle, w.vfunction41, 0xe1, 0x186, 0x3a, h);
    g.dialog_button(toggle).offset_0xc = 0;
    let img = g.res.DAT_005e8dec;
    let sell = FUN_00506820(g, 0x67, this, b"Sell", img);
    g.sim_fish(this).offset_0x14 = sell;
    FUN_0046e880(g, sell, 0x4403, toggle, 0x48, 0, 0, 0);
    g.dialog_button(sell).offset_0xc = 0;
    let img = g.res.DAT_005e8d28;
    let rename = FUN_00506820(g, 0x68, this, b"Rename", img);
    g.sim_fish(this).offset_0x18 = rename;
    FUN_0046e880(g, rename, 0x4402, toggle, -2, 0, 0, 0);
    FUN_0046e880(g, rename, 0x20000, sell, 0, 0, 3, 0);
    for b in [toggle, sell, rename] {
        vcall!(g, b, w.vfunction35, 0, FUN_00433320(0xffffff));
    }
    let ret = g.sim_fish(this).offset_0x8;
    vcall!(g, ret, w.vfunction35, 0, FUN_00433360(0xff, 0xf0, 0));

    let bg = crate::sexy::blit::new_memory_image(g, 0x280, 0x1e0);
    g.sim_fish(this).offset_0x7c = bg;
    let hide_all = g.sim_fish(this).offset_0x1c;
    crate::sexy::blit::render_offscreen(g, bg, |g, gfx| {
        let frame = g.res.DAT_005e8c70;
        FUN_004563f0(gfx, g, &Rect::new(-5, -5, 0x28a, 0x1ea), frame);
        let bar = g.res.DAT_005e8d3c;
        let h = g.image(bar).offset_0x24;
        FUN_004563f0(gfx, g, &Rect::new(0x14, 0, 600, h), bar);
        let bf = g.res.DAT_005e8e6c;
        let h = g.image(bf).offset_0x24;
        let r = g.wc(hide_all).clone();
        FUN_004563f0(gfx, g, &Rect::new(r.offset_0x2c - 1, r.offset_0x30 - 1, r.offset_0x34 + 2, h), bf);
        let pf = g.res.DAT_005e8c58;
        let w = g.image(pf).offset_0x20;
        FUN_004563f0(gfx, g, &Rect::new(0xd8, 0x30, w, 0x15e), pf);
        let plate = g.res.DAT_005e8e7c;
        FUN_00455d20(gfx, g, plate, 0xdd, 0x181);
    });
    let w = g.image(g.res.DAT_005e8c58).offset_0x20;
    crate::game::board_parts::FUN_00500120(g, bubbles, &Rect::new(0xec, 0x30, w - 0x28, 0x14f));
    crate::game::board_parts::FUN_00500180(g, bubbles, 10, 3);
    crate::game::board_parts::FUN_00512080(g, bubbles);
    g.sim_fish(this).offset_0x28 = [NULL; 20];
    g.sim_fish(this).field_0x78 = NULL;

    // A multimap by purchase time (`FUN_0051ba60` makes its head node, `FUN_0052d990`
    // inserts, `FUN_0051b720` steps, `FUN_0052da00` clears).
    let mut by_time: Vec<(i64, Ptr)> = Vec::new();
    let board = g.wfa(param_1).offset_0x4;
    let objs = g.board(board).offset_0x7c.clone();
    let mut it = objs.first().copied();
    while let Some(o) = it {
        let slot = g.go(o).offset_0x24 as u32;
        if slot < 0x14 {
            let t = g.go(o).field_0x48;
            FUN_0052d990(&mut by_time, t, o);
        }
        it = crate::game::store::FUN_004e3e30(&objs, o);
    }
    for (i, &(_, o)) in by_time.iter().enumerate() {
        if i < 0x14 {
            FUN_00518cc0(g, this, i as i32, o);
        }
    }
    for i in 0..0x14 {
        if g.sim_fish(this).offset_0x28[i] == NULL {
            FUN_00518cc0(g, this, i as i32, NULL);
        }
    }

    // The overlay: a plain Widget with SimFishScreenOverlay's vftable and the screen at +0x88.
    let (mut owc, mut ow) = crate::sexy::widget::Widget();
    ow.offset_0x1 = false;
    owc.offset_0x3c = true;
    owc.offset_0x34 = 0x280;
    owc.offset_0x38 = 0x1e0;
    let overlay = g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::SimFishScreenOverlay_vftable),
        node: Node::Widget(WidgetObj { wc: owc, w: ow, ext: WExt::GameSelectorOverlay(this) }),
    });
    let d = g.sim_fish(this);
    d.offset_0x24 = overlay;
    d.field_0x78 = NULL;
    d.ext_0x114 = 0;
    d.ext_0x118 = false;
    d.ext_0x119 = false;
    d.ext_0x11a = false;
    FUN_00529cc0(g, this);
    this
}

/// port: 0052d990 FUN_0052d990
/// `std::multimap<__int64, GameObject*>::insert` (an STL instance): after the entries with
/// an equal or smaller key.
pub fn FUN_0052d990(this: &mut Vec<(i64, Ptr)>, key: i64, value: Ptr) {
    let at = this.iter().position(|e| e.0 > key).unwrap_or(this.len());
    this.insert(at, (key, value));
}

/// port: 00518cc0 FUN_00518cc0
/// `AddSlot(int i, GameObject*)`: the slot's button (disabled when empty) in a 4-column,
/// 5-row grid, and its shape cut out of the background.
pub fn FUN_00518cc0(g: &mut G, this: Ptr, param_1: i32, param_2: Ptr) {
    if 0x13 < param_1 {
        return;
    }
    let b = FishButtonWidget(g, param_2, param_1, this);
    g.sim_fish(this).offset_0x28[param_1 as usize] = b;
    let d = g.sim_fish(this).clone();
    let (x, y) = if param_1 < 5 {
        (d.ext_0x10c, param_1 * 0x53 + 0x29)
    } else if param_1 < 10 {
        (d.ext_0x110 + d.ext_0x10c, param_1 * 0x53 - 0x176)
    } else if param_1 < 0xf {
        (d.ext_0x10c + 0x18 + d.ext_0x110 * 4, param_1 * 0x53 - 0x315)
    } else {
        (d.ext_0x110 * 5 + 0x18 + d.ext_0x10c, param_1 * 0x53 - 0x4b4)
    };
    vcall!(g, b, w.vfunction41, x, y, 0x5a, 0x53);
    let mask = g.res.DAT_005e8e4c;
    let (bx, by) = (g.wc(b).offset_0x2c, g.wc(b).offset_0x30);
    FUN_005032a0(g, d.offset_0x7c, mask, bx, by);
}

/// port: 005032a0 FUN_005032a0
/// `CutOut(MemoryImage* dest, Image* mask, int x, int y)`: clears the destination's pixels
/// (to transparent black) wherever the mask, placed at (x, y), is not fully transparent.
pub fn FUN_005032a0(g: &mut G, param_1: Ptr, param_2: Ptr, param_3: i32, param_4: i32) {
    let (dw, dh) = (g.image(param_1).offset_0x20, g.image(param_1).offset_0x24);
    let (mw, mh) = (g.image(param_2).offset_0x20, g.image(param_2).offset_0x24);
    let r = Rect::new(0, 0, dw, dh).intersection(&Rect::new(param_3, param_4, mw, mh));
    if r.mWidth <= 0 || r.mHeight <= 0 {
        return;
    }
    let mask = g.image(param_2).mBits.clone();
    // (Port addition) a file image cut into no longer matches its HD art.
    let mut hd = std::mem::take(&mut g.hd);
    hd.changed(g, param_1);
    g.hd = hd;
    let dest = &mut g.image(param_1).mBits;
    for yy in 0..r.mHeight {
        for xx in 0..r.mWidth {
            let (dx, dy) = (r.mX + xx, r.mY + yy);
            let m = mask[((dy - param_4) * mw + (dx - param_3)) as usize];
            if m & 0xff00_0000 != 0 {
                dest[(dy * dw + dx) as usize] = 0;
            }
        }
    }
}

/// port: 00529cc0 FUN_00529cc0
/// `UpdateLabels()`: "Hide All" (id 0x69) while any creature shows, else "Show All" (id
/// 0x6a); the toggle reads "Show" for a hidden selected creature, else "Hide".
pub fn FUN_00529cc0(g: &mut G, this: Ptr) {
    let d = g.sim_fish(this).clone();
    let any_shown = d.offset_0x28.iter().any(|&b| {
        if b == NULL {
            return false;
        }
        let f = g.fish_button(b).offset_0x38;
        f != NULL && g.go(f).offset_0x6c
    });
    if any_shown {
        g.btn(d.offset_0x1c).field_0x4 = b"Hide All".to_vec();
        g.btn(d.offset_0x1c).offset_0x0 = 0x69;
    } else {
        g.btn(d.offset_0x1c).field_0x4 = b"Show All".to_vec();
        g.btn(d.offset_0x1c).offset_0x0 = 0x6a;
    }
    let sel = d.field_0x78;
    let show = sel != NULL && {
        let f = g.fish_button(sel).offset_0x38;
        f != NULL && !g.go(f).offset_0x6c
    };
    g.btn(d.offset_0x10).field_0x4 = if show { b"Show".to_vec() } else { b"Hide".to_vec() };
}

/// port: 00518b80 Sexy::SimFishScreen::~SimFishScreen
pub fn dtor_SimFishScreen(g: &mut G, this: Ptr) {
    let d = g.sim_fish(this).clone();
    if d.offset_0x4 != NULL {
        crate::game::board_parts::deleting_destructor__00505420(g, d.offset_0x4, 1);
    }
    for b in d.offset_0x28 {
        if b != NULL {
            vcall!(g, b, w.vfunction1, 1);
        }
    }
    for p in [d.offset_0x1c, d.offset_0x20, d.offset_0x8, d.offset_0xc] {
        if p != NULL {
            vcall!(g, p, w.vfunction1, 1);
        }
    }
    if d.offset_0x7c != NULL {
        g.free(d.offset_0x7c);
    }
    for p in [d.offset_0x24, d.offset_0x10, d.offset_0x14, d.offset_0x18] {
        if p != NULL {
            vcall!(g, p, w.vfunction1, 1);
        }
    }
    crate::sexy::widget::dtor_Widget(g, this);
}

/// port: 0051b1d0 Sexy::SimFishScreen::deleting_destructor
pub fn deleting_destructor(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_SimFishScreen(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

fn children(d: &SimFishScreen_data) -> Vec<Ptr> {
    let mut v: Vec<Ptr> = d.offset_0x28.to_vec();
    v.extend([d.offset_0x24, d.offset_0x8, d.offset_0x1c, d.offset_0x20, d.offset_0x14, d.offset_0x10, d.offset_0x18]);
    v
}

/// port: 00518e40 Sexy::SimFishScreen::vfunction21
/// `AddedToManager(WidgetManager*)`: the slots, the overlay, and the buttons ("Menu" is
/// never added).
pub fn vfunction21(g: &mut G, this: Ptr, param_1: Ptr) {
    crate::sexy::widget_container::vfunction21(g, this, param_1);
    let d = g.sim_fish(this).clone();
    for p in children(&d) {
        vcall!(g, param_1, w.vfunction4, p);
    }
}

/// port: 00518ef0 Sexy::SimFishScreen::vfunction22
/// `RemovedFromManager(WidgetManager*)`.
pub fn vfunction22(g: &mut G, this: Ptr, param_1: Ptr) {
    crate::sexy::widget_container::vfunction22(g, this, param_1);
    let d = g.sim_fish(this).clone();
    for p in children(&d) {
        vcall!(g, param_1, w.vfunction5, p);
    }
}

/// port: 00518fa0 Sexy::SimFishScreen::vfunction31
/// `OrderInManagerChanged()`: brings them to the front.
pub fn vfunction31(g: &mut G, this: Ptr) {
    let wm = g.wc(this).offset_0xc;
    let d = g.sim_fish(this).clone();
    for p in children(&d) {
        vcall!(g, wm, w.vfunction12, p);
    }
}

/// port: 00519040 Sexy::SimFishScreen::vfunction23
/// `Update()`: redraw, the bubbles, and the card cross-fade (down while the price card is
/// to show, else up to 8 and off).
pub fn vfunction23(g: &mut G, this: Ptr) {
    vcall!(g, this, w.vfunction18);
    let b = g.sim_fish(this).offset_0x4;
    crate::game::board_parts::FUN_005110e0(g, b);
    let d = g.sim_fish(this);
    let n = d.ext_0x114;
    if 0 < n {
        if d.ext_0x118 {
            d.ext_0x114 = n - 1;
            return;
        }
        d.ext_0x114 = n + 1;
        if 7 < n + 1 {
            d.ext_0x114 = 0;
        }
    }
}

/// port: 00519090 Sexy::SimFishScreen::vfunction1
/// `ButtonPress(int id, int clickCount)`: the click sound; double-clicking a slot toggles
/// its creature (as Show / Hide).
pub fn vfunction1(g: &mut G, this: Ptr, param_1: i32, param_2: i32) {
    let app = g.sim_fish(this).field_0x0;
    let s = g.res.DAT_005e8c40;
    crate::sexy::sexy_app_base::vfunction55(g, app, s);
    if param_2 == 2 && (param_1 as u32) < 0x14 {
        vcall!(g, this, bl.vfunction3, 0x66);
    }
}

/// port: 005190d0 Sexy::SimFishScreen::vfunction5
/// `ButtonMouseEnter(int id)`: hovering Sell shows the price card (fading in from 8).
pub fn vfunction5(g: &mut G, this: Ptr, param_1: i32) {
    if param_1 == 0x67 {
        let d = g.sim_fish(this);
        d.ext_0x118 = true;
        d.ext_0x114 = 8;
    }
}

/// port: 005190f0 Sexy::SimFishScreen::vfunction6
/// `ButtonMouseLeave(int id)`: leaving Sell (unless its question is up) fades the price
/// card out.
pub fn vfunction6(g: &mut G, this: Ptr, param_1: i32) {
    if param_1 == 0x67 && !g.sim_fish(this).ext_0x119 {
        let d = g.sim_fish(this);
        d.ext_0x118 = false;
        d.ext_0x114 = 1;
    }
}

/// port: 00529dd0 Sexy::SimFishScreen::vfunction3
/// `ButtonDepress(int id)`: a slot selects it; "Return to Tank" (100) stops the music,
/// closes the screen, re-enters the virtual tank when something changed, saves the tank and
/// unpauses; Sell (0x67) asks (mentioning a mama's baby); Show / Hide (0x66) toggles the
/// selected creature; Rename (0x68) asks for a name; Hide All / Show All toggle them all.
pub fn vfunction3(g: &mut G, this: Ptr, param_1: i32) {
    let app = g.sim_fish(this).field_0x0;
    let sel = g.sim_fish(this).field_0x78;
    let sel_fish = |g: &mut G| if sel == NULL { NULL } else { g.fish_button(sel).offset_0x38 };
    if (param_1 as u32) < 0x14 {
        let b = g.sim_fish(this).offset_0x28[param_1 as usize];
        if b == NULL {
            return;
        }
        if sel != NULL {
            if g.btn(sel).offset_0x0 == param_1 {
                return;
            }
            FUN_00532c20(g, sel, false);
        }
        g.sim_fish(this).field_0x78 = b;
        FUN_00532c20(g, b, true);
        FUN_00529cc0(g, this);
        return;
    }
    match param_1 {
        100 => {
            crate::game::win_fish_app::FUN_0054b020(g, app);
            FUN_0054ae70(g, app);
            if g.sim_fish(this).ext_0x11a {
                crate::game::store::FUN_0054aff0(g, app);
            }
            let board = g.wfa(app).offset_0x4;
            if board != NULL {
                crate::game::board_save::FUN_00538940(g, board);
                let board = g.wfa(app).offset_0x4;
                crate::game::board_update::FUN_0053db80(g, board, false);
            }
        }
        0x67 => {
            let f = sel_fish(g);
            if f != NULL {
                let d = g.sim_fish(this);
                d.ext_0x119 = true;
                d.ext_0x118 = true;
                d.ext_0x114 = 0;
                let mut msg = b"Are you sure you want to sell your fish?".to_vec();
                if g.go(f).offset_0x4 == 10 {
                    let board = g.wfa(app).offset_0x4;
                    let slot = g.go(f).offset_0x24;
                    if crate::game::breeder::FUN_0053a810(g, board, slot + 100) != NULL {
                        msg.extend_from_slice(b"\n\nNote that selling the mama fish will not sell her baby fish.");
                    }
                }
                crate::game::sim_setup::FUN_0054fc90(g, app, &msg);
            }
        }
        0x66 => {
            let f = sel_fish(g);
            if f != NULL {
                g.sim_fish(this).ext_0x11a = true;
                let board = g.wfa(app).offset_0x4;
                let shown = g.go(f).offset_0x6c;
                FUN_005449c0(g, board, f, !shown);
                FUN_00529cc0(g, this);
                let wm = g.wc(this).offset_0xc;
                vcall!(g, wm, w.vfunction12, this);
            }
        }
        0x68 => {
            let f = sel_fish(g);
            if f != NULL {
                g.sim_fish(this).ext_0x11a = true;
                let female = g.go(f).offset_0x4 == 10;
                let name = g.go(f).field_0x28.clone();
                crate::game::store::FUN_0054c800(g, app, b"Please choose a name for your fish", female, &name, false);
            }
        }
        0x69 | 0x6a => {
            g.sim_fish(this).ext_0x11a = true;
            let show = param_1 == 0x6a;
            let slots = g.sim_fish(this).offset_0x28;
            for b in slots {
                if b != NULL {
                    let f = g.fish_button(b).offset_0x38;
                    if f != NULL {
                        let board = g.wfa(app).offset_0x4;
                        FUN_005449c0(g, board, f, show);
                    }
                }
                let wm = g.wc(this).offset_0xc;
                vcall!(g, wm, w.vfunction12, this);
                FUN_00529cc0(g, this);
            }
        }
        _ => {}
    }
}

/// `localtime` + `strftime("%b %d %Y")` for the purchase date (`_localtime64` fails past
/// the year 3000, giving "Unknown").
fn purchase_date(g: &mut G, t: i64) -> Vec<u8> {
    if t > 32535215999 {
        return b"Unknown".to_vec();
    }
    let local = t + g.utc_offset_secs;
    let days = local.div_euclid(86400);
    // Civil date from days since 1970-01-01.
    let z = days + 719468;
    let era = z.div_euclid(146097);
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + if month <= 2 { 1 } else { 0 };
    const MON: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    format!("{} {:02} {}", MON[(month - 1) as usize], day, year).into_bytes()
}

/// port: 00528dd0 Sexy::SimFishScreen::vfunction45
/// `DrawOverlay(Graphics*)`: the background, bubbles, "Fish Setup", each slot's frame (and
/// "HIDDEN" over hidden creatures); then "Select a Fish", or the selected creature's
/// portrait, its name (" JR." on its own line when the name is long), its card (fading
/// out while the price card fades in) and the price card.
pub fn vfunction45(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    let ftol = crate::sexy::crt::ftol;
    let d = g.sim_fish(this).clone();
    FUN_00455d20(gfx, g, d.offset_0x7c, 0, 0);
    crate::game::board_parts::FUN_00504b60(g, d.offset_0x4, gfx);
    let f = g.res.DAT_005e8e40;
    FUN_00455880(gfx, f);
    FUN_00455890(gfx, CRect(0xff, 200, 0, 0xff));
    vcall!(g, this, w.vfunction63, gfx, 0x19, b"Fish Setup");
    for b in d.offset_0x28 {
        let (x, y) = (g.wc(b).offset_0x2c, g.wc(b).offset_0x30);
        let back = g.res.DAT_005e8a88;
        FUN_00455d20(gfx, g, back, x, y);
        if !g.w(b).offset_0x2 {
            let rim = g.res.DAT_005e8a38;
            FUN_00455d20(gfx, g, rim, x, y);
            let fish = g.fish_button(b).offset_0x38;
            if fish != NULL && !g.go(fish).offset_0x6c {
                let f = g.res.DAT_005e8a5c;
                FUN_00455880(gfx, f);
                FUN_00455890(gfx, FUN_00433320(0xff6060));
                FUN_00455cf0(gfx, g, b"HIDDEN", x + 0xd, y + 0x32);
            }
        }
    }
    let sel = d.field_0x78;
    let fish = if sel == NULL { NULL } else { g.fish_button(sel).offset_0x38 };
    if fish == NULL {
        let f = g.res.DAT_005e8e18;
        FUN_00455880(gfx, f);
        FUN_00455890(gfx, CRect(0xff, 200, 0, 0xff));
        vcall!(g, this, w.vfunction65, gfx, Rect::new(0xe6, 0xb4, 0xb4, 100), b"Select\na\nFish", -1, 0);
        return;
    }
    FUN_004563d0(gfx, 0x140, 0x5a);
    vcall!(g, fish, go.vfunction80, gfx, 4);
    FUN_004563d0(gfx, -0x140, -0x5a);
    let f = g.res.DAT_005e8e18;
    FUN_00455880(gfx, f);
    FUN_00455890(gfx, CRect(0xff, 200, 0, 0xff));
    let name = g.go(fish).field_0x28.clone();
    let fnt = FUN_00455870(gfx);
    let nw = font::string_width(g, fnt, &name);
    if nw < 0xab || name.len() < 5 || !name.ends_with(b" JR.") {
        vcall!(g, this, w.vfunction63, gfx, 0x50, &name);
    } else {
        let first = name[..name.len() - 4].to_vec();
        vcall!(g, this, w.vfunction63, gfx, 0x50, &first);
        vcall!(g, this, w.vfunction63, gfx, 100, b"JR.");
    }
    if !g.go(fish).offset_0x6c {
        FUN_00455890(gfx, FUN_00433320(0xff6060));
    }
    let mut label = CRect(0xff, 200, 0, 0xff);
    let mut value = FUN_00433360(0xff, 0xff, 0xff);
    let f = g.res.DAT_005e8cf0;
    FUN_00455880(gfx, f);
    let fnt = FUN_00455870(gfx);
    let hh = font::get_height(g, fnt);
    let sp = ftol(hh as f64 * 1.75) as i32;
    let n = d.ext_0x114;
    if !d.ext_0x118 || 0 < n {
        if 0 < n {
            let a = (n * 0xff) / 8;
            label.mAlpha = a;
            value.mAlpha = a;
        }
        FUN_00455890(gfx, label);
        let born = g.go(fish).offset_0x4 == 0 && g.fish(fish).offset_0xd6;
        let l: &[u8] = if born { b"Date of Birth" } else { b"Purchase Date" };
        vcall!(g, this, w.vfunction63, gfx, 0xb4, l);
        let mut t = g.go(fish).field_0x48;
        if t < 0 {
            t = 0;
        }
        let date = purchase_date(g, t);
        FUN_00455890(gfx, value);
        let mut y = hh + 0xb4;
        vcall!(g, this, w.vfunction63, gfx, y, &date);
        FUN_00455890(gfx, label);
        y += sp;
        vcall!(g, this, w.vfunction63, gfx, y, b"Hometown");
        FUN_00455890(gfx, value);
        y += hh;
        let special = g.go(fish).offset_0x9c;
        let pref = g.go(fish).offset_0x54;
        // (0..=0x176 is exactly the table.)
        let forced = usize::try_from(DAT_005e1604).ok().and_then(|i| crate::game::fish_tables::PTR_s_Virtual_Tank_005e0430.get(i));
        let town: Vec<u8> = if let Some(t) = forced {
            t.to_vec()
        } else if (0..=5).contains(&special) {
            crate::game::fish_tables::PTR_s_Philadelphia__Pa__005e0414[special as usize].to_vec()
        } else if (0..=0x176).contains(&pref) {
            crate::game::fish_tables::PTR_s_Virtual_Tank_005e0430[pref as usize].to_vec()
        } else {
            b"Unknown".to_vec()
        };
        vcall!(g, this, w.vfunction63, gfx, y, &town);
        FUN_00455890(gfx, label);
        y += sp;
        vcall!(g, this, w.vfunction63, gfx, y, b"Mental State");
        FUN_00455890(gfx, value);
        y += hh;
        let mood = crate::game::game_object::FUN_004d65f0(g, fish);
        vcall!(g, this, w.vfunction63, gfx, y, mood);
        FUN_00455890(gfx, label);
        y += sp;
        vcall!(g, this, w.vfunction63, gfx, y, b"Additional Notes");
        FUN_00455890(gfx, value);
        // (0..=0x14a is exactly the table.)
        let forced = usize::try_from(DAT_005e1608).ok().and_then(|i| crate::game::fish_tables::PTR_s_Knitting_005dfee8.get(i));
        let notes: Vec<u8> = if let Some(t) = forced {
            let h = String::from_utf8_lossy(t).into_owned();
            format!("Likes {h}, {h}, and {h}.").into_bytes()
        } else {
            FUN_00506b40(g, fish)
        };
        vcall!(g, this, w.vfunction65, gfx, Rect::new(0xea, y + 5, 0xac, 100), &notes, -1, 0);
    }
    if !d.ext_0x118 && n < 1 {
        return;
    }
    if 0 < n {
        let a = 0xff - (n * 0xff) / 8;
        label.mAlpha = a;
        value.mAlpha = a;
    }
    FUN_00455890(gfx, label);
    vcall!(g, this, w.vfunction63, gfx, 0xbe, b"Purchase Price");
    FUN_00455890(gfx, value);
    let mut y = hh + 0xbe;
    let born = g.go(fish).offset_0x4 == 0 && g.fish(fish).offset_0xd6;
    let price: Vec<u8> = if born { b"N/A".to_vec() } else { format!("{} Shells", g.go(fish).field_0x58).into_bytes() };
    vcall!(g, this, w.vfunction63, gfx, y, &price);
    let resale = vcall!(g, fish, go.vfunction73);
    let rs: Vec<u8> = if resale == -1 { b"Full Refund".to_vec() } else { format!("{resale} Shells").into_bytes() };
    FUN_00455890(gfx, label);
    y += sp;
    vcall!(g, this, w.vfunction63, gfx, y, b"Resale Value");
    FUN_00455890(gfx, value);
    vcall!(g, this, w.vfunction63, gfx, y + hh, &rs);
}

/// port: 00506b40 FUN_00506b40
/// The creature's notes: a special fish's own line, else "Likes a, b, and c." from its
/// three hobbies (the third replaced by what its trait class makes it like).
pub fn FUN_00506b40(g: &mut G, param_2: Ptr) -> Vec<u8> {
    let s: &[u8] = match g.go(param_2).offset_0x9c {
        0 => b"By doctor's orders, is on a special low-carb high-Ultravore diet.",
        1 => b"Likes monster truck rallies, Thai kick-boxing, and Beethoven.",
        2 => b"A fishy philanthropist who likes to feed food to famished fish? That's Fish-tastic!",
        3 => b"Has been trying to stop eating fast food and pizza.",
        4 => b"Likes to intimidate other fish by playing the music of Wagner.",
        5 => b"Likes giving lots of toys to all the good girls and boys.",
        _ => {
            let h = crate::game::fish_tables::PTR_s_Knitting_005dfee8;
            let p = g.go(param_2).offset_0x5c;
            let third = match FUN_00501120(g, param_2) {
                Some(t) => t.to_vec(),
                None => h[p[2] as usize].to_vec(),
            };
            let a = String::from_utf8_lossy(h[p[0] as usize]).into_owned();
            let b = String::from_utf8_lossy(h[p[1] as usize]).into_owned();
            let c = String::from_utf8_lossy(&third).into_owned();
            return format!("Likes {a}, {b}, and {c}.").into_bytes();
        }
    };
    s.to_vec()
}

/// port: 00501120 FUN_00501120
/// What the creature's trait class makes it like (null for none).
pub fn FUN_00501120(g: &mut G, param_1: Ptr) -> Option<&'static [u8]> {
    let class = crate::game::store::FUN_00501060(g, param_1);
    if class == 9 {
        return None;
    }
    let feeder = g.go(param_1).offset_0x68;
    match class {
        0 => Some(b"stealth"),
        1 => Some(b"eating"),
        2 => Some(b"quickness"),
        3 => Some(b"singing"),
        4 => Some(b"swimming backwards"),
        5 => match feeder {
            3 => Some(b"pizza"),
            4 => Some(b"ice cream"),
            5 => Some(b"chicken"),
            _ => None,
        },
        6 => match feeder {
            1000 => Some(b"eating guppies"),
            0x3ed => Some(b"eating carnivores"),
            0x3ee => Some(b"eating ultravores"),
            _ => None,
        },
        _ => Some(b"being different"),
    }
}

/// port: 005449c0 FUN_005449c0
/// `SetShown(GameObject*, bool)`: a mama's baby follows her; showing puts the creature back
/// on the board (out of and back into the object set through `AddObject`, onto the widget
/// manager, its shadow); hiding drops its speech bubble and takes it off the board but keeps
/// it in the object set. Either marks +0x2a7.
pub fn FUN_005449c0(g: &mut G, this: Ptr, param_1: Ptr, param_2: bool) {
    if g.go(param_1).offset_0x4 == 10 {
        let slot = g.go(param_1).offset_0x24;
        let baby = crate::game::breeder::FUN_0053a810(g, this, slot + 100);
        if baby != NULL {
            FUN_005449c0(g, this, baby, param_2);
        }
    }
    if g.go(param_1).offset_0x6c == param_2 {
        return;
    }
    if param_2 {
        crate::game::board_level::FUN_005418e0(&mut g.board(this).offset_0x7c, param_1);
        crate::game::board_level::FUN_00542ee0(g, this, param_1, 1);
        let wm = g.wc(this).offset_0xc;
        vcall!(g, wm, w.vfunction4, param_1);
        crate::game::board_level::FUN_00544800(g, this, param_1);
        g.go(param_1).offset_0x6c = true;
        g.board(this).field_0x21b = true;
        return;
    }
    let msg = g.go(param_1).offset_0x94;
    if msg != -1 {
        crate::game::board::FUN_005384a0(g, this, msg);
    }
    crate::game::alien::FUN_004d6830(g, param_1, false);
    crate::game::board_level::FUN_00540ec0(&mut g.board(this).offset_0x7c, param_1);
    g.go(param_1).offset_0x6c = false;
    g.board(this).field_0x21b = true;
}

/// port: 0052a090 FUN_0052a090
/// The sale answered: the price card fades out; on yes, the selected creature is sold for
/// its resale value (its price when that is not positive) and leaves the board; a mama's
/// baby takes her slot and her place on the button (else the slot is disabled).
pub fn FUN_0052a090(g: &mut G, this: Ptr, param_1: bool) {
    let d = g.sim_fish(this);
    d.ext_0x119 = false;
    d.ext_0x118 = false;
    d.ext_0x114 = 1;
    let sel = d.field_0x78;
    let app = d.field_0x0;
    if !param_1 || sel == NULL {
        return;
    }
    let fish = g.fish_button(sel).offset_0x38;
    if fish == NULL {
        return;
    }
    g.sim_fish(this).ext_0x11a = true;
    let mut v = vcall!(g, fish, go.vfunction73);
    if v < 1 {
        v = g.go(fish).field_0x58;
    }
    let profile = g.wfa(app).offset_0x18c;
    crate::game::profile::FUN_00501200(g, profile, v);
    crate::game::alien::FUN_004d6830(g, fish, false);
    let mut baby = NULL;
    if g.go(fish).offset_0x4 == 10 {
        let board = g.wfa(app).offset_0x4;
        let slot = g.go(fish).offset_0x24;
        baby = crate::game::breeder::FUN_0053a810(g, board, slot + 100);
        if baby != NULL {
            g.go(baby).offset_0x24 = slot;
        }
    }
    crate::sexy::sexy_app_base::vfunction35(g, app, fish);
    g.fish_button(sel).offset_0x38 = baby;
    vcall!(g, sel, w.vfunction38, baby == NULL);
    FUN_00529cc0(g, this);
}

/// port: 00529da0 FUN_00529da0
/// The new name, for the selected creature.
pub fn FUN_00529da0(g: &mut G, this: Ptr, param_1: &[u8]) {
    let sel = g.sim_fish(this).field_0x78;
    if sel != NULL {
        let f = g.fish_button(sel).offset_0x38;
        if f != NULL {
            g.go(f).field_0x28 = param_1.to_vec();
        }
    }
}

/// port: 0054c060 FUN_0054c060
/// `ShowSimFishScreen()`: closes the dialogs and any fish screen, pauses the board, and
/// opens the fish screen full-screen with the focus.
pub fn FUN_0054c060(g: &mut G, this: Ptr) {
    crate::game::win_fish_app::FUN_0054b4c0(g, this);
    FUN_0054ae70(g, this);
    let board = g.wfa(this).offset_0x4;
    if board != NULL {
        crate::game::board_update::FUN_0053db80(g, board, true);
    }
    let s = SimFishScreenOverlay(g, this);
    g.wfa(this).offset_0xfc = s;
    let (w, h) = (g.sab(this).field_0xb8, g.sab(this).field_0xbc);
    vcall!(g, s, w.vfunction41, 0, 0, w, h);
    let wm = g.sab(this).offset_0x318;
    vcall!(g, wm, w.vfunction4, s);
    vcall!(g, wm, w.vfunction9, s);
}

/// port: 0054ae70 FUN_0054ae70
/// `RemoveSimFishScreen()` (+0x828), giving the board the focus back.
pub fn FUN_0054ae70(g: &mut G, this: Ptr) {
    let s = g.wfa(this).offset_0xfc;
    if s != NULL {
        let wm = g.sab(this).offset_0x318;
        vcall!(g, wm, w.vfunction5, s);
        crate::sexy::sexy_app_base::vfunction35(g, this, s);
        g.wfa(this).offset_0xfc = NULL;
        let board = g.wfa(this).offset_0x4;
        if board != NULL {
            vcall!(g, wm, w.vfunction9, board);
        }
    }
}

/// port: 00532410 Sexy::FishButtonWidget::FishButtonWidget
/// `FishButtonWidget(GameObject*, int theId, ButtonListener*)`: cels from quarters of the
/// slot image (normal the second, down the third, over the fourth, disabled the first; the
/// third is also the selected look); disabled without a creature.
pub fn FishButtonWidget(g: &mut G, param_1: Ptr, param_2: i32, param_3: Ptr) -> Ptr {
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
    let d = FishButtonWidget_data {
        offset_0x0: Rect::new(0, 0, 0, 0),
        offset_0x10: Rect::new(q * 2, 0, q, ih),
        offset_0x20: Rect::new(0, 0, 0, 0),
        offset_0x30: false,
        offset_0x34: app,
        offset_0x38: param_1,
    };
    if param_1 == NULL {
        w.offset_0x2 = true;
    }
    g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__FishButtonWidget_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::Button(Box::new(ButtonExt { b, sub: BtnSub::FishButton(d) })) }),
    })
}

/// port: 00530180 Sexy::FishButtonWidget::~FishButtonWidget
pub fn dtor_FishButtonWidget(g: &mut G, this: Ptr) {
    crate::sexy::button_widget::dtor_ButtonWidget(g, this);
}

/// port: 00532550 Sexy::FishButtonWidget::deleting_destructor
pub fn deleting_destructor__00532550(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_FishButtonWidget(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 00530190 Sexy::FishButtonWidget::vfunction23
/// `Update()`: the creature animates (its virtual #79).
pub fn vfunction23__00530190(g: &mut G, this: Ptr) {
    let f = g.fish_button(this).offset_0x38;
    if f != NULL {
        vcall!(g, f, go.vfunction79);
    }
}

/// port: 005301b0 Sexy::FishButtonWidget::vfunction27
/// `Draw(Graphics*)`: the button, then the creature's icon (pose 3).
pub fn vfunction27__005301b0(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    crate::sexy::button_widget::vfunction27(g, this, gfx);
    let f = g.fish_button(this).offset_0x38;
    if f != NULL {
        vcall!(g, f, go.vfunction80, gfx, 3);
    }
}

/// port: 00532c20 FUN_00532c20
/// `SetSelected(bool)`: swaps the normal, over and down cels with the selected ones, and
/// redraws.
pub fn FUN_00532c20(g: &mut G, this: Ptr, param_1: bool) {
    if param_1 == g.fish_button(this).offset_0x30 {
        return;
    }
    g.fish_button(this).offset_0x30 = param_1;
    let fb = g.fish_button(this).clone();
    let b = g.btn(this);
    let (n, o, dn) = (b.offset_0x38, b.offset_0x48, b.offset_0x58);
    b.offset_0x38 = fb.offset_0x10;
    b.offset_0x48 = fb.offset_0x0;
    b.offset_0x58 = fb.offset_0x20;
    let f = g.fish_button(this);
    f.offset_0x10 = n;
    f.offset_0x0 = o;
    f.offset_0x20 = dn;
    vcall!(g, this, w.vfunction18);
}
