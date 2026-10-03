//! The Board's helper objects: `StarField` (the space background's stars), `BubbleMgr` (the
//! tank's rising bubbles), `MyLabelWidget` (the money counter), `MessageWidget` (the
//! message line at the bottom of the tank) and `BoardOverlay` (two full-screen widgets that
//! draw the board's overlay layers above the game objects).

use crate::sexy::graphics::{FUN_00455880, FUN_00455890, FUN_00455cf0};
use crate::sexy::image_font as font;
use crate::sexy::prelude::*;

/// One star of the star field (a `std::list` node's value).
#[derive(Debug, Clone, Default)]
pub struct Star {
    /// +0x08 x.
    pub field_0x8: f32,
    /// +0x0c y.
    pub field_0xc: f32,
    /// +0x10 speed (negative: the stars drift left).
    pub field_0x10: f32,
    /// +0x14.
    pub field_0x14: f32,
    /// +0x18 color (0xRRGGBB).
    pub field_0x18: u32,
}

/// `StarField_data` (object offset 0x4).
#[derive(Debug, Clone, Default)]
pub struct StarField_data {
    /// +0x08 the stars (`std::list`; size at +0x0c is `len()`).
    pub offset_0x4: Vec<Star>,
    /// +0x10 the number of stars asked for.
    pub offset_0xc: i32,
    /// +0x14 IMAGE nebula1 (loaded on first use).
    pub offset_0x10: Ptr,
}

/// One rising bubble (a value of the `std::list` at BubbleMgr +0x44).
#[derive(Debug, Clone, Default)]
pub struct Bubble {
    /// +0x0 cel of IMAGE bubbles (0..2).
    pub field_0x0: i32,
    /// +0x4 sideways wobble (0 or 1 pixel, re-rolled every update).
    pub field_0x4: i32,
    /// +0x8 x.
    pub field_0x8: f32,
    /// +0xc y.
    pub field_0xc: f32,
    /// +0x10 rising speed.
    pub field_0x10: f32,
}

/// One fish swimming across the background (a value of the `std::list` at BubbleMgr +0x50).
#[derive(Debug, Clone, Default)]
pub struct BgFish {
    /// +0x00 kind: the row of the 80x80 fish strips (0..2).
    pub field_0x0: i32,
    /// +0x04 animation counter.
    pub field_0x4: i32,
    /// +0x08 updates left of the turn animation.
    pub field_0x8: i32,
    /// +0x0c x.
    pub field_0xc: f32,
    /// +0x10 y.
    pub field_0x10: f32,
    /// +0x14 x speed.
    pub field_0x14: f32,
    /// +0x18 y speed.
    pub field_0x18: f32,
    /// +0x1c color of the third layer.
    pub field_0x1c: Color,
    /// +0x2c color of the second layer.
    pub field_0x2c: Color,
}

/// `BubbleMgr_data` (object offset 0x4): bubbles rising in area A (+0x04) and fish
/// crossing area B (+0x14), each list with a cap and a per-update spawn chance in percent.
#[derive(Debug, Clone, Default)]
pub struct BubbleMgr_data {
    /// +0x04 bubble area left.
    pub offset_0x0: i32,
    /// +0x08 bubble area top.
    pub offset_0x4: i32,
    /// +0x0c bubble area width.
    pub offset_0x8: i32,
    /// +0x10 bubble area height.
    pub offset_0xc: i32,
    /// +0x14 fish area left.
    pub offset_0x10: i32,
    /// +0x18 fish area top.
    pub offset_0x14: i32,
    /// +0x1c fish area width.
    pub offset_0x18: i32,
    /// +0x20 fish area height.
    pub offset_0x1c: i32,
    /// +0x24 most bubbles at once.
    pub offset_0x20: i32,
    /// +0x28 bubble spawn chance (percent per update).
    pub offset_0x24: i32,
    /// +0x30 most fish at once.
    pub offset_0x2c: i32,
    /// +0x34 fish spawn chance (percent per update).
    pub offset_0x30: i32,
    /// +0x38 y speed given to new fish.
    pub offset_0x34: i32,
    /// +0x3c clip the fish to their area when drawing.
    pub offset_0x38: i32,
    /// +0x44 the bubbles (size at +0x48 is `len()`).
    pub offset_0x40: Vec<Bubble>,
    /// +0x50 the fish (size at +0x54 is `len()`).
    pub offset_0x4c: Vec<BgFish>,
}

/// `MyLabelWidget_data` (object offset 0x88).
#[derive(Debug, Clone, Default)]
pub struct MyLabelWidget_data {
    /// +0x88 the text.
    pub field_0x0: Vec<u8>,
    /// +0xa4 color.
    pub offset_0x1c: Color,
    /// +0xb4 justification: 0 left, 1 center, else right.
    pub offset_0x2c: i32,
    /// +0xb8 font.
    pub offset_0x30: Ptr,
}

/// `MessageWidget_data` (object offset 0x88).
#[derive(Debug, Clone, Default)]
pub struct MessageWidget_data {
    /// +0x88 the app.
    pub offset_0x0: Ptr,
    /// +0x8c the message.
    pub field_0x4: Vec<u8>,
    /// +0xa8 text color.
    pub offset_0x20: Color,
    /// +0xb8 outline color.
    pub offset_0x30: Color,
    /// +0xc8.
    pub offset_0x40: i32,
    /// +0xcc font.
    pub offset_0x44: Ptr,
    /// +0xd0 updates left to show it.
    pub offset_0x48: i32,
    /// +0xd4 blink.
    pub offset_0x4c: bool,
    /// +0xd8.
    pub offset_0x50: i32,
}

/// `BoardOverlay_data` (object offset 0x88).
#[derive(Debug, Clone, Default)]
pub struct BoardOverlay_data {
    /// +0x88 the board.
    pub offset_0x0: Ptr,
    /// +0x8c the overlay layer passed to the board's `DrawOverlay(g, layer)`.
    pub offset_0x4: i32,
}

impl G {
    pub fn star_field(&mut self, p: Ptr) -> &mut StarField_data {
        match &mut self.obj(p).node {
            Node::StarField(s) => s,
            n => panic!("{p} is not a StarField: {n:?}"),
        }
    }
    pub fn bubble_mgr(&mut self, p: Ptr) -> &mut BubbleMgr_data {
        match &mut self.obj(p).node {
            Node::BubbleMgr(s) => s,
            n => panic!("{p} is not a BubbleMgr: {n:?}"),
        }
    }
    pub fn my_label(&mut self, p: Ptr) -> &mut MyLabelWidget_data {
        match &mut self.widget(p).ext {
            WExt::MyLabel(d) => d,
            e => panic!("{p} is not a MyLabelWidget: {e:?}"),
        }
    }
    pub fn message_widget(&mut self, p: Ptr) -> &mut MessageWidget_data {
        match &mut self.widget(p).ext {
            WExt::Message(d) => d,
            e => panic!("{p} is not a MessageWidget: {e:?}"),
        }
    }
    pub fn board_overlay(&mut self, p: Ptr) -> &mut BoardOverlay_data {
        match &mut self.widget(p).ext {
            WExt::BoardOverlay(d) => d,
            e => panic!("{p} is not a BoardOverlay: {e:?}"),
        }
    }
}

/// port: 00505450 Sexy::StarField::StarField
pub fn StarField(g: &mut G) -> Ptr {
    g.alloc(Obj { vt: None, node: Node::StarField(Box::new(StarField_data::default())) })
}

/// port: 00504ec0 Sexy::StarField::~StarField
/// Releases the nebula image (+0x14) and the stars.
pub fn dtor_StarField(g: &mut G, this: Ptr) {
    let neb = g.star_field(this).offset_0x10;
    if neb != NULL {
        g.free(neb);
        g.star_field(this).offset_0x10 = NULL;
    }
    g.star_field(this).offset_0x4.clear();
}

/// port: 00505480 Sexy::StarField::deleting_destructor
pub fn deleting_destructor__00505480(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_StarField(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 00511580 FUN_00511580
/// `StarField::Init(int count)`: loads `images/nebula1` the first time (app vtable +0x104
/// `GetImage(path, commitBits)`), then scatters `count` stars over the screen.
pub fn FUN_00511580(g: &mut G, this: Ptr, param_1: i32) {
    g.star_field(this).offset_0xc = param_1;
    if g.star_field(this).offset_0x10 == NULL {
        let img = crate::sexy::sexy_app_base::vfunction66(g, g.globals.DAT_005eb6a4, "images/nebula1", true);
        g.star_field(this).offset_0x10 = img;
    }
    g.star_field(this).offset_0x4.clear();
    let mut n = param_1;
    while 0 < n {
        let y = (crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32) % 0x1e0;
        let x = (crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32) % 0x280;
        FUN_005114a0(g, this, x, y);
        n -= 1;
    }
}

/// port: 005114a0 FUN_005114a0
/// `StarField::AddStar(int x, int y)`: one of three depths (CRT `rand() % 3`) giving its
/// gray level and drift speed.
pub fn FUN_005114a0(g: &mut G, this: Ptr, param_1: i32, param_2: i32) -> i32 {
    let mut s = Star { field_0xc: param_2 as f32, field_0x8: param_1 as f32, field_0x14: 0.0, ..Default::default() };
    let r = crate::sexy::crt::rand(g);
    let q = r / 3;
    match r % 3 {
        0 => {
            s.field_0x18 = 0x404040;
            s.field_0x10 = f32::from_bits(0xbf19999a);
        }
        1 => {
            s.field_0x18 = 0x909090;
            s.field_0x10 = f32::from_bits(0xbfe66667);
        }
        _ => {
            s.field_0x18 = 0xffffff;
            s.field_0x10 = f32::from_bits(0xc02ccccc);
        }
    }
    g.star_field(this).offset_0x4.push(s);
    q
}

/// port: 00511680 FUN_00511680
/// `StarField::Update()`: every star drifts by its speed; those past the left edge
/// (x <= 0) are dropped, and new ones come in at the right edge (x 640, random y) until
/// there are as many as asked for.
pub fn FUN_00511680(g: &mut G, this: Ptr) {
    let sf = g.star_field(this);
    let mut i = 0;
    while i < sf.offset_0x4.len() {
        let s = &mut sf.offset_0x4[i];
        let x = s.field_0x10 + s.field_0x8;
        s.field_0x8 = x;
        s.field_0xc = s.field_0x14 + s.field_0xc;
        if 0.0 < x {
            i += 1;
        } else {
            sf.offset_0x4.remove(i);
        }
    }
    while (g.star_field(this).offset_0x4.len() as i32) < g.star_field(this).offset_0xc {
        let y = crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32 % 0x1e0;
        FUN_005114a0(g, this, 0x280, y);
    }
}

/// port: 00504f20 FUN_00504f20
/// `StarField::Draw(Graphics*, bool wavy)`: the nebula tiled three times across, scrolling
/// left half a pixel per board update (waving when asked), or black without it; then each
/// star as one pixel of its colour.
pub fn FUN_00504f20(g: &mut G, this: Ptr, param_1: &mut Graphics, param_2: bool) {
    use crate::sexy::graphics::{FUN_00455890, FUN_00455920, FUN_00455d20};
    use crate::sexy::types::FUN_00433320;
    let neb = g.star_field(this).offset_0x10;
    if neb == NULL {
        FUN_00455890(param_1, FUN_00433320(0));
        FUN_00455920(param_1, 0, 0, 0x280, 0x1e0);
    } else {
        let app = g.globals.DAT_005eb6a4;
        let w = g.image(neb).offset_0x20;
        let board = g.wfa(app).offset_0x4;
        let t = g.board(board).ext_0x444;
        let x = (w - (t / 2) % w) - 1;
        FUN_00455d20(param_1, g, neb, x - w, 0);
        FUN_00455d20(param_1, g, neb, x, 0);
        let w = g.image(neb).offset_0x20;
        FUN_00455d20(param_1, g, neb, w + x, 0);
        if param_2 {
            let amp = f32::from_bits(0x41000000);
            let w = g.image(neb).offset_0x20;
            crate::game::board_draw::FUN_00538cf0(g, board, param_1, neb, x - w, 0, amp);
            crate::game::board_draw::FUN_00538cf0(g, board, param_1, neb, x, 0, amp);
            let w = g.image(neb).offset_0x20;
            crate::game::board_draw::FUN_00538cf0(g, board, param_1, neb, w + x, 0, amp);
        }
    }
    let stars = g.star_field(this).offset_0x4.clone();
    for s in &stars {
        FUN_00455890(param_1, FUN_00433320(s.field_0x18));
        let y = crate::sexy::crt::ftol(s.field_0xc as f64) as i32;
        let x = crate::sexy::crt::ftol(s.field_0x8 as f64) as i32;
        FUN_00455920(param_1, x, y, 1, 1);
    }
}

/// port: 00504b00 FUN_00504b00
/// `BubbleMgr::SetFishSpeedY(float)`: the y speed for new fish (+0x38), given to every fish
/// already swimming too (when it changes).
pub fn FUN_00504b00(g: &mut G, this: Ptr, param_1: f32) {
    let d = g.bubble_mgr(this);
    if f32::from_bits(d.offset_0x34 as u32) == param_1 {
        return;
    }
    d.offset_0x34 = param_1.to_bits() as i32;
    for f in d.offset_0x4c.iter_mut() {
        f.field_0x18 = param_1;
    }
}

/// port: 00505350 Sexy::BubbleMgr::BubbleMgr
pub fn BubbleMgr(g: &mut G) -> Ptr {
    let d = BubbleMgr_data {
        offset_0x34: 0,
        offset_0x10: 0,
        offset_0x0: 0,
        offset_0x14: 0,
        offset_0x18: 0x280,
        offset_0x4: 0,
        offset_0x8: 0x280,
        offset_0x20: 10,
        offset_0x24: 10,
        offset_0x30: 10,
        offset_0x1c: 0x1e0,
        offset_0xc: 0x1e0,
        offset_0x2c: 0,
        offset_0x38: 1,
        ..Default::default()
    };
    g.alloc(Obj { vt: None, node: Node::BubbleMgr(Box::new(d)) })
}

/// port: 00504a00 Sexy::BubbleMgr::~BubbleMgr
/// Empties both lists.
pub fn dtor_BubbleMgr(g: &mut G, this: Ptr) {
    let d = g.bubble_mgr(this);
    d.offset_0x4c.clear();
    d.offset_0x40.clear();
}

/// port: 00505420 Sexy::BubbleMgr::deleting_destructor
pub fn deleting_destructor__00505420(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_BubbleMgr(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 00500120 FUN_00500120
/// `BubbleMgr::SetArea(const Rect&)` (+0x4..+0x10 as left, top, width, height).
pub fn FUN_00500120(g: &mut G, this: Ptr, param_1: &Rect) {
    let d = g.bubble_mgr(this);
    d.offset_0x0 = param_1.mX;
    d.offset_0x4 = param_1.mY;
    d.offset_0x8 = param_1.mWidth;
    d.offset_0xc = param_1.mHeight;
}

/// port: 00500180 FUN_00500180
/// `SetBubbles(int max, int chance)` (+0x24/+0x28).
pub fn FUN_00500180(g: &mut G, this: Ptr, param_1: i32, param_2: i32) {
    let d = g.bubble_mgr(this);
    d.offset_0x20 = param_1;
    d.offset_0x24 = param_2;
}

/// port: 00534600 Sexy::MyLabelWidget::MyLabelWidget
pub fn MyLabelWidget(g: &mut G) -> Ptr {
    let (wc, w) = crate::sexy::widget::Widget();
    let d = MyLabelWidget_data {
        field_0x0: Vec::new(),
        offset_0x2c: 0,
        offset_0x1c: CRect(0x6e, 0xfa, 0x6e, 0xff),
        offset_0x30: g.res.DAT_005e8cd0,
    };
    g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__MyLabelWidget_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::MyLabel(Box::new(d)) }),
    })
}

/// port: 00537390 Sexy::ToolTipWidget::deleting_destructor
/// The deleting destructor MyLabelWidget shares with ToolTipWidget (identical code).
pub fn deleting_destructor__00537390(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_MyLabelWidget(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 00530b20 Sexy::MyLabelWidget::vfunction27
/// `Draw(Graphics*)`: the text, vertically centered, left/centered/right aligned.
pub fn vfunction27__00530b20(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    let d = g.my_label(this).clone();
    FUN_00455890(gfx, d.offset_0x1c);
    FUN_00455880(gfx, d.offset_0x30);
    let fh = font::get_height(g, d.offset_0x30);
    let h = g.wc(this).offset_0x38;
    let asc = font::get_ascent(g, d.offset_0x30);
    let x = match d.offset_0x2c {
        0 => 0,
        1 => (g.wc(this).offset_0x34 - font::string_width(g, d.offset_0x30, &d.field_0x0)) / 2,
        _ => g.wc(this).offset_0x34 - font::string_width(g, d.offset_0x30, &d.field_0x0),
    };
    FUN_00455cf0(gfx, g, &d.field_0x0, x, asc + (h - fh) / 2);
}

/// port: 00534490 Sexy::MessageWidget::MessageWidget
/// `MessageWidget(WinFishApp*, std::string theText)`: the line at (90, 445), 460 wide.
pub fn MessageWidget(g: &mut G, param_1: Ptr, param_2: &[u8]) -> Ptr {
    let (wc, mut w) = crate::sexy::widget::Widget();
    w.offset_0x1 = false;
    let f = g.res.DAT_005e8d1c;
    let d = MessageWidget_data {
        offset_0x0: param_1,
        field_0x4: param_2.to_vec(),
        offset_0x20: CRect(0xb4, 0xfa, 0x5a, 0xff),
        offset_0x30: CRect(0, 0x4b, 0, 0xff),
        offset_0x40: 0,
        offset_0x44: f,
        offset_0x48: 0xb9,
        offset_0x4c: false,
        offset_0x50: -1,
    };
    let this = g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__MessageWidget_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::Message(Box::new(d)) }),
    });
    let fh = font::get_height(g, f);
    let c = g.wc(this);
    c.offset_0x2c = 0x5a;
    c.offset_0x30 = 0x1bd;
    c.offset_0x38 = fh + 6;
    c.offset_0x34 = 0x1cc;
    this
}

/// port: 00533150 Sexy::MyLabelWidget::~MyLabelWidget
/// Frees the text, then the Widget part.
pub fn dtor_MyLabelWidget(g: &mut G, this: Ptr) {
    if let WExt::MyLabel(d) = &mut g.widget(this).ext {
        d.field_0x0.clear();
    }
    crate::sexy::widget::dtor_Widget(g, this);
}

/// port: 00533100 Sexy::MessageWidget::~MessageWidget
/// Frees the message, then the Widget part.
pub fn dtor_MessageWidget(g: &mut G, this: Ptr) {
    g.message_widget(this).field_0x4.clear();
    crate::sexy::widget::dtor_Widget(g, this);
}

/// port: 00533360 Sexy::MessageWidget::deleting_destructor
pub fn deleting_destructor__00533360(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_MessageWidget(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 00530800 Sexy::MessageWidget::vfunction23
/// `Update()`: counts the message's time down while the board runs unpaused.
pub fn vfunction23__00530800(g: &mut G, this: Ptr) {
    let app = g.message_widget(this).offset_0x0;
    let board = g.wfa(app).offset_0x4;
    if board != NULL && !g.board(board).field_0x8 {
        let t = g.message_widget(this).offset_0x48;
        if 0 < t {
            g.message_widget(this).offset_0x48 = t - 1;
        }
    }
}

/// port: 00530830 Sexy::MessageWidget::vfunction27
/// `Draw(Graphics*)`: while shown, the text centered, outline color in its font then the text
/// color in `DAT_005e8b04`; blinking hides it for 5 of every 32 updates.
pub fn vfunction27__00530830(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    let d = g.message_widget(this).clone();
    if 0 < d.offset_0x48 {
        let fh = font::get_height(g, d.offset_0x44);
        let h = g.wc(this).offset_0x38;
        let asc = font::get_ascent(g, d.offset_0x44);
        let y = asc + (h - fh) / 2;
        let sw = font::string_width(g, d.offset_0x44, &d.field_0x4);
        let x = (g.wc(this).offset_0x34 - sw) / 2 + 2;
        // `t % 32` (signed, as compiled).
        let r = d.offset_0x48 % 32;
        if r < 0x1b || !d.offset_0x4c {
            FUN_00455890(gfx, d.offset_0x30);
            FUN_00455880(gfx, d.offset_0x44);
            FUN_00455cf0(gfx, g, &d.field_0x4, x, y);
            FUN_00455890(gfx, d.offset_0x20);
            let f2 = g.res.DAT_005e8b04;
            FUN_00455880(gfx, f2);
            FUN_00455cf0(gfx, g, &d.field_0x4, x, y);
        }
    }
}

/// port: 00537500 BoardOverlay::BoardOverlay
/// `BoardOverlay(Board*, int layer)`: a 640x480 transparent widget that ignores the mouse.
pub fn BoardOverlay(g: &mut G, param_1: Ptr, param_2: i32) -> Ptr {
    let (mut wc, mut w) = crate::sexy::widget::Widget();
    wc.offset_0x3c = true;
    w.offset_0x1 = false;
    wc.offset_0x34 = 0x280;
    wc.offset_0x38 = 0x1e0;
    let d = BoardOverlay_data { offset_0x0: param_1, offset_0x4: param_2 };
    g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::BoardOverlay_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::BoardOverlay(Box::new(d)) }),
    })
}

/// port: 00537540 BoardOverlay::vfunction27
/// `Draw(Graphics*)`: the board's `DrawOverlay(g, layer)` (vtable +0xac).
pub fn vfunction27__00537540(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    let d = g.board_overlay(this).clone();
    vcall!(g, d.offset_0x0, w.vfunction44, gfx, d.offset_0x4);
}

/// port: 00500150 FUN_00500150
/// `BubbleMgr::SetFishArea(const Rect&)` (+0x14..+0x20).
pub fn FUN_00500150(g: &mut G, this: Ptr, param_1: &Rect) {
    let d = g.bubble_mgr(this);
    d.offset_0x10 = param_1.mX;
    d.offset_0x14 = param_1.mY;
    d.offset_0x18 = param_1.mWidth;
    d.offset_0x1c = param_1.mHeight;
}

/// port: 005001a0 FUN_005001a0
/// `SetFish(int max, int chance)` (+0x30/+0x34).
pub fn FUN_005001a0(g: &mut G, this: Ptr, param_1: i32, param_2: i32) {
    let d = g.bubble_mgr(this);
    d.offset_0x2c = param_1;
    d.offset_0x30 = param_2;
}

/// port: 00512080 FUN_00512080
/// Runs 500 updates so the screen starts full.
pub fn FUN_00512080(g: &mut G, this: Ptr) {
    let mut n = 500;
    loop {
        FUN_005110e0(g, this);
        n -= 1;
        if n == 0 {
            break;
        }
    }
}

/// `Sexy::Rand()` as the C code uses it (an `int`).
fn rand_i(g: &mut G) -> i32 {
    crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32
}

/// port: 005110e0 FUN_005110e0
/// `BubbleMgr::Update()`: moves the bubbles (dropping those past the top), maybe spawns
/// one, moves the fish (dropping those 80 pixels past the side they swim to), maybe spawns
/// one. The original returns whatever was last in EAX (unused by every caller).
pub fn FUN_005110e0(g: &mut G, this: Ptr) {
    let mut i = 0;
    while i < g.bubble_mgr(this).offset_0x40.len() {
        let mut b = g.bubble_mgr(this).offset_0x40[i].clone();
        FUN_004ffec0(g, &mut b);
        let d = g.bubble_mgr(this);
        d.offset_0x40[i] = b.clone();
        if b.field_0xc <= d.offset_0x4 as f32 {
            d.offset_0x40.remove(i);
        } else {
            i += 1;
        }
    }
    let (n, max) = (g.bubble_mgr(this).offset_0x40.len() as i32, g.bubble_mgr(this).offset_0x20);
    if n < max && rand_i(g) % 100 < g.bubble_mgr(this).offset_0x24 {
        FUN_00510fe0(g, this);
    }
    let d = g.bubble_mgr(this);
    let mut i = 0;
    while i < d.offset_0x4c.len() {
        let f = &mut d.offset_0x4c[i];
        FUN_005000f0(f);
        let (x, vx) = (f.field_0xc, f.field_0x14);
        if vx < 0.0 && x <= (d.offset_0x10 - 0x50) as f32 {
            d.offset_0x4c.remove(i);
        } else if 0.0 < vx && (d.offset_0x18 + d.offset_0x10) as f32 <= x {
            d.offset_0x4c.remove(i);
        } else {
            i += 1;
        }
    }
    let (n, max) = (d.offset_0x4c.len() as i32, d.offset_0x2c);
    if n < max && rand_i(g) % 100 < g.bubble_mgr(this).offset_0x30 {
        FUN_00511010(g, this);
    }
}

/// port: 00510fe0 FUN_00510fe0
/// Spawns a bubble at a random x along the bottom of the bubble area (wider than 20 only).
pub fn FUN_00510fe0(g: &mut G, this: Ptr) {
    let w = g.bubble_mgr(this).offset_0x8;
    if 0x14 < w {
        let r = rand_i(g);
        let d = g.bubble_mgr(this);
        let (x, y) = (r % (w - 0x14) + d.offset_0x0, d.offset_0xc + d.offset_0x4);
        FUN_00510f70(g, this, x, y);
    }
}

/// port: 00510f70 FUN_00510f70
/// `AddBubble(int x, int y)`.
pub fn FUN_00510f70(g: &mut G, this: Ptr, param_1: i32, param_2: i32) {
    let mut b = Bubble::default();
    FUN_004ffdc0(g, &mut b);
    b.field_0x8 = param_1 as f32;
    b.field_0xc = param_2 as f32;
    g.bubble_mgr(this).offset_0x40.push(b);
}

/// port: 004ffdc0 FUN_004ffdc0
/// `Bubble::Bubble()`: a random cel (3 is re-rolled once), position and one of six speeds.
pub fn FUN_004ffdc0(g: &mut G, this: &mut Bubble) {
    this.field_0x0 = rand_i(g) % 4;
    if this.field_0x0 == 3 {
        this.field_0x0 = rand_i(g) % 4;
    }
    this.field_0xc = (rand_i(g) % 6 + 400) as f32;
    let r = rand_i(g);
    this.field_0x4 = 0;
    this.field_0x8 = (r % 0x16 + 0x96) as f32;
    let bits = match rand_i(g) % 6 {
        0 => 0x40000000,
        1 => 0x40200000,
        2 => 0x40400000,
        3 => 0x404ccccd,
        4 => 0x40133333,
        5 => 0x40333333,
        _ => return,
    };
    this.field_0x10 = f32::from_bits(bits);
}

/// port: 004ffec0 FUN_004ffec0
/// `Bubble::Update()`.
pub fn FUN_004ffec0(g: &mut G, this: &mut Bubble) {
    this.field_0x4 = rand_i(g) % 2;
    this.field_0xc -= this.field_0x10;
}

/// port: 004ffef0 FUN_004ffef0
/// `Bubble::Draw(Graphics*)`: additive.
pub fn FUN_004ffef0(g: &mut G, this: &Bubble, gfx: &mut Graphics) {
    crate::sexy::graphics::FUN_004558c0(gfx, 1);
    let ftol = crate::sexy::crt::ftol;
    let y = ftol(this.field_0xc as f64) as i32;
    let x = ftol(this.field_0x4 as f64 + this.field_0x8 as f64) as i32;
    let img = g.res.DAT_005e8c84;
    crate::sexy::graphics::FUN_00456950(gfx, g, img, x, y, this.field_0x0);
    crate::sexy::graphics::FUN_004558c0(gfx, 0);
}

/// `DAT_005df8e8` .. `DAT_005dfb40`: the 30 fish color schemes (red, green, blue of the
/// third layer, then of the second layer).
pub const DAT_005df8e8: [i32; 30] = [125, 255, 135, 175, 195, 255, 255, 60, 255, 190, 190, 145, 240, 255, 210, 175, 255, 65, 240, 240, 255, 255, 175, 225, 130, 160, 205, 190, 240, 210];
pub const DAT_005df960: [i32; 30] = [210, 240, 250, 235, 140, 255, 255, 60, 65, 60, 190, 145, 145, 255, 215, 240, 180, 70, 240, 225, 230, 40, 215, 175, 165, 230, 50, 140, 250, 250];
pub const DAT_005df9d8: [i32; 30] = [255, 60, 245, 135, 90, 85, 255, 60, 65, 255, 190, 175, 235, 40, 175, 175, 110, 160, 240, 125, 0, 0, 110, 95, 145, 50, 180, 250, 240, 50];
pub const DAT_005dfa50: [i32; 30] = [50, 45, 255, 50, 65, 250, 230, 0, 75, 255, 250, 30, 110, 210, 110, 30, 210, 240, 0, 240, 0, 255, 15, 65, 125, 145, 255, 250, 80, 110];
pub const DAT_005dfac8: [i32; 30] = [110, 95, 0, 115, 120, 155, 130, 0, 75, 0, 250, 25, 10, 10, 95, 80, 10, 240, 5, 5, 125, 225, 30, 55, 15, 0, 145, 245, 250, 210];
pub const DAT_005dfb40: [i32; 30] = [210, 195, 125, 210, 65, 0, 250, 0, 75, 125, 250, 225, 225, 0, 210, 125, 0, 240, 130, 240, 0, 0, 15, 190, 15, 210, 220, 175, 145, 0];

/// port: 004fff40 FUN_004fff40
/// `BgFish::BgFish()`: one of six speeds, a kind (half are kind 0) and a color scheme
/// brightened by 5.
pub fn FUN_004fff40(g: &mut G, this: &mut BgFish) {
    this.field_0x1c = crate::sexy::types::FUN_00433300();
    this.field_0x2c = crate::sexy::types::FUN_00433300();
    this.field_0x14 = 2.0;
    this.field_0x4 = 0;
    this.field_0x18 = 0.0;
    this.field_0x8 = 0;
    let bits: u32 = match rand_i(g) % 6 {
        0 => 0x3fa66666,
        1 => 0x3fd9999a,
        2 => 0x40000000,
        3 => 0x40200000,
        4 => 0x40400000,
        5 => 0x40600000,
        _ => 0,
    };
    if bits != 0 {
        this.field_0x14 = f32::from_bits(bits);
    }
    let r = rand_i(g);
    if r % 10 < 5 {
        this.field_0x0 = 0;
    } else {
        this.field_0x0 = (7 < r % 10) as i32 + 1;
    }
    let k = (rand_i(g) % 0x1e) as usize;
    let c = |v: i32| if v + 5 < 0x100 { v + 5 } else { 0xff };
    this.field_0x1c = crate::sexy::types::CRect(c(DAT_005df8e8[k]), c(DAT_005df960[k]), c(DAT_005df9d8[k]), 0xff);
    this.field_0x2c = crate::sexy::types::CRect(c(DAT_005dfa50[k]), c(DAT_005dfac8[k]), c(DAT_005dfb40[k]), 0xff);
}

/// port: 005000f0 FUN_005000f0
/// `BgFish::Update()`.
pub fn FUN_005000f0(this: &mut BgFish) {
    this.field_0x4 += 1;
    this.field_0xc = this.field_0x14 + this.field_0xc;
    this.field_0x10 = this.field_0x18 + this.field_0x10;
    if this.field_0x8 != 0 {
        this.field_0x8 -= 1;
    }
}

/// port: 00511010 FUN_00511010
/// `AddFish()`: at a random height of the fish area (taller than 80 only), entering from
/// the left or (swimming left) from the right.
pub fn FUN_00511010(g: &mut G, this: Ptr) {
    if 0x50 < g.bubble_mgr(this).offset_0x1c {
        let mut f = BgFish::default();
        FUN_004fff40(g, &mut f);
        let r = rand_i(g);
        let d = g.bubble_mgr(this);
        f.field_0x10 = (r % (d.offset_0x1c - 0x50) + d.offset_0x14) as f32;
        f.field_0x18 = f32::from_bits(d.offset_0x34 as u32);
        d.offset_0x4c.push(f);
        let r = rand_i(g);
        let d = g.bubble_mgr(this);
        let (left, right) = (d.offset_0x10 - 0x50, d.offset_0x18 + d.offset_0x10);
        let f = d.offset_0x4c.last_mut().unwrap();
        if r % 2 == 0 {
            f.field_0xc = left as f32;
            return;
        }
        f.field_0xc = right as f32;
        f.field_0x14 = -f.field_0x14;
    }
}

/// port: 00504a20 FUN_00504a20
/// `ScatterFish()`: every fish dashes (at 20) toward the nearer side, turning around if it
/// was headed the other way.
pub fn FUN_00504a20(g: &mut G, this: Ptr) {
    let d = g.bubble_mgr(this);
    let mid = (d.offset_0x18 / 2 + d.offset_0x10) as f32;
    for f in d.offset_0x4c.iter_mut() {
        if mid <= f.field_0xc {
            if f.field_0x14 < 0.0 {
                f.field_0x8 = 5;
            }
            f.field_0x14 = f32::from_bits(0x41a00000);
        } else {
            if 0.0 < f.field_0x14 {
                f.field_0x8 = 5;
            }
            f.field_0x14 = f32::from_bits(0xc1a00000);
        }
    }
}

/// port: 00504b60 FUN_00504b60
/// `BubbleMgr::Draw(Graphics*)`: the bubbles clipped to their area, then the fish (clipped
/// to theirs when +0x3c is set), each set on its own copy of the Graphics.
pub fn FUN_00504b60(g: &mut G, this: Ptr, param_1: &mut Graphics) {
    use crate::sexy::graphics::{FUN_004563b0, FUN_00455800};
    let d = g.bubble_mgr(this).clone();
    let mut g1 = FUN_00455800(param_1);
    FUN_004563b0(&mut g1, &Rect::new(d.offset_0x0, d.offset_0x4, d.offset_0x8, d.offset_0xc));
    for b in &d.offset_0x40 {
        FUN_004ffef0(g, b, &mut g1);
    }
    let mut g2 = FUN_00455800(param_1);
    if d.offset_0x38 as u8 != 0 {
        FUN_004563b0(&mut g2, &Rect::new(d.offset_0x10, d.offset_0x14, d.offset_0x18, d.offset_0x1c));
    }
    for f in &d.offset_0x4c {
        FUN_00502e10(g, f, &mut g2);
    }
}

/// port: 00502e10 FUN_00502e10
/// `BgFish::Draw(Graphics*)`: bobbing 3 pixels; the base strip (IMAGE id 0xb8, or the turn
/// strip 0xae while turning) then two additive colorized layers (ids +1, +2).
pub fn FUN_00502e10(g: &mut G, this: &BgFish, param_1: &mut Graphics) {
    use crate::sexy::graphics::{FUN_004558c0, FUN_004558e0, FUN_00455890, FUN_004560a0};
    let ftol = crate::sexy::crt::ftol;
    let t = this.field_0x4;
    let mut src = Rect::new(((t / 2) % 10) * 0x50, this.field_0x0 * 0x50, 0x50, 0x50);
    let x = ftol(this.field_0xc as f64) as i32;
    let a = ((((t * 5) as f64) * DAT_0059be10) / 180.0) as f32;
    let s = (a as f64).sin() as f32;
    let y = ftol(s as f64 * 3.0 + this.field_0x10 as f64) as i32;
    let mut mirror = 0.0 < this.field_0x14;
    let mut id = 0xb8;
    if 0 < this.field_0x8 {
        src.mX = (5 - this.field_0x8) * 0xa0;
        mirror = !mirror;
        id = 0xae;
    }
    let img = crate::sexy::res::FUN_005016a0(g, id) as Ptr;
    FUN_004560a0(param_1, g, img, x, y, &src, mirror);
    FUN_004558c0(param_1, 1);
    FUN_004558e0(param_1, true);
    FUN_00455890(param_1, this.field_0x2c);
    let img = crate::sexy::res::FUN_005016a0(g, id + 1) as Ptr;
    FUN_004560a0(param_1, g, img, x, y, &src, mirror);
    FUN_00455890(param_1, this.field_0x1c);
    let img = crate::sexy::res::FUN_005016a0(g, id + 2) as Ptr;
    FUN_004560a0(param_1, g, img, x, y, &src, mirror);
    FUN_004558c0(param_1, 0);
    FUN_004558e0(param_1, false);
}

/// `DAT_0059be10`: the double the game uses for pi (3.14159 rounded through a float).
pub const DAT_0059be10: f64 = 3.141590118408203;

/// port: 005346d0 FUN_005346d0
/// `MyLabelWidget::SetText(const string&)`, then `MarkDirty`.
pub fn FUN_005346d0(g: &mut G, this: Ptr, param_1: &[u8]) {
    g.my_label(this).field_0x0 = param_1.to_vec();
    vcall!(g, this, w.vfunction18);
}
