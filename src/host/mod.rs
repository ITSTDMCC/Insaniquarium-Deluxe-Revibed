//! The Bevy side: window, timing, input, screen upload, sound playback, file loading.
//!
//! This replaces the parts of `SexyAppBase` that talk to Windows and DirectX (window
//! creation, the message pump, DirectDraw presentation, DirectSound). Game state lives in
//! the [`G`] resource and is only touched from these systems, on Bevy's main schedule.
//!
//! Timing: the framework runs logic in fixed `mFrameTime` steps of 10 ms (100 updates per
//! second) and redraws dirty widgets after updating; [`run_app`] does the same with Bevy's
//! frame time as the clock.

pub mod audio;
pub mod music;
pub mod openmpt;
pub mod upscale;

use crate::sexy::prelude::*;
use crate::sexy::vfs::Vfs;
use bevy::asset::RenderAssetUsages;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::tasks::{AsyncComputeTaskPool, Task, block_on, futures_lite::future};
use std::path::PathBuf;

pub const SCREEN_W: u32 = 640;
pub const SCREEN_H: u32 = 480;
/// `SexyAppBase::mFrameTime` default (ms per logic update).
pub const FRAME_TIME_MS: f64 = 10.0;

/// Where the game's install lives (images/, data/, properties/, sounds/).
#[derive(Resource, Clone)]
pub struct GameDir(pub PathBuf);

#[derive(Resource)]
struct VfsTask(Task<std::io::Result<Vfs>>);

/// Host-side bookkeeping that is not part of the original program's state.
#[derive(Resource, Default)]
pub struct HostState {
    pub booted: bool,
    pub screen: Option<Handle<Image>>,
    pub accum_ms: f64,
    pub last_mouse: Option<(i32, i32)>,
    pub frames: u64,
    /// When the host started (the origin of `G::tick_count`).
    pub started: Option<std::time::Instant>,
    /// Logic updates run under `WINFISH_FIXED_STEPS`.
    pub fixed_updates: u64,
    /// The screen mode last given to the window (`mIsWindowed`), once the app exists.
    pub applied_windowed: Option<bool>,
}

pub struct WinFishPlugin {
    pub game_dir: PathBuf,
}

impl Plugin for WinFishPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(G::default())
            .insert_resource(GameDir(self.game_dir.clone()))
            .insert_resource(ClearColor(bevy::color::Color::BLACK))
            .init_resource::<HostState>()
            .add_systems(Startup, (setup_screen, start_vfs_load))
            .add_plugins(audio::plugin)
            .add_plugins(music::plugin)
            .add_plugins(upscale::plugin)
            .add_systems(Update, (poll_vfs_load, focus, input, run_app, audio::play_sounds, music::sync_music, upload_screen, refresh_screen_material, flush_saves, apply_screen_mode, fit_screen).chain());
    }
}

/// `SexyAppBase::SwitchScreenMode` (replaced) on the window: the game's `mIsWindowed`
/// (Options' Fullscreen checkbox, the saved `ScreenMode`) picks a normal window or exclusive
/// full screen on the current monitor. The original also switched the display to 640x480;
/// here the monitor keeps its mode and the picture is scaled up (`fit_screen`).
fn apply_screen_mode(mut g: ResMut<G>, mut host: ResMut<HostState>, mut windows: Query<&mut Window>) {
    let app = g.globals.DAT_005eb6a4;
    if !host.booted || app == NULL {
        return;
    }
    let windowed = g.sab(app).field_0x33b;
    if host.applied_windowed == Some(windowed) {
        return;
    }
    let Ok(mut window) = windows.single_mut() else { return };
    window.mode = if windowed {
        bevy::window::WindowMode::Windowed
    } else {
        bevy::window::WindowMode::Fullscreen(bevy::window::MonitorSelection::Current, bevy::window::VideoModeSelection::Current)
    };
    info!("screen mode: {}", if windowed { "windowed" } else { "full screen" });
    host.applied_windowed = Some(windowed);
}

/// Scales the 640x480 picture to the window, keeping its shape (black bars on the long
/// sides); `input` maps the mouse back through the same scale.
fn fit_screen(windows: Query<&Window>, mut quads: Query<&mut Transform, With<ScreenQuad>>) {
    let Ok(window) = windows.single() else { return };
    let s = (window.width() / SCREEN_W as f32).min(window.height() / SCREEN_H as f32);
    if !(s > 0.0) {
        return;
    }
    let scale = Vec3::new(s, s, 1.0);
    for mut t in &mut quads {
        if t.scale != scale {
            t.scale = scale;
        }
    }
}

/// The quad the screen is drawn on.
#[derive(Component)]
struct ScreenQuad;

/// The screen image changes every frame; touching the material rebinds it to the new
/// texture.
fn refresh_screen_material(host: Res<HostState>, quads: Query<&MeshMaterial2d<upscale::UpscaleMaterial>>, mut materials: ResMut<Assets<upscale::UpscaleMaterial>>) {
    for m in &quads {
        if let Some(mut mat) = materials.get_mut(&m.0)
            && let Some(screen) = &host.screen
            && mat.screen != *screen
        {
            mat.screen = screen.clone();
        }
    }
}

fn setup_screen(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<upscale::UpscaleMaterial>>,
    mut host: ResMut<HostState>,
) {
    commands.spawn(Camera2d);
    let img = Image::new_fill(
        Extent3d { width: SCREEN_W, height: SCREEN_H, depth_or_array_layers: 1 },
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );
    let handle = images.add(img);
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(SCREEN_W as f32, SCREEN_H as f32))),
        MeshMaterial2d(materials.add(upscale::UpscaleMaterial::new(handle.clone()))),
        Transform::default(),
        ScreenQuad,
    ));
    host.screen = Some(handle);
}

fn start_vfs_load(mut commands: Commands, dir: Res<GameDir>) {
    let root = dir.0.clone();
    let task = AsyncComputeTaskPool::get().spawn(async move { Vfs::load_dir(&root) });
    commands.insert_resource(VfsTask(task));
}

fn poll_vfs_load(mut commands: Commands, task: Option<ResMut<VfsTask>>, mut g: ResMut<G>, mut host: ResMut<HostState>) {
    let Some(mut task) = task else { return };
    if let Some(result) = block_on(future::poll_once(&mut task.0)) {
        commands.remove_resource::<VfsTask>();
        match result {
            Ok(vfs) => {
                info!("loaded {} game files", vfs.len());
                g.vfs = vfs;
                crate::game::boot::FUN_005024f0(&mut g);
                host.booted = true;
            }
            Err(e) => error!("cannot read the game install: {e}"),
        }
    }
}

/// Window input -> `WidgetManager` mouse/key handlers (what `SexyAppBase`'s window procedure
/// did with WM_MOUSEMOVE, WM_LBUTTONDOWN, ...). Coordinates are mapped to the 640x480 screen.
/// The window procedure's `WM_ACTIVATEAPP` (replaced): `mActive` follows the window's
/// focus, and losing it is the app's `LostFocus()`. Scripted test runs keep the app active,
/// since their window is usually in the background.
fn focus(mut g: ResMut<G>, host: Res<HostState>, mut events: MessageReader<bevy::window::WindowFocused>) {
    let scripted = std::env::var_os("WINFISH_SCRIPT").is_some() || std::env::var_os("WINFISH_AUTOPLAY").is_some();
    for e in events.read() {
        if !host.booted || scripted {
            continue;
        }
        let app = g.globals.DAT_005eb6a4;
        if app == NULL {
            continue;
        }
        g.sab(app).field_0x4ca = e.focused;
        if !e.focused {
            crate::game::win_fish_app::vfunction84(&mut g, app);
        }
    }
}

fn input(
    mut g: ResMut<G>,
    mut host: ResMut<HostState>,
    windows: Query<&Window>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut wheel: MessageReader<MouseWheel>,
    mut keys: MessageReader<bevy::input::keyboard::KeyboardInput>,
) {
    if !host.booted {
        return;
    }
    let wm = crate::game::boot::widget_manager(&mut g);
    if wm == NULL {
        return;
    }
    let Ok(window) = windows.single() else { return };
    let pos = window.cursor_position().map(|p| {
        let sx = window.width() / SCREEN_W as f32;
        let sy = window.height() / SCREEN_H as f32;
        let s = sx.min(sy);
        let ox = (window.width() - SCREEN_W as f32 * s) / 2.0;
        let oy = (window.height() - SCREEN_H as f32 * s) / 2.0;
        (((p.x - ox) / s) as i32, ((p.y - oy) / s) as i32)
    });
    match (pos, host.last_mouse) {
        (Some(p), last) if Some(p) != last => {
            crate::sexy::widget_manager::FUN_0046da50(&mut g, wm, p.0, p.1);
            host.last_mouse = Some(p);
        }
        (None, Some(_)) => {
            crate::sexy::widget_manager::FUN_0046db70(&mut g, wm);
            host.last_mouse = None;
        }
        _ => {}
    }
    if let Some((x, y)) = pos {
        for (b, clicks) in [(MouseButton::Left, 1), (MouseButton::Right, -1), (MouseButton::Middle, 3)] {
            if buttons.just_pressed(b) {
                crate::sexy::widget_manager::FUN_0046d940(&mut g, wm, x, y, clicks);
            }
            if buttons.just_released(b) {
                crate::sexy::widget_manager::FUN_0046d890(&mut g, wm, x, y, clicks);
            }
        }
    }
    for ev in wheel.read() {
        crate::sexy::widget_manager::FUN_0046dba0(&mut g, wm, ev.y as i32);
    }
    // WM_KEYDOWN / WM_KEYUP with Windows virtual-key codes, WM_CHAR with the typed byte.
    for ev in keys.read() {
        if let Some(vk) = virtual_key(ev.key_code) {
            if ev.state.is_pressed() {
                crate::sexy::widget_manager::FUN_0046dc10(&mut g, wm, vk);
            } else {
                crate::sexy::widget_manager::FUN_0046dc50(&mut g, wm, vk);
            }
        }
        if ev.state.is_pressed() {
            let ch = match &ev.logical_key {
                bevy::input::keyboard::Key::Character(s) => s.chars().next().filter(|c| c.is_ascii()).map(|c| c as u8),
                bevy::input::keyboard::Key::Space => Some(b' '),
                bevy::input::keyboard::Key::Backspace => Some(8),
                bevy::input::keyboard::Key::Enter => Some(b'\r'),
                bevy::input::keyboard::Key::Tab => Some(b'\t'),
                bevy::input::keyboard::Key::Escape => Some(27),
                _ => None,
            };
            if let Some(c) = ch {
                crate::sexy::widget_manager::FUN_0046dbc0(&mut g, wm, c);
            }
        }
    }
    // Test hook: WINFISH_SCRIPT="<frame>:<action>:<arg>;..." runs input on that frame:
    // `click:<x>,<y>` (left press + release), `type:<text>` (WM_CHAR per byte), `vk:<code>`
    // (key down + up, decimal virtual-key code), `shells:<n>` (sets the profile's shells), `levels:<n>`, `finished:<n>`, `stories:<hexmask>`.
    // `WINFISH_SCRIPT=@<file>` reads the script
    // from a file. The script is parsed once into a per-frame table.
    static SCRIPT: std::sync::OnceLock<std::collections::HashMap<u64, Vec<(String, String)>>> = std::sync::OnceLock::new();
    let script = SCRIPT.get_or_init(|| {
        let mut table: std::collections::HashMap<u64, Vec<(String, String)>> = std::collections::HashMap::new();
        let Some(s) = std::env::var("WINFISH_SCRIPT").ok() else { return table };
        let text = match s.strip_prefix('@') {
            Some(path) => std::fs::read_to_string(path).unwrap_or_default(),
            None => s,
        };
        for step in text.trim().split(';') {
            let mut parts = step.splitn(3, ':');
            let (Some(f), Some(action), Some(arg)) = (parts.next(), parts.next(), parts.next()) else { continue };
            if let Ok(f) = f.parse::<u64>() {
                table.entry(f).or_default().push((action.to_string(), arg.to_string()));
            }
        }
        table
    });
    if let Some(steps) = script.get(&host.frames) {
        for (action, arg) in steps {
            script_action(&mut g, wm, action, arg);
        }
    }
    // Test hook: WINFISH_AUTOPLAY=1 plays by looking at the game state (see `autoplay`).
    static AUTOPLAY: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if *AUTOPLAY.get_or_init(|| std::env::var("WINFISH_AUTOPLAY").is_ok()) {
        autoplay(&mut g, wm, host.frames);
    }
}

/// One scripted input action (see `WINFISH_SCRIPT`).
fn script_action(g: &mut G, wm: Ptr, action: &str, arg: &str) {
    match action {
        "click" => {
            if let Some((x, y)) = arg.split_once(',') {
                click(g, wm, x.parse().unwrap_or(0), y.parse().unwrap_or(0));
            }
        }
        "type" => {
            for b in arg.bytes() {
                crate::sexy::widget_manager::FUN_0046dbc0(g, wm, b);
            }
        }
        "vk" => {
            let vk: u32 = arg.parse().unwrap_or(0);
            crate::sexy::widget_manager::FUN_0046dc10(g, wm, vk);
            crate::sexy::widget_manager::FUN_0046dc50(g, wm, vk);
        }
        // `shells:<n>` sets the current profile's shells (test setup only; not a game path).
        "shells" => {
            let app = g.globals.DAT_005eb6a4;
            let profile = g.wfa(app).offset_0x18c;
            if profile != NULL {
                g.profile(profile).field_0x48 = arg.parse().unwrap_or(0);
            }
        }
        // `levels:<n>`: the first n levels finished (n pets earned), the adventure placed
        // after them (test setup only).
        "levels" => {
            let app = g.globals.DAT_005eb6a4;
            let profile = g.wfa(app).offset_0x18c;
            if profile != NULL {
                let n = arg.parse::<usize>().unwrap_or(0).min(24);
                let p = g.profile(profile);
                for i in 0..24 {
                    p.field_0x0[i] = i < n;
                }
                p.field_0x18 = n as i32;
                p.field_0x1c = (n / 6) as i32 + 1;
                p.field_0x20 = (n % 6) as i32 + 1;
            }
        }
        // `stories:<mask>`: the unlocked stories (profile +0x7c; test setup only).
        "stories" => {
            let app = g.globals.DAT_005eb6a4;
            let profile = g.wfa(app).offset_0x18c;
            if profile != NULL {
                g.profile(profile).field_0x7c = i64::from_str_radix(arg.trim_start_matches("0x"), 16).unwrap_or(0) as i32;
            }
        }
        // `ending:<lost pets, comma separated>`: opens the ending as if the final battle had
        // just been won losing those pets (test setup only).
        "ending" => {
            let app = g.globals.DAT_005eb6a4;
            g.globals.DAT_005e8fe0.resize(0x28, 0);
            g.globals.DAT_005e9080 = 0;
            for p in arg.split(',').filter(|s| !s.is_empty()) {
                let n = g.globals.DAT_005e9080 as usize;
                g.globals.DAT_005e8fe0[n] = p.parse().unwrap_or(0);
                g.globals.DAT_005e9080 += 1;
            }
            crate::game::win_fish_app::FUN_0054b360(g, app);
            crate::game::interlude_screen::FUN_0054bd80(g, app);
        }
        // `finished:<n>`: the adventure counted as beaten n times (test setup only).
        "finished" => {
            let app = g.globals.DAT_005eb6a4;
            let profile = g.wfa(app).offset_0x18c;
            if profile != NULL {
                let n: i32 = arg.parse().unwrap_or(1);
                let p = g.profile(profile);
                p.field_0x50 = n;
                p.field_0x59 = 0 < n;
            }
        }
        _ => {}
    }
}

/// A left click (move, press, release) at (x, y).
fn click(g: &mut G, wm: Ptr, x: i32, y: i32) {
    crate::sexy::widget_manager::FUN_0046da50(g, wm, x, y);
    crate::sexy::widget_manager::FUN_0046d940(g, wm, x, y, 1);
    crate::sexy::widget_manager::FUN_0046d890(g, wm, x, y, 1);
}

/// Test-only player (`WINFISH_AUTOPLAY`): every 4th frame one click chosen from the game
/// state, the way a person would play. In a tank (not paused): shoot an alien, collect
/// a coin, feed a hungry guppy (at most two pellets out), buy the egg (shop slot 7) with
/// 3000 shells, guppies (slot 1) while there are fewer than 4, and with 1000 shells or more
/// the other slots (2..7) in turn. Elsewhere it cycles through the
/// spots of the menu and screen buttons (continue, adventure, dialog OK, dialog Yes). Pure input:
/// it never writes game state.
fn autoplay(g: &mut G, wm: Ptr, frame: u64) {
    use crate::game::board_level::vec_index;
    if frame % 4 != 0 {
        return;
    }
    let app = g.globals.DAT_005e8f28;
    if app == NULL {
        return;
    }
    let board = g.wfa(app).offset_0x4;
    let in_tank = board != NULL && g.is_live(board) && g.wc(board).offset_0x10 != NULL && !g.board(board).field_0x8 && g.sab(app).offset_0x32c.is_empty() && g.sab(app).offset_0x320.is_empty();
    if in_tank {
        let center = |g: &mut G, o: Ptr| {
            let wc = g.wc(o).clone();
            (wc.offset_0x2c + wc.offset_0x34 / 2, wc.offset_0x30 + wc.offset_0x38 / 2)
        };
        let aliens = g.board(board).offset_0xc[vec_index(0xb8)].clone();
        if let Some(&a) = aliens.first() {
            let (x, y) = center(g, a);
            click(g, wm, x, y);
            return;
        }
        let coins = g.board(board).offset_0xc[vec_index(0xa8)].clone();
        for c in coins {
            if !g.coin(c).offset_0x44 {
                let (x, y) = center(g, c);
                if (0..640).contains(&x) && (75..480).contains(&y) {
                    click(g, wm, x, y);
                    return;
                }
            }
        }
        let money = g.board(board).field_0x364;
        let guppies = g.board(board).offset_0xc[vec_index(0xa0)].clone();
        let food = g.board(board).offset_0xc[vec_index(0xac)].len();
        if food < 2 {
            for f in &guppies {
                if g.go(*f).offset_0x14 < 300 {
                    let (x, y) = center(g, *f);
                    click(g, wm, x, (y - 30).max(90));
                    return;
                }
            }
        }
        if frame % 200 == 0 {
            if money >= 3000 {
                click(g, wm, 0x1b4 + 0x1d, 33);
                return;
            }
            if guppies.len() < 4 && money >= 100 {
                click(g, wm, 47, 33);
                return;
            }
            if money >= 1000 {
                // The shop slots' x (`FUN_005409b0`), centers; slots 2..7 in turn.
                const XS: [i32; 6] = [0x57, 0x90, 0xd9, 0x122, 0x16b, 0x1b4];
                let x = XS[((frame / 200) % 6) as usize] + 0x1d;
                click(g, wm, x, 33);
                return;
            }
        }
        return;
    }
    if frame % 120 == 0 {
        const SPOTS: [(i32, i32); 7] = [(320, 431), (320, 458), (460, 85), (320, 270), (320, 300), (320, 440), (243, 271)];
        let (x, y) = SPOTS[((frame / 120) % SPOTS.len() as u64) as usize];
        if frame % 1200 == 0 {
            let paused = board != NULL && g.is_live(board) && g.board(board).field_0x8;
            info!("autoplay frame {frame}: menus (dialogs {}, list {}, board paused {paused})", g.sab(app).offset_0x320.len(), g.sab(app).offset_0x32c.len());
        }
        click(g, wm, x, y);
    }
}

/// Windows virtual-key code for a Bevy key code (the keys the game reads).
fn virtual_key(k: KeyCode) -> Option<u32> {
    Some(match k {
        KeyCode::Backspace => 0x08,
        KeyCode::Tab => 0x09,
        KeyCode::Enter | KeyCode::NumpadEnter => 0x0d,
        KeyCode::ShiftLeft | KeyCode::ShiftRight => 0x10,
        KeyCode::ControlLeft | KeyCode::ControlRight => 0x11,
        KeyCode::AltLeft | KeyCode::AltRight => 0x12,
        KeyCode::Pause => 0x13,
        KeyCode::Escape => 0x1b,
        KeyCode::Space => 0x20,
        KeyCode::PageUp => 0x21,
        KeyCode::PageDown => 0x22,
        KeyCode::End => 0x23,
        KeyCode::Home => 0x24,
        KeyCode::ArrowLeft => 0x25,
        KeyCode::ArrowUp => 0x26,
        KeyCode::ArrowRight => 0x27,
        KeyCode::ArrowDown => 0x28,
        KeyCode::Insert => 0x2d,
        KeyCode::Delete => 0x2e,
        KeyCode::Digit0 => 0x30,
        KeyCode::Digit1 => 0x31,
        KeyCode::Digit2 => 0x32,
        KeyCode::Digit3 => 0x33,
        KeyCode::Digit4 => 0x34,
        KeyCode::Digit5 => 0x35,
        KeyCode::Digit6 => 0x36,
        KeyCode::Digit7 => 0x37,
        KeyCode::Digit8 => 0x38,
        KeyCode::Digit9 => 0x39,
        KeyCode::KeyA => 0x41,
        KeyCode::KeyB => 0x42,
        KeyCode::KeyC => 0x43,
        KeyCode::KeyD => 0x44,
        KeyCode::KeyE => 0x45,
        KeyCode::KeyF => 0x46,
        KeyCode::KeyG => 0x47,
        KeyCode::KeyH => 0x48,
        KeyCode::KeyI => 0x49,
        KeyCode::KeyJ => 0x4a,
        KeyCode::KeyK => 0x4b,
        KeyCode::KeyL => 0x4c,
        KeyCode::KeyM => 0x4d,
        KeyCode::KeyN => 0x4e,
        KeyCode::KeyO => 0x4f,
        KeyCode::KeyP => 0x50,
        KeyCode::KeyQ => 0x51,
        KeyCode::KeyR => 0x52,
        KeyCode::KeyS => 0x53,
        KeyCode::KeyT => 0x54,
        KeyCode::KeyU => 0x55,
        KeyCode::KeyV => 0x56,
        KeyCode::KeyW => 0x57,
        KeyCode::KeyX => 0x58,
        KeyCode::KeyY => 0x59,
        KeyCode::KeyZ => 0x5a,
        KeyCode::F1 => 0x70,
        KeyCode::F2 => 0x71,
        KeyCode::F3 => 0x72,
        KeyCode::F4 => 0x73,
        KeyCode::F5 => 0x74,
        KeyCode::F6 => 0x75,
        KeyCode::F7 => 0x76,
        KeyCode::F8 => 0x77,
        KeyCode::F9 => 0x78,
        KeyCode::F10 => 0x79,
        KeyCode::F11 => 0x7a,
        KeyCode::F12 => 0x7b,
        _ => return None,
    })
}

/// Fixed 10 ms logic steps, then one redraw of the dirty widgets.
fn run_app(mut g: ResMut<G>, mut host: ResMut<HostState>, time: Res<Time>) {
    if !host.booted {
        return;
    }
    g.now_time64 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    // Test hook: WINFISH_FIXED_STEPS=<n> runs exactly n logic updates per rendered frame and
    // derives GetTickCount from the update count, so scripted runs are reproducible.
    let fixed = std::env::var("WINFISH_FIXED_STEPS").ok().and_then(|v| v.parse::<u32>().ok());
    if let Some(n) = fixed {
        g.tick_count = (host.fixed_updates as f64 * FRAME_TIME_MS) as u32;
        crate::game::boot::loading_thread_slice(&mut g);
        for _ in 0..n {
            crate::game::boot::update_frame(&mut g);
            host.fixed_updates += 1;
        }
        crate::game::boot::draw_frame(&mut g);
        return;
    }
    let started = *host.started.get_or_insert_with(std::time::Instant::now);
    g.tick_count = started.elapsed().as_millis() as u32;
    crate::game::boot::loading_thread_slice(&mut g);
    host.accum_ms += time.delta_secs_f64() * 1000.0;
    // At most 10 catch-up updates per frame, like the framework's frame skip limit.
    let mut steps = 0;
    while host.accum_ms >= FRAME_TIME_MS && steps < 10 {
        crate::game::boot::update_frame(&mut g);
        host.accum_ms -= FRAME_TIME_MS;
        steps += 1;
    }
    if steps == 10 {
        host.accum_ms = 0.0;
    }
    crate::game::boot::draw_frame(&mut g);
}

/// Writes the files the game saved this frame (`users.dat`, `user%d.dat`, ...) to the
/// install folder on the I/O task pool, creating folders as needed (the original's
/// `MkDir` + synchronous write). Systems never wait on the disk.
fn flush_saves(mut g: ResMut<G>) {
    if g.vfs.dirty.is_empty() && g.vfs.deleted.is_empty() {
        return;
    }
    let files: Vec<(String, Vec<u8>)> = g.vfs.dirty.drain().collect();
    let deleted: Vec<String> = std::mem::take(&mut g.vfs.deleted);
    // Test hook: WINFISH_NO_SAVE=1 keeps saves in memory only (scripted runs).
    if std::env::var_os("WINFISH_NO_SAVE").is_some() {
        return;
    }
    let root = g.vfs.root.clone();
    bevy::tasks::IoTaskPool::get()
        .spawn(async move {
            for rel in deleted {
                let _ = std::fs::remove_file(root.join(&rel));
            }
            for (rel, data) in files {
                let path = root.join(&rel);
                if let Some(dir) = path.parent() {
                    let _ = std::fs::create_dir_all(dir);
                }
                if let Err(e) = std::fs::write(&path, &data) {
                    bevy::log::error!("could not save {}: {e}", path.display());
                }
            }
        })
        .detach();
}

fn upload_screen(
    mut g: ResMut<G>,
    mut host: ResMut<HostState>,
    mut images: ResMut<Assets<Image>>,
    mut exit: MessageWriter<AppExit>,
) {
    let (Some(handle), true) = (host.screen.clone(), host.booted) else { return };
    let Some(bits) = crate::game::boot::screen_bits(&mut g) else { return };
    let data: Vec<u8> = bits.iter().flat_map(|p| [(p >> 16) as u8, (p >> 8) as u8, *p as u8, 255]).collect();
    // Test hook: WINFISH_SNAPSHOT=<frames>:<file.png>[|<frames>:<file.png>...] saves the
    // screen after those many frames and quits after the last one.
    host.frames += 1;
    // `SexyAppBase::Shutdown` ended with `PostQuitMessage`: once `mShutdown` is set, the
    // host ends the app (the saves it queued still go out through `flush_saves`).
    let app = g.globals.DAT_005eb6a4;
    if app != NULL && g.sab(app).field_0x339 {
        crate::game::boot::FUN_005024f0_end(&mut g);
        exit.write(AppExit::Success);
    }
    if let Ok(spec) = std::env::var("WINFISH_SNAPSHOT") {
        let shots: Vec<(u64, &str)> =
            spec.split('|').filter_map(|s| s.split_once(':').map(|(n, p)| (n.parse::<u64>().unwrap_or(0), p))).collect();
        let last = shots.iter().map(|s| s.0).max().unwrap_or(0);
        for (n, path) in shots {
            if host.frames == n {
                let _ = image::save_buffer(path, &data, SCREEN_W, SCREEN_H, image::ExtendedColorType::Rgba8);
                // With the HD screen up, it is saved too, as `<name>_hd.png`.
                if let Some((bits, w, h)) = crate::game::boot::screen_bits_hd(&g) {
                    let hd: Vec<u8> = bits.iter().flat_map(|p| [(p >> 16) as u8, (p >> 8) as u8, *p as u8, 255]).collect();
                    let hd_path = path.strip_suffix(".png").map_or_else(|| format!("{path}_hd.png"), |stem| format!("{stem}_hd.png"));
                    let _ = image::save_buffer(hd_path, &hd, w as u32, h as u32, image::ExtendedColorType::Rgba8);
                }
                if n == last {
                    exit.write(AppExit::Success);
                }
            }
        }
    }
    // The HD screen (main menu with upscaled art) replaces the picture while it is up.
    let (data, w, h) = match crate::game::boot::screen_bits_hd(&g) {
        Some((bits, w, h)) => (bits.iter().flat_map(|p| [(p >> 16) as u8, (p >> 8) as u8, *p as u8, 255]).collect(), w as u32, h as u32),
        None => (data, SCREEN_W, SCREEN_H),
    };
    // A size change gets a new image (the material is pointed at it in
    // `refresh_screen_material`).
    let resized = images.get(&handle).is_some_and(|img| img.width() != w || img.height() != h);
    if resized {
        let img = Image::new(
            Extent3d { width: w, height: h, depth_or_array_layers: 1 },
            TextureDimension::D2,
            data,
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::default(),
        );
        images.remove(&handle);
        host.screen = Some(images.add(img));
    } else if let Some(mut img) = images.get_mut(&handle) {
        img.data = Some(data);
    }
}
