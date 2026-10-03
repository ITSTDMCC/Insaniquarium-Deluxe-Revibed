//! The player profile (`PlayerInfo`, 0x12c bytes): the value type of `ProfileMgr`'s
//! name -> profile map; its summary (name, use order, id) is saved in `userdata\users.dat`,
//! the rest in `userdata\user%d.dat`. The database has no type for it; fields are named by
//! their offset from the profile pointer (`app->mProfile`, WinFishApp +0x8b8).

use crate::sexy::data_sync::{DataSync, SyncResult, FUN_00503010, FUN_00503070, FUN_00505860};
use crate::sexy::prelude::*;

/// A player profile (offsets from the profile pointer).
#[derive(Debug, Clone, Default)]
pub struct PlayerProfile {
    /// +0x00: tank levels finished (4 tanks x 6 levels).
    pub field_0x0: [bool; 24],
    /// +0x18: how many of +0x00 are set.
    pub field_0x18: i32,
    /// +0x1c: adventure tank (1..5).
    pub field_0x1c: i32,
    /// +0x20: level within the tank (1..5, 6 = bonus level).
    pub field_0x20: i32,
    /// +0x24: the player's name (std::string).
    pub field_0x24: Vec<u8>,
    /// +0x40: last-used sequence number (most recent profile = highest).
    pub field_0x40: i32,
    /// +0x44: profile id (the N of `user%d.dat`).
    pub field_0x44: i32,
    /// +0x48: shells (virtual tank currency; 200 for a new player).
    pub field_0x48: i32,
    /// +0x4c: games played (0 = new player).
    pub field_0x4c: i32,
    /// +0x50: times the final boss was beaten.
    pub field_0x50: i32,
    /// +0x54.
    pub field_0x54: i32,
    /// +0x58.
    pub field_0x58: bool,
    /// +0x59: adventure mode finished.
    pub field_0x59: bool,
    /// +0x5a: 24 flags.
    pub field_0x5a: [bool; 24],
    /// +0x72: 6 flags (the first set for a new player).
    pub field_0x72: [bool; 6],
    /// +0x78: 4 flags.
    pub field_0x78: [bool; 4],
    /// +0x7c.
    pub field_0x7c: i32,
    /// +0x80: the gold trophy (all challenge/time-trial levels finished).
    pub field_0x80: i32,
    /// +0x84: bit flags (`FUN_005014d0`).
    pub field_0x84: u32,
    /// +0x88: random key (checks `scr%d.dat`).
    pub field_0x88: i32,
    /// +0x8c.
    pub field_0x8c: i32,
    /// +0x90.
    pub field_0x90: i32,
    /// +0x94.
    pub field_0x94: i32,
    /// +0x98: random.
    pub field_0x98: i32,
    /// +0xa0.
    pub field_0xa0: i32,
    /// +0xa4.
    pub field_0xa4: i32,
    /// +0xa8: 8 flags.
    pub field_0xa8: [bool; 8],
    /// +0xb0: trophy progress (a trophy shows from 6).
    pub field_0xb0: i32,
    /// +0xb4: virtual-tank unlock level (3 for a new player).
    pub field_0xb4: i32,
    /// +0xb8: 21 ints (-1 = unset).
    pub field_0xb8: [i32; 21],
    /// +0x10c: 4 ints (-1 = unset).
    pub field_0x10c: [i32; 4],
    /// +0x11c: 4 ints (-1 = unset).
    pub field_0x11c: [i32; 4],
}

impl G {
    pub fn profile(&mut self, p: Ptr) -> &mut PlayerProfile {
        match &mut self.obj(p).node {
            Node::Profile(pr) => pr,
            n => panic!("{p} is not a profile: {n:?}"),
        }
    }
}

/// port: 00501530 FUN_00501530
/// `PlayerInfo::Reset()`: a new player's state (tank 1-1, 200 shells, ...). The two random
/// fields come from `Sexy::Rand()`.
pub fn FUN_00501530(g: &mut G, this: &mut PlayerProfile) {
    this.field_0x1c = 1;
    this.field_0x20 = 1;
    this.field_0x18 = 0;
    this.field_0x4c = 0;
    this.field_0x50 = 0;
    this.field_0x54 = 0;
    this.field_0xb4 = 3;
    this.field_0x84 = 0;
    FUN_005014d0(this, 1, true);
    this.field_0x8c = 0;
    this.field_0x90 = 0;
    this.field_0x94 = 0;
    this.field_0x98 = crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32;
    this.field_0x88 = crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32;
    this.field_0x78 = [false; 4];
    this.field_0x7c = 0;
    this.field_0x80 = 0;
    for i in 0..0x18 {
        this.field_0x5a[i] = false;
        this.field_0x0[i] = false;
    }
    this.field_0x72 = [false; 6];
    this.field_0xa0 = 0;
    this.field_0xa4 = 0;
    this.field_0x72[0] = true;
    this.field_0xa8 = [false; 8];
    this.field_0xb0 = 0;
    this.field_0x48 = 200;
    this.field_0xb8 = [-1; 21];
    this.field_0x10c = [-1; 4];
    this.field_0x11c = [-1; 4];
    this.field_0x59 = false;
    this.field_0x58 = false;
}

/// port: 00504e00 FUN_00504e00
/// `PlayerInfo::PlayerInfo()`: empty name, then `Reset()`.
pub fn FUN_00504e00(g: &mut G) -> PlayerProfile {
    let mut p = PlayerProfile::default();
    FUN_00501530(g, &mut p);
    p
}

/// port: 005014d0 FUN_005014d0
/// Sets or clears bit `param_1` of +0x84.
pub fn FUN_005014d0(this: &mut PlayerProfile, param_1: u8, param_2: bool) {
    let bit = 1u32 << (param_1 & 0x1f);
    if param_2 {
        this.field_0x84 |= bit;
        return;
    }
    this.field_0x84 &= !bit;
}

/// port: 00501420 FUN_00501420
/// Sets tank-level flag `param_1`, keeping the count at +0x18.
pub fn FUN_00501420(this: &mut PlayerProfile, param_1: usize, param_2: bool) {
    if this.field_0x0[param_1] != param_2 {
        this.field_0x0[param_1] = param_2;
        if !param_2 {
            this.field_0x18 -= 1;
            return;
        }
        this.field_0x18 += 1;
    }
}

/// port: 00501450 FUN_00501450
/// Marks every level before the current one finished (all 20 once adventure is beaten).
pub fn FUN_00501450(this: &mut PlayerProfile) {
    let mut n = this.field_0x1c * 5 + -6 + this.field_0x20;
    if 4 < this.field_0x1c {
        n -= 1;
    }
    if this.field_0x59 {
        n = 0x14;
    }
    let mut i = 0;
    while i < n {
        FUN_00501420(this, i as usize, true);
        i += 1;
    }
}

/// port: 00505990 FUN_00505990
/// `PlayerInfo::SyncSummary(DataSync&)`: name, use order, id (the `users.dat` record).
pub fn FUN_00505990(this: &mut PlayerProfile, sync: &mut DataSync) -> SyncResult {
    FUN_00505860(sync, &mut this.field_0x24)?;
    FUN_00503010(sync, &mut this.field_0x40)?;
    FUN_00503010(sync, &mut this.field_0x44)
}

/// port: 00503730 FUN_00503730
/// `PlayerInfo::SyncDetails(DataSync&)`: the `user%d.dat` record. A file starting with a
/// value below 0x7fffffff is the older format without the version word: the reader steps
/// back over it, the four +0x78 flags are ints and three fields are missing. After a load
/// the finished-level flags are rebuilt and counted.
pub fn FUN_00503730(g: &mut G, this: &mut PlayerProfile, sync: &mut DataSync) -> SyncResult {
    let reading = matches!(sync.io, crate::sexy::data_sync::DataIo::Read(_));
    if reading {
        FUN_00501530(g, this);
    }
    let mut old_format = false;
    let mut version: i32 = 0x80000000u32 as i32;
    FUN_00503010(sync, &mut version)?;
    if (version as u32) < 0x7fffffff {
        if let crate::sexy::data_sync::DataIo::Read(r) = &mut sync.io {
            old_format = true;
            crate::sexy::data_sync::DataReaderException__00500230(r, 4)?;
        }
    }
    FUN_00503010(sync, &mut this.field_0x48)?;
    FUN_00503010(sync, &mut this.field_0x1c)?;
    FUN_00503010(sync, &mut this.field_0x20)?;
    FUN_00503070(sync, &mut this.field_0x59)?;
    FUN_00503070(sync, &mut this.field_0x58)?;
    FUN_00503010(sync, &mut this.field_0x90)?;
    FUN_00503010(sync, &mut this.field_0x94)?;
    FUN_00503010(sync, &mut this.field_0x8c)?;
    FUN_00503010(sync, &mut this.field_0x4c)?;
    FUN_00503010(sync, &mut this.field_0x50)?;
    FUN_00503010(sync, &mut this.field_0x54)?;
    if reading {
        this.field_0xa0 = 0;
        this.field_0xa4 = 0;
    }
    FUN_00503010(sync, &mut this.field_0xa0)?;
    FUN_00503010(sync, &mut this.field_0x98)?;
    FUN_00503010(sync, &mut this.field_0xb0)?;
    FUN_00503010(sync, &mut this.field_0xb4)?;
    for i in 0..0x18 {
        FUN_00503070(sync, &mut this.field_0x0[i])?;
    }
    for i in 0..6 {
        FUN_00503070(sync, &mut this.field_0x72[i])?;
    }
    for i in 0..8 {
        FUN_00503070(sync, &mut this.field_0xa8[i])?;
    }
    for i in 0..4 {
        if old_format {
            let mut v: i32 = this.field_0x78[i] as i32;
            FUN_00503010(sync, &mut v)?;
            this.field_0x78[i] = v != 0;
        } else {
            FUN_00503070(sync, &mut this.field_0x78[i])?;
        }
        FUN_00503010(sync, &mut this.field_0x11c[i])?;
        FUN_00503010(sync, &mut this.field_0x10c[i])?;
    }
    if !old_format {
        FUN_00503010(sync, &mut this.field_0x7c)?;
        FUN_00503010(sync, &mut this.field_0x80)?;
        let mut flags = this.field_0x84 as i32;
        FUN_00503010(sync, &mut flags)?;
        this.field_0x84 = flags as u32;
    }
    for i in 0..0x15 {
        FUN_00503010(sync, &mut this.field_0xb8[i])?;
    }
    if 0x7fffffff < version as u32 {
        FUN_00503010(sync, &mut this.field_0x88)?;
    }
    if reading {
        FUN_00501450(this);
        this.field_0x18 = this.field_0x0.iter().filter(|&&b| b).count() as i32;
    }
    Ok(())
}

/// port: 005145a0 FUN_005145a0
/// `PlayerInfo::LoadDetails()`: reads `userdata\user%d.dat` when it exists, then sets flag
/// bit 6 off. A truncated file stops the read where it ends (the original's
/// DataReaderException unwinds to here).
pub fn FUN_005145a0(g: &mut G, this: Ptr) {
    let id = g.profile(this).field_0x44;
    let path = format!("userdata/user{id}.dat");
    let Some(bytes) = g.vfs.read(&path).map(|b| b.to_vec()) else { return };
    let mut sync = crate::sexy::data_sync::DataSync__00513e30(crate::sexy::data_sync::DataReader::from_bytes(bytes));
    let mut p = g.profile(this).clone();
    let r = FUN_00503730(g, &mut p, &mut sync);
    if r.is_ok() {
        FUN_005014d0(&mut p, 6, false);
    }
    *g.profile(this) = p;
}

/// port: 00514730 FUN_00514730
/// `PlayerInfo::SaveDetails()`: writes `userdata\user%d.dat` (the userdata folder is
/// created by the host when it writes the file).
pub fn FUN_00514730(g: &mut G, this: Ptr) {
    let mut sync = crate::sexy::data_sync::DataSync__00513ee0(crate::sexy::data_sync::DataWriter::default());
    let mut p = g.profile(this).clone();
    let _ = FUN_00503730(g, &mut p, &mut sync);
    *g.profile(this) = p;
    let id = g.profile(this).field_0x44;
    if let crate::sexy::data_sync::DataIo::Write(w) = sync.io {
        g.vfs.write(&format!("userdata/user{id}.dat"), w.data);
    }
}

/// port: 005013f0 FUN_005013f0
/// For the final-boss tank (5) past its first level, `FUN_005013d0`.
pub fn FUN_005013f0(g: &mut G, this: Ptr) {
    let p = g.profile(this);
    if p.field_0x1c == 5 && 1 < p.field_0x20 {
        FUN_005013d0(g, this);
    }
}

/// port: 00501200 FUN_00501200
/// Adds shells, clamped to 0..9,999,999.
pub fn FUN_00501200(g: &mut G, this: Ptr, param_1: i32) {
    let p = g.profile(this);
    p.field_0x48 = p.field_0x48.wrapping_add(param_1);
    if 9_999_999 < p.field_0x48 {
        p.field_0x48 = 9_999_999;
        return;
    }
    if p.field_0x48 < 0 {
        p.field_0x48 = 0;
    }
}

/// port: 005148c0 FUN_005148c0
/// Reads the shells the screensaver earned from `userdata\scr%d.dat` (deleting the file when
/// `param_1`); 0 when there is no file, its key does not match +0x88, or it is truncated.
pub fn FUN_005148c0(g: &mut G, this: Ptr, param_1: bool) -> i32 {
    let id = g.profile(this).field_0x44;
    let path = format!("userdata/scr{id}.dat");
    let Some(bytes) = g.vfs.read(&path).map(|b| b.to_vec()) else { return 0 };
    if param_1 {
        crate::sexy::app_host::FUN_0047fbc0(g, &path);
    }
    let mut r = crate::sexy::data_sync::DataReader::from_bytes(bytes);
    let key = g.profile(this).field_0x88;
    let read = |r: &mut crate::sexy::data_sync::DataReader| -> Result<i32, crate::sexy::data_sync::ReadError> {
        let k = crate::sexy::data_sync::FUN_00500340(r)?;
        if k != key {
            return Ok(0);
        }
        for _ in 0..0x32 {
            crate::sexy::data_sync::FUN_00500340(r)?;
        }
        let v = crate::sexy::data_sync::FUN_00500340(r)?;
        for _ in 0..0x32 {
            crate::sexy::data_sync::FUN_00500340(r)?;
        }
        Ok(v)
    };
    read(&mut r).unwrap_or(0)
}

/// port: 00514b00 FUN_00514b00
/// Banks the screensaver's shells (at most 100,000), re-keys the profile and saves it.
pub fn FUN_00514b00(g: &mut G, this: Ptr) {
    let mut n = FUN_005148c0(g, this, true);
    if n < 0x186a1 {
        if n < 1 {
            return;
        }
    } else {
        n = 100000;
    }
    FUN_00501200(g, this, n);
    let r = crate::sexy::sexy_app_base::FUN_0040f9e0(g);
    g.profile(this).field_0x88 = r as i32;
    FUN_00514730(g, this);
}

/// port: 00501410 FUN_00501410
/// Whether tank level `param_1` (0-based, tank-major) has been finished.
pub fn FUN_00501410(g: &mut G, this: Ptr, param_1: usize) -> bool {
    g.profile(this).field_0x0[param_1]
}

/// port: 00501370 FUN_00501370
/// `NextLevel()`: advances the adventure level (tank 4's fifth level leads to tank 5;
/// finishing tank 5 counts a playthrough and marks adventure finished), then the tank after
/// level 6 (5 once finished); remembers that progress changed.
pub fn FUN_00501370(this: &mut PlayerProfile) {
    this.field_0x20 += 1;
    if this.field_0x1c == 4 && this.field_0x20 == 6 {
        this.field_0x1c = 5;
        this.field_0x20 = 1;
    }
    if this.field_0x1c == 5 && 1 < this.field_0x20 {
        this.field_0x50 += 1;
        this.field_0x59 = true;
    }
    if 6 < this.field_0x20 || (this.field_0x59 && 5 < this.field_0x20) {
        this.field_0x1c += 1;
        this.field_0x20 = 1;
    }
    this.field_0x58 = true;
}

/// port: 00500dd0 FUN_00500dd0
/// The adventure level index (tank 1..4 levels 1..5, tank 5 level 1): tank*5 - 6 + level;
/// -1 otherwise.
pub fn FUN_00500dd0(param_1: i32, param_2: i32) -> i32 {
    if ((param_1 - 1) as u32) < 5 && ((param_2 - 1) as u32) < 5 && (param_1 != 5 || param_2 == 1) {
        return param_1 * 5 - 6 + param_2;
    }
    -1
}

/// port: 005012a0 FUN_005012a0
/// The best challenge money of tank `param_1` (+0x10c), when beaten.
pub fn FUN_005012a0(this: &mut PlayerProfile, param_1: i32, param_2: i32) {
    let i = (param_1 - 1) as u32;
    if i < 4 && this.field_0x10c[i as usize] < param_2 {
        this.field_0x10c[i as usize] = param_2;
    }
}

/// port: 005012d0 FUN_005012d0
/// The best time-trial time of tank `param_1` (+0x11c, -1 unset), when beaten.
pub fn FUN_005012d0(this: &mut PlayerProfile, param_1: i32, param_2: i32) {
    let i = (param_1 - 1) as u32;
    if i < 4 {
        let v = this.field_0x11c[i as usize];
        if param_2 < v || v == -1 {
            this.field_0x11c[i as usize] = param_2;
        }
    }
}

/// port: 00501230 FUN_00501230
/// The level's best (+0xb8): most shells in tank 5, else the shortest time (-1 unset).
pub fn FUN_00501230(this: &mut PlayerProfile, param_1: i32, param_2: i32, param_3: i32) {
    let i = FUN_00500dd0(param_1, param_2);
    if -1 < i {
        let i = i as usize;
        if param_1 == 5 && param_2 == 1 {
            if this.field_0xb8[i] < param_3 {
                this.field_0xb8[i] = param_3;
            }
        } else {
            let v = this.field_0xb8[i];
            if param_3 < v || v == -1 {
                this.field_0xb8[i] = param_3;
            }
        }
    }
}

/// port: 00501500 FUN_00501500
/// Toggles profile flag `param_1`; true when it is now set.
pub fn FUN_00501500(this: &mut PlayerProfile, param_1: u8) -> bool {
    let bit = 1u32 << (param_1 & 0x1f);
    this.field_0x84 ^= bit;
    this.field_0x84 & bit != 0
}

/// port: 00515910 FUN_00515910
/// `AddScreenSaverShells(int)`: writes `userdata\scr%d.dat` (the host creates the folder)
/// with the profile's key (+0x88), 50 random filler longs, the shells already there plus
/// `param_1`, and 50 more fillers. Nothing for `param_1` <= 0.
pub fn FUN_00515910(g: &mut G, this: Ptr, param_1: i32) {
    if param_1 < 1 {
        return;
    }
    let id = g.profile(this).field_0x44;
    let path = format!("userdata/scr{id}.dat");
    let had = FUN_005148c0(g, this, false);
    let mut w = crate::sexy::data_sync::DataWriter::default();
    crate::sexy::data_sync::FUN_005004e0(&mut w, g.profile(this).field_0x88);
    for _ in 0..0x32 {
        let r = crate::sexy::crt::rand(g);
        crate::sexy::data_sync::FUN_005004e0(&mut w, r % 20000);
    }
    crate::sexy::data_sync::FUN_005004e0(&mut w, had + param_1);
    for _ in 0..0x32 {
        let r = crate::sexy::crt::rand(g);
        crate::sexy::data_sync::FUN_005004e0(&mut w, r % 20000);
    }
    g.vfs.write(&path, w.data);
    let app = g.globals.DAT_005eb6a4;
    crate::sexy::sexy_app_base::FUN_00479f90(g, app, false);
}

/// port: 00501300 FUN_00501300
/// The best time of adventure level `param_1`-`param_2` (+0xb8), -1 for none.
pub fn FUN_00501300(this: &PlayerProfile, param_1: i32, param_2: i32) -> i32 {
    let i = FUN_00500dd0(param_1, param_2);
    if i < 0 {
        return -1;
    }
    this.field_0xb8[i as usize]
}

/// port: 00501330 FUN_00501330
/// The best Time Trial money of tank `param_1` (+0x10c), -1 for none.
pub fn FUN_00501330(this: &PlayerProfile, param_1: i32) -> i32 {
    if ((param_1 - 1) as u32) < 4 {
        return this.field_0x10c[(param_1 - 1) as usize];
    }
    -1
}

/// port: 00501350 FUN_00501350
/// The best Challenge time of tank `param_1` (+0x11c), -1 for none.
pub fn FUN_00501350(this: &PlayerProfile, param_1: i32) -> i32 {
    if ((param_1 - 1) as u32) < 4 {
        return this.field_0x11c[(param_1 - 1) as usize];
    }
    -1
}

/// port: 005013d0 FUN_005013d0
/// `ResetAdventure()`: back to tank 1 level 1, the final-boss attempts and flag cleared.
pub fn FUN_005013d0(g: &mut G, this: Ptr) {
    let p = g.profile(this);
    p.field_0x1c = 1;
    p.field_0x20 = 1;
    p.field_0x54 = 0;
    p.field_0x58 = false;
}
