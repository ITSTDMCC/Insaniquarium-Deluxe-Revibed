//! (Port addition) Frame timing log for performance work, off unless `WINFISH_PERF=1`.
//!
//! Every 5 seconds it logs the frame times seen (average, 99th percentile, worst, and how
//! many frames took more than 1.5x the typical one), the logic updates run, and the time
//! spent in each part of the host's frame. Any single frame over 25 ms is also logged with
//! its own breakdown, so a hitch can be traced to what caused it.

use std::sync::Mutex;
use std::time::Instant;

#[derive(Clone, Copy, Debug)]
pub enum Section {
    /// Logic updates (`update_frame`), all the steps of one frame.
    Update,
    /// `DrawScreen` and the 640x480 blits.
    Draw,
    /// Painting the HD screen.
    HdPaint,
    /// Copying the screen into the texture.
    Upload,
    /// The resource-loading slice.
    Loading,
}

const SECTIONS: usize = 5;
const NAMES: [&str; SECTIONS] = ["update", "draw", "hd_paint", "upload", "loading"];

#[derive(Default)]
struct Perf {
    window: Option<Instant>,
    last_frame: Option<Instant>,
    frame_ms: Vec<f32>,
    total: [f64; SECTIONS],
    worst: [f64; SECTIONS],
    this_frame: [f64; SECTIONS],
    updates: u64,
    catchup_caps: u64,
}

static PERF: Mutex<Option<Perf>> = Mutex::new(None);

pub fn enabled() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("WINFISH_PERF").is_some_and(|v| v != "0"))
}

/// Runs `f`, adding its time to `section`.
#[inline]
pub fn span<R>(section: Section, f: impl FnOnce() -> R) -> R {
    if !enabled() {
        return f();
    }
    let t = Instant::now();
    let r = f();
    let ms = t.elapsed().as_secs_f64() * 1000.0;
    PERF.lock().unwrap().get_or_insert_with(Perf::default).this_frame[section as usize] += ms;
    r
}

/// Counts logic updates run this frame, and whether the catch-up limit was hit.
pub fn updates(n: u64, capped: bool) {
    if !enabled() {
        return;
    }
    let mut guard = PERF.lock().unwrap();
    let p = guard.get_or_insert_with(Perf::default);
    p.updates += n;
    p.catchup_caps += capped as u64;
}

/// Called once per host frame (first system of the frame).
pub fn frame(objects: usize) {
    if !enabled() {
        return;
    }
    let now = Instant::now();
    let mut guard = PERF.lock().unwrap();
    let p = guard.get_or_insert_with(Perf::default);
    let start = *p.window.get_or_insert(now);
    if let Some(last) = p.last_frame {
        let dt = (now - last).as_secs_f64() * 1000.0;
        p.frame_ms.push(dt as f32);
        if dt > 25.0 {
            let parts: Vec<String> = (0..SECTIONS).filter(|&i| p.this_frame[i] >= 0.05).map(|i| format!("{} {:.1}", NAMES[i], p.this_frame[i])).collect();
            bevy::log::warn!("perf: slow frame {dt:.1} ms (previous frame's work: {})", if parts.is_empty() { "none".into() } else { parts.join(", ") });
        }
    }
    p.last_frame = Some(now);
    for i in 0..SECTIONS {
        p.total[i] += p.this_frame[i];
        p.worst[i] = p.worst[i].max(p.this_frame[i]);
        p.this_frame[i] = 0.0;
    }
    let secs = (now - start).as_secs_f64();
    if secs < 5.0 || p.frame_ms.is_empty() {
        return;
    }
    let n = p.frame_ms.len();
    let mut sorted = p.frame_ms.clone();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let median = sorted[n / 2];
    let p99 = sorted[(n * 99 / 100).min(n - 1)];
    let worst = sorted[n - 1];
    let avg = sorted.iter().map(|&v| v as f64).sum::<f64>() / n as f64;
    let hitches = sorted.iter().filter(|&&v| v > median * 1.5).count();
    let parts: Vec<String> = (0..SECTIONS)
        .filter(|&i| p.worst[i] > 0.0)
        .map(|i| format!("{} avg {:.2} max {:.1}", NAMES[i], p.total[i] / n as f64, p.worst[i]))
        .collect();
    bevy::log::info!(
        "perf: {:.1} fps, frame avg {avg:.2} median {median:.2} p99 {p99:.2} max {worst:.1} ms, {hitches} frames over 1.5x median, {:.1} updates/s, {} catch-up caps, {objects} object slots | {}",
        n as f64 / secs,
        p.updates as f64 / secs,
        p.catchup_caps,
        parts.join(", ")
    );
    *p = Perf { last_frame: p.last_frame, window: Some(now), ..Perf::default() };
}
