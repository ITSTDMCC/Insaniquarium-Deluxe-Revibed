//! Debug menu (experimental, a port addition): F1 shows an overlay with live information and
//! a few cheats and display switches, picked with the number keys while it is open. Those
//! keys (and F1) are not passed to the game while the menu is open.

use super::upscale::UpscaleMaterial;
use super::HostState;
use crate::sexy::prelude::*;
use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;

/// The aliens the menu can bring in: (kind, name). Kinds as `Board::AddAlien` takes them.
pub const ALIENS: [(i32, &str); 8] = [
    (1, "Sylvester (kind 1)"),
    (2, "Sylvester (kind 2)"),
    (3, "Balrog"),
    (4, "Gus"),
    (5, "Destructor"),
    (6, "Ulysses"),
    (7, "Psychosquid"),
    (8, "Bilaterus"),
];

/// The game speeds the menu cycles through (0 = paused).
pub const SPEEDS: [f64; 4] = [1.0, 2.0, 4.0, 0.0];

#[derive(Resource, Default)]
pub struct DebugMenu {
    pub open: bool,
    /// Index into `SPEEDS`.
    pub speed: usize,
    /// Index into `ALIENS`: the alien key 7 brings in.
    pub alien: usize,
    /// Holding the left button collects the money under the cursor (key 9).
    pub auto_collect: bool,
    /// Smoothed frame time (ms).
    frame_ms: f64,
    /// Logic updates per second, measured each second from the app's update counter.
    ups: f64,
    ups_window: (f64, i32),
    /// The last action's result, shown under the list.
    message: String,
    /// The game folder has HD art (checked once).
    hd_art: Option<bool>,
}

impl DebugMenu {
    pub fn speed(&self) -> f64 {
        SPEEDS[self.speed]
    }
}

#[derive(Component)]
pub struct DebugPanel;

#[derive(Component)]
pub struct DebugText;

/// Keys the menu takes from the game while it is open.
pub fn captures(menu: &DebugMenu, key: KeyCode) -> bool {
    key == KeyCode::F1 || (menu.open && matches!(key, KeyCode::Digit1 | KeyCode::Digit2 | KeyCode::Digit3 | KeyCode::Digit4 | KeyCode::Digit5 | KeyCode::Digit6 | KeyCode::Digit7 | KeyCode::Digit8 | KeyCode::Digit9))
}

pub fn setup(mut commands: Commands, mut menu: ResMut<DebugMenu>) {
    // Test hook: WINFISH_DEBUG_MENU=1 starts with the menu open.
    menu.open = std::env::var_os("WINFISH_DEBUG_MENU").is_some();
    commands
        .spawn((
            bevy::ui::Node { position_type: PositionType::Absolute, left: Val::Px(8.0), top: Val::Px(8.0), padding: UiRect::all(Val::Px(10.0)), ..default() },
            BackgroundColor(bevy::color::Color::srgba(0.0, 0.0, 0.0, 0.78)),
            Visibility::Hidden,
            GlobalZIndex(10),
            DebugPanel,
        ))
        .with_children(|p| {
            p.spawn((Text::new(""), TextFont { font_size: bevy::text::FontSize::Px(18.0), ..default() }, TextColor(bevy::color::Color::WHITE), DebugText));
        });
}

const FILTERS: [(&str, f32); 4] = [("bicubic", 2.0), ("nearest", 3.0), ("bilinear", 0.0), ("xBR", 1.0)];

fn tank(g: &mut G) -> Ptr {
    let app = g.globals.DAT_005eb6a4;
    if app == NULL {
        return NULL;
    }
    let board = g.wfa(app).offset_0x4;
    if board != NULL && g.is_live(board) { board } else { NULL }
}

fn profile(g: &mut G) -> Ptr {
    let app = g.globals.DAT_005eb6a4;
    if app == NULL { NULL } else { g.wfa(app).offset_0x18c }
}

/// F1 and the menu's number keys.
pub fn keys(
    keys: Res<ButtonInput<KeyCode>>,
    mut menu: ResMut<DebugMenu>,
    mut g: ResMut<G>,
    host: Res<HostState>,
    quads: Query<&MeshMaterial2d<UpscaleMaterial>>,
    mut materials: ResMut<Assets<UpscaleMaterial>>,
) {
    // Test hook: WINFISH_DEBUG_KEYS="<frame>:<key>;..." presses F1 or 1..6 on those frames.
    let scripted: Vec<KeyCode> = std::env::var("WINFISH_DEBUG_KEYS")
        .unwrap_or_default()
        .split(';')
        .filter_map(|s| s.split_once(':'))
        .filter(|(f, _)| f.parse::<u64>().ok() == Some(host.frames))
        .filter_map(|(_, k)| match k {
            "F1" => Some(KeyCode::F1),
            "1" => Some(KeyCode::Digit1),
            "2" => Some(KeyCode::Digit2),
            "3" => Some(KeyCode::Digit3),
            "4" => Some(KeyCode::Digit4),
            "5" => Some(KeyCode::Digit5),
            "6" => Some(KeyCode::Digit6),
            "7" => Some(KeyCode::Digit7),
            "8" => Some(KeyCode::Digit8),
            "9" => Some(KeyCode::Digit9),
            _ => None,
        })
        .collect();
    let pressed = |k: KeyCode| keys.just_pressed(k) || scripted.contains(&k);
    if pressed(KeyCode::F1) {
        menu.open = !menu.open;
        info!("debug menu {}", if menu.open { "opened" } else { "closed" });
    }
    let before = menu.message.clone();
    if !menu.open || !host.booted {
        return;
    }
    if pressed(KeyCode::Digit1) {
        let board = tank(&mut g);
        menu.message = if board != NULL {
            // `Board::AddMoney`, so the counter (and the virtual tank's shells) update as in play.
            crate::game::board::FUN_0053c1e0(&mut g, board, 1000);
            "Added $1,000.".into()
        } else {
            "Money can only be added in a tank.".into()
        };
    }
    if pressed(KeyCode::Digit2) {
        let p = profile(&mut g);
        menu.message = if p != NULL {
            crate::game::profile::FUN_00501200(&mut g, p, 1000);
            "Added 1,000 shells.".into()
        } else {
            "No player profile yet.".into()
        };
    }
    if pressed(KeyCode::Digit3) {
        let p = profile(&mut g);
        menu.message = if p != NULL {
            let pr = g.profile(p);
            for i in 0..24 {
                pr.field_0x0[i] = true;
            }
            pr.field_0x18 = 24;
            pr.field_0x1c = 5;
            pr.field_0x20 = 1;
            "Unlocked all adventure tanks (re-open the menu screen to see it).".into()
        } else {
            "No player profile yet.".into()
        };
    }
    if pressed(KeyCode::Digit4) {
        let has_art = *menu.hd_art.get_or_insert_with(|| g.vfs.root.join("hd").is_dir());
        menu.message = if has_art {
            g.hd.enabled = !g.hd.enabled;
            format!("HD art {}.", if g.hd.enabled { "on" } else { "off" })
        } else {
            "No HD art in the game folder (see README, \"HD art\").".into()
        };
    }
    if pressed(KeyCode::Digit5) {
        for m in &quads {
            if let Some(mut mat) = materials.get_mut(&m.0) {
                let i = FILTERS.iter().position(|f| f.1 == mat.mode.x).map_or(0, |i| (i + 1) % FILTERS.len());
                mat.mode.x = FILTERS[i].1;
                menu.message = format!("Scaling filter: {}.", FILTERS[i].0);
            }
        }
    }
    if pressed(KeyCode::Digit6) {
        menu.speed = (menu.speed + 1) % SPEEDS.len();
        menu.message = match menu.speed() {
            0.0 => "Game paused.".into(),
            s => format!("Game speed {s}x."),
        };
    }
    if pressed(KeyCode::Digit7) {
        let board = tank(&mut g);
        let (kind, name) = ALIENS[menu.alien];
        menu.message = if board != NULL {
            // As the game's own debug mode does (`Board::KeyChar`): the board's alien kind,
            // then `AddAlienAnywhere(kind, announce)`.
            g.board(board).field_0x230 = kind;
            crate::game::board_level::FUN_005475b0(&mut g, board, kind, true);
            format!("{name} is coming!")
        } else {
            "Aliens can only be brought into a tank.".into()
        };
    }
    if pressed(KeyCode::Digit8) {
        menu.alien = (menu.alien + 1) % ALIENS.len();
        menu.message = format!("Next alien: {}.", ALIENS[menu.alien].1);
    }
    if pressed(KeyCode::Digit9) {
        menu.auto_collect = !menu.auto_collect;
        menu.message = format!("Hold to collect money: {}.", if menu.auto_collect { "on" } else { "off" });
    }
    if menu.message != before {
        info!("debug menu: {}", menu.message);
    }
    if std::env::var_os("WINFISH_DEBUG_MENU").is_some() && host.frames % 300 == 0 {
        info!("debug menu: {:.1} game updates/s", menu.ups);
    }
}

/// Refreshes the overlay while it is open.
#[allow(clippy::too_many_arguments)]
pub fn overlay(
    mut menu: ResMut<DebugMenu>,
    mut g: ResMut<G>,
    time: Res<Time>,
    windows: Query<&Window>,
    quads: Query<&MeshMaterial2d<UpscaleMaterial>>,
    materials: Res<Assets<UpscaleMaterial>>,
    mut panel: Query<&mut Visibility, With<DebugPanel>>,
    mut text: Query<&mut Text, With<DebugText>>,
) {
    let dt = time.delta_secs_f64() * 1000.0;
    menu.frame_ms = if menu.frame_ms == 0.0 { dt } else { menu.frame_ms * 0.95 + dt * 0.05 };
    // Logic updates per second (the app's update counter, +0x47c).
    let app = g.globals.DAT_005eb6a4;
    if app != NULL {
        let count = g.sab(app).field_0x47c;
        let (elapsed, start) = menu.ups_window;
        if elapsed == 0.0 && start == 0 {
            menu.ups_window = (0.0, count);
        } else {
            menu.ups_window.0 += dt;
            if menu.ups_window.0 >= 1000.0 {
                menu.ups = (count - start) as f64 * 1000.0 / menu.ups_window.0;
                menu.ups_window = (0.0, count);
            }
        }
    }
    let want = if menu.open { Visibility::Visible } else { Visibility::Hidden };
    for mut v in &mut panel {
        if *v != want {
            *v = want;
        }
    }
    if !menu.open {
        return;
    }
    let window = windows.single().map(|w| format!("{}x{}", w.physical_width(), w.physical_height())).unwrap_or_default();
    let filter = quads
        .iter()
        .next()
        .and_then(|m| materials.get(&m.0))
        .and_then(|mat| FILTERS.iter().find(|f| f.1 == mat.mode.x))
        .map_or("?", |f| f.0);
    let has_art = *menu.hd_art.get_or_insert_with(|| g.vfs.root.join("hd").is_dir());
    let hd = match (has_art, g.hd.enabled) {
        (false, _) => "no HD art",
        (true, true) => "on",
        (true, false) => "off",
    };
    let board = tank(&mut g);
    let money = if board != NULL { format!("${}", g.board(board).field_0x364) } else { "-".into() };
    let p = profile(&mut g);
    let shells = if p != NULL { g.profile(p).field_0x48.to_string() } else { "-".into() };
    let speed = match menu.speed() {
        0.0 => "paused".to_string(),
        s => format!("{s}x"),
    };
    let fps = if menu.frame_ms > 0.0 { 1000.0 / menu.frame_ms } else { 0.0 };
    let s = format!(
        "DEBUG MENU (experimental)   F1 closes\n\
         \n\
         {fps:.0} fps ({:.1} ms)   {:.0} game updates/s   window {window}\n\
         HD art: {hd}   filter: {filter}   speed: {speed}\n\
         money: {money}   shells: {shells}\n\
         \n\
         1  Add $1,000 (in a tank)\n\
         2  Add 1,000 shells\n\
         3  Unlock all adventure tanks (saved to the profile)\n\
         4  HD art on / off\n\
         5  Next scaling filter\n\
         6  Game speed: 1x, 2x, 4x, paused\n\
         7  Bring in an alien: {}\n\
         8  Choose the alien\n\
         9  Hold the left button to collect money: {}\n\
         \n\
         {}",
        menu.frame_ms,
        menu.ups,
        ALIENS[menu.alien].1,
        if menu.auto_collect { "on" } else { "off" },
        menu.message
    );
    for mut t in &mut text {
        if t.0 != s {
            t.0 = s.clone();
        }
    }
}

/// With "hold to collect" on (key 9): while the left button is held in a running tank, every
/// coin, pearl or other item under the cursor gets the mouse press a click would give it
/// (`Coin::MouseDown`), so it is collected as if clicked. Test hook:
/// `WINFISH_DEBUG_HOLD=<y>` holds the button with the cursor sweeping across the tank at y.
pub fn auto_collect(menu: Res<DebugMenu>, buttons: Res<ButtonInput<MouseButton>>, host: Res<HostState>, mut g: ResMut<G>) {
    let sweep = std::env::var("WINFISH_DEBUG_HOLD").ok().and_then(|v| v.parse::<i32>().ok());
    let held = buttons.pressed(MouseButton::Left) || sweep.is_some();
    if !menu.auto_collect || !held || !host.booted {
        return;
    }
    let cursor = match sweep {
        Some(y) => Some(((host.frames * 7 % 640) as i32, y)),
        None => host.last_mouse,
    };
    let Some((mx, my)) = cursor else { return };
    let board = tank(&mut g);
    if board == NULL || g.wc(board).offset_0x10 == NULL || g.board(board).field_0x8 {
        return;
    }
    let app = g.globals.DAT_005eb6a4;
    if !g.sab(app).offset_0x32c.is_empty() || !g.sab(app).offset_0x320.is_empty() {
        return;
    }
    let coins = g.board(board).offset_0xc[crate::game::board_level::vec_index(0xa8)].clone();
    for c in coins {
        if !g.is_live(c) || g.coin(c).offset_0x44 {
            continue;
        }
        let (x, y, w, h) = (g.wc(c).offset_0x2c, g.wc(c).offset_0x30, g.wc(c).offset_0x34, g.wc(c).offset_0x38);
        if mx >= x && mx < x + w && my >= y && my < y + h {
            crate::game::coin::vfunction55(&mut g, c, mx - x, my - y, 1);
        }
    }
}
