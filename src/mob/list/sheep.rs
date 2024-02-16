use bevy_rapier2d::geometry::Collider;

use crate::{mob::MobType, mob_maker};

#[derive(Component)]
pub struct Sheep {
    pub shorn: bool,
}

impl Default for Sheep {
    fn default() -> Self {
        Self { shorn: true }
    }
}

mob_maker!(Sheep, "sheep", MobType::Passive);
