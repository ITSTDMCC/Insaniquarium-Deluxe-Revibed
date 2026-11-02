//! (Port addition) Software display: shows the game's picture with no GPU at all.
//!
//! The game already paints its whole screen on the CPU; normally the host hands that picture
//! to the GPU, which scales it to the window. In software mode the engine's renderer is not
//! started (it needs Vulkan, DX12 or OpenGL, and on a software-only "GPU" it crawls), and
//! the picture is scaled to the window here and copied to it with the operating system's
//! plain 2D drawing (`softbuffer`). Chosen with `--software`, `WINFISH_SOFTWARE=1`, or
//! automatically when no hardware GPU is found ([`gpu_usable`]).
//!
//! The picture is scaled with nearest-neighbour sampling, keeping its shape (black bars on
//! the long sides), and only redrawn when it, the window or the cursor changed. Without the
//! GPU there is no vsync, so frames are paced to 60 a second by sleeping.
//! The F1 debug overlay (engine UI) is not shown in this mode.

use crate::host::{SCREEN_H, SCREEN_W};
use crate::sexy::prelude::*;
use bevy::prelude::*;
use crate::sexy::prelude::Color;
use bevy::window::{PrimaryWindow, RawHandleWrapper, ThreadLockedRawWindowHandleWrapper};
use std::num::NonZeroU32;
use std::time::{Duration, Instant};

/// Software display is on (the renderer was not started).
#[derive(Resource, Clone, Copy)]
pub struct SoftwareDisplay;

/// The game's own cursor image to draw over the picture (software display only): the image
/// and the mouse position in window pixels.
#[derive(Resource, Default)]
pub struct SoftCursor(pub Option<(Ptr, Vec2)>);

struct Presenter {
    surface: softbuffer::Surface<ThreadLockedRawWindowHandleWrapper, ThreadLockedRawWindowHandleWrapper>,
    /// Window size the buffer has.
    size: (u32, u32),
    /// What was last shown: picture serial, window size, cursor.
    shown: Option<(u64, (u32, u32), Option<(Ptr, IVec2)>)>,
    /// Destination column -> source column, and row -> row, for the current sizes.
    xs: Vec<u32>,
    ys: Vec<u32>,
    map_for: ((u32, u32), (u32, u32)),
}

#[derive(Default)]
struct SoftState {
    presenter: Option<Presenter>,
    failed: bool,
    next_frame: Option<Instant>,
    /// Bumped whenever the picture to show changed.
    serial: u64,
    last_bits: Vec<u32>,
}

pub fn plugin(app: &mut App) {
    app.insert_resource(SoftwareDisplay)
        .init_resource::<SoftCursor>()
        .insert_non_send(SoftState::default())
        .add_systems(Last, (present, pace).chain());
}

/// A hardware GPU that the engine can render with exists (any backend but a CPU-only one).
/// `WINFISH_SOFTWARE=1` / `--software` skip this; it runs before the app (and its log) exists.
pub fn gpu_usable() -> bool {
    use wgpu::{DeviceType, Instance, InstanceDescriptor, PowerPreference, RequestAdapterOptions};
    let instance = Instance::new(InstanceDescriptor::new_without_display_handle_from_env());
    let adapter = bevy::tasks::block_on(instance.request_adapter(&RequestAdapterOptions {
        power_preference: PowerPreference::HighPerformance,
        force_fallback_adapter: false,
        compatible_surface: None,
    }));
    match adapter {
        Ok(a) => {
            let info = a.get_info();
            eprintln!("GPU: {} ({:?}, {:?})", info.name, info.device_type, info.backend);
            info.device_type != DeviceType::Cpu
        }
        Err(e) => {
            eprintln!("no GPU found ({e}): using the software display");
            false
        }
    }
}

fn presenter(handle: &RawHandleWrapper) -> Result<Presenter, softbuffer::SoftBufferError> {
    // SAFETY: the presenter is a non-send resource, used only on the main thread, where the
    // window lives; the handles keep the window alive.
    let (display, window) = unsafe { (handle.get_handle(), handle.get_handle()) };
    let context = softbuffer::Context::new(display)?;
    let surface = softbuffer::Surface::new(&context, window)?;
    Ok(Presenter { surface, size: (0, 0), shown: None, xs: Vec::new(), ys: Vec::new(), map_for: ((0, 0), (0, 0)) })
}

fn present(mut state: NonSendMut<SoftState>, mut g: ResMut<G>, cursor: Res<SoftCursor>, windows: Query<(&Window, &RawHandleWrapper), With<PrimaryWindow>>) {
    let Ok((window, handle)) = windows.single() else { return };
    let state = &mut *state;
    if state.failed {
        return;
    }
    if state.presenter.is_none() {
        match presenter(handle) {
            Ok(p) => state.presenter = Some(p),
            Err(e) => {
                error!("software display: cannot draw to the window: {e}");
                state.failed = true;
                return;
            }
        }
    }
    // The picture: the HD screen when it is up, else the 640x480 one.
    let (bits, sw, sh): (&[u32], u32, u32) = if g.hd.active {
        (&g.hd.bits, g.hd.width as u32, g.hd.height as u32)
    } else {
        let wm = crate::game::boot::widget_manager(&mut g);
        if wm == NULL {
            (&[], 0, 0)
        } else {
            let screen = crate::sexy::widget_manager::wm(&mut g, wm).offset_0xc;
            let im = g.image(screen);
            (&im.mBits, im.offset_0x20 as u32, im.offset_0x24 as u32)
        }
    };
    if sw == 0 || sh == 0 || bits.len() != (sw * sh) as usize {
        return;
    }
    if state.last_bits != bits {
        state.last_bits.clear();
        state.last_bits.extend_from_slice(bits);
        state.serial += 1;
    }
    let p = state.presenter.as_mut().unwrap();
    let (ww, wh) = (window.physical_width(), window.physical_height());
    let (Some(nw), Some(nh)) = (NonZeroU32::new(ww), NonZeroU32::new(wh)) else { return };
    let cur = cursor.0.map(|(img, pos)| (img, pos.as_ivec2()));
    let key = (state.serial, (ww, wh), cur);
    if p.shown == Some(key) {
        return;
    }
    if p.size != (ww, wh) {
        if let Err(e) = p.surface.resize(nw, nh) {
            error!("software display: resize failed: {e}");
            return;
        }
        p.size = (ww, wh);
    }
    // The picture's place in the window: as large as fits, centred.
    let s = (ww as f32 / sw as f32).min(wh as f32 / sh as f32);
    let (dw, dh) = (((sw as f32 * s) as u32).clamp(1, ww), ((sh as f32 * s) as u32).clamp(1, wh));
    let (ox, oy) = ((ww - dw) / 2, (wh - dh) / 2);
    if p.map_for != ((sw, sh), (dw, dh)) {
        p.xs = (0..dw).map(|x| ((x as u64 * sw as u64) / dw as u64) as u32).collect();
        p.ys = (0..dh).map(|y| ((y as u64 * sh as u64) / dh as u64) as u32).collect();
        p.map_for = ((sw, sh), (dw, dh));
    }
    let Ok(mut buf) = p.surface.buffer_mut() else { return };
    let src = &state.last_bits;
    let (ww_us, ox_us) = (ww as usize, ox as usize);
    for y in 0..wh as usize {
        if y < oy as usize || y >= (oy + dh) as usize {
            buf[y * ww_us..(y + 1) * ww_us].fill(0);
            continue;
        }
        let dy = y - oy as usize;
        // A row that samples the same source row as the one above is a straight copy.
        if dy > 0 && p.ys[dy] == p.ys[dy - 1] {
            let (above, this) = buf.split_at_mut(y * ww_us);
            this[..ww_us].copy_from_slice(&above[(y - 1) * ww_us..]);
            continue;
        }
        let row = &mut buf[y * ww_us..(y + 1) * ww_us];
        row[..ox_us].fill(0);
        row[ox_us + dw as usize..].fill(0);
        let srow = &src[p.ys[dy] as usize * sw as usize..][..sw as usize];
        for (d, &sx) in row[ox_us..ox_us + dw as usize].iter_mut().zip(&p.xs) {
            *d = srow[sx as usize] & 0x00ff_ffff;
        }
    }
    // The game's own cursor, centred on the mouse, scaled like the picture (640x480 units).
    if let Some((img, pos)) = cursor.0
        && g.is_live(img)
    {
        let im = g.image(img);
        let (cw, ch) = (im.offset_0x20, im.offset_0x24);
        let k = (ww as f32 / SCREEN_W as f32).min(wh as f32 / SCREEN_H as f32);
        let (tw, th) = ((cw as f32 * k) as i32, (ch as f32 * k) as i32);
        let (x0, y0) = (pos.x as i32 - tw / 2, pos.y as i32 - th / 2);
        for ty in 0..th {
            let y = y0 + ty;
            if y < 0 || y >= wh as i32 {
                continue;
            }
            let sy = (ty * ch / th.max(1)).min(ch - 1);
            for tx in 0..tw {
                let x = x0 + tx;
                if x < 0 || x >= ww as i32 {
                    continue;
                }
                let sx = (tx * cw / tw.max(1)).min(cw - 1);
                let s = im.mBits[(sy * cw + sx) as usize];
                let d = &mut buf[y as usize * ww_us + x as usize];
                *d = crate::sexy::blit::normal_px(*d | 0xff00_0000, s, Color::WHITE) & 0x00ff_ffff;
            }
        }
    }
    if let Err(e) = buf.present() {
        error!("software display: present failed: {e}");
        return;
    }
    p.shown = Some(key);
}

/// No vsync without the GPU: sleeps out the rest of a 60 Hz frame.
fn pace(mut state: NonSendMut<SoftState>) {
    const FRAME: Duration = Duration::from_nanos(16_666_667);
    let now = Instant::now();
    let next = state.next_frame.unwrap_or(now + FRAME);
    if next > now {
        std::thread::sleep(next - now);
        state.next_frame = Some(next + FRAME);
    } else {
        // Behind (a slow frame): start counting again from now.
        state.next_frame = Some(now + FRAME);
    }
}
