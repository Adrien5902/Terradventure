use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    animation::AnimationController,
    effects::{Effect, EffectsController},
    gui::hud::UseItemEvent,
    items::item::Item,
    player::Player,
};

use super::ItemObject;

#[derive(Clone, Deserialize, Serialize, Reflect, PartialEq, Default)]
pub struct LevitationPotion {
    pub seconds: f32,
    pub level: u8,
}

impl Eq for LevitationPotion {}

impl Item for LevitationPotion {
    fn use_item(&self) -> bool {
        true
    }
}

pub fn use_levitation_potion(
    mut query: Query<(&mut EffectsController, &mut AnimationController), With<Player>>,
    mut events: EventReader<UseItemEvent>,
) {
    if let Ok((mut effects, mut animation_controller)) = query.get_single_mut() {
        for ev in events.read() {
            if let ItemObject::LevitationPotion(potion) = &ev.item {
                animation_controller.play("Elixir");
                effects.add_new(Effect::Levitation, potion.seconds, potion.level)
            }
        }
    }
}
