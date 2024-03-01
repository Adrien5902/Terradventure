use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Keybinds {
    pub move_left: Keybind,
    pub move_right: Keybind,
    pub interact: Keybind,
    pub jump: Keybind,
    pub inventory: Keybind,
    pub attack: Keybind,
    pub special_attack_1: Keybind,
    pub special_attack_2: Keybind,
    pub special_attack_3: Keybind,
}

impl Default for Keybinds {
    fn default() -> Self {
        Self {
            move_left: Keybind(KeyCode::Q),
            move_right: Keybind(KeyCode::D),
            interact: Keybind(KeyCode::E),
            jump: Keybind(KeyCode::Space),
            inventory: Keybind(KeyCode::A),
            attack: Keybind(KeyCode::J),
            special_attack_1: Keybind(KeyCode::K),
            special_attack_2: Keybind(KeyCode::L),
            special_attack_3: Keybind(KeyCode::M),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Keybind(KeyCode);

impl Keybind {
    pub fn get(&self) -> KeyCode {
        self.0
    }
}
