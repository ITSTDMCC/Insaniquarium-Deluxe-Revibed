//! (Port addition) Sends the screen's pixels straight into its GPU texture.
//!
//! Changing an `Image` asset makes the engine take its whole pixel buffer to the render world
//! and write it into the texture: a fresh 20 MB buffer per HD update (its memory faulted in
//! anew each time) or, if the asset keeps its pixels, a 20 MB clone. Here the texture asset
//! is left alone after it is created; the host copies the new picture into a reused buffer and
//! the render world writes that buffer into the texture, then hands it back for reuse.

use bevy::prelude::*;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_resource::{Extent3d, TexelCopyBufferLayout};
use bevy::render::renderer::RenderQueue;
use bevy::render::texture::GpuImage;
use bevy::render::{Render, RenderApp, RenderSystems};
use std::sync::{Arc, Mutex};

struct Frame {
    image: AssetId<Image>,
    width: u32,
    height: u32,
    bytes: Vec<u8>,
}

#[derive(Default)]
struct Shared {
    /// The newest picture not written yet.
    pending: Option<Frame>,
    /// Buffers already written, for reuse.
    spare: Vec<Vec<u8>>,
}

#[derive(Resource, Clone, Default)]
pub struct ScreenUpload(Arc<Mutex<Shared>>);

impl ScreenUpload {
    /// Queues `bits` (0xAARRGGBB, `width * height`, ORed with `or`) for the BGRA texture of
    /// `image`. A picture queued before and not written yet is replaced.
    pub fn send(&self, image: AssetId<Image>, width: u32, height: u32, bits: &[u32], or: u32) {
        let mut buf = {
            let mut s = self.0.lock().unwrap();
            s.pending.take().map(|f| f.bytes).or_else(|| s.spare.pop()).unwrap_or_default()
        };
        buf.clear();
        buf.reserve(bits.len() * 4);
        if or == 0 && cfg!(target_endian = "little") {
            // 0xAARRGGBB in memory on a little-endian machine is B, G, R, A: as is.
            // SAFETY: any u32 slice is a valid byte slice four times as long.
            let bytes = unsafe { std::slice::from_raw_parts(bits.as_ptr() as *const u8, bits.len() * 4) };
            buf.extend_from_slice(bytes);
        } else {
            for px in bits {
                buf.extend_from_slice(&(px | or).to_le_bytes());
            }
        }
        self.0.lock().unwrap().pending = Some(Frame { image, width, height, bytes: buf });
    }
}

fn write_screen(upload: Res<ScreenUpload>, images: Res<RenderAssets<GpuImage>>, queue: Res<RenderQueue>) {
    let mut s = upload.0.lock().unwrap();
    let Some(frame) = s.pending.take() else { return };
    let Some(gpu) = images.get(frame.image) else {
        // The texture is not on the GPU yet: try again next frame.
        s.pending = Some(frame);
        return;
    };
    let size = gpu.texture_descriptor.size;
    if size.width == frame.width && size.height == frame.height && frame.bytes.len() == (frame.width * frame.height * 4) as usize {
        queue.write_texture(
            gpu.texture.as_image_copy(),
            &frame.bytes,
            TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(frame.width * 4), rows_per_image: Some(frame.height) },
            Extent3d { width: frame.width, height: frame.height, depth_or_array_layers: 1 },
        );
    }
    if s.spare.len() < 2 {
        s.spare.push(frame.bytes);
    }
}

pub fn plugin(app: &mut App) {
    let upload = ScreenUpload::default();
    app.insert_resource(upload.clone());
    if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
        render_app.insert_resource(upload).add_systems(Render, write_screen.in_set(RenderSystems::Prepare));
    }
}
