//! `Sexy::ProfileMgr` (0x18 bytes; the database's type stops at 0x10): the players' profiles
//! by name, `userdata\users.dat`. The original keeps a `std::map<std::string, PlayerInfo>`
//! ordered by `_stricmp`; here it is a vector kept in the same order (the map's template
//! instances in the game region are listed as replaced in the manifest).

use crate::game::profile::{FUN_00504e00, FUN_00505990, FUN_005145a0, PlayerProfile};
use crate::sexy::data_sync::{DataReader, DataSync, DataWriter, SyncResult, FUN_00500360, FUN_00500500, FUN_00503010};
use crate::sexy::prelude::*;

/// `ProfileMgr` (+0x8.. the map, +0x10 next id, +0x14 next use sequence).
#[derive(Debug, Clone, Default)]
pub struct ProfileMgr {
    /// +0x08 the map (head node; its size at +0x0c is `len()`): (name key, profile), sorted by
    /// `_stricmp` of the key.
    pub offset_0x4: Vec<(Vec<u8>, Ptr)>,
    /// +0x10 the next profile id (`user%d.dat`).
    pub ext_0x10: u32,
    /// +0x14 the next use sequence number.
    pub ext_0x14: u32,
}

impl G {
    pub fn profile_mgr(&mut self, p: Ptr) -> &mut ProfileMgr {
        match &mut self.obj(p).node {
            Node::ProfileMgr(m) => m,
            n => panic!("{p} is not a ProfileMgr: {n:?}"),
        }
    }
}

/// `DAT_005e042c`: the `users.dat` format version.
pub const DAT_005e042c: i32 = 14;

/// CRT `_stricmp` in the C locale: byte-wise compare of the ASCII-lowercased strings.
pub fn _stricmp(a: &[u8], b: &[u8]) -> i32 {
    let mut i = 0;
    loop {
        let ca = a.get(i).copied().unwrap_or(0).to_ascii_lowercase() as i32;
        let cb = b.get(i).copied().unwrap_or(0).to_ascii_lowercase() as i32;
        if ca != cb || ca == 0 {
            return ca - cb;
        }
        i += 1;
    }
}

/// The map's `lower_bound`: the first entry whose key is not less than `key`.
fn lower_bound(m: &ProfileMgr, key: &[u8]) -> usize {
    m.offset_0x4.partition_point(|(k, _)| _stricmp(k, key) < 0)
}

/// The map's `find`: the entry with an equal key (by `_stricmp`).
fn find(m: &ProfileMgr, key: &[u8]) -> Option<usize> {
    let i = lower_bound(m, key);
    if i < m.offset_0x4.len() && !(_stricmp(key, &m.offset_0x4[i].0) < 0) {
        return Some(i);
    }
    None
}

/// port: 00515b00 Sexy::ProfileMgr::ProfileMgr
pub fn ProfileMgr(g: &mut G) -> Ptr {
    let this = g.alloc(Obj { vt: None, node: Node::ProfileMgr(Box::new(ProfileMgr::default())) });
    FUN_005125f0(g, this);
    this
}

/// port: 00514b80 Sexy::ProfileMgr::~ProfileMgr
/// Frees every profile and the map.
pub fn dtor_ProfileMgr(g: &mut G, this: Ptr) {
    let all = std::mem::take(&mut g.profile_mgr(this).offset_0x4);
    for (_, p) in all {
        g.free(p);
    }
}

/// port: 00515b80 Sexy::ProfileMgr::deleting_destructor
pub fn deleting_destructor(g: &mut G, this: Ptr, param_1: u8) -> Ptr {
    dtor_ProfileMgr(g, this);
    if param_1 & 1 != 0 {
        g.free(this);
    }
    this
}

/// port: 005125f0 FUN_005125f0
/// `Clear()`: no profiles; ids and use numbers restart at 1.
pub fn FUN_005125f0(g: &mut G, this: Ptr) {
    let old = std::mem::take(&mut g.profile_mgr(this).offset_0x4);
    for (_, p) in old {
        g.free(p);
    }
    let m = g.profile_mgr(this);
    m.ext_0x10 = 1;
    m.ext_0x14 = 1;
}

/// port: 00513850 FUN_00513850
/// `SyncState(DataSync&)`: the version word, then the profiles' summaries (a 16-bit count,
/// then name/use/id each). Loading replaces the map and sets the next id and use number past
/// the largest loaded (unsigned comparisons). Another version loads nothing. (The original
/// also stores the version read in the DataSync object; nothing reads it there.)
pub fn FUN_00513850(g: &mut G, this: Ptr, param_1: &mut DataSync) -> SyncResult {
    let mut version = DAT_005e042c;
    FUN_00503010(param_1, &mut version)?;
    if version != DAT_005e042c {
        return Ok(());
    }
    match &mut param_1.io {
        crate::sexy::data_sync::DataIo::Write(w) => {
            let n = g.profile_mgr(this).offset_0x4.len() as u16;
            FUN_00500500(w, n);
            let entries = g.profile_mgr(this).offset_0x4.clone();
            for (_, p) in entries {
                let mut pr = g.profile(p).clone();
                FUN_00505990(&mut pr, param_1)?;
            }
        }
        crate::sexy::data_sync::DataIo::Read(_) => {
            FUN_005125f0_keep_counters(g, this);
            let n = match &mut param_1.io {
                crate::sexy::data_sync::DataIo::Read(r) => FUN_00500360(r)? & 0xffff,
                crate::sexy::data_sync::DataIo::Write(_) => unreachable!(),
            };
            let (mut max_use, mut max_id) = (0u32, 0u32);
            for _ in 0..n {
                let mut tmp = FUN_00504e00(g);
                FUN_00505990(&mut tmp, param_1)?;
                if max_use < tmp.field_0x40 as u32 {
                    max_use = tmp.field_0x40 as u32;
                }
                if max_id < tmp.field_0x44 as u32 {
                    max_id = tmp.field_0x44 as u32;
                }
                let key = tmp.field_0x24.clone();
                let p = FUN_00512980(g, this, &key);
                *g.profile(p) = tmp;
            }
            let m = g.profile_mgr(this);
            m.ext_0x10 = max_id.wrapping_add(1);
            m.ext_0x14 = max_use.wrapping_add(1);
        }
    }
    Ok(())
}

/// The map's `clear()` as done at the start of a load (the counters are set afterwards).
fn FUN_005125f0_keep_counters(g: &mut G, this: Ptr) {
    let old = std::mem::take(&mut g.profile_mgr(this).offset_0x4);
    for (_, p) in old {
        g.free(p);
    }
}

/// port: 00512980 FUN_00512980
/// The map's `operator[](name)`: the profile with that name, inserting a new one (a fresh
/// `PlayerInfo`) when there is none.
pub fn FUN_00512980(g: &mut G, this: Ptr, param_1: &[u8]) -> Ptr {
    if let Some(i) = find(g.profile_mgr(this), param_1) {
        return g.profile_mgr(this).offset_0x4[i].1;
    }
    let tmp = FUN_00504e00(g);
    let p = g.alloc(Obj { vt: None, node: Node::Profile(Box::new(tmp)) });
    let i = lower_bound(g.profile_mgr(this), param_1);
    g.profile_mgr(this).offset_0x4.insert(i, (param_1.to_vec(), p));
    p
}

/// port: 00514c00 FUN_00514c00
/// `Load()`: `userdata\users.dat` from the app data folder (empty in this game, so the
/// install folder), when it exists. Read errors are caught and ignored.
pub fn FUN_00514c00(g: &mut G, this: Ptr) {
    let Some(bytes) = g.vfs.read("userdata/users.dat").map(|b| b.to_vec()) else { return };
    let mut sync = crate::sexy::data_sync::DataSync__00513e30(DataReader::from_bytes(bytes));
    let _ = FUN_00513850(g, this, &mut sync);
}

/// port: 00514d50 FUN_00514d50
/// `Save()`: writes `userdata\users.dat` (the host creates the folder).
pub fn FUN_00514d50(g: &mut G, this: Ptr) {
    let mut sync = crate::sexy::data_sync::DataSync__00513ee0(DataWriter::default());
    let _ = FUN_00513850(g, this, &mut sync);
    if let crate::sexy::data_sync::DataIo::Write(w) = sync.io {
        g.vfs.write("userdata/users.dat", w.data);
    }
}

/// port: 005127e0 FUN_005127e0
/// `AddProfile(const std::string& name)`: a new profile with the next id and use number;
/// null when the name is taken (compared without case). Keeps at most 200 profiles.
pub fn FUN_005127e0(g: &mut G, this: Ptr, param_1: &[u8]) -> Ptr {
    let tmp: PlayerProfile = FUN_00504e00(g);
    if find(g.profile_mgr(this), param_1).is_some() {
        return NULL;
    }
    let p = g.alloc(Obj { vt: None, node: Node::Profile(Box::new(tmp)) });
    let i = lower_bound(g.profile_mgr(this), param_1);
    g.profile_mgr(this).offset_0x4.insert(i, (param_1.to_vec(), p));
    g.profile(p).field_0x24 = param_1.to_vec();
    let m = g.profile_mgr(this);
    let (id, seq) = (m.ext_0x10, m.ext_0x14);
    m.ext_0x10 = id.wrapping_add(1);
    m.ext_0x14 = seq.wrapping_add(1);
    g.profile(p).field_0x44 = id as i32;
    g.profile(p).field_0x40 = seq as i32;
    FUN_00511470(g, this);
    p
}

/// port: 005113a0 FUN_005113a0
/// `DeleteOldestProfile()`: the profile with the lowest last-used sequence number (the
/// first of equals, compared unsigned) loses its files and its entry.
pub fn FUN_005113a0(g: &mut G, this: Ptr) {
    if g.profile_mgr(this).offset_0x4.is_empty() {
        return;
    }
    let mut best = 0;
    let n = g.profile_mgr(this).offset_0x4.len();
    for i in 0..n {
        let cur = g.profile_mgr(this).offset_0x4[i].1;
        let b = g.profile_mgr(this).offset_0x4[best].1;
        if (g.profile(cur).field_0x40 as u32) < (g.profile(b).field_0x40 as u32) {
            best = i;
        }
    }
    let p = g.profile_mgr(this).offset_0x4[best].1;
    FUN_005059d0(g, p);
    FUN_005100f0(g, this, best);
}

/// port: 00511470 FUN_00511470
/// Deletes the least recently used profiles while there are more than 200.
pub fn FUN_00511470(g: &mut G, this: Ptr) {
    while 200 < g.profile_mgr(this).offset_0x4.len() as u32 {
        FUN_005113a0(g, this);
    }
}

/// port: 00514eb0 FUN_00514eb0
/// `GetProfile(const std::string& name)`: loads its details and marks it most recent; null
/// when there is no such profile.
pub fn FUN_00514eb0(g: &mut G, this: Ptr, param_1: &[u8]) -> Ptr {
    let Some(i) = find(g.profile_mgr(this), param_1) else { return NULL };
    let p = g.profile_mgr(this).offset_0x4[i].1;
    FUN_005145a0(g, p);
    let m = g.profile_mgr(this);
    let seq = m.ext_0x14;
    m.ext_0x14 = seq.wrapping_add(1);
    g.profile(p).field_0x40 = seq as i32;
    p
}

/// port: 00514bc0 FUN_00514bc0
/// `GetAnyProfile()`: the first by name, loaded and marked most recent; null when there are
/// none.
pub fn FUN_00514bc0(g: &mut G, this: Ptr) -> Ptr {
    if g.profile_mgr(this).offset_0x4.is_empty() {
        return NULL;
    }
    let p = g.profile_mgr(this).offset_0x4[0].1;
    FUN_005145a0(g, p);
    let m = g.profile_mgr(this);
    let seq = m.ext_0x14;
    m.ext_0x14 = seq.wrapping_add(1);
    g.profile(p).field_0x40 = seq as i32;
    p
}

/// port: 00505290 FUN_00505290
/// The map's `find(name)` (an STL instance; compared without case): the entry's index.
pub fn FUN_00505290(m: &ProfileMgr, param_1: &[u8]) -> Option<usize> {
    find(m, param_1)
}

/// port: 005059d0 FUN_005059d0
/// `PlayerInfo::DeleteFiles()`: removes the profile's `userdata/user%d.dat`,
/// `userdata/scr%d.dat` and its six saved games.
pub fn FUN_005059d0(g: &mut G, this: Ptr) {
    let id = g.profile(this).field_0x44;
    crate::sexy::app_host::FUN_0047fbc0(g, &format!("userdata/user{id}.dat"));
    crate::sexy::app_host::FUN_0047fbc0(g, &format!("userdata/scr{id}.dat"));
    for i in 0..6 {
        let path = crate::game::win_fish_app::FUN_00505880(i, id);
        crate::sexy::app_host::FUN_0047fbc0(g, &path);
    }
}

/// port: 005100f0 FUN_005100f0
/// The map's `erase(where)` (an STL instance): the entry goes, the following index is
/// returned. (The node's profile is freed with it.)
pub fn FUN_005100f0(g: &mut G, this: Ptr, param_1: usize) -> usize {
    let (_, p) = g.profile_mgr(this).offset_0x4.remove(param_1);
    g.free(p);
    param_1
}

/// port: 00511300 FUN_00511300
/// `erase(iterator)` of the profile map: deletes the profile's files, then the entry.
pub fn FUN_00511300(g: &mut G, this: Ptr, param_1: usize) {
    let p = g.profile_mgr(this).offset_0x4[param_1].1;
    FUN_005059d0(g, p);
    FUN_005100f0(g, this, param_1);
}

/// port: 00511340 FUN_00511340
/// `DeleteProfile(const std::string& name)`: false when there is none by that name.
pub fn FUN_00511340(g: &mut G, this: Ptr, param_1: &[u8]) -> bool {
    let Some(i) = FUN_00505290(g.profile_mgr(this), param_1) else { return false };
    FUN_00511300(g, this, i);
    true
}

/// port: 00505c30 FUN_00505c30
/// `std::pair<const std::string, PlayerInfo>(name, info)` (an STL instance).
pub fn FUN_00505c30(param_1: &[u8], param_2: Ptr) -> (Vec<u8>, Ptr) {
    (param_1.to_vec(), param_2)
}

/// port: 00504e60 FUN_00504e60
/// The pair's destructor (its key string; an STL instance).
pub fn FUN_00504e60(param_1: (Vec<u8>, Ptr)) {
    drop(param_1);
}

/// port: 005118f0 FUN_005118f0
/// The map's `insert(pair)` (an STL instance): the entry's index and whether it was added
/// (false when the name is taken, compared without case).
pub fn FUN_005118f0(m: &mut ProfileMgr, param_1: (Vec<u8>, Ptr)) -> (usize, bool) {
    if let Some(i) = find(m, &param_1.0) {
        return (i, false);
    }
    let i = lower_bound(m, &param_1.0);
    m.offset_0x4.insert(i, param_1);
    (i, true)
}

/// port: 00512630 FUN_00512630
/// `RenameProfile(const std::string& old, std::string& new)`: false when there is no such
/// profile or the new name is taken by another; a name differing only in case keeps its
/// entry. The profile takes the new name.
pub fn FUN_00512630(g: &mut G, this: Ptr, param_1: &[u8], param_2: &[u8]) -> bool {
    let Some(mut i) = FUN_00505290(g.profile_mgr(this), param_1) else { return false };
    if _stricmp(param_1, param_2) != 0 {
        let p = g.profile_mgr(this).offset_0x4[i].1;
        let pair = FUN_00505c30(param_2, p);
        let (at, added) = FUN_005118f0(g.profile_mgr(this), pair.clone());
        FUN_00504e60(pair);
        if !added {
            return false;
        }
        // The old node goes; its info was copied into the new one (here: the same object).
        let old = FUN_00505290(g.profile_mgr(this), param_1).filter(|&k| k != at);
        if let Some(k) = old {
            g.profile_mgr(this).offset_0x4.remove(k);
        }
        i = FUN_00505290(g.profile_mgr(this), param_2).unwrap_or(at);
    }
    let p = g.profile_mgr(this).offset_0x4[i].1;
    g.profile(p).field_0x24 = param_2.to_vec();
    true
}
