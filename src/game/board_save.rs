//! The saved game: `userdata\<mode><id>.dat` (`FUN_00505880`) holds the board's state and
//! every board object, written through a `DataSync` (version 0x36, the `time()` after the
//! board data, then the pointer fix-ups). Loading needs version 0x35 or later.
//!
//! The original catches nothing here: a short read throws `DataReaderException` out of the
//! load (the caller treats the game as not loaded only through the `false` paths below).
//! The port returns `false` on a short read at the same points instead of unwinding, and
//! the objects created so far stay on the board, as they would after the throw.

use crate::game::board_level::vec_index;
use crate::sexy::data_sync::{DataIo, DataSync, ReadError, SyncResult};
use crate::sexy::prelude::*;

/// port: 00537570 FUN_00537570
/// `CreateObject(int type)`: an empty object of a GameObject type id (to be filled by its
/// `Sync`); null for an unknown id.
pub fn FUN_00537570(g: &mut G, param_1: i32) -> Ptr {
    use crate::game as m;
    match param_1 {
        0 => m::fish::Fish__004eee60(g),
        5 => m::oscar::Oscar__004efd80(g),
        6 => m::ultra::Ultra__004f0270(g),
        7 => m::gekko::Gekko__004ef970(g),
        8 => m::penta::Penta__004ec040(g),
        9 => m::grubber::Grubber__004ea700(g),
        10 => m::breeder::Breeder__004ee500(g),
        0x14 => m::other_pet::OtherTypePet__004eb590(g),
        0x15 => m::fish_type_pet::FishTypePet__004ef3e0(g),
        0x16 => m::alien::Alien__004ecf80(g),
        0x17 => m::bilaterus::Bilaterus__004f98b0(g),
        0x19 => m::coin::Coin__004eea80(g),
        0x1a => m::dead_alien::DeadAlien__004eebe0(g),
        0x1b => m::dead_fish::DeadFish__004eed00(g),
        0x1c => m::food::Food__004ef820(g),
        0x1d => m::larva::Larva__004ead20(g),
        0x1e => m::missle::Missle__004eae00(g),
        0x1f => m::shadow::Shadow__004ec4b0(g),
        0x20 => m::shot::Shot__004ec5d0(g),
        0x21 => m::warp::Warp__004ecee0(g),
        0x22 => m::vt_fish::SylvesterFish__004f01b0(g),
        0x23 => m::vt_fish::BallFish__004f0550(g),
        0x24 => m::vt_fish::BiFish__004f0600(g),
        _ => NULL,
    }
}

/// port: 00537b00 FUN_00537b00
/// `ReadObject(DataSync&)`: peeks the type id (the object's own `Sync` reads it again),
/// creates the object and syncs it; an unknown type throws.
pub fn FUN_00537b00(g: &mut G, sync: &mut DataSync) -> Result<Ptr, ReadError> {
    let DataIo::Read(r) = &mut sync.io else { unreachable!("FUN_00537b00 reads") };
    let ty = crate::sexy::data_sync::FUN_00500340(r)?;
    crate::sexy::data_sync::DataReaderException__00500230(r, 4)?;
    let o = FUN_00537570(g, ty);
    if o == NULL {
        return Err(ReadError);
    }
    vcall!(g, o, go.vfunction81, sync)?;
    Ok(o)
}

/// port: 005387f0 FUN_005387f0
/// `SyncAlienPointer(DataSync&, Alien*&)` (the boss): whether there is one, then its data.
/// Reading deletes the current one and builds a new empty alien to fill.
pub fn FUN_005387f0(g: &mut G, param_1: &mut DataSync, param_2: &mut Ptr) -> SyncResult {
    if let DataIo::Write(w) = &mut param_1.io {
        crate::sexy::data_sync::FUN_00500540(w, *param_2 != NULL);
        if *param_2 != NULL {
            vcall!(g, *param_2, go.vfunction81, param_1)?;
        }
        return Ok(());
    }
    if *param_2 != NULL {
        vcall!(g, *param_2, w.vfunction1, 1);
    }
    let DataIo::Read(r) = &mut param_1.io else { unreachable!() };
    if crate::sexy::data_sync::FUN_005003a0(r)? {
        let a = crate::game::alien::Alien__004ecf80(g);
        *param_2 = a;
        vcall!(g, a, go.vfunction81, param_1)?;
        return Ok(());
    }
    *param_2 = NULL;
    Ok(())
}

/// port: 00530780 FUN_00530780
/// `MessageWidget::Sync(DataSync&)`: the text, both colors, and its timers.
pub fn FUN_00530780(g: &mut G, this: Ptr, param_1: &mut DataSync) -> SyncResult {
    use crate::sexy::data_sync::{FUN_00503010, FUN_00503070, FUN_00505860};
    let mut d = g.message_widget(this).clone();
    let r = (|| {
        FUN_00505860(param_1, &mut d.field_0x4)?;
        crate::game::fish::FUN_005036a0(param_1, &mut d.offset_0x20)?;
        crate::game::fish::FUN_005036a0(param_1, &mut d.offset_0x30)?;
        FUN_00503010(param_1, &mut d.offset_0x40)?;
        FUN_00503010(param_1, &mut d.offset_0x48)?;
        FUN_00503070(param_1, &mut d.offset_0x4c)?;
        FUN_00503010(param_1, &mut d.offset_0x50)
    })();
    *g.message_widget(this) = d;
    r
}

/// port: 00546240 FUN_00546240
/// `Board::Sync(DataSync&)`: the level (reading a saved adventure of another level than
/// the profile's gives up), the modes, every board object but the shadows (they come with
/// their owners), the boss, the message line, the per-pet counts, the level state, the
/// statistics and hint flags, the shared globals (follow mode, breeder spot, backdrop tint)
/// and the shop slots. When saving in the screensaver the weapon level is saved as the
/// screensaver's own (+0x3ec). Afterwards the follow mode is dropped with no aliens, the
/// pizza event without its pet, and the backdrop is refreshed.
pub fn FUN_00546240(g: &mut G, this: Ptr, param_1: &mut DataSync) -> Result<bool, ReadError> {
    use crate::sexy::data_sync::{FUN_00503010, FUN_00503070};
    let app = g.board(this).field_0x0;
    let mut v = g.board(this).field_0x33c;
    FUN_00503010(param_1, &mut v)?;
    g.board(this).field_0x33c = v;
    let mut v = g.board(this).field_0x340;
    FUN_00503010(param_1, &mut v)?;
    g.board(this).field_0x340 = v;
    let reading = matches!(param_1.io, DataIo::Read(_));
    if g.wfa(app).offset_0x150 == 0 && reading {
        let profile = g.wfa(app).offset_0x18c;
        let (tank, level) = (g.profile(profile).field_0x1c, g.profile(profile).field_0x20);
        if g.board(this).field_0x33c != tank || g.board(this).field_0x340 != level {
            return Ok(false);
        }
    }
    let mut b = g.board(this).field_0x219;
    FUN_00503070(param_1, &mut b)?;
    g.board(this).field_0x219 = b;
    let mut b = g.board(this).field_0x21a;
    FUN_00503070(param_1, &mut b)?;
    g.board(this).field_0x21a = b;
    let mut saved_weapon = -1;
    if crate::game::win_fish_app::thunk_FUN_00479fc0(g, app) && !reading && g.board(this).field_0x360 != -1 {
        saved_weapon = g.board(this).field_0x35c;
        g.board(this).field_0x35c = g.board(this).field_0x360;
        crate::game::board_update::FUN_00539a50(g, this);
    }
    if !reading {
        let DataIo::Write(w) = &mut param_1.io else { unreachable!() };
        let at = w.data.len();
        crate::sexy::data_sync::FUN_005004e0(w, 0);
        let mut n = 0;
        let objs: Vec<Ptr> = g.board(this).offset_0x7c.iter().copied().collect();
        for o in objs {
            if g.go(o).offset_0x4 != 0x1f {
                n += 1;
                vcall!(g, o, go.vfunction81, param_1)?;
            }
        }
        let DataIo::Write(w) = &mut param_1.io else { unreachable!() };
        FUN_00500580(w, n, at);
    } else {
        let mgr = g.wc(this).offset_0xc;
        crate::game::board::FUN_0053c480(g, this, mgr);
        let DataIo::Read(r) = &mut param_1.io else { unreachable!() };
        let mut n = crate::sexy::data_sync::FUN_00500340(r)?;
        while 0 < n {
            let o = FUN_00537b00(g, param_1)?;
            if !g.go(o).offset_0x6c {
                crate::game::board_level::FUN_00540ec0(&mut g.board(this).offset_0x7c, o);
            } else {
                crate::game::board_level::FUN_00542ee0(g, this, o, 0);
                let mgr = g.wc(this).offset_0xc;
                vcall!(g, mgr, w.vfunction4, o);
                crate::game::board_level::FUN_00544800(g, this, o);
            }
            n -= 1;
        }
    }
    let mut boss = g.board(this).field_0x84;
    let r = FUN_005387f0(g, param_1, &mut boss);
    g.board(this).field_0x84 = boss;
    r?;
    if boss != NULL {
        let mgr = g.wc(this).offset_0xc;
        vcall!(g, mgr, w.vfunction4, boss);
    }
    let msg = g.board(this).offset_0x8c;
    FUN_00530780(g, msg, param_1)?;
    let mut d = g.board(this).clone();
    let r = (|| -> SyncResult {
        for v in d.field_0xb8.iter_mut() {
            FUN_00503010(param_1, v)?;
        }
        FUN_00503070(param_1, &mut d.field_0x218)?;
        for v in [
            &mut d.field_0x21c, &mut d.field_0x230, &mut d.field_0x234, &mut d.field_0x238, &mut d.field_0x23c, &mut d.field_0x240,
            &mut d.field_0x244, &mut d.field_0x248, &mut d.field_0x24c, &mut d.field_0x250, &mut d.field_0x324, &mut d.field_0x328,
            &mut d.field_0x32c, &mut d.field_0x330, &mut d.field_0x334, &mut d.field_0x338, &mut d.field_0x358, &mut d.field_0x35c,
            &mut d.field_0x364, &mut d.field_0x368, &mut d.field_0x36c, &mut d.field_0x370, &mut d.field_0x220, &mut d.field_0x224,
            &mut d.ext_0x43c,
        ] {
            FUN_00503010(param_1, v)?;
        }
        FUN_00503070(param_1, &mut d.ext_0x440)?;
        for v in [
            &mut d.ext_0x444, &mut d.ext_0x448, &mut d.ext_0x44c, &mut d.ext_0x450, &mut d.ext_0x454, &mut d.ext_0x458, &mut d.ext_0x45c,
        ] {
            FUN_00503010(param_1, v)?;
        }
        for v in d.ext_0x460.iter_mut() {
            FUN_00503010(param_1, v)?;
        }
        for v in [&mut d.ext_0x4fc, &mut d.ext_0x4fd, &mut d.ext_0x4fe, &mut d.ext_0x4ff, &mut d.ext_0x500] {
            FUN_00503070(param_1, v)?;
        }
        for v in d.ext_0x4b0.iter_mut() {
            let mut b = *v != 0;
            FUN_00503070(param_1, &mut b)?;
            *v = b as u8;
        }
        FUN_00503070(param_1, &mut d.ext_0x4e6)?;
        FUN_00503010(param_1, &mut d.ext_0x4e8)?;
        FUN_00503010(param_1, &mut d.ext_0x4f0)?;
        FUN_00503070(param_1, &mut d.ext_0x4f4)
    })();
    *g.board(this) = d;
    r?;
    let gl = &mut g.globals;
    let mut v = [gl.DAT_005e89d0, gl.DAT_005e89d4, gl.DAT_005e89d8, gl.DAT_005e89c0, gl.DAT_005e89c4, gl.DAT_005e89c8];
    for x in v.iter_mut() {
        FUN_00503010(param_1, x)?;
    }
    let gl = &mut g.globals;
    [gl.DAT_005e89d0, gl.DAT_005e89d4, gl.DAT_005e89d8, gl.DAT_005e89c0, gl.DAT_005e89c4, gl.DAT_005e89c8] = v;
    FUN_00503070(param_1, &mut g.globals.DAT_005e89cc)?;
    FUN_00503010(param_1, &mut g.globals.DAT_005e89dc)?;
    FUN_00503010(param_1, &mut g.globals.DAT_005df58c)?;
    crate::game::fish::FUN_005036a0(param_1, &mut g.globals.DAT_005e89e8)?;
    if reading {
        g.board(this).field_0x254 = 0;
    }
    for i in 0..0xc {
        let mut d = g.board(this).clone();
        let r = (|| -> SyncResult {
            FUN_00503010(param_1, &mut d.field_0x258[i])?;
            FUN_00503010(param_1, &mut d.field_0x288[i])?;
            FUN_00503010(param_1, &mut d.field_0x2b8[i])?;
            FUN_00503010(param_1, &mut d.field_0x2e8[i])?;
            FUN_00503070(param_1, &mut d.field_0x318[i])
        })();
        *g.board(this) = d;
        r?;
        if reading && g.board(this).field_0x318[i] {
            crate::game::board_level::FUN_005409b0(g, this, i, false);
        }
    }
    if saved_weapon != -1 {
        g.board(this).field_0x35c = saved_weapon;
        crate::game::board_update::FUN_00539a50(g, this);
    }
    let follow = g.globals.DAT_005e89d0;
    let keep = follow != 0
        && (follow < 0
            || !g.board(this).offset_0xc[vec_index(0xb8)].is_empty()
            || !g.board(this).offset_0xc[vec_index(0xf4)].is_empty());
    if !keep {
        g.globals.DAT_005e89d0 = 0;
    }
    if g.globals.DAT_005e89cc && g.board(this).field_0xb8[8] == 0 {
        g.globals.DAT_005e89cc = false;
    }
    crate::game::board_level::FUN_00537d30(g, this);
    Ok(true)
}

/// port: 00500580 FUN_00500580
/// `DataWriter::WriteLongAt(long, ulong pos)`: patches a long already written.
pub fn FUN_00500580(this: &mut crate::sexy::data_sync::DataWriter, param_1: i32, param_2: usize) {
    if param_2 + 4 <= this.data.len() {
        this.data[param_2..param_2 + 4].copy_from_slice(&param_1.to_le_bytes());
    }
}

/// port: 00546990 FUN_00546990
/// `SaveGame(path)`: version 0x36, the board, the time, the pointer fix-ups; written to
/// the file (the host creates the folder). In the screensaver the shells it earned are
/// handed to the profile's screensaver file and reset.
pub fn FUN_00546990(g: &mut G, this: Ptr, param_1: &str) {
    let w = crate::sexy::data_sync::DataWriter::default();
    let mut sync = crate::sexy::data_sync::DataSync__00513ee0(w);
    sync.offset_0x8 = 0x36;
    let DataIo::Write(w) = &mut sync.io else { unreachable!() };
    crate::sexy::data_sync::FUN_005004e0(w, 0x36);
    let _ = FUN_00546240(g, this, &mut sync);
    let t = g.now_time64 as i32;
    let DataIo::Write(w) = &mut sync.io else { unreachable!() };
    crate::sexy::data_sync::FUN_005004e0(w, t);
    let _ = crate::sexy::data_sync::FUN_00512310(g, &mut sync);
    if let DataIo::Write(w) = std::mem::replace(&mut sync.io, DataIo::Write(Default::default())) {
        g.vfs.write(param_1, w.data);
    }
    let app = g.board(this).field_0x0;
    crate::sexy::sexy_app_base::FUN_00479f90(g, app, false);
    if crate::game::win_fish_app::thunk_FUN_00479fc0(g, app) {
        let profile = g.wfa(app).offset_0x18c;
        let n = g.board(this).field_0x374;
        crate::game::profile::FUN_00515910(g, profile, n);
        g.board(this).field_0x374 = 0;
    }
    crate::sexy::data_sync::dtor_DataSync(&mut sync);
}

/// port: 00546aa0 FUN_00546aa0
/// `LoadGame(const std::string& path)`: banks screensaver shells (outside the screensaver),
/// then reads the saved game (version 0x35 or later, the right level): the objects, the
/// pointer fix-ups; in the virtual tank, aliens left over from more than two minutes ago
/// are removed. Then the music, the menu button and labels back on top, the shop and HUD
/// refreshed, the game in progress; in the screensaver the shell cap grows with the time
/// played (+0x444; 5,000..50,000); in the virtual tank its setup and resume. False when there is no save
/// or it cannot be used.
pub fn FUN_00546aa0(g: &mut G, this: Ptr, param_1: &str) -> bool {
    let app = g.board(this).field_0x0;
    if !crate::game::win_fish_app::thunk_FUN_00479fc0(g, app) {
        let profile = g.wfa(app).offset_0x18c;
        crate::game::profile::FUN_00514b00(g, profile);
    }
    let Some(bytes) = g.vfs.read(param_1).map(|b| b.to_vec()) else { return false };
    let r = crate::sexy::data_sync::DataReader::from_bytes(bytes);
    let mut sync = crate::sexy::data_sync::DataSync__00513e30(r);
    let ok = (|| -> Result<bool, ReadError> {
        let DataIo::Read(r) = &mut sync.io else { unreachable!() };
        let ver = crate::sexy::data_sync::FUN_00500340(r)?;
        if ver < 0x35 {
            return Ok(false);
        }
        sync.offset_0x8 = ver;
        if !FUN_00546240(g, this, &mut sync)? {
            return Ok(false);
        }
        let DataIo::Read(r) = &mut sync.io else { unreachable!() };
        let saved = crate::sexy::data_sync::FUN_00500340(r)? as u32;
        crate::sexy::data_sync::FUN_00512310(g, &mut sync)?;
        if g.wfa(app).offset_0x150 == 5 {
            let now = g.now_time64;
            // Stale unless 0 <= now - saved < 121 seconds (unsigned 64-bit compare).
            let fresh = 0 <= now && (now as u64).wrapping_sub(saved as u64) < 0x79;
            if !fresh {
                FUN_005437b0(g, this);
            }
        }
        Ok(true)
    })();
    if !matches!(ok, Ok(true)) {
        crate::sexy::data_sync::dtor_DataSync(&mut sync);
        return false;
    }
    FUN_00538940(g, this);
    let mgr = g.wc(this).offset_0xc;
    let (a, b, c) = (g.board(this).offset_0x88, g.board(this).offset_0x3ac, g.board(this).offset_0x8c);
    vcall!(g, mgr, w.vfunction4, a);
    vcall!(g, mgr, w.vfunction4, b);
    vcall!(g, mgr, w.vfunction4, c);
    crate::game::board_level::FUN_0053a360(g, this);
    crate::game::board_level::FUN_00539fe0(g, this);
    crate::sexy::sexy_app_base::FUN_00479f90(g, app, false);
    g.board(this).ext_0x4ee = true;
    crate::game::board_level::FUN_0053aa40(g, this);
    if crate::game::win_fish_app::thunk_FUN_00479fc0(g, app) {
        let played = (g.board(this).ext_0x444 / 36) as f64;
        let mut cap = crate::sexy::crt::ftol(played * f64::from_bits(0x3f77b425ed097b42) + 5000.0) as i32;
        if cap < 0xc351 {
            if cap < 5000 {
                cap = 5000;
            }
        } else {
            cap = 50000;
        }
        g.globals.DAT_005df590 = cap;
    }
    if g.wfa(app).offset_0x150 == 5 {
        FUN_0053e280(g, this);
        crate::game::board_update::FUN_0053a4f0(g, this);
        crate::game::board_update::FUN_0053a450(g, this);
        let b = g.board(this).field_0x21a;
        crate::game::board_update::FUN_00539ad0(g, this, b);
        FUN_0053aa00(g, this);
    }
    crate::sexy::data_sync::dtor_DataSync(&mut sync);
    true
}

/// port: 005437b0 FUN_005437b0
/// Clears out a stale virtual-tank save: every object that is not the tank's own (bought
/// creatures, shadows and coins being carried stay) is destroyed.
pub fn FUN_005437b0(g: &mut G, this: Ptr) {
    let mut doomed = std::collections::BTreeSet::new();
    let objs: Vec<Ptr> = g.board(this).offset_0x7c.iter().copied().collect();
    for o in objs {
        let ty = g.go(o).offset_0x4;
        if ty != 0x1f && (ty != 0x19 || g.coin(o).offset_0x3c == NULL) && g.go(o).offset_0x24 < 0 {
            crate::game::board_level::FUN_00540ec0(&mut doomed, o);
        }
    }
    for o in doomed {
        crate::game::alien::FUN_004d6830(g, o, true);
    }
}

/// port: 00538940 FUN_00538940
/// `ResumeMusic()`: with aliens (or a boss, or the boss level's state 1) the alien
/// music; otherwise during a level's countdown (2..0x113) the tank music resumes (the
/// virtual tank's only with +0x4fe), else the plain tank music (the virtual tank's own).
pub fn FUN_00538940(g: &mut G, this: Ptr) {
    use crate::game::win_fish_app::{FUN_0054b020, FUN_0054b2d0, FUN_0054c480};
    let app = g.board(this).field_0x0;
    let t = g.board(this).field_0x234;
    let quiet = t != 1
        && g.board(this).offset_0xc[vec_index(0xb8)].is_empty()
        && g.board(this).offset_0xc[vec_index(0xf4)].is_empty()
        && g.board(this).field_0x84 == NULL;
    if quiet {
        if t < 2 || 0x113 < t {
            if g.wfa(app).offset_0x150 != 5 {
                FUN_0054b2d0(g, app);
                return;
            }
            FUN_0054b020(g, app);
            return;
        }
        FUN_0054b020(g, app);
        if g.wfa(app).offset_0x150 == 5 && !g.board(this).ext_0x4fe {
            return;
        }
        FUN_0054c480(g, app, false);
        return;
    }
    FUN_0054b020(g, app);
    crate::game::win_fish_app::FUN_0054b1a0(g, app, 1, 3, false);
}

/// port: 0053aa00 FUN_0053aa00
/// After loading the virtual tank (not in the screensaver): its resume, and the welcome
/// dialog unless +0x2b8.
pub fn FUN_0053aa00(g: &mut G, this: Ptr) {
    let app = g.board(this).field_0x0;
    if !crate::game::win_fish_app::thunk_FUN_00479fc0(g, app) && g.wfa(app).offset_0x150 == 5 {
        crate::game::virtual_tank::FUN_0053a980(g, this);
        if !g.board(this).field_0x22c {
            crate::game::win_fish_app::FUN_0054b4e0(g, app);
        }
    }
}

/// port: 0053e280 FUN_0053e280
/// `SetupVirtualTank()` (once): the message line widened along the top (lower in the
/// screensaver), and the six side buttons along the top bar with their icons and captions:
/// Visit Store (STORE), Add/Remove Fish (FISH), Add/Remove Pets (PETS), Special Food
/// (FEED), Tank Options (TANK), Back to Main Menu (BACK).
pub fn FUN_0053e280(g: &mut G, this: Ptr) {
    if g.board(this).field_0x394 != NULL {
        return;
    }
    let app = g.board(this).field_0x0;
    let msg = g.board(this).offset_0x8c;
    if msg != NULL {
        g.wc(msg).offset_0x2c = 0x14;
        g.wc(msg).offset_0x34 = 600;
        if crate::game::win_fish_app::thunk_FUN_00479fc0(g, app) {
            g.wc(msg).offset_0x30 = 0x3c;
        }
    }
    let specs: [(i32, &[u8], i32, &[u8], i32); 6] = [
        (10, b"Visit Store", 0x46, b"STORE", 2),
        (11, b"Add/Remove Fish", 0x90, b"FISH", 3),
        (12, b"Add/Remove Pets", 0xd8, b"PETS", 4),
        (13, b"Tank Options", 0x16c, b"TANK", 5),
        (14, b"Special Food", 0x122, b"FEED", 1),
        (15, b"Back to Main Menu", 0x1b6, b"BACK", 0),
    ];
    for (i, (id, tip, x, caption, col)) in specs.into_iter().enumerate() {
        let mgr = g.wc(this).offset_0xc;
        let b = crate::game::menu_button::MenuButtonWidget(g, mgr, id, this, tip);
        match i {
            0 => g.board(this).field_0x394 = b,
            1 => g.board(this).field_0x398 = b,
            2 => g.board(this).field_0x39c = b,
            3 => g.board(this).field_0x3a0 = b,
            4 => g.board(this).field_0x3a4 = b,
            _ => g.board(this).field_0x3a8 = b,
        }
        let (img, over, down) = (g.res.DAT_005e8c60, g.res.DAT_005e8e74, g.res.DAT_005e8d64);
        let bd = g.btn(b);
        bd.offset_0x28 = img;
        bd.offset_0x2c = over;
        bd.offset_0x30 = down;
        vcall!(g, b, w.vfunction41, x, 2, 0x3a, 0x3c);
        let mgr = g.wc(this).offset_0xc;
        vcall!(g, mgr, w.vfunction4, b);
        crate::game::menu_button::FUN_00534430(g, b, caption);
        let icon = g.res.DAT_005e8e28;
        crate::game::menu_button::FUN_00530410(g, b, icon, 5, 1, 0, col);
    }
}
