//! `ImageLib::GetImage`: finds an image by base name, tries each supported extension, and
//! merges a separate alpha image (`_name` or `name_`), exactly as the original does. The
//! format decoders themselves (statically linked libpng/libjpeg/gif code, listed as
//! `replaced` in port/manifest.csv) are the `image` crate here.

use crate::sexy::vfs::Vfs;

/// `ImageLib::Image`: +4 `mWidth`, +8 `mHeight`, +0xc `mBits` (0xAARRGGBB per pixel).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Image {
    pub mWidth: i32,
    pub mHeight: i32,
    pub mBits: Vec<u32>,
}

/// Decoder stand-in for the per-format loaders (`FUN_0049bfd0` tga, `FUN_0049f800` jpg,
/// `FUN_0049be40` png, `FUN_0049c160` gif, `FUN_0049fb10` jpeg2000).
fn decode(vfs: &Vfs, path: &str, format: image::ImageFormat) -> Option<Image> {
    let bytes = vfs.read(path)?;
    let img = image::load_from_memory_with_format(bytes, format).ok()?.to_rgba8();
    let (w, h) = img.dimensions();
    // The GIF loader writes the transparent colour index as 0 (black, alpha 0), not as its
    // palette colour; an alpha image (`_name.gif`) reads its alpha from those low bits.
    let gif = format == image::ImageFormat::Gif;
    let bits = img
        .pixels()
        .map(|p| if gif && p[3] == 0 { 0 } else { ((p[3] as u32) << 24) | ((p[0] as u32) << 16) | ((p[1] as u32) << 8) | p[2] as u32 })
        .collect();
    Some(Image { mWidth: w as i32, mHeight: h as i32, mBits: bits })
}

/// `DAT_005e7078`: the color given to pixels of an alpha-only image (white).
pub const DAT_005e7078: u32 = 0x00ff_ffff;

/// port: 0049fed0 FUN_0049fed0
/// `ImageLib::GetImage(std::string theFileName, bool lookForAlphaImage)`.
pub fn FUN_0049fed0(vfs: &Vfs, param_1: &str, param_2: bool) -> Option<Image> {
    let dot = param_1.rfind('.').map_or(-1, |i| i as i64);
    let back = param_1.rfind('\\').map_or(-1, |i| i as i64);
    let fwd = param_1.rfind('/').map_or(-1, |i| i as i64);
    let sep = if fwd < back { '\\' } else { '/' };
    let slash = param_1.rfind(sep).map_or(-1, |i| i as i64);
    let (base, ext) = if slash < dot {
        (&param_1[..dot as usize], &param_1[dot as usize..])
    } else {
        (param_1, "")
    };
    let try_ext = |want: &str| ext.eq_ignore_ascii_case(want) || ext.is_empty();
    let mut img = None;
    if try_ext(".tga") {
        img = decode(vfs, &format!("{base}.tga"), image::ImageFormat::Tga);
    }
    if img.is_none() && try_ext(".jpg") {
        img = decode(vfs, &format!("{base}.jpg"), image::ImageFormat::Jpeg);
    }
    if img.is_none() && try_ext(".png") {
        img = decode(vfs, &format!("{base}.png"), image::ImageFormat::Png);
    }
    if img.is_none() && try_ext(".gif") {
        img = decode(vfs, &format!("{base}.gif"), image::ImageFormat::Gif);
    }
    // .j2k / .jp2 (JPEG 2000) are not used by any file of the game; no decoder is linked here.
    if param_2 {
        let file = &param_1[(slash + 1) as usize..];
        let dir = &param_1[..(slash + 1) as usize];
        let mut alpha = FUN_0049fed0(vfs, &format!("{dir}_{file}"), false);
        if alpha.is_none() {
            alpha = FUN_0049fed0(vfs, &format!("{base}_"), false);
        }
        if let Some(a) = alpha {
            match &mut img {
                None => {
                    let mut a = a;
                    for p in a.mBits.iter_mut() {
                        *p = (*p << 0x18) | DAT_005e7078;
                    }
                    img = Some(a);
                }
                Some(c) => {
                    if c.mWidth == a.mWidth && c.mHeight == a.mHeight {
                        for (p, q) in c.mBits.iter_mut().zip(a.mBits.iter()) {
                            *p = (*p & 0x00ff_ffff) | ((q & 0xff) << 24);
                        }
                    }
                }
            }
        }
    }
    img
}
