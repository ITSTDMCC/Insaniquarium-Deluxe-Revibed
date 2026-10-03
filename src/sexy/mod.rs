//! Ported pieces of the PopCap "SexyApp" framework that game behavior depends on.
pub mod app_host;
pub mod blit;
pub mod button_widget;
pub mod checkbox;
pub mod crt;
pub mod data_sync;
pub mod dialog;
pub mod dialog_button;
pub mod edit_widget;
pub mod g;
pub mod graphics;
pub mod image;
pub mod image_font;
pub mod image_lib;
pub mod mt_rand;
pub mod music;
pub mod object;
pub mod res;
pub mod resource_manager;
pub mod res_gen;
pub mod sexy_app_base;
pub mod list_widget;
pub mod scrollbar;
pub mod slider;
pub mod std_list;
pub mod types;
pub mod update_check_dialog;
pub mod vfs;
pub mod vtables_gen;
pub mod widget;
pub mod trivial;
pub mod widget_container;
pub mod widget_manager;

/// What every ported module imports.
pub mod prelude {
    pub use super::data_sync::DataSync;
    pub use super::g::G;
    pub use super::graphics::Graphics;
    pub use super::mt_rand::MTRand;
    pub use super::object::{GameObjectExt, GoSub, Node, Obj, WExt, WidgetObj};
    pub use super::types::*;
    pub use super::vtables_gen::VTable;
    pub use crate::vcall;
}

/// Called by vtable slots whose target has not been translated yet: fails loudly with the
/// original address instead of silently doing nothing.
#[track_caller]
pub fn pending(address: u32, name: &str) -> ! {
    panic!("not ported yet: {name} @ {address:08x} (see port/manifest.csv)")
}

/// Like [`pending`] for a not-yet-translated function whose effect the program can survive
/// without for now (loading a save that is absent, music): logs once instead of stopping.
/// Still listed as `pending` in port/manifest.csv.
pub fn pending_soft(address: u32, name: &str) {
    use std::sync::Mutex;
    static SEEN: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    let mut seen = SEEN.lock().unwrap();
    if !seen.contains(&address) {
        seen.push(address);
        bevy::log::warn!("not ported yet, skipped: {name} @ {address:08x}");
    }
}
