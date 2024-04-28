use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{interactable::Interactable, player::inventory::SlotType, world::BLOCK_SIZE};

use super::{
    item::{ItemBundle, ItemTrait},
    list::Item,
};

#[derive(Clone, Deserialize, Serialize, Reflect, Component)]
pub struct ItemStack {
    pub item: Item,
    pub count: u8,
}

impl ItemStack {
    pub fn actual_count(&self) -> u16 {
        self.count as u16 + 1
    }

    pub fn bundle(self, asset_server: &AssetServer, pos: Vec2) -> ItemBundle {
        ItemBundle {
            sprite: SpriteBundle {
                texture: asset_server.load(self.item.texture()),
                transform: Transform::from_translation(pos.extend(11.0)),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(BLOCK_SIZE / 1.7)),
                    ..Default::default()
                },
                ..Default::default()
            },
            interactable: Interactable::new("player.actions.take"),
            rigid_body: RigidBody::Dynamic,
            mass: ColliderMassProperties::Mass(100.),
            collider: Collider::cuboid(BLOCK_SIZE / 4., BLOCK_SIZE / 4.),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            item_stack: self,
        }
    }

    pub fn new_one<T>(item: T) -> Self
    where
        T: Into<Item>,
    {
        Self {
            count: 0,
            item: item.into(),
        }
    }

    /// # Returns
    /// [`false`] if all the items were consumed
    pub fn try_remove(&mut self, actual_count: u8) -> bool {
        let can_remove = self.count >= actual_count;
        if can_remove {
            self.count -= actual_count;
        }

        can_remove
    }

    pub fn can_put_in_slot_type(&self, slot_type: SlotType) -> bool {
        self.item.can_put_in().contains(slot_type)
    }
}
