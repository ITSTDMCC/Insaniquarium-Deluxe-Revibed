//! Runs the port: `cargo run [path to the Insaniquarium Deluxe install] [--hd]`.
//! `--hd` (or `WINFISH_HD=1`) turns on the HD art made by `tools/upscale_art.py`; without it
//! the game uses its original art.
use bevy::prelude::*;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let hd = args.iter().any(|a| a.eq_ignore_ascii_case("--hd")) || std::env::var_os("WINFISH_HD").is_some_and(|v| v != "0");
    let game_dir = args
        .iter()
        .find(|a| !a.starts_with("--"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../Insaniquarium Deluxe"));
    // Software display (no GPU): asked for, or no hardware GPU found.
    let software = args.iter().any(|a| a.eq_ignore_ascii_case("--software"))
        || std::env::var_os("WINFISH_SOFTWARE").is_some_and(|v| v != "0")
        || !winfish_rs::host::soft_present::gpu_usable();
    let render = bevy::render::RenderPlugin {
        // No backends: the renderer is not started (`soft_present` shows the picture).
        render_creation: if software { bevy::render::settings::WgpuSettings { backends: None, ..default() }.into() } else { default() },
        ..default()
    };
    App::new()
        .add_plugins(DefaultPlugins.set(render).set(bevy::log::LogPlugin {
            // Without the renderer the engine logs two harmless startup complaints; hide them.
            filter: if software {
                format!("{},bevy_render::extract_resource=off,bevy_render::texture=error", bevy::log::DEFAULT_FILTER)
            } else {
                bevy::log::DEFAULT_FILTER.to_string()
            },
            ..default()
        }).set(WindowPlugin {
            primary_window: Some(Window {
                title: "Insaniquarium Deluxe (Rust port)".into(),
                resolution: (640u32, 480u32).into(),
                ..default()
            }),
            // The close button goes through the game's Shutdown (it saves the settings).
            close_when_requested: false,
            ..default()
        }))
        .add_plugins(winfish_rs::host::WinFishPlugin { game_dir, hd, software })
        .run();
}
