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
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Insaniquarium Deluxe (Rust port)".into(),
                resolution: (640u32, 480u32).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(winfish_rs::host::WinFishPlugin { game_dir, hd })
        .run();
}
