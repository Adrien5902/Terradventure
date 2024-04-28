use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    animation::AnimationController, gui::hud::UseItemEvent, items::item::ItemTrait, player::Player,
};

#[derive(Clone, Deserialize, Serialize, Reflect, PartialEq, Eq, Default)]
pub struct ManaPotion;

impl ItemTrait for ManaPotion {
    // fn stack_size(&self) -> crate::items::item::StackSize {
    //     16
    // }

    fn use_item(&self) -> bool {
        true
    }
}

pub fn use_mana_potion(
    mut query: Query<(&mut Player, &mut AnimationController)>,
    mut events: EventReader<UseItemEvent>,
) {
    if let Ok((mut player, mut animation_controller)) = query.get_single_mut() {
        for ev in events.read() {
            if ev.item == ManaPotion.into() {
                animation_controller.play("Elixir");
                player.mana += 70.;
            }
        }
    }
}
