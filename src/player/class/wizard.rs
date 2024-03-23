use std::time::Duration;

use crate::{
    animation::{
        AnimatedSpriteBundle, Animation, AnimationController, AnimationDirection, AnimationMode,
    },
    gui::settings::{keybinds::Keybind, Settings},
    mob::Mob,
    player::{sprite_vec, Player},
    stats::Stats,
    world::BLOCK_SIZE,
};

use super::{can_attack, is_of_class, PlayerClass, PlayerClasses};
use bevy::{prelude::*, sprite::Anchor, utils::hashbrown::HashMap};
use bevy_rapier2d::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Wizard;

impl PlayerClass for Wizard {
    fn name(&self) -> &'static str {
        "wizard"
    }

    fn normal_attack_chain_count(&self) -> u8 {
        3
    }
}

impl Plugin for Wizard {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            ((special_attacks).run_if(can_attack), charge_update).run_if(is_of_class::<Self>),
        );
    }
}

fn special_attacks(
    mut commands: Commands,
    settings: Res<Settings>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut query: Query<(
        &mut Player,
        &mut AnimationController,
        &Transform,
        &TextureAtlasSprite,
    )>,
    asset_server: Res<AssetServer>,
) {
    let Ok((mut player, mut animation_controller, transform, sprite)) = query.get_single_mut()
    else {
        return;
    };

    for i in 1..=3 {
        let capital_name = format!("Special_Attack_{i}");

        if settings
            .keybinds
            .get_field::<Keybind>(&format!("special_attack_{i}"))
            .unwrap()
            .just_pressed(&keyboard_input, &mouse_input)
            && player.mana.get() >= Charge::MANA_COST
        {
            animation_controller.play(&capital_name);
        }

        if animation_controller.just_finished(&capital_name)
            && player.mana.try_remove(Charge::MANA_COST)
        {
            let PlayerClasses::Wizard(class) = &player.class else {
                return;
            };

            commands.spawn(Charge::bundle(
                i,
                transform.translation.xy(),
                class,
                sprite,
                &asset_server,
            ));
        }
    }
}

fn charge_update(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    player_query: Query<Entity, With<Player>>,
    mut query: Query<(
        Entity,
        &TextureAtlasSprite,
        &mut Transform,
        &Collider,
        &mut Charge,
        &mut AnimationController,
    )>,
    mut mob_query: Query<(&mut Mob, &mut Stats)>,
    time: Res<Time>,
) {
    let Ok(player_entity) = player_query.get_single() else {
        return;
    };

    for (entity, sprite, mut transform, collider, mut charge, mut animation_controller) in
        query.iter_mut()
    {
        if !charge.hit {
            let movement = sprite_vec(sprite).x * Charge::SPEED * time.delta_seconds();
            transform.translation.x += movement;
            charge.traveled_dist += movement;

            if charge.traveled_dist >= Charge::MAX_TRAVEL_DIST {
                commands.entity(entity).despawn();
                continue;
            }

            rapier_context.intersections_with_shape(
                transform.translation.xy(),
                0.0,
                collider,
                QueryFilter {
                    predicate: Some(&|e| e != entity && e != player_entity),
                    ..Default::default()
                },
                |hit_entity| {
                    charge.hit = true;
                    if let Ok((mut mob, mut stats)) = mob_query.get_mut(hit_entity) {
                        mob.hit_animation();
                        stats.take_damage(15.);
                    }

                    true
                },
            );
        } else {
            animation_controller.tick(&time);
            if animation_controller.just_finished.is_some() {
                commands.entity(entity).despawn();
            }
        }
    }
}

#[derive(Component, Default)]
pub struct Charge {
    hit: bool,
    traveled_dist: f32,
}

impl Charge {
    const SPEED: f32 = 400.;
    const MAX_TRAVEL_DIST: f32 = BLOCK_SIZE * 400.;
    const MANA_COST: f32 = 40.;

    pub fn animations(
        class: &Wizard,
        i: u32,
        asset_server: &AssetServer,
    ) -> HashMap<String, Animation> {
        let mut map = HashMap::new();
        map.insert(
            "Blow".into(),
            Animation::new(
                class.get_texture_path(&format!("Charge_{i}")),
                asset_server,
                Duration::from_secs_f32(1.0),
                128,
                AnimationMode::Custom,
                AnimationDirection::Forwards,
            ),
        );
        map
    }

    pub fn bundle(
        i: u32,
        pos: Vec2,
        class: &Wizard,
        player_sprite: &TextureAtlasSprite,
        asset_server: &Res<AssetServer>,
    ) -> ChargeBundle {
        let mut animation_controller =
            AnimationController::new(Self::animations(class, i, asset_server));
        animation_controller.play("Blow");

        ChargeBundle {
            charge: Charge::default(),
            collider: Collider::capsule_x(Player::SIZE / 10., Player::SIZE / 10.),
            rigid_body: RigidBody::Fixed,
            sprite: AnimatedSpriteBundle {
                animation_controller,
                sprite: SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        flip_x: player_sprite.flip_x,
                        custom_size: Some(Vec2::splat(Player::SIZE)),
                        anchor: Anchor::default(),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(
                        (pos + sprite_vec(player_sprite) * Player::SIZE / 4.).extend(15.0),
                    ),
                    ..Default::default()
                },
            },
        }
    }
}

#[derive(Bundle)]
pub struct ChargeBundle {
    charge: Charge,
    collider: Collider,
    rigid_body: RigidBody,
    sprite: AnimatedSpriteBundle,
}
