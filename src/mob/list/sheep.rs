use crate::{
    animation::Animation,
    animation_maker,
    mob::{MobTrait, MobType},
    stats::Stats,
};
use bevy::{prelude::*, utils::hashbrown::HashMap};
use bevy_rapier2d::geometry::Collider;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Sheep {
    pub shorn: bool,
}

impl MobTrait for Sheep {
    fn name(&self) -> &'static str {
        "sheep"
    }
    fn animations(&self, asset_server: &Res<AssetServer>) -> HashMap<String, Animation> {
        let function = |a| self.texture(a);
        animation_maker!(&asset_server, function, 16, [
            "Idle" => (5.0, AnimationMode::Once, AnimationDirection::Forwards)
        ])
    }
    fn default_stats(&self) -> Stats {
        Stats::default().with_health(10.0).with_speed(50.0)
    }
    fn typ(&self) -> MobType {
        MobType::Passive
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
