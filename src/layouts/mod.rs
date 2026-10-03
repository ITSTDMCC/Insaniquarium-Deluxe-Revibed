//! Byte-exact mirrors of every type in `port_types` (generated; see tools/gen_layouts.py).
//!
//! These are reference layouts of the 32-bit original, used for save-format work and to
//! keep field names traceable. Gameplay code uses the typed components in `crate::game`.
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, clippy::all)]
pub mod generated;
