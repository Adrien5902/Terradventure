use crate::gui::settings::Settings;

use super::{can_attack, is_of_class, PlayerClass};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Enchantress;

impl PlayerClass for Enchantress {
    fn name(&self) -> &'static str {
        "enchantress"
    }

    fn normal_attack_chain_count(&self) -> u8 {
        4
    }
}

impl Plugin for Enchantress {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (special_attack_1, special_attack_2, special_attack_3)
                .run_if(is_of_class::<Self>.and_then(can_attack)),
        );
    }
}

fn special_attack_1(settings: Res<Settings>) {}
fn special_attack_2() {}
fn special_attack_3() {}
