//! `Sexy::Slider` (framework, 0x0046dd30..0x0046fa80): a track image (three cels: two ends
//! and a tiled middle) and a thumb image dragged along it; reports `SliderVal(id, value)` to
//! its SliderListener. `Slider_data` starts at object offset 0x88.

use crate::sexy::graphics::{FUN_00455800, FUN_00455d20, FUN_00455e40, FUN_00456340, FUN_00457480, FUN_00457490};
use crate::sexy::prelude::*;

/// `Slider_data` (object offset 0x88, 44 bytes).
#[derive(Debug, Clone, Default)]
pub struct Slider_data {
    /// +0x88 `mListener` (the object whose SliderListener vftable receives the value).
    pub offset_0x0: Ptr,
    /// +0x90 `mVal` (0..1).
    pub offset_0x8: f64,
    /// +0x98 `mId`.
    pub offset_0x10: i32,
    /// +0x9c `mTrackImage`.
    pub offset_0x14: Ptr,
    /// +0xa0 `mThumbImage`.
    pub offset_0x18: Ptr,
    /// +0xa4 `mDragging`.
    pub offset_0x1c: bool,
    /// +0xa8 `mRelX` (where in the thumb the drag started).
    pub offset_0x20: i32,
    /// +0xac `mRelY`.
    pub offset_0x24: i32,
    /// +0xb0 `mHorizontal`.
    pub offset_0x28: bool,
}

impl G {
    pub fn slider(&mut self, p: Ptr) -> &mut Slider_data {
        match &mut self.widget(p).ext {
            WExt::Slider(d) => d,
            e => panic!("{p} is not a Slider: {e:?}"),
        }
    }
}

fn app_of(g: &mut G, this: Ptr) -> Ptr {
    let wm = g.wc(this).offset_0xc;
    crate::sexy::widget_manager::wm(g, wm).offset_0x8
}

/// port: 0046f360 Sexy::Slider::Slider
/// `Slider(Image* theTrackImage, Image* theThumbImage, int theId, SliderListener*)`:
/// horizontal, at 0.
pub fn Slider(g: &mut G, param_1: Ptr, param_2: Ptr, param_3: i32, param_4: Ptr) -> Ptr {
    let (wc, w) = crate::sexy::widget::Widget();
    let d = Slider_data {
        offset_0x8: 0.0,
        offset_0x0: param_4,
        offset_0x18: param_2,
        offset_0x1c: false,
        offset_0x24: 0,
        offset_0x20: 0,
        offset_0x10: param_3,
        offset_0x14: param_1,
        offset_0x28: true,
    };
    g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__Slider_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::Slider(Box::new(d)) }),
    })
}

/// port: 0046dd30 Sexy::Slider::deleting_destructor
pub fn deleting_destructor(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    crate::sexy::widget::dtor_Widget(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 0046f3c0 Sexy::Slider::vfunction71
/// `SetValue(double)`: clamped to 0..1, then redrawn.
pub fn vfunction71(g: &mut G, this: Ptr, param_1: f64) {
    let v = if param_1 < 0.0 {
        0.0
    } else if 1.0 < param_1 {
        1.0
    } else {
        param_1
    };
    g.slider(this).offset_0x8 = v;
    vcall!(g, this, w.vfunction20);
}

/// The thumb's offset along the track for the current value.
fn thumb_at(g: &mut G, this: Ptr) -> (i32, i32) {
    let d = g.slider(this).clone();
    let (w, h) = (g.wc(this).offset_0x34, g.wc(this).offset_0x38);
    if d.offset_0x28 {
        let tw = FUN_00457480(g, d.offset_0x18);
        (crate::sexy::crt::ftol((w - tw) as f64 * d.offset_0x8) as i32, tw)
    } else {
        let th = FUN_00457490(g, d.offset_0x18);
        (crate::sexy::crt::ftol((h - th) as f64 * d.offset_0x8) as i32, th)
    }
}

/// port: 0046f410 Sexy::Slider::vfunction27
/// `Draw(Graphics*)`: the track (end cels and the tiled middle, clipped), then the thumb at
/// the value.
pub fn vfunction27(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    let d = g.slider(this).clone();
    let (w, h) = (g.wc(this).offset_0x34, g.wc(this).offset_0x38);
    let track = d.offset_0x14;
    if track != NULL {
        let tw = if d.offset_0x28 { FUN_00457480(g, track) / 3 } else { FUN_00457480(g, track) };
        let th = if d.offset_0x28 { FUN_00457490(g, track) } else { FUN_00457490(g, track) / 3 };
        if d.offset_0x28 {
            let y0 = (h - th) / 2;
            FUN_00455e40(gfx, g, track, 0, y0, &Rect::new(0, 0, tw, th));
            let mut g2 = FUN_00455800(gfx);
            FUN_00456340(&mut g2, tw, y0, w - tw * 2, th);
            let n = (w - tw - 1) / tw;
            let mut x = tw;
            for _ in 0..n {
                FUN_00455e40(&mut g2, g, track, x, y0, &Rect::new(tw, 0, tw, th));
                x += tw;
            }
            FUN_00455e40(gfx, g, track, w - tw, y0, &Rect::new(tw * 2, 0, tw, th));
        } else {
            FUN_00455e40(gfx, g, track, 0, 0, &Rect::new(0, 0, tw, th));
            let mut g2 = FUN_00455800(gfx);
            FUN_00456340(&mut g2, 0, th, tw, h - th * 2);
            let n = (h - th - 1) / th;
            let mut y = th;
            for _ in 0..n {
                FUN_00455e40(&mut g2, g, track, 0, y, &Rect::new(0, th, tw, th));
                y += th;
            }
            FUN_00455e40(gfx, g, track, 0, h - th, &Rect::new(0, th * 2, tw, th));
        }
    }
    let thumb = d.offset_0x18;
    if thumb == NULL {
        return;
    }
    if d.offset_0x28 {
        let y = (h - FUN_00457490(g, thumb)) / 2;
        let x = crate::sexy::crt::ftol((w - FUN_00457480(g, thumb)) as f64 * d.offset_0x8) as i32;
        FUN_00455d20(gfx, g, thumb, x, y);
    } else {
        let y = crate::sexy::crt::ftol((h - FUN_00457490(g, thumb)) as f64 * d.offset_0x8) as i32;
        let x = (w - FUN_00457480(g, thumb)) / 2;
        FUN_00455d20(gfx, g, thumb, x, y);
    }
}

/// port: 0046f770 Sexy::Slider::vfunction55
/// `MouseDown(int x, int y, int theClickCount)`: on the thumb starts dragging (the drag
/// cursor); elsewhere jumps to the clicked spot.
pub fn vfunction55(g: &mut G, this: Ptr, param_1: i32, param_2: i32, _param_3: i32) {
    let horiz = g.slider(this).offset_0x28;
    let size = if horiz { g.wc(this).offset_0x34 } else { g.wc(this).offset_0x38 };
    let at = if horiz { param_1 } else { param_2 };
    let (t, len) = thumb_at(g, this);
    if t <= at && at < len + t {
        let app = app_of(g, this);
        crate::sexy::sexy_app_base::FUN_004891c0(g, app, 2);
        let d = g.slider(this);
        d.offset_0x1c = true;
        if horiz {
            d.offset_0x20 = param_1 - t;
        } else {
            d.offset_0x24 = param_2 - t;
        }
        return;
    }
    vfunction71(g, this, at as f64 / size as f64);
}

/// port: 0046f880 Sexy::Slider::vfunction53
/// `MouseMove(int x, int y)`: the drag cursor over the thumb, else the arrow.
pub fn vfunction53(g: &mut G, this: Ptr, param_1: i32, param_2: i32) {
    let horiz = g.slider(this).offset_0x28;
    let at = if horiz { param_1 } else { param_2 };
    let (t, len) = thumb_at(g, this);
    let app = app_of(g, this);
    let c = if t <= at && at < len + t { 2 } else { 0 };
    crate::sexy::sexy_app_base::FUN_004891c0(g, app, c);
}

/// port: 0046f950 Sexy::Slider::vfunction59
/// `MouseDrag(int x, int y)`: while dragging, the value follows the thumb (clamped); a
/// change is reported and redrawn.
pub fn vfunction59(g: &mut G, this: Ptr, param_1: i32, param_2: i32) {
    let d = g.slider(this).clone();
    if !d.offset_0x1c {
        return;
    }
    let old = d.offset_0x8;
    let v = if d.offset_0x28 {
        let tw = FUN_00457480(g, d.offset_0x18);
        (param_1 - d.offset_0x20) as f64 / (g.wc(this).offset_0x34 - tw) as f64
    } else {
        let th = FUN_00457490(g, d.offset_0x18);
        (param_2 - d.offset_0x24) as f64 / (g.wc(this).offset_0x38 - th) as f64
    };
    let v = if v < 0.0 { 0.0 } else { v };
    let v = if 1.0 < v { 1.0 } else { v };
    g.slider(this).offset_0x8 = v;
    if old != v {
        vcall!(g, d.offset_0x0, sl.vfunction1, d.offset_0x10, v);
        vcall!(g, this, w.vfunction20);
    }
}

/// port: 0046fa40 Sexy::Slider::vfunction58
/// `MouseUp(int x, int y)`: the drag ends (the arrow), and the value is reported.
pub fn vfunction58(g: &mut G, this: Ptr, _param_1: i32, _param_2: i32) {
    g.slider(this).offset_0x1c = false;
    let app = app_of(g, this);
    crate::sexy::sexy_app_base::FUN_004891c0(g, app, 0);
    let d = g.slider(this).clone();
    vcall!(g, d.offset_0x0, sl.vfunction1, d.offset_0x10, d.offset_0x8);
}

/// port: 0046fa80 Sexy::Slider::vfunction52
/// `MouseLeave()`: the arrow unless dragging.
pub fn vfunction52(g: &mut G, this: Ptr) {
    if !g.slider(this).offset_0x1c {
        let app = app_of(g, this);
        crate::sexy::sexy_app_base::FUN_004891c0(g, app, 0);
    }
}
