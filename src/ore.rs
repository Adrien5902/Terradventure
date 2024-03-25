use std::path::{Path, PathBuf};

use crate::{
    interactable::Interactable,
    items::{list::unprocessed_ore::UnprocessedOre, stack::ItemStack},
    random::RandomWeightedTable,
    world::BLOCK_SIZE,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(
    Debug,
    Component,
    EnumString,
    Clone,
    Display,
    Reflect,
    Deserialize,
    Serialize,
    PartialEq,
    Eq,
    Default,
)]
pub enum Ore {
    Amethyst,
    Copper,
    Diamond,
    Emerald,
    Gold,
    Ruby,
    #[default]
    Silver,
    WitheDiamond,
}

impl Ore {
    pub fn ore_type(&self) -> OreType {
        match self {
            Self::Amethyst => OreType::Gem,
            Self::Copper => OreType::Ingot,
            Self::Diamond => OreType::Gem,
            Self::Emerald => OreType::Gem,
            Self::Gold => OreType::Ingot,
            Self::Ruby => OreType::Gem,
            Self::Silver => OreType::Ingot,
            Self::WitheDiamond => OreType::Gem,
        }
    }

    pub fn get_texture(&self) -> PathBuf {
        Path::new("textures/ores").join(format!("{}.png", self.to_string()))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum OreType {
    Gem,
    Ingot,
}

#[derive(Component)]
pub struct MinableOre {
    pub current_ore: Ore,
    pub random_ore_table: RandomWeightedTable<Ore>,
}

#[derive(Bundle)]
pub struct MinableOreBundle {
    pub ore: MinableOre,
    pub sprite: SpriteBundle,
    pub collider: Collider,
    pub interactable: Interactable,
}

impl MinableOreBundle {
    pub fn new(
        table: RandomWeightedTable<Ore>,
        pos: Vec2,
        rotation: f32,
        asset_server: &AssetServer,
    ) -> Self {
        let current_ore = table.get_random().first().unwrap().clone();
        let mut transform = Transform::from_translation(pos.extend(28.));
        transform.rotate_z(rotation);

        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(BLOCK_SIZE * 2.5)),
                    ..Default::default()
                },
                transform,
                texture: asset_server.load(current_ore.get_texture()),
                ..Default::default()
            },
            ore: MinableOre {
                current_ore,
                random_ore_table: table,
            },
            collider: Collider::capsule_y(BLOCK_SIZE / 4., BLOCK_SIZE / 2.),
            interactable: Interactable::new("player.actions.mine"),
        }
    }
}

pub struct OrePlugin;
impl Plugin for OrePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, minable_ore_update);
    }
}

fn minable_ore_update(
    mut commands: Commands,
    query: Query<(Entity, &MinableOre, &Interactable, &Transform)>,
    asset_server: Res<AssetServer>,
) {
    for (entity, ore, interactable, transform) in query.iter() {
        if interactable.just_pressed() {
            commands.entity(entity).despawn_recursive();
            commands.spawn(
                ItemStack::new_one(UnprocessedOre(ore.current_ore.clone()))
                    .bundle(&asset_server, transform.translation.xy()),
            );
        }
    }
}
