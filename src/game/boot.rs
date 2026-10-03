//! Program start (`WinMain`, @ 005024f0) and the entry points the Bevy host drives.
//!
//! `WinMain` does `gApp = new WinFishApp; gApp->Init(); gApp->Start(); gApp->Shutdown();
//! delete gApp;`. `Start()` is the framework's blocking main loop; here the host calls
//! [`update_frame`] / [`draw_frame`] every frame instead, which is what that loop does.

use crate::sexy::prelude::*;

/// port: 005024f0 FUN_005024f0
/// `WinMain` up to (and including) the start of the main loop; the host then drives the
/// loop, and `FUN_005024f0_end` is the rest (`delete gApp`) once it has ended.
pub fn FUN_005024f0(g: &mut G) {
    let app = crate::game::win_fish_app::WinFishApp(g);
    g.globals.DAT_005e8f28 = app;
    // gApp->Init() (WinFishApp vtable +0xc4), then gApp->Start() (+0xc0).
    crate::game::win_fish_app::vfunction50(g, app);
    crate::game::win_fish_app::vfunction49(g, app);
}

/// The rest of `WinMain` once the main loop has ended (`Shutdown()` done): `delete gApp`.
/// The app globals are cleared so nothing reaches the freed object afterwards.
pub fn FUN_005024f0_end(g: &mut G) {
    let app = g.globals.DAT_005e8f28;
    if app == NULL {
        return;
    }
    crate::game::win_fish_app::deleting_destructor__00551c60(g, app, 1);
    g.globals.DAT_005e8f28 = NULL;
    g.globals.DAT_005eb6a4 = NULL;
}

/// Runs a slice of the loading thread (see `win_fish_app::LoadingThread`), about 8 ms worth.
pub fn loading_thread_slice(g: &mut G) {
    let Some(mut lt) = g.loading_thread.take() else { return };
    let app = g.globals.DAT_005e8f28;
    let start = std::time::Instant::now();
    let fixed = std::env::var_os("WINFISH_FIXED_STEPS").is_some();
    let mut steps = 0;
    loop {
        if !crate::game::win_fish_app::vfunction23(g, app, &mut lt) {
            g.sab(app).field_0x4fa = true;
            return;
        }
        // (WINFISH_FIXED_STEPS test runs: a fixed 64 resource steps per frame instead of 8 ms.)
        steps += 1;
        if fixed {
            if 64 <= steps {
                break;
            }
        } else if start.elapsed().as_millis() >= 8 {
            break;
        }
    }
    g.loading_thread = Some(lt);
}

/// The app's `mWidgetManager` (SexyAppBase +0x320), or null before Init.
pub fn widget_manager(g: &mut G) -> Ptr {
    let app = g.globals.DAT_005e8f28;
    if app == NULL {
        return NULL;
    }
    g.sab(app).offset_0x318
}

/// One logic update: the app's `UpdateFrames()` (WinFishApp's override, `vfunction9`).
pub fn update_frame(g: &mut G) {
    let app = g.globals.DAT_005e8f28;
    if app != NULL {
        crate::game::win_fish_app::vfunction9(g, app);
    } else {
        update_app_step(g);
    }
}

/// `SexyApp::UpdateFrames` / `SexyAppBase::UpdateFrames` (vfunction9, replaced by the host):
/// mUpdateCount++, then `mWidgetManager->UpdateFrame()`.
pub fn update_app_step(g: &mut G) {
    let app0 = g.globals.DAT_005e8f28;
    if app0 != NULL {
        crate::sexy::app_host::update_opening_url(g, app0);
    }
    let app = g.globals.DAT_005e8f28;
    if app != NULL {
        g.sab(app).field_0x47c += 1;
    }
    let wm = widget_manager(g);
    if wm != NULL {
        crate::sexy::widget_manager::FUN_0046d6f0(g, wm);
    }
    // mMusicInterface->Update() (vtable +0x4c): the fades step.
    if app != NULL && g.sab(app).offset_0x36c != NULL {
        crate::sexy::music::update(&mut g.music);
    }
}

/// One redraw (`SexyAppBase::DrawDirtyStuff`): `DrawScreen`, then the recorded blits are
/// rasterized onto the persistent screen image.
pub fn draw_frame(g: &mut G) {
    let wm = widget_manager(g);
    if wm == NULL {
        return;
    }
    crate::sexy::widget_manager::FUN_0046d460(g, wm);
    let cmds = std::mem::take(&mut *crate::sexy::widget_manager::wm(g, wm).screen_cmds.lock().unwrap());
    let screen = crate::sexy::widget_manager::wm(g, wm).offset_0xc;
    let mut bits = std::mem::take(&mut g.image(screen).mBits);
    // The HD screen (port addition) is painted from the same calls; when it starts, it is
    // seeded with the screen as it was before this frame.
    let mut hd = std::mem::take(&mut g.hd);
    let hd_on = std::env::var_os("WINFISH_NO_HD").is_none() && hd.available(g);
    if hd_on {
        hd.maintain(g);
    } else {
        hd.created.clear();
    }
    let prev = if hd_on && !hd.active { bits.clone() } else { Vec::new() };
    {
        let (w, h) = (g.image(screen).offset_0x20, g.image(screen).offset_0x24);
        let mut target = crate::sexy::blit::Target { bits: &mut bits, width: w, height: h };
        for c in &cmds {
            target.apply(g, c);
        }
    }
    if hd_on {
        if !hd.active {
            let (w, h) = (g.image(screen).offset_0x20, g.image(screen).offset_0x24);
            hd.activate(&prev, w, h);
        }
        hd.paint(g, &cmds);
    } else {
        hd.active = false;
    }
    g.hd = hd;
    g.image(screen).mBits = bits;
    for img in std::mem::take(&mut g.frame_temp_images) {
        g.free(img);
    }
}

/// The HD screen's pixels and size, while it is displayed.
pub fn screen_bits_hd(g: &G) -> Option<(Vec<u32>, i32, i32)> {
    g.hd.active.then(|| (g.hd.bits.clone(), g.hd.width, g.hd.height))
}

/// The screen's pixels, for upload.
pub fn screen_bits(g: &mut G) -> Option<Vec<u32>> {
    let wm = widget_manager(g);
    if wm == NULL {
        return None;
    }
    let screen = crate::sexy::widget_manager::wm(g, wm).offset_0xc;
    Some(g.image(screen).mBits.clone())
}
