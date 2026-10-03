//! `Sexy::DialogButton` (framework, 0x00476eb0..0x00477350): a `ButtonWidget` drawn from a
//! three-cel "component image" (normal / over / down) stretched as a 9-slice box, with the
//! label centered on it. `DialogButton_data` starts at object offset 0x120.

use crate::sexy::button_widget::{BtnSub, ButtonExt};
use crate::sexy::graphics::{FUN_00455880, FUN_00455890, FUN_004558e0, FUN_00455cf0, FUN_004563d0, FUN_004563f0, FUN_00456430};
use crate::sexy::prelude::*;

/// `DialogButton_data` (object offset 0x120, 20 bytes).
#[derive(Debug, Clone, Default)]
pub struct DialogButton_data {
    /// +0x120 `mComponentImage`.
    pub offset_0x0: Ptr,
    /// +0x124 `mTranslateX` (label/box shift while pressed).
    pub offset_0x4: i32,
    /// +0x128 `mTranslateY`.
    pub offset_0x8: i32,
    /// +0x12c `mTextOffsetX`.
    pub offset_0xc: i32,
    /// +0x130 `mTextOffsetY`.
    pub offset_0x10: i32,
}

impl G {
    pub fn dialog_button(&mut self, p: Ptr) -> &mut DialogButton_data {
        match &mut self.widget(p).ext {
            WExt::Button(e) => match &mut e.sub {
                BtnSub::DialogButton(d) => d,
                s => panic!("{p} is not a DialogButton: {s:?}"),
            },
            e => panic!("{p} is not a ButtonWidget: {e:?}"),
        }
    }
}

/// `gDialogButtonColors` (@ 005e73b8): label, label hilite, dark outline, light outline,
/// medium outline, background.
const DIALOG_BUTTON_COLORS: [[i32; 3]; 6] = [[255, 255, 255], [255, 255, 255], [0, 0, 0], [255, 255, 255], [132, 132, 132], [212, 212, 212]];

/// port: 00476eb0 Sexy::DialogButton::DialogButton
/// `DialogButton(Image* theComponentImage, int theId, ButtonListener*)`.
pub fn DialogButton(g: &mut G, param_1: Ptr, param_2: i32, param_3: Ptr) -> Ptr {
    let (wc, w, b) = crate::sexy::button_widget::ButtonWidget(param_2, param_3);
    let d = DialogButton_data { offset_0x0: param_1, offset_0x4: 1, offset_0x8: 1, offset_0xc: 0, offset_0x10: 0 };
    let this = g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__DialogButton_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::Button(Box::new(ButtonExt { b, sub: BtnSub::DialogButton(d) })) }),
    });
    g.w(this).offset_0x28 = true;
    crate::sexy::widget::vfunction34(g, this, &DIALOG_BUTTON_COLORS);
    this
}

/// port: 00476f50 Sexy::DialogButton::deleting_destructor
pub fn deleting_destructor(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    crate::sexy::button_widget::dtor_ButtonWidget(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 00476f70 Sexy::DialogButton::vfunction27
/// `Draw(Graphics*)`: plain `ButtonWidget::Draw` without a component image; otherwise the
/// box (whole image, or the disabled/down/over/normal cel rect, the over cel faded in by
/// `mOverAlpha`), then the label. Pressed buttons are shifted by `mTranslateX/Y`.
pub fn vfunction27(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    if g.btn(this).offset_0x79 {
        return;
    }
    if g.dialog_button(this).offset_0x0 == NULL {
        crate::sexy::button_widget::vfunction27(g, this, gfx);
        return;
    }
    if g.btn(this).offset_0x24 == NULL && !g.btn(this).field_0x4.is_empty() {
        // `new SysFont(theApp, "Arial Unicode MS", 12, true)`: a Windows GDI font. Every
        // DialogButton the game creates gets an ImageFont first, so this is never reached.
        crate::sexy::pending(0x00476f70, "DialogButton::Draw fallback SysFont (Windows GDI font)");
    }
    let down = vcall!(g, this, btn.vfunction73);
    let (w, h) = (g.wc(this).offset_0x34, g.wc(this).offset_0x38);
    let image = g.dialog_button(this).offset_0x0;
    let (tx, ty) = (g.dialog_button(this).offset_0x4, g.dialog_button(this).offset_0x8);
    let dest = Rect::new(0, 0, w, h);
    if g.btn(this).offset_0x38.mWidth == 0 {
        if down {
            FUN_004563d0(gfx, tx, ty);
        }
        FUN_004563f0(gfx, g, &dest, image);
    } else {
        let b = g.btn(this).clone();
        let disabled = g.w(this).offset_0x2;
        if disabled && 0 < b.offset_0x68.mWidth && 0 < b.offset_0x68.mHeight {
            FUN_00456430(gfx, g, &b.offset_0x68, image, &dest);
        } else if vcall!(g, this, btn.vfunction73) {
            FUN_00456430(gfx, g, &b.offset_0x58, image, &dest);
        } else if 0.0 < b.offset_0x80 {
            if b.offset_0x80 < 1.0 {
                FUN_00456430(gfx, g, &b.offset_0x38, image, &dest);
            }
            FUN_004558e0(gfx, true);
            let a = crate::sexy::crt::ftol(b.offset_0x80 * 255.0) as i32;
            FUN_00455890(gfx, CRect(0xff, 0xff, 0xff, a));
            FUN_00456430(gfx, g, &b.offset_0x48, image, &dest);
            FUN_004558e0(gfx, false);
        } else if g.w(this).offset_0x5 {
            FUN_00456430(gfx, g, &b.offset_0x48, image, &dest);
        } else {
            FUN_00456430(gfx, g, &b.offset_0x38, image, &dest);
        }
        if down {
            FUN_004563d0(gfx, tx, ty);
        }
    }
    let font = g.btn(this).offset_0x24;
    if font != NULL {
        FUN_00455880(gfx, font);
        let idx = if g.w(this).offset_0x5 { 1 } else { 0 };
        let c = g.w(this).offset_0xc[idx];
        FUN_00455890(gfx, c);
        let label = g.btn(this).field_0x4.clone();
        let x = (w - crate::sexy::image_font::string_width(g, font, &label)) / 2;
        let asc = crate::sexy::image_font::get_ascent(g, font);
        let pad = crate::sexy::image_font::get_ascent_padding(g, font);
        let y = (crate::sexy::image_font::get_ascent(g, font) + (h - asc / 6 - pad - 1)) / 2;
        let (ox, oy) = (g.dialog_button(this).offset_0xc, g.dialog_button(this).offset_0x10);
        FUN_00455cf0(gfx, g, &label, ox + x, oy + y);
    }
    if down {
        FUN_004563d0(gfx, -tx, -ty);
    }
}
