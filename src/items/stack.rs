use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{interactable::Interactable, world::BLOCK_SIZE};

use super::{
    item::{Item, ItemBundle},
    list::ItemObject,
};

#[derive(Clone, Deserialize, Serialize, Reflect, Component)]
pub struct ItemStack {
    pub count: u8,
    pub item: ItemObject,
}

impl ItemStack {
    pub fn actual_count(&self) -> u16 {
        self.count as u16 + 1
    }

    pub fn bundle(self, asset_server: &Res<AssetServer>, pos: Vec2) -> ItemBundle {
        ItemBundle {
            sprite: SpriteBundle {
                texture: asset_server.load(self.item.texture()),
                transform: Transform::from_translation(pos.extend(11.0)),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(BLOCK_SIZE / 2.)),
                    ..Default::default()
                },
                ..Default::default()
            },
            interactable: Interactable::new("player.actions.take"),
            rigid_body: RigidBody::Dynamic,
            collider: Collider::cuboid(BLOCK_SIZE / 4., BLOCK_SIZE / 4.),
            item_stack: self,
        }
    }
}
