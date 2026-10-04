//! Replacements for the OS-facing parts of `SexyAppBase` / `SexyApp` initialization:
//! window and DirectDraw setup, the property files, the registry.
//!
//! `SexyAppBase::Init` (@ 004883f0) creates the window, the DirectDraw device and screen
//! surface, the sound and music managers, the `WidgetManager` and the `ResourceManager`,
//! and reads settings from the registry; `SexyApp::Init` adds `properties\partner.xml`.
//! [`sexy_app_init`] performs the same state changes the game relies on, with Bevy owning
//! the window. The registry lives in `G::registry`, kept in the game folder as
//! `userdata\registry.ini` (one `name=value` per line).

use crate::sexy::prelude::*;
use std::collections::HashMap;

/// `SexyAppBase` property maps (`mBoolProperties` +0x604, `mIntProperties` +0x610,
/// `mStringProperties`), filled from `properties\partner.xml`.
#[derive(Debug, Default, Clone)]
pub struct Properties {
    pub bools: HashMap<String, bool>,
    pub ints: HashMap<String, i32>,
    pub strings: HashMap<String, String>,
}

/// `PropertiesParser::ParsePropertiesFile` (replaced): `<Boolean id=..>`, `<Integer id=..>`,
/// `<String id=..>` elements.
pub fn parse_properties(text: &str, props: &mut Properties) {
    let mut rest = text;
    while let Some(start) = rest.find('<') {
        rest = &rest[start + 1..];
        let Some(end) = rest.find('>') else { break };
        let tag = &rest[..end];
        rest = &rest[end + 1..];
        let mut parts = tag.split_whitespace();
        let Some(kind) = parts.next() else { continue };
        if !matches!(kind, "Boolean" | "Integer" | "String") {
            continue;
        }
        let id = tag.split("id=\"").nth(1).and_then(|s| s.split('"').next()).unwrap_or("").to_string();
        let Some(close) = rest.find("</") else { break };
        let value = rest[..close].trim();
        match kind {
            "Boolean" => {
                props.bools.insert(id, value.eq_ignore_ascii_case("true") || value == "1");
            }
            "Integer" => {
                props.ints.insert(id, value.parse().unwrap_or(0));
            }
            _ => {
                props.strings.insert(id, value.to_string());
            }
        }
    }
}

/// port: 00487170 FUN_00487170
/// `SexyAppBase::GetBoolean(const std::string& id, bool default)`.
pub fn FUN_00487170(g: &mut G, param_1: &str, param_2: bool) -> bool {
    g.properties.bools.get(param_1).copied().unwrap_or(param_2)
}

/// port: 004871e0 FUN_004871e0
/// `SexyAppBase::GetInteger(const std::string& id, int default)`.
pub fn FUN_004871e0(g: &mut G, param_1: &str, param_2: i32) -> i32 {
    g.properties.ints.get(param_1).copied().unwrap_or(param_2)
}

/// `SexyAppBase::RegistryReadString` (@ 0047ec70) against the replaced registry.
pub fn registry_read_string(g: &mut G, key: &str) -> Option<String> {
    g.registry.get(key).cloned()
}

/// `SexyAppBase::RegistryReadInteger` against the replaced registry.
pub fn registry_read_integer(g: &mut G, key: &str) -> Option<i32> {
    g.registry.get(key).and_then(|v| v.parse().ok())
}

/// `SexyAppBase::RegistryReadBoolean` (@ 0047ed70) against the replaced registry (stored as
/// an integer, non-zero is true).
pub fn registry_read_boolean(g: &mut G, key: &str) -> Option<bool> {
    registry_read_integer(g, key).map(|v| v != 0)
}

/// Where the replaced registry is kept (written through the save path, like the profiles).
pub const REGISTRY_FILE: &str = "userdata\\registry.ini";

/// Reads the saved registry (before `SexyAppBase::Init` reads its settings).
pub fn registry_load(g: &mut G) {
    let Some(bytes) = g.vfs.read(REGISTRY_FILE) else { return };
    let text = String::from_utf8_lossy(bytes).to_string();
    for line in text.lines() {
        if let Some((k, v)) = line.split_once('=') {
            g.registry.insert(k.trim().to_string(), v.trim().to_string());
        }
    }
}

/// `RegSetValueEx` on the replaced registry: the value, and the file rewritten when it changed.
pub fn registry_write(g: &mut G, key: &str, value: String) {
    if g.registry.get(key) == Some(&value) {
        return;
    }
    g.registry.insert(key.to_string(), value);
    let mut keys: Vec<_> = g.registry.iter().collect();
    keys.sort();
    let text: String = keys.iter().map(|(k, v)| format!("{k}={v}\r\n")).collect();
    g.vfs.write(REGISTRY_FILE, text.into_bytes());
}

/// What `SexyAppBase::Init` + `SexyApp::Init` leave behind that the game reads: screen size,
/// the screen image, the widget manager (resized to the screen), properties, and the
/// registration flags `partner.xml` implies.
pub fn sexy_app_init(g: &mut G, app: Ptr) {
    if let Some(bytes) = g.vfs.read("properties\\partner.xml") {
        let text = String::from_utf8_lossy(bytes).to_string();
        let mut props = Properties::default();
        parse_properties(&text, &mut props);
        g.properties = props;
    }
    // SexyAppBase::Init: ReadFromRegistry (vtable +0x60, WinFishApp's override).
    crate::game::win_fish_app::vfunction25(g, app);
    // SexyApp::Init: `if (GetBoolean("NoReg", false)) { mIsRegistered = true; mBuildUnlocked = true; }`
    if FUN_00487170(g, "NoReg", false) {
        g.app_obj(app).sa.offset_0xe4 = true;
        g.app_obj(app).sa.offset_0xe5 = true;
    }
    // SexyApp::Init: `mDontUpdate = GetBoolean("DontUpdate", mDontUpdate)`.
    let dont = g.app_obj(app).sa.field_0x6d;
    g.app_obj(app).sa.field_0x6d = FUN_00487170(g, "DontUpdate", dont);
    g.sab(app).field_0x390 = b"1.1".to_vec();
    g.sab(app).d3d_tester = true;
    let (w, h) = (640, 480);
    g.sab(app).field_0xb8 = w;
    g.sab(app).field_0xbc = h;
    let mut screen = crate::sexy::image::Image::new();
    screen.offset_0x20 = w;
    screen.offset_0x24 = h;
    screen.mBits = vec![0xff00_0000; (w * h) as usize];
    let screen = g.alloc(Obj { vt: None, node: Node::Image(Box::new(screen)) });
    let cmds = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let wm = crate::sexy::widget_manager::WidgetManager(g, app, screen, cmds);
    g.sab(app).offset_0x318 = wm;
    let r = Rect::new(0, 0, w, h);
    crate::sexy::widget_manager::FUN_0046d2d0(g, wm, r, r);
    // SexyAppBase::Init: mMusicInterface = CreateMusicInterface() (WinFishApp's override).
    let mi = crate::game::win_fish_app::vfunction19(g, app);
    g.sab(app).offset_0x36c = mi;
    // SexyAppBase::Init, once the sound manager and music interface exist:
    // SetSfxVolume(mSfxVolume), SetMusicVolume(mMusicVolume) (vtable +0xf4 / +0xf0).
    let sfx = g.sab(app).field_0xd0;
    crate::sexy::sexy_app_base::vfunction62(g, app, sfx);
    let music = g.sab(app).field_0xc8;
    crate::game::win_fish_app::vfunction61(g, app, music);
    // SexyAppBase::Init, once the window is made: PreDisplayHook (WinFishApp's override).
    crate::game::win_fish_app::vfunction17(g, app);
}

/// port: 0047fbc0 FUN_0047fbc0
/// `SexyAppBase::DeleteFile(const std::string&)`: deletes the file (from the in-memory file
/// system now, from disk when the host flushes); true when it existed.
pub fn FUN_0047fbc0(g: &mut G, param_1: &str) -> bool {
    let app = g.globals.DAT_005eb6a4;
    if g.sab(app).field_0x509 {
        return true;
    }
    g.vfs.remove(param_1)
}

/// port: 004107c0 FUN_004107c0
/// `CommaSeperate(int)`: the number formatted through a stringstream imbued with the
/// "LOCALE" property's locale (default "English_United_States"): thousands grouped with
/// commas. (Host boundary: that locale's grouping is applied whatever the property says.)
pub fn FUN_004107c0(_g: &mut G, param_1: i32) -> Vec<u8> {
    let digits = (param_1 as i64).unsigned_abs().to_string();
    let mut out = String::new();
    if param_1 < 0 {
        out.push('-');
    }
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    out.into_bytes()
}

/// `mMusicInterface->PlayMusic(song, order, noLoop)` (BASS interface; see `crate::sexy::music`).
pub fn music_play(g: &mut G, music: i32, order: i32, no_loop: bool) {
    crate::sexy::music::play_music(&mut g.music, music, order, no_loop);
}

/// `mMusicInterface->FadeIn(song, order, speed, noLoop)`.
pub fn music_fade_in(g: &mut G, music: i32, order: i32, speed: f64, no_loop: bool) {
    crate::sexy::music::fade_in(&mut g.music, music, order, speed, no_loop);
}

/// `mMusicInterface->FadeOut(song, stopSong, speed)`.
pub fn music_fade_out(g: &mut G, music: i32, stop: bool, speed: f64) {
    crate::sexy::music::fade_out(&mut g.music, music, stop, speed);
}

/// Replacement for `SexyAppBase::vfunction38` (@ 0047ae40, `OpenURL`: `ShellExecute`s the
/// URL). Host boundary: the port opens no browser, so the request is only logged; it
/// reports success like a browser that opened.
pub fn open_url(g: &mut G, app: Ptr, url: &[u8], shutdown: bool) -> bool {
    if g.sab(app).field_0x3e4 && g.sab(app).field_0x3e8 == url {
        return true;
    }
    g.sab(app).field_0x3e5 = shutdown;
    g.sab(app).field_0x3e4 = true;
    g.sab(app).field_0x3e8 = url.to_vec();
    g.sab(app).field_0x404 = g.tick_count;
    bevy::log::info!("URL requested (not opened): {}", String::from_utf8_lossy(url));
    true
}

/// Part of `SexyAppBase::UpdateAppStep` (replaced): an URL being opened counts as opened
/// once the app loses the focus to the browser (`URLOpenSucceeded`, app vtable +0x94 + 4)
/// and as failed after 8 seconds still in front (`URLOpenFailed`). The port never opens a
/// browser, so it is the failure path (with the URL to open by hand) unless the user
/// switches away.
pub fn update_opening_url(g: &mut G, app: Ptr) {
    if !g.sab(app).field_0x3e4 {
        return;
    }
    if !g.sab(app).field_0x4ca {
        crate::game::win_fish_app::vfunction37(g, app);
    } else if 8000 < g.tick_count.wrapping_sub(g.sab(app).field_0x404) {
        let url = g.sab(app).field_0x3e8.clone();
        crate::game::win_fish_app::vfunction36(g, app, &url);
    }
}

/// Replacement for `SexyAppBase::WriteError`-style crash log (`SexyAppBase::vfunction34`,
/// appending to "ScrError.txt"): logged.
pub fn write_error_log(_g: &mut G, _app: Ptr, code: i32) {
    bevy::log::error!("error log entry {code}");
}

/// Replacement for `SexyAppBase::DebugKeyDown` (vfunction86: the 3D-mode switch and perf
/// results keys of debug builds): not handled.
pub fn debug_key_down(_g: &mut G, _app: Ptr, _key: i32) -> bool {
    false
}

/// Replacement for `SetCurrentDirectoryA`: the port's paths are relative to the game folder
/// it was started with, so nothing changes; false like a failed call.
pub fn set_current_dir(_g: &mut G, _dir: &[u8]) -> bool {
    false
}

/// Replacement for `IsVistaOrLater()` (`FUN_0040fab0`, `GetVersionEx` major > 5): the
/// port takes the OS to be NT 6+.
pub fn is_nt6(_g: &mut G) -> bool {
    true
}

/// Replacement for `GetAppDataFolder()` (`FUN_0040fb60`, `DAT_005e7f48`): the in-memory
/// file system's root (the game's writable files live at relative paths).
pub fn app_data_folder(_g: &mut G) -> Vec<u8> {
    Vec::new()
}

/// Replacement for `SexyAppBase::Popup(const string&)` (a Win32 message box): logged.
pub fn popup(_g: &mut G, msg: &[u8]) {
    bevy::log::warn!("popup: {}", String::from_utf8_lossy(msg));
}

/// `mMusicInterface->StopMusic(song)`.
pub fn music_stop(g: &mut G, music: i32) {
    crate::sexy::music::stop_music(&mut g.music, music);
}

/// `mMusicInterface->StopAllMusic()`.
pub fn music_stop_all(g: &mut G) {
    crate::sexy::music::stop_all_music(&mut g.music);
}

/// Replacement for `GetUserNameA`: the host user's name (`USERNAME` / `USER`), when known.
pub fn user_name(_g: &mut G) -> Option<Vec<u8>> {
    std::env::var("USERNAME").or_else(|_| std::env::var("USER")).ok().map(|s| s.into_bytes())
}

/// Replacement for the folded `@ 0040ca20` (`InternetManager::GetUpdateURL()`: the URL from
/// the parsed update reply, `FUN_0040b2d0`, or ""). The port's update checks always fail
/// (no network), so there is never a reply and the URL is empty.
pub fn update_check_new_version_url(_g: &mut G, _app: Ptr) -> Vec<u8> {
    Vec::new()
}

/// Replacement for `SexyApp::Validate(name, code)` (`FUN_00490d00` @ 00490d00, framework):
/// an RSA signature check of the registration code over an MD5 of the cleaned-up name and
/// the product name. Not reproduced by the port (the shipped build is registered through
/// partner.xml): every code is refused, which the game reports as "Invalid Code".
pub fn validate_registration(_g: &mut G, _app: Ptr, _name: &[u8], _code: &[u8]) -> bool {
    false
}

/// Replacement for `SexyApp::vfunction101` (@ 00493780, `OpenRegisterPage()`: builds the
/// registration URL from the product's parameters and opens it in the browser). Host
/// boundary: the port opens no browser and makes no network requests, so the request is
/// only logged.
pub fn open_registration_page(_g: &mut G, _app: Ptr) {
    bevy::log::info!("registration page requested (not opened)");
}

/// Replacement for `InternetManager::CheckForUpdates(url)` (`FUN_0040b1d0` @ 0040b1d0, an
/// HTTP GET on a worker thread). Host boundary: the port makes no network requests, so the
/// query fails at once (`update_check_result` reports it).
pub fn check_for_updates(g: &mut G, _url: &[u8]) {
    g.update_check_failed = true;
}

/// Replacement for `InternetManager::GetResultCode()` (`FUN_0040b1f0`): 3 (failed) once a
/// check was started, 0 (busy) before.
pub fn update_check_result(g: &mut G) -> i32 {
    if g.update_check_failed {
        3
    } else {
        0
    }
}

/// Replacement for `InternetManager::ParseResult()` (`FUN_0040ca80`): reads the server's
/// version list; never reached in the port (the check never succeeds).
pub fn update_check_parse(_g: &mut G) {}

/// Replacement for `InternetManager::IsUpToDate()` (`FUN_0040ca00`): true without a parsed
/// answer, as in the original.
pub fn update_check_up_to_date(_g: &mut G) -> bool {
    true
}
