//! `Sexy::ListWidget` (framework, 0x00470b60..0x004728d0): a scrollable list of text lines,
//! each with its own color, with a hilited (hovered) and a selected line; reports clicks,
//! hilite changes and closing to its ListListener and follows an optional ScrollbarWidget.
//! Lists can be chained side by side (`mParent`/`mChild`) to share lines, hilite, selection
//! and scrolling.
//!
//! `ListWidget_data` starts at object offset 0x8c (after the ScrollListener vftable at
//! 0x88). The object is 0xf4 bytes, past the database's 0xbc (fields `ext_0xNN`).

use crate::sexy::graphics::{FUN_00455800, FUN_00455870, FUN_00455880, FUN_00455890, FUN_00455920, FUN_004559b0, FUN_00455cf0, FUN_00456340};
use crate::sexy::image_font as font;
use crate::sexy::prelude::*;

/// `gDefaultListWidgetColors` (@ 005e72d0): background, outline, text, hilite, select,
/// select text.
pub const DAT_005e72d0: [[i32; 3]; 6] = [[255, 255, 255], [255, 255, 255], [0, 0, 0], [0, 0, 192], [0, 0, 128], [255, 255, 255]];

/// `ListWidget_data` (object offset 0x8c) plus the fields past the database size.
#[derive(Debug, Clone, Default)]
pub struct ListWidget_data {
    /// +0x8c `mId`.
    pub field_0x0: i32,
    /// +0x90 `mFont`.
    pub field_0x4: Ptr,
    /// +0x94 `mScrollbar`.
    pub field_0x8: Ptr,
    /// +0x98 `mJustify` (0 left, 1 center, 2 right).
    pub field_0xc: i32,
    /// +0x9c `mLines` (std::vector<std::string>; begin at +0xa0).
    pub offset_0x14: Vec<Vec<u8>>,
    /// +0xac `mLineColors` (std::vector<Color>; begin at +0xb0).
    pub offset_0x24: Vec<Color>,
    /// +0xc0 `mPosition` (first line shown).
    pub ext_0xc0: f64,
    /// +0xc8 `mPageSize` (lines shown).
    pub ext_0xc8: f64,
    /// +0xd0 `mHiliteIdx`.
    pub ext_0xd0: i32,
    /// +0xd4 `mSelectIdx`.
    pub ext_0xd4: i32,
    /// +0xd8 `mListListener` (the object whose ListListener vftable is told).
    pub ext_0xd8: Ptr,
    /// +0xdc `mParent`.
    pub ext_0xdc: Ptr,
    /// +0xe0 `mChild`.
    pub ext_0xe0: Ptr,
    /// +0xe4 `mSortFromChild`.
    pub ext_0xe4: bool,
    /// +0xe5 `mDrawOutline`.
    pub ext_0xe5: bool,
    /// +0xe8 `mMaxNumericPlaces` (sort keys are zero-padded to it).
    pub ext_0xe8: i32,
    /// +0xec `mItemHeight` (-1: the font's height).
    pub ext_0xec: i32,
    /// +0xf0 `mDrawSelectWhenHilited`.
    pub ext_0xf0: bool,
    /// +0xf1 `mDoFingerWhenHilited`.
    pub ext_0xf1: bool,
}

impl G {
    pub fn list(&mut self, p: Ptr) -> &mut ListWidget_data {
        match &mut self.widget(p).ext {
            WExt::List(d) => d,
            e => panic!("{p} is not a ListWidget: {e:?}"),
        }
    }
}

/// The first list of the chain this one belongs to.
fn chain_head(g: &mut G, this: Ptr) -> Ptr {
    let mut l = this;
    while g.list(l).ext_0xdc != NULL {
        l = g.list(l).ext_0xdc;
    }
    l
}

/// `mItemHeight`, or the font's height.
fn item_height(g: &mut G, this: Ptr) -> i32 {
    let h = g.list(this).ext_0xec;
    if h == -1 {
        let f = g.list(this).field_0x4;
        return font::get_height(g, f);
    }
    h
}

/// The scrollbar's range follows the line count.
fn update_scroll_max(g: &mut G, this: Ptr) {
    let sb = g.list(this).field_0x8;
    if sb != NULL {
        let n = g.list(this).offset_0x14.len() as u32;
        vcall!(g, sb, scroll.vfunction72, n as f64);
    }
}

/// port: 00470b60 Sexy::ListWidget::ListWidget
/// `ListWidget(int theId, Font*, ListListener*)`: the default colors, left
/// justified, outlined, the font's item height, nothing hilited or selected.
pub fn ListWidget(g: &mut G, param_1: i32, param_2: Ptr, param_3: Ptr) -> Ptr {
    let (wc, w) = crate::sexy::widget::Widget();
    let d = ListWidget_data {
        field_0x0: param_1,
        field_0x4: param_2,
        ext_0xd0: -1,
        ext_0xd4: -1,
        ext_0xd8: param_3,
        ext_0xe5: true,
        ext_0xf1: true,
        ..Default::default()
    };
    let this = g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__ListWidget_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::List(Box::new(d)) }),
    });
    let ih = if param_2 != NULL { font::get_height(g, param_2) } else { -1 };
    g.list(this).ext_0xec = ih;
    crate::sexy::widget::vfunction34(g, this, &DAT_005e72d0);
    this
}

/// port: 00470cb0 Sexy::ListWidget::~ListWidget
pub fn dtor_ListWidget(g: &mut G, this: Ptr) {
    let d = g.list(this);
    d.offset_0x24.clear();
    d.offset_0x14.clear();
    crate::sexy::widget::dtor_Widget(g, this);
}

/// port: 00470c90 Sexy::ListWidget::deleting_destructor_for_Widget
pub fn deleting_destructor_for_Widget(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_ListWidget(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 00470d40 Sexy::ListWidget::vfunction22_for_Widget
/// `RemovedFromManager(WidgetManager*)`: tells the listener the list closed.
pub fn vfunction22_for_Widget(g: &mut G, this: Ptr, param_1: Ptr) {
    crate::sexy::widget_container::vfunction22(g, this, param_1);
    let d = g.list(this).clone();
    if d.ext_0xd8 != NULL {
        vcall!(g, d.ext_0xd8, ll.vfunction2, d.field_0x0);
    }
}

/// port: 00470d80 Sexy::ListWidget::vfunction71_for_Widget
/// `GetSortKey(int idx)`: the line zero-padded to `mMaxNumericPlaces`, followed (or, with
/// `mSortFromChild`, preceded) by the child list's key.
pub fn vfunction71_for_Widget(g: &mut G, this: Ptr, param_1: i32) -> Vec<u8> {
    let d = g.list(this).clone();
    let mut s = d.offset_0x14[param_1 as usize].clone();
    while (s.len() as u32) < d.ext_0xe8 as u32 {
        let mut t = b"0".to_vec();
        t.extend_from_slice(&s);
        s = t;
    }
    if d.ext_0xe4 {
        let mut k = vcall!(g, d.ext_0xe0, list.vfunction71, param_1);
        k.extend_from_slice(&s);
        return k;
    }
    if d.ext_0xe0 == NULL {
        return s;
    }
    let k = vcall!(g, d.ext_0xe0, list.vfunction71, param_1);
    s.extend_from_slice(&k);
    s
}

/// port: 00470fb0 Sexy::ListWidget::vfunction72_for_Widget
/// `Sort(bool ascending)`: a bubble sort on the sort keys, then every list of the chain
/// reordered the same way.
pub fn vfunction72_for_Widget(g: &mut G, this: Ptr, param_1: bool) {
    let n = g.list(this).offset_0x14.len();
    let mut map: Vec<usize> = (0..n).collect();
    let mut keys: Vec<Vec<u8>> = Vec::with_capacity(n);
    for i in 0..n {
        let k = vcall!(g, this, list.vfunction71, i as i32);
        keys.push(k);
    }
    if 1 < n {
        for i in 1..n {
            for j in 0..n - i {
                let c = keys[j].cmp(&keys[j + 1]);
                if (param_1 && c == std::cmp::Ordering::Greater) || (!param_1 && c == std::cmp::Ordering::Less) {
                    map.swap(j, j + 1);
                    keys.swap(j, j + 1);
                }
            }
        }
    }
    let mut l = chain_head(g, this);
    loop {
        let d = g.list(l);
        let lines: Vec<Vec<u8>> = map.iter().map(|&k| d.offset_0x14[k].clone()).collect();
        let colors: Vec<Color> = map.iter().map(|&k| d.offset_0x24[k]).collect();
        d.offset_0x14 = lines;
        d.offset_0x24 = colors;
        vcall!(g, l, w.vfunction18);
        l = g.list(l).ext_0xe0;
        if l == NULL {
            break;
        }
    }
}

/// port: 00471460 Sexy::ListWidget::vfunction73_for_Widget
/// `GetStringAt(int idx)`.
pub fn vfunction73_for_Widget(g: &mut G, this: Ptr, param_1: i32) -> Vec<u8> {
    g.list(this).offset_0x14[param_1 as usize].clone()
}

/// port: 004714e0 Sexy::ListWidget::vfunction41_for_Widget
/// `Resize(x, y, w, h)`: the page is the lines that fit inside the 4-pixel border.
pub fn vfunction41_for_Widget(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32, param_4: i32) {
    crate::sexy::widget::vfunction41(g, this, param_1, param_2, param_3, param_4);
    let ih = item_height(g, this);
    let h = g.wc(this).offset_0x38;
    let mut page = 1.0;
    if ih + 8 < h {
        page = (h as f64 - 8.0) / ih as f64;
    }
    g.list(this).ext_0xc8 = page;
    let sb = g.list(this).field_0x8;
    if sb != NULL {
        vcall!(g, sb, scroll.vfunction73, page);
    }
}

/// port: 00471580 Sexy::ListWidget::vfunction74_for_Widget
/// `AddLine(const string&, bool alphabetical)`: inserted before the first line it sorts
/// before (alphabetical) or appended, in the text color; the chain's other lists get "-".
/// Returns its index.
pub fn vfunction74_for_Widget(g: &mut G, this: Ptr, param_1: &[u8], param_2: bool) -> i32 {
    let text = g.w(this).offset_0xc[2];
    let mut at: Option<usize> = None;
    if param_2 {
        let lines = g.list(this).offset_0x14.clone();
        for (i, l) in lines.iter().enumerate() {
            if crate::sexy::crt::strcmp(param_1, l) < 0 {
                at = Some(i);
                break;
            }
        }
    }
    let idx = match at {
        Some(i) => i,
        None => g.list(this).offset_0x14.len(),
    };
    let mut l = chain_head(g, this);
    loop {
        let line = if l == this { param_1.to_vec() } else { b"-".to_vec() };
        let d = g.list(l);
        match at {
            Some(i) => {
                d.offset_0x14.insert(i, line);
                d.offset_0x24.insert(i, text);
            }
            None => {
                d.offset_0x14.push(line);
                d.offset_0x24.push(text);
            }
        }
        vcall!(g, l, w.vfunction18);
        l = g.list(l).ext_0xe0;
        if l == NULL {
            break;
        }
    }
    update_scroll_max(g, this);
    idx as i32
}

/// port: 004719e0 Sexy::ListWidget::vfunction75_for_Widget
/// `SetLine(int idx, const string&)`.
pub fn vfunction75_for_Widget(g: &mut G, this: Ptr, param_1: i32, param_2: &[u8]) {
    g.list(this).offset_0x14[param_1 as usize] = param_2.to_vec();
    vcall!(g, this, w.vfunction18);
}

/// port: 00471a50 Sexy::ListWidget::vfunction76_for_Widget
/// `GetLineCount()`.
pub fn vfunction76_for_Widget(g: &mut G, this: Ptr) -> i32 {
    g.list(this).offset_0x14.len() as i32
}

/// port: 00471a80 Sexy::ListWidget::vfunction77_for_Widget
/// `GetLineIdx(const string&)`: -1 when absent.
pub fn vfunction77_for_Widget(g: &mut G, this: Ptr, param_1: &[u8]) -> i32 {
    let lines = g.list(this).offset_0x14.clone();
    for (i, l) in lines.iter().enumerate() {
        if crate::sexy::crt::strcmp(l, param_1) == 0 {
            return i as i32;
        }
    }
    -1
}

/// port: 00471b60 Sexy::ListWidget::vfunction78_for_Widget
/// `SetColor(const string& line, const Color&)`.
pub fn vfunction78_for_Widget(g: &mut G, this: Ptr, param_1: &[u8], param_2: Color) {
    let i = vcall!(g, this, list.vfunction77, param_1);
    vcall!(g, this, list.vfunction79, i, param_2);
}

/// port: 00471ba0 Sexy::ListWidget::vfunction79_for_Widget
/// `SetColor(int idx, const Color&)`: the line's color in every list of the chain.
pub fn vfunction79_for_Widget(g: &mut G, this: Ptr, param_1: i32, param_2: Color) {
    if -1 < param_1 && param_1 < g.list(this).offset_0x14.len() as i32 {
        let mut l = chain_head(g, this);
        loop {
            g.list(l).offset_0x24[param_1 as usize] = param_2;
            vcall!(g, l, w.vfunction18);
            l = g.list(l).ext_0xe0;
            if l == NULL {
                break;
            }
        }
    }
}

/// port: 00471c60 Sexy::ListWidget::vfunction80_for_Widget
/// `RemoveLine(int idx)`, from every list of the chain.
pub fn vfunction80_for_Widget(g: &mut G, this: Ptr, param_1: i32) {
    if param_1 != -1 {
        let mut l = chain_head(g, this);
        loop {
            let d = g.list(l);
            d.offset_0x14.remove(param_1 as usize);
            d.offset_0x24.remove(param_1 as usize);
            vcall!(g, l, w.vfunction18);
            l = g.list(l).ext_0xe0;
            if l == NULL {
                break;
            }
        }
    }
    update_scroll_max(g, this);
}

/// port: 00471e30 Sexy::ListWidget::vfunction81_for_Widget
/// `RemoveAll()`: every list of the chain emptied, nothing hilited or selected.
pub fn vfunction81_for_Widget(g: &mut G, this: Ptr) {
    let mut l = chain_head(g, this);
    loop {
        let d = g.list(l);
        d.offset_0x14.clear();
        d.offset_0x24.clear();
        d.ext_0xd4 = -1;
        d.ext_0xd0 = -1;
        vcall!(g, l, w.vfunction18);
        l = g.list(l).ext_0xe0;
        if l == NULL {
            break;
        }
    }
    update_scroll_max(g, this);
}

/// port: 00471ff0 Sexy::ListWidget::vfunction82_for_Widget
/// `GetOptimalWidth()`: the widest line plus 16.
pub fn vfunction82_for_Widget(g: &mut G, this: Ptr) -> i32 {
    let d = g.list(this).clone();
    let mut best = 0;
    for l in &d.offset_0x14 {
        let w = font::string_width(g, d.field_0x4, l);
        if best <= w {
            best = font::string_width(g, d.field_0x4, l);
        }
    }
    best + 0x10
}

/// port: 004720e0 Sexy::ListWidget::vfunction83_for_Widget
/// `GetOptimalHeight()`: all lines plus the border.
pub fn vfunction83_for_Widget(g: &mut G, this: Ptr) -> i32 {
    let ih = item_height(g, this);
    g.list(this).offset_0x14.len() as i32 * ih + 8
}

/// port: 00472140 Sexy::ListWidget::vfunction31_for_Widget
/// `OrderInManagerChanged()`: the child list and the scrollbar stay in front of it.
pub fn vfunction31_for_Widget(g: &mut G, this: Ptr) {
    // `DAT_005eb680` (gSexyAppBase; the same object as `DAT_005eb6a4`).
    let app = g.globals.DAT_005eb6a4;
    let wm = g.sab(app).offset_0x318;
    let d = g.list(this).clone();
    if d.ext_0xe0 != NULL {
        vcall!(g, wm, w.vfunction15, d.ext_0xe0, this);
    }
    if d.field_0x8 != NULL {
        vcall!(g, wm, w.vfunction15, d.field_0x8, this);
    }
}

/// port: 00472190 Sexy::ListWidget::vfunction27_for_Widget
/// `Draw(Graphics*)`: the background; the visible lines (the selected one, or a hilited one
/// when so set, on the select color) in the hilite, select-text or line color, justified;
/// the outline.
pub fn vfunction27_for_Widget(g: &mut G, this: Ptr, param_1: &mut Graphics) {
    let (w, h) = (g.wc(this).offset_0x34, g.wc(this).offset_0x38);
    let colors = g.w(this).offset_0xc.clone();
    FUN_00455890(param_1, colors[0]);
    FUN_00455920(param_1, 0, 0, w, h);
    let mut clip = FUN_00455800(param_1);
    FUN_00456340(&mut clip, 4, 4, w - 8, h - 8);
    let mut sel_clip = FUN_00455800(param_1);
    FUN_00456340(&mut sel_clip, 0, 4, w, h - 8);
    let d = g.list(this).clone();
    FUN_00455880(&mut clip, d.field_0x4);
    let first = crate::sexy::crt::ftol(d.ext_0xc0) as i32;
    let n = d.offset_0x14.len() as i32;
    let mut last = crate::sexy::crt::ftol(d.ext_0xc8) as i32 + 1 + first;
    if n - 1 < last {
        last = n - 1;
    }
    let (ih, off) = if d.ext_0xec == -1 {
        (font::get_height(g, d.field_0x4), 0)
    } else {
        let fh = font::get_height(g, d.field_0x4);
        (d.ext_0xec, (d.ext_0xec - fh) / 2)
    };
    let mut i = first;
    while i <= last {
        let dim = crate::sexy::crt::ftol((i as f64 - d.ext_0xc0) * ih as f64) as i32;
        if i == d.ext_0xd4 || (i == d.ext_0xd0 && d.ext_0xf0) {
            FUN_00455890(&mut sel_clip, colors[4]);
            FUN_00455920(&mut sel_clip, 0, dim + 4, w, ih);
        }
        let c = if i == d.ext_0xd0 {
            colors[3]
        } else if i == d.ext_0xd4 && 5 < colors.len() {
            colors[5]
        } else {
            d.offset_0x24[i as usize]
        };
        FUN_00455890(&mut clip, c);
        let s = d.offset_0x14[i as usize].clone();
        let x = match d.field_0xc {
            0 => 4,
            1 => (w - font::string_width(g, d.field_0x4, &s)) / 2,
            _ => (w - font::string_width(g, d.field_0x4, &s)) - 4,
        };
        let a = font::get_ascent(g, d.field_0x4);
        FUN_00455cf0(&mut clip, g, &s, x, a + dim + 4 + off);
        i += 1;
    }
    if d.ext_0xe5 {
        FUN_00455890(param_1, colors[1]);
        FUN_004559b0(param_1, 0, 0, w - 1, h - 1);
    }
    let _ = FUN_00455870(&clip);
}

/// port: 00472680 FUN_00472680
/// `SetHilite(int idx)` (list in EAX, index in EDX): a change is reported to the listener.
pub fn FUN_00472680(g: &mut G, this: Ptr, param_1: i32) {
    let old = g.list(this).ext_0xd0;
    g.list(this).ext_0xd0 = param_1;
    let d = g.list(this).clone();
    if old != param_1 && d.ext_0xd8 != NULL {
        vcall!(g, d.ext_0xd8, ll.vfunction3, d.field_0x0, old, param_1);
    }
}

/// port: 00472630 Sexy::ListWidget::vfunction1_for_ScrollListener
/// `ScrollPosition(int id, double pos)`: the child list follows; the first line shown.
pub fn vfunction1_for_ScrollListener(g: &mut G, this: Ptr, param_1: i32, param_2: f64) {
    let child = g.list(this).ext_0xe0;
    if child != NULL {
        vcall!(g, child, scl.vfunction1, param_1, param_2);
    }
    g.list(this).ext_0xc0 = param_2;
    vcall!(g, this, w.vfunction18);
}

/// port: 004726b0 Sexy::ListWidget::vfunction53_for_Widget
/// `MouseMove(x, y)`: hilites the line under the mouse in the whole chain, with the finger
/// cursor over a line (when so set).
pub fn vfunction53_for_Widget(g: &mut G, this: Ptr, _x: i32, param_2: i32) {
    let ih = item_height(g, this);
    let pos = g.list(this).ext_0xc0;
    let mut idx = crate::sexy::crt::ftol((param_2 - 4) as f64 / ih as f64 + pos) as i32;
    if idx < 0 || g.list(this).offset_0x14.len() as i32 <= idx {
        idx = -1;
    }
    if idx != g.list(this).ext_0xd0 {
        let mut l = chain_head(g, this);
        loop {
            FUN_00472680(g, l, idx);
            vcall!(g, l, w.vfunction18);
            l = g.list(l).ext_0xe0;
            if l == NULL {
                break;
            }
        }
        let wm = g.wc(this).offset_0xc;
        let app = crate::sexy::widget_manager::wm(g, wm).offset_0x8;
        let d = g.list(this).clone();
        if d.ext_0xd0 != -1 && d.ext_0xf1 {
            crate::sexy::sexy_app_base::FUN_004891c0(g, app, 1);
            return;
        }
        crate::sexy::sexy_app_base::FUN_004891c0(g, app, 0);
    }
}

/// port: 004727a0 Sexy::ListWidget::vfunction54_for_Widget
/// `MouseDown(x, y, btn, clicks)`: a click on the hilited line goes to the listener.
pub fn vfunction54_for_Widget(g: &mut G, this: Ptr, _x: i32, _y: i32, _btn: i32, param_4: i32) {
    let d = g.list(this).clone();
    if d.ext_0xd0 != -1 && d.ext_0xd8 != NULL {
        vcall!(g, d.ext_0xd8, ll.vfunction1, d.field_0x0, d.ext_0xd0, param_4);
    }
}

/// port: 004727e0 Sexy::ListWidget::vfunction52_for_Widget
/// `MouseLeave()`: nothing hilited in the chain; the arrow cursor.
pub fn vfunction52_for_Widget(g: &mut G, this: Ptr) {
    let mut l = chain_head(g, this);
    loop {
        FUN_00472680(g, l, -1);
        vcall!(g, l, w.vfunction18);
        l = g.list(l).ext_0xe0;
        if l == NULL {
            break;
        }
    }
    let wm = g.wc(this).offset_0xc;
    let app = crate::sexy::widget_manager::wm(g, wm).offset_0x8;
    crate::sexy::sexy_app_base::FUN_004891c0(g, app, 0);
}

/// port: 00472830 Sexy::ListWidget::vfunction84_for_Widget
/// `SetSelect(int idx)` in the whole chain.
pub fn vfunction84_for_Widget(g: &mut G, this: Ptr, param_1: i32) {
    let mut l = chain_head(g, this);
    loop {
        g.list(l).ext_0xd4 = param_1;
        vcall!(g, l, w.vfunction18);
        l = g.list(l).ext_0xe0;
        if l == NULL {
            break;
        }
    }
}

/// port: 00472870 Sexy::ListWidget::vfunction60_for_Widget
/// `MouseWheel(int delta)`: scrolls five lines.
pub fn vfunction60_for_Widget(g: &mut G, this: Ptr, param_1: i32) {
    let sb = g.list(this).field_0x8;
    if sb != NULL {
        let v = g.scrollbar(sb).ext_0xa0;
        if 0 < param_1 {
            vcall!(g, sb, scroll.vfunction74, v - 5.0);
            return;
        }
        if param_1 < 0 {
            vcall!(g, sb, scroll.vfunction74, v + 5.0);
        }
    }
}
