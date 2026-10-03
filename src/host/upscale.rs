//! The material that draws the game's 640x480 screen scaled to the window (`upscale.wgsl`).
//! By default the enlarged picture is filtered with Catmull-Rom bicubic, which keeps every
//! original pixel exact; `WINFISH_FILTER=bilinear|xbr|nearest` picks another filter.

use bevy::asset::{AssetPath, embedded_asset, embedded_path};
use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;
use bevy::sprite_render::{Material2d, Material2dPlugin};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct UpscaleMaterial {
    /// x: filter (0 bilinear, 1 xBR, 2 Catmull-Rom, 3 nearest).
    #[uniform(0)]
    pub mode: Vec4,
    #[texture(1)]
    #[sampler(2)]
    pub screen: Handle<Image>,
}

impl UpscaleMaterial {
    pub fn new(screen: Handle<Image>) -> Self {
        let filter = std::env::var("WINFISH_FILTER").unwrap_or_default().to_ascii_lowercase();
        let mode = match filter.as_str() {
            "bilinear" => 0.0,
            "xbr" => 1.0,
            "nearest" => 3.0,
            _ => 2.0,
        };
        UpscaleMaterial { mode: Vec4::new(mode, 0.0, 0.0, 0.0), screen }
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
