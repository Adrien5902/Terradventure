pub mod keybinds;
pub mod ui;

use std::ops::Range;

use bevy::prelude::*;
use bevy_persistent::Persistent;
use serde::{Deserialize, Serialize};

use crate::CONFIG_DIR;

use self::{keybinds::Keybinds, ui::SettingsUiPlugin};

use super::UiChild;

#[derive(Serialize, Deserialize, Resource)]
pub struct Settings {
    pub fov: Range<f32>,
    pub keybinds: Keybinds,
}

fn load_settings(mut commands: Commands) {
    commands.insert_resource(
        Persistent::<Settings>::builder()
            .name("settings")
            .format(bevy_persistent::StorageFormat::Json)
            .path(CONFIG_DIR.join("settings.json"))
            .build()
            .expect("Settings init failed"),
    );
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_settings)
            .add_plugins(SettingsUiPlugin);
    }
}

impl<T> UiChild for Range<T> {
    fn bundle(&self, asset_server: &Res<AssetServer>) -> Vec<impl Bundle> {
        Entry
    }
}
