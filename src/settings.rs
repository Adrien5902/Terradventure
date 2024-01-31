use bevy::prelude::*;
use bevy_persistent::prelude::*;
use serde::{Deserialize, Serialize};

pub struct Settings {
    // keybinds: Keybinds,
}

fn load_settings(mut commands: Commands) {}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_settings);
    }
}

struct Key(Option<KeyCode>);

// #[derive(Resource, States, Debug, Hash, PartialEq, Eq, Clone, Serialize, Deserialize)]
// pub struct Keybinds {
//     move_left: KeyCode,
//     move_right: KeyCode,
//     interact: KeyCode,
//     jump: KeyCode,
//     inventory: KeyCode,
// }

// impl Default for Keybinds {
//     fn default() -> Self {
//         Self {
//             move_left: KeyCode::Q,
//             move_right: KeyCode::D,
//             interact: KeyCode::E,
//             jump: KeyCode::Space,
//             inventory: KeyCode::A,
//         }
//     }
// }
