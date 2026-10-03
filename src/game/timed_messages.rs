//! The board's melody player (0x24 bytes, built by `FUN_00515c40`): a list of note
//! sequences played on four pitch-ranged samples, used when objects "speak". Each finished
//! sequence may queue a follow-up sample 400 ms later.

use crate::sexy::prelude::*;

/// One note (0x18 bytes).
#[derive(Debug, Clone, Copy, Default)]
pub struct Note {
    /// +0x00 pitch in semitones (<= -1000 = a rest).
    pub field_0x0: f32,
    /// +0x08 volume.
    pub field_0x8: f64,
    /// +0x10 duration, then (after `FUN_005039d0`) start time in ms.
    pub field_0x10: u32,
}

/// One note sequence (a value of the player's `std::list`).
#[derive(Debug, Clone, Default)]
pub struct Melody {
    /// +0x04 the notes (`std::vector`).
    pub field_0x4: Vec<Note>,
    /// +0x10 the next note to play.
    pub field_0x10: i32,
    /// +0x14 tick it started.
    pub field_0x14: u32,
    /// +0x18 total length in ms.
    pub field_0x18: i32,
    /// +0x1c sample for the middle range.
    pub field_0x1c: i32,
    /// +0x20 sample for notes from 12 up (played an octave down).
    pub field_0x20: i32,
    /// +0x24 sample for notes up to -12 (played an octave up).
    pub field_0x24: i32,
    /// +0x28 sample for notes from 24 up (played two octaves down).
    pub field_0x28: i32,
    /// +0x2c sample to play once this melody has finished (-1 = none).
    pub field_0x2c: i32,
    /// +0x30.
    pub field_0x30: i32,
}

/// The melody player.
#[derive(Debug, Clone, Default)]
pub struct TimedMessages {
    /// +0x04 the melodies (`std::list`; size at +0x08 is `len()`).
    pub field_0x4: Vec<Melody>,
    /// +0x0c sound (`DAT_005e8ec4`).
    pub field_0xc: i32,
    /// +0x10 sound (`DAT_005e8a68`).
    pub field_0x10: i32,
    /// +0x14 sound (`DAT_005e8a7c`).
    pub field_0x14: i32,
    /// +0x18 sound (`DAT_005e8a3c`).
    pub field_0x18: i32,
    /// +0x1c melodies started.
    pub field_0x1c: i32,
    /// +0x20 last tick it ran.
    pub field_0x20: u32,
}

impl G {
    pub fn timed_messages(&mut self, p: Ptr) -> &mut TimedMessages {
        match &mut self.obj(p).node {
            Node::TimedMessages(m) => m,
            n => panic!("{p} is not a timed-message list: {n:?}"),
        }
    }
}

/// port: 00515c40 FUN_00515c40
/// Constructor: empty, sounds unset (-1).
pub fn FUN_00515c40(g: &mut G) -> Ptr {
    let m = TimedMessages {
        field_0x4: Vec::new(),
        field_0xc: -1,
        field_0x10: -1,
        field_0x18: -1,
        field_0x14: -1,
        field_0x1c: 0,
        field_0x20: 0,
    };
    g.alloc(Obj { vt: None, node: Node::TimedMessages(Box::new(m)) })
}

/// port: 00513c20 FUN_00513c20
/// `Clear()`.
pub fn FUN_00513c20(g: &mut G, this: Ptr) {
    g.timed_messages(this).field_0x4.clear();
}

/// port: 00515ec0 FUN_00515ec0
/// `Update()`: once per tick-count change, plays the due notes of every melody, drops the
/// finished ones and queues the last finished melody's follow-up sample (a 400 ms rest,
/// then the sample).
pub fn FUN_00515ec0(g: &mut G, this: Ptr) {
    let tick = g.tick_count;
    if tick == g.timed_messages(this).field_0x20 {
        return;
    }
    g.timed_messages(this).field_0x20 = tick;
    let mut follow = -1;
    let mut i = 0;
    while i < g.timed_messages(this).field_0x4.len() {
        let mut m = g.timed_messages(this).field_0x4[i].clone();
        let more = FUN_00503ac0(g, &mut m, tick, true);
        g.timed_messages(this).field_0x4[i] = m.clone();
        if !more {
            if m.field_0x2c != -1 {
                follow = m.field_0x2c;
            }
            g.timed_messages(this).field_0x4.remove(i);
        } else {
            i += 1;
        }
    }
    if follow != -1 {
        let mut m = Melody::default();
        FUN_00511770(&mut m);
        m.field_0x1c = follow;
        FUN_00511830(&mut m.field_0x4, Note { field_0x0: -10000.0, field_0x8: 1.0, field_0x10: 400 });
        FUN_00511830(&mut m.field_0x4, Note { field_0x0: 0.0, field_0x8: 1.0, field_0x10: 100 });
        FUN_00515e30(g, this, m);
    }
}

/// port: 00511770 FUN_00511770
/// `Melody::Melody()`: no notes, no samples.
pub fn FUN_00511770(this: &mut Melody) {
    this.field_0x4.clear();
    this.field_0x10 = 0;
    this.field_0x14 = 0;
    this.field_0x1c = -1;
    this.field_0x20 = -1;
    this.field_0x24 = -1;
    this.field_0x28 = -1;
    this.field_0x2c = -1;
    this.field_0x30 = -1;
    this.field_0x18 = 0;
}

/// port: 00511830 FUN_00511830
/// `std::vector<Note>::push_back` (an STL instance; `Vec::push`).
pub fn FUN_00511830(this: &mut Vec<Note>, param_1: Note) {
    this.push(param_1);
}

/// port: 00502620 FUN_00502620
/// The number of notes.
pub fn FUN_00502620(this: &Melody) -> i32 {
    this.field_0x4.len() as i32
}

/// port: 00515e30 FUN_00515e30
/// `Add(const Melody&)`: appends it with the player's samples for any range it leaves
/// unset, and starts it at normal speed.
pub fn FUN_00515e30(g: &mut G, this: Ptr, param_1: Melody) {
    let p = g.timed_messages(this).clone();
    let mut m = param_1;
    if m.field_0x1c == -1 {
        m.field_0x1c = p.field_0xc;
    }
    if m.field_0x24 == -1 {
        m.field_0x24 = p.field_0x18;
    }
    if m.field_0x20 == -1 {
        m.field_0x20 = p.field_0x10;
    }
    if m.field_0x28 == -1 {
        m.field_0x28 = p.field_0x14;
    }
    let tick = g.tick_count;
    FUN_005039d0(&mut m, 1.0, 0, tick);
    g.timed_messages(this).field_0x4.push(m);
}

/// port: 00515c70 FUN_00515c70
/// `AddSong(const Song&, int id)`: one melody per line that has notes, tagged `id`, on the
/// player's samples, started at the song's tempo and transposition; the index of the
/// longest (the first of equals), or `None` for a song with no notes.
pub fn FUN_00515c70(g: &mut G, this: Ptr, param_1: &crate::game::fish_songs::Song, param_2: i32) -> Option<usize> {
    let mut best: Option<usize> = None;
    let mut best_len: u32 = 0;
    for line in 0..3 {
        if param_1.field_0x0[line].is_empty() {
            continue;
        }
        let mut m = Melody::default();
        FUN_00511770(&mut m);
        m.field_0x30 = param_2;
        for n in &param_1.field_0x0[line] {
            FUN_00511830(&mut m.field_0x4, *n);
        }
        let p = g.timed_messages(this).clone();
        m.field_0x1c = p.field_0xc;
        m.field_0x24 = p.field_0x18;
        m.field_0x20 = p.field_0x10;
        m.field_0x28 = p.field_0x14;
        let tick = g.tick_count;
        FUN_005039d0(&mut m, param_1.field_0x3c, param_1.field_0x40, tick);
        let len = m.field_0x18 as u32;
        let tm = g.timed_messages(this);
        tm.field_0x4.push(m);
        if best.is_none() || best_len < len {
            best = Some(tm.field_0x4.len() - 1);
            best_len = len;
        }
    }
    best
}

/// x87 `FISTP` in the default rounding mode (to nearest, ties to even).
fn fistp(v: f32) -> i32 {
    let r = (v as f64).round_ties_even();
    r as i64 as i32
}

/// port: 005039d0 FUN_005039d0
/// `Start(float tempo, int transpose)`: turns note durations into start times scaled by
/// `tempo`, transposes, and records the length and the start tick (`GetTickCount`).
pub fn FUN_005039d0(this: &mut Melody, param_1: f32, param_2: i32, tick: u32) {
    let mut acc: f32 = 0.0;
    let mut i = 0;
    while (i as i32) < FUN_00502620(this) {
        let n = &mut this.field_0x4[i];
        let d = n.field_0x10 as f32;
        n.field_0x10 = fistp(acc) as u32;
        n.field_0x0 += param_2 as f32;
        acc += d * param_1;
        i += 1;
    }
    this.field_0x18 = fistp(acc);
    this.field_0x14 = tick;
}

/// port: 00503ac0 FUN_00503ac0
/// `Play(DWORD tick, bool sound)`: plays every note now due on the sample for its range
/// (pitch adjusted into it), at its volume; true while notes remain.
pub fn FUN_00503ac0(g: &mut G, this: &mut Melody, param_1: u32, param_2: bool) -> bool {
    let elapsed = param_1.wrapping_sub(this.field_0x14);
    while this.field_0x10 < FUN_00502620(this) {
        let n = this.field_0x4[this.field_0x10 as usize];
        if elapsed < n.field_0x10 {
            break;
        }
        if -1000.0 < n.field_0x0 as f64 {
            let mut shift = 0;
            let mut id = this.field_0x1c;
            if !(-12.0 < n.field_0x0) {
                if 0 <= this.field_0x24 {
                    shift = 0xc;
                    id = this.field_0x24;
                }
            } else if 12.0 <= n.field_0x0 {
                if 0 <= this.field_0x28 && 24.0 <= n.field_0x0 {
                    id = this.field_0x28;
                    shift = -0x18;
                } else if 0 <= this.field_0x20 {
                    shift = -0xc;
                    id = this.field_0x20;
                }
            }
            if param_2 {
                // SoundManager::GetSoundInstance(id); SetVolume; AdjustPitch; Play(false, true).
                let pitch = shift as f64 + n.field_0x0 as f64;
                g.sound_requests.push(crate::sexy::sexy_app_base::SoundRequest { id, volume: n.field_0x8, pan: 0, pitch });
            }
        }
        this.field_0x10 += 1;
    }
    this.field_0x10 < FUN_00502620(this)
}

/// port: 005051b0 FUN_005051b0
/// `Resume()`: moves every melody's start forward by the time since the list last ran.
pub fn FUN_005051b0(g: &mut G, this: Ptr) {
    let now = g.tick_count;
    let t = g.timed_messages(this);
    let d = now.wrapping_sub(t.field_0x20);
    for m in t.field_0x4.iter_mut() {
        m.field_0x14 = m.field_0x14.wrapping_add(d);
    }
}

/// port: 00505120 FUN_00505120
/// How long the oldest melody with id `param_1` has been playing (0 when none).
pub fn FUN_00505120(g: &mut G, this: Ptr, param_1: i32) -> u32 {
    let t = g.timed_messages(this);
    let mut longest = 0u32;
    for m in &t.field_0x4 {
        let age = t.field_0x20.wrapping_sub(m.field_0x14);
        if m.field_0x30 == param_1 && longest < age {
            longest = age;
        }
    }
    longest
}

/// port: 00505170 FUN_00505170
/// Whether a melody with id `param_1` is in the list.
pub fn FUN_00505170(g: &mut G, this: Ptr, param_1: i32) -> bool {
    g.timed_messages(this).field_0x4.iter().any(|m| m.field_0x30 == param_1)
}

/// port: 005156f0 FUN_005156f0
/// Removes every melody with id `param_1`.
pub fn FUN_005156f0(g: &mut G, this: Ptr, param_1: i32) {
    g.timed_messages(this).field_0x4.retain(|m| m.field_0x30 != param_1);
}
