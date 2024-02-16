use bevy::prelude::*;

#[derive(Component)]
pub struct Stats {
    pub strength: u32,

    pub regen_rate: u32,
    pub health: u32,
    pub max_health: u32,

    pub def: u32,

    pub speed: u32,
}
