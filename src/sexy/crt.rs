//! Replacements for the few MSVC runtime helpers whose exact results the game depends on.
//! (The C runtime itself is listed as `replaced` in port/manifest.csv.)

/// `_ftol2` / `_ftol2_sse` (`FUN_00564160`): truncates toward zero to a 64-bit integer.
/// NaN and values outside the i64 range give the "integer indefinite" 0x8000000000000000,
/// as `FISTP`/`CVTTSD2SI` do. Callers that keep an `int` take the low 32 bits.
#[inline]
pub fn ftol(x: f64) -> i64 {
    if x.is_nan() || x >= 9.223372036854775807e18 || x < -9.223372036854775808e18 {
        i64::MIN
    } else {
        x.trunc() as i64
    }
}

/// CRT `rand()`: the MSVC linear congruential generator (`_holdrand * 214013 + 2531011`,
/// bits 16..30).
pub fn rand(g: &mut crate::sexy::g::G) -> i32 {
    g.crt_holdrand = g.crt_holdrand.wrapping_mul(214013).wrapping_add(2531011);
    ((g.crt_holdrand >> 16) & 0x7fff) as i32
}

/// The inlined CRT `strcmp` on two NUL-free strings: <0, 0 or >0 by the first differing
/// byte (unsigned), a proper prefix sorting first.
pub fn strcmp(a: &[u8], b: &[u8]) -> i32 {
    match a.cmp(b) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

/// CRT `atol`: optional leading whitespace and sign, then decimal digits (wrapping like the
/// original's unchecked accumulation).
pub fn atol(s: &[u8]) -> i32 {
    let mut i = 0;
    while i < s.len() && matches!(s[i], b' ' | 9 | 10 | 13 | 0x0b | 0x0c) {
        i += 1;
    }
    let neg = i < s.len() && s[i] == b'-';
    if i < s.len() && (s[i] == b'-' || s[i] == b'+') {
        i += 1;
    }
    let mut v: i32 = 0;
    while i < s.len() && s[i].is_ascii_digit() {
        v = v.wrapping_mul(10).wrapping_add((s[i] - b'0') as i32);
        i += 1;
    }
    if neg { v.wrapping_neg() } else { v }
}
