use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::collections::HashMap;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum MusicTrack {
    Title,
    Overworld,
    Dungeon,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum SoundId {
    Sword,
    Pickup,
    Currency,
    Door,
    EnemyDie,
    PlayerHit,
    Message,
}

pub struct Audio {
    _stream: Option<OutputStream>,
    handle: Option<OutputStreamHandle>,
    music_sink: Option<Sink>,
    current_music: Option<MusicTrack>,
    music: HashMap<MusicTrack, Vec<u8>>,
    sounds: HashMap<SoundId, Vec<u8>>,
}

impl Audio {
    pub async fn load() -> Self {
        let (stream, handle) = OutputStream::try_default()
            .map(|(stream, handle)| (Some(stream), Some(handle)))
            .unwrap_or((None, None));

        let base = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");
        let mut music = HashMap::new();
        let mut sounds = HashMap::new();

        if let Some(bytes) = load_first_existing(&base, "music", "intro", &["wav", "ogg", "mp3"]) {
            music.insert(MusicTrack::Title, bytes);
        }
        if let Some(bytes) =
            load_first_existing(&base, "music", "overworld", &["wav", "ogg", "mp3"])
        {
            music.insert(MusicTrack::Overworld, bytes);
        }
        if let Some(bytes) = load_first_existing(&base, "music", "dungeon", &["wav", "ogg", "mp3"])
        {
            music.insert(MusicTrack::Dungeon, bytes);
        }

        load_sound_slot(&base, "sfx", "sword", SoundId::Sword, &mut sounds);
        load_sound_slot(&base, "sfx", "pickup", SoundId::Pickup, &mut sounds);
        load_sound_slot(&base, "sfx", "currency", SoundId::Currency, &mut sounds);
        load_sound_slot(&base, "sfx", "door", SoundId::Door, &mut sounds);
        load_sound_slot(&base, "sfx", "enemy_die", SoundId::EnemyDie, &mut sounds);
        load_sound_slot(&base, "sfx", "player_hit", SoundId::PlayerHit, &mut sounds);
        load_sound_slot(&base, "sfx", "message", SoundId::Message, &mut sounds);

        Self {
            _stream: stream,
            handle,
            music_sink: None,
            current_music: None,
            music,
            sounds,
        }
    }

    pub fn play_music(&mut self, track: MusicTrack) {
        if self.current_music == Some(track) {
            return;
        }
        self.stop_music();
        let Some(handle) = &self.handle else {
            return;
        };
        let Some(bytes) = self.music.get(&track) else {
            return;
        };
        let Ok(decoder) = Decoder::new(Cursor::new(bytes.clone())) else {
            return;
        };
        let Ok(sink) = Sink::try_new(handle) else {
            return;
        };
        sink.append(decoder.repeat_infinite());
        sink.set_volume(crate::config::get().audio.music_volume);
        self.music_sink = Some(sink);
        self.current_music = Some(track);
    }

    pub fn stop_music(&mut self) {
        if let Some(sink) = self.music_sink.take() {
            sink.stop();
        }
        self.current_music = None;
    }

    pub fn sword(&self) {
        self.play_effect(SoundId::Sword, crate::config::get().audio.sfx_volume);
    }

    pub fn pickup(&self) {
        self.play_effect(SoundId::Pickup, crate::config::get().audio.sfx_volume);
    }

    pub fn currency(&self) {
        self.play_effect(SoundId::Currency, crate::config::get().audio.sfx_volume);
    }

    pub fn door(&self) {
        self.play_effect(SoundId::Door, crate::config::get().audio.sfx_volume);
    }

    pub fn enemy_die(&self) {
        self.play_effect(SoundId::EnemyDie, crate::config::get().audio.sfx_volume);
    }

    pub fn player_hit(&self) {
        self.play_effect(SoundId::PlayerHit, crate::config::get().audio.sfx_volume);
    }

    pub fn message(&self) {
        self.play_effect(SoundId::Message, crate::config::get().audio.sfx_volume);
    }

    fn play_effect(&self, sound_id: SoundId, volume: f32) {
        let Some(handle) = &self.handle else {
            return;
        };
        let Some(bytes) = self.sounds.get(&sound_id) else {
            return;
        };
        let Ok(decoder) = Decoder::new(Cursor::new(bytes.clone())) else {
            return;
        };
        let Ok(sink) = Sink::try_new(handle) else {
            return;
        };
        sink.set_volume(volume);
        sink.append(decoder);
        sink.detach();
    }
}

fn load_sound_slot(
    base: &Path,
    folder: &str,
    stem: &str,
    sound_id: SoundId,
    out: &mut HashMap<SoundId, Vec<u8>>,
) {
    if let Some(bytes) = load_first_existing(base, folder, stem, &["wav", "ogg", "mp3"]) {
        out.insert(sound_id, bytes);
    }
}

fn load_first_existing(base: &Path, folder: &str, stem: &str, exts: &[&str]) -> Option<Vec<u8>> {
    for ext in exts {
        let path = base.join(folder).join(format!("{stem}.{ext}"));
        if path.exists() {
            if let Ok(bytes) = fs::read(&path) {
                return Some(bytes);
            }
        }
    }
    None
}
