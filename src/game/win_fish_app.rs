//! `Sexy::WinFishApp`: construction, `Init`, `Start` and the loading thread.

use crate::game::app::WinFishApp as AppData;
use crate::sexy::prelude::*;

/// port: 0054cb00 FUN_0054cb00
/// Allocates an empty `std::list` head (the two lists at WinFishApp +0x758/+0x764).
pub fn FUN_0054cb00() -> Vec<Ptr> {
    Vec::new()
}

/// port: 0054d850 Sexy::WinFishApp::WinFishApp
/// `new WinFishApp`: the SexyApp/SexyAppBase parts get their defaults, then the game's own.
/// The game's `MTRand` (+0x7b0) is seeded with `Sexy::Rand()` from the global generator.
pub fn WinFishApp(g: &mut G) -> Ptr {
    let mut a = AppData::default();
    a.wfa.offset_0x2c = FUN_0054cb00();
    a.wfa.offset_0x38 = FUN_0054cb00();
    a.sab.field_0x44c = 0x1c;
    a.sab.field_0x5c3 = true;
    // SexyAppBase::SexyAppBase: mSfxVolume 0.85, windowed.
    a.sab.field_0xd0 = f64::from_bits(0x3feb333333333333);
    a.sab.field_0x33b = true;
    a.sab.field_0xc8 = 1.5;
    a.sab.field_0x4f8 = false;
    a.sab.field_0x4ca = true;
    a.sab.field_0x3c = b"Insaniquarium".to_vec();
    a.sab.field_0x504 = 0;
    a.wfa.offset_0x18c = NULL;
    a.wfa.offset_0x78 = -1;
    a.wfa.offset_0x7c = -1;
    a.wfa.offset_0x80 = -1;
    a.wfa.offset_0x157 = true;
    a.wfa.offset_0x150 = 0;
    a.wfa.offset_0x155 = true;
    a.wfa.offset_0x156 = true;
    a.wfa.offset_0x10d = true;
    a.wfa.offset_0x10e = true;
    a.wfa.offset_0x10f = true;
    a.wfa.offset_0x15c = 1;
    a.wfa.offset_0x160 = -1;
    a.wfa.offset_0x164 = -1;
    a.wfa.field_0x40 = b"ScreenSaver\\".to_vec();
    a.wfa.field_0x5c = a.wfa.field_0x40.clone();
    let p = g.alloc(Obj { vt: None, node: Node::App(Box::new(a)) });
    // gSexyAppBase / gSexyApp = this (set by the base constructors).
    g.globals.DAT_005eb6a4 = p;
    let seed = crate::sexy::sexy_app_base::FUN_0040f9e0(g);
    let rng = g.alloc(Obj { vt: None, node: Node::MTRand(Box::new(crate::sexy::mt_rand::FUN_0040ae10(seed))) });
    g.wfa(p).offset_0x84 = rng;
    let mgr = crate::game::profile_mgr::ProfileMgr(g);
    g.wfa(p).offset_0x184 = mgr;
    let hs = crate::game::high_score::HighScoreMgr(g);
    g.wfa(p).offset_0x180 = hs;
    g.wfa(p).offset_0x188 = 0;
    let day = FUN_005005b0(g);
    g.wfa(p).offset_0x1c = day;
    g.wfa(p).offset_0x14 = day;
    g.wfa(p).offset_0x24 = 0;
    p
}

/// port: 005005b0 FUN_005005b0
/// The current day number: local time truncated to midnight, in days since 1970, plus the
/// debug day offset `DAT_005e8f1c`.
pub fn FUN_005005b0(g: &mut G) -> i64 {
    let mut t = g.now_time64;
    if t < 0 {
        t = 0;
    }
    let local = t + g.utc_offset_secs;
    let secs_today = local.rem_euclid(86400);
    let midnight = t - secs_today;
    midnight / 0x15180 + g.globals.DAT_005e8f1c as i64
}

/// port: 00479fc0 FUN_00479fc0
/// `SexyAppBase::IsScreenSaver()` (+0x358).
pub fn FUN_00479fc0(g: &mut G, this: Ptr) -> bool {
    g.sab(this).field_0x350
}

/// port: 005503c0 Sexy::WinFishApp::vfunction50
/// `Init()`.
pub fn vfunction50(g: &mut G, this: Ptr) {
    // ParseCommandLine (vtable +0xac): no arguments in the port.
    g.sab(this).field_0xea = true;
    if FUN_00479fc0(g, this) {
        g.globals.DAT_005e6f16 = false;
    }
    // SexyApp::Init -> SexyAppBase::Init (window, DirectDraw, managers): replaced.
    crate::sexy::app_host::sexy_app_init(g, this);
    // (Vista user-data folders, the screensaver install path in the registry: Windows-only,
    // replaced by the in-memory file system.)
    if !g.resource_manager.parse_resources_file(&g.vfs.clone(), "properties\\resources.xml") {
        vfunction107(g, this, true);
        return;
    }
    if crate::sexy::resource_manager::load_resources(g, "Init")
        && crate::game::res_extract_gen::FUN_00506ea0(g)
        && FUN_0050e6b0(g, 0xca) != NULL
    {
        let mgr = g.wfa(this).offset_0x184;
        crate::game::profile_mgr::FUN_00514c00(g, mgr);
        let hsm = g.wfa(this).offset_0x180;
        crate::game::high_score::FUN_00514030(g, hsm);
        let mut cur_user = None;
        if let Some(name) = crate::sexy::app_host::registry_read_string(g, "LastUser") {
            if FUN_00479fc0(g, this) && g.wfa(this).offset_0x144 != 0 {
                cur_user = Some(name);
            }
        }
        if let Some(name) = cur_user {
            let mgr = g.wfa(this).offset_0x184;
            let p = crate::game::profile_mgr::FUN_00514eb0(g, mgr, name.as_bytes());
            g.wfa(this).offset_0x18c = p;
        }
        if g.wfa(this).offset_0x18c == NULL {
            if let Some(name) = crate::sexy::app_host::registry_read_string(g, "CurUser") {
                let mgr = g.wfa(this).offset_0x184;
                let p = crate::game::profile_mgr::FUN_00514eb0(g, mgr, name.as_bytes());
                g.wfa(this).offset_0x18c = p;
            }
        }
        if g.wfa(this).offset_0x18c == NULL {
            let mgr = g.wfa(this).offset_0x184;
            let p = crate::game::profile_mgr::FUN_00514bc0(g, mgr);
            g.wfa(this).offset_0x18c = p;
        }
        if !FUN_00479fc0(g, this) || g.wfa(this).offset_0x18c != NULL {
            g.wfa(this).offset_0x170 = crate::sexy::app_host::FUN_004871e0(g, "MaxExecutions", 0);
            g.wfa(this).offset_0x174 = crate::sexy::app_host::FUN_004871e0(g, "MaxPlays", 0);
            g.wfa(this).offset_0x178 = crate::sexy::app_host::FUN_004871e0(g, "MaxTime", 0x3c);
            // SetMusicVolume(mMusicVolume) (vtable +0xf0)
            let vol = g.sab(this).field_0xc8;
            vfunction61(g, this, vol);
            let (title, mask) = (g.res.DAT_005e8bec, g.res.DAT_005e8e80);
            let img = FUN_00503120(g, title, mask, 0x180, 0x184);
            FUN_0050e770(g, 6, img);
            g.wfa(this).offset_0x10 = vec![0; 0x518 / 4];
            let ts = crate::game::title_screen::TitleScreen(g, this);
            g.wfa(this).offset_0x8 = ts;
            let (w, h) = (g.sab(this).field_0xb8, g.sab(this).field_0xbc);
            vcall!(g, ts, w.vfunction41, 0, 0, w, h);
            let wm = g.sab(this).offset_0x318;
            vcall!(g, wm, w.vfunction4, ts);
            if !FUN_00479fc0(g, this) {
                // mMusicInterface->LoadMusic (vtable +0x4): the MO3 tracker modules.
                crate::sexy::music::load_music(&mut g.music, 1, "music\\Alien.mo3");
                crate::sexy::music::load_music(&mut g.music, 3, "music\\Lullaby.mo3");
                crate::sexy::music::load_music(&mut g.music, 4, "music\\Insaniq2.mo3");
                FUN_0054b1a0(g, this, 2, 0x2d, false);
            }
            let cursors = [g.res.DAT_005e8a4c, g.res.DAT_005e8e20, g.res.DAT_005e8a80, g.res.DAT_005e8b50];
            for (i, c) in cursors.into_iter().enumerate() {
                crate::sexy::sexy_app_base::FUN_0047b0a0(g, this, i as u32, c);
            }
        } else {
            // "No User Profile" message box (vtable +0x84), then do not start.
            g.sab(this).field_0x4f5 = true;
        }
        return;
    }
    vfunction107(g, this, true);
}

/// port: 0050e6b0 FUN_0050e6b0
/// `LoadImageById(ResourceManager*, int id)`: loads that image resource now and stores it in
/// its global.
pub fn FUN_0050e6b0(g: &mut G, param_2: i32) -> Ptr {
    let addr = crate::sexy::res_gen::RES_BY_ID[param_2 as usize];
    let id = crate::sexy::res::FUN_005016c0(param_2);
    let p = crate::sexy::resource_manager::load_image_now(g, id);
    g.res.set(addr, p as i32);
    p
}

/// port: 0050e770 FUN_0050e770
/// `SetImageById(ResourceManager*, int id, Image*)`: replaces the image resource and its global.
pub fn FUN_0050e770(g: &mut G, param_2: i32, param_3: Ptr) {
    let addr = crate::sexy::res_gen::RES_BY_ID[param_2 as usize];
    let id = crate::sexy::res::FUN_005016c0(param_2);
    {
        if let Some(r) = g.resource_manager.images.get_mut(id) {
            r.image = param_3;
        }
    }
    g.res.set(addr, param_3 as i32);
}

/// port: 005502c0 Sexy::WinFishApp::vfunction107
/// `ShowResourceError(bool doExit)`.
pub fn vfunction107(g: &mut G, this: Ptr, param_1: bool) {
    let err = g.resource_manager.error.clone().unwrap_or_default();
    bevy::log::error!("resource error: {err}");
    if param_1 {
        g.sab(this).field_0x339 = true;
    }
}

/// The loading thread's last part: the tiled post and rail strips (resources 0x51, 0x50),
/// the masked tank-backdrop corners (0xa5..0xaa) and the 40x40 pet icons (the app's table
/// +0x328..; Amp's from a wider strip of its sheet). Each counts as a finished task.
fn loading_thread_generated_images(g: &mut G, this: Ptr) {
    let post = g.res.DAT_005e8cbc;
    let rail = g.res.DAT_005e8a58;
    let (pw, ph) = (g.image(post).offset_0x20, g.image(post).offset_0x24);
    let (rw, rh) = (g.image(rail).offset_0x20, g.image(rail).offset_0x24);
    let a = crate::sexy::blit::new_memory_image(g, pw, 0x3c);
    let b = crate::sexy::blit::new_memory_image(g, 0x3c, rh);
    crate::sexy::blit::render_offscreen(g, a, |g, gfx| {
        let mut y = 0;
        while y < 0x3c {
            crate::sexy::graphics::FUN_00455d20(gfx, g, post, 0, y);
            y += ph;
        }
    });
    crate::sexy::blit::render_offscreen(g, b, |g, gfx| {
        let mut x = 0;
        while x < 0x3c {
            crate::sexy::graphics::FUN_00455d20(gfx, g, rail, x, 0);
            x += rw;
        }
    });
    FUN_0050e770(g, 0x51, a);
    FUN_0050e770(g, 0x50, b);
    for id in 0xa5..0xab {
        let mask = crate::sexy::res::FUN_005016a0(g, id) as Ptr;
        let src = crate::sexy::res::FUN_005016a0(g, id - 6) as Ptr;
        let img = FUN_00503120(g, src, mask, 0, 0x16d);
        FUN_0050e770(g, id, img);
        if g.sab(this).field_0x339 {
            return;
        }
        g.sab(this).field_0x504 += 1;
    }
    for i in 0..0x18 {
        let src = crate::sexy::res::FUN_005016a0(g, i + 0xe8) as Ptr;
        let dst = crate::sexy::blit::new_memory_image(g, 0x28, 0x28);
        let (sw, sh) = (g.image(src).offset_0x20, g.image(src).offset_0x24);
        let (dw, dh) = (0x28, 0x28);
        let (d, s) = if i == 0x10 {
            (Rect::new(0, 10, dw, dh - 0x14), Rect::new(0, 0, sw.min(0x40), sh.min(0x18)))
        } else {
            (Rect::new(0, 0, dw, dh), Rect::new(0, 0, sw.min(0x3c), sh.min(0x3c)))
        };
        crate::sexy::blit::render_offscreen(g, dst, |g, gfx| {
            crate::sexy::graphics::FUN_00455900(gfx, false);
            crate::sexy::graphics::FUN_00456260(gfx, src, &d, &s);
            let _ = g;
        });
        g.wfa(this).offset_0x10[0x328 / 4 + i as usize] = dst as i32;
        if g.sab(this).field_0x339 {
            return;
        }
        g.sab(this).field_0x504 += 1;
    }
}

/// port: 0054cad0 Sexy::WinFishApp::vfunction49
/// `Start()`: the framework main loop (the Bevy host drives it) unless Init asked not to start.
pub fn vfunction49(g: &mut G, this: Ptr) {
    if !g.sab(this).field_0x4f5 {
        // SexyAppBase::Start: the loading thread starts with the main loop.
        g.sab(this).field_0x4f1 = true;
        g.loading_thread = Some(LoadingThread::default());
        return;
    }
    if FUN_00479fc0(g, this) && !g.globals.DAT_005e9634 {
        crate::game::sim_setup::FUN_0054ca60(g, this);
    }
}

/// `WinFishApp::LoadingThreadProc` (vfunction23, @ 005510b0) as resumable steps: the
/// original runs it on a thread while the title screen animates; the port runs a slice of
/// it every frame on the main schedule (Bevy systems may not share `G` with a thread).
#[derive(Debug, Default, Clone)]
pub struct LoadingThread {
    pub phase: u32,
    pub group: usize,
    pub counted: bool,
}

const LOADING_GROUPS: [&str; 2] = ["Register", "LoadingThread"];

/// port: 005510b0 Sexy::WinFishApp::vfunction23
/// `LoadingThreadProc()`, one slice per call (see [`LoadingThread`]): counts the work, loads
/// the "Register" and "LoadingThread" resource groups (a failure ends the app), then makes
/// the generated images. Returns false when the thread has finished.
pub fn vfunction23(g: &mut G, this: Ptr, lt: &mut LoadingThread) -> bool {
    if !lt.counted {
        for grp in LOADING_GROUPS {
            let n = g.resource_manager.get_num_resources(grp);
            g.sab(this).field_0x500 += n;
        }
        g.sab(this).field_0x500 += 0x21;
        lt.counted = true;
        g.resource_manager.start_load_resources(LOADING_GROUPS[0]);
    }
    if lt.phase == 0 {
        if g.sab(this).field_0x339 {
            return false;
        }
        if crate::sexy::resource_manager::load_next_resource(g) {
            g.sab(this).field_0x504 += 1;
            return true;
        }
        let grp = LOADING_GROUPS[lt.group];
        let ok = g.resource_manager.error.is_none()
            && crate::sexy::res::FUN_0050ff10(g, grp);
        if !ok {
            vfunction107(g, this, false);
            g.sab(this).field_0x4f5 = true;
            return false;
        }
        g.wfa(this).offset_0x14c = 1;
        lt.group += 1;
        if lt.group < LOADING_GROUPS.len() {
            g.resource_manager.start_load_resources(LOADING_GROUPS[lt.group]);
            return true;
        }
        lt.phase = 1;
        return true;
    }
    // Phase 1: the generated images (the 0x21 extra tasks).
    loading_thread_generated_images(g, this);
    g.sab(this).field_0x4fa = true;
    false
}

/// port: 00552560 Sexy::WinFishApp::vfunction105
/// `TitleScreenIsFinished()`: removes the title screen and drops its images.
pub fn vfunction105(g: &mut G, this: Ptr) {
    let wm = g.sab(this).offset_0x318;
    let ts = g.wfa(this).offset_0x8;
    vcall!(g, wm, w.vfunction5, ts);
    crate::sexy::sexy_app_base::vfunction35(g, this, ts);
    g.wfa(this).offset_0x8 = NULL;
    for id in ["IMAGE_TITLEPAGE", "IMAGE_LOADERBAR", "IMAGE_LOADERBAROVER", "IMAGE_LOADERBAROVER2", "IMAGE_LOADERPLAY", "IMAGE_TITLEPAGEMASK"] {
        crate::sexy::resource_manager::delete_image(g, id);
    }
    if FUN_00479fc0(g, this) {
        g.wfa(this).offset_0x150 = 5;
        FUN_00552380(g, this, true, true);
        crate::sexy::sexy_app_base::FUN_00479f90(g, this, true);
        return;
    }
    FUN_00552100(g, this);
}

/// port: 00503120 FUN_00503120
/// `CreateMaskedImage(Image* src, Image* mask, int x, int y)`: copies the part of `src`
/// under `mask` placed at (x, y) into a new image and gives it the mask's alpha.
pub fn FUN_00503120(g: &mut G, param_1: Ptr, param_2: Ptr, param_3: i32, param_4: i32) -> Ptr {
    let (mw, mh) = (g.image(param_2).offset_0x20, g.image(param_2).offset_0x24);
    let (sw, sh) = (g.image(param_1).offset_0x20, g.image(param_1).offset_0x24);
    let r = crate::sexy::graphics::FUN_00468430(&Rect::new(param_3, param_4, mw, mh), &Rect::new(0, 0, sw, sh));
    if !(0 < r.mWidth && 0 < r.mHeight) {
        return NULL;
    }
    let img = crate::sexy::blit::new_memory_image(g, r.mWidth, r.mHeight);
    crate::sexy::blit::render_offscreen(g, img, |g, gfx| {
        crate::sexy::graphics::FUN_00455d20(gfx, g, param_1, -r.mX, -r.mY);
    });
    let n = (r.mWidth * r.mHeight) as usize;
    let mask_bits: Vec<u32> = g.image(param_2).mBits.iter().take(n).copied().collect();
    let bits = &mut g.image(img).mBits;
    for (p, m) in bits.iter_mut().zip(mask_bits) {
        *p = (*p & 0x00ff_ffff) | (m & 0xff00_0000);
    }
    img
}

/// `thunk_FUN_00479fc0` (@ 0054abe0, a jump to `IsScreenSaver`; the database has no entry
/// for the thunk itself).
pub fn thunk_FUN_00479fc0(g: &mut G, this: Ptr) -> bool {
    FUN_00479fc0(g, this)
}

/// port: 0054eca0 FUN_0054eca0
/// `PauseGame()` (space bar): with a board up, outside the screensaver and not already
/// paused, the board pauses, the dialogs close and "GAME PAUSED" (dialog 0x12) opens.
pub fn FUN_0054eca0(g: &mut G, this: Ptr) {
    let board = g.wfa(this).offset_0x4;
    if board == NULL || thunk_FUN_00479fc0(g, this) || g.board(board).field_0x8 {
        return;
    }
    crate::game::board_update::FUN_0053db80(g, board, true);
    FUN_0054b4c0(g, this);
    vfunction73(g, this, 0x12, true, b"GAME PAUSED", b"Click to resume game", b"Resume Game", 3);
}

/// port: 00551d50 Sexy::WinFishApp::vfunction84
/// `LostFocus()`: a game in progress (outside the virtual tank) pauses.
pub fn vfunction84(g: &mut G, this: Ptr) {
    if g.wfa(this).offset_0x4 != NULL && g.wfa(this).offset_0x150 != 5 {
        FUN_0054eca0(g, this);
    }
}

/// `DAT_005e28cc`: the songs live in one module (`Insaniq2.mo3`, music 4) and the old
/// music ids 0 and 2 are mapped onto its orders. Initialised to 1 in the data section and
/// never written.
pub const DAT_005e28cc: bool = true;

/// port: 0054b090 FUN_0054b090
/// Maps a (music, order) pair onto the combined module (registers: music in ESI, order in
/// ECX): music 0 orders 0x16/0/0xd and music 2 orders 0x20/0x2d/0/0x36/5 become music 4
/// orders 0/0xc/0x3a and 0x19/0x25/0x2d/0x31/0x32.
pub fn FUN_0054b090(music: &mut i32, order: &mut i32) {
    if !DAT_005e28cc {
        return;
    }
    if *music == 0 {
        if *order == 0x16 {
            *order = 0;
            *music = 4;
            return;
        }
        if *order == 0 {
            *order = 0xc;
            *music = 4;
            return;
        }
    }
    if *music == 2 {
        let to = match *order {
            0x20 => Some(0x19),
            0x2d => Some(0x25),
            0 => Some(0x2d),
            0x36 => Some(0x31),
            5 => Some(0x32),
            _ => None,
        };
        if let Some(o) = to {
            *order = o;
            *music = 4;
            return;
        }
    }
    if *music == 0 && *order == 0xd {
        *order = 0x3a;
        *music = 4;
    }
}

/// port: 0054b150 FUN_0054b150
/// The fade-out side of `FUN_0054b090`: music 0 and 2 are music 4.
pub fn FUN_0054b150(music: &mut i32) {
    if DAT_005e28cc && (*music == 0 || *music == 2) {
        *music = 4;
    }
}

/// port: 0054b1a0 FUN_0054b1a0
/// `PlayMusic(int music, int order, bool noLoop)`: remembers what was asked (+0x88c,
/// +0x890), maps it onto the combined module and starts it (`MusicInterface::PlayMusic`).
pub fn FUN_0054b1a0(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: bool) {
    g.wfa(this).offset_0x164 = param_2;
    g.wfa(this).offset_0x160 = param_1;
    let (mut music, mut order) = (param_1, param_2);
    FUN_0054b090(&mut music, &mut order);
    crate::sexy::app_host::music_play(g, music, order, param_3);
}

/// port: 0054b1e0 FUN_0054b1e0
/// `FadeInMusic(int music, int order, double speed, bool noLoop)`: as `PlayMusic`, fading in.
pub fn FUN_0054b1e0(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: f64, param_4: bool) {
    g.wfa(this).offset_0x164 = param_2;
    g.wfa(this).offset_0x160 = param_1;
    let (mut music, mut order) = (param_1, param_2);
    FUN_0054b090(&mut music, &mut order);
    crate::sexy::app_host::music_fade_in(g, music, order, param_3, param_4);
}

/// port: 0054b230 FUN_0054b230
/// `FadeOutMusic(int music, bool stopSong, double speed)`: forgets the current music if it
/// is this one, then fades the (mapped) music out.
pub fn FUN_0054b230(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: f64) {
    if g.wfa(this).offset_0x160 == param_1 {
        g.wfa(this).offset_0x160 = -1;
    }
    let mut music = param_1;
    FUN_0054b150(&mut music);
    crate::sexy::app_host::music_fade_out(g, music, param_2 != 0, param_3);
}

/// port: 005516c0 Sexy::WinFishApp::vfunction38
/// `OpenURL(const string& url, bool shutdownOnOpen)`: "Opening Browser" (dialog 9, no
/// buttons), drawn at once (`DrawDirtyStuff`), then the framework's `OpenURL`.
pub fn vfunction38__005516c0(g: &mut G, this: Ptr, param_1: &[u8], param_2: bool) {
    vfunction73(g, this, 9, true, b"Opening Browser", b"Opening Browser", b"", 0);
    crate::game::boot::draw_frame(g);
    crate::sexy::app_host::open_url(g, this, param_1, param_2);
}

/// port: 00552100 FUN_00552100
/// `ShowGameSelector()`: ends any game in progress, replaces the selector with a new one,
/// asks for a name when there is no profile, and shows the trial-expired dialog when due.
pub fn FUN_00552100(g: &mut G, this: Ptr) {
    FUN_0054bc30(g, this);
    FUN_0054ac80(g, this);
    let wm = g.sab(this).offset_0x318;
    let old = g.wfa(this).offset_0xc;
    if old != NULL {
        vcall!(g, wm, w.vfunction5, old);
        crate::sexy::sexy_app_base::vfunction35(g, this, old);
    }
    let profile = g.wfa(this).offset_0x18c;
    if profile != NULL {
        crate::game::profile::FUN_00514b00(g, profile);
    }
    let sel = crate::game::game_selector::GameSelectorOverlay(g, this);
    g.wfa(this).offset_0xc = sel;
    let (w, h) = (g.sab(this).field_0xb8, g.sab(this).field_0xbc);
    vcall!(g, sel, w.vfunction41, 0, 0, w, h);
    vcall!(g, wm, w.vfunction4, sel);
    vcall!(g, wm, w.vfunction13, sel);
    vcall!(g, wm, w.vfunction9, sel);
    if g.wfa(this).offset_0x18c == NULL {
        FUN_0054b6a0(g, this);
    }
    if FUN_0054bbb0(g, this) {
        crate::game::register_dialog::FUN_0054ea40(g, this);
    }
}

/// port: 0054bc30 FUN_0054bc30
/// `EndGame()`: closes the in-game dialogs; with a board up, saves it, marks a play when more
/// than two minutes were played, and removes it.
pub fn FUN_0054bc30(g: &mut G, this: Ptr) {
    FUN_0054b4c0(g, this);
    let board = g.wfa(this).offset_0x4;
    if board != NULL {
        crate::game::board::FUN_005497a0(g, board);
        if 0x78 < crate::game::board::FUN_00538040(g, board) {
            g.wfa(this).offset_0x158 = true;
        }
        let wm = g.sab(this).offset_0x318;
        vcall!(g, wm, w.vfunction5, board);
        crate::sexy::sexy_app_base::vfunction35(g, this, board);
        g.wfa(this).offset_0x4 = NULL;
    }
}

/// port: 0054b4c0 FUN_0054b4c0
/// Closes dialogs 0x1f and 0x20.
pub fn FUN_0054b4c0(g: &mut G, this: Ptr) {
    FUN_0054b450(g, this, -1);
    FUN_0054b4b0(g, this);
}

/// port: 0054b450 FUN_0054b450
/// Closes dialog 0x1f; with `param_1 >= 0`, a pending `offset_0x188` and a board, passes
/// `param_1` to the board's object at `FUN_00539540` (its vtable +0x124).
pub fn FUN_0054b450(g: &mut G, this: Ptr, param_1: i32) {
    vfunction78(g, this, 0x1f);
    if -1 < param_1 && g.wfa(this).offset_0x188 != 0 {
        let board = g.wfa(this).offset_0x4;
        if board != NULL {
            let p = crate::game::board_level::FUN_00539540(g, board);
            if p != NULL {
                vcall!(g, p, go.vfunction74, param_1);
            }
        }
    }
    g.wfa(this).offset_0x188 = 0;
}

/// port: 0054b4b0 FUN_0054b4b0
/// Closes dialog 0x20; whether there was one (the `KillDialog` result is left in AL).
pub fn FUN_0054b4b0(g: &mut G, this: Ptr) -> bool {
    vfunction78(g, this, 0x20)
}

/// port: 0054ac80 FUN_0054ac80
/// `UpdateTrialCounters()`: folds this session's plays (or minutes, for a time-limited trial)
/// into `mTimesPlayed`. (The original leaves a leftover value in EAX; no caller reads it.)
pub fn FUN_0054ac80(g: &mut G, this: Ptr) {
    if g.wfa(this).offset_0x158 {
        g.wfa(this).offset_0x16c += 1;
        g.wfa(this).offset_0x158 = false;
    }
    if 0 < g.wfa(this).offset_0x178 {
        let u = (g.wfa(this).offset_0x168 as u32).wrapping_mul(g.sab(this).field_0x44c as u32);
        g.app_obj(this).sa.offset_0xe8 = g.app_obj(this).sa.offset_0xe8.wrapping_add((u / 60000) as i32);
        g.wfa(this).offset_0x16c = 0;
        g.wfa(this).offset_0x168 = 0;
        return;
    }
    if 0 < g.wfa(this).offset_0x174 {
        let n = g.wfa(this).offset_0x16c;
        g.app_obj(this).sa.offset_0xe8 += n;
    }
    g.wfa(this).offset_0x16c = 0;
    g.wfa(this).offset_0x168 = 0;
}

/// port: 0054bbb0 FUN_0054bbb0
/// `IsTrialExpired()`: never for a registered copy or the screensaver; otherwise by minutes
/// (`MaxTime`), plays (`MaxPlays`) or executions (`MaxExecutions`), whichever is configured.
pub fn FUN_0054bbb0(g: &mut G, this: Ptr) -> bool {
    if g.app_obj(this).sa.offset_0xe4 {
        return false;
    }
    if thunk_FUN_00479fc0(g, this) {
        return false;
    }
    FUN_0054ac80(g, this);
    let a = &g.app_obj(this);
    let (max_time, max_plays, max_exec) = (a.wfa.offset_0x178, a.wfa.offset_0x174, a.wfa.offset_0x170);
    let (played, executed, playing) = (a.sa.offset_0xe8, a.sa.offset_0xec, a.wfa.offset_0x158);
    if max_time < 1 {
        if max_plays < 1 {
            if 0 < max_exec && (max_exec - executed) + 1 < 1 {
                return true;
            }
        } else {
            let mut left = max_plays - played;
            if playing {
                left -= 1;
            }
            if left < 1 {
                return true;
            }
        }
    } else if max_time - played < 1 {
        return true;
    }
    false
}

/// port: 0054cfc0 Sexy::WinFishApp::vfunction78
/// `KillDialog(int)`: after a dialog closes and none is left, gives focus back to the
/// in-game dialog, the board or the selector; and unpauses the board when nothing else
/// keeps it paused.
pub fn vfunction78(g: &mut G, this: Ptr, param_1: i32) -> bool {
    if !crate::sexy::sexy_app_base::vfunction78(g, this, param_1) {
        return false;
    }
    if g.sab(this).offset_0x320.is_empty() {
        let w = g.wfa(this);
        let target = if w.offset_0xe4 != NULL {
            w.offset_0xe4
        } else if w.offset_0x4 != NULL {
            w.offset_0x4
        } else {
            w.offset_0xc
        };
        if target != NULL {
            let wm = g.sab(this).offset_0x318;
            vcall!(g, wm, w.vfunction9, target);
        }
    }
    let board = g.wfa(this).offset_0x4;
    if board != NULL && !FUN_0054cf40(g, this) {
        crate::game::board_update::FUN_0053db80(g, board, false);
    }
    true
}

/// port: 0054cf40 FUN_0054cf40
/// Whether something keeps the game paused: one of four overlay screens, or a dialog other
/// than the in-game messages 0x1f/0x20.
pub fn FUN_0054cf40(g: &mut G, this: Ptr) -> bool {
    let w = g.wfa(this);
    if w.offset_0xe4 == NULL && w.offset_0xe0 == NULL && w.offset_0xf8 == NULL && w.offset_0xfc == NULL {
        if !g.sab(this).offset_0x32c.is_empty() {
            let d = crate::game::win_fish_app::FUN_0054cf10(g, this);
            let id = g.dialog(d).ext_0x13c;
            if id != 0x1f && id != 0x20 {
                return true;
            }
        }
        return false;
    }
    true
}

/// port: 0054b170 FUN_0054b170
/// Whether song (`param_1`, `param_2`) is the one playing.
pub fn FUN_0054b170(g: &mut G, this: Ptr, param_1: i32, param_2: i32) -> bool {
    g.wfa(this).offset_0x160 == param_1 && g.wfa(this).offset_0x164 == param_2
}

/// port: 0054b020 FUN_0054b020
/// `StopAllMusic()`: stops music channels 0..4 (`mMusicInterface` vtable +0xc) and forgets
/// the current song. Without a music interface (see `offset_0x36c`) nothing happens.
pub fn FUN_0054b020(g: &mut G, this: Ptr) {
    if g.sab(this).offset_0x36c != NULL {
        for i in 0..5 {
            crate::sexy::app_host::music_stop(g, i);
        }
        g.wfa(this).offset_0x160 = -1;
        g.wfa(this).offset_0x164 = -1;
    }
}

/// port: 0054b5a0 Sexy::WinFishApp::vfunction73
/// `DoDialog(int theId, bool isModal, const string& theHeader, const string& theLines, const
/// string& theFooter, int theButtonMode)`: the framework's.
pub fn vfunction73(g: &mut G, this: Ptr, param_1: i32, param_2: bool, param_3: &[u8], param_4: &[u8], param_5: &[u8], param_6: i32) -> Ptr {
    crate::sexy::sexy_app_base::vfunction73(g, this, param_1, param_2, param_3, param_4, param_5, param_6)
}

/// port: 0054cf90 Sexy::WinFishApp::vfunction81
/// `ModalOpen()`: pauses the board when something now keeps it paused.
pub fn vfunction81(g: &mut G, this: Ptr) {
    let board = g.wfa(this).offset_0x4;
    if board != NULL && FUN_0054cf40(g, this) {
        crate::game::board_update::FUN_0053db80(g, board, true);
    }
}

/// port: 0054b6a0 FUN_0054b6a0
/// `ShowNewUserDialog()`: dialog 0x19, 400 pixels wide, centered.
pub fn FUN_0054b6a0(g: &mut G, this: Ptr) {
    vfunction78(g, this, 0x19);
    let d = crate::game::money_dialog::FUN_00534740(g, this, false);
    let h = vcall!(g, d, dlg.vfunction74, 400);
    let (aw, ah) = (g.sab(this).field_0xb8, g.sab(this).field_0xbc);
    vcall!(g, d, w.vfunction41, (aw + -400) / 2, (ah - h) / 2, 400, h);
    crate::sexy::sexy_app_base::vfunction76(g, this, 0x19, d);
}

/// port: 005527c0 Sexy::WinFishApp::vfunction3
/// `ButtonDepress(int theId)`: dialog results arrive as dialog id + 2000 (yes/OK) or + 3000
/// (no/cancel), decided on `theId % 10000`; each closes its dialog and acts.
pub fn vfunction3(g: &mut G, this: Ptr, param_1: i32) {
    let r = param_1 % 10000;
    if r < 2000 {
        return;
    }
    if r < 3000 {
        let id = param_1 - 2000;
        if 0x2716 < id {
            if id == 0x4e26 {
                vfunction78(g, this, 0x4e26);
                vfunction78(g, this, 6);
                return;
            }
            vfunction78(g, this, id);
            return;
        }
        if id == 0x2716 {
            vfunction78(g, this, 0x2716);
            vfunction78(g, this, 6);
            let url = crate::sexy::app_host::update_check_new_version_url(g, this);
            vfunction38__005516c0(g, this, &url, true);
            return;
        }
        match id {
            0 => {
                vfunction78(g, this, 0);
                FUN_00552100(g, this);
            }
            1 => {
                FUN_0054ba40(g, this);
            }
            2 => {
                let d = crate::sexy::sexy_app_base::vfunction74(g, this, 2);
                if d == NULL {
                    return;
                }
                let name = crate::game::register_dialog::FUN_00531620(g, d);
                let code = crate::game::register_dialog::FUN_00531630(g, d);
                if code.is_empty() {
                    crate::sexy::app_host::open_registration_page(g, this);
                } else if !crate::sexy::app_host::validate_registration(g, this, &name, &code) {
                    crate::game::register_dialog::FUN_0054edc0(g, this);
                } else {
                    vfunction78(g, this, 2);
                    vfunction78(g, this, 3);
                    g.app_obj(this).sa.field_0x90 = name;
                    g.app_obj(this).sa.field_0xc8 = code;
                    let wm = g.sab(this).offset_0x318;
                    g.app_obj(this).sa.offset_0xe4 = true;
                    vcall!(g, wm, w.vfunction11);
                    let title = g.wfa(this).offset_0x8;
                    if title != NULL {
                        crate::game::title_screen::FUN_0051a750(g, title);
                    } else {
                        vfunction73(g, this, 0x17, true, b"Thanks!", b"Thank you for registering Insaniquarium!", &DAT_005bf7b0(), 3);
                    }
                }
            }
            3 => crate::game::register_dialog::FUN_0054e8a0(g, this),
            4 => {
                vfunction78(g, this, 4);
                FUN_0054cbf0(g, this);
            }
            5 => {
                vfunction78(g, this, 5);
                FUN_00551a70(g, this);
            }
            0xb => {
                vfunction78(g, this, 0xb);
                vfunction43(g, this);
            }
            0xd => {
                vfunction78(g, this, 0xd);
                crate::game::register_dialog::FUN_0054e8a0(g, this);
            }
            0xe => {
                vfunction78(g, this, 0xe);
            }
            0xf => crate::game::store::FUN_0054eec0(g, this),
            0x10 => {
                vfunction78(g, this, 0x10);
                let board = g.wfa(this).offset_0x4;
                crate::game::board_level::FUN_00546d70(g, board);
            }
            0x11 => {
                vfunction78(g, this, 0x11);
                FUN_0054bc30(g, this);
                FUN_00552100(g, this);
            }
            0x12 => {
                vfunction78(g, this, 0x12);
            }
            0x13 => {
                vfunction78(g, this, 0x13);
                let board = g.wfa(this).offset_0x4;
                crate::game::board::FUN_0053c1e0(g, board, 100);
            }
            0x14 => {
                vfunction78(g, this, 0x14);
                crate::game::pets_screen::FUN_0054af90(g, this);
                FUN_0054cbf0(g, this);
            }
            0x15 => {
                vfunction78(g, this, 0x15);
                let board = g.wfa(this).offset_0x4;
                if board != NULL {
                    crate::game::board_level::FUN_005460f0(g, board);
                }
            }
            0x16 => {
                vfunction78(g, this, 0x16);
                crate::game::win_fish_app::FUN_00552230(g, this);
            }
            0x18 => crate::game::user_dialog::FUN_0054ce30(g, this, true),
            0x19 => FUN_0054f4a0(g, this, true),
            0x1a => crate::game::user_dialog::FUN_0054f8b0(g, this, true),
            0x1b => crate::game::user_dialog::FUN_0054fad0(g, this, true),
            0x1c | 0x1d => FUN_0054b850(g, this, id),
            0x22 => crate::game::continue_dialog::FUN_00552480(g, this),
            0x23 => crate::game::store::FUN_0054b900(g, this, true),
            0x24 => crate::game::sim_setup::FUN_0054b8a0(g, this, true),
            0x25 => crate::game::sim_setup::FUN_0054ff90(g, this),
            0x26 => FUN_0054c6d0(g, this),
            0x28 => crate::game::user_dialog::FUN_0054f280(g, this, true),
            _ => {
                vfunction78(g, this, id);
            }
        }
        return;
    }
    if 4000 <= r {
        return;
    }
    let id = param_1 - 3000;
    if 0x2716 < id {
        vfunction78(g, this, id);
        return;
    }
    if id == 0x2716 {
        vfunction78(g, this, 0x2716);
        vfunction78(g, this, 6);
        return;
    }
    match id {
        3 => {
            vfunction78(g, this, 3);
            vfunction43(g, this);
        }
        4 => {
            vfunction78(g, this, 4);
        }
        0x18 => crate::game::user_dialog::FUN_0054ce30(g, this, false),
        0x19 => FUN_0054f4a0(g, this, false),
        0x1a => crate::game::user_dialog::FUN_0054f8b0(g, this, false),
        0x1b => crate::game::user_dialog::FUN_0054fad0(g, this, false),
        0x1f => FUN_0054b450(g, this, -1),
        0x23 => crate::game::store::FUN_0054b900(g, this, false),
        0x24 => crate::game::sim_setup::FUN_0054b8a0(g, this, false),
        0x26 => FUN_0054c6d0(g, this),
        0x28 => crate::game::user_dialog::FUN_0054f280(g, this, false),
        _ => {
            vfunction78(g, this, id);
        }
    }
}

/// `DAT_005bf7b0`: the footer of the "Thanks!" dialog ("Ok").
#[allow(non_snake_case)]
fn DAT_005bf7b0() -> Vec<u8> {
    b"Ok".to_vec()
}

/// port: 0054f4a0 FUN_0054f4a0
/// The New User dialog's result: OK with a name adds the profile (or reports a name
/// conflict), saves `users.dat`, makes it current and closes dialogs 0x18/0x19; Cancel
/// closes the dialog only when a profile exists; without a name the player is asked again.
pub fn FUN_0054f4a0(g: &mut G, this: Ptr, param_1: bool) {
    let d = crate::sexy::sexy_app_base::vfunction74(g, this, 0x19);
    if d == NULL {
        return;
    }
    let name = crate::game::money_dialog::FUN_005333c0(g, d);
    let ask = |g: &mut G| {
        vfunction73(
            g,
            this,
            0x1c,
            true,
            b"Enter Your Name",
            b"Please enter your name to create a new user profile for storing high score data and game progress.",
            b"OK",
            3,
        );
    };
    if !param_1 || !name.is_empty() {
        if g.wfa(this).offset_0x18c != NULL || (param_1 && !name.is_empty()) {
            if !param_1 {
                vfunction78(g, this, 0x19);
            } else {
                let mgr = g.wfa(this).offset_0x184;
                let p = crate::game::profile_mgr::FUN_005127e0(g, mgr, &name);
                if p == NULL {
                    vfunction73(
                        g,
                        this,
                        0x1c,
                        true,
                        b"Name Conflict",
                        b"The name you entered is already being used.  Please enter a unique player name.",
                        b"OK",
                        3,
                    );
                } else {
                    crate::game::profile_mgr::FUN_00514d50(g, mgr);
                    g.wfa(this).offset_0x18c = p;
                    vfunction78(g, this, 0x18);
                    vfunction78(g, this, 0x19);
                    let wm = g.sab(this).offset_0x318;
                    vcall!(g, wm, w.vfunction11);
                    let sel = g.wfa(this).offset_0xc;
                    if sel != NULL {
                        // `Sexy::GameObject::vfunction75` on the selector: the empty function at
                        // 004e9e00 (identical code folded with an empty GameSelector method).
                        crate::game::game_object::vfunction75(g, sel);
                    }
                }
            }
            return;
        }
        ask(g);
        return;
    }
    ask(g);
}

/// port: 0054b360 FUN_0054b360
/// `RemoveGameSelector()`.
pub fn FUN_0054b360(g: &mut G, this: Ptr) {
    let sel = g.wfa(this).offset_0xc;
    if sel != NULL {
        let wm = g.sab(this).offset_0x318;
        vcall!(g, wm, w.vfunction5, sel);
        crate::sexy::sexy_app_base::vfunction35(g, this, sel);
        g.wfa(this).offset_0xc = NULL;
    }
}

/// port: 00552380 FUN_00552380
/// `StartGame(bool tryResume, bool allowHelp)`: back to the selector when the trial is
/// over; in the screensaver's first virtual-tank start, queues a background job; resumes a
/// saved game when there is one; clears the profile's +0x5a flags; shows the help screen
/// before the first adventure tanks; else the tank screen (`FUN_0054c2d0`) or a new board
/// (`FUN_0054cbf0`).
pub fn FUN_00552380(g: &mut G, this: Ptr, param_1: bool, param_2: bool) {
    if FUN_0054bbb0(g, this) {
        FUN_00552100(g, this);
        return;
    }
    FUN_0054bc30(g, this);
    if g.wfa(this).offset_0x150 == 5 && !g.globals.DAT_005e963c {
        g.globals.DAT_005e8f16 = true;
        g.globals.DAT_005e963c = true;
        FUN_005025f0(g, crate::game::fish_songs::load_songs_job, 0);
    }
    if param_1 && FUN_0054cb20(g, this) {
        return;
    }
    let profile = g.wfa(this).offset_0x18c;
    g.profile(profile).field_0x5a = [false; 24];
    let p = g.profile(profile).clone();
    if param_2 && g.wfa(this).offset_0x150 == 0 && !p.field_0x59 && p.field_0x1c == 1 && p.field_0x20 < 4 {
        FUN_0054c220(g, this, true);
        return;
    }
    let resume_level = FUN_0054b2a0(g, this);
    let mode = g.wfa(this).offset_0x150;
    // (3 is the level bound above, still in EDX: more than three levels finished.)
    if !resume_level && mode != 5 && 3 < p.field_0x18 && (mode != 0 || p.field_0x1c != 5) {
        crate::game::pets_screen::FUN_0054c2d0(g, this);
        return;
    }
    FUN_0054cbf0(g, this);
}

/// port: 0054b2a0 FUN_0054b2a0
/// Whether this start goes straight into a level: an adventure bonus level, or the board's
/// +0x2a5.
pub fn FUN_0054b2a0(g: &mut G, this: Ptr) -> bool {
    let profile = g.wfa(this).offset_0x18c;
    if g.wfa(this).offset_0x150 == 0 && g.profile(profile).field_0x20 == 6 {
        return true;
    }
    let board = g.wfa(this).offset_0x4;
    if board != NULL {
        return g.board(board).field_0x219;
    }
    false
}

/// port: 0054c220 FUN_0054c220
/// `ShowHelpScreen(bool firstTime)`: replaces the selector and the previous help screen.
pub fn FUN_0054c220(g: &mut G, this: Ptr, param_1: bool) {
    FUN_0054b360(g, this);
    FUN_0054af10(g, this);
    let hs = crate::game::help_screen::HelpScreen(g, this, param_1);
    g.wfa(this).offset_0x108 = hs;
    let (w, h) = (g.sab(this).field_0xb8, g.sab(this).field_0xbc);
    vcall!(g, hs, w.vfunction41, 0, 0, w, h);
    let wm = g.sab(this).offset_0x318;
    vcall!(g, wm, w.vfunction4, hs);
}

/// port: 0054af10 FUN_0054af10
/// `RemoveHelpScreen()`.
pub fn FUN_0054af10(g: &mut G, this: Ptr) {
    let hs = g.wfa(this).offset_0x108;
    if hs != NULL {
        let wm = g.sab(this).offset_0x318;
        vcall!(g, wm, w.vfunction5, hs);
        crate::sexy::sexy_app_base::vfunction35(g, this, hs);
        g.wfa(this).offset_0x108 = NULL;
    }
}

/// port: 0054cb20 FUN_0054cb20
/// `TryResumeGame()`: makes a board and loads the saved game for this mode and profile;
/// without one the board is dropped again (false).
pub fn FUN_0054cb20(g: &mut G, this: Ptr) -> bool {
    FUN_0054c540(g, this);
    let mode = g.wfa(this).offset_0x150;
    let profile = g.wfa(this).offset_0x18c;
    let id = g.profile(profile).field_0x44;
    let path = FUN_00505880(mode, id);
    let board = g.wfa(this).offset_0x4;
    if !crate::game::board_save::FUN_00546aa0(g, board, &path) {
        FUN_0054bc30(g, this);
        return false;
    }
    g.wfa(this).offset_0x157 = false;
    if g.wfa(this).offset_0x150 != 5 {
        crate::game::continue_dialog::FUN_0054b3a0(g, this);
    }
    true
}

/// port: 0054c540 FUN_0054c540
/// `NewBoard()`: ends any game, makes the board full screen, adds it on top with focus and
/// counts a game played.
pub fn FUN_0054c540(g: &mut G, this: Ptr) {
    FUN_0054bc30(g, this);
    g.wfa(this).offset_0x158 = false;
    let board = crate::game::board::Board(g, this);
    g.wfa(this).offset_0x4 = board;
    let (w, h) = (g.sab(this).field_0xb8, g.sab(this).field_0xbc);
    vcall!(g, board, w.vfunction41, 0, 0, w, h);
    let wm = g.sab(this).offset_0x318;
    vcall!(g, wm, w.vfunction4, board);
    vcall!(g, wm, w.vfunction13, board);
    vcall!(g, wm, w.vfunction9, board);
    let profile = g.wfa(this).offset_0x18c;
    g.profile(profile).field_0x4c += 1;
}

/// port: 00505880 FUN_00505880
/// The saved-game path for a mode and profile id: `userdata\<sim|tim|sur|snd|adv><id>.dat`.
pub fn FUN_00505880(param_2: i32, param_3: i32) -> String {
    let prefix = match param_2 {
        5 => "sim",
        1 => "tim",
        4 => "sur",
        3 => "snd",
        _ => "adv",
    };
    format!("userdata\\{prefix}{param_3}.dat")
}

/// port: 0054cbf0 FUN_0054cbf0
/// `NewGame()`: resumes a saved virtual tank, else makes the board, sets the level up and
/// starts it (bonus round, virtual tank or a normal level), then the music.
pub fn FUN_0054cbf0(g: &mut G, this: Ptr) {
    g.wfa(this).offset_0x157 = false;
    if g.wfa(this).offset_0x150 == 5 && FUN_0054cb20(g, this) {
        return;
    }
    FUN_0054c540(g, this);
    let board = g.wfa(this).offset_0x4;
    crate::game::board_level::FUN_00541a20(g, board);
    let bonus = FUN_0054b2a0(g, this);
    let board = g.wfa(this).offset_0x4;
    if bonus {
        crate::game::board_level::FUN_00537e20(g, board);
        FUN_0054b2d0(g, this);
        return;
    }
    if g.wfa(this).offset_0x150 == 5 {
        crate::game::virtual_tank::FUN_0053c3f0(g, board);
        FUN_0054b2d0(g, this);
        return;
    }
    crate::game::board_level::FUN_005498b0(g, board);
    FUN_0054b2d0(g, this);
}

/// port: 0054b2d0 FUN_0054b2d0
/// `StartLevelMusic()`: after the level-start bookkeeping, the bonus song or the tank's
/// song (none in the virtual tank).
pub fn FUN_0054b2d0(g: &mut G, this: Ptr) {
    FUN_0054b020(g, this);
    if g.wfa(this).offset_0x150 != 5 {
        if FUN_0054b2a0(g, this) {
            FUN_0054b1a0(g, this, 0, 0xd, false);
            return;
        }
        if FUN_0054acf0(g, this) == 1 {
            FUN_0054b1a0(g, this, 0, 0x16, false);
            return;
        }
        if FUN_0054acf0(g, this) == 2 {
            FUN_0054b1a0(g, this, 0, 0, false);
            return;
        }
        if FUN_0054acf0(g, this) == 3 {
            FUN_0054b1a0(g, this, 2, 0x20, false);
            return;
        }
        if FUN_0054acf0(g, this) == 4 {
            FUN_0054b1a0(g, this, 2, 0x2d, false);
        }
    }
}

/// port: 0054acf0 FUN_0054acf0
/// The current tank: the board's, else the profile's.
pub fn FUN_0054acf0(g: &mut G, this: Ptr) -> i32 {
    let board = g.wfa(this).offset_0x4;
    if board != NULL {
        return g.board(board).field_0x33c;
    }
    let profile = g.wfa(this).offset_0x18c;
    g.profile(profile).field_0x1c
}

/// port: 0054be80 FUN_0054be80
/// `ShowHatchScreen(int pet)`: closes the dialogs, marks a play in progress (the board's
/// game is no longer in progress), opens the hatch screen full-size, saves the profile and
/// ends the game.
pub fn FUN_0054be80(g: &mut G, this: Ptr, param_1: i32) {
    FUN_0054b4c0(g, this);
    g.wfa(this).offset_0x158 = true;
    let board = g.wfa(this).offset_0x4;
    if board != NULL {
        g.board(board).ext_0x4ee = false;
    }
    FUN_0054adf0(g, this);
    let s = crate::game::hatch_screen::HatchScreenOverlay(g, this, param_1);
    g.wfa(this).offset_0xe8 = s;
    let (w, h) = (g.sab(this).field_0xb8, g.sab(this).field_0xbc);
    vcall!(g, s, w.vfunction41, 0, 0, w, h);
    let wm = g.sab(this).offset_0x318;
    vcall!(g, wm, w.vfunction4, s);
    FUN_0054afd0(g, this);
    FUN_0054bc30(g, this);
}

/// port: 0054adf0 FUN_0054adf0
/// Closes the hatch screen (+0x814), if it is up.
pub fn FUN_0054adf0(g: &mut G, this: Ptr) {
    let s = g.wfa(this).offset_0xe8;
    if s != NULL {
        let wm = g.sab(this).offset_0x318;
        vcall!(g, wm, w.vfunction5, s);
        crate::sexy::sexy_app_base::vfunction35(g, this, s);
        g.wfa(this).offset_0xe8 = NULL;
    }
}

/// port: 0054afd0 FUN_0054afd0
/// Saves the current profile's details, if there is one.
pub fn FUN_0054afd0(g: &mut G, this: Ptr) -> bool {
    let profile = g.wfa(this).offset_0x18c;
    if profile != NULL {
        crate::game::profile::FUN_00514730(g, profile);
    }
    true
}

/// port: 0054c390 FUN_0054c390
/// `PlayTankMusic(bool fade)`: the virtual tank just stops the music; otherwise the
/// current music fades out (speed 0.008) and the tank's tune fades in at 0.002, or (no
/// fade) starts at once: tank 1 song 0x16 and tank 2 song 0 of music 0, tanks 3/4 songs
/// 0x20/0x2d of music 2.
pub fn FUN_0054c390(g: &mut G, this: Ptr, param_1: bool) {
    if g.wfa(this).offset_0x150 == 5 {
        FUN_0054b020(g, this);
        return;
    }
    let speed = if param_1 {
        FUN_0054b230(g, this, 1, 1, 0.008);
        0.002
    } else {
        FUN_0054b020(g, this);
        1.0
    };
    let (music, song) = match FUN_0054acf0(g, this) {
        1 => (0, 0x16),
        2 => (0, 0),
        3 => (2, 0x20),
        4 => (2, 0x2d),
        _ => return,
    };
    FUN_0054b1e0(g, this, music, song, speed, false);
}

/// port: 0054b5b0 FUN_0054b5b0
/// `DoGameDialog(...)`: `DoDialog` (vtable +0x120), then the dialog's slot 77 with 0x1e.
pub fn FUN_0054b5b0(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: &[u8], param_4: &[u8], param_5: &[u8], param_6: i32) -> Ptr {
    // (app vtable +0x120: WinFishApp::DoDialog)
    let d = vfunction73(g, this, param_1, param_2 != 0, param_3, param_4, param_5, param_6);
    vcall!(g, d, money.vfunction77, 0x1e);
    d
}

/// port: 0054c480 FUN_0054c480
/// `PlayAlienMusic(bool fade)`: (not in the virtual tank) the tank tune fades out (speed
/// 0.008) or all music stops, then the alien theme (music 1) starts, fading in at 0.002.
pub fn FUN_0054c480(g: &mut G, this: Ptr, param_1: bool) {
    let mut fade = param_1;
    if g.wfa(this).offset_0x150 == 5 {
        fade = false;
        FUN_0054b020(g, this);
    } else if fade {
        match FUN_0054acf0(g, this) {
            1 | 2 => FUN_0054b230(g, this, 0, 0, 0.008),
            3 | 4 => FUN_0054b230(g, this, 2, 0, 0.008),
            _ => {}
        }
    } else {
        FUN_0054b020(g, this);
    }
    FUN_0054b1a0(g, this, 1, 0, false);
    if fade {
        // mMusicInterface->vfunction (+0xc)(1): stop music 1 before fading it in. Without a
        // music interface (see `offset_0x36c`) there is nothing to stop.
        FUN_0054b1e0(g, this, 1, -1, 0.002, false);
    }
}

/// port: 0054b940 Sexy::WinFishApp::vfunction26
/// `NewDialog(int theId, bool isModal, header, lines, footer, int theButtonMode)`: a
/// `MoneyDialog` with the game's box and button images, as wide as the header plus 150
/// (at least 348), as tall as it wants (at least two thirds of the box image plus 10), at
/// (143, 142).
pub fn vfunction26(g: &mut G, this: Ptr, param_1: i32, param_2: bool, param_3: &[u8], param_4: &[u8], param_5: &[u8], param_6: i32) -> Ptr {
    use crate::game::money_dialog::{MoneyDialog, MoneyDialog_data};
    use crate::sexy::dialog::MoneySub;
    let d = crate::sexy::dialog::alloc_dialog(
        g,
        &crate::sexy::vtables_gen::Sexy__MoneyDialog_vftable,
        crate::sexy::dialog::DlgSub::Money(MoneyDialog_data::default(), MoneySub::None),
    );
    let (comp, btn) = (g.res.DAT_005e8cc0, g.res.DAT_005e8c54);
    MoneyDialog(g, d, this, comp, btn, param_1, param_2, param_3, param_4, param_5, param_6);
    let hf = g.dialog(d).offset_0x6c;
    let w = (crate::sexy::image_font::string_width(g, hf, param_3) + 0x96).max(0x15c);
    let mut h = vcall!(g, d, dlg.vfunction74, w);
    let img = g.dialog(d).field_0x4;
    let min = (g.image(img).offset_0x24 * 2) / 3 + 10;
    if h < min {
        h = min;
    }
    vcall!(g, d, w.vfunction41, 0x8f, 0x8e, w, h);
    d
}

/// port: 0054cf10 FUN_0054cf10
/// `mDialogList.back()`: the topmost dialog.
pub fn FUN_0054cf10(g: &mut G, this: Ptr) -> Ptr {
    *g.sab(this).offset_0x32c.last().expect("FUN_0054cf10 on an empty dialog list")
}

/// port: 0054baf0 Sexy::WinFishApp::vfunction61
/// `SetMusicVolume(double)`: SexyAppBase's.
pub fn vfunction61(g: &mut G, this: Ptr, param_1: f64) {
    crate::sexy::sexy_app_base::vfunction61(g, this, param_1);
}

/// port: 0054c620 FUN_0054c620
/// `ShowOptionsDialog(bool fromMainMenu)`: closes other dialogs' focus, then dialog 1 at
/// (143, 26), 348 wide, as tall as it wants.
pub fn FUN_0054c620(g: &mut G, this: Ptr, param_1: bool) {
    FUN_0054b4c0(g, this);
    let d = crate::game::options_dialog::FUN_005349d0(g, this, param_1);
    let h = vcall!(g, d, dlg.vfunction74, 0x15c);
    vcall!(g, d, w.vfunction41, 0x8f, 0x1a, 0x15c, h);
    crate::sexy::sexy_app_base::vfunction76(g, this, 1, d);
}

/// port: 0054ba40 FUN_0054ba40
/// The Options dialog's OK: applies the screen mode (fullscreen unchecked = windowed) and
/// 3D, the custom cursors, closes the dialog, forgets the time spent, and refreshes the
/// board's shop buttons. False when the dialog is gone.
pub fn FUN_0054ba40(g: &mut G, this: Ptr) -> bool {
    let d = crate::sexy::sexy_app_base::vfunction74(g, this, 1);
    if d == NULL {
        return false;
    }
    let o = g.options_dialog(d).clone();
    let full = vcall!(g, o.offset_0xc, checkbox.vfunction72);
    let is3d = vcall!(g, o.offset_0x14, checkbox.vfunction72);
    crate::sexy::sexy_app_base::vfunction69(g, this, !full, is3d, false);
    let cursors = vcall!(g, o.offset_0x10, checkbox.vfunction72);
    crate::sexy::sexy_app_base::FUN_004891d0(g, this, cursors);
    vfunction78(g, this, 1);
    crate::sexy::sexy_app_base::FUN_00479f90(g, this, true);
    let board = g.wfa(this).offset_0x4;
    if board != NULL {
        crate::game::board_level::FUN_00539fe0(g, board);
    }
    true
}

/// port: 005025f0 FUN_005025f0
/// `WorkerThread::DoTask(func, arg)`: waits for the previous task, hands this one to the
/// worker thread and wakes it. The port has no worker thread: in-memory file reads do not
/// block, so the task runs at once (its effects are those the main thread saw once the
/// worker finished).
pub fn FUN_005025f0(g: &mut G, param_1: fn(&mut G, i32), param_2: i32) {
    FUN_005025c0(g);
    param_1(g, param_2);
}

/// port: 00504280 Sexy::WorkerThread::WorkerThread
/// Creates the two events and starts the thread (`FUN_00503ca0`). The port has no worker
/// thread (see `FUN_005025f0`), so there is nothing to create.
#[allow(non_snake_case)]
pub fn WorkerThread(_g: &mut G) {}

/// port: 00503c50 Sexy::WorkerThread::~WorkerThread
/// Waits for the task in hand, tells the thread to stop and waits for it (5 s), closes the
/// events. Without a thread only the wait remains (and it returns at once).
pub fn dtor_WorkerThread(g: &mut G) {
    FUN_005025c0(g);
}

/// port: 005042d0 Sexy::WorkerThread::deleting_destructor
pub fn deleting_destructor__005042d0(g: &mut G, _param_1: u8) {
    dtor_WorkerThread(g);
}

/// port: 00503ca0 FUN_00503ca0
/// The worker thread's entry: lowest priority, then the task loop (`FUN_00502560`). Not
/// started in the port (tasks run inline in `FUN_005025f0`).
pub fn FUN_00503ca0(g: &mut G) {
    FUN_00502560(g);
}

/// port: 00502560 FUN_00502560
/// `WorkerThread::ProcLoop()`: until told to stop, runs each task handed over and signals
/// its end. With tasks run inline there is never one waiting, so it has nothing to do.
pub fn FUN_00502560(_g: &mut G) {}

/// port: 005025c0 FUN_005025c0
/// `WorkerThread::WaitForTask()`: nothing to wait for without a worker thread.
pub fn FUN_005025c0(_g: &mut G) {}

/// port: 0054b4e0 FUN_0054b4e0
/// `ShowVirtualTankWelcome()`: dialog 0x27, centered, 70 from the top.
pub fn FUN_0054b4e0(g: &mut G, this: Ptr) {
    vfunction78(g, this, 0x27);
    let d = crate::game::virtual_tank::FUN_00536c80(g, this);
    let (w, h) = (g.wc(d).offset_0x34, g.wc(d).offset_0x38);
    vcall!(g, d, w.vfunction41, 0x140 - w / 2, 0x46, w, h);
    crate::sexy::sexy_app_base::vfunction76(g, this, 0x27, d);
}

/// port: 00552230 FUN_00552230
/// `BackToMainMenu()`: ends the virtual-tank session (`FUN_0054afd0`), closes the dialogs
/// and screens (`FUN_0054ba40`, `FUN_0054bc30`), and shows the game selector.
pub fn FUN_00552230(g: &mut G, this: Ptr) {
    FUN_0054afd0(g, this);
    FUN_0054ba40(g, this);
    FUN_0054bc30(g, this);
    FUN_00552100(g, this);
}

/// port: 0054c9c0 Sexy::WinFishApp::vfunction43
/// `Shutdown()`: once: ends a game in progress, folds the trial counters, adds this run's
/// play time to the day's (restarting it on a new day or past an hour), then the base's.
pub fn vfunction43(g: &mut G, this: Ptr) {
    if g.sab(this).field_0x339 {
        return;
    }
    if g.wfa(this).offset_0x4 != NULL {
        FUN_0054afd0(g, this);
        FUN_0054bc30(g, this);
    }
    FUN_0054ac80(g, this);
    let day = FUN_005005b0(g);
    let add = g.sab(this).field_0x47c / 0x24;
    g.wfa(this).offset_0x24 += add;
    if 0xe0f < g.wfa(this).offset_0x24 || g.wfa(this).offset_0x14 <= day {
        g.wfa(this).offset_0x14 = day;
        g.wfa(this).offset_0x24 = 0;
    }
    crate::sexy::sexy_app_base::vfunction43(g, this);
}

/// `DAT_005e9635`: debug slow motion (one update in four). 0 in the data section, never
/// written.
pub const DAT_005e9635: bool = false;
/// `DAT_005e9636`: debug fast forward (100 updates per frame). 0, never written.
pub const DAT_005e9636: bool = false;

/// port: 0054bb30 Sexy::WinFishApp::vfunction9
/// `UpdateFrames()`: the framework's, every fourth time in slow motion, 100 times in fast
/// forward (both debug switches off in this build).
pub fn vfunction9(g: &mut G, _this: Ptr) {
    if DAT_005e9635 {
        g.globals.DAT_005e9638 += 1;
        if g.globals.DAT_005e9638 < 4 {
            return;
        }
        g.globals.DAT_005e9638 = 0;
    } else if DAT_005e9636 {
        for _ in 0..100 {
            crate::game::boot::update_app_step(g);
        }
        return;
    }
    crate::game::boot::update_app_step(g);
}

/// port: 0054dbf0 Sexy::WinFishApp::~WinFishApp
/// Ends a game in progress (saved), stops the music, closes dialogs 0..0x28, removes and
/// deletes the board and every screen, then the profile and high-score managers and the
/// random generator. (The framework's destructor that follows is replaced.)
pub fn dtor_WinFishApp(g: &mut G, this: Ptr) {
    if g.wfa(this).offset_0x4 != NULL {
        FUN_0054afd0(g, this);
    }
    FUN_0054b020(g, this);
    for i in 0..0x29 {
        vfunction78(g, this, i);
    }
    let wm = g.sab(this).offset_0x318;
    let board = g.wfa(this).offset_0x4;
    if board != NULL {
        crate::game::board::FUN_005497a0(g, board);
        vcall!(g, wm, w.vfunction5, board);
        vcall!(g, board, w.vfunction1, 1);
        g.wfa(this).offset_0x4 = NULL;
    }
    let w = g.wfa(this);
    let screens = [
        w.offset_0x8,
        w.offset_0xe4,
        w.offset_0xe8,
        w.offset_0xec,
        w.offset_0x104,
        w.offset_0x108,
        w.offset_0xf0,
        w.offset_0xf4,
        w.offset_0xfc,
        w.offset_0x100,
        w.offset_0xe0,
        w.offset_0xf8,
        w.offset_0xc,
    ];
    for s in screens {
        if s != NULL {
            vcall!(g, wm, w.vfunction5, s);
            vcall!(g, s, w.vfunction1, 1);
        }
    }
    let mgr = g.wfa(this).offset_0x184;
    if mgr != NULL {
        crate::game::profile_mgr::deleting_destructor(g, mgr, 1);
    }
    let hsm = g.wfa(this).offset_0x180;
    if hsm != NULL {
        g.free(hsm);
    }
    let r = g.wfa(this).offset_0x84;
    if r != NULL {
        g.free(r);
    }
}

/// port: 00551c60 Sexy::WinFishApp::deleting_destructor
pub fn deleting_destructor__00551c60(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_WinFishApp(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 0054abf0 Sexy::WinFishApp::vfunction19
/// `CreateMusicInterface()`: a silent one for the screensaver, else the framework's (the
/// BASS one; its state is `G::music`, played by the host). The returned object only marks
/// that an interface exists.
pub fn vfunction19(g: &mut G, this: Ptr) -> Ptr {
    let silent = thunk_FUN_00479fc0(g, this);
    g.music = crate::sexy::music::MusicInterface { silent, ..Default::default() };
    g.alloc(Obj { vt: None, node: Node::MusicInterface })
}

/// port: 0054caa0 Sexy::WinFishApp::vfunction34
/// The crash-log hook (`SexyAppBase::vfunction34`): our screensaver is switched off first,
/// unless already done.
pub fn vfunction34(g: &mut G, this: Ptr, param_1: i32) {
    if !g.globals.DAT_005e9634 {
        crate::game::sim_setup::FUN_0054ca60(g, this);
    }
    crate::sexy::app_host::write_error_log(g, this, param_1);
}

/// port: 00551570 Sexy::WinFishApp::vfunction36
/// `URLOpenFailed(const string& url)`: closes "Opening Browser", copies the URL to the
/// clipboard and shows it in "Open Browser" (dialog 9).
pub fn vfunction36(g: &mut G, this: Ptr, param_1: &[u8]) {
    g.sab(this).field_0x3e4 = false;
    vfunction78(g, this, 9);
    g.clipboard = param_1.to_vec();
    let mut lines = b"Please open the following URL in your browser\n\n".to_vec();
    lines.extend_from_slice(param_1);
    lines.extend_from_slice(b"\n\nFor your convenience, this URL has already been copied to your clipboard.");
    vfunction73(g, this, 9, true, b"Open Browser", &lines, b"OK", 3);
}

/// port: 0054bb80 Sexy::WinFishApp::vfunction37
/// `URLOpenSucceeded(const string&)`: the framework's (shutting down when asked to, and
/// noting it), then "Opening Browser" closes.
pub fn vfunction37(g: &mut G, this: Ptr) {
    g.sab(this).field_0x3e4 = false;
    if g.sab(this).field_0x3e5 {
        crate::sexy::sexy_app_base::vfunction43(g, this);
        g.app_obj(this).sa.offset_0x6c = true;
    }
    vfunction78(g, this, 9);
}

/// port: 00550fb0 Sexy::WinFishApp::vfunction53
/// From the screensaver: the working folder becomes the game's ("<key>Directory" in the
/// registry). (Host boundary: `set_current_dir`.)
pub fn vfunction53(g: &mut G, this: Ptr) {
    if !thunk_FUN_00479fc0(g, this) {
        return;
    }
    let key = format!("{}Directory", String::from_utf8_lossy(&g.wfa(this).field_0x40));
    if let Some(dir) = crate::sexy::app_host::registry_read_string(g, &key) {
        crate::sexy::app_host::set_current_dir(g, dir.as_bytes());
    }
}

/// port: 0054bb10 Sexy::WinFishApp::vfunction86
/// `DebugKeyDown(int key)`: the framework's.
pub fn vfunction86(g: &mut G, this: Ptr, param_1: i32) -> bool {
    crate::sexy::app_host::debug_key_down(g, this, param_1)
}

/// port: 00552250 Sexy::WinFishApp::vfunction17
/// `PreDisplayHook()`: outside the screensaver, asks about checking for updates when it is
/// time to.
pub fn vfunction17(g: &mut G, this: Ptr) {
    if !thunk_FUN_00479fc0(g, this) && crate::sexy::sexy_app_base::vfunction99_for_ButtonListener(g, this) {
        FUN_00551920(g, this);
    }
}

/// port: 00551920 FUN_00551920
/// `AskCheckForUpdates()`: notes the time, then "Updates" (dialog 5, yes/no), 348 wide at
/// (146, 50).
pub fn FUN_00551920(g: &mut G, this: Ptr) {
    crate::sexy::sexy_app_base::vfunction100_for_ButtonListener(g, this);
    let d = vfunction73(
        g,
        this,
        5,
        true,
        b"Updates",
        b"Do you want to check for updates to Insaniquarium? New versions may offer new features and bug fixes.  This requires an active Internet connection.",
        b"",
        1,
    );
    let h = vcall!(g, d, dlg.vfunction74, 0x15c);
    vcall!(g, d, w.vfunction41, 0x92, 0x32, 0x15c, h);
}

/// port: 00551d70 Sexy::WinFishApp::vfunction25
/// `ReadFromRegistry()`: the framework's values; the screensaver's sound and Powersave
/// properties; the screensaver's registry prefix gains the user's name; its old path and
/// enabled flag; the screensaver settings (`FUN_0054e2d0`). From the screensaver the
/// display follows Powersave and the sound setting mutes. Otherwise "ldaccum" (the day's
/// play time) and "ldinfo" (the day it was for, low 32 bits): unless that day is unknown or
/// later than today, the accumulated time starts again at 0; else today becomes that day.
/// (`SexyApp::ReadFromRegistry` is replaced: its `SexyAppBase` part and
/// `LastVerCheckQueryTime` are read, its registration and trial values are not, the build
/// being registered through partner.xml; `FUN_0047a800`, a binary registry read into the prefix, has no host
/// counterpart and leaves it as is.)
pub fn vfunction25(g: &mut G, this: Ptr) {
    use crate::sexy::app_host::{registry_read_boolean, registry_read_integer, registry_read_string};
    crate::sexy::sexy_app_base::vfunction25(g, this);
    if let Some(v) = registry_read_integer(g, "LastVerCheckQueryTime") {
        g.app_obj(this).sa.offset_0x68 = v;
    }
    g.wfa(this).offset_0x10d = crate::sexy::app_host::FUN_00487170(g, "ScrSound", true);
    g.wfa(this).offset_0x112 = crate::sexy::app_host::FUN_00487170(g, "ScrPowersave", false);
    if let Some(name) = crate::sexy::app_host::user_name(g) {
        let p = &mut g.wfa(this).field_0x5c;
        p.extend_from_slice(&name);
        p.push(b'\\');
    }
    let base = String::from_utf8_lossy(&g.wfa(this).field_0x5c).into_owned();
    if let Some(v) = registry_read_string(g, &format!("{base}OldPath")) {
        g.wfa(this).field_0x114 = v.into_bytes();
    }
    if let Some(v) = registry_read_boolean(g, &format!("{base}Enabled")) {
        g.wfa(this).offset_0x10c = v;
    }
    crate::game::sim_setup::FUN_0054e2d0(g, this);
    if thunk_FUN_00479fc0(g, this) {
        if !g.wfa(this).offset_0x112 {
            g.sab(this).field_0x33b = false;
            g.sab(this).field_0x351 = false;
        } else {
            g.sab(this).field_0x33b = true;
            g.sab(this).field_0x33d = true;
        }
        if !g.wfa(this).offset_0x10d {
            g.sab(this).field_0xe8 = true;
        }
        return;
    }
    if let Some(v) = registry_read_integer(g, "ldaccum") {
        g.wfa(this).offset_0x24 = v;
    }
    let restart = match registry_read_integer(g, "ldinfo") {
        None => true,
        Some(v) => {
            let w = g.wfa(this);
            w.offset_0x14 = (w.offset_0x14 & !0xffff_ffff) | (v as u32 as i64);
            w.offset_0x1c < w.offset_0x14
        }
    };
    if !restart {
        g.wfa(this).offset_0x24 = 0;
    } else {
        let d = g.wfa(this).offset_0x14;
        g.wfa(this).offset_0x1c = d;
    }
}

/// port: 0054e180 Sexy::WinFishApp::vfunction24
/// `WriteToRegistry()`: the current player ("CurUser", not from the screensaver) and their
/// profile file; then (not from the screensaver) SexyApp's values, "ldinfo" (the play day,
/// low 32 bits) and "ldaccum" (the day's play time).
pub fn vfunction24(g: &mut G, this: Ptr) {
    use crate::sexy::app_host::registry_write;
    let profile = g.wfa(this).offset_0x18c;
    if profile != NULL {
        if !FUN_00479fc0(g, this) {
            let name = String::from_utf8_lossy(&g.profile(profile).field_0x24).into_owned();
            registry_write(g, "CurUser", name);
        }
        crate::game::profile::FUN_00514730(g, profile);
    }
    if !FUN_00479fc0(g, this) {
        crate::sexy::sexy_app_base::vfunction24_for_ButtonListener(g, this);
        let d = g.wfa(this).offset_0x14 as i32;
        registry_write(g, "ldinfo", d.to_string());
        let a = g.wfa(this).offset_0x24;
        registry_write(g, "ldaccum", a.to_string());
    }
}

/// port: 005517f0 FUN_005517f0
/// `ShowQuitDialog()`: dialog 0xb "Quit" — "Stop the insanity?" (yes/no, the yes button
/// labelled "Quit").
pub fn FUN_005517f0(g: &mut G, this: Ptr) {
    let d = vfunction73(g, this, 0xb, true, b"Quit", b"Stop the insanity?", b"", 2);
    let yes = g.dialog(d).offset_0x8;
    g.btn(yes).field_0x4 = b"Quit".to_vec();
}

/// port: 0054b850 FUN_0054b850
/// "Name Conflict" (0x1c over the new-user dialog, 0x1d over the rename dialog) closed:
/// the name field gets the focus back.
pub fn FUN_0054b850(g: &mut G, this: Ptr, param_1: i32) {
    vfunction78(g, this, param_1);
    let id = if param_1 != 0x1c { 0x1b } else { 0x19 };
    let d = crate::sexy::sexy_app_base::vfunction74(g, this, id);
    if d != NULL {
        let wm = g.sab(this).offset_0x318;
        let edit = g.new_user(d).offset_0x4;
        vcall!(g, wm, w.vfunction9, edit);
    }
}

/// port: 0054c6d0 FUN_0054c6d0
/// "TIME'S UP!" (0x26) closed: the results.
pub fn FUN_0054c6d0(g: &mut G, this: Ptr) {
    vfunction78(g, this, 0x26);
    crate::game::bonus_screen::FUN_0054bf60(g, this);
}

/// port: 0054b590 FUN_0054b590
/// Whether the loading thread has done more than 29 tasks (+0x50c).
pub fn FUN_0054b590(g: &mut G, this: Ptr) -> bool {
    0x1d < g.sab(this).field_0x504
}

/// port: 0054eb80 FUN_0054eb80
/// `TimeTrialOver()`: "TIME'S UP!" (dialog 0x26) with the minutes played.
pub fn FUN_0054eb80(g: &mut G, this: Ptr) {
    FUN_0054b4c0(g, this);
    let board = g.wfa(this).offset_0x4;
    let msg = format!("Your {} minutes are up!", g.board(board).field_0x330 / 0x3c).into_bytes();
    FUN_0054b5b0(g, this, 0x26, 1, b"TIME'S UP!", &msg, b"Click for Results", 3);
}

/// port: 0054e760 FUN_0054e760
/// "Leave Game?" (dialog 0x16, yes/no relabelled LEAVE / CANCEL): back to the menu, the
/// game saved.
pub fn FUN_0054e760(g: &mut G, this: Ptr) {
    let d = vfunction73(g, this, 0x16, true, b"Leave Game?", b"Do you want to return to the
main menu?

Your game will be saved.", b"", 1);
    let (yes, no) = (g.dialog(d).offset_0x8, g.dialog(d).offset_0xc);
    g.btn(yes).field_0x4 = b"LEAVE".to_vec();
    g.btn(no).field_0x4 = b"CANCEL".to_vec();
}

/// port: 00551a70 FUN_00551a70
/// `CheckForUpdates()`: closes the Options and its sub-dialog, opens the update-check
/// dialog (6, the game's look, 348 wide at (146, 50)) and starts the query for this product,
/// version and partner.
pub fn FUN_00551a70(g: &mut G, this: Ptr) {
    vfunction78(g, this, 6);
    vfunction78(g, this, 5);
    let (comp, btn, stripe) = (g.res.DAT_005e8cc0, g.res.DAT_005e8c54, g.res.DAT_005e8b58);
    let d = crate::sexy::update_check_dialog::FUN_0046eb60(g, comp, btn, stripe, 6);
    crate::game::money_dialog::FUN_00504060(g, d);
    let h = vcall!(g, d, dlg.vfunction74, 0x15c);
    vcall!(g, d, w.vfunction41, 0x92, 0x32, 0x15c, h);
    crate::sexy::sexy_app_base::vfunction76(g, this, 6, d);
    let mut url = b"http://www.popcap.com/win32updatecheck.php?prod=".to_vec();
    url.extend_from_slice(&g.sab(this).field_0x3c);
    url.extend_from_slice(b"&ver=");
    url.extend_from_slice(&g.sab(this).field_0x390);
    url.extend_from_slice(b"&referid=");
    url.extend_from_slice(&g.wfa(this).field_0x8c);
    crate::sexy::app_host::check_for_updates(g, &url);
}
