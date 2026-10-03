//! [`G`]: the whole state of the running program (every heap object plus the globals the
//! original kept in `.data`). It is one Bevy [`Resource`]; systems reach it through
//! `ResMut<G>`, so there is no global state outside the ECS.

use crate::sexy::prelude::*;
use bevy::prelude::Resource;

/// Globals of the original, named by address. Only those the ported code uses exist.
#[derive(Debug, Default)]
pub struct Globals {
    /// `DAT_005eb6a4`: `gSexyAppBase` / the `WinFishApp*`.
    pub DAT_005eb6a4: Ptr,
    /// `DAT_005e9090`: the store's day-indexed trait table (built on first use).
    pub DAT_005e9090: Vec<i32>,
    /// `DAT_005e9220`: `DAT_005e9090` is built.
    pub DAT_005e9220: bool,
    /// `DAT_005e9628`: which sample results the board-less bonus screen shows next.
    pub DAT_005e9628: i32,
    /// `DAT_005e9088`: the story last viewed.
    pub DAT_005e9088: i32,
    /// `DAT_005e8f1c`: compared against 1 by `GameObject::vfunction73`.
    pub DAT_005e8f1c: i32,
    /// `_DAT_0062090c`: counter bumped by `WidgetContainer::SysColorChangedAll`.
    pub DAT_0062090c: i32,
    /// `DAT_005e89cd`: set by `Board::Update` while the board is live and not paused.
    pub DAT_005e89cd: bool,
    /// `DAT_005e6f16`.
    pub DAT_005e6f16: bool,
    /// `DAT_005e8f17`: resources were extracted.
    pub DAT_005e8f17: bool,
    /// `DAT_005e8f28`: the `WinFishApp*` WinMain created.
    pub DAT_005e8f28: Ptr,
    /// `DAT_005e9638`: the slow-motion debug mode's frame counter (0..3).
    pub DAT_005e9638: i32,
    /// `DAT_005e9634`.
    pub DAT_005e9634: bool,
    /// `DAT_005e963c`: the screensaver's background job was queued.
    pub DAT_005e963c: bool,
    /// `DAT_005e8f16`: the fish songs are being loaded in the background.
    pub DAT_005e8f16: bool,
    /// The fish songs and their play lists (`DAT_005e8f15`, `DAT_005e8f20`..`DAT_005e8f70`).
    pub songs: crate::game::fish_songs::SongLibrary,
    /// `DAT_005e0f30`: the last parse error (empty = none).
    pub DAT_005e0f30: Vec<u8>,
    /// `DAT_005e89ce`: set from profile flag bit 4 when a board is created.
    pub DAT_005e89ce: bool,
    /// `DAT_005e7360`: `gDialogColors` (7 x RGB), mutable.
    pub DAT_005e7360: crate::sexy::dialog::DialogColors,
    /// `DAT_005e15f8`: the help page shown last (initialized to 1).
    pub DAT_005e15f8: i32,
    /// `DAT_005e15fc`: the Hall of Fame tab shown last (initialized to 1).
    pub DAT_005e15fc: i32,
    /// `DAT_005e89dc`.
    pub DAT_005e89dc: i32,
    /// `DAT_005df58c` (initialized to 1).
    pub DAT_005df58c: i32,
    /// `DAT_005e89cc`.
    pub DAT_005e89cc: bool,
    /// `DAT_005e89d0`.
    pub DAT_005e89d0: i32,
    /// `DAT_005e89c0`: breeders are held at (`DAT_005e89c4`, `DAT_005e89c8`).
    pub DAT_005e89c0: i32,
    pub DAT_005e89c4: i32,
    pub DAT_005e89c8: i32,
    /// `DAT_005e89d4`, `DAT_005e89d8`: where the fish swim to in follow mode (`DAT_005e89d0`).
    pub DAT_005e89d4: i32,
    pub DAT_005e89d8: i32,
    /// `DAT_005df590` (5000).
    pub DAT_005df590: i32,
    /// `_DAT_005e8f18`.
    pub DAT_005e8f18: i32,
    /// `DAT_005e89e8`: the tank backdrop's tint (set per backdrop by `FUN_00537d30`).
    pub DAT_005e89e8: Color,
    /// `DAT_005e8f14`: pets are mouse-visible (set while one is selected).
    pub DAT_005e8f14: bool,
    /// `DAT_005e89f8`: a pet is forwarding a click to the board (re-entry guard).
    pub DAT_005e89f8: bool,
    /// `DAT_005e89f9`: a swimming pet is forwarding a click to the board (re-entry guard).
    pub DAT_005e89f9: bool,
    /// `DAT_005e9631`: a click is being passed to the tank (re-entry guard).
    pub DAT_005e9631: bool,
    /// `DAT_005e9630`: a click is being passed to the pets (re-entry guard).
    pub DAT_005e9630: bool,
    /// `DAT_005e1600`: the ending's pet return times are still to be worked out (set at
    /// start and by `FUN_00518650`; cleared by the ending's first draw).
    pub DAT_005e1600: bool,
    /// `DAT_005e9040`: the ending's scroll position at which each lost pet comes back
    /// (`int[15]`; -100 for slots without one).
    pub DAT_005e9040: [i32; 15],
    /// `DAT_005e907c`: the length of the ending's roll.
    pub DAT_005e907c: i32,
    /// `DAT_005e9084`: the ending's scroll position where the credits start (0 = not known).
    pub DAT_005e9084: i32,
    /// `DAT_005e9080`: count of `DAT_005e8fe0` (the pets lost in the final battle).
    pub DAT_005e9080: i32,
    /// `DAT_005e8fe0`: the pets lost in the final battle, then -1 (`int[0x28]`; grown to 0x28 entries on first use).
    pub DAT_005e8fe0: Vec<i32>,
}

#[derive(Resource)]
pub struct G {
    /// The heap: index = [`Ptr`]. Slot 0 is the null pointer and stays empty.
    pub objs: Vec<Option<Obj>>,
    pub globals: Globals,
    /// `DAT_005eb928`: the global `Sexy::MTRand` behind `Sexy::Rand()` (`FUN_0040f9e0`).
    pub DAT_005eb928: MTRand,
    /// Wall clock for `_time64` (seconds since 1970), set by the host each frame so the
    /// game logic itself never blocks on or reads the OS.
    pub now_time64: i64,
    /// `GetTickCount()`: milliseconds of a monotonic clock, set by the host each frame.
    pub tick_count: u32,
    /// Cursor the game asked for (`SexyAppBase::mCursorNum` after `EnforceCursor`).
    pub cursor_num: i32,
    /// The clipboard (replaces the Win32 clipboard behind `CopyToClipboard`/`GetClipboard`).
    pub clipboard: Vec<u8>,
    /// The replaced InternetManager: an update check was started (and, without network, failed).
    pub update_check_failed: bool,
    /// The C runtime's `rand()` state (`_holdrand`, 1 at start; the game never calls `srand`).
    pub crt_holdrand: u32,
    /// Stack `MemoryImage`s drawn this frame (the original blits them before they go out of
    /// scope; here blits are replayed at the end of the frame, so they are freed then).
    pub frame_temp_images: Vec<Ptr>,
    /// The HD screen (port addition; see `crate::sexy::hd`).
    pub hd: crate::sexy::hd::HdScreen,
    /// Resource globals (`DAT_005e8aa0` = IMAGE_FOOD ...).
    pub res: crate::sexy::res_gen::Res,
    /// Sounds started this frame, drained by the host.
    pub sound_requests: Vec<crate::sexy::sexy_app_base::SoundRequest>,
    /// The game's files (see `crate::sexy::vfs`).
    pub vfs: crate::sexy::vfs::Vfs,
    /// `mResourceManager` (SexyAppBase +0x634), replaced; see `crate::sexy::resource_manager`.
    pub resource_manager: crate::sexy::resource_manager::ResourceManager,
    /// `partner.xml` properties (see `crate::sexy::app_host`).
    pub properties: crate::sexy::app_host::Properties,
    /// The registry values under the game's key (replaced registry).
    pub registry: std::collections::HashMap<String, String>,
    /// Local time minus UTC, seconds (for `_localtime64`).
    pub utc_offset_secs: i64,
    /// `mMusicInterface`'s state (see `crate::sexy::music`); the host plays it.
    pub music: crate::sexy::music::MusicInterface,
    /// The loading thread's progress, while it runs.
    pub loading_thread: Option<crate::game::win_fish_app::LoadingThread>,
}

impl Default for G {
    fn default() -> Self {
        G { objs: vec![None], globals: Globals { DAT_005e15f8: 1, DAT_005e1600: true, DAT_005e15fc: 1, DAT_005df58c: 1, DAT_005df590: 5000, ..Globals::default() }, DAT_005eb928: MTRand::default(), now_time64: 0, tick_count: 0, cursor_num: 0, clipboard: Vec::new(), update_check_failed: false, crt_holdrand: 1, frame_temp_images: Vec::new(), hd: Default::default(), res: Default::default(), sound_requests: Vec::new(), vfs: Default::default(), resource_manager: Default::default(), properties: Default::default(), registry: Default::default(), utc_offset_secs: 0, music: Default::default(), loading_thread: None }
    }
}
