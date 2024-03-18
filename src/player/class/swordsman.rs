use super::{can_attack, is_of_class, swords_user_special_attacks, PlayerClass, SwordUserClass};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Swordsman;

impl PlayerClass for Swordsman {
    fn name(&self) -> &'static str {
        "swordsman"
    }

    fn normal_attack_chain_count(&self) -> u8 {
        3
    }
}

impl SwordUserClass for Swordsman {}

impl Plugin for Swordsman {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (swords_user_special_attacks).run_if(is_of_class::<Self>.and_then(can_attack)),
        );
    }
}
