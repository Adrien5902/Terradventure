use std::path::{Path, PathBuf};

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::gui::settings::{range::RangeSetting, Settings};

pub struct MusicPlugin;
impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .add_audio_channel::<BackgroundAudio>()
            .add_systems(Startup, play_bg_audio);
    }
}

fn play_bg_audio(
    asset_server: Res<AssetServer>,
    background_audio: Res<AudioChannel<BackgroundAudio>>,
    settings: Res<Settings>,
) {
    let path: PathBuf = MusicAsset("background").into();
    let handle = asset_server.load(path);
    background_audio
        .play(handle)
        .loop_from(0.0)
        .with_volume(settings.audio.background_music.get_value() as f64);
}

pub struct MusicAsset(pub &'static str);
impl From<MusicAsset> for AudioAsset {
    fn from(value: MusicAsset) -> Self {
        AudioAsset(Path::new("music").join(value.0))
    }
}

impl From<MusicAsset> for PathBuf {
    fn from(value: MusicAsset) -> Self {
        let audio: AudioAsset = value.into();
        audio.into()
    }
}

pub struct AudioAsset(pub PathBuf);

impl From<AudioAsset> for PathBuf {
    fn from(value: AudioAsset) -> Self {
        Path::new("audio").join(format!("{}.ogg", value.0.to_string_lossy()))
    }
}

#[derive(Resource)]
pub struct BackgroundAudio;
