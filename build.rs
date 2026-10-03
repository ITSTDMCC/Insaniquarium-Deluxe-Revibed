//! Links the vendored libopenmpt (vendor/libopenmpt, the official 0.8.9 Windows dev package,
//! BSD-3-Clause; see its LICENSE.txt and Licenses/) and copies its DLLs next to the built
//! executables, so the music player (`src/host/openmpt.rs`) finds them at run time.

use std::path::PathBuf;

fn main() {
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let vendor = root.join("vendor").join("libopenmpt");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", vendor.display());
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        // Elsewhere libopenmpt comes from the system (`libopenmpt.so` / `.dylib`).
        println!("cargo:rustc-link-lib=dylib=openmpt");
        return;
    }
    println!("cargo:rustc-link-search=native={}", vendor.join("lib").join("amd64").display());
    // target/<profile>/build/<pkg>/out -> target/<profile>
    let out = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let Some(profile_dir) = out.ancestors().nth(3) else { return };
    let bin = vendor.join("bin").join("amd64");
    if let Ok(entries) = std::fs::read_dir(&bin) {
        for e in entries.flatten() {
            let p = e.path();
            if p.extension().is_some_and(|x| x.eq_ignore_ascii_case("dll")) {
                for dir in [profile_dir.to_path_buf(), profile_dir.join("deps"), profile_dir.join("examples")] {
                    let _ = std::fs::create_dir_all(&dir);
                    let _ = std::fs::copy(&p, dir.join(p.file_name().unwrap()));
                }
            }
        }
    }
}
