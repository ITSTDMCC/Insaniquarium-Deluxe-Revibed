//! Resource lookups by numeric id (the generated `Res.cpp` helpers).

use crate::sexy::prelude::*;
use crate::sexy::res_gen::RES_BY_ID;

/// port: 005016a0 FUN_005016a0
/// `GetIntById(int id)`: the value of the resource global for numeric id `param_1`.
pub fn FUN_005016a0(g: &G, param_1: i32) -> i32 {
    g.res.get(RES_BY_ID[param_1 as usize])
}

/// port: 005016c0 FUN_005016c0
/// `GetStringIdById(int id)`: the resource id string for numeric id `param_1` ("" outside
/// the table); the same table `GetIntById` indexes.
pub fn FUN_005016c0(param_1: i32) -> &'static str {
    let Some(&addr) = crate::sexy::res_gen::RES_BY_ID.get(param_1 as usize) else { return "" };
    crate::sexy::res_gen::RES_GLOBALS.iter().find(|r| r.0 == addr).map(|r| r.1).unwrap_or("")
}

/// port: 0050ff10 FUN_0050ff10
/// `ExtractResourcesByName(ResourceManager*, const char* group)`: the generated extractor
/// for "Init", "LoadingThread" or "Register"; false for any other group.
pub fn FUN_0050ff10(g: &mut G, param_2: &str) -> bool {
    use crate::game::res_extract_gen as r;
    match param_2 {
        "Init" => r::FUN_00506ea0(g),
        "LoadingThread" => r::FUN_005073e0(g),
        "Register" => r::FUN_0050e190(g),
        _ => false,
    }
}
