//! `Sexy::Checkbox` (framework, 0x00474dd0..0x00475050): an on/off box drawn from an
//! unchecked / checked image pair (or two rects of one image, or plain rectangles), that
//! reports `CheckboxChecked(id, checked)` to its CheckboxListener. `Checkbox_data` starts at
//! object offset 0x88 (the object is 0xec bytes; the database's 68-byte type stops before the
//! two colors at +0xcc and +0xdc).

use crate::sexy::graphics::{FUN_00455890, FUN_00455920, FUN_00455c50, FUN_00455d20, FUN_00455e40};
use crate::sexy::prelude::*;

/// `Checkbox_data` (object offset 0x88).
#[derive(Debug, Clone, Default)]
pub struct Checkbox_data {
    /// +0x88 `mListener`.
    pub offset_0x0: Ptr,
    /// +0x8c `mId`.
    pub offset_0x4: i32,
    /// +0x90 `mChecked`.
    pub offset_0x8: bool,
    /// +0x94 `mUncheckedImage`.
    pub offset_0xc: Ptr,
    /// +0x98 `mCheckedImage`.
    pub offset_0x10: Ptr,
    /// +0x9c `mCheckedRect` (in `mUncheckedImage`).
    pub offset_0x14: Rect,
    /// +0xac `mUncheckedRect`.
    pub offset_0x24: Rect,
    /// +0xbc `mOutlineColor`.
    pub offset_0x34: Color,
    /// +0xcc `mBkgColor`.
    pub field_0x44: Color,
    /// +0xdc `mCheckColor`.
    pub field_0x54: Color,
}

impl G {
    pub fn checkbox(&mut self, p: Ptr) -> &mut Checkbox_data {
        match &mut self.widget(p).ext {
            WExt::Checkbox(d) => d,
            e => panic!("{p} is not a Checkbox: {e:?}"),
        }
    }
}

/// port: 00474dd0 Sexy::Checkbox::Checkbox
/// `Checkbox(Image* theUncheckedImage, Image* theCheckedImage, int theId,
/// CheckboxListener*)`: unchecked, white outline, grey background, yellow check; the finger
/// cursor.
pub fn Checkbox(g: &mut G, param_1: Ptr, param_2: Ptr, param_3: i32, param_4: Ptr) -> Ptr {
    let (wc, mut w) = crate::sexy::widget::Widget();
    let d = Checkbox_data {
        offset_0x0: param_4,
        offset_0x4: param_3,
        offset_0xc: param_1,
        offset_0x10: param_2,
        offset_0x8: false,
        offset_0x14: Rect::new(0, 0, 0, 0),
        offset_0x24: Rect::new(0, 0, 0, 0),
        offset_0x34: Color::WHITE,
        field_0x44: FUN_00433360(0x50, 0x50, 0x50),
        field_0x54: FUN_00433360(0xff, 0xff, 0),
    };
    w.offset_0x28 = true;
    g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__Checkbox_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::Checkbox(Box::new(d)) }),
    })
}

/// port: 00474ea0 Sexy::Checkbox::vfunction71
/// `SetChecked(bool checked, bool tellListener)`.
pub fn vfunction71(g: &mut G, this: Ptr, param_1: bool, param_2: bool) {
    g.checkbox(this).offset_0x8 = param_1;
    let d = g.checkbox(this).clone();
    if param_2 && d.offset_0x0 != NULL {
        vcall!(g, d.offset_0x0, cl.vfunction1, d.offset_0x4, param_1);
    }
    vcall!(g, this, w.vfunction18);
}

/// port: 00474ee0 Sexy::Checkbox::vfunction72
/// `IsChecked()`.
pub fn vfunction72(g: &mut G, this: Ptr) -> bool {
    g.checkbox(this).offset_0x8
}

/// port: 00474ef0 Sexy::Checkbox::vfunction27
/// `Draw(Graphics*)`: the checked or unchecked image (or rect of the unchecked image);
/// with no images an outlined box with a cross when checked.
pub fn vfunction27(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    let d = g.checkbox(this).clone();
    if d.offset_0x14.mWidth == 0 {
        if d.offset_0x10 != NULL && d.offset_0xc != NULL {
            let img = if d.offset_0x8 { d.offset_0x10 } else { d.offset_0xc };
            FUN_00455d20(gfx, g, img, 0, 0);
            return;
        }
    } else if d.offset_0xc != NULL {
        let r = if d.offset_0x8 { d.offset_0x14 } else { d.offset_0x24 };
        FUN_00455e40(gfx, g, d.offset_0xc, 0, 0, &r);
        return;
    }
    if d.offset_0xc == NULL && d.offset_0x10 == NULL {
        let (w, h) = (g.wc(this).offset_0x34, g.wc(this).offset_0x38);
        FUN_00455890(gfx, d.offset_0x34);
        FUN_00455920(gfx, 0, 0, w, h);
        FUN_00455890(gfx, d.field_0x44);
        FUN_00455920(gfx, 1, 1, w - 2, h - 2);
        if d.offset_0x8 {
            FUN_00455890(gfx, d.field_0x54);
            FUN_00455c50(gfx, 1, 1, w - 2, h - 2);
            FUN_00455c50(gfx, w - 1, 1, 1, h - 2);
        }
    }
}

/// port: 00475050 Sexy::Checkbox::vfunction54
/// `MouseDown(int x, int y, int theBtnNum, int theClickCount)`: toggles and reports.
pub fn vfunction54(g: &mut G, this: Ptr, _x: i32, _y: i32, _btn: i32, _clicks: i32) {
    let checked = !g.checkbox(this).offset_0x8;
    g.checkbox(this).offset_0x8 = checked;
    let d = g.checkbox(this).clone();
    if d.offset_0x0 != NULL {
        vcall!(g, d.offset_0x0, cl.vfunction1, d.offset_0x4, checked);
    }
    vcall!(g, this, w.vfunction18);
}
