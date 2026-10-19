//! The mouse cursor: what the original's `SexyAppBase::EnforceCursor()` and
//! `DDInterface`'s software cursor did. With Options' Custom Cursors on, the game's own
//! cursor image for `mCursorNum` (pointer, hand, dragging, text: `SetCursorImage`) is drawn
//! over the picture, centered on the mouse like `DDInterface` drew it, and scaled with the
//! picture; the system cursor is hidden inside the window. With it off, or for a cursor
//! without an image, the matching system cursor shows.

use super::{HostState, SCREEN_H, SCREEN_W};
use crate::sexy::prelude::*;
use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::window::{CursorIcon, CursorOptions, SystemCursorIcon};

/// The sprite the custom cursor is drawn with.
#[derive(Component)]
pub struct CursorSprite;

/// The game image last turned into the sprite's texture.
#[derive(Resource, Default)]
pub struct CursorCache {
    image: Ptr,
    handle: Option<Handle<Image>>,
}

pub fn setup(mut commands: Commands) {
    commands.init_resource::<CursorCache>();
    commands.spawn((Sprite::default(), Transform::from_xyz(0.0, 0.0, 10.0), Visibility::Hidden, CursorSprite));
}

/// `mCursorNum` (`CURSOR_POINTER` 0, `CURSOR_HAND` 1, `CURSOR_DRAGGING` 2, `CURSOR_TEXT` 3,
/// ..., `CURSOR_NONE` 11) as a system cursor; `None` hides it.
fn system_cursor(num: i32) -> Option<SystemCursorIcon> {
    Some(match num {
        1 => SystemCursorIcon::Pointer,
        2 => SystemCursorIcon::Grabbing,
        3 => SystemCursorIcon::Text,
        4 => SystemCursorIcon::NotAllowed,
        5 => SystemCursorIcon::Move,
        10 => SystemCursorIcon::Wait,
        11 => return None,
        _ => SystemCursorIcon::Default,
    })
}

pub fn update(
    mut commands: Commands,
    mut g: ResMut<G>,
    host: Res<HostState>,
    mut cache: ResMut<CursorCache>,
    mut images: ResMut<Assets<Image>>,
    windows: Query<(Entity, &Window, Option<&CursorIcon>, Option<&CursorOptions>)>,
    mut sprites: Query<(&mut Sprite, &mut Transform, &mut Visibility), With<CursorSprite>>,
) {
    let Ok((entity, window, icon, options)) = windows.single() else { return };
    let Ok((mut sprite, mut transform, mut visibility)) = sprites.single_mut() else { return };
    let app = g.globals.DAT_005eb6a4;
    let (num, custom) = if host.booted && app != NULL {
        let num = g.sab(app).field_0x4a8;
        let img = if g.sab(app).field_0x4f8 && (0..13).contains(&num) { g.sab(app).field_0x3ac[num as usize] } else { NULL };
        (num, img)
    } else {
        (0, NULL)
    };
    let custom = if custom != NULL && !g.image(custom).mBits.is_empty() { custom } else { NULL };
    let pos = window.cursor_position();

    // The game's image, centered on the mouse and scaled like the picture (`fit_screen`).
    let show = custom != NULL && pos.is_some();
    if let (true, Some(p)) = (show, pos) {
        if cache.image != custom || cache.handle.is_none() {
            let im = g.image(custom);
            let (w, h) = (im.offset_0x20.max(1) as u32, im.offset_0x24.max(1) as u32);
            let data: Vec<u8> = im.mBits.iter().flat_map(|p| [(p >> 16) as u8, (p >> 8) as u8, *p as u8, (p >> 24) as u8]).collect();
            let mut tex = Image::new_fill(Extent3d { width: w, height: h, depth_or_array_layers: 1 }, TextureDimension::D2, &[0, 0, 0, 0], TextureFormat::Rgba8UnormSrgb, RenderAssetUsages::default());
            tex.data = Some(data);
            if let Some(old) = cache.handle.take() {
                images.remove(&old);
            }
            let handle = images.add(tex);
            sprite.image = handle.clone();
            cache.handle = Some(handle);
            cache.image = custom;
        }
        let s = (window.width() / SCREEN_W as f32).min(window.height() / SCREEN_H as f32);
        let im = g.image(custom);
        sprite.custom_size = Some(Vec2::new(im.offset_0x20 as f32 * s, im.offset_0x24 as f32 * s));
        let at = Vec3::new(p.x - window.width() / 2.0, window.height() / 2.0 - p.y, 10.0);
        if transform.translation != at {
            transform.translation = at;
        }
    }
    let vis = if show { Visibility::Visible } else { Visibility::Hidden };
    if *visibility != vis {
        *visibility = vis;
    }

    // The system cursor: hidden under the game's image (or for `CURSOR_NONE`), else the
    // one matching the cursor number.
    let sys = system_cursor(num);
    let want_visible = !show && sys.is_some();
    if options.is_none_or(|o| o.visible != want_visible) {
        let mut o = options.cloned().unwrap_or_default();
        o.visible = want_visible;
        commands.entity(entity).insert(o);
    }
    if let Some(sys) = sys {
        let want = CursorIcon::System(sys);
        if icon != Some(&want) {
            commands.entity(entity).insert(want);
        }
    }
}
