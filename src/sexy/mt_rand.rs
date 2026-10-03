//! `Sexy::MTRand`: the MT19937 generator the original seeds once and draws every random
//! number from (`Sexy::Rand()` and friends). Ported bit-for-bit so a given seed yields the
//! same sequence as WinFish.exe.

/// Default seed the original uses when none is given (`0x1105`).
pub const DEFAULT_SEED: u32 = 0x1105;
const N: usize = 0x270;
const M: usize = 0x18d;
/// `DAT_005e80ac`: `{0, 0x9908b0df}`.
const MAG01: [u32; 2] = [0, 0x9908_b0df];

/// Field layout matches the original object: `mt[0x270]` then `mti` at `+0x9c0`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MTRand {
    pub mt: [u32; N],
    pub mti: i32,
}

impl Default for MTRand {
    fn default() -> Self {
        FUN_0040ae20()
    }
}

/// port: 0040ae10 FUN_0040ae10
/// `MTRand::MTRand(unsigned long seed)`.
pub fn FUN_0040ae10(param_1: u32) -> MTRand {
    let mut r = MTRand { mt: [0; N], mti: 0 };
    FUN_0040ae50(&mut r, param_1);
    r
}

/// port: 0040ae20 FUN_0040ae20
/// `MTRand::MTRand()`: seeds with 0x1105.
pub fn FUN_0040ae20() -> MTRand {
    let mut r = MTRand { mt: [0; N], mti: 0 };
    FUN_0040ae50(&mut r, DEFAULT_SEED);
    r
}

/// port: 0040ae50 FUN_0040ae50
/// `MTRand::SRand(seed)`; a zero seed is replaced by 0x1105.
pub fn FUN_0040ae50(this: &mut MTRand, mut param_1: u32) {
    if param_1 == 0 {
        param_1 = DEFAULT_SEED;
    }
    this.mt[0] = param_1;
    this.mti = 1;
    loop {
        let i = this.mti as usize;
        let prev = this.mt[i - 1];
        this.mt[i] = ((prev >> 0x1e) ^ prev).wrapping_mul(0x6c07_8965).wrapping_add(i as u32);
        this.mti += 1;
        if this.mti >= N as i32 {
            break;
        }
    }
}

/// port: 0040aeb0 FUN_0040aeb0
/// `MTRand::Next()`: a 31-bit value (the tempered output masked with 0x7fffffff).
pub fn FUN_0040aeb0(this: &mut MTRand) -> u32 {
    let mt = &mut this.mt;
    if this.mti > 0x26f {
        let mut i = 0usize;
        while i < N - M {
            let y = ((mt[i + 1] ^ mt[i]) & 0x7fff_ffff) ^ mt[i];
            mt[i] = (y >> 1) ^ MAG01[(y & 1) as usize] ^ mt[i + M];
            i += 1;
        }
        while i < N - 1 {
            let y = ((mt[i + 1] ^ mt[i]) & 0x7fff_ffff) ^ mt[i];
            mt[i] = (y >> 1) ^ MAG01[(y & 1) as usize] ^ mt[i + M - N];
            i += 1;
        }
        let y = ((mt[N - 1] ^ mt[0]) & 0x7fff_ffff) ^ mt[N - 1];
        mt[N - 1] = MAG01[(y & 1) as usize] ^ mt[M - 1] ^ (y >> 1);
        this.mti = 0;
    }
    let mut y = mt[this.mti as usize];
    this.mti += 1;
    y ^= y >> 0xb;
    y ^= (y & 0xff3a_58ad) << 7;
    y ^= (y & 0xffff_df8c) << 0xf;
    ((y >> 0x12) ^ y) & 0x7fff_ffff
}

impl MTRand {
    pub fn next(&mut self) -> u32 {
        FUN_0040aeb0(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Reference MT19937 (Matsumoto & Nishimura, init_genrand + genrand_int32), masked like the original.
    #[test]
    fn matches_reference_mt19937() {
        let mut r = FUN_0040ae10(5489);
        // First outputs of genrand_int32() with seed 5489, masked to 31 bits.
        let want = [3499211612u32, 581869302, 3890346734, 3586334585, 545404204];
        for w in want {
            assert_eq!(r.next(), w & 0x7fff_ffff);
        }
        for _ in 0..2000 {
            r.next();
        }
        assert!(r.mti >= 0 && r.mti <= N as i32);
    }
}
