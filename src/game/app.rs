//! `Sexy::WinFishApp` (with its `SexyAppBase` and `SexyApp` parts): the application object.
//! Fields are added as ported code reads them; names follow tools/off.py.

use crate::sexy::prelude::*;

/// `SexyAppBase_data` (object offset 0x8).
#[derive(Debug, Default)]
pub struct SexyAppBase_data {
    /// +0x98: the folder the screensaver file is looked for in (std::string; nothing in the
    /// ported code sets it).
    pub field_0x90: Vec<u8>,
    /// +0xc0 `mWidth`.
    pub field_0xb8: i32,
    /// +0xc4 `mHeight`.
    pub field_0xbc: i32,
    /// +0xd0 `mMusicVolume`.
    pub field_0xc8: f64,
    /// +0xd8 `mSfxVolume`.
    pub field_0xd0: f64,
    /// +0x90 `mRegKey` (the registry key the settings live under).
    pub field_0x88: Vec<u8>,
    /// +0xb8 / +0xbc `mPreferredX` / `mPreferredY` (window position; -1 = centered).
    pub field_0xb0: i32,
    pub field_0xb4: i32,
    /// +0x378 `mReadFromRegistry`.
    pub field_0x370: bool,
    /// +0x5c8 `mWaitForVSync`.
    pub field_0x5c0: bool,
    /// +0x428 `mMuteCount` (>0: music and sounds silent).
    pub field_0x420: i32,
    /// +0x42c `mAutoMuteCount` (mutes from losing focus; muted counts only the rest).
    pub field_0x424: i32,
    /// `mSoundManager->SetVolume()` / `mMusicInterface->SetVolume()`: the volumes handed to
    /// the sound manager and the music interface (both replaced by the host, which reads
    /// these when it plays).
    pub sound_manager_volume: f64,
    pub music_interface_volume: f64,
    /// +0xf2: command line parsed.
    pub field_0xea: bool,
    /// +0xf5 `mbAllowExtendedChars`: edit widgets accept bytes 0x80..0xff.
    pub field_0xed: bool,
    /// +0x320 `mWidgetManager`.
    pub offset_0x318: Ptr,
    /// +0x324 `mDialogMap` (std::map<int, Dialog*>; its size at +0x32c is `len()`).
    pub offset_0x320: std::collections::BTreeMap<i32, Ptr>,
    /// +0x330 `mDialogList` (std::list<Dialog*>; its size at +0x338 is `len()`).
    pub offset_0x32c: Vec<Ptr>,
    /// +0x374 `mMusicInterface`. Music (MO3 modules through BASS) is not reproduced, so the
    /// port has no music interface object and this stays null.
    pub offset_0x36c: Ptr,
    /// +0x341 `mShutdown`.
    pub field_0x339: bool,
    /// +0x342 `mExitToTop`.
    pub field_0x33a: bool,
    /// +0xe0 / +0xe8: the music and sound volumes restored at shutdown when +0x511 is set.
    pub field_0xd8: f64,
    pub field_0xe0: f64,
    /// +0x36a: position the Windows system caret for edit widgets (accessibility).
    pub field_0x362: bool,
    /// +0x358 `mIsScreenSaver`.
    pub field_0x350: bool,
    /// +0x36c `mDDInterface`. The renderer replaces DDInterface; only its 3D flag is kept.
    pub offset_0x364: Ptr,
    /// +0x398 `mProductVersion` (from the exe's version resource: "1.1").
    pub field_0x390: Vec<u8>,
    /// +0x44 `mProdName` ("Insaniquarium").
    pub field_0x3c: Vec<u8>,
    /// +0x3b4 `mCursorImages[13]`.
    pub field_0x3ac: [Ptr; 13],
    /// +0xb4: update steps allowed to catch up (set to 1000 by `ClearUpdateBacklog(true)`).
    /// The host paces updates itself, so the port never reads it.
    pub field_0xac: i32,
    /// +0x468: accumulated update time (`ClearUpdateBacklog` zeroes it; host-paced, unread).
    pub field_0x460: i64,
    /// +0x470: the time of the last update check (`timeGetTime`; host-paced, unread).
    pub field_0x468: u32,
    /// +0x454: `mSleepDuration`?, 0x1c.
    pub field_0x44c: i32,
    /// +0x484 `mUpdateCount` (incremented by every `UpdateApp` step).
    pub field_0x47c: i32,
    /// +0x4f9 `mLoadingThreadStarted`.
    pub field_0x4f1: bool,
    /// +0x4fb `mLoadingFailed`.
    pub field_0x4f3: bool,
    /// +0x4fd: do not start (set when Init failed).
    pub field_0x4f5: bool,
    /// +0x500 `mCustomCursorsEnabled`.
    pub field_0x4f8: bool,
    /// +0x343 `mIsWindowed`.
    pub field_0x33b: bool,
    /// +0x346 `mForceFullscreen` (no windowed mode on this desktop).
    pub field_0x33e: bool,
    /// +0x3ec `mIsOpeningURL`.
    pub field_0x3e4: bool,
    /// +0x3ed `mShutdownOnURLOpen`.
    pub field_0x3e5: bool,
    /// +0x3f0 `mOpeningURL` (std::string).
    pub field_0x3e8: Vec<u8>,
    /// +0x40c `mOpeningURLTime` (`GetTickCount`).
    pub field_0x404: u32,
    /// +0x345 (set by the screensaver with Powersave on).
    pub field_0x33d: bool,
    /// +0x359 (cleared by the screensaver with Powersave off).
    pub field_0x351: bool,
    /// +0xf0 (set by the screensaver with its sound off).
    pub field_0xe8: bool,
    /// +0x5ca: the display changed (3D on/off).
    pub field_0x5c2: bool,
    /// +0x46c: when the screen mode last changed (`timeGetTime`).
    pub field_0x464: u32,
    /// `mDDInterface->mD3DTester` (+0x34) exists, and its failure (+0x8c) and warning
    /// (+0xa8) results. The port renders through the host, so it reports a tester that
    /// passed (set up in `sexy_app_init`).
    pub d3d_tester: bool,
    pub d3d_tester_fail: i32,
    pub d3d_tester_warn: i32,
    /// +0x502 `mLoadingThreadCompleted`.
    pub field_0x4fa: bool,
    /// +0x508 `mNumLoadingThreadTasks`.
    pub field_0x500: i32,
    /// +0x50c `mCompletedLoadingThreadTasks`.
    pub field_0x504: i32,
    /// +0x5ac: when set, the game selector ignores typed cheat codes.
    pub field_0x5a4: bool,
    /// +0x511: when set, `DeleteFile` reports success without deleting.
    pub field_0x509: bool,
    /// +0x5cb.
    pub field_0x5c3: bool,
    /// `mDDInterface->mIs3D` (DDInterface +0x38), read by `Is3DAccelerated()`.
    pub ddinterface_is_3d: bool,
    /// +0x4b0 `mCursorNum`.
    pub field_0x4a8: i32,
    /// +0x4c0 `mDeferredDeletes` (std::list<Widget*>).
    pub field_0x4b8: Vec<Ptr>,
    /// +0x4d2 `mActive`: set by the constructor; the (replaced) window procedure clears it
    /// while the window is inactive.
    pub field_0x4ca: bool,
}

/// `SexyApp_data` (object offset 0x638).
#[derive(Debug, Default)]
pub struct SexyApp_data {
    /// +0x6a0 `mLastVerCheckQueryTime` (written to the registry at shutdown).
    pub offset_0x68: i32,
    /// +0x6a4: the app shut down because an URL opened.
    pub offset_0x6c: bool,
    /// +0x6c8 `mRegUserName`.
    pub field_0x90: Vec<u8>,
    /// +0x700 `mRegCode`.
    pub field_0xc8: Vec<u8>,
    /// +0x6a5 `mDontUpdate`: no update checks (hides the Options dialog's Check Updates
    /// button); the "DontUpdate" property.
    pub field_0x6d: bool,
    /// +0x71c `mIsRegistered`.
    pub offset_0xe4: bool,
    /// +0x71d `mBuildUnlocked`.
    pub offset_0xe5: bool,
    /// +0x720 `mTimesPlayed` (trial plays / minutes used).
    pub offset_0xe8: i32,
    /// +0x724 `mTimesExecuted`.
    pub offset_0xec: i32,
}

/// `WinFishApp_data` (object offset 0x72c).
#[derive(Debug, Default)]
pub struct WinFishApp_data {
    /// +0x730 `mBoard`.
    pub offset_0x4: Ptr,
    /// +0x734: the title screen while it is up.
    pub offset_0x8: Ptr,
    /// +0x738: the game selector while it is up.
    pub offset_0xc: Ptr,
    /// +0x73c: a 0x518-byte table allocated by Init.
    pub offset_0x10: Vec<i32>,
    /// +0x740: day number at start.
    pub offset_0x14: i64,
    /// +0x748: day number.
    pub offset_0x1c: i64,
    /// +0x750.
    pub offset_0x24: i32,
    /// +0x758: a std::list.
    pub offset_0x2c: Vec<Ptr>,
    /// +0x764: a std::list.
    pub offset_0x38: Vec<Ptr>,
    /// +0x7b8: the partner refer id sent with update checks (std::string; nothing sets it).
    pub field_0x8c: Vec<u8>,
    /// +0x76c: "ScreenSaver\\" (std::string).
    pub field_0x40: Vec<u8>,
    /// +0x7a4.
    pub offset_0x78: i32,
    /// +0x7a8.
    pub offset_0x7c: i32,
    /// +0x7ac.
    pub offset_0x80: i32,
    /// +0x830: the Hall of Fame while it is up.
    pub offset_0x104: Ptr,
    /// +0x834: the help screen while it is up.
    pub offset_0x108: Ptr,
    /// +0x788: "ScreenSaver\\" (the screensaver's registry prefix; std::string).
    pub field_0x5c: Vec<u8>,
    /// +0x838: the system screensaver is this game's.
    pub offset_0x10c: bool,
    /// +0x83e: the screensaver's Powersave setting.
    pub offset_0x112: bool,
    /// +0x840: the system screensaver before ours ("OldPath"; std::string).
    pub field_0x114: Vec<u8>,
    /// +0x85c: the screensaver's user (std::string).
    pub field_0x130: Vec<u8>,
    /// +0x839.
    pub offset_0x10d: bool,
    /// +0x83a.
    pub offset_0x10e: bool,
    /// +0x83b.
    pub offset_0x10f: bool,
    /// +0x83c: the screensaver shows the shells collected.
    pub offset_0x110: bool,
    /// +0x83d: the screensaver shows the "running" warning.
    pub offset_0x111: bool,
    /// +0x870.
    pub offset_0x144: i32,
    /// +0x878: the loading thread finished a group.
    pub offset_0x14c: i32,
    /// +0x880.
    pub offset_0x154: bool,
    /// +0x881.
    pub offset_0x155: bool,
    /// +0x882.
    pub offset_0x156: bool,
    /// +0x883.
    pub offset_0x157: bool,
    /// +0x884: a play is in progress (counted when it ends).
    pub offset_0x158: bool,
    /// +0x888: the tank for the non-adventure modes (1 initially).
    pub offset_0x15c: i32,
    /// +0x88c.
    pub offset_0x160: i32,
    /// +0x890.
    pub offset_0x164: i32,
    /// +0x894: trial time accumulated this session.
    pub offset_0x168: i32,
    /// +0x898: plays finished this session.
    pub offset_0x16c: i32,
    /// +0x89c: `MaxExecutions` property.
    pub offset_0x170: i32,
    /// +0x8a0: `MaxPlays` property.
    pub offset_0x174: i32,
    /// +0x8a4: `MaxTime` property.
    pub offset_0x178: i32,
    /// +0x80c: an overlay screen that pauses the game.
    pub offset_0xe0: Ptr,
    /// +0x810: the in-game dialog/screen that takes focus back after dialogs close.
    pub offset_0xe4: Ptr,
    /// +0x814: the hatch screen while it is up.
    pub offset_0xe8: Ptr,
    /// +0x818: the bonus screen while it is up.
    pub offset_0xec: Ptr,
    /// +0x81c: the interlude screen while it is up.
    pub offset_0xf0: Ptr,
    /// +0x824: an overlay screen that pauses the game.
    pub offset_0xf8: Ptr,
    /// +0x828: an overlay screen that pauses the game.
    pub offset_0xfc: Ptr,
    /// +0x820: the story screen while it is up.
    pub offset_0xf4: Ptr,
    /// +0x82c: the tank screen (Time Trial / Challenge) while it is up.
    pub offset_0x100: Ptr,
    /// +0x7b0: the game's own `MTRand*`.
    pub offset_0x84: Ptr,
    /// +0x87c: current game mode.
    pub offset_0x150: i32,
    /// +0x8ac: the `HighScoreMgr`.
    pub offset_0x180: Ptr,
    /// +0x8b0: the `ProfileMgr`.
    pub offset_0x184: Ptr,
    /// +0x8b4.
    pub offset_0x188: i32,
    /// +0x8b8: the current user profile.
    pub offset_0x18c: Ptr,
}

#[derive(Debug, Default)]
pub struct WinFishApp {
    pub sab: SexyAppBase_data,
    pub sa: SexyApp_data,
    pub wfa: WinFishApp_data,
}

impl G {
    pub fn app_obj(&mut self, p: Ptr) -> &mut WinFishApp {
        match &mut self.obj(p).node {
            Node::App(a) => a,
            n => panic!("{p} is not the app: {n:?}"),
        }
    }
    /// `SexyAppBase_data` of the app.
    pub fn sab(&mut self, p: Ptr) -> &mut SexyAppBase_data {
        &mut self.app_obj(p).sab
    }
    /// `WinFishApp_data` of the app.
    pub fn wfa(&mut self, p: Ptr) -> &mut WinFishApp_data {
        &mut self.app_obj(p).wfa
    }
}
