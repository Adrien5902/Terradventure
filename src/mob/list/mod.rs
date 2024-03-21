use bevy::ecs::component::Component;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use self::pig::Pig;
use self::rabbit::Rabbit;

pub mod pig;
pub mod rabbit;

#[derive(Serialize, Deserialize, Component, Clone)]
#[enum_dispatch(MobTrait)]
pub enum MobObject {
    Rabbit(Rabbit),
    Pig(Pig),
}
