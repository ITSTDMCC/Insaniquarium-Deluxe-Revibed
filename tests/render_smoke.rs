//! Loads real game images through the ported `ImageLib::GetImage` and draws them through the
//! ported `Graphics` + blitter path. Skipped when the game install is not next to the crate.
//! Set `WINFISH_SMOKE_PNG=<path>` to also write the result as a PNG for inspection.

use std::path::Path;
use winfish_rs::sexy::graphics::{FUN_00455890, FUN_004558e0, FUN_00455d20, FUN_00456950, Graphics};
use winfish_rs::sexy::image::Image;
use winfish_rs::sexy::prelude::*;
use winfish_rs::sexy::vfs::Vfs;

fn game_dir() -> Option<std::path::PathBuf> {
    let p = Path::new(env!("CARGO_MANIFEST_DIR")).join("../Insaniquarium Deluxe");
    p.join("images").is_dir().then_some(p)
}

#[test]
fn title_and_food_render() {
    let Some(dir) = game_dir() else { return };
    let vfs = Vfs::load_dir(&dir).unwrap();
    let title = winfish_rs::sexy::image_lib::FUN_0049fed0(&vfs, "images/titlescreen", true).expect("titlescreen");
    assert_eq!((title.mWidth, title.mHeight), (640, 480));
    let food = winfish_rs::sexy::image_lib::FUN_0049fed0(&vfs, "images/food", true).expect("food");
    assert!(food.mBits.iter().any(|p| p >> 24 == 0), "food has transparent pixels from its alpha image");

    let mut g = G::default();
    let mut add = |g: &mut G, li: winfish_rs::sexy::image_lib::Image, rows: i32, cols: i32| {
        let mut img = Image::new();
        img.offset_0x20 = li.mWidth;
        img.offset_0x24 = li.mHeight;
        img.offset_0x28 = rows;
        img.offset_0x2c = cols;
        img.mBits = li.mBits;
        g.alloc(Obj { vt: None, node: Node::Image(Box::new(img)) })
    };
    let t = add(&mut g, title, 1, 1);
    let f = add(&mut g, food, 5, 10);
    let mut screen = Image::new();
    screen.offset_0x20 = 640;
    screen.offset_0x24 = 480;
    screen.mBits = vec![0xff00_0000; 640 * 480];
    let s = g.alloc(Obj { vt: None, node: Node::Image(Box::new(screen)) });

    let out = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let mut gfx = Graphics::new(s, 640, 480, out.clone());
    FUN_00455d20(&mut gfx, &mut g, t, 0, 0);
    FUN_00456950(&mut gfx, &mut g, f, 300, 200, 3);
    FUN_004558e0(&mut gfx, true);
    FUN_00455890(&mut gfx, CRect(255, 255, 255, 128));
    FUN_00456950(&mut gfx, &mut g, f, 340, 200, 3);
    let cmds = out.lock().unwrap().clone();
    assert_eq!(cmds.len(), 3);

    let mut bits = std::mem::take(&mut g.image(s).mBits);
    {
        let mut target = winfish_rs::sexy::blit::Target { bits: &mut bits, width: 640, height: 480 };
        for c in &cmds {
            target.apply(&g, c);
        }
    }
    if let Ok(path) = std::env::var("WINFISH_SMOKE_PNG") {
        let rgba: Vec<u8> = bits.iter().flat_map(|p| [(p >> 16) as u8, (p >> 8) as u8, *p as u8, 255]).collect();
        image::save_buffer(path, &rgba, 640, 480, image::ExtendedColorType::Rgba8).unwrap();
    }
}
