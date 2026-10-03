//! `Sexy::WidgetContainer` (framework, 0x0046b4b0..0x0046ca20): the child list, z-ordering,
//! dirty tracking and the recursive `UpdateAll`/`DrawAll` walks that decide in which order
//! every game object updates and draws.

use crate::sexy::graphics::{FUN_00455710, FUN_004557a0, FUN_00456340, FUN_004563d0, FUN_00468430};
use crate::sexy::object::WidgetContainer_data;
use crate::sexy::prelude::*;
use crate::sexy::std_list::ListIter;

/// port: 0046b4b0 Sexy::WidgetContainer::WidgetContainer
/// Fresh container state (the decompiler shows it working on ESI: the object being built).
pub fn WidgetContainer() -> WidgetContainer_data {
    WidgetContainer_data { offset_0x3d: true, ..Default::default() }
}

/// port: 0046b510 Sexy::WidgetContainer::deleting_destructor
pub fn deleting_destructor(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_WidgetContainer(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 0046b530 Sexy::WidgetContainer::~WidgetContainer
/// Destroys the child list (the children themselves are not freed). The original also
/// resets the vftable to the abstract `WidgetContainer` one, which nothing can observe.
pub fn dtor_WidgetContainer(g: &mut G, this: Ptr) {
    g.wc(this).offset_0x4.clear();
}

/// port: 0046b550 Sexy::WidgetContainer::vfunction8
/// `RemoveAllWidgets(bool doDelete, bool recursive)`.
pub fn vfunction8(g: &mut G, this: Ptr, do_delete: bool, recursive: bool) {
    while g.wc(this).offset_0x4.len() != 0 {
        let w = g.wc(this).offset_0x4.front();
        vcall!(g, this, w.vfunction5, w);
        if recursive {
            vcall!(g, w, w.vfunction8, do_delete, recursive);
        }
        if do_delete && w != NULL {
            vcall!(g, w, w.vfunction1, 1);
        }
    }
}

/// port: 0046b5b0 Sexy::WidgetContainer::vfunction2
/// `GetRect()`.
pub fn vfunction2(g: &mut G, this: Ptr) -> Rect {
    let wc = g.wc(this);
    Rect::new(wc.offset_0x2c, wc.offset_0x30, wc.offset_0x34, wc.offset_0x38)
}

/// port: 0046b5d0 Sexy::WidgetContainer::vfunction3
/// `Intersects(WidgetContainer*)`.
pub fn vfunction3(g: &mut G, this: Ptr, other: Ptr) -> bool {
    let a = vcall!(g, other, w.vfunction2);
    let b = vcall!(g, this, w.vfunction2);
    b.mX < a.mWidth + a.mX && b.mY < a.mHeight + a.mY && a.mX < b.mWidth + b.mX && a.mY < b.mHeight + b.mY
}

/// port: 0046bb20 FUN_0046bb20
/// `InsertWidgetHelper(const iterator& where, Widget*)`: inserts by `mZOrder`, searching
/// forward from `where`, then backward.
pub fn FUN_0046bb20(g: &mut G, this: Ptr, where_: ListIter, param_1: Ptr) {
    let z = g.wc(param_1).offset_0x4c;
    let mut it = where_;
    let mut backward = false;
    loop {
        let list = &g.wc(this).offset_0x4;
        if it == list.end() {
            backward = true;
            break;
        }
        let w = list.get(it);
        if z <= g.wc(w).offset_0x4c {
            break;
        }
        it = g.wc(this).offset_0x4.next(it);
    }
    if !backward {
        let list = &g.wc(this).offset_0x4;
        if it != list.begin() {
            // The framework steps to the previous node but then compares the current one.
            let w = list.get(it);
            if z < g.wc(w).offset_0x4c {
                backward = true;
            }
        }
    }
    if backward {
        loop {
            let list = &g.wc(this).offset_0x4;
            if it == list.begin() {
                g.wc(this).offset_0x4.push_front(param_1);
                return;
            }
            it = list.prev(it);
            let w = list.get(it);
            if !(z < g.wc(w).offset_0x4c) {
                break;
            }
        }
        it = g.wc(this).offset_0x4.next(it);
    }
    g.wc(this).offset_0x4.insert(it, param_1);
}

/// port: 0046b640 Sexy::WidgetContainer::vfunction4
/// `AddWidget(Widget*)`: no-op if already a child.
pub fn vfunction4(g: &mut G, this: Ptr, widget: Ptr) {
    if g.wc(this).offset_0x4.find(widget) == 0 {
        let end = g.wc(this).offset_0x4.end();
        FUN_0046bb20(g, this, end, widget);
        let wm = g.wc(this).offset_0xc;
        g.wc(widget).offset_0xc = wm;
        g.wc(widget).offset_0x10 = this;
        if wm != NULL {
            vcall!(g, widget, w.vfunction21, wm);
            vcall!(g, widget, w.vfunction20);
            crate::sexy::widget_manager::FUN_0046d820(g, wm);
        }
        vcall!(g, this, w.vfunction18);
    }
}

/// port: 0046b700 Sexy::WidgetContainer::vfunction6
/// `HasWidget(Widget*)`.
pub fn vfunction6(g: &mut G, this: Ptr, widget: Ptr) -> bool {
    g.wc(this).offset_0x4.find(widget) != 0
}

/// port: 0046b760 Sexy::WidgetContainer::vfunction5
/// `RemoveWidget(Widget*)`: keeps a running `UpdateAll` iterator valid.
pub fn vfunction5(g: &mut G, this: Ptr, widget: Ptr) {
    let it = g.wc(this).offset_0x4.find(widget);
    if it != 0 {
        crate::sexy::widget::FUN_0046dd90(g, widget);
        g.wc(widget).offset_0x10 = NULL;
        let erased_cur = it == g.wc(this).offset_0x1c;
        let next = g.wc(this).offset_0x4.erase(it);
        if erased_cur {
            let wc = g.wc(this);
            wc.offset_0x1c = next;
            wc.offset_0x14 = true;
        }
    }
}

/// port: 0046ba10 FUN_0046ba10
/// `IsBelowHelper(Widget* w1, Widget* w2, bool* found)`: depth-first search for whichever
/// widget comes first.
pub fn FUN_0046ba10(g: &mut G, this: Ptr, param_1: Ptr, param_2: Ptr, param_3: &mut bool) -> bool {
    let mut it = g.wc(this).offset_0x4.begin();
    let mut r;
    loop {
        if it == 0 {
            return false;
        }
        let w = g.wc(this).offset_0x4.get(it);
        if w == param_1 {
            *param_3 = true;
            return true;
        }
        if w == param_2 {
            *param_3 = true;
            return false;
        }
        r = FUN_0046ba10(g, w, param_1, param_2, param_3);
        if *param_3 {
            break;
        }
        it = g.wc(this).offset_0x4.next(it);
    }
    r
}

/// port: 0046ba90 Sexy::WidgetContainer::vfunction10
/// `IsBelow(Widget*, Widget*)`.
pub fn vfunction10(g: &mut G, this: Ptr, w1: Ptr, w2: Ptr) -> bool {
    let mut found = false;
    FUN_0046ba10(g, this, w1, w2, &mut found)
}

/// port: 0046bab0 Sexy::WidgetContainer::vfunction11
/// `MarkAllDirty()`.
pub fn vfunction11(g: &mut G, this: Ptr) {
    vcall!(g, this, w.vfunction18);
    let mut it = g.wc(this).offset_0x4.begin();
    while it != 0 {
        let w = g.wc(this).offset_0x4.get(it);
        g.wc(w).offset_0x28 = true;
        vcall!(g, w, w.vfunction11);
        it = g.wc(this).offset_0x4.next(it);
    }
}

/// Shared head of BringToFront/BringToBack/PutBehind/PutInfront: unlinks `widget`, keeping
/// the update iterator valid. Returns false when `widget` is not a child.
fn unlink_for_reorder(g: &mut G, this: Ptr, widget: Ptr) -> bool {
    let it = g.wc(this).offset_0x4.find(widget);
    if it == 0 {
        return false;
    }
    if it == g.wc(this).offset_0x1c {
        let wc = g.wc(this);
        wc.offset_0x1c = wc.offset_0x4.next(it);
        wc.offset_0x14 = true;
    }
    g.wc(this).offset_0x4.erase(it);
    true
}

/// port: 0046bc50 Sexy::WidgetContainer::vfunction12
/// `BringToFront(Widget*)`.
pub fn vfunction12(g: &mut G, this: Ptr, widget: Ptr) {
    if unlink_for_reorder(g, this, widget) {
        let end = g.wc(this).offset_0x4.end();
        FUN_0046bb20(g, this, end, widget);
        vcall!(g, widget, w.vfunction31);
    }
}

/// port: 0046bd20 Sexy::WidgetContainer::vfunction13
/// `BringToBack(Widget*)`.
pub fn vfunction13(g: &mut G, this: Ptr, widget: Ptr) {
    if unlink_for_reorder(g, this, widget) {
        let begin = g.wc(this).offset_0x4.begin();
        FUN_0046bb20(g, this, begin, widget);
        vcall!(g, widget, w.vfunction31);
    }
}

/// port: 0046bdf0 Sexy::WidgetContainer::vfunction14
/// `PutBehind(Widget*, Widget* ref)`.
pub fn vfunction14(g: &mut G, this: Ptr, widget: Ptr, reference: Ptr) {
    if unlink_for_reorder(g, this, widget) {
        let at = g.wc(this).offset_0x4.find(reference);
        FUN_0046bb20(g, this, at, widget);
        vcall!(g, widget, w.vfunction31);
    }
}

/// port: 0046bf00 Sexy::WidgetContainer::vfunction15
/// `PutInfront(Widget*, Widget* ref)`.
pub fn vfunction15(g: &mut G, this: Ptr, widget: Ptr, reference: Ptr) {
    if unlink_for_reorder(g, this, widget) {
        let mut at = g.wc(this).offset_0x4.find(reference);
        if at != 0 {
            at = g.wc(this).offset_0x4.next(at);
        }
        FUN_0046bb20(g, this, at, widget);
        vcall!(g, widget, w.vfunction31);
    }
}

/// port: 0046c030 Sexy::WidgetContainer::vfunction16
/// `GetAbsPos()`.
pub fn vfunction16(g: &mut G, this: Ptr) -> Point {
    let (x, y, parent) = {
        let wc = g.wc(this);
        (wc.offset_0x2c, wc.offset_0x30, wc.offset_0x10)
    };
    if parent == NULL {
        Point { mX: x, mY: y }
    } else {
        let p = vcall!(g, parent, w.vfunction16);
        Point { mX: p.mX + x, mY: p.mY + y }
    }
}

/// port: 0046c090 Sexy::WidgetContainer::vfunction21
/// `AddedToManager(WidgetManager*)`.
pub fn vfunction21(g: &mut G, this: Ptr, manager: Ptr) {
    let mut it = g.wc(this).offset_0x4.begin();
    while it != 0 {
        let w = g.wc(this).offset_0x4.get(it);
        g.wc(w).offset_0xc = manager;
        vcall!(g, w, w.vfunction21, manager);
        it = g.wc(this).offset_0x4.next(it);
        vcall!(g, this, w.vfunction18);
    }
}

/// port: 0046c100 Sexy::WidgetContainer::vfunction22
/// `RemovedFromManager(WidgetManager*)`.
pub fn vfunction22(g: &mut G, this: Ptr, manager: Ptr) {
    let mut it = g.wc(this).offset_0x4.begin();
    while it != 0 {
        let w = g.wc(this).offset_0x4.get(it);
        vcall!(g, manager, w.vfunction7, w, true);
        vcall!(g, w, w.vfunction22, manager);
        g.wc(w).offset_0xc = NULL;
        it = g.wc(this).offset_0x4.next(it);
    }
    let wmd = crate::sexy::widget_manager::wm(g, manager);
    if wmd.offset_0x18 == this {
        wmd.offset_0x18 = NULL;
    }
}

/// port: 0046c180 Sexy::WidgetContainer::vfunction18
/// `MarkDirty()`: asks the parent, or sets `mDirty` on a root.
pub fn vfunction18(g: &mut G, this: Ptr) {
    let parent = g.wc(this).offset_0x10;
    if parent != NULL {
        vcall!(g, parent, w.vfunction17, this);
    } else {
        g.wc(this).offset_0x28 = true;
    }
}

/// port: 0046c1a0 Sexy::WidgetContainer::vfunction20
/// `MarkDirtyFull()`.
pub fn vfunction20(g: &mut G, this: Ptr) {
    let parent = g.wc(this).offset_0x10;
    if parent != NULL {
        vcall!(g, parent, w.vfunction19, this);
    } else {
        g.wc(this).offset_0x28 = true;
    }
}

/// port: 0046c1c0 Sexy::WidgetContainer::vfunction19
/// `MarkDirtyFull(WidgetContainer*)`: also dirties siblings below `param_1` that it fully
/// covers or that intersect it.
pub fn vfunction19(g: &mut G, this: Ptr, param_1: Ptr) {
    vcall!(g, this, w.vfunction20);
    g.wc(param_1).offset_0x28 = true;
    if g.wc(this).offset_0x10 != NULL {
        return;
    }
    let found = g.wc(this).offset_0x4.find(param_1);
    if found == 0 {
        return;
    }
    let begin = g.wc(this).offset_0x4.begin();
    if found != begin {
        let mut it = g.wc(this).offset_0x4.prev(found);
        loop {
            let w = g.wc(this).offset_0x4.get(it);
            if g.w(w).offset_0x0 {
                let (transp, alpha) = (g.w(w).offset_0x6, g.wc(w).offset_0x3c);
                let mut covered = false;
                if !transp && !alpha {
                    let (tw, th) = (g.wc(this).offset_0x34, g.wc(this).offset_0x38);
                    let r = {
                        let p = g.wc(param_1);
                        Rect::new(p.offset_0x2c, p.offset_0x30, p.offset_0x34, p.offset_0x38)
                    };
                    let c = FUN_00468430(&Rect::new(0, 0, tw, th), &r);
                    if vcall!(g, w, w.vfunction69, c.mX, c.mY)
                        && vcall!(g, w, w.vfunction69, c.mWidth - 1 + c.mX, c.mHeight - 1 + c.mY)
                    {
                        vcall!(g, w, w.vfunction18);
                        covered = true;
                    }
                }
                if covered {
                    break;
                }
                if vcall!(g, w, w.vfunction3, param_1) {
                    vcall!(g, this, w.vfunction17, w);
                }
            }
            if it == g.wc(this).offset_0x4.begin() {
                break;
            }
            it = g.wc(this).offset_0x4.prev(it);
        }
    }
    let mut it = found;
    while it != 0 {
        let w = g.wc(this).offset_0x4.get(it);
        if g.w(w).offset_0x0 && vcall!(g, w, w.vfunction3, param_1) {
            vcall!(g, this, w.vfunction17, w);
        }
        it = g.wc(this).offset_0x4.next(it);
    }
}

/// port: 0046c430 Sexy::WidgetContainer::vfunction17
/// `MarkDirty(WidgetContainer*)`.
pub fn vfunction17(g: &mut G, this: Ptr, param_1: Ptr) {
    if g.wc(param_1).offset_0x28 {
        return;
    }
    vcall!(g, this, w.vfunction18);
    g.wc(param_1).offset_0x28 = true;
    if g.wc(this).offset_0x10 != NULL {
        return;
    }
    if g.wc(param_1).offset_0x3c {
        vcall!(g, this, w.vfunction19, param_1);
        return;
    }
    let mut after = false;
    let mut it = g.wc(this).offset_0x4.begin();
    while it != 0 {
        let w = g.wc(this).offset_0x4.get(it);
        if w == param_1 {
            after = true;
        } else if after && g.w(w).offset_0x0 && vcall!(g, w, w.vfunction3, param_1) {
            vcall!(g, this, w.vfunction17, w);
        }
        it = g.wc(this).offset_0x4.next(it);
    }
}

/// port: 0046c4f0 Sexy::WidgetContainer::vfunction23
/// `Update()`: counts updates.
pub fn vfunction23(g: &mut G, this: Ptr) {
    g.wc(this).offset_0x24 += 1;
}

/// Applies `mWidgetFlagsMod` to the flags (the `AutoModalFlags` constructor).
fn apply_flags_mod(g: &mut G, this: Ptr, flags: &mut ModalFlags) {
    let (add, remove) = {
        let wc = g.wc(this);
        (wc.offset_0x40, wc.offset_0x44)
    };
    flags.mOverFlags = (add | flags.mOverFlags) & !remove;
    flags.mUnderFlags = (flags.mUnderFlags | add) & !remove;
}

/// port: 0046c500 Sexy::WidgetContainer::vfunction24
/// `UpdateAll(ModalFlags*)`: `Update()` once per manager tick, then every child's
/// `UpdateAll` in list order, tolerating children added or removed meanwhile.
pub fn vfunction24(g: &mut G, this: Ptr, flags: &mut ModalFlags) {
    let saved = (flags.mOverFlags, flags.mUnderFlags);
    apply_flags_mod(g, this, flags);
    if flags.get_flags() & WIDGETFLAGS_MARK_DIRTY != 0 {
        vcall!(g, this, w.vfunction18);
    }
    let wm = g.wc(this).offset_0xc;
    if wm != NULL {
        let wm_count = g.wc(wm).offset_0x24;
        if flags.get_flags() & WIDGETFLAGS_UPDATE != 0 && g.wc(this).offset_0x20 != wm_count {
            g.wc(this).offset_0x20 = wm_count;
            vcall!(g, this, w.vfunction23);
        }
        let begin = g.wc(this).offset_0x4.begin();
        g.wc(this).offset_0x1c = begin;
        loop {
            let it = g.wc(this).offset_0x1c;
            if it == 0 {
                break;
            }
            g.wc(this).offset_0x14 = false;
            let child = g.wc(this).offset_0x4.get(it);
            if child == crate::sexy::widget_manager::wm(g, wm).offset_0x40 {
                flags.mIsOver = true;
            }
            vcall!(g, child, w.vfunction24, flags);
            if !g.wc(this).offset_0x14 {
                let wc = g.wc(this);
                wc.offset_0x1c = wc.offset_0x4.next(wc.offset_0x1c);
            }
        }
        g.wc(this).offset_0x14 = true;
    }
    flags.mOverFlags = saved.0;
    flags.mUnderFlags = saved.1;
}

/// port: 0046c6a0 Sexy::WidgetContainer::vfunction26
/// `UpdateFAll(ModalFlags*, float)`.
pub fn vfunction26(g: &mut G, this: Ptr, flags: &mut ModalFlags, frac: f32) {
    let saved = (flags.mOverFlags, flags.mUnderFlags);
    apply_flags_mod(g, this, flags);
    if flags.get_flags() & WIDGETFLAGS_UPDATE != 0 {
        vcall!(g, this, w.vfunction25, frac);
    }
    let begin = g.wc(this).offset_0x4.begin();
    g.wc(this).offset_0x1c = begin;
    loop {
        let it = g.wc(this).offset_0x1c;
        if it == 0 {
            break;
        }
        g.wc(this).offset_0x14 = false;
        let child = g.wc(this).offset_0x4.get(it);
        let wm = g.wc(this).offset_0xc;
        if child == crate::sexy::widget_manager::wm(g, wm).offset_0x40 {
            flags.mIsOver = true;
        }
        vcall!(g, child, w.vfunction26, flags, frac);
        if !g.wc(this).offset_0x14 {
            let wc = g.wc(this);
            wc.offset_0x1c = wc.offset_0x4.next(wc.offset_0x1c);
        }
    }
    g.wc(this).offset_0x14 = true;
    flags.mOverFlags = saved.0;
    flags.mUnderFlags = saved.1;
}

/// port: 0046c7f0 Sexy::WidgetContainer::vfunction28
/// `DrawAll(ModalFlags*, Graphics*)`: own `Draw`, then each visible child translated to
/// its position, in list (z) order.
pub fn vfunction28(g: &mut G, this: Ptr, flags: &mut ModalFlags, gfx: &mut Graphics) {
    let wm = g.wc(this).offset_0xc;
    let prio = g.wc(this).offset_0x48;
    if crate::sexy::widget_manager::wm(g, wm).offset_0x2c < prio {
        crate::sexy::widget_manager::FUN_0046cd30(g, wm, prio);
    }
    let saved = (flags.mOverFlags, flags.mUnderFlags);
    apply_flags_mod(g, this, flags);
    if g.wc(this).offset_0x3d && flags.get_flags() & WIDGETFLAGS_CLIP != 0 {
        let (w, h) = (g.wc(this).offset_0x34, g.wc(this).offset_0x38);
        FUN_00456340(gfx, 0, 0, w, h);
    }
    if g.wc(this).offset_0x4.len() == 0 {
        if flags.get_flags() & WIDGETFLAGS_DRAW != 0 {
            vcall!(g, this, w.vfunction27, gfx);
        }
    } else {
        if flags.get_flags() & WIDGETFLAGS_DRAW != 0 {
            FUN_00455710(gfx);
            vcall!(g, this, w.vfunction27, gfx);
            FUN_004557a0(gfx);
        }
        let mut it = g.wc(this).offset_0x4.begin();
        while it != 0 {
            let child = g.wc(this).offset_0x4.get(it);
            if g.w(child).offset_0x0 {
                let wm = g.wc(this).offset_0xc;
                if child == crate::sexy::widget_manager::wm(g, wm).offset_0x40 {
                    flags.mIsOver = true;
                }
                let mut clip_g = Graphics { s: gfx.s.clone(), mStateStack: Vec::new(), out: gfx.out.clone() };
                let (x, y) = (g.wc(child).offset_0x2c, g.wc(child).offset_0x30);
                FUN_004563d0(&mut clip_g, x, y);
                vcall!(g, child, w.vfunction28, flags, &mut clip_g);
                g.wc(child).offset_0x28 = false;
            }
            it = g.wc(this).offset_0x4.next(it);
        }
    }
    flags.mUnderFlags = saved.1;
    flags.mOverFlags = saved.0;
}

/// port: 0046c9c0 Sexy::WidgetContainer::vfunction29
/// `SysColorChangedAll()`.
pub fn vfunction29(g: &mut G, this: Ptr) {
    vcall!(g, this, w.vfunction30);
    if g.wc(this).offset_0x4.len() != 0 {
        g.globals.DAT_0062090c += 1;
    }
    let mut it = g.wc(this).offset_0x4.begin();
    while it != 0 {
        let w = g.wc(this).offset_0x4.get(it);
        vcall!(g, w, w.vfunction29);
        it = g.wc(this).offset_0x4.next(it);
    }
}
