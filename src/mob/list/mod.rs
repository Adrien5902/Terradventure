use bevy::ecs::component::Component;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use self::{rabbit::Rabbit, sheep::Sheep};

pub mod rabbit;
pub mod sheep;

#[derive(Serialize, Deserialize, Component, Clone)]
#[enum_dispatch(MobTrait)]
pub enum MobObject {
    Sheep(Sheep),
    Rabbit(Rabbit),
}
