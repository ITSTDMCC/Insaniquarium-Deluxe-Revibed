//! `Sexy::StoryScreen`: one of the 33 character stories (unlocked one per Challenge win,
//! the talk show after buying every bonus item): the title, the animated character, the
//! story, "N of 33", "Back"/"Next" through the unlocked ones and "Back To Main Menu". Plus
//! the app functions that pick the story, open the screen and close it.
//!
//! `StoryScreen_data` starts at object offset 0x8c (after the ButtonListener vftable at
//! 0x88); the object is 0xa8 bytes.

use crate::game::board_parts::{FUN_00500120, FUN_00500180, FUN_00504b60, FUN_005110e0, FUN_00512080};
use crate::game::story_tables::{PTR_s_STINKY_knew_from_an_early_age_th_005e1610, PTR_s_STINKY_the_Snail_005e1698};
use crate::sexy::graphics::{FUN_00455870, FUN_00455880, FUN_00455890, FUN_00455920, FUN_00455cf0, FUN_00455d20, FUN_004560a0, FUN_00456980};
use crate::sexy::prelude::*;
use crate::sexy::types::{CRect, FUN_00433320};

/// `StoryScreen_data` (object offset 0x8c).
#[derive(Debug, Clone, Default)]
pub struct StoryScreen_data {
    /// +0x8c the app.
    pub field_0x0: Ptr,
    /// +0x90 "Back To Main Menu" (id 0).
    pub offset_0x4: Ptr,
    /// +0x94 "Next" (id 2).
    pub offset_0x8: Ptr,
    /// +0x98 "Back" (id 1).
    pub offset_0xc: Ptr,
    /// +0x9c the bubbles.
    pub offset_0x10: Ptr,
    /// +0xa0 the talk show backdrop ("images/talkshow"), loaded for story 32.
    pub offset_0x14: Ptr,
    /// +0xa4 the story shown (0..32).
    pub offset_0x18: i32,
}

impl G {
    pub fn story(&mut self, p: Ptr) -> &mut StoryScreen_data {
        match &mut self.widget(p).ext {
            WExt::StoryScreen(d) => d,
            e => panic!("{p} is not a StoryScreen: {e:?}"),
        }
    }
}

/// port: 0052c360 FUN_0052c360
/// A gray link (white on hover, no underline) with the label (in EBX), sized to it.
pub fn FUN_0052c360(g: &mut G, param_1: i32, param_2: Ptr, label: &[u8]) -> Ptr {
    let b = crate::sexy::button_widget::HyperlinkWidget(g, param_1, param_2);
    g.btn(b).field_0x4 = label.to_vec();
    let f = g.res.DAT_005e8cf0;
    vcall!(g, b, btn.vfunction72, f);
    let h = g.hyperlink(b);
    h.offset_0x0 = FUN_00433320(0x808080);
    h.field_0x10 = FUN_00433320(0xffffff);
    h.offset_0x20 = 0;
    let font = g.btn(b).offset_0x24;
    let w = crate::sexy::image_font::string_width(g, font, label);
    g.wc(b).offset_0x34 = w + 0x14;
    let h = crate::sexy::image_font::get_height(g, font);
    g.wc(b).offset_0x38 = h;
    b
}

/// port: 0052ee80 Sexy::StoryScreen::StoryScreen
/// `StoryScreen(WinFishApp*, int story)`: the story music; bubbles; full screen; "Back To
/// Main Menu" at the bottom right, "Back"/"Next" at the top (hidden with fewer than two
/// stories unlocked); an out-of-range story shows the last one viewed; then moves to an
/// unlocked story.
pub fn StoryScreen(g: &mut G, param_1: Ptr, param_2: i32) -> Ptr {
    let (wc, w) = crate::sexy::widget::Widget();
    let d = StoryScreen_data { field_0x0: param_1, ..Default::default() };
    let this = g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__StoryScreen_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::StoryScreen(Box::new(d)) }),
    });
    crate::game::win_fish_app::FUN_0054b1a0(g, param_1, 2, 5, false);
    g.story(this).offset_0x18 = param_2;
    let bm = crate::game::board_parts::BubbleMgr(g);
    g.story(this).offset_0x10 = bm;
    FUN_00500120(g, bm, &Rect::new(0, -0x14, 0x280, 0x208));
    FUN_00500180(g, bm, 10, 3);
    FUN_00512080(g, bm);
    let wc = g.wc(this);
    wc.offset_0x2c = 0;
    wc.offset_0x30 = 0;
    let (aw, ah) = (g.sab(param_1).field_0xb8, g.sab(param_1).field_0xbc);
    let wc = g.wc(this);
    wc.offset_0x34 = aw;
    wc.offset_0x38 = ah;
    let back = FUN_0052c360(g, 0, this, b"Back To Main Menu");
    g.story(this).offset_0x4 = back;
    let bw = g.wc(back).offset_0x34;
    g.wc(back).offset_0x2c = 0x276 - bw;
    let bh = g.wc(back).offset_0x38;
    g.wc(back).offset_0x30 = 0x1cc - bh / 2;
    let prev = FUN_0052c360(g, 1, this, b"Back");
    g.story(this).offset_0xc = prev;
    let next = FUN_0052c360(g, 2, this, b"Next");
    g.story(this).offset_0x8 = next;
    let pw = g.wc(prev).offset_0x34;
    g.wc(prev).offset_0x2c = 0xdc - pw;
    g.wc(next).offset_0x2c = 0x1a4;
    g.wc(next).offset_0x30 = 0xf;
    g.wc(prev).offset_0x30 = 0xf;
    let profile = g.wfa(param_1).offset_0x18c;
    let mut bits = g.profile(profile).field_0x7c as u32;
    let mut n = 0;
    while bits != 0 && n <= 1 {
        if bits & 1 != 0 {
            n += 1;
        }
        bits >>= 1;
    }
    if n <= 1 {
        g.w(prev).offset_0x0 = false;
        g.w(next).offset_0x0 = false;
    }
    let s = g.story(this).offset_0x18;
    g.story(this).offset_0x14 = NULL;
    if !(0..=0x20).contains(&s) {
        g.story(this).offset_0x18 = g.globals.DAT_005e9088;
    }
    FUN_0052e9d0(g, this, 0);
    FUN_0052c490(g, this);
    this
}

/// port: 0051a290 Sexy::StoryScreen::~StoryScreen
/// Deletes the bubbles, the links and the backdrop; remembers the story (`DAT_005e9088`).
pub fn dtor_StoryScreen(g: &mut G, this: Ptr) {
    let d = g.story(this).clone();
    if d.offset_0x10 != NULL {
        crate::game::board_parts::deleting_destructor__00505420(g, d.offset_0x10, 1);
    }
    for b in [d.offset_0x4, d.offset_0x8, d.offset_0xc] {
        if b != NULL {
            vcall!(g, b, w.vfunction1, 1);
        }
    }
    if d.offset_0x14 != NULL {
        g.free(d.offset_0x14);
    }
    g.globals.DAT_005e9088 = d.offset_0x18;
    crate::sexy::widget::dtor_Widget(g, this);
}

/// port: 0051b660 Sexy::StoryScreen::deleting_destructor_for_Widget
pub fn deleting_destructor_for_Widget(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_StoryScreen(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 0051a360 Sexy::StoryScreen::vfunction21_for_Widget
/// `AddedToManager(WidgetManager*)`: the three links.
pub fn vfunction21_for_Widget(g: &mut G, this: Ptr, param_1: Ptr) {
    crate::sexy::widget_container::vfunction21(g, this, param_1);
    let d = g.story(this).clone();
    for b in [d.offset_0x4, d.offset_0x8, d.offset_0xc] {
        vcall!(g, param_1, w.vfunction4, b);
    }
}

/// port: 0051a3b0 Sexy::StoryScreen::vfunction22_for_Widget
/// `RemovedFromManager(WidgetManager*)`.
pub fn vfunction22_for_Widget(g: &mut G, this: Ptr, param_1: Ptr) {
    crate::sexy::widget_container::vfunction22(g, this, param_1);
    let d = g.story(this).clone();
    for b in [d.offset_0x4, d.offset_0x8, d.offset_0xc] {
        vcall!(g, param_1, w.vfunction5, b);
    }
}

/// port: 0051a400 Sexy::StoryScreen::vfunction23_for_Widget
/// `Update()`: while the app is active (+0x4d2), the bubbles and a redraw.
pub fn vfunction23_for_Widget(g: &mut G, this: Ptr) {
    crate::sexy::widget_container::vfunction23(g, this);
    let app = g.story(this).field_0x0;
    if g.sab(app).field_0x4ca {
        let bm = g.story(this).offset_0x10;
        FUN_005110e0(g, bm);
        vcall!(g, this, w.vfunction18);
    }
}

/// port: 0051a430 Sexy::StoryScreen::vfunction55_for_Widget
/// `MouseDown(int x, int y, int clicks)`: `Widget::MouseDown`. (The linker folded this
/// body into the vtables of several screens.)
pub fn vfunction55_for_Widget(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32) {
    crate::sexy::widget::vfunction55(g, this, param_1, param_2, param_3);
}

/// port: 0052c490 FUN_0052c490
/// Story 32 (the talk show) loads its backdrop once.
pub fn FUN_0052c490(g: &mut G, this: Ptr) {
    let d = g.story(this).clone();
    if d.offset_0x18 == 0x20 && d.offset_0x14 == NULL {
        let img = crate::sexy::sexy_app_base::vfunction66(g, d.field_0x0, "images/talkshow", true);
        g.story(this).offset_0x14 = img;
    }
}

/// Whether story `s` is unlocked: its profile bit (+0x7c), or for 32 the talk show (+0x80).
fn story_unlocked(p: &crate::game::profile::PlayerProfile, s: i32) -> bool {
    if s < 0x20 {
        (p.field_0x7c as u32) & (1u32 << (s & 0x1f)) != 0
    } else {
        p.field_0x80 == 1
    }
}

/// port: 0052e9d0 FUN_0052e9d0
/// Steps through the stories (-1 back, +1 forward; 0 keeps an unlocked one, else goes
/// forward) to the next unlocked one, wrapping over all 33; stays put when none is.
pub fn FUN_0052e9d0(g: &mut G, this: Ptr, param_1: i32) {
    let app = g.story(this).field_0x0;
    let profile = g.wfa(app).offset_0x18c;
    let mut dir = param_1;
    if param_1 == 0 {
        let s = g.story(this).offset_0x18;
        if (g.profile(profile).field_0x7c as u32) & (1u32 << (s & 0x1f)) != 0 {
            return;
        }
        dir = 1;
    }
    let p = g.profile(profile).clone();
    let mut s = g.story(this).offset_0x18;
    let mut tries = 0;
    loop {
        s = (dir + 0x21 + s) % 0x21;
        if story_unlocked(&p, s) {
            break;
        }
        tries += 1;
        if 0x21 <= tries {
            break;
        }
    }
    if tries != 0x21 {
        g.story(this).offset_0x18 = s;
    }
    FUN_0052c490(g, this);
}

/// port: 0052eaf0 Sexy::StoryScreen::vfunction3_for_ButtonListener
/// `ButtonDepress(int id)`: "Back To Main Menu" (0) stops the music, closes and goes to
/// the menu; "Back" (1) / "Next" (2) step through the stories.
pub fn vfunction3_for_ButtonListener(g: &mut G, this: Ptr, param_1: i32) {
    let app = g.story(this).field_0x0;
    if param_1 == 0 {
        crate::game::win_fish_app::FUN_0054b020(g, app);
        FUN_0054adb0(g, app);
        crate::game::win_fish_app::FUN_00552100(g, app);
        return;
    }
    if param_1 == 1 || param_1 == 2 {
        FUN_0052e9d0(g, this, if param_1 != 1 { 1 } else { -1 });
    }
}

/// port: 0052c530 Sexy::StoryScreen::vfunction27_for_Widget
/// `Draw(Graphics*)`: the talk show backdrop with white titles, or blue water with the
/// bubbles and cyan titles; the character animated in the middle (the alien mob as a row of
/// aliens; the talk show host's portrait left out when the backdrop is up); the story;
/// black bars top and bottom with "N of 33".
pub fn vfunction27_for_Widget(g: &mut G, this: Ptr, param_1: &mut Graphics) {
    let d = g.story(this).clone();
    let s = d.offset_0x18;
    let name = PTR_s_STINKY_the_Snail_005e1698[s as usize];
    if s == 0x20 && d.offset_0x14 != NULL {
        FUN_00455d20(param_1, g, d.offset_0x14, 0, 0);
        let f = g.res.DAT_005e8e40;
        FUN_00455880(param_1, f);
        FUN_00455890(param_1, FUN_00433320(0xffffff));
        vcall!(g, this, w.vfunction63, param_1, 0x50, b"The Story Of");
        let f = g.res.DAT_005e8e18;
        FUN_00455880(param_1, f);
        FUN_00455890(param_1, FUN_00433320(0xffffff));
        vcall!(g, this, w.vfunction63, param_1, 0x6e, name);
    } else {
        FUN_00455890(param_1, CRect(0x19, 0x69, 0xc3, 0xff));
        let (w, h) = (g.wc(this).offset_0x34, g.wc(this).offset_0x38);
        FUN_00455920(param_1, 0, 0, w, h);
        FUN_00504b60(g, d.offset_0x10, param_1);
        let f = g.res.DAT_005e8a9c;
        FUN_00455880(param_1, f);
        FUN_00455890(param_1, CRect(0xd7, 0xff, 0xff, 0xff));
        vcall!(g, this, w.vfunction63, param_1, 0x50, b"The Story Of");
        let f = g.res.DAT_005e8bbc;
        FUN_00455880(param_1, f);
        FUN_00455890(param_1, CRect(0x96, 0xfa, 0xfa, 0xff));
        vcall!(g, this, w.vfunction63, param_1, 0x6e, name);
    }
    let id = if s < 0x18 { s + 0xca } else { s + 0xa5 };
    let img = crate::sexy::res::FUN_005016a0(g, id) as Ptr;
    let x = 0x140 - crate::sexy::graphics::FUN_004574b0(g, img) / 2;
    let y = 0xbe - crate::sexy::graphics::FUN_004574a0(g, img) / 2;
    let t = g.wc(this).offset_0x24;
    let cel = crate::sexy::image::FUN_004578a0(g, img, t);
    let row = (s == 7) as i32;
    let mut r = Rect::new(0x32, 0x118, 0x21c, 0);
    let f = g.res.DAT_005e8b04;
    FUN_00455880(param_1, f);
    if img == g.res.DAT_005e8f00 {
        let n = t % 0x14;
        let c = n / 2;
        let pp = if 9 < n { 0x13 - n } else { n };
        let src = crate::sexy::image::FUN_00457500(g, img, c, 5);
        FUN_004560a0(param_1, g, img, 0x18b, 0x87, &src, true);
        FUN_00456980(param_1, g, img, 0x168, 0x91, 2, 7);
        FUN_00456980(param_1, g, img, 0x14a, 0x8c, 2, 6);
        FUN_00456980(param_1, g, img, 300, 0x91, 2, 6);
        FUN_00456980(param_1, g, img, 0x10e, 0x9b, 2, 7);
        FUN_00456980(param_1, g, img, 0xf0, 0xa0, 2, 6);
        FUN_00456980(param_1, g, img, 0xd2, 0x9b, 2, 7);
        FUN_00456980(param_1, g, img, 0xb4, 0x96, pp, 2);
    } else if img == g.res.DAT_005e8cb8 && d.offset_0x14 != NULL {
        let f = g.res.DAT_005e8cd0;
        FUN_00455880(param_1, f);
        r.mY = 0x14a;
    } else {
        FUN_00456980(param_1, g, img, x, y, cel, row);
    }
    FUN_00455890(param_1, FUN_00433320(0xffffff));
    let story = PTR_s_STINKY_knew_from_an_early_age_th_005e1610[s as usize];
    vcall!(g, this, w.vfunction65, param_1, r, story, -1, -1);
    FUN_00455890(param_1, FUN_00433320(0));
    let (w, h) = (g.wc(this).offset_0x34, g.wc(this).offset_0x38);
    FUN_00455920(param_1, 0, 0, w, 0x28);
    FUN_00455920(param_1, 0, h - 0x28, w, 0x28);
    FUN_00455890(param_1, FUN_00433320(0xffffff));
    let f = g.res.DAT_005e8ce4;
    FUN_00455880(param_1, f);
    let label = format!("{} of 33", s + 1).into_bytes();
    let fnt = FUN_00455870(param_1);
    let lw = crate::sexy::image_font::string_width(g, fnt, &label);
    FUN_00455cf0(param_1, g, &label, 0x140 - lw / 2, 0x1c);
}

/// port: 0054adb0 FUN_0054adb0
/// `RemoveStoryScreen()` (+0x820).
pub fn FUN_0054adb0(g: &mut G, this: Ptr) {
    let s = g.wfa(this).offset_0xf4;
    if s != NULL {
        let wm = g.sab(this).offset_0x318;
        vcall!(g, wm, w.vfunction5, s);
        crate::sexy::sexy_app_base::vfunction35(g, this, s);
        g.wfa(this).offset_0xf4 = NULL;
    }
}

/// The eight stories each Challenge tank unlocks, in order (`FUN_00552e50`'s table).
const TANK_STORIES: [i32; 32] = [
    0, 1, 2, 3, 4, 0x14, 0x18, 0x19, //
    5, 6, 7, 8, 9, 0x15, 0x1a, 0x1b, //
    10, 0xb, 0xc, 0xd, 0xe, 0x16, 0x1c, 0x1d, //
    0xf, 0x10, 0x11, 0x12, 0x13, 0x17, 0x1e, 0x1f,
];

/// port: 00552e50 FUN_00552e50
/// `ShowStory(int tank)`: closes the dialogs and any story screen, then picks the story:
/// none for a tank outside 1..4 (the screen shows the last one viewed); with every story
/// unlocked, the talk show the first time after all six bonus purchases, else any (the
/// talk show only once unlocked); otherwise the tank's next locked story (a bonus pet's
/// only once the pet is owned), unlocked and saved, or once all eight are unlocked a random
/// one of them. Opens the screen (+0x820) in front.
pub fn FUN_00552e50(g: &mut G, this: Ptr, param_1: i32) {
    crate::game::win_fish_app::FUN_0054b4c0(g, this);
    FUN_0054adb0(g, this);
    let mut tank = param_1 - 1;
    let mut in_range = false;
    if tank < 0 {
        tank = 0;
    } else if tank < 4 {
        in_range = true;
    } else {
        tank = 3;
    }
    let story = 'pick: {
        if !in_range {
            break 'pick -2;
        }
        let profile = g.wfa(this).offset_0x18c;
        if g.profile(profile).field_0x7c == -1 {
            let talk = g.profile(profile).field_0x80;
            if talk == 0 && 5 < g.profile(profile).field_0xb0 {
                g.profile(profile).field_0x80 = 1;
                crate::game::win_fish_app::FUN_0054afd0(g, this);
                break 'pick 0x20;
            }
            let r = crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32;
            break 'pick r % (0x21 - (talk != 1) as i32);
        }
        let base = tank as usize * 8;
        let mut found = None;
        for k in 0..8 {
            let s = TANK_STORIES[base + k];
            let bits = g.profile(profile).field_0x7c as u32;
            if bits & (1u32 << (s & 0x1f)) == 0 && ((s - 0x14) as u32 > 3 || crate::game::profile::FUN_00501410(g, profile, s as usize)) {
                found = Some(s);
                break;
            }
        }
        if let Some(s) = found {
            g.profile(profile).field_0x7c |= 1 << (s & 0x1f);
            crate::game::win_fish_app::FUN_0054afd0(g, this);
            break 'pick s;
        }
        let mut v: Vec<i32> = Vec::new();
        for k in 0..8 {
            let s = TANK_STORIES[base + k];
            if (g.profile(profile).field_0x7c as u32) & (1u32 << (s & 0x1f)) != 0 {
                v.push(s);
            }
        }
        if v.is_empty() {
            v.push(0);
        }
        let r = crate::sexy::sexy_app_base::FUN_0040f9e0(g);
        v[(r % v.len() as u32) as usize]
    };
    let s = StoryScreen(g, this, story);
    g.wfa(this).offset_0xf4 = s;
    let (w, h) = (g.sab(this).field_0xb8, g.sab(this).field_0xbc);
    vcall!(g, s, w.vfunction41, 0, 0, w, h);
    let wm = g.sab(this).offset_0x318;
    vcall!(g, wm, w.vfunction4, s);
    vcall!(g, wm, w.vfunction9, s);
}
