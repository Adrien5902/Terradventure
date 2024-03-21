use crate::{
    animation::Animation,
    animation_maker,
    mob::{MobTrait, MobType},
    stats::Stats,
};
use bevy::{prelude::*, sprite::Anchor, utils::hashbrown::HashMap};
use bevy_rapier2d::geometry::Collider;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, Clone, Default)]
pub struct Pig;

impl MobTrait for Pig {
    fn name(&self) -> &'static str {
        "pig"
    }
    fn animations(&self, asset_server: &Res<AssetServer>) -> HashMap<String, Animation> {
        let function = |a| self.texture(a);
        animation_maker!(&asset_server, function, 40, [
            "Idle" => (1.0, AnimationMode::Repeating, AnimationDirection::Forwards),
            "Walk" => (1.0, AnimationMode::Repeating, AnimationDirection::Forwards)
        ])
    }
    fn default_stats(&self) -> Stats {
        Stats::default().with_health(15.0)
    }
    fn typ(&self) -> MobType {
        MobType::Passive
    }

    fn collider(&self) -> Collider {
        Collider::capsule_x(11.0, 14.0)
    }

    fn sprite_custom_size_and_anchor(&self) -> (Option<Vec2>, Option<Anchor>) {
        (
            Some(Vec2::new(50., 50.)),
            Some(Anchor::Custom(Vec2::new(0.0, -0.25))),
        )
    }
}
