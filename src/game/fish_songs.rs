//! The fish songs: `fishsongs\*.txt`, a small note language (up to three parallel lines,
//! `*` commands, comma-separated notes such as `c#5 ed` or `r q+e`) the virtual tank's fish
//! sing in. `FUN_005166f0` loads every file into `Song`s (100 bytes each) and sorts them
//! into the play lists by their `*attrib` tags.
//!
//! The original parses with `fopen` / `fgets` / `strtok`; the port reads the same bytes
//! from the in-memory file system and splits them the same way. Parse errors go to the
//! global error string (`DAT_005e0f30`), exactly as there; a file that fails is dropped
//! and listed in `fishsongerror.txt`.

use crate::game::timed_messages::{FUN_00511830, Note};
use crate::sexy::prelude::*;
use std::collections::BTreeMap;

/// One song (`DAT_005e8f68` element, 100 bytes).
#[derive(Debug, Clone, Default)]
pub struct Song {
    /// +0x00 the three lines (`std::vector<Note>`, 0x10 bytes each).
    pub field_0x0: [Vec<Note>; 3],
    /// +0x30 each line's volume (`*volume`).
    pub field_0x30: [f32; 3],
    /// +0x3c tempo multiplier for durations (`*speed`, 3.0 by default).
    pub field_0x3c: f32,
    /// +0x40 transposition of every line (`*shift`).
    pub field_0x40: i32,
    /// +0x44 each line's own transposition (`*localshift`).
    pub field_0x44: [i32; 3],
    /// +0x50 skipping notes (`*off` / `*skip true`).
    pub field_0x50: bool,
    /// +0x54 the line notes go to (`*line n`, 0-based).
    pub field_0x54: i32,
    /// +0x58 `*attrib key = value` (a `std::map` ordered with `_stricmp`: keys compare
    /// ignoring case, kept here lowercased).
    pub field_0x58: BTreeMap<Vec<u8>, Vec<u8>>,
}

/// The loaded songs and their play lists.
#[derive(Debug, Clone, Default)]
pub struct SongLibrary {
    /// `DAT_005e8f15`: loaded.
    pub DAT_005e8f15: bool,
    /// `DAT_005e8f68`: every song that parsed.
    pub DAT_005e8f68: Vec<Song>,
    /// `DAT_005e8f2c`: the songs played normally (indices into the songs).
    pub DAT_005e8f2c: Vec<usize>,
    /// `DAT_005e8f48`: the Beethoven songs (`beethoven`, `beethovenrare`).
    pub DAT_005e8f48: Vec<usize>,
    /// `DAT_005e8f58`: the Christmas songs (`santa`, `santarare`).
    pub DAT_005e8f58: Vec<usize>,
    /// `DAT_005e8f3c`: the long version of a song by name (`long` attribute; a case-sensitive
    /// `std::map<std::string, Song*>`).
    pub DAT_005e8f3c: BTreeMap<Vec<u8>, usize>,
    /// `DAT_005e8f20`: the `kilgore` song.
    pub DAT_005e8f20: Option<usize>,
    /// `DAT_005e8f24`: the `test` song.
    pub DAT_005e8f24: Option<usize>,
    /// `DAT_005e8f9c`: the next normal song.
    pub DAT_005e8f9c: i32,
    /// `DAT_005e8fa0`: the next Christmas song.
    pub DAT_005e8fa0: i32,
    /// `DAT_005e8fa4`: the next Beethoven song.
    pub DAT_005e8fa4: i32,
    /// `DAT_005e8f98`: songs picked since the last long version (counts up to 5).
    pub DAT_005e8f98: i32,
}

fn lower(s: &[u8]) -> Vec<u8> {
    s.to_ascii_lowercase()
}

/// `FUN_0041d940` / `FUN_00426be0` on the global error string.
fn set_error(g: &mut G, s: &[u8]) {
    g.globals.DAT_005e0f30 = s.to_vec();
}

fn error_set(g: &G) -> bool {
    !g.globals.DAT_005e0f30.is_empty()
}

/// `FUN_004102f0`: whitespace trimmed from both ends.
fn trim(s: &[u8]) -> &[u8] {
    let ws = |c: &u8| matches!(*c, b' ' | b'\t' | b'\r' | b'\n');
    let start = s.iter().position(|c| !ws(c)).unwrap_or(s.len());
    let end = s.iter().rposition(|c| !ws(c)).map_or(start, |e| e + 1);
    &s[start..end]
}

/// `sscanf("%s...")`: the whitespace-separated words.
fn words(s: &[u8]) -> Vec<&[u8]> {
    s.split(|c| matches!(*c, b' ' | b'\t' | b'\r' | b'\n' | 0x0b | 0x0c)).filter(|w| !w.is_empty()).collect()
}

/// CRT `atol`.
fn atol(s: &[u8]) -> i32 {
    let s = trim(s);
    let (neg, digits) = match s.first() {
        Some(b'-') => (true, &s[1..]),
        Some(b'+') => (false, &s[1..]),
        _ => (false, s),
    };
    let mut v: i32 = 0;
    for &c in digits {
        if !c.is_ascii_digit() {
            break;
        }
        v = v.wrapping_mul(10).wrapping_add((c - b'0') as i32);
    }
    if neg { v.wrapping_neg() } else { v }
}

/// CRT `atof` (the leading number).
fn atof(s: &[u8]) -> f64 {
    let s = trim(s);
    let mut end = 0;
    let mut seen_dot = false;
    let mut seen_e = false;
    while end < s.len() {
        let c = s[end];
        let ok = c.is_ascii_digit()
            || ((c == b'-' || c == b'+') && (end == 0 || matches!(s[end - 1], b'e' | b'E')))
            || (c == b'.' && !seen_dot && !seen_e)
            || ((c == b'e' || c == b'E') && !seen_e && end > 0);
        if !ok {
            break;
        }
        seen_dot |= c == b'.';
        seen_e |= c == b'e' || c == b'E';
        end += 1;
    }
    std::str::from_utf8(&s[..end]).ok().and_then(|t| t.parse().ok()).unwrap_or(0.0)
}

/// port: 0050ebe0 FUN_0050ebe0
/// `Song::Reset()`: speed 3, no shift; each line emptied, at full volume, unshifted.
pub fn FUN_0050ebe0(this: &mut Song) {
    this.field_0x3c = 3.0;
    this.field_0x40 = 0;
    for i in 0..3 {
        this.field_0x0[i].clear();
        this.field_0x44[i] = 0;
        this.field_0x30[i] = 1.0;
    }
}

/// port: 00515bb0 FUN_00515bb0
/// `Song::Song()`.
pub fn FUN_00515bb0() -> Song {
    let mut s = Song::default();
    FUN_0050ebe0(&mut s);
    s
}

/// port: 005050b0 FUN_005050b0
/// The total duration of line `param_1`.
pub fn FUN_005050b0(this: &Song, param_1: usize) -> u32 {
    this.field_0x0[param_1].iter().fold(0u32, |a, n| a.wrapping_add(n.field_0x10))
}

/// port: 00513a60 FUN_00513a60
/// `map<string, string, stricmp>::operator[]`: the attribute's value (inserted empty).
pub fn FUN_00513a60<'a>(this: &'a mut BTreeMap<Vec<u8>, Vec<u8>>, param_1: &[u8]) -> &'a mut Vec<u8> {
    this.entry(lower(param_1)).or_default()
}

/// port: 0050ecc0 FUN_0050ecc0
/// `Song::HasAttrib(const string& key, string* value)`: whether the attribute is set,
/// copying its value when asked.
pub fn FUN_0050ecc0(this: &Song, param_1: &[u8], param_2: Option<&mut Vec<u8>>) -> bool {
    match this.field_0x58.get(&lower(param_1)) {
        Some(v) => {
            if let Some(out) = param_2 {
                *out = v.clone();
            }
            true
        }
        None => false,
    }
}

/// port: 0050e860 FUN_0050e860
/// `Note::Parse(const char*)`: `<letter>[#|b][octave] <duration>`: the letters a..g (or `r`,
/// a rest), octave 4 by default; the duration a number or a sum (`+`) of w/h/q/e/s/t/z (192
/// down to 3), each optionally a triplet (`t`, two thirds) and dotted (`d`, `dd`, ...). Errors
/// go to the global error string; a note with an unknown accidental is silently left at
/// pitch 0.
pub fn FUN_0050e860(g: &mut G, param_1: &[u8]) -> Note {
    let mut n = Note { field_0x0: 0.0, field_0x8: 1.0, field_0x10: 0 };
    let w = words(param_1);
    if w.len() != 2 {
        let mut e = b"Invalid note".to_vec();
        if w.len() == 3 {
            e.extend_from_slice(b" (missing comma?)");
        }
        set_error(g, &e);
        return n;
    }
    let name = w[0];
    let dur = w[1];
    let at = |s: &[u8], i: usize| s.get(i).copied().unwrap_or(0);
    let mut semi: i32 = match at(name, 0).to_ascii_lowercase() {
        b'a' => 9,
        b'b' => 0xb,
        b'c' => 0,
        b'd' => 2,
        b'e' => 4,
        b'f' => 5,
        b'g' => 7,
        b'r' => -10000,
        _ => {
            set_error(g, b"Invalid note");
            return n;
        }
    };
    let mut octave = 4;
    let c1 = at(name, 1);
    if c1 != 0 {
        if !c1.is_ascii_digit() {
            match c1 {
                b'#' => semi += 1,
                b'b' => semi -= 1,
                _ => return n,
            }
            let c2 = at(name, 2);
            if c2.is_ascii_digit() {
                octave = (c2 - b'0') as i32;
            }
        } else {
            octave = (c1 - b'0') as i32;
        }
    }
    if at(dur, 0).is_ascii_digit() {
        n.field_0x10 = atol(dur) as u32;
    } else {
        let mut i = 0;
        while at(dur, i) != 0 {
            let mut base: u32 = match at(dur, i).to_ascii_lowercase() {
                b'e' => 0x18,
                b'h' => 0x60,
                b'q' => 0x30,
                b's' => 0xc,
                b't' => 6,
                b'w' => 0xc0,
                b'z' => 3,
                _ => {
                    set_error(g, b"Invalid duration");
                    return n;
                }
            };
            let mut p = i + 1;
            if at(dur, p).to_ascii_lowercase() == b't' {
                base = (base * 2) / 3;
                p = i + 2;
            }
            let mut total = base;
            while at(dur, p).to_ascii_lowercase() == b'd' {
                p += 1;
                base = ((base as i32) / 2) as u32;
                total = total.wrapping_add(base);
            }
            n.field_0x10 = n.field_0x10.wrapping_add(total);
            if at(dur, p) != b'+' {
                if at(dur, p) != 0 {
                    set_error(g, b"Invalid note (missing comma?)");
                }
                break;
            }
            i = p + 1;
        }
    }
    n.field_0x0 = (semi + (octave * 3 - 0xc) * 4) as f32;
    n
}

/// port: 00514f30 FUN_00514f30
/// A `*` command line: `skip true|false`, and unless skipping `line n` (1..3), `rest n`
/// (pads this line with a rest up to line n's length), `attrib key = value`, `speed`,
/// `volume` (this line), `shift`, `localshift` (this line); a single word `on` / `off`.
pub fn FUN_00514f30(g: &mut G, this: &mut Song, param_1: &[u8]) {
    let w = words(&param_1[1..]);
    if w.len() >= 2 {
        let (cmd, arg) = (lower(w[0]), w[1]);
        if cmd == b"skip" {
            this.field_0x50 = lower(arg) == b"true";
            return;
        }
        if this.field_0x50 {
            return;
        }
        match cmd.as_slice() {
            b"line" => {
                let n = atol(arg);
                if (n.wrapping_sub(1) as u32) < 3 {
                    this.field_0x54 = n - 1;
                } else {
                    set_error(g, b"Invalid line");
                }
            }
            b"rest" => {
                let n = atol(arg);
                if (n.wrapping_sub(1) as u32) < 3 {
                    let cur = FUN_005050b0(this, this.field_0x54 as usize);
                    let other = FUN_005050b0(this, (n - 1) as usize);
                    if cur < other {
                        let rest = Note { field_0x0: f32::from_bits(0xc61c4000), field_0x8: 1.0, field_0x10: other - cur };
                        FUN_00511830(&mut this.field_0x0[this.field_0x54 as usize], rest);
                    }
                }
            }
            b"attrib" => {
                let mut value = Vec::new();
                if let Some(eq) = param_1.iter().position(|&c| c == b'=') {
                    value = trim(&param_1[eq + 1..]).to_vec();
                }
                *FUN_00513a60(&mut this.field_0x58, arg) = value;
            }
            b"speed" => this.field_0x3c = atof(arg) as f32,
            b"volume" => this.field_0x30[this.field_0x54 as usize] = atof(arg) as f32,
            b"shift" => this.field_0x40 = atol(arg),
            b"localshift" => this.field_0x44[this.field_0x54 as usize] = atol(arg),
            _ => set_error(g, b"Unrecognized command"),
        }
        return;
    }
    let cmd = w.first().map(|c| lower(c)).unwrap_or_default();
    if cmd == b"on" {
        this.field_0x50 = false;
    } else if cmd == b"off" {
        this.field_0x50 = true;
    } else {
        set_error(g, b"Unrecognized command");
    }
}

/// port: 005152f0 FUN_005152f0
/// `Song::Load(const char* path)`: resets, then reads the file a line (up to 7999 bytes) at
/// a time until the end or an error: `#` comments, `*` commands, else (unless skipping) the
/// comma-separated notes onto the current line, each transposed by the shifts, its duration
/// scaled by the speed (rounded) and given the line's volume. Afterwards shift 0 and speed
/// 1; an error becomes "<error>: <path> line <n>". True when there was no error.
pub fn FUN_005152f0(g: &mut G, this: &mut Song, param_1: &str) -> bool {
    FUN_0050ebe0(this);
    set_error(g, b"");
    this.field_0x50 = false;
    let Some(bytes) = g.vfs.read(param_1).map(|b| b.to_vec()) else {
        set_error(g, format!("File not found: {param_1}").as_bytes());
        return false;
    };
    this.field_0x54 = 0;
    let mut line_no = 0;
    let mut pos = 0;
    while pos < bytes.len() && !error_set(g) {
        line_no += 1;
        // fgets(buf, 8000): up to and including the newline, at most 7999 bytes.
        let rest = &bytes[pos..];
        let mut n = rest.iter().position(|&c| c == b'\n').map_or(rest.len(), |i| i + 1);
        n = n.min(7999);
        let line = rest[..n].to_vec();
        pos += n;
        match line.first() {
            Some(b'#') => {}
            Some(b'*') => FUN_00514f30(g, this, &line),
            _ if !this.field_0x50 => {
                for tok in line.split(|&c| c == b',').filter(|t| !t.is_empty()) {
                    let t = trim(tok);
                    if t.is_empty() {
                        continue;
                    }
                    let note = FUN_0050e860(g, t);
                    let l = this.field_0x54 as usize;
                    FUN_00511830(&mut this.field_0x0[l], note);
                    let shift = this.field_0x44[l] + this.field_0x40;
                    let speed = this.field_0x3c;
                    let vol = this.field_0x30[l];
                    let last = this.field_0x0[l].last_mut().unwrap();
                    last.field_0x0 += shift as f32;
                    let d = (last.field_0x10 as f32 as f64 * speed as f64).round_ties_even();
                    last.field_0x10 = d as i64 as u32;
                    last.field_0x8 = vol as f64;
                    if error_set(g) {
                        break;
                    }
                }
            }
            _ => {}
        }
    }
    if !error_set(g) {
        this.field_0x40 = 0;
        this.field_0x3c = 1.0;
        return true;
    }
    let e = String::from_utf8_lossy(&g.globals.DAT_005e0f30).to_string();
    set_error(g, format!("{e}: {param_1} line {line_no}").as_bytes());
    false
}

/// port: 005166f0 FUN_005166f0
/// `LoadFishSongs(bool reload)` (once unless reloading): forgets the old songs, loads every
/// `fishsongs\*.txt` (failures are dropped and listed in `fishsongerror.txt`), gives the
/// `<name>_long` / `<name>_short` files their `long` / `short` attribute, then sorts the
/// songs: `test` is the test song; `long` ones are only reachable by name; `beethovenrare`
/// / `santarare` only go to their own lists, `beethoven` / `santa` to theirs and the normal
/// list; `kilgore` is remembered; the rest are normal.
pub fn FUN_005166f0(g: &mut G, param_1: bool) {
    if !param_1 && g.globals.songs.DAT_005e8f15 {
        return;
    }
    g.globals.songs.DAT_005e8f15 = true;
    let lib = &mut g.globals.songs;
    lib.DAT_005e8f68.clear();
    lib.DAT_005e8f2c.clear();
    lib.DAT_005e8f48.clear();
    lib.DAT_005e8f3c.clear();
    lib.DAT_005e8f20 = None;
    lib.DAT_005e8f24 = None;
    let files = g.vfs.find_files("fishsongs", ".txt");
    let mut errors: Option<Vec<u8>> = None;
    for name in files {
        let path = format!("fishsongs\\{name}");
        // (The original pushes the new song and parses it in place, popping it on failure.)
        let mut song = FUN_00515bb0();
        if FUN_005152f0(g, &mut song, &path) {
            g.globals.songs.DAT_005e8f68.push(song);
        } else {
            let log = errors.get_or_insert_with(Vec::new);
            log.extend_from_slice(name.as_bytes());
            log.extend_from_slice(b" - ");
            log.extend_from_slice(&g.globals.DAT_005e0f30);
            log.push(b'\n');
        }
        if let Some(us) = name.find('_') {
            let (prefix, suffix) = (&name[..us], &name[us + 1..]);
            let key: Option<&[u8]> = if suffix.eq_ignore_ascii_case("long.txt") {
                Some(b"long")
            } else if suffix.eq_ignore_ascii_case("short.txt") {
                Some(b"short")
            } else {
                None
            };
            if let (Some(k), Some(last)) = (key, g.globals.songs.DAT_005e8f68.last_mut()) {
                *FUN_00513a60(&mut last.field_0x58, k) = prefix.as_bytes().to_vec();
            }
        }
    }
    if let Some(log) = errors {
        g.vfs.write("fishsongerror.txt", log);
    }
    let lib = &mut g.globals.songs;
    for i in 0..lib.DAT_005e8f68.len() {
        let s = &lib.DAT_005e8f68[i];
        let mut normal = true;
        let mut value = Vec::new();
        if FUN_0050ecc0(s, b"test", None) {
            lib.DAT_005e8f24 = Some(i);
        } else if FUN_0050ecc0(s, b"long", Some(&mut value)) {
            lib.DAT_005e8f3c.insert(value, i);
            normal = false;
        } else if FUN_0050ecc0(s, b"beethovenrare", None) {
            lib.DAT_005e8f48.push(i);
            normal = false;
        } else if FUN_0050ecc0(s, b"beethoven", None) {
            lib.DAT_005e8f48.push(i);
        } else if FUN_0050ecc0(s, b"santarare", None) {
            lib.DAT_005e8f58.push(i);
            normal = false;
        } else if FUN_0050ecc0(s, b"santa", None) {
            lib.DAT_005e8f58.push(i);
        }
        if FUN_0050ecc0(&lib.DAT_005e8f68[i], b"kilgore", None) {
            lib.DAT_005e8f20 = Some(i);
        }
        if normal {
            lib.DAT_005e8f2c.push(i);
        }
    }
}

/// port: 005049a0 FUN_005049a0
/// `std::random_shuffle(first, last)` over a song list (`FUN_005047c0`: each element from
/// the second on swapped with one at `rand() % (i + 1)`).
pub fn FUN_005049a0(g: &mut G, param_1: &mut [usize]) {
    let mut i = 1;
    while i < param_1.len() {
        let r = crate::sexy::sexy_app_base::FUN_0040f9e0(g) as i32 % (i as i32 + 1);
        param_1.swap(i, r as usize);
        i += 1;
    }
}

/// port: 0050fd00 FUN_0050fd00
/// `PickSong(int kind)`: none while the songs load; kind 4 is Kilgore's song; a `test` song
/// always wins; otherwise the next of the Beethoven (1), Christmas (5) or normal list, the
/// list reshuffled each time it starts over. Every fifth pick of a song with a `short`
/// version name plays its long version instead (Beethoven songs only for kind 1).
pub fn FUN_0050fd00(g: &mut G, param_1: i32) -> Option<usize> {
    if g.globals.DAT_005e8f16 {
        return None;
    }
    if param_1 == 4 {
        return g.globals.songs.DAT_005e8f20;
    }
    if g.globals.songs.DAT_005e8f24.is_some() {
        return g.globals.songs.DAT_005e8f24;
    }
    let lib = &mut g.globals.songs;
    let (mut list, idx) = match param_1 {
        1 => (std::mem::take(&mut lib.DAT_005e8f48), lib.DAT_005e8fa4),
        5 => (std::mem::take(&mut lib.DAT_005e8f58), lib.DAT_005e8fa0),
        _ => (std::mem::take(&mut lib.DAT_005e8f2c), lib.DAT_005e8f9c),
    };
    let put_back = |g: &mut G, list: Vec<usize>, idx: i32| {
        let lib = &mut g.globals.songs;
        match param_1 {
            1 => { lib.DAT_005e8f48 = list; lib.DAT_005e8fa4 = idx; }
            5 => { lib.DAT_005e8f58 = list; lib.DAT_005e8fa0 = idx; }
            _ => { lib.DAT_005e8f2c = list; lib.DAT_005e8f9c = idx; }
        }
    };
    if list.is_empty() {
        put_back(g, list, idx);
        return None;
    }
    let mut idx = idx;
    if idx >= list.len() as i32 {
        idx = 0;
        FUN_005049a0(g, &mut list);
    } else if idx == 0 {
        FUN_005049a0(g, &mut list);
    }
    let song = list[idx as usize];
    put_back(g, list, idx + 1);
    let lib = &mut g.globals.songs;
    if lib.DAT_005e8f98 < 5 {
        lib.DAT_005e8f98 += 1;
    }
    let mut short = Vec::new();
    if !FUN_0050ecc0(&lib.DAT_005e8f68[song], b"short", Some(&mut short)) {
        return Some(song);
    }
    let beethoven = FUN_0050ecc0(&lib.DAT_005e8f68[song], b"beethoven", None) && param_1 != 1;
    if beethoven || lib.DAT_005e8f98 != 5 {
        return Some(song);
    }
    match lib.DAT_005e8f3c.get(&short) {
        Some(&long) => {
            lib.DAT_005e8f98 = 0;
            Some(long)
        }
        None => Some(song),
    }
}

/// The virtual tank's background job queued by `StartGame` (`LAB_0054b280`, no function
/// of its own in the database): loads the fish songs, then clears `DAT_005e8f16`.
pub fn load_songs_job(g: &mut G, _param_1: i32) {
    FUN_005166f0(g, false);
    g.globals.DAT_005e8f16 = false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn note_parse() {
        let mut g = G::default();
        let n = FUN_0050e860(&mut g, b"c#5 ed");
        assert!(g.globals.DAT_005e0f30.is_empty());
        assert_eq!(n.field_0x0, 13.0);
        assert_eq!(n.field_0x10, 0x18 + 0xc);
        let n = FUN_0050e860(&mut g, b"r q+e");
        assert_eq!(n.field_0x0, -10000.0);
        assert_eq!(n.field_0x10, 0x30 + 0x18);
        let n = FUN_0050e860(&mut g, b"f3 tt");
        assert_eq!(n.field_0x0, -7.0);
        assert_eq!(n.field_0x10, 4);
    }

    #[test]
    fn song_parse() {
        let mut g = G::default();
        let text = b"*attrib title = O' Susanna\n*speed 7.5\n\n*on\n*line 1\nf4 s, g4 s,\n*line 2\n*volume 0.5\nr e,\n*rest 1\n";
        g.vfs = crate::sexy::vfs::Vfs::from_files([("fishsongs/x.txt".to_string(), text.to_vec())]);
        let mut s = FUN_00515bb0();
        assert!(FUN_005152f0(&mut g, &mut s, "fishsongs/x.txt"));
        assert_eq!(s.field_0x58.get(&b"title".to_vec()).unwrap(), b"O' Susanna");
        assert_eq!(s.field_0x0[0].len(), 2);
        assert_eq!(s.field_0x0[0][0].field_0x10, 90);
        assert_eq!(s.field_0x0[1][0].field_0x8, 0.5);
        assert_eq!(s.field_0x0[1][0].field_0x10, 180);
        assert!(s.field_0x0[1].len() == 1);
    }

    /// Every shipped song parses (skipped when the install is not next to the crate).
    #[test]
    fn shipped_songs() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../Insaniquarium Deluxe/fishsongs");
        if !dir.exists() {
            return;
        }
        let mut files = Vec::new();
        for e in std::fs::read_dir(&dir).unwrap() {
            let p = e.unwrap().path();
            let name = p.file_name().unwrap().to_string_lossy().to_string();
            files.push((format!("fishsongs/{name}"), std::fs::read(&p).unwrap()));
        }
        let mut g = G::default();
        g.vfs = crate::sexy::vfs::Vfs::from_files(files);
        FUN_005166f0(&mut g, true);
        assert!(g.vfs.read("fishsongerror.txt").is_none(), "{}", String::from_utf8_lossy(g.vfs.read("fishsongerror.txt").unwrap_or(b"")));
        let lib = &g.globals.songs;
        assert!(lib.DAT_005e8f68.len() > 50);
        assert!(!lib.DAT_005e8f2c.is_empty() && !lib.DAT_005e8f3c.is_empty());
    }

    #[test]
    fn pick_song_long_every_fifth() {
        let mut g = G::default();
        let mut short = FUN_00515bb0();
        *FUN_00513a60(&mut short.field_0x58, b"short") = b"x".to_vec();
        let long = FUN_00515bb0();
        let lib = &mut g.globals.songs;
        lib.DAT_005e8f68 = vec![short, long];
        lib.DAT_005e8f2c = vec![0];
        lib.DAT_005e8f3c.insert(b"x".to_vec(), 1);
        let picks: Vec<_> = (0..6).map(|_| FUN_0050fd00(&mut g, 0)).collect();
        assert_eq!(picks, [Some(0), Some(0), Some(0), Some(0), Some(1), Some(0)]);
        assert_eq!(FUN_0050fd00(&mut g, 1), None);
    }
}
