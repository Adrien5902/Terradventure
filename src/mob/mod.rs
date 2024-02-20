pub mod list;

use crate::{items::loot_table::LootTable, stats::Stats};
use bevy::{asset::AssetPath, prelude::*};
use bevy_rapier2d::prelude::*;
use std::path::Path;

#[derive(Component)]
pub struct Mob {
    pub typ: MobType,
    pub death_loot_table: Option<Handle<LootTable>>,
    pub ai: &'static dyn MobAi,
}

impl Mob {
    pub fn new(typ: MobType, death_loot_table: Option<Handle<LootTable>>) -> Self {
        Self {
            ai: typ.clone().into(),
            typ,
            death_loot_table,
        }
    }
}

#[derive(Bundle)]
pub struct MobBundle {
    mob: Mob,
    stats: Stats,
    collider: Collider,
    sprite: SpriteBundle,
}

#[derive(Clone)]
pub enum MobType {
    Passive,
    Neutral,
    Agressive,
}

impl Into<&'static dyn MobAi> for MobType {
    fn into(self) -> &'static dyn MobAi {
        match self {
            Self::Passive => &PassiveDefaultMobAi,
            _ => &PassiveDefaultMobAi,
        }
    }
}

pub trait MobAi: Sync {
    fn update(&self, transform: &mut Transform, stats: &Stats);
}

pub struct PassiveDefaultMobAi;
impl MobAi for PassiveDefaultMobAi {
    fn update(&self, transform: &mut Transform, stats: &Stats) {}
}

pub struct MobTexture(pub &'static str);
impl<'a> Into<AssetPath<'a>> for MobTexture {
    fn into(self) -> AssetPath<'a> {
        let path = Path::new("textures/mobs").join(self.0);
        AssetPath::from(path)
    }
}

pub struct MobLootTable(pub &'static str);
impl<'a> Into<AssetPath<'a>> for MobLootTable {
    fn into(self) -> AssetPath<'a> {
        let path = Path::new("loot_tables/mobs").join(format!("{}.loot_table.json", self.0));
        AssetPath::from(path)
    }
}

#[macro_export]
macro_rules! mob_maker {
    ($custom_type:ty, $name:literal, $mob_type:expr) => {
        use crate::mob::MobTexture;
        use bevy::prelude::*;

        pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>, position: Vec2) {
            commands
                .spawn(<$custom_type>::default())
                .insert(SpriteBundle {
                    texture: asset_server.load(MobTexture($name)),
                    ..Default::default()
                })
                .insert(Transform {
                    translation: position.extend(0.),
                    ..Default::default()
                });
        }
    };
}

pub struct MobName(&'static str);

pub trait MobTrait: Sized + Component {
    fn name(&self) -> &'static str;
    fn texture(&self) -> MobTexture {
        MobTexture(self.name())
    }
    fn bundle(&self, asset_server: Res<AssetServer>) -> MobBundle;
    fn spawn(self, mut commands: Commands, asset_server: Res<AssetServer>) {
        let bundle = self.bundle(asset_server);
        commands.spawn(self).insert(bundle);
    }
}
