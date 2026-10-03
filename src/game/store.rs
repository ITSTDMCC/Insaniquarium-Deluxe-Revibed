//! The virtual tank's store ("the Fish Emporium"): `Sexy::StoreScreen`, its eight
//! `Sexy::StoreButtonWidget` shelves, the "Back" button, the shell count, the shopkeeper and
//! the speech bubble describing the hovered item; plus the daily stock picker that fills the
//! shelves (two plain fish, two colored ones, two random creatures with traits, a service
//! item or a special fish, and a backdrop or one more creature).
//!
//! `StoreScreen_data` starts at object offset 0x8c; the object is 0xbb0 bytes, far larger
//! than the database's 0xc4 (fields `ext_0xNNN`). It owns an MTRand at +0x1d0 seeded from the
//! day number, so the stock is the same all day. The database names the store's constructor
//! `StoreScreenOverlay::StoreScreenOverlay` (the overlay's constructor is inlined in it); the
//! overlay is the shared overlay class (`WExt::GameSelectorOverlay`, owner at +0x88).

use crate::sexy::button_widget::{BtnSub, ButtonExt};
use crate::sexy::graphics::{FUN_00455880, FUN_00455890, FUN_004558c0, FUN_004558e0, FUN_00455cf0, FUN_00455d20, FUN_004563d0, FUN_00456950, FUN_00456980, FUN_00456a00};
use crate::sexy::image_font as font;
use crate::sexy::mt_rand::{MTRand, FUN_0040ae20, FUN_0040ae50, FUN_0040aeb0};
use crate::sexy::prelude::*;
use crate::sexy::types::{CRect, FUN_00433320, FUN_00433360};
use std::collections::BTreeSet;

/// `StoreScreen_data` (object offset 0x8c) plus the bytes past the database's class size.
#[derive(Clone)]
pub struct StoreScreen_data {
    /// +0x8c the app.
    pub offset_0x0: Ptr,
    /// +0x90 (0).
    pub offset_0x4: i32,
    /// +0x94 the overlay widget (draws `DrawOverlay` above the shelves).
    pub offset_0x8: Ptr,
    /// +0x98 the "Back" button (id 99).
    pub offset_0xc: Ptr,
    /// +0x9c..+0xb8 the eight shelves (ids 0..7).
    pub offset_0x10: [Ptr; 8],
    /// +0xbc the shelf being bought from (highlighted while its dialog is up).
    pub field_0x30: Ptr,
    /// +0xc0 the shell count label.
    pub offset_0x34: Ptr,
    /// +0xc4 the shell count text (`sprintf` buffer).
    pub ext_0xc4: Vec<u8>,
    /// +0x1c4 x of the first creature shelf (0xc0).
    pub ext_0x1c4: i32,
    /// +0x1c8 spacing of the creature shelves (0x6f).
    pub ext_0x1c8: i32,
    /// +0x1cc the shopkeeper's animation countdown (also the anim time it is drawn at).
    pub ext_0x1cc: i32,
    /// +0x1d0 the stock RNG (MTRand, through +0xb94).
    pub ext_0x1d0: Box<MTRand>,
    /// +0xb98 the stock seed: day number + `DAT_005e8f1c` + the offset given to the picker.
    pub ext_0xb98: i64,
    /// +0xba0 "Thanks for shopping" countdown (0 = not shown).
    pub ext_0xba0: i32,
    /// +0xba4 leaving the store also runs `FUN_0054aff0` (back into the virtual tank).
    pub ext_0xba4: bool,
    /// +0xba8 the hovered shelf (-1 = none).
    pub ext_0xba8: i32,
    /// +0xbac updates the hovered shelf has been hovered.
    pub ext_0xbac: i32,
}

impl std::fmt::Debug for StoreScreen_data {
    /// Everything but the RNG's 624-word state.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StoreScreen_data")
            .field("offset_0x0", &self.offset_0x0)
            .field("offset_0x8", &self.offset_0x8)
            .field("offset_0xc", &self.offset_0xc)
            .field("offset_0x10", &self.offset_0x10)
            .field("field_0x30", &self.field_0x30)
            .field("offset_0x34", &self.offset_0x34)
            .field("ext_0x1cc", &self.ext_0x1cc)
            .field("ext_0xb98", &self.ext_0xb98)
            .field("ext_0xba0", &self.ext_0xba0)
            .field("ext_0xba4", &self.ext_0xba4)
            .field("ext_0xba8", &self.ext_0xba8)
            .field("ext_0xbac", &self.ext_0xbac)
            .finish_non_exhaustive()
    }
}

/// `StoreButtonWidget_data` (object offset 0x120).
#[derive(Debug, Clone, Default)]
pub struct StoreButtonWidget_data {
    /// +0x120 the app (`gApp` at construction).
    pub offset_0x0: Ptr,
    /// +0x124 the creature on the shelf (owned; kind 1).
    pub offset_0x4: Ptr,
    /// +0x128 price in shells.
    pub offset_0x8: i32,
    /// +0x12c the price text (std::string 0x12c..0x148; "SOLD" when sold out).
    pub field_0xc: Vec<u8>,
    /// +0x148 selected (normal and over cels swapped).
    pub offset_0x28: bool,
    /// +0x14c what is on the shelf: 0 sold out, 1 a creature, 2 the Bubbulator, 3 a backdrop,
    /// 4 the Alien Attractor, 5 the food upgrade.
    pub offset_0x2c: i32,
    /// +0x150 price label center x.
    pub offset_0x30: i32,
    /// +0x154 price label y.
    pub offset_0x34: i32,
    /// +0x158 the item's parameter (the backdrop number).
    pub offset_0x38: i32,
}

impl G {
    pub fn store(&mut self, p: Ptr) -> &mut StoreScreen_data {
        match &mut self.widget(p).ext {
            WExt::Store(d) => d,
            e => panic!("{p} is not a StoreScreen: {e:?}"),
        }
    }
    pub fn store_button(&mut self, p: Ptr) -> &mut StoreButtonWidget_data {
        match &mut self.widget(p).ext {
            WExt::Button(e) => match &mut e.sub {
                BtnSub::StoreButton(d) => d,
                s => panic!("{p} is not a StoreButtonWidget: {s:?}"),
            },
            e => panic!("{p} is not a ButtonWidget: {e:?}"),
        }
    }
}

/// The store's own RNG (+0x1d0).
fn store_rand(g: &mut G, this: Ptr) -> u32 {
    FUN_0040aeb0(&mut g.store(this).ext_0x1d0)
}

/// The app's RNG (+0x7b0).
fn app_rand(g: &mut G, app: Ptr) -> u32 {
    let r = g.wfa(app).offset_0x84;
    FUN_0040aeb0(g.mtrand(r))
}

/// The board's objects, in set order.
fn board_objects(g: &mut G, app: Ptr) -> BTreeSet<Ptr> {
    let board = g.wfa(app).offset_0x4;
    g.board(board).offset_0x7c.clone()
}

/// port: 004e3e30 FUN_004e3e30
/// `std::set<GameObject*>::iterator::operator++` (an STL instance): the element after
/// `param_1`, `None` at the end.
pub fn FUN_004e3e30(this: &BTreeSet<Ptr>, param_1: Ptr) -> Option<Ptr> {
    use std::ops::Bound::{Excluded, Unbounded};
    this.range((Excluded(param_1), Unbounded)).next().copied()
}

/// port: 0051c200 FUN_0051c200
/// `std::set<int>::find` (an STL instance): the element equal to `param_2`, `None` (end)
/// when absent.
pub fn FUN_0051c200(this: &BTreeSet<i32>, param_2: &i32) -> Option<i32> {
    this.get(param_2).copied()
}

/// port: 00501060 FUN_00501060
/// The creature's trait class for the store: 8 a special fish; else the last trait set (0
/// +0xfc, 1 +0x104, 2 +0xf6, 3 +0x110, 4 +0xf5, 5 an exotic feeder, 6 a special feeder),
/// 7 for two or more, 9 for none.
pub fn FUN_00501060(g: &mut G, param_1: Ptr) -> i32 {
    let go = g.go(param_1).clone();
    if go.offset_0x9c != -1 {
        return 8;
    }
    let mut n: u8 = go.offset_0x74 as u8;
    let feeder = go.offset_0x68;
    let mut r = 9;
    if go.offset_0x74 {
        r = 0;
    }
    if go.offset_0x7c {
        r = 1;
        n = n.wrapping_add(1);
    }
    if go.offset_0x6e {
        r = 2;
        n = n.wrapping_add(1);
    }
    if go.offset_0x88 {
        r = 3;
        n = n.wrapping_add(1);
    }
    if go.offset_0x6d {
        r = 4;
        n = n.wrapping_add(1);
    }
    if feeder == 3 || feeder == 4 || feeder == 5 {
        r = 5;
        n = n.wrapping_add(1);
    }
    if feeder == 0x3ee || feeder == 0x3ed || feeder == 1000 {
        r = 6;
        n = n.wrapping_add(1);
    }
    if 1 < n {
        r = 7;
    }
    r
}

/// port: 00501490 FUN_00501490
/// `SetStockDay(__int64)`: a new stock day clears the eight sold-out flags.
pub fn FUN_00501490(g: &mut G, this: Ptr, param_1: i32, param_2: i32) {
    let p = g.profile(this);
    if param_1 != p.field_0xa0 || param_2 != p.field_0xa4 {
        p.field_0xa0 = param_1;
        p.field_0xa4 = param_2;
        p.field_0xa8 = [false; 8];
    }
}

/// port: 005195e0 FUN_005195e0
/// Whether trait `param_1` suits creature type `param_2` (ECX trait, EAX type): 1 not on
/// BallFish or Penta, 4 not on Penta, Grubber or BallFish, 5 not on BallFish, 6 not on
/// Penta or Grubber.
pub fn FUN_005195e0(param_1: i32, param_2: i32) -> bool {
    match param_1 {
        1 => param_2 != 0x23 && param_2 != 8,
        4 => param_2 != 8 && param_2 != 9 && param_2 != 0x23,
        5 => param_2 != 0x23,
        6 => param_2 != 8 && param_2 != 9,
        _ => true,
    }
}

/// port: 00519650 FUN_00519650
/// Whether all of the creature's (EDX) checked traits suit its type.
pub fn FUN_00519650(g: &mut G, param_1: Ptr) -> bool {
    let go = g.go(param_1).clone();
    let (feeder, ty) = (go.offset_0x68, go.offset_0x4);
    let mut ok = true;
    if go.offset_0x7c {
        ok = FUN_005195e0(1, ty);
    }
    if go.offset_0x6d {
        ok = ok && FUN_005195e0(4, ty);
    }
    if feeder == 3 || feeder == 4 || feeder == 5 {
        ok = ok && FUN_005195e0(5, ty);
    }
    if feeder == 0x3ee || feeder == 0x3ed || feeder == 1000 {
        ok = ok && FUN_005195e0(6, ty);
    }
    ok
}

/// port: 00519700 FUN_00519700
/// Clears the creature's (EAX) traits.
pub fn FUN_00519700(g: &mut G, param_1: Ptr) {
    let go = g.go(param_1);
    go.offset_0x68 = 0;
    go.offset_0x7c = false;
    go.offset_0x6d = false;
    go.offset_0x74 = false;
    go.offset_0x88 = false;
    go.offset_0x6e = false;
}

/// port: 00519730 FUN_00519730
/// The creature's (ECX) price: by type (colored and rainbow guppies and Oscars cost more),
/// 5000 more with any trait; a trait class fixes it (25000..50000); else rounded down to
/// a multiple of 5 (at least 10).
pub fn FUN_00519730(g: &mut G, this: Ptr) -> i32 {
    let ty = g.go(this).offset_0x4;
    let mut price: u32 = 100;
    match ty {
        0 => {
            let f = g.fish(this).clone();
            price = if !f.field_0x7c { 0x19 } else if f.field_0x7d { 2500 } else { 500 };
        }
        5 => {
            let f = g.fish(this).clone();
            if f.field_0x7c {
                price = if f.field_0x7d { 5000 } else { 1000 };
            }
        }
        6 | 0x24 => price = 10000,
        7 => price = 0xdac,
        8 | 9 => price = 0x9c4,
        10 => price = 5000,
        0x22 => price = 15000,
        0x23 => price = 20000,
        _ => {}
    }
    let class = FUN_00501060(g, this);
    if class != 9 {
        price += 5000;
    }
    match class {
        0 | 3 | 6 => 30000,
        1 | 2 | 4 | 5 => 25000,
        7 => 40000,
        8 => 50000,
        _ => {
            let p = ((price / 5) * 5) as i32;
            if p == 0 { 10 } else { p }
        }
    }
}

/// port: 00519890 FUN_00519890
/// Gives the fish a random color (`SetColorSeed(rand % 1000, rainbow)`), rainbow one time
/// in 30 when `param_2`.
pub fn FUN_00519890(g: &mut G, this: Ptr, param_1: Ptr, param_2: bool) {
    let rainbow = param_2 && store_rand(g, this) % 0x1e == 0;
    let seed = (store_rand(g, this) % 1000) as i32;
    vcall!(g, param_1, fish.vfunction91, seed, rainbow);
}

/// port: 005198f0 FUN_005198f0
/// A random special feeder (1000, 0x3ed or 0x3ee) the creature can eat from: not 0x3ed for
/// an Ultra, not 1000 for an Oscar, Grubber or BiFish; a Sylvester always gets 1000.
pub fn FUN_005198f0(g: &mut G, this: Ptr, param_1: Ptr) -> u32 {
    let ty = g.go(param_1).offset_0x4;
    loop {
        let r = store_rand(g, this);
        let q = r / 3;
        match r % 3 {
            0 => g.go(param_1).offset_0x68 = 1000,
            1 => g.go(param_1).offset_0x68 = 0x3ed,
            _ => g.go(param_1).offset_0x68 = 0x3ee,
        }
        if ty == 6 {
            if g.go(param_1).offset_0x68 != 0x3ed {
                return q;
            }
            continue;
        }
        if ty != 5 && ty != 9 && ty != 0x24 {
            if ty == 0x22 {
                g.go(param_1).offset_0x68 = 1000;
            }
            return q;
        }
        if g.go(param_1).offset_0x68 != 1000 {
            return q;
        }
    }
}

/// port: 005199a0 FUN_005199a0
/// A random exotic feeder (3, 4 or 5). The original returns a leftover register (rand / 3,
/// or the creature pointer for 4); no caller reads it, so nothing is returned.
pub fn FUN_005199a0(g: &mut G, this: Ptr, param_1: Ptr) {
    let r = store_rand(g, this);
    match r % 3 {
        0 => g.go(param_1).offset_0x68 = 3,
        1 => g.go(param_1).offset_0x68 = 4,
        _ => g.go(param_1).offset_0x68 = 5,
    }
}

/// port: 00519a00 FUN_00519a00
/// A trait to give: with `param_1` the day's (a fixed 100-entry table, built once into
/// `DAT_005e9090`, indexed by the stock seed), else a weighted random one (5 most likely;
/// 7 means two traits at once, 8 a special fish).
pub fn FUN_00519a00(g: &mut G, this: Ptr, param_1: bool) -> i32 {
    let weights = [(0x30, 5), (0x14, 1), (10, 3), (5, 2), (5, 0), (5, 6), (5, 4), (2, 7)];
    if !g.globals.DAT_005e9220 {
        g.globals.DAT_005e9220 = true;
        let t = &mut g.globals.DAT_005e9090;
        t.resize(100, 0);
        for u in 0u32..100 {
            if u & 1 == 0 {
                t[u as usize] = 5;
            } else {
                let m = u % 10;
                if m == 1 || m == 5 {
                    t[u as usize] = 3;
                } else if m == 3 {
                    t[u as usize] = 1;
                } else {
                    match u % 0x14 {
                        7 => t[u as usize] = 2,
                        9 => t[u as usize] = 0,
                        0x11 => t[u as usize] = 6,
                        0x13 => t[u as usize] = 4,
                        _ => {}
                    }
                }
            }
            if u == (u / 10) * 10 {
                t[u as usize] = ((u / 10) & 1 != 0) as i32 + 7;
            }
        }
    }
    if param_1 {
        // `__allrem`: signed 64-bit remainder.
        let i = (g.store(this).ext_0xb98 % 100) as usize;
        return g.globals.DAT_005e9090[i];
    }
    let r = (store_rand(g, this) % 100) as i32;
    let mut acc = 0;
    let mut i = 0;
    while i < 8 {
        acc += weights[i].0;
        if r < acc {
            break;
        }
        i += 1;
    }
    if i == 8 {
        i = 0;
    }
    weights[i].1
}

/// port: 00519be0 FUN_00519be0
/// Two random traits (or one plus a feeder) that suit the creature; up to 100 tries.
pub fn FUN_00519be0(g: &mut G, this: Ptr, param_1: Ptr) {
    let mut tries = 0;
    loop {
        let r = store_rand(g, this);
        match r % 7 {
            0 => {
                g.go(param_1).offset_0x74 = true;
                g.go(param_1).offset_0x7c = true;
            }
            1 => {
                g.go(param_1).offset_0x74 = true;
                g.go(param_1).offset_0x88 = true;
            }
            3 => {
                g.go(param_1).offset_0x6d = true;
                g.go(param_1).offset_0x6e = true;
            }
            4 => {
                g.go(param_1).offset_0x7c = true;
                g.go(param_1).offset_0x6e = true;
            }
            5 | 2 => {
                if r % 7 == 5 {
                    g.go(param_1).offset_0x74 = true;
                }
                g.go(param_1).offset_0x7c = true;
                g.go(param_1).offset_0x88 = true;
            }
            _ => {
                match store_rand(g, this) % 5 {
                    0 => g.go(param_1).offset_0x74 = true,
                    1 => g.go(param_1).offset_0x7c = true,
                    2 => g.go(param_1).offset_0x6e = true,
                    3 => g.go(param_1).offset_0x88 = true,
                    _ => g.go(param_1).offset_0x6d = true,
                }
                if store_rand(g, this) & 1 == 0 {
                    FUN_005198f0(g, this, param_1);
                } else {
                    FUN_005199a0(g, this, param_1);
                }
            }
        }
        if FUN_00519650(g, param_1) {
            return;
        }
        FUN_00519700(g, param_1);
        tries += 1;
        if 99 < tries {
            return;
        }
    }
}

/// port: 00519d30 FUN_00519d30
/// A trait that suits the creature (the day's while `param_2`, random after 50 failed
/// tries); up to 100 tries.
pub fn FUN_00519d30(g: &mut G, this: Ptr, param_1: Ptr, mut param_2: bool) {
    let mut i = 0;
    loop {
        match FUN_00519a00(g, this, param_2) {
            0 => g.go(param_1).offset_0x74 = true,
            1 => g.go(param_1).offset_0x7c = true,
            2 => g.go(param_1).offset_0x6e = true,
            3 => g.go(param_1).offset_0x88 = true,
            4 => g.go(param_1).offset_0x6d = true,
            5 => FUN_005199a0(g, this, param_1),
            6 => {
                FUN_005198f0(g, this, param_1);
            }
            7 | 8 => FUN_00519be0(g, this, param_1),
            _ => {}
        }
        if FUN_00519650(g, param_1) {
            return;
        }
        FUN_00519700(g, param_1);
        if 0x31 < i {
            param_2 = false;
        }
        i += 1;
        if 100 <= i {
            return;
        }
    }
}

/// port: 00519e00 FUN_00519e00
/// A guppy or an Oscar, at random.
pub fn FUN_00519e00(g: &mut G, this: Ptr) -> Ptr {
    if store_rand(g, this) & 1 == 0 {
        crate::game::fish::Fish__004eef00(g, 0, 0)
    } else {
        crate::game::oscar::Oscar__004efda0(g, 0, 0)
    }
}

/// port: 00519eb0 FUN_00519eb0
/// Shelves 0 and 1: a guppy and an Oscar.
pub fn FUN_00519eb0(g: &mut G, this: Ptr) {
    let fish = crate::game::fish::Fish__004eef00(g, 0, 0);
    let oscar = crate::game::oscar::Oscar__004efda0(g, 0, 0);
    let b = g.store(this).offset_0x10;
    let p = FUN_00519730(g, fish);
    FUN_00537290(g, b[0], fish, p);
    let p = FUN_00519730(g, oscar);
    FUN_00537290(g, b[1], oscar, p);
}

/// port: 00519f70 FUN_00519f70
/// Shelves 3 and 4: a colored guppy and a colored Oscar.
pub fn FUN_00519f70(g: &mut G, this: Ptr) {
    let fish = crate::game::fish::Fish__004eef00(g, 0, 0);
    let oscar = crate::game::oscar::Oscar__004efda0(g, 0, 0);
    FUN_00519890(g, this, fish, true);
    FUN_00519890(g, this, oscar, true);
    let b = g.store(this).offset_0x10;
    let p = FUN_00519730(g, fish);
    FUN_00537290(g, b[3], fish, p);
    let p = FUN_00519730(g, oscar);
    FUN_00537290(g, b[4], oscar, p);
}

/// port: 0051b260 FUN_0051b260
/// A random creature: with `param_1`, three times in four a guppy or Oscar (colored three
/// times in four); else a weighted pick (Penta 26%, Grubber 25%, Gekko 25%, Breeder 10%,
/// Ultra 5%, BiFish, BallFish and Sylvester 3% each) that is not type `param_2` and suits
/// trait `param_3`; after 100 failures, any creature (`(param_1, -1, 9)`).
pub fn FUN_0051b260(g: &mut G, this: Ptr, param_1: bool, param_2: i32, param_3: i32) -> Ptr {
    if param_1 && store_rand(g, this) % 100 <= 0x4b {
        let f = FUN_00519e00(g, this);
        if store_rand(g, this) % 100 <= 0x4b {
            FUN_00519890(g, this, f, true);
        }
        return f;
    }
    let table = [(0x1a, 8), (0x19, 9), (0x19, 7), (10, 10), (5, 6), (3, 0x24), (3, 0x23), (3, 0x22)];
    let mut tries = 0;
    loop {
        let r = (store_rand(g, this) % 100) as i32;
        let mut acc = 0;
        let mut i = 0;
        while i < 8 {
            acc += table[i].0;
            if r < acc {
                break;
            }
            i += 1;
        }
        if i == 8 {
            i = 0;
        }
        let ty = table[i].1;
        if ty != param_2 && FUN_005195e0(param_3, ty) {
            return match ty {
                6 => crate::game::ultra::Ultra__004f02a0(g, 0, 0),
                7 => crate::game::gekko::Gekko__004ef9a0(g, 0, 0),
                9 => crate::game::grubber::Grubber__004ea7b0(g, 0, 0),
                10 => crate::game::breeder::Breeder__004ee530(g, 0, 0),
                0x22 => crate::game::vt_fish::SylvesterFish__004f01e0(g, 0, 0, false),
                0x23 => crate::game::vt_fish::BallFish__004f0570(g, 0, 0, false),
                0x24 => {
                    let b = crate::game::vt_fish::BiFish__004f0630(g, 0, 0, false);
                    let r = store_rand(g, this);
                    g.bi_fish(b).offset_0x4 = (r & 1) as i32;
                    b
                }
                // 8 and anything else.
                _ => crate::game::penta::Penta__004ec0f0(g, 0, 0),
            };
        }
        tries += 1;
        if 99 < tries {
            return FUN_0051b260(g, this, param_1, -1, 9);
        }
    }
}

/// port: 0051b790 FUN_0051b790
/// The speech bubble for a creature: its name ("This is Rocky.", Cookie's own line) when
/// it has one, else a line for its trait class, else one for its type. (The original
/// formats into the static buffer `DAT_005e9228`; a null creature gives null.)
pub fn FUN_0051b790(g: &mut G, param_1: Ptr) -> Option<Vec<u8>> {
    if param_1 == NULL {
        return None;
    }
    let go = g.go(param_1).clone();
    if !go.field_0x28.is_empty() {
        if go.offset_0x9c == 2 {
            return Some(b"This is Cookie.\nCookie will feed\nyour more exotic fish.".to_vec());
        }
        let name = String::from_utf8_lossy(&go.field_0x28).into_owned();
        let s = if go.field_0x28.last() == Some(&b'.') { format!("This is {name}") } else { format!("This is {name}.") };
        return Some(s.into_bytes());
    }
    let s: &[u8] = match FUN_00501060(g, param_1) {
        0 => b"You've never seen\na fish like this!",
        1 => b"This fish is a\nvoracious eater.",
        2 => b"This fish goes\nfrom 0 to 60\nin 1.3 seconds!",
        3 => b"This fish is\na musical genius!",
        4 => b"This fish is\n\"forwardly challenged\"\nbut it's great otherwise!",
        5 => b"This fish has\ndeveloped a taste\nfor exotic food!",
        6 => b"This fish has a\nvery special diet.",
        7 | 8 => b"This fish is extremely \nrare.  We hardly ever\nget them in stock.",
        _ => match go.offset_0x4 {
            0 => {
                let f = g.fish(param_1).clone();
                if !f.field_0x7c {
                    b"These adorable little\nfish are a mainstay\nof any aquarium!"
                } else if !f.field_0x7d {
                    b"Choose from a dazzling\narray of colors.  C'mon!\nyou know you want one!"
                } else {
                    b"This one's pretty\ncool, isn't it?"
                }
            }
            5 => {
                let f = g.fish(param_1).clone();
                if f.field_0x7c && f.field_0x7d {
                    b"This one's pretty\ncool, isn't it?"
                } else {
                    b"Specially trained not\nto eat store-bought\nguppies.  I promise!"
                }
            }
            6 => b"Many regard this\nto be the\nSUV of fish!",
            8 => b"Isn't this little guy\njust the cutest?\nAnswer: Yes.",
            9 => b"It won't eat\nyour guppies, but\nwatch your fingers!",
            10 => b"Take care of this fish,\nand she'll give you a\nbaby fish at\nno extra charge!",
            0x22 => b"Don't worry.  This\n\"fish\" has been\ndomesticated.",
            0x23 => b"Some people think\nthis fish looks\nlike a ball.",
            0x24 => b"This \"fish\" may look\ndead, but he is actually\nquite lively!",
            _ => b"All of our fish come\nwith a free plastic bag!\nTake one home today!",
        },
    };
    Some(s.to_vec())
}

/// port: 0051b9e0 FUN_0051b9e0
/// The speech bubble for a shelf.
pub fn FUN_0051b9e0(g: &mut G, param_1: Ptr) -> Option<Vec<u8>> {
    let s: &[u8] = match g.store_button(param_1).offset_0x2c {
        0 => b"Sorry, this is sold out.\nCheck back tomorrow!",
        1 => {
            let item = FUN_00531990(g, param_1, false);
            return FUN_0051b790(g, item);
        }
        2 => b"Want to keep more fish\nin your tank?  Buy\nThe Bubbulator!",
        3 => b"Keep your fish happy\nwith this beautiful\nnew backdrop!",
        4 => b"Need more excitement\nin your tank?  Buy\nthe alien attractor!\nTrust me.  It's safe!",
        5 => b"Your fish will\ngrow faster with\nthis food upgrade!",
        _ => return None,
    };
    Some(s.to_vec())
}

/// port: 0051c0b0 FUN_0051c0b0
/// Whether the tank holds special fish `param_1`.
pub fn FUN_0051c0b0(g: &mut G, this: Ptr, param_1: i32) -> bool {
    let app = g.store(this).offset_0x0;
    let objs = board_objects(g, app);
    let mut it = objs.first().copied();
    while let Some(o) = it {
        if g.go(o).offset_0x9c == param_1 {
            return true;
        }
        it = FUN_004e3e30(&objs, o);
    }
    false
}

/// port: 0052b800 FUN_0052b800
/// Special fish `param_1`: 0 Rocky (a blue and white guppy with trait +0x104 and feeder
/// 0x3ee), 1 Ludwig (a black, green and blue Oscar with trait +0x110), 2 Cookie (a Gekko
/// with feeder 6), 3 Johnny V. (a random-colored rainbow Oscar with feeder 3), 4 Kilgore (an
/// Ultra with traits +0x104 and +0x110); its special id is set. Others give null.
pub fn FUN_0052b800(g: &mut G, this: Ptr, param_1: i32) -> Ptr {
    let f = match param_1 {
        0 => {
            let f = crate::game::fish::Fish__004eef00(g, 0, 0);
            let d = g.fish(f);
            d.field_0x7c = true;
            d.field_0x84 = FUN_00433320(0x2288ff);
            d.field_0x94 = FUN_00433320(0xffffff);
            let go = g.go(f);
            go.offset_0x7c = true;
            go.offset_0x68 = 0x3ee;
            go.field_0x28 = b"Rocky".to_vec();
            f
        }
        1 => {
            let f = crate::game::oscar::Oscar__004efda0(g, 0, 0);
            g.go(f).offset_0x88 = true;
            g.go(f).field_0x28 = b"Ludwig".to_vec();
            let d = g.fish(f);
            d.field_0x7c = true;
            d.field_0x84 = FUN_00433320(0);
            d.field_0x94 = FUN_00433320(0x8df5be);
            d.field_0xa4 = FUN_00433320(0x9ffa);
            f
        }
        2 => {
            let f = crate::game::gekko::Gekko__004ef9a0(g, 0, 0);
            g.go(f).field_0x28 = b"Cookie".to_vec();
            g.go(f).offset_0x68 = 6;
            f
        }
        3 => {
            let f = crate::game::oscar::Oscar__004efda0(g, 0, 0);
            let seed = (store_rand(g, this) % 1000) as i32;
            vcall!(g, f, fish.vfunction91, seed, true);
            g.go(f).field_0x28 = b"Johnny V.".to_vec();
            g.go(f).offset_0x68 = 3;
            f
        }
        4 => {
            let f = crate::game::ultra::Ultra__004f02a0(g, 0, 0);
            g.go(f).field_0x28 = b"Kilgore".to_vec();
            g.go(f).offset_0x7c = true;
            g.go(f).offset_0x88 = true;
            f
        }
        _ => return NULL,
    };
    g.go(f).offset_0x9c = param_1;
    f
}

/// port: 0052f0d0 FUN_0052f0d0
/// Shelves 2 and 7. Shelf 2: the Bubbulator (20000) until bought, then the Alien Attractor
/// (50000), then the food upgrade (20000) while food is not upgraded; after that Cookie
/// (until the tank has her) or a random creature, half the time with a trait. Shelf 7: a
/// backdrop the player lacks (the fifth only one day in five), picked by the stock seed, at
/// 15000; else a random creature, half the time with a trait.
pub fn FUN_0052f0d0(g: &mut G, this: Ptr) {
    let app = g.store(this).offset_0x0;
    let profile = g.wfa(app).offset_0x18c;
    let b = g.store(this).offset_0x10;
    let p = g.profile(profile).clone();
    if p.field_0x90 == 0 {
        FUN_00537230(g, b[2], 2, 0, 20000);
    } else if p.field_0x94 == 0 {
        FUN_00537230(g, b[2], 4, 0, 50000);
    } else if g.globals.DAT_005e89dc == 0 {
        FUN_00537230(g, b[2], 5, 0, 20000);
    } else {
        let f = if !FUN_0051c0b0(g, this, 2) {
            FUN_0052b800(g, this, 2)
        } else {
            let f = FUN_0051b260(g, this, false, -1, 9);
            if store_rand(g, this) & 1 == 0 {
                FUN_00519d30(g, this, f, false);
            }
            f
        };
        let price = FUN_00519730(g, f);
        FUN_00537290(g, b[2], f, price);
    }
    let mut backdrops: Vec<i32> = Vec::new();
    for i in 0..6 {
        let owned = g.profile(profile).field_0x72[i as usize];
        if !owned && (i != 5 || store_rand(g, this) % 5 == 0) {
            backdrops.push(i);
        }
    }
    if !backdrops.is_empty() {
        let n = backdrops.len() as i64;
        let k = (g.store(this).ext_0xb98 % n) as usize;
        FUN_00537230(g, b[7], 3, backdrops[k], 15000);
    } else {
        let f = FUN_0051b260(g, this, true, -1, 9);
        if store_rand(g, this) & 1 == 0 {
            FUN_00519d30(g, this, f, false);
        }
        let price = FUN_00519730(g, f);
        FUN_00537290(g, b[7], f, price);
    }
}

/// port: 0052fb80 FUN_0052fb80
/// A special fish the tank does not hold yet (not Cookie and not 5), at random; null when
/// the tank has them all.
pub fn FUN_0052fb80(g: &mut G, this: Ptr) -> Ptr {
    let app = g.store(this).offset_0x0;
    let mut have: BTreeSet<i32> = BTreeSet::new();
    have.insert(5);
    have.insert(2);
    let objs = board_objects(g, app);
    let mut it = objs.first().copied();
    while let Some(o) = it {
        let id = g.go(o).offset_0x9c;
        if id != -1 {
            have.insert(id);
        }
        it = FUN_004e3e30(&objs, o);
    }
    let mut ids: Vec<i32> = Vec::new();
    for i in 0..6 {
        if FUN_0051c200(&have, &i).is_none() {
            ids.push(i);
        }
    }
    if ids.is_empty() {
        return NULL;
    }
    let k = store_rand(g, this) % ids.len() as u32;
    FUN_0052b800(g, this, ids[k as usize])
}

/// port: 0052fe10 FUN_0052fe10
/// Shelves 5 and 6: a random creature (no trait), and another of a different type with the
/// day's trait, or a missing special fish when the day's trait is 8.
pub fn FUN_0052fe10(g: &mut G, this: Ptr) {
    let first = FUN_0051b260(g, this, false, -1, 9);
    let k = FUN_00519a00(g, this, true);
    let mut second = NULL;
    if k == 8 {
        second = FUN_0052fb80(g, this);
    }
    if second == NULL {
        let ty = g.go(first).offset_0x4;
        second = FUN_0051b260(g, this, true, ty, k);
        FUN_00519d30(g, this, second, true);
    }
    let b = g.store(this).offset_0x10;
    let p = FUN_00519730(g, first);
    FUN_00537290(g, b[5], first, p);
    let p = FUN_00519730(g, second);
    FUN_00537290(g, b[6], second, p);
}

/// port: 0052fea0 FUN_0052fea0
/// `RestockShelves(int dayOffset)`: seeds the stock RNG from the day (`DAT_005e8f1c` + the
/// app's day number + `param_1`, plus the profile's random key), starts a new stock day in
/// the profile, fills the shelves and shows each one, or "SOLD" when it was bought today or
/// is empty.
pub fn FUN_0052fea0(g: &mut G, this: Ptr, param_1: i32) {
    let app = g.store(this).offset_0x0;
    let seed = g.globals.DAT_005e8f1c as i64 + g.wfa(app).offset_0x1c + param_1 as i64;
    g.store(this).ext_0xb98 = seed;
    let profile = g.wfa(app).offset_0x18c;
    let key = g.profile(profile).field_0x98.wrapping_add(seed as i32);
    FUN_0040ae50(&mut g.store(this).ext_0x1d0, key as u32);
    FUN_00501490(g, profile, seed as i32, (seed >> 32) as i32);
    FUN_00519eb0(g, this);
    FUN_00519f70(g, this);
    FUN_0052fe10(g, this);
    FUN_0052f0d0(g, this);
    let b = g.store(this).offset_0x10;
    for i in 0..8 {
        let sold = g.profile(profile).field_0xa8[i];
        if !sold && g.store_button(b[i]).offset_0x2c != 0 {
            FUN_005363d0(g, b[i]);
        } else {
            FUN_005372f0(g, b[i]);
        }
    }
}

/// port: 0052b2d0 StoreScreenOverlay::StoreScreenOverlay
/// `StoreScreen(WinFishApp*)`: full screen; the "Back" button (id 99, yellow label) at the
/// bottom left; eight shelves (three tall ones on the left, four creature shelves along the
/// top, the backdrop shelf by the counter); the shell count label; the shopkeeper's timer;
/// the store music (song 2); the overlay.
pub fn StoreScreenOverlay(g: &mut G, param_1: Ptr) -> Ptr {
    let (wc, w) = crate::sexy::widget::Widget();
    let d = StoreScreen_data {
        offset_0x0: param_1,
        offset_0x4: 0,
        offset_0x8: NULL,
        offset_0xc: NULL,
        offset_0x10: [NULL; 8],
        field_0x30: NULL,
        offset_0x34: NULL,
        ext_0xc4: Vec::new(),
        ext_0x1c4: 0xc0,
        ext_0x1c8: 0x6f,
        ext_0x1cc: 0,
        ext_0x1d0: Box::new(FUN_0040ae20()),
        ext_0xb98: 0,
        ext_0xba0: 0,
        ext_0xba4: false,
        ext_0xba8: -1,
        ext_0xbac: 0,
    };
    let this = g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__StoreScreen_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::Store(Box::new(d)) }),
    });
    let wm = g.sab(param_1).offset_0x318;
    let (aw, ah) = (g.sab(param_1).field_0xb8, g.sab(param_1).field_0xbc);
    let wc = g.wc(this);
    wc.offset_0xc = wm;
    wc.offset_0x2c = 0;
    wc.offset_0x30 = 0;
    wc.offset_0x34 = aw;
    wc.offset_0x38 = ah;

    let f = g.res.DAT_005e8cf0;
    let back = crate::game::game_selector::FUN_00506760(g, 99, this, b"Back", f);
    g.store(this).offset_0xc = back;
    g.w(back).offset_0xc[0] = CRect(0xff, 0xf0, 0, 0xff);
    let h = g.wc(back).offset_0x38;
    vcall!(g, back, w.vfunction41, 6, 0x1b8, 0x71, h);

    let (x0, dx) = (g.store(this).ext_0x1c4, g.store(this).ext_0x1c8);
    let rects = [
        Rect::new(1, 5, 0x6d, 0x8c),
        Rect::new(1, 0x8f, 0x6d, 0x8c),
        Rect::new(1, 0x11a, 0x6d, 0x8c),
        Rect::new(x0 + 1, 0x26, 0x62, 0x8c),
        Rect::new(x0 + dx, 0x26, 0x62, 0x8c),
        Rect::new(x0 + dx * 2, 0x26, 0x62, 0x8c),
        Rect::new(x0 + dx * 2 + dx, 0x26, 0x62, 0x8c),
        Rect::new(0x1e9, 0x153, 0x79, 100),
    ];
    let (tall, short, counter) = (g.res.DAT_005e8c5c, g.res.DAT_005e8c78, g.res.DAT_005e8b3c);
    let images = [tall, tall, tall, short, short, short, short, counter];
    for i in 0..8 {
        let b = StoreButtonWidget(g, i as i32, this, images[i]);
        g.store(this).offset_0x10[i] = b;
        vcall!(g, b, w.vfunction40, rects[i]);
    }
    let b = g.store(this).offset_0x10;
    g.store_button(b[7]).offset_0x34 = 0x54;
    for i in 3..7 {
        g.store_button(b[i]).offset_0x30 = 0;
    }

    let label = crate::game::board_parts::MyLabelWidget(g);
    g.store(this).offset_0x34 = label;
    g.my_label(label).offset_0x2c = 2;
    g.w(label).offset_0x1 = false;
    g.wc(label).offset_0x2c = 0xee;
    g.wc(label).offset_0x30 = 0x1b7;
    let lf = g.res.DAT_005e8ce4;
    g.my_label(label).offset_0x30 = lf;
    let fh = font::get_height(g, lf);
    g.wc(label).offset_0x38 = fh;
    g.wc(label).offset_0x34 = 0x50;
    g.my_label(label).offset_0x1c = CRect(0xfa, 0x9b, 0x96, 0xff);
    let profile = g.wfa(param_1).offset_0x18c;
    let shells = g.profile(profile).field_0x48;
    let text = format!("{shells}").into_bytes();
    g.store(this).ext_0xc4 = text.clone();
    crate::game::board_parts::FUN_005346d0(g, label, &text);

    let r = app_rand(g, param_1);
    g.store(this).ext_0xbac = 0;
    g.store(this).ext_0x1cc = (r % 0x96 + 100) as i32;
    crate::game::win_fish_app::FUN_0054b020(g, param_1);
    crate::game::win_fish_app::FUN_0054b1a0(g, param_1, 2, 0, false);
    g.store(this).field_0x30 = NULL;

    // The overlay: a plain Widget with StoreScreenOverlay's vftable and the store at +0x88.
    let (mut owc, mut ow) = crate::sexy::widget::Widget();
    ow.offset_0x1 = false;
    owc.offset_0x3c = true;
    owc.offset_0x34 = 0x280;
    owc.offset_0x38 = 0x1e0;
    let overlay = g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::StoreScreenOverlay_vftable),
        node: Node::Widget(WidgetObj { wc: owc, w: ow, ext: WExt::GameSelectorOverlay(this) }),
    });
    g.store(this).offset_0x8 = overlay;
    this
}

/// port: 00519520 Sexy::StoreScreen::~StoreScreen
pub fn dtor_StoreScreen(g: &mut G, this: Ptr) {
    let d = g.store(this).clone();
    for b in d.offset_0x10 {
        if b != NULL {
            vcall!(g, b, w.vfunction1, 1);
        }
    }
    for p in [d.offset_0xc, d.offset_0x34, d.offset_0x8] {
        if p != NULL {
            vcall!(g, p, w.vfunction1, 1);
        }
    }
    crate::sexy::widget::dtor_Widget(g, this);
}

/// port: 0051b230 Sexy::StoreScreen::deleting_destructor
pub fn deleting_destructor(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_StoreScreen(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 0052ff90 Sexy::StoreScreen::vfunction21
/// `AddedToManager(WidgetManager*)`: restocks for today, then adds the shelves, the
/// overlay, the "Back" button and the shell count.
pub fn vfunction21(g: &mut G, this: Ptr, param_1: Ptr) {
    crate::sexy::widget_container::vfunction21(g, this, param_1);
    FUN_0052fea0(g, this, 0);
    let d = g.store(this).clone();
    for b in d.offset_0x10 {
        vcall!(g, param_1, w.vfunction4, b);
    }
    vcall!(g, param_1, w.vfunction4, d.offset_0x8);
    vcall!(g, param_1, w.vfunction4, d.offset_0xc);
    vcall!(g, param_1, w.vfunction4, d.offset_0x34);
}

/// port: 0051a050 Sexy::StoreScreen::vfunction22
/// `RemovedFromManager(WidgetManager*)`.
pub fn vfunction22(g: &mut G, this: Ptr, param_1: Ptr) {
    crate::sexy::widget_container::vfunction22(g, this, param_1);
    let d = g.store(this).clone();
    for b in d.offset_0x10 {
        vcall!(g, param_1, w.vfunction5, b);
    }
    vcall!(g, param_1, w.vfunction5, d.offset_0x8);
    vcall!(g, param_1, w.vfunction5, d.offset_0xc);
    vcall!(g, param_1, w.vfunction5, d.offset_0x34);
}

/// port: 0051a0c0 Sexy::StoreScreen::vfunction31
/// `OrderInManagerChanged()`: brings the shelves, the overlay, the "Back" button and the
/// shell count to the front.
pub fn vfunction31(g: &mut G, this: Ptr) {
    let wm = g.wc(this).offset_0xc;
    let d = g.store(this).clone();
    for b in d.offset_0x10 {
        vcall!(g, wm, w.vfunction12, b);
    }
    vcall!(g, wm, w.vfunction12, d.offset_0x8);
    vcall!(g, wm, w.vfunction12, d.offset_0xc);
    vcall!(g, wm, w.vfunction12, d.offset_0x34);
}

/// port: 0051a120 Sexy::StoreScreen::vfunction53
/// `MouseMove(int x, int y)`: while the thanks bubble shows, moving shortens it (by 3) and
/// nothing is hovered; else the shelf under the mouse.
pub fn vfunction53(g: &mut G, this: Ptr, param_1: i32, param_2: i32) {
    let t = g.store(this).ext_0xba0;
    if 0 < t {
        let t = t - 3;
        g.store(this).ext_0xba0 = if t < 0 { 0 } else { t };
        g.store(this).ext_0xba8 = -1;
        return;
    }
    let b = g.store(this).offset_0x10;
    for (i, &p) in b.iter().enumerate() {
        if vcall!(g, p, w.vfunction69, param_1, param_2) {
            g.store(this).ext_0xba8 = i as i32;
            return;
        }
    }
    g.store(this).ext_0xba8 = -1;
}

/// port: 0051a1b0 Sexy::StoreScreen::vfunction23
/// `Update()`: the shopkeeper's timer (restarting at 15..29 one time in seven, else
/// 100..249, from the app's RNG); redraw; the hover time; the thanks bubble counts down
/// and closes once a shelf is hovered.
pub fn vfunction23(g: &mut G, this: Ptr) {
    g.store(this).ext_0x1cc -= 1;
    if g.store(this).ext_0x1cc < 1 {
        let app = g.store(this).offset_0x0;
        let t = if app_rand(g, app) % 7 == 0 { app_rand(g, app) % 0xf + 0xf } else { app_rand(g, app) % 0x96 + 100 };
        g.store(this).ext_0x1cc = t as i32;
    }
    vcall!(g, this, w.vfunction18);
    let d = g.store(this);
    let t = d.ext_0xba0;
    d.ext_0xbac += 1;
    if t != 0 {
        if -1 < d.ext_0xba8 {
            d.ext_0xba0 = 0;
            return;
        }
        d.ext_0xba0 = t - 1;
    }
}

/// port: 0051a260 Sexy::StoreScreen::vfunction51
/// `MouseEnter()` (back from a dialog): un-highlights the shelf that was being bought from.
/// (The base call, @ 00486bf0, does nothing.)
pub fn vfunction51(g: &mut G, this: Ptr) {
    let b = g.store(this).field_0x30;
    if b != NULL {
        FUN_00532ec0(g, b, false);
        g.store(this).field_0x30 = NULL;
    }
}

/// port: 0052bb50 Sexy::StoreScreen::vfunction45
/// `DrawOverlay(Graphics*)`: the store backdrop, every shelf's price, the shopkeeper, and
/// the speech bubble: the thanks line, or the hovered shelf's line once it has been hovered
/// for 16 updates (moved down for short text, up for long).
pub fn vfunction45(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    let bg = g.res.DAT_005e8cd8;
    FUN_00455d20(gfx, g, bg, 0, 0);
    let b = g.store(this).offset_0x10;
    for p in b {
        let (x, y) = (g.wc(p).offset_0x2c, g.wc(p).offset_0x30);
        FUN_004563d0(gfx, x, y);
        vcall!(g, p, w.vfunction45, gfx);
        FUN_004563d0(gfx, -x, -y);
    }
    let keeper = g.res.DAT_005e8ad4;
    let t = g.store(this).ext_0x1cc;
    FUN_00456a00(gfx, g, keeper, 0xff, 0x121, t);
    let d = g.store(this).clone();
    let hover = d.ext_0xba8;
    if !((-1 < hover && hover < 8 && 0xf < d.ext_0xbac) || d.ext_0xba0 != 0) {
        return;
    }
    let text = if d.ext_0xba0 == 0 {
        match FUN_0051b9e0(g, d.offset_0x10[hover as usize]) {
            Some(t) => t,
            None => return,
        }
    } else {
        b"Thanks for shopping at\nthe Fish Emporium!".to_vec()
    };
    let lines = text.iter().filter(|&&c| c == b'\n').count() as i32;
    let bubble = g.res.DAT_005e8c18;
    FUN_00455d20(gfx, g, bubble, 0x186, 0xdc);
    FUN_00455890(gfx, CRect(0, 0, 0, 0xff));
    let f = g.res.DAT_005e8ea8;
    FUN_00455880(gfx, f);
    let mut r = Rect::new(0x189, 0xeb, 200, 300);
    if lines < 1 {
        r.mY = 0xfa;
    }
    if lines < 2 {
        r.mY += 10;
    } else if 2 < lines {
        r.mY -= 5;
    }
    vcall!(g, this, w.vfunction65, gfx, r, &text, 0x14, 0);
}

/// port: 0052bf90 Sexy::StoreScreen::vfunction3
/// `ButtonDepress(int id)`: "Back" (99) leaves the store (into the virtual tank when +0xba4
/// is set), saving the tank and unpausing it. A shelf: "Not Enough Shells" when the price is
/// over the player's shells; else the shelf is highlighted and a creature asks for a name
/// (or says the tank is full), anything else asks to confirm the purchase.
pub fn vfunction3(g: &mut G, this: Ptr, param_1: i32) {
    let app = g.store(this).offset_0x0;
    if param_1 == 99 {
        if g.store(this).ext_0xba4 {
            FUN_0054aff0(g, app);
        }
        let board = g.wfa(app).offset_0x4;
        crate::game::board_save::FUN_00538940(g, board);
        FUN_0054ad10(g, app);
        let board = g.wfa(app).offset_0x4;
        crate::game::board_update::FUN_0053db80(g, board, false);
        return;
    }
    if 8 <= param_1 {
        return;
    }
    let btn = g.store(this).offset_0x10[param_1 as usize];
    if btn == NULL {
        return;
    }
    let profile = g.wfa(app).offset_0x18c;
    let shells = g.profile(profile).field_0x48;
    if shells < g.store_button(btn).offset_0x8 {
        let what = match g.store_button(btn).offset_0x2c {
            1 => "this fish",
            2 => "the Bubbulator",
            3 => "this backdrop",
            4 => "the Alien Attractor",
            5 => "this upgrade",
            _ => "item",
        };
        let msg = format!("Sorry, but you need more Shells to purchase {what}.\n\nKeep playing to earn more!");
        crate::game::win_fish_app::vfunction73(g, app, 0xe, true, b"Not Enough Shells", msg.as_bytes(), b"OK", 3);
        return;
    }
    g.store(this).field_0x30 = btn;
    FUN_00532ec0(g, btn, true);
    let item = FUN_00531990(g, btn, false);
    if item == NULL || g.store_button(btn).offset_0x2c != 1 {
        let msg: &[u8] = match g.store_button(btn).offset_0x2c {
            2 => b"Would you like to\nbuy The Bubbulator?",
            3 => b"Would you like to\nbuy this backdrop?",
            4 => b"Would you like to\nbuy the Alien Attractor?",
            5 => b"Would you like to\nupgrade your fish food?",
            _ => return,
        };
        FUN_0054fdd0(g, app, msg);
        return;
    }
    let board = g.wfa(app).offset_0x4;
    let slot = FUN_0053a890(g, board);
    if slot < 0 {
        let lines: &[u8] = if g.profile(profile).field_0x90 == 0 {
            b"You'll need to free up some room in your tank before you can buy another fish.  You can either buy a bubbulator or sell a fish back to us."
        } else {
            b"You'll need to free up some room in your tank before you can buy another fish."
        };
        crate::game::win_fish_app::vfunction73(g, app, 0xe, true, b"Fishtank is Full", lines, b"OK", 3);
        return;
    }
    g.go(item).offset_0x24 = slot;
    let female = g.go(item).offset_0x4 == 10;
    let name = g.go(item).field_0x28.clone();
    FUN_0054c800(g, app, b"Please choose a name for your fish.", female, &name, true);
}

/// port: 0053a890 FUN_0053a890
/// The first free tank slot (+0xac of the tank's objects; 10 slots, 20 with the
/// Bubbulator), -1 when the tank is full.
pub fn FUN_0053a890(g: &mut G, this: Ptr) -> i32 {
    let profile = {
        let a = g.board(this).field_0x0;
        g.wfa(a).offset_0x18c
    };
    let size: i32 = if g.profile(profile).field_0x90 != 0 { 20 } else { 10 };
    let mut used = [false; 20];
    let objs = g.board(this).offset_0x7c.clone();
    let mut it = objs.first().copied();
    while let Some(o) = it {
        let s = g.go(o).offset_0x24;
        if -1 < s && s < size {
            used[s as usize] = true;
        }
        it = FUN_004e3e30(&objs, o);
    }
    for i in 0..size {
        if !used[i as usize] {
            return i;
        }
    }
    -1
}

/// port: 0054ad10 FUN_0054ad10
/// `RemoveStore()`: takes the store (+0x810) down and gives the board the focus back.
pub fn FUN_0054ad10(g: &mut G, this: Ptr) {
    let s = g.wfa(this).offset_0xe4;
    if s != NULL {
        let wm = g.sab(this).offset_0x318;
        vcall!(g, wm, w.vfunction5, s);
        crate::sexy::sexy_app_base::vfunction35(g, this, s);
        g.wfa(this).offset_0xe4 = NULL;
        let board = g.wfa(this).offset_0x4;
        if board != NULL {
            vcall!(g, wm, w.vfunction9, board);
        }
    }
}

/// port: 0054aff0 FUN_0054aff0
/// In the virtual tank (mode 5): `FUN_0054afd0`, then the board's `FUN_005497a0`.
pub fn FUN_0054aff0(g: &mut G, this: Ptr) {
    if g.wfa(this).offset_0x150 == 5 {
        crate::game::win_fish_app::FUN_0054afd0(g, this);
        let board = g.wfa(this).offset_0x4;
        if board != NULL {
            crate::game::board::FUN_005497a0(g, board);
        }
    }
}

/// port: 0054bca0 FUN_0054bca0
/// `ShowStore()`: closes the dialogs and any store, pauses the board, banks the profile's
/// screensaver shells, then opens the store full-screen with the focus.
pub fn FUN_0054bca0(g: &mut G, this: Ptr) {
    crate::game::win_fish_app::FUN_0054b4c0(g, this);
    FUN_0054ad10(g, this);
    let board = g.wfa(this).offset_0x4;
    if board != NULL {
        crate::game::board_update::FUN_0053db80(g, board, true);
    }
    let profile = g.wfa(this).offset_0x18c;
    if profile != NULL {
        crate::game::profile::FUN_00514b00(g, profile);
    }
    let s = StoreScreenOverlay(g, this);
    g.wfa(this).offset_0xe4 = s;
    let (w, h) = (g.sab(this).field_0xb8, g.sab(this).field_0xbc);
    vcall!(g, s, w.vfunction41, 0, 0, w, h);
    let wm = g.sab(this).offset_0x318;
    vcall!(g, wm, w.vfunction4, s);
    vcall!(g, wm, w.vfunction9, s);
}

/// port: 0054fdd0 FUN_0054fdd0
/// `ConfirmPurchase(const string& theLines)`: dialog 0x23 "Confirm Purchase" (yes/no) with
/// a "BUY" button, 350 wide at its place.
pub fn FUN_0054fdd0(g: &mut G, this: Ptr, param_1: &[u8]) {
    let d = crate::game::win_fish_app::vfunction73(g, this, 0x23, true, b"Confirm Purchase", param_1, b"", 2);
    let yes = g.dialog(d).offset_0x8;
    g.btn(yes).field_0x4 = b"BUY".to_vec();
    let h = vcall!(g, d, dlg.vfunction74, 0x15e);
    let (x, y) = (g.wc(d).offset_0x2c, g.wc(d).offset_0x30);
    vcall!(g, d, w.vfunction41, x, y, 0x15e, h);
}

/// port: 005331a0 Sexy::StoreButtonWidget::StoreButtonWidget
/// `StoreButtonWidget(int theId, ButtonListener*, Image*)`: the left half of the image is
/// the normal cel, the right half the over and down cels; price 100, label at (13, 108).
pub fn StoreButtonWidget(g: &mut G, param_1: i32, param_2: Ptr, param_3: Ptr) -> Ptr {
    let (mut wc, mut w, mut b) = crate::sexy::button_widget::ButtonWidget(param_1, param_2);
    let app = g.globals.DAT_005eb6a4;
    wc.offset_0x3d = false;
    w.offset_0x28 = true;
    b.offset_0x28 = param_3;
    let (iw, ih) = (g.image(param_3).offset_0x20, g.image(param_3).offset_0x24);
    let half = iw / 2;
    b.offset_0x38 = Rect::new(0, 0, half, ih);
    b.offset_0x48 = Rect::new(half, 0, half, ih);
    b.offset_0x58 = Rect::new(half, 0, half, ih);
    let d = StoreButtonWidget_data {
        offset_0x0: app,
        offset_0x4: NULL,
        offset_0x8: 100,
        field_0xc: Vec::new(),
        offset_0x28: false,
        offset_0x2c: 0,
        offset_0x30: 0xd,
        offset_0x34: 0x6c,
        offset_0x38: 0,
    };
    g.alloc(Obj {
        vt: Some(&crate::sexy::vtables_gen::Sexy__StoreButtonWidget_vftable),
        node: Node::Widget(WidgetObj { wc, w, ext: WExt::Button(Box::new(ButtonExt { b, sub: BtnSub::StoreButton(d) })) }),
    })
}

/// port: 00533290 Sexy::StoreButtonWidget::~StoreButtonWidget
pub fn dtor_StoreButtonWidget(g: &mut G, this: Ptr) {
    let item = g.store_button(this).offset_0x4;
    if item != NULL {
        vcall!(g, item, w.vfunction1, 1);
    }
    g.store_button(this).field_0xc.clear();
    crate::sexy::button_widget::dtor_ButtonWidget(g, this);
}

/// port: 00533390 Sexy::StoreButtonWidget::deleting_destructor
pub fn deleting_destructor__00533390(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_StoreButtonWidget(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 005319b0 Sexy::StoreButtonWidget::vfunction51
/// `MouseEnter()`: the store's hovered shelf is this one, hovered from now.
pub fn vfunction51__005319b0(g: &mut G, this: Ptr) {
    crate::sexy::button_widget::vfunction51(g, this);
    let app = g.store_button(this).offset_0x0;
    let s = g.wfa(app).offset_0xe4;
    if s != NULL {
        let id = g.btn(this).offset_0x0;
        g.store(s).ext_0xba8 = id;
        g.store(s).ext_0xbac = 0;
    }
}

/// port: 00531a00 Sexy::StoreButtonWidget::vfunction52
/// `MouseLeave()`: no shelf is hovered.
pub fn vfunction52__00531a00(g: &mut G, this: Ptr) {
    crate::sexy::button_widget::vfunction52(g, this);
    let app = g.store_button(this).offset_0x0;
    let s = g.wfa(app).offset_0xe4;
    if s != NULL {
        g.store(s).ext_0xba8 = -1;
    }
}

/// port: 00531a30 Sexy::StoreButtonWidget::vfunction23
/// `Update()`: the creature on the shelf animates (its virtual #79).
pub fn vfunction23__00531a30(g: &mut G, this: Ptr) {
    crate::sexy::widget_container::vfunction23(g, this);
    let item = g.store_button(this).offset_0x4;
    if item != NULL {
        vcall!(g, item, go.vfunction79);
    }
}

/// port: 00531a60 Sexy::StoreButtonWidget::vfunction27
/// `Draw(Graphics*)`: the button (shifted inside the tall and creature shelf images), then
/// what is on the shelf: the creature's icon (pose 0 for the tall shelves, 1 for the
/// creature shelves, 2 for the counter), the Bubbulator, the backdrop (darkened when
/// hovered, held or selected), the Alien Attractor, or the animated food upgrade.
pub fn vfunction27__00531a60(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    let img = g.btn(this).offset_0x28;
    let (mut dx, mut dy) = (0, 0);
    if img == g.res.DAT_005e8c5c {
        dy = 4;
        dx = 0xf;
    } else if img == g.res.DAT_005e8c78 {
        dy = 5;
    }
    let id = g.btn(this).offset_0x0;
    let mut pose = 0;
    if 2 < id {
        pose = (6 < id) as i32 + 1;
    }
    FUN_004563d0(gfx, dx, dy);
    crate::sexy::button_widget::vfunction27(g, this, gfx);
    FUN_004563d0(gfx, -dx, -dy);
    if !g.w(this).offset_0x1 {
        return;
    }
    let d = g.store_button(this).clone();
    match d.offset_0x2c {
        1 => {
            if d.offset_0x4 != NULL {
                vcall!(g, d.offset_0x4, go.vfunction80, gfx, pose);
            }
        }
        2 => {
            let i = g.res.DAT_005e8c80;
            FUN_00456950(gfx, g, i, 0x23, 0x17, 0);
        }
        3 => {
            let i = g.res.DAT_005e8a60;
            FUN_00456950(gfx, g, i, 0, 0, d.offset_0x38);
            let w = g.w(this).clone();
            if w.offset_0x5 || w.offset_0x4 || d.offset_0x28 {
                FUN_004558c0(gfx, 1);
                FUN_004558e0(gfx, true);
                FUN_00455890(gfx, FUN_00433360(0x32, 0x32, 0x32));
                FUN_00456950(gfx, g, i, 0, 0, d.offset_0x38);
                FUN_004558e0(gfx, false);
                FUN_004558c0(gfx, 0);
            }
        }
        4 => {
            let i = g.res.DAT_005e8c68;
            FUN_00455d20(gfx, g, i, 0x1e, 0x14);
        }
        5 => {
            let n = g.wc(this).offset_0x24;
            let i = g.res.DAT_005e8aa0;
            FUN_00456980(gfx, g, i, 0x28, 0x28, (n / 4) % 10, 2);
        }
        _ => {}
    }
}

/// port: 00531c30 Sexy::StoreButtonWidget::vfunction45
/// `DrawOverlay(Graphics*)` (called by the store, translated to the shelf): the price in
/// green after a shell, or grey without the shell when sold out; a glow over the shelf
/// when hovered, held or selected.
pub fn vfunction45__00531c30(g: &mut G, this: Ptr, gfx: &mut Graphics) {
    let f = g.res.DAT_005e8cd0;
    FUN_00455880(gfx, f);
    FUN_00455890(gfx, FUN_00433360(0x6e, 0xfa, 0x6e));
    let d = g.store_button(this).clone();
    let sw = font::string_width(g, f, &d.field_0xc);
    let x0 = d.offset_0x30 - sw / 2;
    let mut tx = x0 + 0x2d;
    if !g.w(this).offset_0x1 {
        FUN_00455890(gfx, FUN_00433320(0xcccccc));
        tx = x0 + 0x28;
    } else {
        let shell = g.res.DAT_005e8c3c;
        FUN_00455d20(gfx, g, shell, x0 + 0x19, d.offset_0x34 + 3);
    }
    let asc = font::get_ascent(g, f);
    FUN_00455cf0(gfx, g, &d.field_0xc, tx, asc + d.offset_0x34);
    let w = g.w(this).clone();
    if w.offset_0x5 || w.offset_0x4 || d.offset_0x28 {
        FUN_00455890(gfx, FUN_00433360(0xf0, 0xff, 0xc1));
        FUN_004558e0(gfx, true);
        let img = g.btn(this).offset_0x28;
        let (glow, x, y) = if img == g.res.DAT_005e8c5c {
            (g.res.DAT_005e8d18, 4, 4)
        } else if img == g.res.DAT_005e8c78 {
            (g.res.DAT_005e8c44, -7, -2)
        } else {
            (g.res.DAT_005e8b38, -4, -3)
        };
        FUN_00455d20(gfx, g, glow, x, y);
        FUN_004558e0(gfx, false);
    }
}

/// port: 00531990 FUN_00531990
/// The creature on the shelf; `param_1` takes it off (the shelf no longer owns it).
pub fn FUN_00531990(g: &mut G, this: Ptr, param_1: bool) -> Ptr {
    let item = g.store_button(this).offset_0x4;
    if param_1 {
        g.store_button(this).offset_0x4 = NULL;
    }
    item
}

/// port: 00532ec0 FUN_00532ec0
/// `SetSelected(bool)`: swaps the normal and over cels.
pub fn FUN_00532ec0(g: &mut G, this: Ptr, param_1: bool) {
    if param_1 != g.store_button(this).offset_0x28 {
        g.store_button(this).offset_0x28 = param_1;
        let b = g.btn(this);
        std::mem::swap(&mut b.offset_0x38, &mut b.offset_0x48);
    }
}

/// port: 00536350 FUN_00536350
/// `SetPrice(int)`: the price and its text.
pub fn FUN_00536350(g: &mut G, this: Ptr, param_1: i32) {
    let d = g.store_button(this);
    d.offset_0x8 = param_1;
    d.field_0xc = format!("{param_1}").into_bytes();
}

/// port: 005363d0 FUN_005363d0
/// Shows the shelf for sale (mouse-visible, price text).
pub fn FUN_005363d0(g: &mut G, this: Ptr) {
    g.w(this).offset_0x1 = true;
    let p = g.store_button(this).offset_0x8;
    FUN_00536350(g, this, p);
}

/// port: 00537230 FUN_00537230
/// `SetItem(int kind, int param, int price)`: a service item or backdrop (any creature
/// is deleted unless `kind` is 1), shown for sale.
pub fn FUN_00537230(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: i32) {
    if param_1 != 1 {
        let item = g.store_button(this).offset_0x4;
        if item != NULL {
            vcall!(g, item, w.vfunction1, 1);
        }
        g.store_button(this).offset_0x4 = NULL;
    }
    g.store_button(this).offset_0x2c = param_1;
    g.store_button(this).offset_0x38 = param_2;
    FUN_00536350(g, this, param_3);
    FUN_005363d0(g, this);
}

/// port: 00537290 FUN_00537290
/// `SetCreature(GameObject*, int price)`: replaces (deletes) the creature on the shelf; a
/// creature is priced and the shelf shows it, none leaves the shelf empty (kind 0).
pub fn FUN_00537290(g: &mut G, this: Ptr, param_1: Ptr, param_2: i32) {
    let old = g.store_button(this).offset_0x4;
    if old != NULL {
        vcall!(g, old, w.vfunction1, 1);
    }
    g.store_button(this).offset_0x4 = param_1;
    if param_1 != NULL {
        g.w(this).offset_0x1 = true;
        g.go(param_1).field_0x58 = param_2;
        g.store_button(this).offset_0x2c = 1;
        FUN_00536350(g, this, param_2);
        return;
    }
    g.store_button(this).offset_0x2c = 0;
    FUN_00536350(g, this, param_2);
}

/// port: 005372f0 FUN_005372f0
/// `SetSoldOut()`: empty, unselected, not clickable, "SOLD".
pub fn FUN_005372f0(g: &mut G, this: Ptr) {
    FUN_00537290(g, this, NULL, 0);
    FUN_00532ec0(g, this, false);
    g.w(this).offset_0x1 = false;
    g.store_button(this).field_0xc = b"SOLD".to_vec();
}

/// port: 0054c800 FUN_0054c800
/// `ShowFishNamingDialog(const string& theLines, bool isFemale, const string& theName, bool
/// showNote)`: the naming dialog with the current name selected, 400 wide (lower and to the
/// right while the store is up), as dialog 0xf. (`param_2` is not read by the original.)
pub fn FUN_0054c800(g: &mut G, this: Ptr, param_1: &[u8], param_2: bool, param_3: &[u8], param_4: bool) {
    let _ = param_2;
    let d = crate::game::money_dialog::FUN_00533a40(g, this, param_1);
    g.fish_naming(d).ext_0x168 = param_4;
    let edit = g.new_user(d).offset_0x4;
    vcall!(g, edit, edit.vfunction74, param_3, true);
    crate::sexy::edit_widget::FUN_00473730(g, edit);
    let len = g.edit(edit).field_0x4.len() as i32;
    g.edit(edit).offset_0x54 = len;
    g.edit(edit).offset_0x58 = 0;
    let h = vcall!(g, d, dlg.vfunction74, 400);
    if g.wfa(this).offset_0xe4 == NULL {
        vcall!(g, d, w.vfunction41, 0x78, 0x96, 400, h);
    } else {
        vcall!(g, d, w.vfunction41, 0xda, 200, 400, h);
    }
    crate::sexy::sexy_app_base::vfunction76(g, this, 0xf, d);
}

/// port: 0052bd70 FUN_0052bd70
/// `CompletePurchase()`: starts the thanks bubble (108 updates) and, if a shelf is being
/// bought from, marks it sold today, takes the price from the shells (not below 0), updates
/// the shell count, and delivers: a creature is taken off the shelf with its purchase time
/// and fresh mood (and returned for the caller to name and add); the Bubbulator, a
/// backdrop, the Alien Attractor and the food upgrade take effect at once. The shelf then
/// shows "SOLD". Returns the creature, else null.
pub fn FUN_0052bd70(g: &mut G, this: Ptr) -> Ptr {
    g.store(this).ext_0xba0 = 0x6c;
    let btn = g.store(this).field_0x30;
    if btn == NULL {
        return NULL;
    }
    g.store(this).ext_0xba4 = true;
    let app = g.store(this).offset_0x0;
    let profile = g.wfa(app).offset_0x18c;
    let id = g.btn(btn).offset_0x0;
    g.profile(profile).field_0xa8[id as usize] = true;
    let price = g.store_button(btn).offset_0x8;
    crate::game::profile::FUN_00501200(g, profile, -price);
    if g.profile(profile).field_0x48 < 0 {
        g.profile(profile).field_0x48 = 0;
    }
    let text = format!("{}", g.profile(profile).field_0x48).into_bytes();
    g.store(this).ext_0xc4 = text.clone();
    let label = g.store(this).offset_0x34;
    crate::game::board_parts::FUN_005346d0(g, label, &text);
    let board = g.wfa(app).offset_0x4;
    match g.store_button(btn).offset_0x2c {
        1 => {
            let item = FUN_00531990(g, btn, true);
            if item != NULL {
                let now = g.now_time64;
                g.go(item).field_0x48 = now;
                crate::game::game_object::FUN_004d6720(g, item);
                FUN_005372f0(g, btn);
                return item;
            }
        }
        2 => {
            g.profile(profile).field_0x90 = 1;
            g.board(board).ext_0x4fd = true;
            g.board(board).ext_0x448 = 0;
        }
        3 => {
            let n = g.store_button(btn).offset_0x38 as u32;
            if n < 6 {
                g.profile(profile).field_0x72[n as usize] = true;
                crate::game::board_level::FUN_00538a10(g, board, n as i32 + 1);
            }
        }
        4 => {
            g.profile(profile).field_0x94 = 1;
            g.board(board).ext_0x4fe = true;
        }
        5 => g.globals.DAT_005e89dc = 2,
        _ => {}
    }
    FUN_005372f0(g, btn);
    NULL
}

/// port: 00544940 FUN_00544940
/// `AddCreature(GameObject*, bool placeRandomly)`: optionally at a random spot (from the
/// app's RNG), then into the board's objects and onto the widget manager, then its setup.
pub fn FUN_00544940(g: &mut G, this: Ptr, param_1: Ptr, param_2: bool) {
    if param_2 {
        let app = g.board(this).field_0x0;
        let x = app_rand(g, app) % 0x208 + 0x14;
        let y = app_rand(g, app) % 0x109 + 0x69;
        vcall!(g, param_1, go.vfunction77, x as i32, y as i32);
    }
    crate::game::board_level::FUN_00542ee0(g, this, param_1, 1);
    let wm = g.wc(this).offset_0xc;
    vcall!(g, wm, w.vfunction4, param_1);
    crate::game::board_level::FUN_00544800(g, this, param_1);
}

/// port: 0054cc70 FUN_0054cc70
/// Name cheats for a bought creature: "1SingingFish" makes it sing (+0x110); "santa" turns a
/// plain guppy into the Santa fish (special 5: white, red and white layers, singing, size
/// 1) unless the tank already has one.
pub fn FUN_0054cc70(g: &mut G, this: Ptr, param_1: Ptr) {
    let name = g.go(param_1).field_0x28.clone();
    if name.eq_ignore_ascii_case(b"1SingingFish") {
        g.go(param_1).offset_0x88 = true;
        return;
    }
    if !(name.eq_ignore_ascii_case(b"santa") && g.go(param_1).offset_0x4 == 0) {
        return;
    }
    let board = g.wfa(this).offset_0x4;
    let objs = g.board(board).offset_0x7c.clone();
    let mut it = objs.first().copied();
    while let Some(o) = it {
        if g.go(o).offset_0x9c == 5 {
            return;
        }
        it = FUN_004e3e30(&objs, o);
    }
    let f = g.fish(param_1).clone();
    if !f.field_0x7c && !f.field_0x7d && g.go(param_1).offset_0x9c == -1 {
        g.go(param_1).offset_0x9c = 5;
        let d = g.fish(param_1);
        d.field_0x7c = true;
        d.field_0x84 = FUN_00433320(0xffffff);
        d.field_0x94 = FUN_00433320(0xff0000);
        d.field_0xa4 = FUN_00433320(0xffffff);
        g.go(param_1).offset_0x88 = true;
        g.fish(param_1).field_0x54 = 1;
    }
}

/// port: 0054d040 FUN_0054d040
/// `Trim(string&, const string& chars)`: strips the characters from both ends.
pub fn FUN_0054d040(param_1: &mut Vec<u8>, param_2: &[u8]) {
    let last = FUN_0054c910(param_1, param_2, -1);
    param_1.truncate((last + 1) as usize);
    let first = param_1.iter().position(|c| !param_2.contains(c)).unwrap_or(param_1.len());
    param_1.drain(..first);
}

/// port: 0054c910 FUN_0054c910
/// `std::string::find_last_not_of(const char*, size_type pos, size_type n)` (an STL
/// instance): the last index at or before `param_3` not in the set, -1 when none.
pub fn FUN_0054c910(this: &[u8], param_1: &[u8], param_3: i32) -> i32 {
    if this.is_empty() {
        return -1;
    }
    let mut i = if param_3 < 0 || param_3 as usize >= this.len() { this.len() - 1 } else { param_3 as usize };
    loop {
        if !param_1.contains(&this[i]) {
            return i as i32;
        }
        if i == 0 {
            return -1;
        }
        i -= 1;
    }
}

/// port: 0054eec0 FUN_0054eec0
/// OK in "Name Your Fish": the name, trimmed of white space, must not be empty ("Invalid
/// Name"); else the dialog closes and, in the store, the purchase completes: the creature
/// gets the name, the name cheats, and joins the tank at a random spot, and the store comes
/// back to the front; from the tank screen (+0x828) the name goes to its selected fish.
pub fn FUN_0054eec0(g: &mut G, this: Ptr) {
    let d = crate::sexy::sexy_app_base::vfunction74(g, this, 0xf);
    let edit = g.new_user(d).offset_0x4;
    let mut name = g.edit(edit).field_0x4.clone();
    FUN_0054d040(&mut name, b" \t\r\n");
    if name.is_empty() {
        crate::game::win_fish_app::vfunction73(g, this, 0xe, true, b"Invalid Name", b"Names must be one or more letters in length", b"OK", 3);
        return;
    }
    crate::game::win_fish_app::vfunction78(g, this, 0xf);
    let front;
    let store = g.wfa(this).offset_0xe4;
    if store != NULL {
        let fish = FUN_0052bd70(g, store);
        if fish == NULL {
            return;
        }
        g.go(fish).field_0x28 = name;
        FUN_0054cc70(g, this, fish);
        let board = g.wfa(this).offset_0x4;
        FUN_00544940(g, board, fish, true);
        front = g.wfa(this).offset_0xe4;
    } else {
        let screen = g.wfa(this).offset_0xfc;
        if screen == NULL {
            return;
        }
        crate::game::sim_fish::FUN_00529da0(g, screen, &name);
        front = g.wfa(this).offset_0xfc;
    }
    let wm = g.sab(this).offset_0x318;
    vcall!(g, wm, w.vfunction12, front);
}

/// port: 0054b900 FUN_0054b900
/// "Confirm Purchase" closed: on yes, completes the purchase in the store (+0x810), else in
/// the bonus screen (+0x818).
pub fn FUN_0054b900(g: &mut G, this: Ptr, param_1: bool) {
    crate::game::win_fish_app::vfunction78(g, this, 0x23);
    if param_1 {
        let store = g.wfa(this).offset_0xe4;
        if store != NULL {
            FUN_0052bd70(g, store);
            return;
        }
        let bonus = g.wfa(this).offset_0xec;
        if bonus != NULL {
            crate::game::bonus_screen::FUN_005172e0(g, bonus);
        }
    }
}
