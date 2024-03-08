pub mod audio;
pub mod fov;
pub mod keybinds;
pub mod lang;
pub mod range;
pub mod ui;

use std::{
    fs,
    path::{Path, PathBuf},
};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    lang::{Lang, LangIdentifier, Langs},
    CONFIG_DIR,
};

use self::{
    audio::AudioChannelsVolumeRanges, fov::FovRange, keybinds::Keybinds, ui::SettingsUiPlugin,
};

#[derive(Serialize, Deserialize, Resource)]
pub struct Settings {
    pub fov: FovRange,
    pub keybinds: Keybinds,
    pub lang: LangIdentifier,
    pub audio: AudioChannelsVolumeRanges,
}

impl Settings {
    const FILE_NAME: &'static str = "settings.json";

    fn path() -> PathBuf {
        CONFIG_DIR.join(Self::FILE_NAME)
    }

    pub fn read() -> Self {
        let res = (|| {
            let data = fs::read(Self::path()).map_err(|e| e.to_string())?;
            serde_json::from_slice(&data).map_err(|e| e.to_string())
        })();

        match res {
            Ok(data) => data,
            Err(_) => {
                let data = Self::default();
                data.save().unwrap();
                data
            }
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let path = Self::path();
        let parent = path.parent().ok_or::<String>("No parent dir".into())?;
        if !Path::exists(parent) {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let data = serde_json::to_vec(self).map_err(|e| e.to_string())?;
        fs::write(path, data).map_err(|e| e.to_string())
    }

    pub fn update<F>(&mut self, callback: F)
    where
        F: Fn(&mut Self),
    {
        let old_lang = self.lang.clone();
        callback(self);
        if self.lang != old_lang {
            // Loading into ressources is required for lang
            panic!("Call update_lang instead")
        }
        self.save().unwrap();
    }

    pub fn update_lang(&mut self, lang_indent: LangIdentifier, lang_res: &mut ResMut<Lang>) {
        let lang: Lang = lang_indent.clone().into();
        **lang_res = lang.load();
        self.lang = lang_indent;
        self.save().unwrap()
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            lang: Langs::default().into(),
            fov: FovRange { value: 20.0 },
            keybinds: Keybinds::default(),
            audio: AudioChannelsVolumeRanges::default(),
        }
    }
}

fn load_settings(mut commands: Commands) {
    let settings = Settings::read();
    let lang: Lang = settings.lang.clone().into();
    commands.insert_resource(lang.load());
    commands.insert_resource(settings);
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_settings)
            .add_plugins(SettingsUiPlugin);
    }
}
