use super::{can_attack, is_of_class, swords_user_special_attacks, PlayerClass, SwordUserClass};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Musketeer;

impl PlayerClass for Musketeer {
    fn name(&self) -> &'static str {
        "musketeer"
    }

    fn normal_attack_chain_count(&self) -> u8 {
        4
    }
}

impl SwordUserClass for Musketeer {}

impl Plugin for Musketeer {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (swords_user_special_attacks).run_if(is_of_class::<Self>.and_then(can_attack)),
        );
    }
}
