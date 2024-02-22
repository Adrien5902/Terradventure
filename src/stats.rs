use bevy::prelude::*;

#[derive(Component)]
pub struct Stats {
    pub strength: f32,

    pub regen_rate: f32,
    pub health: f32,
    pub max_health: f32,

    pub def: f32,

    pub speed: f32,

    pub mass: f32,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            strength: 0.0,
            regen_rate: 0.5,
            health: 20.0,
            max_health: 20.0,
            def: 0.0,
            speed: 300.0,
            mass: 15.0,
        }
    }
}

impl Stats {
    pub fn with_health(mut self, health: f32) -> Self {
        self.health = health;
        self.max_health = health;
        self
    }

    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }
}
