//! `Sexy::HighScoreMgr`: the high-score tables in `userdata\highscores.dat`: per tank the
//! challenge money (best first) and the time-trial times, and per adventure level the
//! completion time (tank 5: shells, best first). Each table is a `std::list<HighScore>`
//! kept sorted, at most 5 entries (3 for tank 5).
//!
//! Every new record re-reads the file before inserting (so two running copies share it)
//! and writes the whole file back right away, like the original.

use crate::sexy::data_sync::{DataReader, DataSync, DataWriter, FUN_00500360, FUN_00500500, FUN_00503010, FUN_00505860, SyncResult};
use crate::sexy::prelude::*;

/// `HighScore` (a list node's value): name (+0x8 in the node) and score (+0x24).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HighScore {
    pub field_0x0: Vec<u8>,
    pub offset_0x1c: i32,
}

/// `HighScoreMgr_data` (object offset 0x4).
#[derive(Debug, Clone)]
pub struct HighScoreMgr_data {
    /// +0x04 per tank: challenge money.
    pub field_0x0: [Vec<HighScore>; 4],
    /// +0x34 per tank: time-trial times.
    pub field_0x30: [Vec<HighScore>; 4],
    /// +0x64 per adventure level (0x15): times (the last, tank 5, shells).
    pub field_0x60: Vec<Vec<HighScore>>,
}

impl Default for HighScoreMgr_data {
    fn default() -> Self {
        HighScoreMgr_data { field_0x0: Default::default(), field_0x30: Default::default(), field_0x60: vec![Vec::new(); 0x15] }
    }
}

impl G {
    pub fn high_score_mgr(&mut self, p: Ptr) -> &mut HighScoreMgr_data {
        match &mut self.obj(p).node {
            Node::HighScoreMgr(m) => m,
            n => panic!("{p} is not a HighScoreMgr: {n:?}"),
        }
    }
}

/// Which table a record goes to (the original passes the list's address).
#[derive(Clone, Copy, Debug)]
pub enum Table {
    Challenge(usize),
    TimeTrial(usize),
    Level(usize),
}

fn table(d: &mut HighScoreMgr_data, t: Table) -> &mut Vec<HighScore> {
    match t {
        Table::Challenge(i) => &mut d.field_0x0[i],
        Table::TimeTrial(i) => &mut d.field_0x30[i],
        Table::Level(i) => &mut d.field_0x60[i],
    }
}

/// port: 005124c0 Sexy::HighScoreMgr::HighScoreMgr
/// Empty tables.
pub fn HighScoreMgr(g: &mut G) -> Ptr {
    g.alloc(Obj { vt: None, node: Node::HighScoreMgr(Box::new(HighScoreMgr_data::default())) })
}

/// port: 00512560 Sexy::HighScoreMgr::~HighScoreMgr
pub fn dtor_HighScoreMgr(g: &mut G, this: Ptr) {
    *g.high_score_mgr(this) = HighScoreMgr_data::default();
}

/// port: 00513030 Sexy::HighScoreMgr::deleting_destructor
pub fn deleting_destructor(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_HighScoreMgr(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 005112b0 FUN_005112b0
/// `Clear()`: empties every table.
pub fn FUN_005112b0(g: &mut G, this: Ptr) {
    let d = g.high_score_mgr(this);
    for i in 0..4 {
        d.field_0x0[i].clear();
        d.field_0x30[i].clear();
    }
    for l in d.field_0x60.iter_mut() {
        l.clear();
    }
}

/// The default tables (`FUN_00513170`'s three local arrays): challenge names with
/// 1000..600, time-trial names and level names with 3300..3540 (0xce4 + 60 n).
const DEFAULT_CHALLENGE: [&str; 20] = [
    "Amy", "Ben", "Brenna", "Brian", "Chad", "Damon", "Dave", "Don", "Eric", "Greg", "Hans", "Jason", "Jeff", "John", "Josh", "Juho", "Kathy", "Katrina",
    "Mark", "Nick",
];
const DEFAULT_TIME_TRIAL: [&str; 20] = [
    "Shawn", "Sukhbir", "Tysen", "Walter", "jb63", "MrsDbolt", "Lexi", "GrukX", "cheryl", "Rambobear", "Kingdom", "Inkmei", "wolfpackmama", "hottentots",
    "Dyne", "monkeyboy", "lilith", "HRACH", "Lovedog", "Syrinx",
];
const DEFAULT_LEVEL: [&str; 20] = [
    "HomerJay", "Zelda", "lala", "DixieWriter", "samspade", "baasi1", "DaveDude", "Bubgrl", "Illyria", "hermitcrab", "kane", "ClaireBear", "wuggies",
    "jebksb1977", "firedog", "WoodRat", "Meatboy", "Splam11", "WendigoWolf", "bugsymcd24",
];
const DEFAULT_MONEY: [i32; 5] = [1000, 900, 800, 700, 600];
const DEFAULT_TIME: [i32; 5] = [0xce4, 0xd20, 0xd5c, 0xd98, 0xdd4];

/// port: 00513170 FUN_00513170
/// `SetDefaults()`: five made-up entries per challenge and time-trial table, one per
/// adventure level of tanks 1..4 (none for tank 5).
pub fn FUN_00513170(g: &mut G, this: Ptr) {
    FUN_005112b0(g, this);
    let d = g.high_score_mgr(this);
    for pass in 0..2 {
        for t in 0..4 {
            for k in 0..5 {
                let i = t * 5 + k;
                let e = if pass == 0 {
                    HighScore { field_0x0: DEFAULT_CHALLENGE[i].as_bytes().to_vec(), offset_0x1c: DEFAULT_MONEY[k] }
                } else {
                    HighScore { field_0x0: DEFAULT_TIME_TRIAL[i].as_bytes().to_vec(), offset_0x1c: DEFAULT_TIME[k] }
                };
                if pass == 0 { &mut d.field_0x0[t] } else { &mut d.field_0x30[t] }.push(e);
            }
        }
    }
    for (i, name) in DEFAULT_LEVEL.iter().enumerate() {
        d.field_0x60[i].push(HighScore { field_0x0: name.as_bytes().to_vec(), offset_0x1c: DEFAULT_TIME[i % 5] });
    }
}

/// port: 00505960 FUN_00505960
/// `HighScore::Sync(DataSync&)`: name, score.
pub fn FUN_00505960(this: &mut HighScore, sync: &mut DataSync) -> SyncResult {
    FUN_00505860(sync, &mut this.field_0x0)?;
    FUN_00503010(sync, &mut this.offset_0x1c)
}

/// port: 00513060 FUN_00513060
/// `SyncList(DataSync&, list&)`: a 16-bit count, then the entries.
pub fn FUN_00513060(sync: &mut DataSync, list: &mut Vec<HighScore>) -> SyncResult {
    match &mut sync.io {
        crate::sexy::data_sync::DataIo::Write(w) => {
            FUN_00500500(w, list.len() as u16);
            for e in list.iter_mut() {
                FUN_00505960(e, sync)?;
            }
        }
        crate::sexy::data_sync::DataIo::Read(r) => {
            list.clear();
            let n = FUN_00500360(r)?;
            for _ in 0..n {
                let mut e = HighScore::default();
                FUN_00505960(&mut e, sync)?;
                list.push(e);
            }
        }
    }
    Ok(())
}

/// port: 00513f90 FUN_00513f90
/// `Sync(DataSync&)`: version 2 (another version on reading restores the defaults), then
/// per tank challenge and time trial, then the 21 level tables.
pub fn FUN_00513f90(g: &mut G, this: Ptr, sync: &mut DataSync) -> SyncResult {
    let reading = matches!(sync.io, crate::sexy::data_sync::DataIo::Read(_));
    let mut ver = 2;
    FUN_00503010(sync, &mut ver)?;
    if reading {
        FUN_005112b0(g, this);
        if ver != 2 {
            FUN_00513170(g, this);
            return Ok(());
        }
    }
    let mut d = g.high_score_mgr(this).clone();
    let r = (|| {
        for i in 0..4 {
            FUN_00513060(sync, &mut d.field_0x0[i])?;
            FUN_00513060(sync, &mut d.field_0x30[i])?;
        }
        for l in d.field_0x60.iter_mut() {
            FUN_00513060(sync, l)?;
        }
        Ok(())
    })();
    *g.high_score_mgr(this) = d;
    r
}

/// port: 00514030 FUN_00514030
/// `Load()`: `userdata\highscores.dat`, or the defaults when there is none. (A damaged file
/// leaves what was read so far; the original catches the reader's exception there.)
pub fn FUN_00514030(g: &mut G, this: Ptr) {
    FUN_005112b0(g, this);
    match g.vfs.read("userdata/highscores.dat").map(|b| b.to_vec()) {
        None => FUN_00513170(g, this),
        Some(bytes) => {
            let mut sync = crate::sexy::data_sync::DataSync__00513e30(DataReader::from_bytes(bytes));
            let _ = FUN_00513f90(g, this, &mut sync);
        }
    }
}

/// port: 00514190 FUN_00514190
/// `Save()`: writes `userdata\highscores.dat` (the host creates the folder).
pub fn FUN_00514190(g: &mut G, this: Ptr) {
    let mut sync = crate::sexy::data_sync::DataSync__00513ee0(DataWriter::default());
    let _ = FUN_00513f90(g, this, &mut sync);
    if let crate::sexy::data_sync::DataIo::Write(w) = sync.io {
        g.vfs.write("userdata/highscores.dat", w.data);
    }
}

/// port: 00504d50 FUN_00504d50
/// The insertion point for `param_1`: before the first entry it beats (higher-is-better
/// tables when `param_2`, else lower-is-better).
pub fn FUN_00504d50(list: &[HighScore], param_1: i32, param_2: bool) -> usize {
    for (i, e) in list.iter().enumerate() {
        if param_2 {
            if e.offset_0x1c < param_1 {
                return i;
            }
        } else if param_1 < e.offset_0x1c {
            return i;
        }
    }
    list.len()
}

/// port: 005142f0 FUN_005142f0
/// `AddScore(list, name, score, maxEntries, higherIsBetter, reload)`: false when the score
/// does not make the table; otherwise (after re-reading the file when `reload`) inserts it,
/// trims the table and saves.
pub fn FUN_005142f0(g: &mut G, this: Ptr, param_1: Table, param_2: &[u8], param_3: i32, param_4: i32, param_5: bool, param_6: bool) -> bool {
    let max = param_4.max(0) as usize;
    let l = table(g.high_score_mgr(this), param_1);
    while max < l.len() && !l.is_empty() {
        l.pop();
    }
    let pos = FUN_00504d50(l, param_3, param_5);
    if pos == l.len() && (max as i64) <= l.len() as i64 {
        return false;
    }
    if param_6 {
        FUN_00514030(g, this);
        return FUN_005142f0(g, this, param_1, param_2, param_3, param_4, param_5, false);
    }
    let l = table(g.high_score_mgr(this), param_1);
    l.insert(pos, HighScore { field_0x0: param_2.to_vec(), offset_0x1c: param_3 });
    while max < l.len() && !l.is_empty() {
        l.pop();
    }
    FUN_00514190(g, this);
    true
}

/// port: 00514470 FUN_00514470
/// `AddChallengeScore(tank, profile, money)` (tanks 1..4): also the profile's best.
pub fn FUN_00514470(g: &mut G, this: Ptr, param_1: i32, param_2: Ptr, param_3: i32) -> bool {
    if !(1..=4).contains(&param_1) {
        return false;
    }
    crate::game::profile::FUN_005012a0(g.profile(param_2), param_1, param_3);
    let name = g.profile(param_2).field_0x24.clone();
    FUN_005142f0(g, this, Table::Challenge((param_1 - 1) as usize), &name, param_3, 5, true, true)
}

/// port: 005144c0 FUN_005144c0
/// `AddTimeTrialScore(tank, profile, time)` (tanks 1..4): also the profile's best.
pub fn FUN_005144c0(g: &mut G, this: Ptr, param_1: i32, param_2: Ptr, param_3: i32) -> bool {
    if !(1..=4).contains(&param_1) {
        return false;
    }
    crate::game::profile::FUN_005012d0(g.profile(param_2), param_1, param_3);
    let name = g.profile(param_2).field_0x24.clone();
    FUN_005142f0(g, this, Table::TimeTrial((param_1 - 1) as usize), &name, param_3, 5, false, true)
}

/// port: 00514510 FUN_00514510
/// `AddLevelScore(tank, level, profile, score)`: the level's time (tank 5: shells, best
/// first, 3 entries); also the profile's best.
pub fn FUN_00514510(g: &mut G, this: Ptr, param_1: i32, param_2: i32, param_3: Ptr, param_4: i32) -> bool {
    let idx = crate::game::profile::FUN_00500dd0(param_1, param_2);
    if idx < 0 {
        return false;
    }
    let max = if param_1 == 5 { 3 } else { 1 };
    crate::game::profile::FUN_00501230(g.profile(param_3), param_1, param_2, param_4);
    let name = g.profile(param_3).field_0x24.clone();
    FUN_005142f0(g, this, Table::Level(idx as usize), &name, param_4, max, param_1 == 5, true)
}
