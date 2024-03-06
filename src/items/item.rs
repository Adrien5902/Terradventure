use std::path::{Path, PathBuf};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use crate::{
    interactable::Interactable,
    player::{inventory::SlotType, Player},
    state::AppState,
};

use super::stack::ItemStack;

pub type StackSize = u8;

#[enum_dispatch]
pub trait Item: Sync + Send + Reflect {
    fn name(&self) -> ItemName;
    fn texture(&self) -> PathBuf {
        Path::new("textures/item").join(format!("{}.png", self.name().0))
    }

    fn stack_size(&self) -> StackSize {
        StackSize::MAX
    }

    fn can_put_in(&self) -> SlotType {
        SlotType::default()
    }

    // fn get_use(&self) -> Option<fn() -> ()> {
    //     None
    // }
}

pub struct ItemTexture(&'static str);

impl From<&'static str> for ItemTexture {
    fn from(value: &'static str) -> Self {
        Self(value)
    }
}

impl From<ItemName> for ItemTexture {
    fn from(value: ItemName) -> Self {
        Self::from(value.get())
    }
}

#[derive(Deserialize, Serialize)]
pub struct ItemName(&'static str);

impl From<&'static str> for ItemName {
    fn from(value: &'static str) -> Self {
        Self(value)
    }
}

impl ItemName {
    pub fn get(&self) -> &'static str {
        self.0
    }
}

#[derive(Bundle)]
pub struct ItemBundle {
    pub item_stack: ItemStack,
    pub sprite: SpriteBundle,
    pub interactable: Interactable,
    pub rigid_body: RigidBody,
    pub collider: Collider,
}

pub struct ItemPlugin;
impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, interact.run_if(in_state(AppState::InGame)));
    }
}

fn interact(
    mut commands: Commands,
    mut player_query: Query<&mut Player>,
    mut item_query: Query<(Entity, &mut ItemStack, &Interactable)>,
) {
    for (entity, item_stack, interactable) in item_query.iter_mut() {
        if interactable.just_pressed() {
            if let Ok(mut player) = player_query.get_single_mut() {
                let optional_item_stack = &mut Some(item_stack.to_owned());
                player.inventory.push_item_stack(optional_item_stack);

                if optional_item_stack.is_none() {
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }
}
