//! Host music: plays `G::music` (the game's `BassMusicInterface` state) through Bevy audio,
//! rendering the MO3 modules with libopenmpt as the original did with BASS.
//!
//! Each frame `sync_music` copies the game's music state into a mixer shared with the
//! audio thread: a song whose `start_serial` changed restarts at its order (looping unless
//! the game asked for `noLoop`), and every song sounds at its whole-percent volume times
//! the global music volume while its channel plays. One endless stereo stream (44.1 kHz)
//! renders and mixes the playing modules on the audio thread. A module that is not looping
//! falls silent at its end, as BASS does. Module files are parsed when the game first
//! touches the song (in memory, from the files the host already loaded).

use super::openmpt::Module;
use crate::sexy::prelude::*;
use bevy::audio::{AddAudioSource, AudioPlayer, Decodable, PlaybackSettings};
use bevy::prelude::*;
use std::collections::HashMap;
use std::num::NonZero;
use std::sync::{Arc, Mutex};

const RATE: u32 = 44100;
const CHUNK: usize = 1024;

struct Track {
    module: Option<Module>,
    serial: u32,
    playing: bool,
    volume: f32,
}

#[derive(Default)]
struct Mixer {
    tracks: HashMap<i32, Track>,
}

/// The mixer shared by the sync system and the audio thread.
#[derive(Resource, Default)]
pub struct MusicPlayer {
    mixer: Arc<Mutex<Mixer>>,
    started: bool,
}

/// The endless music stream (one per app).
#[derive(Asset, TypePath, Clone)]
pub struct MusicStream {
    mixer: Arc<Mutex<Mixer>>,
}

pub struct MusicDecoder {
    mixer: Arc<Mutex<Mixer>>,
    buf: Vec<f32>,
    tmp: Vec<f32>,
    pos: usize,
}

impl MusicDecoder {
    fn refill(&mut self) {
        self.buf.clear();
        self.buf.resize(CHUNK * 2, 0.0);
        if let Ok(mut m) = self.mixer.lock() {
            for t in m.tracks.values_mut() {
                if !t.playing || t.volume <= 0.0 {
                    continue;
                }
                let Some(module) = t.module.as_mut() else { continue };
                self.tmp.clear();
                self.tmp.resize(CHUNK * 2, 0.0);
                let n = module.read_stereo(RATE as i32, &mut self.tmp);
                if n < CHUNK {
                    t.playing = false;
                }
                for (o, s) in self.buf.iter_mut().zip(&self.tmp[..n * 2]) {
                    *o += s * t.volume;
                }
            }
        }
        self.pos = 0;
    }
}

impl Iterator for MusicDecoder {
    type Item = bevy::audio::Sample;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.buf.len() {
            self.refill();
        }
        let s = self.buf[self.pos];
        self.pos += 1;
        Some(s)
    }
}

impl bevy::audio::Source for MusicDecoder {
    fn current_span_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> bevy::audio::ChannelCount {
        NonZero::new(2).unwrap()
    }
    fn sample_rate(&self) -> bevy::audio::SampleRate {
        NonZero::new(RATE).unwrap()
    }
    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

impl Decodable for MusicStream {
    type Decoder = MusicDecoder;
    fn decoder(&self) -> MusicDecoder {
        MusicDecoder { mixer: self.mixer.clone(), buf: Vec::new(), tmp: Vec::new(), pos: 0 }
    }
}

pub fn plugin(app: &mut App) {
    app.add_audio_source::<MusicStream>().init_resource::<MusicPlayer>();
}

/// Copies the game's music state into the mixer (and starts the stream once).
pub fn sync_music(mut commands: Commands, g: Res<G>, mut player: ResMut<MusicPlayer>, mut streams: ResMut<Assets<MusicStream>>) {
    if std::env::var_os("WINFISH_NO_AUDIO").is_some() {
        return;
    }
    if !player.started {
        let h = streams.add(MusicStream { mixer: player.mixer.clone() });
        commands.spawn((AudioPlayer::<MusicStream>(h), PlaybackSettings::ONCE));
        player.started = true;
    }
    let master = g.music.volume;
    let Ok(mut m) = player.mixer.lock() else { return };
    for (&id, s) in &g.music.songs {
        let t = m.tracks.entry(id).or_insert_with(|| Track {
            module: g.vfs.read(&s.file).and_then(Module::load),
            serial: 0,
            playing: false,
            volume: 0.0,
        });
        if t.serial != s.start_serial {
            t.serial = s.start_serial;
            if let Some(module) = t.module.as_mut() {
                module.set_repeat_count(if s.looping { -1 } else { 0 });
                module.set_position(s.order, 0);
            }
        }
        // (A module that ended by itself reads no more frames, so it stays silent.)
        t.playing = s.playing;
        t.volume = (crate::sexy::music::channel_volume(s) * master).max(0.0) as f32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The title music (music 2 order 0x2d, mapped to Insaniq2.mo3 order 0x25) renders.
    #[test]
    fn title_music_renders() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../Insaniquarium Deluxe/music/Insaniq2.mo3");
        let Ok(bytes) = std::fs::read(&path) else { return };
        let mut m = Module::load(&bytes).expect("libopenmpt loads the MO3");
        let (mut music, mut order) = (2, 0x2d);
        crate::game::win_fish_app::FUN_0054b090(&mut music, &mut order);
        assert_eq!((music, order), (4, 0x25));
        m.set_repeat_count(-1);
        m.set_position(order, 0);
        let mut buf = vec![0f32; RATE as usize * 2];
        let n = m.read_stereo(RATE as i32, &mut buf);
        assert_eq!(n, RATE as usize);
        let energy: f32 = buf.iter().map(|s| s * s).sum();
        assert!(energy > 1.0, "silent: {energy}");
    }
}
