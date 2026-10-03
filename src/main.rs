//! Runs the port: `cargo run [path to the Insaniquarium Deluxe install]`.
use bevy::prelude::*;
use std::path::PathBuf;

fn main() {
    let game_dir = std::env::args()
        .nth(1)
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
        .add_plugins(winfish_rs::host::WinFishPlugin { game_dir })
        .run();
}
