use std::f32::consts::PI;

use bevy::prelude::*;
use rand::random;
use serde::{Deserialize, Serialize};

use crate::{
    animation::AnimationController, gui::styles::text_style, mob::Mob,
    player::money::DropMoneyEvent, state::AppState,
};

pub struct StatsPlugin;
impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update.run_if(in_state(AppState::InGame)));
    }
}

#[derive(Component)]
pub struct DamageTaken {
    lifetime: Timer,
}

fn update(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Stats)>,
    transform_query: Query<&Transform, With<Stats>>,
    mut damage_query: Query<(Entity, &mut DamageTaken, &mut Transform), Without<Stats>>,
    mut animation_controller_query: Query<&mut AnimationController>,
    mob_query: Query<&Mob>,
    mut money_event: EventWriter<DropMoneyEvent>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
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

        if let Ok(transform) = transform_query.get(entity) {
            if stats.taken_damage != 0.0 {
                let mut taken_damage_transform = Transform::from_translation(transform.translation);
                taken_damage_transform.translation.z += 1.0;
                taken_damage_transform.rotate_z(random::<f32>() * PI / 4. - PI / 8.);

                commands.spawn((
                    DamageTaken {
                        lifetime: Timer::from_seconds(1.0, TimerMode::Once),
                    },
                    Text2dBundle {
                        text: Text::from_section(
                            (-stats.taken_damage).to_string(),
                            TextStyle {
                                color: Color::RED,
                                font_size: 15.,
                                ..text_style(&asset_server)
                            },
                        ),
                        transform: taken_damage_transform,
                        ..Default::default()
                    },
                ));
            }

            stats.taken_damage = 0.;

            if stats.health <= 0. {
                if let Ok(mut animation_controller) = animation_controller_query.get_mut(entity) {
                    if animation_controller
                        .animations
                        .get(&"Dead".to_owned())
                        .is_none()
                        || animation_controller.just_finished("Dead")
                    {
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
                    } else if animation_controller.current_animation != Some("Dead".to_owned()) {
                        animation_controller.play("Dead");
                    }
                }
            }
        }
    }

    for (entity, mut damage_taken, mut transform) in damage_query.iter_mut() {
        damage_taken.lifetime.tick(time.delta());
        transform.translation.y += 50.0 * time.delta_seconds();

        if damage_taken.lifetime.finished() {
            commands.entity(entity).despawn();
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

    #[serde(skip)]
    taken_damage: f32,
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
            taken_damage: 0.,
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
        self.taken_damage = calc_amount;
        calc_amount
    }
}
