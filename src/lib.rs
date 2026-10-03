//! Rust/Bevy port of Insaniquarium Deluxe (`WinFish.exe`), translated function by function
//! from the decompiled game stored in `gamedb_index/winfish.sqlite`.
//!
//! * [`layouts`]: byte-exact mirrors of every type in the database (generated).
//! * [`sexy`]: the parts of the PopCap framework whose behavior the game depends on
//!   (random numbers, drawing calls, save serialization), ported with the same names.
//! * [`game`]: the game classes (`Sexy::Board`, `Sexy::Fish`, ...).
//!
//! Each ported function carries a `/// port: <address> <qualified name>` tag; see
//! `port/manifest.csv` for the status of all 9,143 functions and `tests/parity.rs` for the
//! checks against the database.
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

pub mod game;
pub mod host;
pub mod layouts;
pub mod sexy;
