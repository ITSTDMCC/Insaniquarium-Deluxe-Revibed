//! `Sexy::UpdateCheckDialog` (framework, 0x0046eb60..0x0046f360): "checking for updates"
//! with a scrolling striped progress bar, polling the app's InternetManager; when the query
//! ends it opens "up to date" (dialog id + 20000) or "new version" (id + 10000) over itself.
//!
//! The object is 0x160 bytes: Dialog (0x150), then its fields at +0x150..+0x15c.

use crate::sexy::dialog::{alloc_dialog, DlgSub};
use crate::sexy::graphics::{FUN_00455890, FUN_004559b0, FUN_00455d20, FUN_004563b0};
use crate::sexy::prelude::*;
use crate::sexy::types::FUN_00433360;

/// The fields past `Dialog` (object offset 0x150).
#[derive(Debug, Clone, Default)]
pub struct UpdateCheckDialog_data {
    /// +0x150 the query has ended (the result dialog is up).
    pub offset_0x50: bool,
    /// +0x154 the progress bar's stripe image.
    pub offset_0x54: Ptr,
    /// +0x158 the stripes' scroll (0..31).
    pub offset_0x58: i32,
    /// +0x15c room under the lines for the bar (66).
    pub offset_0x5c: i32,
}

impl G {
    pub fn update_check(&mut self, p: Ptr) -> &mut UpdateCheckDialog_data {
        match &mut self.widget(p).ext {
            WExt::Dialog(e) => match &mut e.sub {
                DlgSub::UpdateCheck(d) => d,
                s => panic!("{p} is not an UpdateCheckDialog: {s:?}"),
            },
            e => panic!("{p} is not a Dialog: {e:?}"),
        }
    }
}

/// port: 0046eb60 FUN_0046eb60
/// `UpdateCheckDialog(Image* theComponentImage, Image* theButtonImage, Image* theStripe,
/// int theId)`: modal, one button; the title, body and button text from the
/// UPDATE_CHECK_TITLE, UPDATE_CHECK_BODY and DIALOG_BUTTON_CANCEL strings.
pub fn FUN_0046eb60(g: &mut G, param_1: Ptr, param_2: Ptr, param_3: Ptr, param_4: i32) -> Ptr {
    let app = g.globals.DAT_005eb6a4;
    let cancel = crate::sexy::sexy_app_base::FUN_004872f0(g, app, "DIALOG_BUTTON_CANCEL", b"");
    let body = crate::sexy::sexy_app_base::FUN_004872f0(g, app, "UPDATE_CHECK_BODY", b"");
    let title = crate::sexy::sexy_app_base::FUN_004872f0(g, app, "UPDATE_CHECK_TITLE", b"");
    let this = alloc_dialog(g, &crate::sexy::vtables_gen::Sexy__UpdateCheckDialog_vftable, DlgSub::UpdateCheck(UpdateCheckDialog_data::default()));
    crate::sexy::dialog::Dialog(g, this, param_1, param_2, param_4, true, &title, &body, &cancel, 3);
    let d = g.update_check(this);
    d.offset_0x54 = param_3;
    d.offset_0x50 = false;
    d.offset_0x58 = 0;
    d.offset_0x5c = 0x42;
    this
}

/// port: 0046edd0 Sexy::UpdateCheckDialog::~UpdateCheckDialog
pub fn dtor_UpdateCheckDialog(g: &mut G, this: Ptr) {
    crate::sexy::dialog::dtor_Dialog(g, this);
}

/// port: 0046edb0 Sexy::UpdateCheckDialog::deleting_destructor_for_Widget
pub fn deleting_destructor_for_Widget(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_UpdateCheckDialog(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 0046edf0 Sexy::UpdateCheckDialog::vfunction74_for_Widget
/// `GetPreferredHeight(int)`: room for the bar.
pub fn vfunction74_for_Widget(g: &mut G, this: Ptr, param_1: i32) -> i32 {
    crate::sexy::dialog::vfunction74_for_Widget(g, this, param_1) + 0x20
}

/// port: 0046ee00 Sexy::UpdateCheckDialog::vfunction27_for_Widget
/// `Draw(Graphics*)`: the dialog, then the bar: the stripe image tiled across a 16-pixel
/// strip, scrolled, in a black frame.
pub fn vfunction27_for_Widget(g: &mut G, this: Ptr, param_1: &mut Graphics) {
    crate::sexy::dialog::vfunction27_for_Widget(g, this, param_1);
    let (w, h) = (g.wc(this).offset_0x34, g.wc(this).offset_0x38);
    let d = g.dialog(this).clone();
    let u = g.update_check(this).clone();
    let y = (h - u.offset_0x5c) - d.ext_0x128;
    let inner = (w - d.ext_0x124) - d.ext_0x11c;
    let x = d.ext_0x11c + 0x18;
    let bw = inner - 0x30;
    let mut clip = crate::sexy::graphics::FUN_00455800(param_1);
    FUN_004563b0(&mut clip, &Rect::new(x, y, bw, 0x10));
    let n = bw / 32 + 2;
    let mut k = 0;
    for _ in 0..n.max(0) {
        FUN_00455d20(&mut clip, g, u.offset_0x54, (k - u.offset_0x58) + x, y);
        k += 0x20;
    }
    FUN_00455890(&mut clip, FUN_00433360(0, 0, 0));
    FUN_004559b0(&mut clip, x - 1, y - 1, inner - 0x2f, 0x11);
}

/// port: 0046ef40 Sexy::UpdateCheckDialog::vfunction23_for_Widget
/// `Update()`: scrolls the stripes every third update while waiting; once the query has
/// failed or finished, opens "up to date" (failure, or no newer version) or "new version"
/// (yes/no), offset up and right of this one.
pub fn vfunction23_for_Widget(g: &mut G, this: Ptr) {
    crate::sexy::widget_container::vfunction23(g, this);
    if !g.update_check(this).offset_0x50 && g.wc(this).offset_0x24 % 3 == 0 {
        let d = g.update_check(this);
        d.offset_0x58 = (d.offset_0x58 + 1) % 32;
        vcall!(g, this, w.vfunction18);
    }
    let failed = crate::sexy::app_host::update_check_result(g) == 3;
    let finished = crate::sexy::app_host::update_check_result(g) == 2;
    if g.update_check(this).offset_0x50 {
        return;
    }
    if !failed && !finished {
        return;
    }
    g.update_check(this).offset_0x50 = true;
    if finished {
        crate::sexy::app_host::update_check_parse(g);
    }
    let app = g.globals.DAT_005eb6a4;
    let id = g.dialog(this).ext_0x13c;
    let d = if failed || crate::sexy::app_host::update_check_up_to_date(g) {
        let ok = crate::sexy::sexy_app_base::FUN_004872f0(g, app, "DIALOG_BUTTON_OK", b"");
        let body = crate::sexy::sexy_app_base::FUN_004872f0(g, app, "UP_TO_DATE_BODY", b"");
        let title = crate::sexy::sexy_app_base::FUN_004872f0(g, app, "UP_TO_DATE_TITLE", b"");
        crate::game::win_fish_app::vfunction73(g, app, id + 20000, true, &title, &body, &ok, 3)
    } else {
        let body = crate::sexy::sexy_app_base::FUN_004872f0(g, app, "NEW_VERSION_BODY", b"");
        let title = crate::sexy::sexy_app_base::FUN_004872f0(g, app, "NEW_VERSION_TITLE", b"");
        crate::game::win_fish_app::vfunction73(g, app, id + 10000, true, &title, &body, b"", 1)
    };
    let (mx, my) = (g.wc(this).offset_0x2c, g.wc(this).offset_0x30);
    vcall!(g, d, w.vfunction42, mx + 0x20, my - 0x20);
}
