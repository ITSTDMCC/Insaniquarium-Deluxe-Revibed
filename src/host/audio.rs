//! Host audio: plays the sounds the game queued (`G::sound_requests`) through Bevy audio,
//! replacing the original's DirectSound manager (`SexyAppBase::mSoundManager`).
//!
//! Sounds are the `<Sound>` resources (`sounds\*.ogg` and Sun `.au` files, 8-bit mu-law or
//! 16-bit linear PCM) read from the game files the host loaded. Each request plays once at
//! `request volume x resource volume x sound-manager volume` (`SetSfxVolume`, 0 while
//! muted), sped up or down by its pitch in semitones. The original's stereo pan has no
//! counterpart in Bevy's non-spatial playback and is dropped. Music is `crate::host::music`.

use crate::sexy::prelude::*;
use bevy::audio::{AddAudioSource, AudioPlayer, AudioSource, Decodable, PlaybackSettings, Volume};
use bevy::prelude::*;
use std::collections::HashMap;
use std::num::NonZero;
use std::sync::Arc;
use std::time::Duration;

/// A decoded `.au` sound (mono or interleaved samples in -1..1).
#[derive(Asset, TypePath, Clone)]
pub struct AuSound {
    pub samples: Arc<[f32]>,
    pub rate: u32,
    pub channels: u16,
}

pub struct AuDecoder {
    samples: Arc<[f32]>,
    pos: usize,
    rate: u32,
    channels: u16,
}

impl Iterator for AuDecoder {
    type Item = bevy::audio::Sample;
    fn next(&mut self) -> Option<Self::Item> {
        let s = self.samples.get(self.pos).copied()?;
        self.pos += 1;
        Some(s)
    }
}

impl bevy::audio::Source for AuDecoder {
    fn current_span_len(&self) -> Option<usize> {
        Some(self.samples.len() - self.pos)
    }
    fn channels(&self) -> bevy::audio::ChannelCount {
        NonZero::new(self.channels.max(1)).unwrap()
    }
    fn sample_rate(&self) -> bevy::audio::SampleRate {
        NonZero::new(self.rate.max(1)).unwrap()
    }
    fn total_duration(&self) -> Option<Duration> {
        let frames = self.samples.len() / self.channels.max(1) as usize;
        Some(Duration::from_secs_f64(frames as f64 / self.rate.max(1) as f64))
    }
}

impl Decodable for AuSound {
    type Decoder = AuDecoder;
    fn decoder(&self) -> AuDecoder {
        AuDecoder { samples: self.samples.clone(), pos: 0, rate: self.rate, channels: self.channels }
    }
}

/// G.711 mu-law to linear (-1..1).
fn mulaw(b: u8) -> f32 {
    let u = !b;
    let sign = u & 0x80;
    let exp = (u >> 4) & 7;
    let mant = (u & 0x0f) as i32;
    let mag = (((mant << 3) + 0x84) << exp) - 0x84;
    let v = if sign != 0 { -mag } else { mag };
    v as f32 / 32768.0
}

/// Parses a Sun/NeXT `.au` file (big-endian header: magic `.snd`, data offset, size,
/// encoding 1 = mu-law, 2 = 8-bit, 3 = 16-bit linear; rate; channels).
pub fn decode_au(bytes: &[u8]) -> Option<AuSound> {
    let be = |o: usize| -> Option<u32> { Some(u32::from_be_bytes(bytes.get(o..o + 4)?.try_into().ok()?)) };
    if bytes.get(0..4)? != b".snd" {
        return None;
    }
    let off = be(4)? as usize;
    let size = be(8)? as usize;
    let enc = be(12)?;
    let rate = be(16)?;
    let channels = be(20)? as u16;
    let end = if size == 0xffff_ffff { bytes.len() } else { (off + size).min(bytes.len()) };
    let data = bytes.get(off..end)?;
    let samples: Vec<f32> = match enc {
        1 => data.iter().map(|&b| mulaw(b)).collect(),
        2 => data.iter().map(|&b| (b as i8) as f32 / 128.0).collect(),
        3 => data.chunks_exact(2).map(|c| i16::from_be_bytes([c[0], c[1]]) as f32 / 32768.0).collect(),
        _ => return None,
    };
    Some(AuSound { samples: samples.into(), rate, channels })
}

#[derive(Clone)]
enum Loaded {
    Ogg(Handle<AudioSource>),
    Au(Handle<AuSound>),
    Missing,
}

/// Sound id -> loaded asset (decoded on first use).
#[derive(Resource, Default)]
pub struct SoundCache(HashMap<i32, Loaded>);

pub fn plugin(app: &mut App) {
    app.add_audio_source::<AuSound>().init_resource::<SoundCache>();
}

fn load(g: &G, id: i32, ogg: &mut Assets<AudioSource>, au: &mut Assets<AuSound>) -> (Loaded, f64) {
    let Some((path, res_volume)) = g.resource_manager.sound_files.get(&id).cloned() else { return (Loaded::Missing, 1.0) };
    let vol = if res_volume < 0.0 { 1.0 } else { res_volume };
    let base = path.replace('/', "\\");
    for (ext, is_au) in [(".ogg", false), (".au", true), ("", false)] {
        let p = format!("{base}{ext}");
        if let Some(bytes) = g.vfs.read(&p) {
            if is_au || p.to_ascii_lowercase().ends_with(".au") {
                if let Some(s) = decode_au(bytes) {
                    return (Loaded::Au(au.add(s)), vol);
                }
            } else {
                return (Loaded::Ogg(ogg.add(AudioSource { bytes: bytes.to_vec().into() })), vol);
            }
        }
    }
    (Loaded::Missing, vol)
}

/// Plays and clears the sounds queued this frame.
pub fn play_sounds(
    mut commands: Commands,
    mut g: ResMut<G>,
    mut cache: ResMut<SoundCache>,
    mut ogg: ResMut<Assets<AudioSource>>,
    mut au: ResMut<Assets<AuSound>>,
) {
    if g.sound_requests.is_empty() {
        return;
    }
    let reqs = std::mem::take(&mut g.sound_requests);
    if std::env::var_os("WINFISH_NO_AUDIO").is_some() {
        return;
    }
    let app = g.globals.DAT_005eb6a4;
    let master = if app != NULL { g.sab(app).sound_manager_volume } else { 1.0 };
    for r in reqs {
        if r.id < 0 {
            continue;
        }
        let (entry, res_vol) = match g.resource_manager.sound_files.get(&r.id) {
            Some((_, v)) => {
                let v = if *v < 0.0 { 1.0 } else { *v };
                let e = match cache.0.get(&r.id) {
                    Some(e) => e.clone(),
                    None => {
                        let (e, _) = load(&g, r.id, &mut ogg, &mut au);
                        cache.0.insert(r.id, e.clone());
                        e
                    }
                };
                (e, v)
            }
            None => (Loaded::Missing, 1.0),
        };
        let volume = (r.volume * res_vol * master).max(0.0) as f32;
        let speed = 2f32.powf(r.pitch as f32 / 12.0);
        let settings = PlaybackSettings::DESPAWN.with_volume(Volume::Linear(volume)).with_speed(speed);
        match entry {
            Loaded::Ogg(h) => {
                commands.spawn((AudioPlayer::new(h), settings));
            }
            Loaded::Au(h) => {
                commands.spawn((AudioPlayer::<AuSound>(h), settings));
            }
            Loaded::Missing => {}
        }
    }
}
