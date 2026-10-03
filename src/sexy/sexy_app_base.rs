//! `Sexy::SexyAppBase` / `Sexy::SexyApp` functions the game calls.

use crate::sexy::prelude::*;

/// port: 004891c0 FUN_004891c0
/// `SexyAppBase::SetCursor(int)`: stores `mCursorNum`, then the virtual `EnforceCursor()`
/// (vtable +0x30; no class overrides it, so it is called directly).
pub fn FUN_004891c0(g: &mut G, this: Ptr, param_1: i32) {
    g.sab(this).field_0x4a8 = param_1;
    enforce_cursor(g, this);
}

/// Replacement for `SexyAppBase::EnforceCursor()` (@ 00486010), which picks a Win32
/// cursor; the host applies `G::cursor_num` to the Bevy window instead.
pub fn enforce_cursor(g: &mut G, this: Ptr) {
    g.cursor_num = g.sab(this).field_0x4a8;
}

/// port: 00489a20 Sexy::MemoryImage::~MemoryImage
/// `SexyAppBase::Is3DAccelerated()`: `mDDInterface->mIs3D`. (The linker folded this body
/// with an identical one, so the decompiler shows a MemoryImage destructor name.)
pub fn dtor_MemoryImage__00489a20(g: &mut G, this: Ptr) -> bool {
    g.sab(this).ddinterface_is_3d
}

/// One sound the game asked to start. The DirectSound manager is replaced by Bevy audio;
/// these carry exactly what the original passed to `SoundInstance::SetVolume/SetPan/Play`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SoundRequest {
    /// Sound id (the value of a `SOUND_*` resource global).
    pub id: i32,
    pub volume: f64,
    pub pan: i32,
    /// `SoundInstance::AdjustPitch` in semitones (0 = as recorded).
    pub pitch: f64,
}

/// port: 004896b0 Sexy::SexyAppBase::vfunction55
/// `PlaySample(int theSoundNum)`: `mSoundManager->GetSoundInstance(n)->Play(false, true)`.
pub fn vfunction55(g: &mut G, _this: Ptr, param_1: i32) {
    g.sound_requests.push(SoundRequest { id: param_1, volume: 1.0, pan: 0, pitch: 0.0 });
}

/// port: 004896e0 Sexy::SexyAppBase::vfunction54
/// `PlaySample(int theSoundNum, int thePan)`.
pub fn vfunction54(g: &mut G, _this: Ptr, param_1: i32, param_2: i32) {
    g.sound_requests.push(SoundRequest { id: param_1, volume: 1.0, pan: param_2, pitch: 0.0 });
}

/// port: 00481a10 Sexy::SexyAppBase::vfunction35
/// `SafeDeleteWidget(Widget*)`: queues the widget on `mDeferredDeletes` (deleted by the
/// next `ProcessSafeDeleteList`).
pub fn vfunction35(g: &mut G, this: Ptr, param_1: Ptr) {
    g.sab(this).field_0x4b8.push(param_1);
}

/// port: 0040f9e0 FUN_0040f9e0
/// `Sexy::Rand()`: the next number of the global generator `DAT_005eb928`.
pub fn FUN_0040f9e0(g: &mut G) -> u32 {
    crate::sexy::mt_rand::FUN_0040aeb0(&mut g.DAT_005eb928)
}

/// port: 0047b0a0 FUN_0047b0a0
/// `SetCursorImage(int num, Image*)`: stores it and re-applies the cursor (vtable +0x30).
pub fn FUN_0047b0a0(g: &mut G, this: Ptr, param_1: u32, param_2: Ptr) {
    if param_1 < 0xd {
        g.sab(this).field_0x3ac[param_1 as usize] = param_2;
        enforce_cursor(g, this);
    }
}

/// port: 0047de00 Sexy::SexyAppBase::vfunction65
/// `GetLoadingThreadProgress()`: 1 once loading failed, else completed/total tasks capped at 1.
pub fn vfunction65(g: &mut G, this: Ptr) -> f64 {
    let s = g.sab(this);
    if s.field_0x4f3 {
        return 1.0;
    }
    if s.field_0x4f1 && s.field_0x500 != 0 {
        let p = s.field_0x504 as f64 / s.field_0x500 as f64;
        return if p < 1.0 { p } else { 1.0 };
    }
    0.0
}

/// port: 0047abf0 Sexy::SexyAppBase::vfunction78
/// `KillDialog(int theDialogId)`: `KillDialog(theDialogId, true, true)` (vtable +0x138).
pub fn vfunction78(g: &mut G, this: Ptr, param_1: i32) -> bool {
    vfunction79(g, this, param_1, true, true)
}

/// port: 0047aa80 Sexy::SexyAppBase::vfunction79
/// `KillDialog(int theDialogId, bool removeWidget, bool deleteWidget)`: false when no dialog
/// has that id. A dialog without a result gets 0; modal dialogs end their modality.
pub fn vfunction79(g: &mut G, this: Ptr, param_1: i32, param_2: bool, param_3: bool) -> bool {
    let Some(&dialog) = g.sab(this).offset_0x320.get(&param_1) else { return false };
    let d = g.dialog(dialog);
    if d.ext_0x144 == -1 {
        d.ext_0x144 = 0;
    }
    let list = &mut g.sab(this).offset_0x32c;
    if let Some(i) = list.iter().position(|&p| p == dialog) {
        list.remove(i);
    }
    g.sab(this).offset_0x320.remove(&param_1);
    let wm = g.sab(this).offset_0x318;
    if param_2 || param_3 {
        vcall!(g, wm, w.vfunction5, dialog);
    }
    if vcall!(g, dialog, dlg.vfunction75) {
        // app vtable +0x144 `ModalClose()`: WinFishApp's is the empty function at 004e9e00.
        crate::game::game_object::vfunction75(g, this);
        crate::sexy::widget_manager::FUN_0046d1e0(g, wm, dialog);
    }
    if param_3 {
        vfunction35(g, this, dialog);
    }
    true
}

/// port: 0047ac30 Sexy::SexyAppBase::vfunction80
/// `GetDialogCount()`.
pub fn vfunction80(g: &mut G, this: Ptr) -> i32 {
    g.sab(this).offset_0x320.len() as i32
}

/// port: 0047a9c0 Sexy::SexyAppBase::vfunction73
/// `DoDialog(int theId, bool isModal, header, lines, footer, int theButtonMode)`: closes a
/// dialog with that id, makes a new one (`NewDialog`, vtable +0x64) and adds it
/// (`AddDialog`, vtable +0x12c). The app's virtuals resolve to WinFishApp's.
pub fn vfunction73(g: &mut G, this: Ptr, param_1: i32, param_2: bool, param_3: &[u8], param_4: &[u8], param_5: &[u8], param_6: i32) -> Ptr {
    crate::game::win_fish_app::vfunction78(g, this, param_1);
    let d = crate::game::win_fish_app::vfunction26(g, this, param_1, param_2, param_3, param_4, param_5, param_6);
    vfunction76(g, this, param_1, d);
    d
}

/// port: 0047ac40 Sexy::SexyAppBase::vfunction76
/// `AddDialog(int theId, Dialog*)`: replaces any dialog with that id; an unsized dialog gets
/// half the window's width, its preferred height, centered horizontally a fifth of the way
/// down. Modal dialogs become the base modal widget (`ModalOpen`, vtable +0x140, follows).
pub fn vfunction76(g: &mut G, this: Ptr, param_1: i32, param_2: Ptr) {
    crate::game::win_fish_app::vfunction78(g, this, param_1);
    if g.wc(param_2).offset_0x34 == 0 {
        let (aw, ah) = (g.sab(this).field_0xb8, g.sab(this).field_0xbc);
        let w = aw / 2;
        let h = vcall!(g, param_2, dlg.vfunction74, w);
        vcall!(g, param_2, w.vfunction41, (aw - w) / 2, ah / 5, w, h);
    }
    g.sab(this).offset_0x320.entry(param_1).or_insert(param_2);
    g.sab(this).offset_0x32c.push(param_2);
    let wm = g.sab(this).offset_0x318;
    vcall!(g, wm, w.vfunction4, param_2);
    if vcall!(g, param_2, dlg.vfunction75) {
        crate::sexy::widget_manager::FUN_0046d1d0(g, wm, param_2);
        crate::game::win_fish_app::vfunction81(g, this);
    }
}

/// Replacement for `SexyAppBase::CopyToClipboard(const std::string&)` (@ 00488ff0, Win32
/// clipboard): the text goes to the port's clipboard, `G::clipboard`.
pub fn copy_to_clipboard(g: &mut G, text: &[u8]) {
    g.clipboard = text.to_vec();
}

/// Replacement for `SexyAppBase::GetClipboard()` (@ 00489100, Win32 clipboard).
pub fn get_clipboard(g: &mut G) -> Vec<u8> {
    g.clipboard.clone()
}

/// port: 004872f0 FUN_004872f0
/// `SexyApp::GetString(const std::string& theId, const std::string& theDefault)`: the
/// `<String>` property with that id, else the default.
pub fn FUN_004872f0(g: &mut G, _this: Ptr, param_2: &str, param_3: &[u8]) -> Vec<u8> {
    match g.properties.strings.get(param_2) {
        Some(v) => v.as_bytes().to_vec(),
        None => param_3.to_vec(),
    }
}

/// port: 0047ad70 Sexy::SexyAppBase::vfunction1
/// `DialogButtonPress(int theDialogId, int theButtonId)` (the app's DialogListener part):
/// `ButtonPress(theDialogId + 2000)` for the yes/OK button, `+ 3000` for no/cancel (app
/// vtable +0x4; WinFishApp's is the empty function at 0051a440).
pub fn vfunction1(_g: &mut G, _this: Ptr, param_1: i32, param_2: i32) {
    if param_2 == 1000 {
        let _ = param_1 + 2000;
        crate::sexy::trivial::vfunction2__0051a440();
        return;
    }
    if param_2 == 0x3e9 {
        let _ = param_1 + 3000;
        crate::sexy::trivial::vfunction2__0051a440();
    }
}

/// port: 0047adc0 Sexy::SexyAppBase::vfunction2
/// `DialogButtonDepress(int theDialogId, int theButtonId)`: `ButtonDepress(theDialogId +
/// 2000)` / `+ 3000` (app vtable +0x8, WinFishApp's).
pub fn vfunction2(g: &mut G, this: Ptr, param_1: i32, param_2: i32) {
    if param_2 == 1000 {
        crate::game::win_fish_app::vfunction3(g, this, param_1 + 2000);
        return;
    }
    if param_2 == 0x3e9 {
        crate::game::win_fish_app::vfunction3(g, this, param_1 + 3000);
    }
}

/// port: 0047aa10 Sexy::SexyAppBase::vfunction74
/// `GetDialog(int theDialogId)`: null when no dialog has that id.
pub fn vfunction74(g: &mut G, this: Ptr, param_1: i32) -> Ptr {
    g.sab(this).offset_0x320.get(&param_1).copied().unwrap_or(NULL)
}

/// port: 004891f0 Sexy::SexyAppBase::vfunction66
/// `GetImage(const std::string& theFileName, bool commitBits)`: the image from the file
/// (`ImageLib::GetImage(path, true)`), or null. The Windows bitmap cache the original checks
/// first (`DDImage:` registry entries) does not exist in the port, and the DDImage/commit
/// steps are the renderer's.
pub fn vfunction66(g: &mut G, _this: Ptr, param_1: &str, _param_2: bool) -> Ptr {
    let Some(li) = crate::sexy::image_lib::FUN_0049fed0(&g.vfs, param_1, true) else { return NULL };
    let mut img = crate::sexy::image::Image::new();
    img.field_0x4 = param_1.to_string();
    img.offset_0x20 = li.mWidth;
    img.offset_0x24 = li.mHeight;
    img.mBits = li.mBits;
    g.alloc(Obj { vt: None, node: Node::Image(Box::new(img)) })
}

/// port: 00489530 FUN_00489530
/// `SexyAppBase::HSLToRGB(int h, int s, int l)` on 0..255 scales; returns 0xFFRRGGBB.
pub fn FUN_00489530(_g: &mut G, _this: Ptr, param_1: i32, param_2: i32, param_3: i32) -> u32 {
    let ftol = |x: f64| crate::sexy::crt::ftol(x) as i32;
    let v = if param_3 < 0x80 { ((param_2 + 0xff) * param_3) / 0xff } else { param_2 - (param_2 * param_3) / 0xff + param_3 };
    let m1 = ftol((param_3 * 2) as f64 - v as f64);
    let sextant = (param_1 * 6) / 256;
    let fract = (param_1 - (sextant * 256) / 6) * 6;
    let d = (fract as f64 * (v as f64 - m1 as f64)) / 255.0;
    let mut mid1 = ftol(m1 as f64 + d);
    if 0xff < mid1 {
        mid1 = 0xff;
    }
    let mut mid2 = ftol(v as f64 - d);
    if mid2 < 0 {
        mid2 = 0;
    }
    let (r, g, b) = match sextant {
        1 => (mid2, v, m1),
        2 => (m1, v, mid1),
        3 => (m1, mid2, v),
        4 => (mid1, m1, v),
        5 => (v, m1, mid2),
        _ => (v, mid1, m1),
    };
    ((((r as u32) | 0xffffff00) << 8 | g as u32) << 8) | b as u32
}

/// port: 00479f90 FUN_00479f90
/// `ClearUpdateBacklog(bool relaxForASecond)`: forgets the time owed to updates; the
/// host paces updates in the port, so these fields only mirror the original.
pub fn FUN_00479f90(g: &mut G, this: Ptr, param_1: bool) {
    let now = g.tick_count;
    let s = g.sab(this);
    s.field_0x460 = 0;
    s.field_0x468 = now;
    if param_1 {
        s.field_0xac = 1000;
    }
}

/// port: 004897f0 Sexy::SexyAppBase::vfunction57
/// `GetMusicVolume()`.
pub fn vfunction57(g: &mut G, this: Ptr) -> f64 {
    g.sab(this).field_0xc8
}

/// port: 00489840 Sexy::SexyAppBase::vfunction58
/// `GetSfxVolume()`.
pub fn vfunction58(g: &mut G, this: Ptr) -> f64 {
    g.sab(this).field_0xd0
}

/// port: 00489800 Sexy::SexyAppBase::vfunction61
/// `SetMusicVolume(double)`: remembered, and handed to the music interface (0 while
/// muted).
pub fn vfunction61(g: &mut G, this: Ptr, param_1: f64) {
    let s = g.sab(this);
    s.field_0xc8 = param_1;
    s.music_interface_volume = if 0 < s.field_0x420 { 0.0 } else { param_1 };
    let v = s.music_interface_volume;
    if s.offset_0x36c != NULL {
        crate::sexy::music::set_volume(&mut g.music, v);
    }
}

/// port: 00489850 Sexy::SexyAppBase::vfunction62
/// `SetSfxVolume(double)`: remembered, and handed to the sound manager (0 while muted).
pub fn vfunction62(g: &mut G, this: Ptr, param_1: f64) {
    let s = g.sab(this);
    s.field_0xd0 = param_1;
    s.sound_manager_volume = if 0 < s.field_0x420 { 0.0 } else { param_1 };
}

/// port: 004891d0 FUN_004891d0
/// `EnableCustomCursors(bool)`, then `EnforceCursor()`.
pub fn FUN_004891d0(g: &mut G, this: Ptr, param_1: bool) {
    g.sab(this).field_0x4f8 = param_1;
    enforce_cursor(g, this);
}

/// port: 00489a30 FUN_00489a30
/// `Is3DAccelerationSupported()`: the D3D tester ran without failures.
pub fn FUN_00489a30(g: &mut G, this: Ptr) -> bool {
    let s = g.sab(this);
    s.d3d_tester && s.d3d_tester_fail == 0
}

/// port: 00489a50 FUN_00489a50
/// `Is3DAccelerationRecommended()`: no failures and no warnings.
pub fn FUN_00489a50(g: &mut G, this: Ptr) -> bool {
    let s = g.sab(this);
    s.d3d_tester && s.d3d_tester_fail == 0 && s.d3d_tester_warn == 0
}

/// port: 00489ae0 FUN_00489ae0
/// `Set3DAcclerated(bool is3D, bool reinit)`: a change marks the display changed; with
/// `reinit` the display is rebuilt (the host's job: it renders either way) and the screen
/// redrawn. (The original's DirectDraw failure dialog has no counterpart.)
pub fn FUN_00489ae0(g: &mut G, this: Ptr, param_1: bool, param_2: bool) {
    if g.sab(this).ddinterface_is_3d == param_1 {
        return;
    }
    g.sab(this).field_0x5c2 = true;
    g.sab(this).ddinterface_is_3d = param_1;
    if param_2 {
        let wm = g.sab(this).offset_0x318;
        vcall!(g, wm, w.vfunction11);
    }
}

/// port: 00485ec0 Sexy::SexyAppBase::vfunction69
/// `SwitchScreenMode(bool wantWindowed, bool is3d, bool force)`: windowed only when the
/// desktop allows; an unchanged mode only switches 3D; otherwise the new mode is kept for
/// the host's window (which shows and focuses it) and the time noted.
pub fn vfunction69(g: &mut G, this: Ptr, param_1: bool, param_2: bool, param_3: bool) {
    let want = if g.sab(this).field_0x33e { false } else { param_1 };
    if g.sab(this).field_0x33b == want && !param_3 {
        FUN_00489ae0(g, this, param_2, true);
        return;
    }
    FUN_00489ae0(g, this, param_2, false);
    g.sab(this).field_0x33b = want;
    g.sab(this).field_0x464 = g.tick_count;
}

/// port: 0047eda0 Sexy::SexyAppBase::vfunction25
/// `ReadFromRegistry()`: the registry key from the `RegistryKey` property; under it the
/// music and sound volumes (in percent), the mute count, the screen mode (0 = windowed),
/// the window position, custom cursors, vertical sync, and whether the last run ended
/// cleanly (`InProgress`, which is then set to 1 until shutdown, outside the screensaver).
/// The registry is the host's replacement (`G::registry`).
pub fn vfunction25(g: &mut G, this: Ptr) {
    use crate::sexy::app_host::{registry_read_integer, registry_write};
    g.sab(this).field_0x370 = true;
    let def = g.sab(this).field_0x88.clone();
    let key = FUN_004872f0(g, this, "RegistryKey", &def);
    g.sab(this).field_0x88 = key;
    if g.sab(this).field_0x88.is_empty() {
        return;
    }
    if let Some(v) = registry_read_integer(g, "MusicVolume") {
        g.sab(this).field_0xc8 = v as f64 / 100.0;
    }
    if let Some(v) = registry_read_integer(g, "SfxVolume") {
        g.sab(this).field_0xd0 = v as f64 / 100.0;
    }
    if let Some(v) = registry_read_integer(g, "Muted") {
        g.sab(this).field_0x420 = v;
    }
    if let Some(v) = registry_read_integer(g, "ScreenMode") {
        g.sab(this).field_0x33b = v == 0;
    }
    if let Some(v) = registry_read_integer(g, "PreferredX") {
        g.sab(this).field_0xb0 = v;
    }
    if let Some(v) = registry_read_integer(g, "PreferredY") {
        g.sab(this).field_0xb4 = v;
    }
    if let Some(v) = registry_read_integer(g, "CustomCursors") {
        FUN_004891d0(g, this, v != 0);
    }
    if let Some(v) = registry_read_integer(g, "WaitForVSync") {
        g.sab(this).field_0x5c0 = v != 0;
    }
    if let Some(v) = registry_read_integer(g, "InProgress") {
        g.sab(this).field_0x4fa = v == 0;
    }
    if !crate::game::win_fish_app::FUN_00479fc0(g, this) {
        registry_write(g, "InProgress", "1".to_string());
    }
}

/// port: 0047e280 Sexy::SexyAppBase::vfunction24
/// `WriteToRegistry()`: the music and sound volumes in percent, muted, the screen mode
/// (1 = windowed... 0 when windowed is off), the window position, custom cursors,
/// `InProgress` = 0 (a clean exit) and vertical sync. (The replaced registry.)
pub fn vfunction24(g: &mut G, this: Ptr) {
    use crate::sexy::app_host::registry_write;
    let ftol = crate::sexy::crt::ftol;
    let s = g.sab(this);
    let music = ftol(s.field_0xc8 * 100.0) as i32;
    let sfx = ftol(s.field_0xd0 * 100.0) as i32;
    let muted = (s.field_0x420 != s.field_0x424 && -1 < s.field_0x420 - s.field_0x424) as i32;
    let mode = (!s.field_0x33b) as i32;
    let (px, py) = (s.field_0xb0, s.field_0xb4);
    let cursors = s.field_0x4f8 as i32;
    let vsync = s.field_0x5c0;
    registry_write(g, "MusicVolume", music.to_string());
    registry_write(g, "SfxVolume", sfx.to_string());
    registry_write(g, "Muted", muted.to_string());
    registry_write(g, "ScreenMode", mode.to_string());
    registry_write(g, "PreferredX", px.to_string());
    registry_write(g, "PreferredY", py.to_string());
    registry_write(g, "CustomCursors", cursors.to_string());
    registry_write(g, "InProgress", "0".to_string());
    // `RegistryWriteBoolean`.
    registry_write(g, "WaitForVSync", (vsync as i32).to_string());
}

/// port: 004922b0 Sexy::SexyApp::vfunction24_for_ButtonListener
/// `SexyApp::WriteToRegistry()`: the base's, then `LastVerCheckQueryTime`. (The original
/// also copies the registration name and code into temporaries it never writes.)
pub fn vfunction24_for_ButtonListener(g: &mut G, this: Ptr) {
    vfunction24(g, this);
    let t = g.app_obj(this).sa.offset_0x68;
    crate::sexy::app_host::registry_write(g, "LastVerCheckQueryTime", t.to_string());
}

/// port: 004936b0 Sexy::SexyApp::vfunction99_for_ButtonListener
/// `ShouldCheckForUpdate()`: never with "DontUpdate"; not within a week of the last check
/// once loading has finished.
pub fn vfunction99_for_ButtonListener(g: &mut G, this: Ptr) -> bool {
    if g.app_obj(this).sa.field_0x6d {
        return false;
    }
    let now = g.now_time64;
    let last = g.app_obj(this).sa.offset_0x68 as u32;
    if last != 0 && g.sab(this).field_0x4fa {
        let d = now - last as i64;
        if d < 0x93a81 {
            return false;
        }
    }
    true
}

/// port: 00493720 Sexy::SexyApp::vfunction100_for_ButtonListener
/// `UpdateCheckQueried()`: remembers when (`LastVerCheckQueryTime`, low 32 bits).
pub fn vfunction100_for_ButtonListener(g: &mut G, this: Ptr) -> i64 {
    let now = g.now_time64;
    g.app_obj(this).sa.offset_0x68 = now as i32;
    now
}

/// port: 00480170 Sexy::SexyAppBase::vfunction43
/// `Shutdown()`: once: marks shutdown and exit-to-top, `PreTerminate()` (empty in
/// WinFish), restores the volumes when +0x511 is set, stops the music, and writes the
/// registry when it was read; the host then ends the app (the original's
/// `PostQuitMessage`). Not reproduced, as the platform pieces are replaced: waiting for
/// the loading thread (it runs as main-schedule steps, never concurrently), the 3D
/// device cleanup, hiding the window and restoring the display mode.
pub fn vfunction43(g: &mut G, this: Ptr) {
    if g.sab(this).field_0x339 {
        return;
    }
    g.sab(this).field_0x33a = true;
    g.sab(this).field_0x339 = true;
    if g.sab(this).field_0x509 {
        let m = g.sab(this).field_0xd8;
        crate::game::win_fish_app::vfunction61(g, this, m);
        let s = g.sab(this).field_0xe0;
        vfunction62(g, this, s);
    }
    if g.sab(this).offset_0x36c != NULL {
        crate::sexy::app_host::music_stop_all(g);
    }
    if g.sab(this).field_0x370 {
        crate::game::win_fish_app::vfunction24(g, this);
    }
}
