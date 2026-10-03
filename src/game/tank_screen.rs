//! `Sexy::TankScreen`: picking the tank for a Time Trial or a Challenge ("Choose a Tank"
//! otherwise): four tank buttons in a 2x2 grid, each framed with its tank's pet portrait or,
//! while locked, how to unlock it; "Stories" (Challenge, once a story is unlocked); "Menu".
//! Plus the app functions that open and close it.
//!
//! `TankScreen_data` starts at object offset 0x8c (after the ButtonListener vftable at
//! 0x88). The object is 0xc8 bytes, past the database's 0xa8: the four tanks' frame
//! positions are at +0xa8..+0xc4 (`ext_0xa8`).

use crate::game::game_selector::{FUN_00506760, FUN_00506820};
use crate::sexy::graphics::{FUN_00455880, FUN_00455890, FUN_00455d20, FUN_004563f0};
use crate::sexy::prelude::*;
use crate::sexy::types::{CRect, FUN_00433320, FUN_00433360};

/// `TankScreen_data` (object offset 0x8c) plus the frame positions past the database size.
#[derive(Debug, Clone, Default)]
pub struct TankScreen_data {
    /// +0x8c the app.
    pub field_0x0: Ptr,
    /// +0x90 "Menu" (id 99).
    pub offset_0x4: Ptr,
    /// +0x94..+0xa0 "Tank 1".."Tank 4" (ids 0..3).
    pub offset_0x8: [Ptr; 4],
    /// +0xa4 "Stories" (id 4).
    pub offset_0x18: Ptr,
    /// +0xa8..+0xc4 each tank's frame (x, y).
    pub ext_0xa8: [(i32, i32); 4],
}

impl G {
    pub fn tank_screen(&mut self, p: Ptr) -> &mut TankScreen_data {
        match &mut self.widget(p).ext {
            WExt::TankScreen(d) => d,
            e => panic!("{p} is not a TankScreen: {e:?}"),
        }
    }
}

/// port: 0052cb40 Sexy::TankScreen::TankScreen
/// `TankScreen(WinFishApp*)`: full screen; "Menu"; "Stories" (shown in a Challenge once a
/// story is unlocked); the frames (the bottom row 20 pixels lower with "Stories"); a button
/// per tank under its frame.
pub fn TankScreen(g: &mut G, param_1: Ptr) -> Ptr {
    let (mut wc, w) = crate::sexy::widget::Widget();
    wc.offset_0x2c = 0;
    wc.offset_0x30 = 0;
    wc.offset_0x34 = g.sab(param_1).field_0xb8;
    wc.offset_0x38 = g.sab(param_1).field_0xbc;
    let d = TankScreen_data { field_0x0: param_1, ..Default::default() };
    let this = g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__TankScreen_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::TankScreen(Box::new(d)) }),
    });
    let img = g.res.DAT_005e8c1c;
    let menu = FUN_00506820(g, 99, this, b"Menu", img);
    g.tank_screen(this).offset_0x4 = menu;
    let h = g.wc(menu).offset_0x38;
    vcall!(g, menu, w.vfunction41, 0x20d, 4, 0x50, h);
    let f = g.res.DAT_005e8a5c;
    let stories = FUN_00506760(g, 4, this, b"Stories", f);
    g.tank_screen(this).offset_0x18 = stories;
    let h = g.wc(stories).offset_0x38;
    vcall!(g, stories, w.vfunction41, 0xf5, 0xf7, 0x96, h);
    vcall!(g, stories, w.vfunction35, 0, FUN_00433360(0xff, 0xf0, 0));
    let profile = g.wfa(param_1).offset_0x18c;
    if g.profile(profile).field_0x7c == 0 || g.wfa(param_1).offset_0x150 != 4 {
        g.w(stories).offset_0x0 = false;
    }
    let low = if g.w(stories).offset_0x0 { 0x108 + 0x14 } else { 0x108 };
    g.tank_screen(this).ext_0xa8 = [(0x46, 0x47), (0x15e, 0x47), (0x46, low), (0x15e, low)];
    for i in 0..4 {
        let label = format!("Tank {}", i + 1).into_bytes();
        let b = FUN_00506820(g, i as i32, this, &label, img);
        g.tank_screen(this).offset_0x8[i] = b;
        vcall!(g, b, btn.vfunction72, f);
        let (x, y) = g.tank_screen(this).ext_0xa8[i];
        let h = g.wc(b).offset_0x38;
        vcall!(g, b, w.vfunction41, x + 2, y + 0x86, 0xc1, h);
    }
    this
}

/// port: 0051a450 Sexy::TankScreen::~TankScreen
pub fn dtor_TankScreen(g: &mut G, this: Ptr) {
    let d = g.tank_screen(this).clone();
    for b in d.offset_0x8 {
        if b != NULL {
            vcall!(g, b, w.vfunction1, 1);
        }
    }
    for b in [d.offset_0x4, d.offset_0x18] {
        if b != NULL {
            vcall!(g, b, w.vfunction1, 1);
        }
    }
    crate::sexy::widget::dtor_Widget(g, this);
}

/// port: 0051b690 Sexy::TankScreen::deleting_destructor_for_Widget
pub fn deleting_destructor_for_Widget(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_TankScreen(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 0051a500 Sexy::TankScreen::vfunction21_for_Widget
/// `AddedToManager(WidgetManager*)`: "Menu"; the tank buttons, a locked one disabled and
/// drawn as a plain white label (Time Trial: tanks past the adventure's reach until it is
/// finished; Challenge: tanks after the first until unlocked there); "Stories".
pub fn vfunction21_for_Widget(g: &mut G, this: Ptr, param_1: Ptr) {
    crate::sexy::widget_container::vfunction21(g, this, param_1);
    let d = g.tank_screen(this).clone();
    vcall!(g, param_1, w.vfunction4, d.offset_0x4);
    let app = d.field_0x0;
    for i in 0..4 {
        let b = d.offset_0x8[i];
        let profile = g.wfa(app).offset_0x18c;
        let p = g.profile(profile).clone();
        let mode = g.wfa(app).offset_0x150;
        let locked = if mode == 1 {
            !(i as i32 + 2 <= p.field_0x1c || p.field_0x59)
        } else {
            mode == 4 && 0 < i && !p.field_0x78[i - 1]
        };
        if locked {
            vcall!(g, b, w.vfunction38, true);
            g.dialog_button(b).offset_0x0 = NULL;
            g.btn(b).offset_0x7a = true;
            vcall!(g, b, w.vfunction35, 0, FUN_00433320(0xffffff));
        } else {
            vcall!(g, b, w.vfunction38, false);
        }
        vcall!(g, param_1, w.vfunction4, b);
    }
    vcall!(g, param_1, w.vfunction4, d.offset_0x18);
}

/// port: 0051a600 Sexy::TankScreen::vfunction22_for_Widget
/// `RemovedFromManager(WidgetManager*)`.
pub fn vfunction22_for_Widget(g: &mut G, this: Ptr, param_1: Ptr) {
    crate::sexy::widget_container::vfunction22(g, this, param_1);
    let d = g.tank_screen(this).clone();
    vcall!(g, param_1, w.vfunction5, d.offset_0x4);
    for b in d.offset_0x8 {
        vcall!(g, param_1, w.vfunction5, b);
    }
    vcall!(g, param_1, w.vfunction5, d.offset_0x18);
}

/// port: 0051a6e0 Sexy::TankScreen::vfunction23_for_Widget
/// `Update()`: redraw.
pub fn vfunction23_for_Widget(g: &mut G, this: Ptr) {
    vcall!(g, this, w.vfunction18);
}

/// port: 0051a660 FUN_0051a660
/// One corner ornament at (x, y) (EDI, ESI), mirrored across the screen horizontally
/// and/or vertically as asked (both: all four corners).
pub fn FUN_0051a660(g: &mut G, gfx: &mut Graphics, x: i32, y: i32, param_1: bool, param_2: bool) {
    let img = g.res.DAT_005e8b18;
    FUN_00455d20(gfx, g, img, x, y);
    if param_1 {
        FUN_00455d20(gfx, g, img, 0x280 - x, y);
    }
    if param_2 {
        FUN_00455d20(gfx, g, img, x, 0x1e0 - y);
    }
    if param_1 && param_2 {
        FUN_00455d20(gfx, g, img, 0x280 - x, 0x1e0 - y);
    }
}

/// port: 0052ce30 Sexy::TankScreen::vfunction27_for_Widget
/// `Draw(Graphics*)`: the frame, rails and posts, the corner ornaments, the title bar and
/// Menu frame, the title and the mode's question, then each tank's frame with its pet
/// portrait, or how to unlock it.
pub fn vfunction27_for_Widget(g: &mut G, this: Ptr, param_1: &mut Graphics) {
    use crate::game::help_screen::{FUN_00503390, FUN_00503400};
    let frame = g.res.DAT_005e8c70;
    FUN_004563f0(param_1, g, &Rect::new(-5, -5, 0x28a, 0x1ea), frame);
    let d = g.tank_screen(this).clone();
    let stories = g.w(d.offset_0x18).offset_0x0;
    let rail = g.res.DAT_005e8a58;
    FUN_00503390(g, param_1, rail, 0xd, 0x82, 0x266);
    FUN_00503390(g, param_1, rail, 0xd, if stories { 0x15e } else { 0x140 }, 0x266);
    let post = g.res.DAT_005e8cbc;
    FUN_00503400(g, param_1, post, 0xb4, 100, 0x171);
    FUN_00503400(g, param_1, post, 0x1cc, 100, 0x171);
    FUN_0051a660(g, param_1, 0x32, 0x6e, true, true);
    FUN_0051a660(g, param_1, 0x1e, 0x96, true, true);
    FUN_0051a660(g, param_1, 0x140, 0x64, false, true);
    FUN_0051a660(g, param_1, 0x140, 0x9e, false, true);
    let bar = g.res.DAT_005e8d3c;
    let h = g.image(bar).offset_0x24;
    FUN_004563f0(param_1, g, &Rect::new(0x14, 0, 600, h), bar);
    let mf = g.res.DAT_005e8e6c;
    let h = g.image(mf).offset_0x24;
    let m = g.wc(d.offset_0x4).clone();
    FUN_004563f0(param_1, g, &Rect::new(m.offset_0x2c - 1, m.offset_0x30 - 1, m.offset_0x34 + 2, h), mf);
    let f = g.res.DAT_005e8e40;
    FUN_00455880(param_1, f);
    FUN_00455890(param_1, CRect(0xff, 200, 0, 0xff));
    let mode = g.wfa(d.field_0x0).offset_0x150;
    let title: &[u8] = match mode {
        1 => b"Time Trial",
        4 => b"Challenge",
        _ => b"Choose a Tank",
    };
    vcall!(g, this, w.vfunction63, param_1, 0x19, title);
    FUN_00455890(param_1, FUN_00433320(0xffffff));
    let f = g.res.DAT_005e8cf0;
    FUN_00455880(param_1, f);
    let q: Option<&[u8]> = match mode {
        1 => Some(b"How much money can you earn before time runs out?"),
        4 => Some(b"Can you fend off the increasingly hungry aliens?"),
        _ => None,
    };
    if let Some(q) = q {
        vcall!(g, this, w.vfunction63, param_1, 0x3c, q);
    }
    for i in 0..4 {
        let (x, y) = d.ext_0xa8[i];
        let fr = g.res.DAT_005e8ba4;
        FUN_00455d20(param_1, g, fr, x, y);
        if !g.w(d.offset_0x8[i]).offset_0x2 {
            let k = if 1 < i { i as i32 + 1 } else { i as i32 };
            let img = crate::sexy::res::FUN_005016a0(g, k + 0x62) as Ptr;
            FUN_00455d20(param_1, g, img, x + 0x10, y + 10);
        } else {
            let r = Rect::new(x + 0x19, y + 0x2b, 0xaa, 200);
            let f = g.res.DAT_005e8cf0;
            FUN_00455880(param_1, f);
            FUN_00455890(param_1, FUN_00433320(0xffffff));
            let mode = g.wfa(d.field_0x0).offset_0x150;
            if mode == 1 {
                let s = format!("Complete Tank {}\nin Adventure Mode\nto Unlock this Tank", i + 1).into_bytes();
                vcall!(g, this, w.vfunction65, param_1, r, &s, -1, 0);
            } else if mode == 4 {
                let s = format!("Complete Tank {}\nin Challenge Mode\nto Unlock this Tank", i).into_bytes();
                vcall!(g, this, w.vfunction65, param_1, r, &s, -1, 0);
            }
        }
    }
}

/// port: 0051a710 FUN_0051a710
/// A tank was picked: closes the screen, remembers the tank (+0x888) and starts.
pub fn FUN_0051a710(g: &mut G, this: Ptr, param_1: i32) {
    let app = g.tank_screen(this).field_0x0;
    FUN_0054aed0(g, app);
    g.wfa(app).offset_0x15c = param_1;
    crate::game::win_fish_app::FUN_00552380(g, app, true, true);
}

/// port: 0051b6c0 Sexy::TankScreen::vfunction3_for_ButtonListener
/// `ButtonDepress(int id)`: "Menu" (99) back to the menu; a tank (0..3) starts it;
/// "Stories" (4) opens the story list.
pub fn vfunction3_for_ButtonListener(g: &mut G, this: Ptr, param_1: i32) {
    let app = g.tank_screen(this).field_0x0;
    if param_1 == 99 {
        FUN_0054aed0(g, app);
        crate::game::win_fish_app::FUN_00552100(g, app);
        return;
    }
    if (param_1 as u32) < 4 {
        FUN_0051a710(g, this, param_1 + 1);
        return;
    }
    if param_1 == 4 {
        FUN_0054aed0(g, app);
        crate::game::story_screen::FUN_00552e50(g, app, -1);
    }
}

/// port: 0054aed0 FUN_0054aed0
/// `RemoveTankScreen()` (+0x82c).
pub fn FUN_0054aed0(g: &mut G, this: Ptr) {
    let s = g.wfa(this).offset_0x100;
    if s != NULL {
        let wm = g.sab(this).offset_0x318;
        vcall!(g, wm, w.vfunction5, s);
        crate::sexy::sexy_app_base::vfunction35(g, this, s);
        g.wfa(this).offset_0x100 = NULL;
    }
}

/// port: 0054c130 FUN_0054c130
/// `ShowTankScreen()`: closes the dialogs and any tank screen, opens it full-screen
/// (+0x82c), in front of the board when there is one.
pub fn FUN_0054c130(g: &mut G, this: Ptr) {
    crate::game::win_fish_app::FUN_0054b4c0(g, this);
    FUN_0054aed0(g, this);
    let s = TankScreen(g, this);
    g.wfa(this).offset_0x100 = s;
    let (w, h) = (g.sab(this).field_0xb8, g.sab(this).field_0xbc);
    vcall!(g, s, w.vfunction41, 0, 0, w, h);
    let wm = g.sab(this).offset_0x318;
    vcall!(g, wm, w.vfunction4, s);
    let board = g.wfa(this).offset_0x4;
    if board == NULL {
        vcall!(g, wm, w.vfunction13, s);
        return;
    }
    vcall!(g, wm, w.vfunction15, s, board);
}
