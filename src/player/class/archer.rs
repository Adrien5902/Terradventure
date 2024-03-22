use std::path::PathBuf;

use super::{can_attack, is_of_class, PlayerClass};
use crate::{
    animation::{AnimatedSpriteBundle, Animation, AnimationController},
    animation_maker,
    gui::settings::{keybinds::Keybind, Settings},
    mob::Mob,
    player::{sprite_vec, Player},
    stats::Stats,
    world::BLOCK_SIZE,
};
use bevy::{prelude::*, sprite::Anchor, utils::hashbrown::HashMap};
use bevy_rapier2d::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Archer;

impl PlayerClass for Archer {
    fn name(&self) -> &'static str {
        "archer"
    }

    fn normal_attack_chain_count(&self) -> u8 {
        1
    }
}

impl Plugin for Archer {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            ((special_attacks).run_if(can_attack), arrow_update).run_if(is_of_class::<Self>),
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

    for (i, name) in vec!["Fire", "Magic", "Poison"].into_iter().enumerate() {
        let capital_name = format!("Special_Attack_{}", i + 1);

        if settings
            .keybinds
            .get_field::<Keybind>(&format!("special_attack_{}", i + 1))
            .unwrap()
            .just_pressed(&keyboard_input, &mouse_input)
            && player.mana.get() >= Arrow::MANA_COST
        {
            animation_controller.play(&capital_name);
        }

        if animation_controller.just_finished(&capital_name)
            && player.mana.try_remove(Arrow::MANA_COST)
        {
            commands.spawn(Arrow::bundle(
                name,
                transform.translation.xy(),
                sprite,
                &asset_server,
            ));
        }
    }
}

fn arrow_update(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    player_query: Query<Entity, With<Player>>,
    mut query: Query<(
        Entity,
        &TextureAtlasSprite,
        &mut Transform,
        &Collider,
        &mut Arrow,
        &mut AnimationController,
    )>,
    mut mob_query: Query<(&mut Mob, &mut Stats)>,
    time: Res<Time>,
) {
    let Ok(player_entity) = player_query.get_single() else {
        return;
    };

    for (entity, sprite, mut transform, collider, mut arrow, mut animation_controller) in
        query.iter_mut()
    {
        if !arrow.hit {
            let movement = sprite_vec(sprite).x * Arrow::SPEED * time.delta_seconds();
            transform.translation.x += movement;
            arrow.traveled_dist += movement;

            if arrow.traveled_dist >= Arrow::MAX_TRAVEL_DIST {
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
                    arrow.hit = true;
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
pub struct Arrow {
    hit: bool,
    traveled_dist: f32,
}

impl Arrow {
    const SPEED: f32 = 400.;
    const MAX_TRAVEL_DIST: f32 = BLOCK_SIZE * 400.;
    const MANA_COST: f32 = 40.;

    pub fn animations(asset_server: &Res<AssetServer>) -> HashMap<String, Animation> {
        let get_texture =
            |name: &str| -> PathBuf { Archer.get_texture_path(&format!("{}_Arrow", name)) };

        animation_maker!(
            asset_server,
            get_texture,
            128,
            [
                "Fire" => (0.5, AnimationMode::Custom, AnimationDirection::Forwards),
                "Magic" => (0.4, AnimationMode::Custom, AnimationDirection::Forwards),
                "Poison" => (0.6, AnimationMode::Custom, AnimationDirection::Forwards)
            ]
        )
    }

    pub fn bundle(
        name: &str,
        pos: Vec2,
        player_sprite: &TextureAtlasSprite,
        asset_server: &Res<AssetServer>,
    ) -> ArrowBundle {
        let mut animation_controller = AnimationController::new(Self::animations(asset_server));
        animation_controller.play(name);

        ArrowBundle {
            arrow: Arrow::default(),
            collider: Collider::capsule_x(Player::SIZE / 8., Player::SIZE / 10.),
            rigid_body: RigidBody::Fixed,
            sprite: AnimatedSpriteBundle {
                animation_controller,
                sprite: SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        flip_x: player_sprite.flip_x,
                        custom_size: Some(Vec2::splat(Player::SIZE)),
                        anchor: if name == "Poison" {
                            Anchor::Custom(Vec2::new(0.0, -0.25))
                        } else {
                            Anchor::default()
                        },
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
pub struct ArrowBundle {
    arrow: Arrow,
    collider: Collider,
    rigid_body: RigidBody,
    sprite: AnimatedSpriteBundle,
}
