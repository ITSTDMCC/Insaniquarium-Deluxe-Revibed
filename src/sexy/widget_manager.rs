//! `Sexy::WidgetManager` (framework, 0x0046ca40..0x0046dc90): the root container that turns
//! window input into widget events, runs the per-tick `UpdateAll`, and redraws dirty
//! widgets onto the screen image.
//!
//! Many of these helpers take their arguments in registers in the original (e.g.
//! `MouseDown` gets `this` in ECX, x on the stack, y in EAX, the click count in EBX); the
//! Rust signatures name them explicitly.

use crate::sexy::graphics::{FUN_00455900, FUN_00455910, FUN_004563d0, ImageCmds};
use crate::sexy::prelude::*;

/// `PreModalInfo` (a `mPreModalInfoList` node payload).
#[derive(Clone, Copy, Debug, Default)]
pub struct PreModalInfo {
    pub mBaseModalWidget: Ptr,
    pub mPrevBaseModalWidget: Ptr,
    pub mPrevFocusWidget: Ptr,
    /// `mPrevBelowModalFlagsMod` (add, remove).
    pub mPrevBelowModalFlagsMod: (u32, u32),
}

/// `WidgetManager_data` (object offset 0x54, 424 bytes).
#[derive(Debug, Clone)]
pub struct WidgetManager_data {
    /// +0x54 `mDefaultTab`.
    pub offset_0x0: Ptr,
    /// +0x58 `mCurG`: the screen Graphics while `DrawScreen` runs.
    pub offset_0x4: Option<Graphics>,
    /// +0x5c `mApp`.
    pub offset_0x8: Ptr,
    /// +0x60 `mImage`: the screen image (its draw commands go to `screen_cmds`).
    pub offset_0xc: Ptr,
    /// +0x64 `mTransientImage`.
    pub offset_0x10: Ptr,
    /// +0x68 `mLastHadTransients`.
    pub offset_0x14: bool,
    /// +0x6c `mPopupCommandWidget`.
    pub offset_0x18: Ptr,
    /// +0x70 `mDeferredOverlayWidgets` (vector<pair<Widget*, int priority>>).
    pub offset_0x1c: Vec<(Ptr, i32)>,
    /// +0x80 `mMinDeferredOverlayPriority`.
    pub offset_0x2c: i32,
    /// +0x84 `mHasFocus`.
    pub offset_0x30: bool,
    /// +0x88 `mFocusWidget`.
    pub offset_0x34: Ptr,
    /// +0x8c `mLastDownWidget`.
    pub offset_0x38: Ptr,
    /// +0x90 `mOverWidget`.
    pub offset_0x3c: Ptr,
    /// +0x94 `mBaseModalWidget`.
    pub offset_0x40: Ptr,
    /// +0x98 `mLostFocusFlagsMod` (add, remove).
    pub offset_0x44: (u32, u32),
    /// +0xa0 `mBelowModalFlagsMod` (add, remove).
    pub offset_0x4c: (u32, u32),
    /// +0xa8 `mDefaultBelowModalFlagsMod` (add, remove).
    pub offset_0x54: (u32, u32),
    /// +0xb0 `mPreModalInfoList` (front = most recent).
    pub offset_0x5c: Vec<PreModalInfo>,
    /// +0xbc `mMouseDestRect`.
    pub offset_0x68: Rect,
    /// +0xcc `mMouseSourceRect`.
    pub offset_0x78: Rect,
    /// +0xdc `mMouseIn`.
    pub offset_0x88: bool,
    /// +0xe0 `mLastMouseX`.
    pub offset_0x8c: i32,
    /// +0xe4 `mLastMouseY`.
    pub offset_0x90: i32,
    /// +0xe8 `mDownButtons`.
    pub offset_0x94: u32,
    /// +0xec `mActualDownButtons`.
    pub offset_0x98: u32,
    /// +0xf0 `mLastInputUpdateCnt`.
    pub offset_0x9c: i32,
    /// +0xf4 `mKeyDown[0xff]`.
    pub offset_0xa0: [bool; 0xff],
    /// +0x1f4 `mLastDownButtonId`.
    pub offset_0x1a0: i32,
    /// +0x1f8 `mWidgetFlags`.
    pub offset_0x1a4: u32,
    /// Port-side: where blits into `mImage` land (the persistent screen; see crate::render).
    pub screen_cmds: ImageCmds,
}

/// The `WidgetManager_data` of `p`.
pub fn wm(g: &mut G, p: Ptr) -> &mut WidgetManager_data {
    match &mut g.widget(p).ext {
        WExt::WidgetManager(d) => d,
        e => panic!("{p} is not a WidgetManager: {e:?}"),
    }
}

/// port: 0046ca40 Sexy::WidgetManager::WidgetManager
/// `WidgetManager(SexyAppBase* theApp)`: allocates the manager; it is its own manager.
pub fn WidgetManager(g: &mut G, param_2: Ptr, screen_image: Ptr, screen_cmds: ImageCmds) -> Ptr {
    let wc = crate::sexy::widget_container::WidgetContainer();
    let d = WidgetManager_data {
        offset_0x0: NULL,
        offset_0x4: None,
        offset_0x8: param_2,
        offset_0xc: screen_image,
        offset_0x10: NULL,
        offset_0x14: false,
        offset_0x18: NULL,
        offset_0x1c: Vec::new(),
        offset_0x2c: 0x7fff_ffff,
        offset_0x30: true,
        offset_0x34: NULL,
        offset_0x38: NULL,
        offset_0x3c: NULL,
        offset_0x40: NULL,
        offset_0x44: (0, 0),
        offset_0x4c: (0, 0),
        offset_0x54: (0, 0x30),
        offset_0x5c: Vec::new(),
        offset_0x68: Rect::default(),
        offset_0x78: Rect::default(),
        offset_0x88: false,
        offset_0x8c: 0,
        offset_0x90: 0,
        offset_0x94: 0,
        offset_0x98: 0,
        offset_0x9c: 0,
        offset_0xa0: [false; 0xff],
        offset_0x1a0: 0,
        offset_0x1a4: 0x3d,
        screen_cmds,
    };
    let p = g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__WidgetManager_vftable),
        node: Node::Widget(WidgetObj { wc, w: Default::default(), ext: WExt::WidgetManager(Box::new(d)) }),
    });
    g.wc(p).offset_0xc = p;
    g.wc(p).offset_0x24 = 0;
    p
}

/// port: 0046cbf0 Sexy::WidgetManager::vfunction7
/// `DisableWidget(Widget*)`: drops every reference the manager holds to it.
pub fn vfunction7(g: &mut G, this: Ptr, widget: Ptr, _disabled: bool) {
    if wm(g, this).offset_0x3c == widget {
        wm(g, this).offset_0x3c = NULL;
        FUN_0046d040(g, widget);
    }
    if wm(g, this).offset_0x38 == widget {
        wm(g, this).offset_0x38 = NULL;
        let buttons = wm(g, this).offset_0x94;
        FUN_0046cf40(g, this, widget, buttons);
        wm(g, this).offset_0x94 = 0;
    }
    let focus = wm(g, this).offset_0x34;
    if focus == widget {
        wm(g, this).offset_0x34 = NULL;
        vcall!(g, focus, w.vfunction47);
    }
    if wm(g, this).offset_0x40 == widget {
        wm(g, this).offset_0x40 = NULL;
    }
}

/// port: 0046cc70 FUN_0046cc70
/// `GetWidgetFlags()`: `mWidgetFlags`, modified by `mLostFocusFlagsMod` while unfocused.
pub fn FUN_0046cc70(g: &mut G, this: Ptr) -> u32 {
    let d = wm(g, this);
    let f = d.offset_0x1a4;
    if d.offset_0x30 { f } else { (f | d.offset_0x44.0) & !d.offset_0x44.1 }
}

/// port: 0046b830 FUN_0046b830
/// `WidgetContainer::GetWidgetAtHelper(x, y, flags, &found, &widgetX, &widgetY)`: the
/// topmost widget under the point that allows mouse input, searching children last to first.
pub fn FUN_0046b830(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: u32, param_4: &mut bool, param_5: &mut Option<(i32, i32)>) -> Ptr {
    let (add, remove) = (g.wc(this).offset_0x40, g.wc(this).offset_0x44);
    let manager = g.wc(this).offset_0xc;
    let mut below_modal = false;
    let mut it = g.wc(this).offset_0x4.end();
    loop {
        if it == g.wc(this).offset_0x4.begin() {
            break;
        }
        it = g.wc(this).offset_0x4.prev(it);
        let w = g.wc(this).offset_0x4.get(it);
        let mut flags = (g.wc(w).offset_0x40 | !remove & (add | param_3)) & !g.wc(w).offset_0x44;
        if below_modal {
            let m = wm(g, manager).offset_0x4c;
            flags = !m.1 & (m.0 | flags);
        }
        if flags & WIDGETFLAGS_ALLOW_MOUSE != 0 && g.w(w).offset_0x0 {
            let (wx, wy) = (g.wc(w).offset_0x2c, g.wc(w).offset_0x30);
            let mut found = false;
            let child = FUN_0046b830(g, w, param_1 - wx, param_2 - wy, flags, &mut found, param_5);
            if child != NULL || found {
                *param_4 = true;
                return child;
            }
            if g.w(w).offset_0x1 {
                let r = vcall!(g, w, w.vfunction70);
                if r.mX <= param_1 && param_1 < r.mWidth + r.mX && r.mY <= param_2 && param_2 < r.mHeight + r.mY {
                    *param_4 = true;
                    if vcall!(g, w, w.vfunction61, param_1 - wx, param_2 - wy) {
                        *param_5 = Some((param_1 - wx, param_2 - wy));
                        return w;
                    }
                }
            }
        }
        below_modal |= w == wm(g, manager).offset_0x40;
    }
    *param_4 = false;
    NULL
}

/// port: 0046cc90 FUN_0046cc90
/// `GetAnyWidgetAt(x, y, &widgetX, &widgetY)`.
pub fn FUN_0046cc90(g: &mut G, this: Ptr, x: i32, y: i32, out: &mut Option<(i32, i32)>) -> Ptr {
    let flags = FUN_0046cc70(g, this);
    let mut found = false;
    FUN_0046b830(g, this, x, y, flags, &mut found, out)
}

/// port: 0046ccc0 FUN_0046ccc0
/// `GetWidgetAt(x, y, &widgetX, &widgetY)`: like `GetAnyWidgetAt` but disabled widgets don't count.
pub fn FUN_0046ccc0(g: &mut G, this: Ptr, x: i32, y: i32, out: &mut Option<(i32, i32)>) -> Ptr {
    let w = FUN_0046cc90(g, this, x, y, out);
    if w != NULL && g.w(w).offset_0x2 { NULL } else { w }
}

/// port: 0046cce0 FUN_0046cce0
/// `IsLeftButtonDown()`.
pub fn FUN_0046cce0(g: &mut G, this: Ptr) -> bool {
    wm(g, this).offset_0x98 & 1 != 0
}

/// port: 0046ccf0 FUN_0046ccf0
/// Sends the pending mouse-ups to `mLastDownWidget` and forgets it.
pub fn FUN_0046ccf0(g: &mut G, this: Ptr) {
    let down = wm(g, this).offset_0x38;
    let buttons = wm(g, this).offset_0x94;
    if down != NULL && buttons != 0 {
        FUN_0046cf40(g, this, down, buttons);
        wm(g, this).offset_0x94 = 0;
        wm(g, this).offset_0x38 = NULL;
    }
}

/// port: 0046cd30 FUN_0046cd30
/// `FlushDeferredOverlayWidgets(int maxPriority)`: draws queued overlays lowest priority
/// first, up to `param_2`.
pub fn FUN_0046cd30(g: &mut G, param_1: Ptr, param_2: i32) {
    loop {
        let mut min = 0x7fff_ffffi32;
        let mut i = 0usize;
        while i < wm(g, param_1).offset_0x1c.len() {
            let (w, prio) = wm(g, param_1).offset_0x1c[i];
            if w != NULL {
                if prio == wm(g, param_1).offset_0x2c {
                    let cur = wm(g, param_1).offset_0x4.clone().expect("deferred overlay outside DrawScreen");
                    let mut gfx = Graphics { s: cur.s.clone(), mStateStack: Vec::new(), out: cur.out.clone() };
                    let dest = wm(g, param_1).offset_0x68;
                    FUN_004563d0(&mut gfx, -dest.mX, -dest.mY);
                    let (x, y) = (g.wc(w).offset_0x2c, g.wc(w).offset_0x30);
                    FUN_004563d0(&mut gfx, x, y);
                    let is3d = gfx.s.mIs3D;
                    FUN_00455900(&mut gfx, !is3d);
                    FUN_00455910(&mut gfx, is3d);
                    vcall!(g, w, w.vfunction44, &mut gfx, prio);
                    wm(g, param_1).offset_0x1c[i].0 = NULL;
                } else if prio < min {
                    min = prio;
                }
            }
            i += 1;
        }
        wm(g, param_1).offset_0x2c = min;
        if min == 0x7fff_ffff {
            wm(g, param_1).offset_0x1c.clear();
            return;
        }
        if param_2 <= min {
            return;
        }
    }
}

/// port: 0046cf40 FUN_0046cf40
/// `DoMouseUps(Widget*, ulong downCode)`: one `MouseUp` per button bit (left 1, right -1, middle 3).
pub fn FUN_0046cf40(g: &mut G, param_1: Ptr, widget: Ptr, down_code: u32) {
    let codes = [1, -1, 3];
    for (i, code) in codes.iter().enumerate() {
        if down_code & (1 << i) != 0 {
            g.w(widget).offset_0x4 = false;
            let (mx, my) = (wm(g, param_1).offset_0x8c, wm(g, param_1).offset_0x90);
            let (wx, wy) = (g.wc(widget).offset_0x2c, g.wc(widget).offset_0x30);
            vcall!(g, widget, w.vfunction57, mx - wx, my - wy, *code);
        }
    }
}

/// port: 0046cfb0 FUN_0046cfb0
/// `RemapMouse(int& x, int& y)`: maps window coordinates into the screen rect.
pub fn FUN_0046cfb0(g: &mut G, this: Ptr, x: &mut i32, y: &mut i32) {
    let d = wm(g, this);
    let (src, dst) = (d.offset_0x78, d.offset_0x68);
    if src.mWidth != 0 && src.mHeight != 0 {
        *x = ((*x - src.mX) * dst.mWidth) / src.mWidth + dst.mX;
        *y = ((*y - src.mY) * dst.mHeight) / src.mHeight + dst.mY;
    }
}

/// port: 0046d010 FUN_0046d010
/// `EnterWidget(Widget*)`: `mIsOver`, `MouseEnter`, and the hand cursor if `mDoFinger`.
pub fn FUN_0046d010(g: &mut G, widget: Ptr) {
    g.w(widget).offset_0x5 = true;
    vcall!(g, widget, w.vfunction51);
    if g.w(widget).offset_0x28 {
        vcall!(g, widget, w.vfunction39, true);
    }
}

/// port: 0046d040 FUN_0046d040
/// `LeaveWidget(Widget*)`.
pub fn FUN_0046d040(g: &mut G, widget: Ptr) {
    g.w(widget).offset_0x5 = false;
    vcall!(g, widget, w.vfunction52);
    if g.w(widget).offset_0x28 {
        vcall!(g, widget, w.vfunction39, false);
    }
}

/// port: 0046d070 FUN_0046d070
/// `SetBaseModal(Widget*, const FlagsMod& belowFlagsMod)`: drops hover/down/focus widgets
/// that are now below the modal widget.
pub fn FUN_0046d070(g: &mut G, this: Ptr, widget: Ptr, below: (u32, u32)) {
    wm(g, this).offset_0x40 = widget;
    wm(g, this).offset_0x4c = below;
    let over = wm(g, this).offset_0x3c;
    if over != NULL && below.1 & WIDGETFLAGS_ALLOW_MOUSE != 0 {
        let base = wm(g, this).offset_0x40;
        if vcall!(g, this, w.vfunction10, over, base) {
            let over = wm(g, this).offset_0x3c;
            wm(g, this).offset_0x3c = NULL;
            FUN_0046d040(g, over);
        }
    }
    let down = wm(g, this).offset_0x38;
    if down != NULL && wm(g, this).offset_0x4c.1 & WIDGETFLAGS_ALLOW_MOUSE != 0 {
        let base = wm(g, this).offset_0x40;
        if vcall!(g, this, w.vfunction10, down, base) {
            let down = wm(g, this).offset_0x38;
            let buttons = wm(g, this).offset_0x94;
            wm(g, this).offset_0x94 = 0;
            wm(g, this).offset_0x38 = NULL;
            FUN_0046cf40(g, this, down, buttons);
        }
    }
    let focus = wm(g, this).offset_0x34;
    if focus != NULL && wm(g, this).offset_0x4c.1 & WIDGETFLAGS_ALLOW_FOCUS != 0 {
        let base = wm(g, this).offset_0x40;
        if vcall!(g, this, w.vfunction10, focus, base) {
            let focus = wm(g, this).offset_0x34;
            wm(g, this).offset_0x34 = NULL;
            vcall!(g, focus, w.vfunction47);
        }
    }
}

/// port: 0046d150 FUN_0046d150
/// `AddBaseModal(Widget*, const FlagsMod& belowFlagsMod)`: pushes the current modal state.
pub fn FUN_0046d150(g: &mut G, this: Ptr, widget: Ptr, below: (u32, u32)) {
    let d = wm(g, this);
    let info = PreModalInfo {
        mBaseModalWidget: widget,
        mPrevBaseModalWidget: d.offset_0x40,
        mPrevFocusWidget: d.offset_0x34,
        mPrevBelowModalFlagsMod: d.offset_0x4c,
    };
    d.offset_0x5c.push(info);
    FUN_0046d070(g, this, widget, below);
}

/// port: 0046d1d0 FUN_0046d1d0
/// `AddBaseModal(Widget*)` with `mDefaultBelowModalFlagsMod`.
pub fn FUN_0046d1d0(g: &mut G, this: Ptr, widget: Ptr) {
    let below = wm(g, this).offset_0x54;
    FUN_0046d150(g, this, widget, below);
}

/// port: 0046d1e0 FUN_0046d1e0
/// `RemoveBaseModal(Widget*)`: pops modal states back to before `param_1`.
pub fn FUN_0046d1e0(g: &mut G, this: Ptr, param_1: Ptr) {
    let mut first = true;
    while !wm(g, this).offset_0x5c.is_empty() {
        let info = *wm(g, this).offset_0x5c.last().unwrap();
        if first && info.mBaseModalWidget != param_1 {
            return;
        }
        let done = !(info.mPrevBaseModalWidget == NULL && wm(g, this).offset_0x5c.len() != 1);
        FUN_0046d070(g, this, info.mPrevBaseModalWidget, info.mPrevBelowModalFlagsMod);
        if wm(g, this).offset_0x34 == NULL {
            wm(g, this).offset_0x34 = info.mPrevFocusWidget;
            if info.mPrevFocusWidget != NULL {
                vcall!(g, info.mPrevFocusWidget, w.vfunction46);
            }
        }
        wm(g, this).offset_0x5c.pop();
        if done {
            return;
        }
        first = false;
    }
}

/// port: 0046d2d0 FUN_0046d2d0
/// `Resize(const Rect& mouseDestRect, const Rect& mouseSourceRect)`.
pub fn FUN_0046d2d0(g: &mut G, this: Ptr, dest: Rect, src: Rect) {
    g.wc(this).offset_0x34 = dest.mWidth + dest.mX * 2;
    g.wc(this).offset_0x38 = dest.mHeight + dest.mY * 2;
    let d = wm(g, this);
    d.offset_0x68 = dest;
    d.offset_0x78 = src;
}

/// port: 0046d340 Sexy::WidgetManager::vfunction9
/// `SetFocus(Widget*)`.
pub fn vfunction9(g: &mut G, this: Ptr, widget: Ptr) {
    let focus = wm(g, this).offset_0x34;
    if widget != focus {
        if focus != NULL {
            vcall!(g, focus, w.vfunction47);
        }
        if widget != NULL && g.wc(widget).offset_0xc == this {
            wm(g, this).offset_0x34 = widget;
            if wm(g, this).offset_0x30 {
                vcall!(g, widget, w.vfunction46);
            }
        } else {
            wm(g, this).offset_0x34 = NULL;
        }
    }
}

/// port: 0046d3a0 FUN_0046d3a0
/// `GotFocus()` (the application window gained focus).
pub fn FUN_0046d3a0(g: &mut G, this: Ptr) {
    if !wm(g, this).offset_0x30 {
        wm(g, this).offset_0x30 = true;
        let focus = wm(g, this).offset_0x34;
        if focus != NULL {
            vcall!(g, focus, w.vfunction46);
        }
    }
}

/// port: 0046d3d0 FUN_0046d3d0
/// `LostFocus()`: releases held keys and the focus widget's focus.
pub fn FUN_0046d3d0(g: &mut G, this: Ptr) {
    if wm(g, this).offset_0x30 {
        wm(g, this).offset_0x98 = 0;
        for k in 0..0xffu32 {
            if wm(g, this).offset_0xa0[k as usize] {
                FUN_0046dc50(g, this, k);
            }
        }
        wm(g, this).offset_0x30 = false;
        let focus = wm(g, this).offset_0x34;
        if focus != NULL {
            vcall!(g, focus, w.vfunction47);
        }
    }
}

/// port: 0046d430 FUN_0046d430
/// `InitModalFlags(ModalFlags*)`.
pub fn FUN_0046d430(g: &mut G, this: Ptr) -> ModalFlags {
    let is_over = wm(g, this).offset_0x40 == NULL;
    let over = FUN_0046cc70(g, this);
    let below = wm(g, this).offset_0x4c;
    ModalFlags { mOverFlags: over, mUnderFlags: (below.0 | over) & !below.1, mIsOver: is_over }
}

/// port: 0046d460 FUN_0046d460
/// `DrawScreen()`: redraws the dirty top-level widgets (and overlays) onto the screen
/// image; returns whether anything was drawn. Pixels of widgets that are not dirty stay as
/// they were, exactly like the original's persistent screen surface (the renderer keeps
/// its target between frames). The DDImage lock/unlock around it is the renderer's business.
pub fn FUN_0046d460(g: &mut G, this: Ptr) -> bool {
    let mut flags = FUN_0046d430(g, this);
    let mut drew = false;
    let dirty_count = g.wc(this).offset_0x4.iter().collect::<Vec<_>>().into_iter().filter(|&w| g.wc(w).offset_0x28).count();
    wm(g, this).offset_0x2c = 0x7fff_ffff;
    wm(g, this).offset_0x1c.clear();
    let screen = wm(g, this).offset_0xc;
    let (sw, sh) = (g.image(screen).offset_0x20, g.image(screen).offset_0x24);
    let out = wm(g, this).screen_cmds.clone();
    let scr_g = Graphics::new(screen, sw, sh, out);
    wm(g, this).offset_0x4 = Some(scr_g.clone());
    if 0 < dirty_count {
        let mut base = Graphics { s: scr_g.s.clone(), mStateStack: Vec::new(), out: scr_g.out.clone() };
        let dest = wm(g, this).offset_0x68;
        FUN_004563d0(&mut base, -dest.mX, -dest.mY);
        let app = wm(g, this).offset_0x8;
        let is3d = crate::sexy::sexy_app_base::dtor_MemoryImage__00489a20(g, app);
        let mut it = g.wc(this).offset_0x4.begin();
        while it != 0 {
            let w = g.wc(this).offset_0x4.get(it);
            let manager = g.wc(this).offset_0xc;
            if w == wm(g, manager).offset_0x40 {
                flags.mIsOver = true;
            }
            if g.wc(w).offset_0x28 && g.w(w).offset_0x0 {
                let mut cg = Graphics { s: base.s.clone(), mStateStack: Vec::new(), out: base.out.clone() };
                FUN_00455900(&mut cg, !is3d);
                FUN_00455910(&mut cg, is3d);
                let (x, y) = (g.wc(w).offset_0x2c, g.wc(w).offset_0x30);
                FUN_004563d0(&mut cg, x, y);
                vcall!(g, w, w.vfunction28, &mut flags, &mut cg);
                drew = true;
                g.wc(w).offset_0x28 = false;
            }
            it = g.wc(this).offset_0x4.next(it);
        }
    }
    FUN_0046cd30(g, this, 0x7fff_ffff);
    wm(g, this).offset_0x4 = None;
    drew
}

/// port: 0046d6f0 FUN_0046d6f0
/// `UpdateFrame()`: one logic tick of the whole widget tree; returns `mDirty`.
pub fn FUN_0046d6f0(g: &mut G, this: Ptr) -> bool {
    let mut flags = FUN_0046d430(g, this);
    let wc = g.wc(this);
    wc.offset_0x24 += 1;
    wc.offset_0x20 = wc.offset_0x24;
    vcall!(g, this, w.vfunction24, &mut flags);
    g.wc(this).offset_0x28
}

/// port: 0046d720 FUN_0046d720
/// `UpdateFrameF(float theFrac)`.
pub fn FUN_0046d720(g: &mut G, this: Ptr, param_1: f32) -> bool {
    let mut flags = FUN_0046d430(g, this);
    vcall!(g, this, w.vfunction26, &mut flags, param_1);
    g.wc(this).offset_0x28
}

/// port: 0046d750 FUN_0046d750
/// `RemovePopups()`: removes `mPopupCommandWidget`.
pub fn FUN_0046d750(g: &mut G, this: Ptr) {
    let w = wm(g, this).offset_0x18;
    if w != NULL {
        wm(g, this).offset_0x18 = NULL;
        vcall!(g, this, w.vfunction5, w);
    }
}

/// port: 0046d770 FUN_0046d770
/// `MouseMove(int x, int y)`: tracks the hovered widget and forwards `MouseMove`.
pub fn FUN_0046d770(g: &mut G, this: Ptr, param_1: i32, param_2: i32) {
    let (lx, ly) = (wm(g, this).offset_0x8c, wm(g, this).offset_0x90);
    wm(g, this).offset_0x8c = param_1;
    wm(g, this).offset_0x90 = param_2;
    let mut pos = None;
    let w = FUN_0046ccc0(g, this, param_1, param_2, &mut pos);
    let over = wm(g, this).offset_0x3c;
    if w == over {
        if lx == param_1 && ly == param_2 {
            return;
        }
        if w == NULL {
            return;
        }
    } else {
        wm(g, this).offset_0x3c = NULL;
        if over != NULL {
            FUN_0046d040(g, over);
        }
        wm(g, this).offset_0x3c = w;
        if w == NULL {
            return;
        }
        FUN_0046d010(g, w);
    }
    let (wx, wy) = pos.unwrap_or((0, 0));
    vcall!(g, w, w.vfunction53, wx, wy);
}

/// port: 0046d820 FUN_0046d820
/// `RehupMouse()`: re-evaluates the hovered widget after widgets moved.
pub fn FUN_0046d820(g: &mut G, this: Ptr) {
    if wm(g, this).offset_0x38 == NULL {
        if wm(g, this).offset_0x88 {
            let (x, y) = (wm(g, this).offset_0x8c, wm(g, this).offset_0x90);
            FUN_0046d770(g, this, x, y);
        }
    } else if wm(g, this).offset_0x3c != NULL {
        let (x, y) = (wm(g, this).offset_0x8c, wm(g, this).offset_0x90);
        let w = FUN_0046ccc0(g, this, x, y, &mut None);
        if w != wm(g, this).offset_0x38 {
            let over = wm(g, this).offset_0x3c;
            wm(g, this).offset_0x3c = NULL;
            FUN_0046d040(g, over);
        }
    }
}

/// port: 0046d890 FUN_0046d890
/// `MouseUp(int x, int y, int clickCount)`.
pub fn FUN_0046d890(g: &mut G, this: Ptr, x: i32, y: i32, clicks: i32) -> bool {
    let cnt = g.wc(this).offset_0x24;
    wm(g, this).offset_0x9c = cnt;
    let bit: u32 = if clicks < 0 { 2 } else if clicks == 3 { 4 } else { 1 };
    wm(g, this).offset_0x98 &= !bit;
    let down = wm(g, this).offset_0x38;
    if down != NULL && bit & wm(g, this).offset_0x94 != 0 {
        let rest = wm(g, this).offset_0x94 & !bit;
        wm(g, this).offset_0x94 = rest;
        if rest == 0 {
            wm(g, this).offset_0x38 = NULL;
        }
        g.w(down).offset_0x4 = false;
        let (wx, wy) = (g.wc(down).offset_0x2c, g.wc(down).offset_0x30);
        vcall!(g, down, w.vfunction57, x - wx, y - wy, clicks);
        FUN_0046d770(g, this, x, y);
        return true;
    }
    wm(g, this).offset_0x94 &= !bit;
    FUN_0046d770(g, this, x, y);
    true
}

/// port: 0046d940 FUN_0046d940
/// `MouseDown(int x, int y, int clickCount)`.
pub fn FUN_0046d940(g: &mut G, this: Ptr, x: i32, y: i32, clicks: i32) -> bool {
    let cnt = g.wc(this).offset_0x24;
    wm(g, this).offset_0x9c = cnt;
    let bit: u32 = if clicks < 0 { 2 } else if clicks == 3 { 4 } else { 1 };
    wm(g, this).offset_0x98 |= bit;
    FUN_0046d770(g, this, x, y);
    let popup = wm(g, this).offset_0x18;
    if popup != NULL && !vcall!(g, popup, w.vfunction69, x, y) {
        FUN_0046d750(g, this);
    }
    let mut pos = None;
    let mut w = FUN_0046ccc0(g, this, x, y, &mut pos);
    if wm(g, this).offset_0x38 != NULL {
        w = wm(g, this).offset_0x38;
    }
    let (b, id) = if clicks < 0 { (2u32, -1) } else if clicks == 3 { (4, 2) } else { (1, 1) };
    wm(g, this).offset_0x94 |= b;
    wm(g, this).offset_0x1a0 = id;
    wm(g, this).offset_0x38 = w;
    if w != NULL {
        if vcall!(g, w, w.vfunction43) {
            vcall!(g, this, w.vfunction9, w);
        }
        g.w(w).offset_0x4 = true;
        // When no widget is under the point but one is still held down, the original passes
        // its uninitialized out-params: widgetX keeps the manager's address and widgetY the
        // screen x. An address has no meaning here, so widgetX is 0 in that case.
        let (wx, wy) = pos.unwrap_or((0, x));
        vcall!(g, w, w.vfunction55, wx, wy, clicks);
    }
    true
}

/// port: 0046da50 FUN_0046da50
/// `MousePosition(int x, int y)`: drag while a button is held, else move.
pub fn FUN_0046da50(g: &mut G, this: Ptr, x: i32, y: i32) -> bool {
    let cnt = g.wc(this).offset_0x24;
    wm(g, this).offset_0x9c = cnt;
    if wm(g, this).offset_0x94 != 0 {
        return FUN_0046da90(g, this, x, y);
    }
    wm(g, this).offset_0x88 = true;
    FUN_0046d770(g, this, x, y);
    true
}

/// port: 0046da90 FUN_0046da90
/// `MouseDrag(int x, int y)`: forwards to `mLastDownWidget` and tracks hovering over it.
pub fn FUN_0046da90(g: &mut G, this: Ptr, param_1: i32, y: i32) -> bool {
    let cnt = g.wc(this).offset_0x24;
    let d = wm(g, this);
    d.offset_0x9c = cnt;
    d.offset_0x88 = true;
    d.offset_0x8c = param_1;
    d.offset_0x90 = y;
    let (over, down) = (d.offset_0x3c, d.offset_0x38);
    if over != NULL && over != down {
        wm(g, this).offset_0x3c = NULL;
        FUN_0046d040(g, over);
    }
    let down = wm(g, this).offset_0x38;
    if down != NULL {
        let abs = vcall!(g, down, w.vfunction16);
        vcall!(g, down, w.vfunction59, param_1 - abs.mX, y - abs.mY);
        let w = FUN_0046ccc0(g, this, param_1, y, &mut None);
        let down = wm(g, this).offset_0x38;
        if w == down && w != NULL {
            if wm(g, this).offset_0x3c == NULL {
                wm(g, this).offset_0x3c = down;
                FUN_0046d010(g, down);
                return true;
            }
        } else if wm(g, this).offset_0x3c != NULL {
            let over = wm(g, this).offset_0x3c;
            wm(g, this).offset_0x3c = NULL;
            FUN_0046d040(g, over);
        }
    }
    true
}

/// port: 0046db70 FUN_0046db70
/// `MouseExit(int, int)`.
pub fn FUN_0046db70(g: &mut G, this: Ptr) -> bool {
    let cnt = g.wc(this).offset_0x24;
    wm(g, this).offset_0x9c = cnt;
    wm(g, this).offset_0x88 = false;
    let over = wm(g, this).offset_0x3c;
    if over != NULL {
        FUN_0046d040(g, over);
        wm(g, this).offset_0x3c = NULL;
    }
    true
}

/// port: 0046dba0 FUN_0046dba0
/// `MouseWheel(int delta)`: to the focus widget.
pub fn FUN_0046dba0(g: &mut G, this: Ptr, delta: i32) {
    let cnt = g.wc(this).offset_0x24;
    wm(g, this).offset_0x9c = cnt;
    let focus = wm(g, this).offset_0x34;
    if focus != NULL {
        vcall!(g, focus, w.vfunction60, delta);
    }
}

/// port: 0046dbc0 FUN_0046dbc0
/// `KeyChar(char)`: Ctrl+Tab goes to `mDefaultTab`, everything else to the focus widget.
pub fn FUN_0046dbc0(g: &mut G, this: Ptr, c: u8) -> bool {
    let cnt = g.wc(this).offset_0x24;
    wm(g, this).offset_0x9c = cnt;
    if c == b'\t' && wm(g, this).offset_0xa0[0x11] {
        let tab = wm(g, this).offset_0x0;
        if tab != NULL {
            vcall!(g, tab, w.vfunction48, 9);
            return true;
        }
    } else {
        let focus = wm(g, this).offset_0x34;
        if focus != NULL {
            vcall!(g, focus, w.vfunction48, c);
        }
    }
    true
}

/// port: 0046dc10 FUN_0046dc10
/// `KeyDown(KeyCode)`.
pub fn FUN_0046dc10(g: &mut G, this: Ptr, key: u32) -> bool {
    let cnt = g.wc(this).offset_0x24;
    wm(g, this).offset_0x9c = cnt;
    if key < 0xff {
        wm(g, this).offset_0xa0[key as usize] = true;
    }
    let focus = wm(g, this).offset_0x34;
    if focus != NULL {
        vcall!(g, focus, w.vfunction49, key as i32);
    }
    true
}

/// port: 0046dc50 FUN_0046dc50
/// `KeyUp(KeyCode)`: Ctrl+Tab release is swallowed.
pub fn FUN_0046dc50(g: &mut G, this: Ptr, key: u32) -> bool {
    let cnt = g.wc(this).offset_0x24;
    wm(g, this).offset_0x9c = cnt;
    if key < 0xff {
        wm(g, this).offset_0xa0[key as usize] = false;
    }
    if !(key == 9 && wm(g, this).offset_0xa0[0x11]) {
        let focus = wm(g, this).offset_0x34;
        if focus != NULL {
            vcall!(g, focus, w.vfunction50, key as i32);
        }
    }
    true
}
