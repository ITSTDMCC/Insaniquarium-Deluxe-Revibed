//! `Sexy::EditWidget` (framework, 0x00472b40..0x00474dd0): a one-line text field with a
//! blinking cursor, selection, word jumps, clipboard and undo, reporting to an
//! `EditListener`. `EditWidget_data` starts at object offset 0x88.
//!
//! Two OS services are replaced: the Windows clipboard (`SexyAppBase::CopyToClipboard` /
//! `GetClipboard`) by `G::clipboard`, and the system caret the framework positions for
//! accessibility when `SexyAppBase` +0x36a is set (never set in this game; kept as a check).

use crate::sexy::graphics::{FUN_00455800, FUN_00455880, FUN_00455890, FUN_00455920, FUN_004559b0, FUN_00455cf0, FUN_00456340};
use crate::sexy::image_font as font;
use crate::sexy::prelude::*;

/// `EditWidget_data` (object offset 0x88, 0x9c bytes).
#[derive(Debug, Clone, Default)]
pub struct EditWidget_data {
    /// +0x88 `mId`.
    pub offset_0x0: i32,
    /// +0x8c `mString` (std::string; its size at +0xa0).
    pub field_0x4: Vec<u8>,
    /// +0xa8 `mPasswordDisplayString`.
    pub field_0x20: Vec<u8>,
    /// +0xc4 `mFont` (owned copy).
    pub offset_0x3c: Ptr,
    /// +0xcc `mWidthCheckList` (std::list of {owned font copy, max pixels}).
    pub offset_0x44: Vec<(Ptr, i32)>,
    /// +0xd4 `mEditListener` (the object whose EditListener vftable receives events).
    pub offset_0x4c: Ptr,
    /// +0xd8 `mShowingCursor`.
    pub offset_0x50: bool,
    /// +0xd9 `mDrawSelOverride`.
    pub offset_0x51: bool,
    /// +0xda `mHadDoubleClick`.
    pub offset_0x52: bool,
    /// +0xdc `mCursorPos`.
    pub offset_0x54: i32,
    /// +0xe0 `mHilitePos` (-1 = no selection).
    pub offset_0x58: i32,
    /// +0xe4 `mBlinkAcc`.
    pub offset_0x5c: i32,
    /// +0xe8 `mBlinkDelay`.
    pub offset_0x60: i32,
    /// +0xec `mLeftPos` (first visible character).
    pub offset_0x64: i32,
    /// +0xf0 `mMaxChars` (-1 = no limit).
    pub offset_0x68: i32,
    /// +0xf4 `mMaxPixels` (-1 = no limit).
    pub offset_0x6c: i32,
    /// +0xf8 `mPasswordChar` (0 = plain text).
    pub offset_0x70: u8,
    /// +0xfc `mUndoString`.
    pub field_0x74: Vec<u8>,
    /// +0x118 `mUndoCursor`.
    pub offset_0x90: i32,
    /// +0x11c `mUndoHilitePos`.
    pub offset_0x94: i32,
    /// +0x120 `mLastModifyIdx`.
    pub offset_0x98: i32,
}

impl G {
    pub fn edit(&mut self, p: Ptr) -> &mut EditWidget_data {
        match &mut self.widget(p).ext {
            WExt::EditWidget(e) => e,
            e => panic!("{p} is not an EditWidget: {e:?}"),
        }
    }
}

/// `gEditWidgetColors` (@ 005e7294): background, outline, text, hilite, hilite text.
const EDIT_WIDGET_COLORS: [[i32; 3]; 5] = [[255, 255, 255], [0, 0, 0], [0, 0, 0], [0, 0, 0], [255, 255, 255]];

/// `std::string::substr(pos, count)`.
fn substr(s: &[u8], pos: i32, count: i32) -> Vec<u8> {
    let pos = pos as usize;
    if pos > s.len() {
        crate::sexy::pending(0x00564335, "std::string::substr out of range (invalid parameter handler)");
    }
    let end = if count < 0 { s.len() } else { (pos + count as usize).min(s.len()) };
    s[pos..end].to_vec()
}

/// port: 00472b40 Sexy::EditWidget::EditWidget
/// `EditWidget(int theId, EditListener*)`.
pub fn EditWidget(g: &mut G, param_1: i32, param_2: Ptr) -> Ptr {
    let (wc, w) = crate::sexy::widget::Widget();
    let d = EditWidget_data {
        offset_0x0: param_1,
        offset_0x4c: param_2,
        offset_0x3c: NULL,
        offset_0x52: false,
        offset_0x58: -1,
        offset_0x64: 0,
        offset_0x90: 0,
        offset_0x94: 0,
        offset_0x98: 0,
        offset_0x5c: 0,
        offset_0x54: 0,
        offset_0x50: false,
        offset_0x51: false,
        offset_0x68: -1,
        offset_0x6c: -1,
        offset_0x70: 0,
        offset_0x60: 0x28,
        ..Default::default()
    };
    let this = g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__EditWidget_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::EditWidget(Box::new(d)) }),
    });
    crate::sexy::widget::vfunction34(g, this, &EDIT_WIDGET_COLORS);
    this
}

/// port: 00472c90 Sexy::EditWidget::~EditWidget
pub fn dtor_EditWidget(g: &mut G, this: Ptr) {
    let f = g.edit(this).offset_0x3c;
    if f != NULL {
        g.free(f);
    }
    FUN_00472da0(g, this);
    let e = g.edit(this);
    e.field_0x74.clear();
    e.field_0x20.clear();
    e.field_0x4.clear();
    crate::sexy::widget::dtor_Widget(g, this);
}

/// port: 00472c70 Sexy::EditWidget::deleting_destructor
pub fn deleting_destructor(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_EditWidget(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 00472da0 FUN_00472da0
/// `ClearWidthCheckFonts()` (this in EAX).
pub fn FUN_00472da0(g: &mut G, this: Ptr) {
    let list = std::mem::take(&mut g.edit(this).offset_0x44);
    for (f, _) in list {
        if f != NULL {
            g.free(f);
        }
    }
}

/// port: 00472e30 FUN_00472e30
/// `AddWidthCheckFont(Font*, int theMaxPixels)`: a copy of the font limits the text width.
pub fn FUN_00472e30(g: &mut G, this: Ptr, param_1: Ptr, param_2: i32) {
    let copy = font::duplicate(g, param_1);
    g.edit(this).offset_0x44.push((copy, param_2));
}

/// port: 00472f20 FUN_00472f20
/// `GetDisplayString()` (this in ESI): the text, or as many password characters.
pub fn FUN_00472f20(g: &mut G, this: Ptr) -> Vec<u8> {
    let e = g.edit(this);
    if e.offset_0x70 == 0 {
        return e.field_0x4.clone();
    }
    if e.field_0x20.len() != e.field_0x4.len() {
        e.field_0x20 = vec![e.offset_0x70; e.field_0x4.len()];
    }
    e.field_0x20.clone()
}

/// port: 00472eb0 Sexy::EditWidget::vfunction74
/// `SetText(const string&, bool leftPosToZero)`: cursor to the end.
pub fn vfunction74(g: &mut G, this: Ptr, text: &[u8], left_pos_to_zero: bool) {
    let e = g.edit(this);
    e.field_0x4 = text.to_vec();
    e.offset_0x54 = e.field_0x4.len() as i32;
    e.offset_0x58 = 0;
    if left_pos_to_zero {
        g.edit(this).offset_0x64 = 0;
        vcall!(g, this, w.vfunction18);
        return;
    }
    vcall!(g, this, edit.vfunction77, true);
    vcall!(g, this, w.vfunction18);
}

/// port: 00472ff0 Sexy::EditWidget::vfunction41
/// `Resize(int x, int y, int w, int h)`.
pub fn vfunction41(g: &mut G, this: Ptr, x: i32, y: i32, w: i32, h: i32) {
    crate::sexy::widget::vfunction41(g, this, x, y, w, h);
    vcall!(g, this, edit.vfunction77, false);
}

/// port: 00473020 Sexy::EditWidget::vfunction73
/// `SetFont(Font*, Font* theWidthCheckFont)`.
pub fn vfunction73(g: &mut G, this: Ptr, param_1: Ptr, param_2: Ptr) {
    let old = g.edit(this).offset_0x3c;
    if old != NULL {
        g.free(old);
    }
    let copy = font::duplicate(g, param_1);
    g.edit(this).offset_0x3c = copy;
    FUN_00472da0(g, this);
    if param_2 != NULL {
        FUN_00472e30(g, this, param_2, -1);
    }
}

/// port: 00473070 Sexy::EditWidget::vfunction27
/// `Draw(Graphics*)`: background; the text clipped to the field; the selection (or the cursor,
/// two pixels wide while showing) redrawn in the hilite colors; the outline.
pub fn vfunction27(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    if g.edit(this).offset_0x3c == NULL {
        // `new SysFont(app, "Arial Unicode MS", 10, false)`: a Windows GDI font; the game sets
        // an ImageFont on every edit widget before it is drawn.
        crate::sexy::pending(0x00473070, "EditWidget::Draw fallback SysFont (Windows GDI font)");
    }
    let s = FUN_00472f20(g, this);
    let (w, h) = (g.wc(this).offset_0x34, g.wc(this).offset_0x38);
    let colors = g.w(this).offset_0xc.clone();
    FUN_00455890(gfx, colors[0]);
    FUN_00455920(gfx, 0, 0, w, h);
    for i in 0..2 {
        let mut clip = FUN_00455800(gfx);
        let f = g.edit(this).offset_0x3c;
        FUN_00455880(&mut clip, f);
        if i == 1 {
            let e = g.edit(this).clone();
            let left = font::string_width(g, f, &substr(&s, 0, e.offset_0x64));
            let mut cursor_x = font::string_width(g, f, &substr(&s, 0, e.offset_0x54)) - left;
            let mut hilite_x = cursor_x + 2;
            if e.offset_0x58 != -1 && e.offset_0x54 != e.offset_0x58 {
                hilite_x = font::string_width(g, f, &substr(&s, 0, e.offset_0x58)) - left;
            }
            if !e.offset_0x50 {
                cursor_x += 2;
            }
            let max = w - 8;
            let cx = cursor_x.max(0);
            let cx = if cx < max { cx } else { max };
            let hx = hilite_x.max(0);
            let hx = if hx < max { hx } else { max };
            let lo = if cx < hx { cx } else { hx };
            let fh = font::get_height(g, f);
            let fh2 = font::get_height(g, f);
            FUN_00456340(&mut clip, lo + 4, (h - fh2) / 2, (hx - cx).abs(), fh);
        } else {
            FUN_00456340(&mut clip, 4, 0, w - 8, h);
        }
        let has_focus = g.w(this).offset_0x3 || g.edit(this).offset_0x51;
        if i == 1 && has_focus {
            FUN_00455890(&mut clip, colors[3]);
            FUN_00455920(&mut clip, 0, 0, w, h);
        }
        if i == 0 || !has_focus {
            FUN_00455890(&mut clip, colors[2]);
        } else {
            FUN_00455890(&mut clip, colors[4]);
        }
        let left_pos = g.edit(this).offset_0x64;
        let text = substr(&s, left_pos, -1);
        let fh = font::get_height(g, f);
        let asc = font::get_ascent(g, f);
        FUN_00455cf0(&mut clip, g, &text, 4, asc + (h - fh) / 2);
    }
    FUN_00455890(gfx, colors[1]);
    FUN_004559b0(gfx, 0, 0, w - 1, h - 1);
}

/// port: 004735b0 FUN_004735b0
/// `UpdateCaretPos()`: positions the Windows system caret (only when SexyAppBase +0x36a is
/// set; replaced, see the module notes).
pub fn FUN_004735b0(_g: &mut G, _this: Ptr) {
    crate::sexy::pending(0x004735b0, "EditWidget::UpdateCaretPos (Windows system caret)");
}

fn os_caret_enabled(g: &mut G, this: Ptr) -> bool {
    let wm = g.wc(this).offset_0xc;
    if wm == NULL {
        return false;
    }
    let app = crate::sexy::widget_manager::wm(g, wm).offset_0x8;
    g.sab(app).field_0x362
}

/// port: 00473630 Sexy::EditWidget::vfunction46
/// `GotFocus()`: the cursor shows at once.
pub fn vfunction46(g: &mut G, this: Ptr) {
    crate::sexy::widget::vfunction46(g, this);
    if os_caret_enabled(g, this) {
        FUN_004735b0(g, this);
    }
    let e = g.edit(this);
    e.offset_0x50 = true;
    e.offset_0x5c = 0;
    vcall!(g, this, w.vfunction18);
}

/// port: 00473690 Sexy::EditWidget::vfunction47
/// `LostFocus()`.
pub fn vfunction47(g: &mut G, this: Ptr) {
    crate::sexy::widget::vfunction47(g, this);
    if os_caret_enabled(g, this) {
        FUN_004735b0(g, this);
    }
    g.edit(this).offset_0x50 = false;
    vcall!(g, this, w.vfunction18);
}

/// port: 004736d0 Sexy::EditWidget::vfunction23
/// `Update()`: blinks the cursor while focused.
pub fn vfunction23(g: &mut G, this: Ptr) {
    crate::sexy::widget_container::vfunction23(g, this);
    if g.w(this).offset_0x3 {
        if os_caret_enabled(g, this) {
            FUN_004735b0(g, this);
        }
        g.edit(this).offset_0x5c += 1;
        if g.edit(this).offset_0x60 < g.edit(this).offset_0x5c {
            vcall!(g, this, w.vfunction18);
            let e = g.edit(this);
            e.offset_0x5c = 0;
            e.offset_0x50 = !e.offset_0x50;
        }
    }
}

/// port: 00473730 FUN_00473730
/// `EnforceMaxPixels()`: trims the text until it fits `mMaxPixels`, or each width-check
/// font's limit (falling back to `mMaxPixels`).
pub fn FUN_00473730(g: &mut G, this: Ptr) {
    let (max, list) = (g.edit(this).offset_0x6c, g.edit(this).offset_0x44.clone());
    if !(0 < max || !list.is_empty()) {
        return;
    }
    if list.is_empty() {
        let f = g.edit(this).offset_0x3c;
        loop {
            let s = g.edit(this).field_0x4.clone();
            if font::string_width(g, f, &s) <= max {
                break;
            }
            let n = s.len() as i32 - 1;
            g.edit(this).field_0x4 = substr(&s, 0, n);
        }
        return;
    }
    for (f, item_max) in list {
        let mut limit = item_max;
        if limit <= 0 {
            limit = max;
            if limit <= 0 {
                continue;
            }
        }
        loop {
            let s = g.edit(this).field_0x4.clone();
            if font::string_width(g, f, &s) <= limit {
                break;
            }
            let n = s.len() as i32 - 1;
            g.edit(this).field_0x4 = substr(&s, 0, n);
        }
    }
}

/// port: 004738f0 Sexy::EditWidget::vfunction75
/// `IsPartOfWord(char)`: letters, digits, '_' and bytes from 0xc0.
pub fn vfunction75(_g: &mut G, _this: Ptr, c: u8) -> bool {
    let s = c as i8;
    !(((s < b'A' as i8 || (b'Z' as i8) < s) && (s < b'a' as i8 || (b'z' as i8) < s)) && (s < b'0' as i8 || (b'9' as i8) < s) && c < 0xc0 && c != 0x5f)
}

/// The character at `i` of the edit text (`mString[i]`; index == length reads the
/// terminating NUL, as std::string's operator[] does).
fn ch(g: &mut G, this: Ptr, i: i32) -> u8 {
    g.edit(this).field_0x4.get(i as usize).copied().unwrap_or(0)
}

/// port: 00473930 Sexy::EditWidget::vfunction71
/// `ProcessKey(KeyCode theKey, char theChar)`: copy (^C), cut (^X), paste (^V), undo (^Z),
/// arrows (Ctrl = by word, Shift = extend selection), Backspace, Delete, Home, End, Enter (to
/// the listener), else inserts an allowed printable character. Then enforces the limits and
/// asks the listener to accept the new text (reverting when it refuses).
pub fn vfunction71(g: &mut G, this: Ptr, key: i32, c: u8) {
    let wm = g.wc(this).offset_0xc;
    let (shift, control) = {
        let m = crate::sexy::widget_manager::wm(g, wm);
        (m.offset_0xa0[0x10], m.offset_0xa0[0x11])
    };
    if key == 0x10 || key == 0x11 {
        return;
    }
    let mut remove_hilite = !shift;
    let mut big_change = false;
    if shift && g.edit(this).offset_0x58 == -1 {
        g.edit(this).offset_0x58 = g.edit(this).offset_0x54;
    }
    let old_string = g.edit(this).field_0x4.clone();
    let old_cursor = g.edit(this).offset_0x54;
    let old_hilite = g.edit(this).offset_0x58;
    let (id, listener) = (g.edit(this).offset_0x0, g.edit(this).offset_0x4c);
    let cur = old_cursor;
    let hil = old_hilite;
    if c == 3 || c == 0x18 {
        if hil != -1 && cur != hil {
            let disp = FUN_00472f20(g, this);
            let sel = if cur < hil { substr(&disp, cur, hil) } else { substr(&disp, hil, cur) };
            crate::sexy::sexy_app_base::copy_to_clipboard(g, &sel);
            if c == 3 {
                remove_hilite = false;
            } else {
                let s = g.edit(this).field_0x4.clone();
                let (lo, hi) = if cur <= hil { (cur, hil) } else { (hil, cur) };
                let mut n = substr(&s, 0, lo);
                n.extend(substr(&s, hi, -1));
                g.edit(this).field_0x4 = n;
                let e = g.edit(this);
                e.offset_0x54 = e.offset_0x54.min(e.offset_0x58);
                e.offset_0x58 = -1;
                big_change = true;
            }
        }
    } else if c == 0x16 {
        let base = crate::sexy::sexy_app_base::get_clipboard(g);
        if !base.is_empty() {
            let mut add: Vec<u8> = Vec::new();
            let f = g.edit(this).offset_0x3c;
            for &b in &base {
                if b == b'\r' || b == b'\n' {
                    break;
                }
                if font::char_width_kern(g, f, b, 0) != 0 && vcall!(g, listener, el.vfunction3, id, b) {
                    add.push(b);
                }
            }
            let s = g.edit(this).field_0x4.clone();
            let (cur, hil) = (g.edit(this).offset_0x54, g.edit(this).offset_0x58);
            if hil == -1 {
                let mut n = substr(&s, 0, cur);
                n.extend_from_slice(&add);
                n.extend(substr(&s, cur, -1));
                g.edit(this).field_0x4 = n;
            } else {
                let (lo, hi) = if hil < cur || hil <= cur { (hil, cur) } else { (cur, hil) };
                let mut n = substr(&s, 0, lo);
                n.extend_from_slice(&add);
                n.extend(substr(&s, hi, -1));
                g.edit(this).field_0x4 = n;
                let e = g.edit(this);
                e.offset_0x54 = if e.offset_0x58 <= e.offset_0x54 { e.offset_0x58 } else { e.offset_0x54 };
                e.offset_0x58 = -1;
            }
            g.edit(this).offset_0x54 += add.len() as i32;
            big_change = true;
        }
    } else if c == 0x1a {
        let e = g.edit(this);
        e.offset_0x98 = -1;
        let swap = e.field_0x4.clone();
        let swap_hilite = e.offset_0x58;
        let swap_cursor = e.offset_0x54;
        e.field_0x4 = e.field_0x74.clone();
        e.offset_0x54 = e.offset_0x90;
        e.offset_0x58 = e.offset_0x94;
        e.field_0x74 = swap;
        e.offset_0x90 = swap_cursor;
        e.offset_0x94 = swap_hilite;
        remove_hilite = false;
    } else if key == 0x25 {
        if control {
            while 0 < g.edit(this).offset_0x54 {
                let p = g.edit(this).offset_0x54 - 1;
                let b = ch(g, this, p);
                if vcall!(g, this, edit.vfunction75, b) {
                    break;
                }
                g.edit(this).offset_0x54 -= 1;
            }
            while 0 < g.edit(this).offset_0x54 {
                let p = g.edit(this).offset_0x54 - 1;
                let b = ch(g, this, p);
                if !vcall!(g, this, edit.vfunction75, b) {
                    break;
                }
                g.edit(this).offset_0x54 -= 1;
            }
        } else if !shift && hil != -1 {
            g.edit(this).offset_0x54 = if hil <= cur { hil } else { cur };
        } else {
            g.edit(this).offset_0x54 = cur - 1;
        }
    } else if key == 0x27 {
        if control {
            loop {
                let len = g.edit(this).field_0x4.len() as i32;
                if !(g.edit(this).offset_0x54 < len - 1) {
                    break;
                }
                let p = g.edit(this).offset_0x54 + 1;
                let b = ch(g, this, p);
                if !vcall!(g, this, edit.vfunction75, b) {
                    break;
                }
                g.edit(this).offset_0x54 += 1;
            }
            loop {
                let len = g.edit(this).field_0x4.len() as i32;
                if !(g.edit(this).offset_0x54 < len - 1) {
                    break;
                }
                let p = g.edit(this).offset_0x54 + 1;
                let b = ch(g, this, p);
                if vcall!(g, this, edit.vfunction75, b) {
                    break;
                }
                g.edit(this).offset_0x54 += 1;
            }
        }
        let hil = g.edit(this).offset_0x58;
        if !shift && hil != -1 {
            let cur = g.edit(this).offset_0x54;
            g.edit(this).offset_0x54 = if hil < cur { cur } else { hil };
        } else {
            g.edit(this).offset_0x54 += 1;
        }
    } else if key == 8 {
        if !g.edit(this).field_0x4.is_empty() {
            if hil != -1 && cur != hil {
                delete_selection(g, this, cur, hil);
                big_change = true;
            } else {
                let s = g.edit(this).field_0x4.clone();
                if cur < 1 {
                    g.edit(this).field_0x4 = substr(&s, cur, -1);
                } else {
                    let mut n = substr(&s, 0, cur - 1);
                    n.extend(substr(&s, cur, -1));
                    g.edit(this).field_0x4 = n;
                }
                let e = g.edit(this);
                e.offset_0x54 -= 1;
                e.offset_0x58 = -1;
                if e.offset_0x54 != e.offset_0x98 {
                    big_change = true;
                }
                e.offset_0x98 = e.offset_0x54 - 1;
            }
        }
    } else if key == 0x2e {
        let len = g.edit(this).field_0x4.len() as i32;
        if len != 0 {
            if hil != -1 && cur != hil {
                delete_selection(g, this, cur, hil);
                big_change = true;
            } else {
                if cur < len {
                    let s = g.edit(this).field_0x4.clone();
                    let mut n = substr(&s, 0, cur);
                    n.extend(substr(&s, cur + 1, -1));
                    g.edit(this).field_0x4 = n;
                }
                let e = g.edit(this);
                if e.offset_0x54 != e.offset_0x98 {
                    big_change = true;
                }
                e.offset_0x98 = e.offset_0x54;
            }
        }
    } else if key == 0x24 {
        g.edit(this).offset_0x54 = 0;
    } else if key == 0x23 {
        g.edit(this).offset_0x54 = g.edit(this).field_0x4.len() as i32;
    } else if key == 0xd {
        let s = g.edit(this).field_0x4.clone();
        vcall!(g, listener, el.vfunction1, id, &s);
    } else {
        let one = [c];
        let range: u32 = if g.sab(g.globals.DAT_005eb6a4).field_0xed { 0xff } else { 0x7f };
        let uc = c as i8 as i32 as u32;
        let f = g.edit(this).offset_0x3c;
        if uc < 0x20 || range < uc || font::string_width(g, f, &one) < 1 || !vcall!(g, listener, el.vfunction3, id, c) {
            remove_hilite = false;
        } else {
            let s = g.edit(this).field_0x4.clone();
            let (cur, hil) = (g.edit(this).offset_0x54, g.edit(this).offset_0x58);
            if hil == -1 || cur == hil {
                let mut n = substr(&s, 0, cur);
                n.push(c);
                n.extend(substr(&s, cur, -1));
                g.edit(this).field_0x4 = n;
                let e = g.edit(this);
                if e.offset_0x54 != e.offset_0x98 + 1 {
                    big_change = true;
                }
                e.offset_0x98 = e.offset_0x54;
            } else {
                let (lo, hi) = if hil <= cur { (hil, cur) } else { (cur, hil) };
                let mut n = substr(&s, 0, lo);
                n.push(c);
                n.extend(substr(&s, hi, -1));
                g.edit(this).field_0x4 = n;
                let e = g.edit(this);
                e.offset_0x54 = if e.offset_0x58 <= e.offset_0x54 { e.offset_0x58 } else { e.offset_0x54 };
                big_change = true;
            }
            let e = g.edit(this);
            e.offset_0x54 += 1;
            e.offset_0x58 = -1;
            vcall!(g, this, edit.vfunction77, false);
        }
    }
    let max_chars = g.edit(this).offset_0x68;
    if max_chars != -1 && max_chars < g.edit(this).field_0x4.len() as i32 {
        let s = g.edit(this).field_0x4.clone();
        g.edit(this).field_0x4 = substr(&s, 0, max_chars);
    }
    FUN_00473730(g, this);
    let e = g.edit(this);
    if e.offset_0x54 < 0 {
        e.offset_0x54 = 0;
    } else if (e.field_0x4.len() as i32) < e.offset_0x54 {
        e.offset_0x54 = e.field_0x4.len() as i32;
    }
    if old_cursor != e.offset_0x54 {
        e.offset_0x5c = 0;
        e.offset_0x50 = true;
    }
    vcall!(g, this, edit.vfunction77, true);
    let e = g.edit(this);
    if remove_hilite || e.offset_0x58 == e.offset_0x54 {
        e.offset_0x58 = -1;
    }
    let s = g.edit(this).field_0x4.clone();
    if !vcall!(g, listener, el.vfunction4, id, &s) {
        let e = g.edit(this);
        e.field_0x4 = old_string;
        e.offset_0x54 = old_cursor;
        e.offset_0x58 = old_hilite;
    } else if big_change {
        let e = g.edit(this);
        e.field_0x74 = old_string;
        e.offset_0x90 = old_cursor;
        e.offset_0x94 = old_hilite;
    }
    vcall!(g, this, w.vfunction18);
}

/// Deletes the selection between `cur` and `hil` (the shared tail of Cut, Backspace and
/// Delete): the cursor goes to its start.
fn delete_selection(g: &mut G, this: Ptr, cur: i32, hil: i32) {
    let s = g.edit(this).field_0x4.clone();
    let (lo, hi) = if hil <= cur { (hil, cur) } else { (cur, hil) };
    let mut n = substr(&s, 0, lo);
    n.extend(substr(&s, hi, -1));
    g.edit(this).field_0x4 = n;
    let e = g.edit(this);
    let (h, c) = (e.offset_0x58, e.offset_0x54);
    e.offset_0x58 = -1;
    e.offset_0x54 = if h <= c { h } else { c };
}

/// port: 004747a0 Sexy::EditWidget::vfunction49
/// `KeyDown(KeyCode)`: keys other than 'A'..'Y' go to `ProcessKey` when the listener allows.
pub fn vfunction49(g: &mut G, this: Ptr, key: i32) {
    if key < 0x41 || 0x59 < key {
        let (id, listener) = (g.edit(this).offset_0x0, g.edit(this).offset_0x4c);
        if vcall!(g, listener, el.vfunction2, id, key) {
            vcall!(g, this, edit.vfunction71, key, 0);
        }
    }
    crate::sexy::widget::vfunction49(g, this, key);
}

/// port: 004747f0 Sexy::EditWidget::vfunction48
/// `KeyChar(char)`.
pub fn vfunction48(g: &mut G, this: Ptr, c: u8) {
    vcall!(g, this, edit.vfunction71, 0, c);
}

/// port: 00474810 Sexy::EditWidget::vfunction76
/// `GetCharAt(int x, int y)`: the character boundary nearest x.
pub fn vfunction76(g: &mut G, this: Ptr, x: i32, _y: i32) -> i32 {
    let mut pos = 0;
    let s = FUN_00472f20(g, this);
    let left = g.edit(this).offset_0x64;
    let f = g.edit(this).offset_0x3c;
    let mut i = left;
    while i < s.len() as i32 {
        let lo = substr(&s, left, i - left);
        let hi = substr(&s, left, (i - left) + 1);
        let lo_w = font::string_width(g, f, &lo);
        let hi_w = font::string_width(g, f, &hi);
        if (hi_w + lo_w) / 2 + 5 <= x {
            pos = i + 1;
        }
        i += 1;
    }
    pos
}

/// port: 00474960 Sexy::EditWidget::vfunction77
/// `FocusCursor(bool bigJump)`: scrolls `mLeftPos` (by 1 or 10) until the cursor is visible.
pub fn vfunction77(g: &mut G, this: Ptr, big_jump: bool) {
    while g.edit(this).offset_0x54 < g.edit(this).offset_0x64 {
        let lp = g.edit(this).offset_0x64;
        let v = if big_jump { lp - 10 } else { lp - 1 };
        g.edit(this).offset_0x64 = v.max(0);
        vcall!(g, this, w.vfunction18);
    }
    if g.edit(this).offset_0x3c == NULL {
        return;
    }
    let s = FUN_00472f20(g, this);
    loop {
        let w = g.wc(this).offset_0x34;
        if w == 8 || w - 8 < 0 {
            return;
        }
        let e = g.edit(this).clone();
        let f = e.offset_0x3c;
        let left = font::string_width(g, f, &substr(&s, 0, e.offset_0x64));
        let cur = font::string_width(g, f, &substr(&s, 0, e.offset_0x54));
        if cur - left < w - 8 {
            return;
        }
        let lp = if big_jump { e.offset_0x64 + 10 } else { e.offset_0x64 + 1 };
        let last = e.field_0x4.len() as i32 - 1;
        g.edit(this).offset_0x64 = if lp < last { lp } else { last };
        vcall!(g, this, w.vfunction18);
    }
}

/// port: 00474b50 Sexy::EditWidget::vfunction54
/// `MouseDown(int x, int y, int theBtnNum, int theClickCount)`: places the cursor; a double
/// click selects the word.
pub fn vfunction54(g: &mut G, this: Ptr, x: i32, y: i32, _btn: i32, clicks: i32) {
    g.edit(this).offset_0x58 = -1;
    let p = vcall!(g, this, edit.vfunction76, x, y);
    g.edit(this).offset_0x54 = p;
    if 1 < clicks {
        g.edit(this).offset_0x52 = true;
        vcall!(g, this, edit.vfunction72);
    }
    vcall!(g, this, w.vfunction18);
    vcall!(g, this, edit.vfunction77, false);
}

/// port: 00474bc0 Sexy::EditWidget::vfunction56
/// `MouseUp(int x, int y, int theBtnNum, int theClickCount)`.
pub fn vfunction56(g: &mut G, this: Ptr, x: i32, y: i32, _btn: i32, _clicks: i32) {
    if g.edit(this).offset_0x58 == g.edit(this).offset_0x54 {
        g.edit(this).offset_0x58 = -1;
    }
    if g.edit(this).offset_0x52 {
        g.edit(this).offset_0x58 = -1;
        let p = vcall!(g, this, edit.vfunction76, x, y);
        g.edit(this).offset_0x54 = p;
        g.edit(this).offset_0x52 = false;
        vcall!(g, this, edit.vfunction72);
    }
    vcall!(g, this, w.vfunction18);
}

/// port: 00474c30 Sexy::EditWidget::vfunction72
/// `HiliteWord()`: selects the word around the cursor.
pub fn vfunction72(g: &mut G, this: Ptr) {
    let s = FUN_00472f20(g, this);
    let len = s.len() as i32;
    let at = |i: i32| s.get(i as usize).copied().unwrap_or(0);
    if g.edit(this).offset_0x54 < len {
        g.edit(this).offset_0x58 = g.edit(this).offset_0x54;
        while 0 < g.edit(this).offset_0x58 {
            let b = at(g.edit(this).offset_0x58 - 1);
            if !vcall!(g, this, edit.vfunction75, b) {
                break;
            }
            g.edit(this).offset_0x58 -= 1;
        }
        while g.edit(this).offset_0x54 < len - 1 {
            let b = at(g.edit(this).offset_0x54 + 1);
            if !vcall!(g, this, edit.vfunction75, b) {
                break;
            }
            g.edit(this).offset_0x54 += 1;
        }
        if g.edit(this).offset_0x54 < len {
            g.edit(this).offset_0x54 += 1;
        }
    }
}

/// port: 00474d20 Sexy::EditWidget::vfunction59
/// `MouseDrag(int x, int y)`: extends the selection.
pub fn vfunction59(g: &mut G, this: Ptr, x: i32, y: i32) {
    if g.edit(this).offset_0x58 == -1 {
        g.edit(this).offset_0x58 = g.edit(this).offset_0x54;
    }
    let p = vcall!(g, this, edit.vfunction76, x, y);
    g.edit(this).offset_0x54 = p;
    vcall!(g, this, w.vfunction18);
    vcall!(g, this, edit.vfunction77, false);
}

/// port: 00474d70 Sexy::EditWidget::vfunction51
/// `MouseEnter()`: the text cursor (`mWidgetManager->mApp->SetCursor(CURSOR_TEXT)`).
pub fn vfunction51(g: &mut G, this: Ptr) {
    let wm = g.wc(this).offset_0xc;
    let app = crate::sexy::widget_manager::wm(g, wm).offset_0x8;
    crate::sexy::sexy_app_base::FUN_004891c0(g, app, 3);
}

/// port: 00474d80 Sexy::EditWidget::vfunction52
/// `MouseLeave()`: the arrow (`SetCursor(CURSOR_POINTER)`).
pub fn vfunction52(g: &mut G, this: Ptr) {
    let wm = g.wc(this).offset_0xc;
    let app = crate::sexy::widget_manager::wm(g, wm).offset_0x8;
    crate::sexy::sexy_app_base::FUN_004891c0(g, app, 0);
}

/// port: 00474d90 Sexy::EditWidget::vfunction18
/// `MarkDirty()`: a full redraw when the background is not opaque.
pub fn vfunction18(g: &mut G, this: Ptr) {
    if g.w(this).offset_0xc[0].mAlpha != 0xff {
        crate::sexy::widget_container::vfunction20(g, this);
        return;
    }
    crate::sexy::widget_container::vfunction18(g, this);
}
