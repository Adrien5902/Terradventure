use crate::{
    mob::{Mob, MobBundle, MobTrait, MobType},
    stats::Stats,
};
use bevy::prelude::*;
use bevy_rapier2d::geometry::Collider;

#[derive(Component)]
pub struct Sheep {
    pub shorn: bool,
}

impl MobTrait for Sheep {
    fn name(&self) -> &'static str {
        "sheep"
    }

    fn bundle(&self, asset_server: Res<AssetServer>) -> MobBundle {
        MobBundle {
            collider: Collider::cuboid(8.0, 4.0),
            mob: Mob::new(MobType::Passive, None),
            sprite: SpriteBundle {
                texture: asset_server.load(self.texture()),
                ..Default::default()
            },
            stats: Stats::default().with_health(10.0),
        }
    }
}

impl Default for Sheep {
    fn default() -> Self {
        Self { shorn: true }
    }
}
