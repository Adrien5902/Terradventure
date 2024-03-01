use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{animation::AnimationController, state::AppState};

pub struct StatsPlugin;
impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update, death).run_if(in_state(AppState::InGame)));
    }
}

fn update(mut query: Query<&mut Stats>, time: Res<Time>) {
    for mut stats in query.iter_mut() {
        if stats.health < stats.max_health {
            let new_val = stats.health + stats.regen_rate * time.delta_seconds();
            stats.health = if new_val > stats.max_health {
                stats.max_health
            } else {
                new_val
            };
        }
    }
}

fn death(mut commands: Commands, mut query: Query<(Entity, &mut AnimationController, &Stats)>) {
    for (entity, mut anim, stats) in query.iter_mut() {
        if stats.health <= 0. {
            if anim.animations.get(&"Dead".to_owned()).is_none() {
                commands.entity(entity).despawn_recursive();
                continue;
            }

            if anim.current_animation != Some("Dead".to_owned()) {
                anim.play("Dead");
                continue;
            }

            if anim.just_finished("Dead") {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

#[derive(Component, Clone, Serialize, Deserialize)]
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
            mass: 300.0,
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
