use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::gui::UiChild;

#[derive(Serialize, Deserialize)]
pub struct Keybinds {
    pub move_left: Keybind,
    pub move_right: Keybind,
    pub interact: Keybind,
    pub jump: Keybind,
    pub inventory: Keybind,
}

impl Default for Keybinds {
    fn default() -> Self {
        Self {
            move_left: Keybind(KeyCode::Q),
            move_right: Keybind(KeyCode::D),
            interact: Keybind(KeyCode::E),
            jump: Keybind(KeyCode::Space),
            inventory: Keybind(KeyCode::A),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Keybind(KeyCode);
impl UiChild for Keybind {
    fn bundle(&self, _asset_server: &Res<AssetServer>) -> Vec<impl Bundle> {
        vec![TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "Test".to_string(),
                    ..Default::default()
                }],
                ..Default::default()
            },
            ..Default::default()
        }]
    }
}

impl Keybind {
    pub fn get(&self) -> KeyCode {
        self.0
    }
}

impl UiChild for Keybinds {
    fn bundle(&self, asset_server: &Res<AssetServer>) -> Vec<impl Bundle> {
        vec![] as Vec<TextBundle>
    }
}
