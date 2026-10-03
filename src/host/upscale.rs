//! The material that draws the game's 640x480 screen scaled to the window, smoothing the
//! enlarged picture with an edge-directed upscaler (`upscale.wgsl`).
//! `WINFISH_FILTER=bilinear` draws it with plain bilinear filtering instead.

use bevy::asset::{AssetPath, embedded_asset, embedded_path};
use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;
use bevy::sprite_render::{Material2d, Material2dPlugin};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct UpscaleMaterial {
    /// x: filter (0 bilinear, 1 xBR).
    #[uniform(0)]
    pub mode: Vec4,
    #[texture(1)]
    #[sampler(2)]
    pub screen: Handle<Image>,
}

impl UpscaleMaterial {
    pub fn new(screen: Handle<Image>) -> Self {
        let bilinear = std::env::var("WINFISH_FILTER").is_ok_and(|v| v.eq_ignore_ascii_case("bilinear"));
        UpscaleMaterial { mode: Vec4::new(if bilinear { 0.0 } else { 1.0 }, 0.0, 0.0, 0.0), screen }
    }
}

impl Material2d for UpscaleMaterial {
    fn fragment_shader() -> ShaderRef {
        ShaderRef::Path(AssetPath::from_path_buf(embedded_path!("upscale.wgsl")).with_source("embedded"))
    }
}

pub fn plugin(app: &mut App) {
    embedded_asset!(app, "upscale.wgsl");
    app.add_plugins(Material2dPlugin::<UpscaleMaterial>::default());
}
