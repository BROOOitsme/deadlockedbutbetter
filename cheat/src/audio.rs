use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::io::Cursor;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Source};

use crate::config::hud::HitmarkerSound;

/// Minimum time between two plays of the same sound. Prevents a shotgun's
/// pellet hits (which all land in the same tick) from stacking on top of
/// each other and blowing out the mix.
const MIN_REPLAY_INTERVAL: Duration = Duration::from_millis(50);

pub struct Audio {
    sink: MixerDeviceSink,
    last_played: Mutex<HashMap<u64, Instant>>,
}

impl Audio {
    pub fn new() -> Self {
        let mut sink = DeviceSinkBuilder::open_default_sink().unwrap_or_else(|error| {
            utils::error!("failed to initialize audio output: {error}");
            std::process::exit(1);
        });
        sink.log_on_drop(false);
        Self {
            sink,
            last_played: Mutex::new(HashMap::new()),
        }
    }

    /// Plays a sound, throttled so identical sounds fired in the same instant
    /// (e.g. shotgun pellets) only play once.
    pub fn play(&self, bytes: &[u8], volume: f32) {
        self.play_throttled(bytes, volume, MIN_REPLAY_INTERVAL);
    }

    /// Explicit throttle window. Pass `Duration::ZERO` to bypass coalescing.
    pub fn play_throttled(&self, bytes: &[u8], volume: f32, min_interval: Duration) {
        if !self.should_play(bytes, min_interval) {
            return;
        }
        let Ok(source) = Decoder::try_from(Cursor::new(bytes.to_vec())) else {
            utils::warn!("failed to decode hitmarker sound");
            return;
        };
        self.sink
        .mixer()
        .add(source.amplify(volume.clamp(0.0, 10.0)));
    }

    fn should_play(&self, bytes: &[u8], min_interval: Duration) -> bool {
        if min_interval.is_zero() {
            return true;
        }
        let key = sound_key(bytes);
        let now = Instant::now();
        let mut map = self.last_played.lock().unwrap();

        if let Some(prev) = map.get(&key) {
            if now.duration_since(*prev) < min_interval {
                return false; // coalesce: same sound fired too recently
            }
        }
        map.insert(key, now);

        // Cheap housekeeping: forget entries we'll never consult again.
        map.retain(|_, t| now.duration_since(*t) < Duration::from_secs(5));
        true
    }

    pub fn validate(bytes: Vec<u8>) -> Result<(), String> {
        Decoder::try_from(Cursor::new(bytes))
        .map(|_| ())
        .map_err(|error| error.to_string())
    }
}

fn sound_key(bytes: &[u8]) -> u64 {
    // Full-bytes hashing would be wasteful for a large custom sound on every
    // pellet. Length + first 256 bytes is plenty to fingerprint distinct
    // hitmarker WAVs/MP3s (they differ in header + early samples).
    let mut hasher = DefaultHasher::new();
    bytes.len().hash(&mut hasher);
    &bytes[..bytes.len().min(256)].hash(&mut hasher);
    hasher.finish()
}

pub fn builtin_sound(sound: HitmarkerSound) -> &'static [u8] {
    match sound {
        HitmarkerSound::Beep => include_bytes!("../assets/audio/beep.wav"),
        HitmarkerSound::Bell => include_bytes!("../assets/audio/bell.wav"),
        HitmarkerSound::BulletCasing => include_bytes!("../assets/audio/bullet_casing.wav"),
        HitmarkerSound::Click => include_bytes!("../assets/audio/click.wav"),
        HitmarkerSound::Clink => include_bytes!("../assets/audio/clink.mp3"),
        HitmarkerSound::KnifeImpact => include_bytes!("../assets/audio/knife_impact.wav"),
        HitmarkerSound::RubberTire => include_bytes!("../assets/audio/rubber_tire.wav"),
        HitmarkerSound::Sine => include_bytes!("../assets/audio/sine.wav"),
        HitmarkerSound::WaterDrip => include_bytes!("../assets/audio/water_drip.wav"),
        HitmarkerSound::Wood => include_bytes!("../assets/audio/wood.mp3"),
        HitmarkerSound::Custom => include_bytes!("../assets/audio/water_drip.wav"),
    }
}
