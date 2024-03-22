use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    animation::AnimationController, mob::Mob, player::money::DropMoneyEvent, state::AppState,
};

pub struct StatsPlugin;
impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update, death).run_if(in_state(AppState::InGame)));
    }
}

#[derive(Component)]
pub struct HealthBar;

fn update(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Stats)>,
    children_query: Query<&Children>,
    mut health_bar_query: Query<(Entity, &mut HealthBar)>,
    time: Res<Time>,
) {
    for (entity, mut stats) in query.iter_mut() {
        if stats.health < stats.max_health {
            let new_val = stats.health + stats.regen_rate * time.delta_seconds();
            stats.health = if new_val > stats.max_health {
                stats.max_health
            } else {
                new_val
            };
        }

        let health_bar_opt = children_query.get(entity).ok().and_then(|children| {
            children
                .iter()
                .find_map(|child| health_bar_query.get(*child).ok())
        });

        if let Some((health_bar_entity, health_bar)) = health_bar_opt {
            if stats.health == stats.max_health {
                commands.entity(health_bar_entity).despawn_recursive();
            }
        } else {
            if stats.health != 0. {
                commands.spawn(Text2dBundle {
                    ..Default::default()
                });
            }
        }
    }
}

fn death(
    mut commands: Commands,
    mut query: Query<(Entity, &mut AnimationController, &Stats, &Transform)>,
    mob_query: Query<&Mob>,
    asset_server: Res<AssetServer>,
    mut money_event: EventWriter<DropMoneyEvent>,
) {
    for (entity, mut anim, stats, transform) in query.iter_mut() {
        if stats.health <= 0. {
            if anim.animations.get(&"Dead".to_owned()).is_none() || anim.just_finished("Dead") {
                if let Ok(mob) = mob_query.get(entity) {
                    let pos = transform.translation.xy();
                    let (money, items) = mob.get_loot();

                    money_event.send(DropMoneyEvent {
                        amount: money,
                        pos: transform.translation.xy(),
                    });

                    items.into_iter().for_each(|loot| {
                        commands.spawn(loot.bundle(&asset_server, pos));
                    });
                }

                commands.entity(entity).despawn_recursive();
            } else if anim.current_animation != Some("Dead".to_owned()) {
                anim.play("Dead");
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

    /// # Returns
    /// The amount of damage actually taken accounting def and stuff
    pub fn take_damage(&mut self, amount: f32) -> f32 {
        let calc_amount = amount - self.def;
        self.health -= calc_amount;
        calc_amount
    }
}
