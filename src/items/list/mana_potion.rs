use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{gui::hud::UseItemEvent, items::item::Item, player::Player};

#[derive(Clone, Deserialize, Serialize, Reflect, PartialEq, Eq)]
pub struct ManaPotion;

impl Item for ManaPotion {
    fn stack_size(&self) -> crate::items::item::StackSize {
        16
    }

    fn use_item(&self) -> bool {
        true
    }
}

pub fn use_mana_potion(mut query: Query<&mut Player>, mut events: EventReader<UseItemEvent>) {
    if let Ok(mut player) = query.get_single_mut() {
        for ev in events.read() {
            if ev.item == ManaPotion.into() {
                player.mana += 70.;
            }
        }
    }
}
