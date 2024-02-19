pub mod fov;
pub mod keybinds;
pub mod range;
pub mod ui;

use bevy::prelude::*;
use bevy_persistent::Persistent;
use serde::{Deserialize, Serialize};

use crate::CONFIG_DIR;

use self::{fov::FovRange, keybinds::Keybinds, range::RangeSetting, ui::SettingsUiPlugin};

#[derive(Serialize, Deserialize, Resource)]
pub struct Settings {
    pub fov: FovRange,
    pub keybinds: Keybinds,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            fov: FovRange::from_value(20.),
            keybinds: Keybinds::default(),
        }
    }
}

fn load_settings(mut commands: Commands) {
    commands.insert_resource(
        Persistent::<Settings>::builder()
            .name("settings")
            .format(bevy_persistent::StorageFormat::Json)
            .path(CONFIG_DIR.join("settings.json"))
            .default(Settings::default())
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
