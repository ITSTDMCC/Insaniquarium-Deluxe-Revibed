//! `Sexy::BassMusicInterface` (framework, @ 00498750..004998e5): the game's music state,
//! following the original's rules exactly; the host (`crate::host::music`) turns it into
//! sound by playing the modules with libopenmpt, as the original did with BASS.
//!
//! Per song (`BassMusicInfo`): +0x20 volume, +0x28 volume step per update, +0x30 volume
//! cap (1.0), +0x38 stop when a fade reaches 0. The channel's state (playing, and the
//! order to (re)start from) is what the original handed to BASS; here it is recorded for
//! the host, which applies it on the audio thread. Volumes reach BASS as whole percents
//! (`ftol(volume * 100)`), so they are quantised the same way.

use std::collections::BTreeMap;

/// One loaded song (`BassMusicInfo`).
#[derive(Clone, Debug, Default)]
pub struct MusicInfo {
    /// The module file (game-relative path).
    pub file: String,
    /// +0x20 volume (0..cap).
    pub volume: f64,
    /// +0x28 volume step applied every update (fades).
    pub volume_add: f64,
    /// +0x30 volume cap.
    pub volume_cap: f64,
    /// +0x38 a fade to 0 stops the song.
    pub stop_on_fade: bool,
    /// The BASS channel is playing.
    pub playing: bool,
    /// Bumped on every `BASS_MusicPlayEx` (a restart from `order`).
    pub start_serial: u32,
    /// The order the last restart began at.
    pub order: i32,
    /// The last restart looped (`BASS_MUSIC_LOOP`, i.e. not `noLoop`).
    pub looping: bool,
}

/// The music interface (`mMusicInterface`): songs by id, and the global music volume.
#[derive(Clone, Debug, Default)]
pub struct MusicInterface {
    pub songs: BTreeMap<i32, MusicInfo>,
    /// The global music volume BASS was given by `SetVolume` (0..1).
    pub volume: f64,
    /// The plain `MusicInterface` the screensaver makes (`WinFishApp::vfunction19`): every
    /// call does nothing.
    pub silent: bool,
}

impl MusicInterface {
    fn song(&mut self, id: i32) -> Option<&mut MusicInfo> {
        if self.silent {
            return None;
        }
        self.songs.get_mut(&id)
    }
}

/// `BassMusicInfo::GetVolume` as BASS receives it: whole percents.
pub fn channel_volume(info: &MusicInfo) -> f64 {
    crate::sexy::crt::ftol(info.volume * 100.0) as f64 / 100.0
}

/// `vfunction2` `LoadMusic(int songId, const string& file)`: volume 0, cap 1.
pub fn load_music(m: &mut MusicInterface, id: i32, file: &str) {
    if m.silent {
        return;
    }
    m.songs.insert(id, MusicInfo { file: file.to_string(), volume: 0.0, volume_add: 0.0, volume_cap: 1.0, ..Default::default() });
}

fn restart(s: &mut MusicInfo, order: i32, looping: bool) {
    s.playing = true;
    s.order = order;
    s.looping = looping;
    s.start_serial = s.start_serial.wrapping_add(1);
}

/// `vfunction3` `PlayMusic(int songId, int offset, bool noLoop)`: full volume at once, no
/// fade; the channel stops and restarts at order `offset` (looping unless `noLoop`).
pub fn play_music(m: &mut MusicInterface, id: i32, offset: i32, no_loop: bool) {
    let Some(s) = m.song(id) else { return };
    s.volume = s.volume_cap;
    s.volume_add = 0.0;
    s.stop_on_fade = no_loop;
    s.playing = false;
    restart(s, offset, !no_loop);
}

/// `vfunction4` `StopMusic(int songId)`: volume 0, channel stopped.
pub fn stop_music(m: &mut MusicInterface, id: i32) {
    let Some(s) = m.song(id) else { return };
    s.volume = 0.0;
    s.playing = false;
}

/// `vfunction7` `StopAllMusic()`.
pub fn stop_all_music(m: &mut MusicInterface) {
    if m.silent {
        return;
    }
    for s in m.songs.values_mut() {
        s.volume = 0.0;
        s.playing = false;
    }
}

/// `vfunction12` `FadeIn(int songId, int offset, double speed, bool noLoop)`: the volume
/// climbs by `speed` per update from where it is; the channel stops and restarts at order
/// `offset`, or (offset -1) resumes where it was.
pub fn fade_in(m: &mut MusicInterface, id: i32, offset: i32, speed: f64, no_loop: bool) {
    let Some(s) = m.song(id) else { return };
    s.volume_add = speed;
    s.stop_on_fade = no_loop;
    s.playing = false;
    if offset == -1 {
        s.playing = true;
        return;
    }
    restart(s, offset, !no_loop);
}

/// `vfunction13` `FadeOut(int songId, bool stopSong, double speed)`: a sounding song's
/// volume falls by `speed` per update; `stopSong` stops it at 0.
pub fn fade_out(m: &mut MusicInterface, id: i32, stop_song: bool, speed: f64) {
    let Some(s) = m.song(id) else { return };
    if s.volume != 0.0 {
        s.volume_add = -speed;
    }
    s.stop_on_fade = stop_song;
}

/// `vfunction20` `Update()` (every logic update): fades step, stopping at the cap or at 0
/// (where a song set to stop on fade stops).
pub fn update(m: &mut MusicInterface) {
    if m.silent {
        return;
    }
    for s in m.songs.values_mut() {
        if s.volume_add == 0.0 {
            continue;
        }
        let v = s.volume_add + s.volume;
        s.volume = v;
        if s.volume_cap < v {
            s.volume = s.volume_cap;
            s.volume_add = 0.0;
        } else if v < 0.0 {
            s.volume = 0.0;
            s.volume_add = 0.0;
            if s.stop_on_fade {
                s.playing = false;
            }
        }
    }
}

/// The constructor's +0x10: the music volume BASS gets at `SetVolume(1.0)`, in percent.
pub const MAX_MUSIC_VOLUME: i32 = 0x28;

/// `vfunction18` `SetVolume(double)`: BASS's global music volume becomes
/// `ftol(MAX_MUSIC_VOLUME * v)` percent (the game's default `mMusicVolume` 1.5 gives 60%).
pub fn set_volume(m: &mut MusicInterface, v: f64) {
    m.volume = crate::sexy::crt::ftol(MAX_MUSIC_VOLUME as f64 * v) as f64 / 100.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fades() {
        let mut m = MusicInterface::default();
        load_music(&mut m, 4, "music\\Insaniq2.mo3");
        play_music(&mut m, 4, 0x2d, false);
        assert_eq!(m.songs[&4].volume, 1.0);
        assert!(m.songs[&4].playing && m.songs[&4].looping);
        fade_out(&mut m, 4, true, 0.5);
        update(&mut m);
        update(&mut m);
        update(&mut m);
        assert_eq!(m.songs[&4].volume, 0.0);
        assert!(!m.songs[&4].playing);
        fade_in(&mut m, 4, 0, 0.25, false);
        for _ in 0..8 {
            update(&mut m);
        }
        assert_eq!(m.songs[&4].volume, 1.0);
        assert_eq!(m.songs[&4].volume_add, 0.0);
    }
}
