use crate::{
    mob::{Mob, MobTrait, MobType},
    stats::Stats,
};
use bevy::prelude::*;
use bevy_rapier2d::geometry::Collider;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize)]
pub struct Sheep {
    pub shorn: bool,
}

impl MobTrait for Sheep {
    fn name(&self) -> &'static str {
        "sheep"
    }
    fn default_stats(&self) -> Stats {
        Stats::default().with_health(10.0).with_speed(50.0)
    }
    fn mob_obj(&self) -> Mob {
        Mob::new(MobType::Passive, None)
    }
    fn collider(&self) -> Collider {
        Collider::cuboid(8.0, 8.0)
    }
}

impl Default for Sheep {
    fn default() -> Self {
        Self { shorn: true }
    }
}
