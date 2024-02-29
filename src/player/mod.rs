pub mod class;
pub mod inventory;

use std::path::{Path, PathBuf};
use std::time::Duration;

use self::class::{PlayerClass, PlayerClasses};
use self::inventory::{Inventory, InventoryPlugin};
use crate::animation::{
    AnimatedSpriteBundle, Animation, AnimationController, AnimationDirection, AnimationMode,
};
use crate::animation_maker;
use crate::gui::{
    misc::ease_out_quad,
    settings::{fov::FOV_MULTIPLIER, range::RangeSetting, Settings},
};
use crate::mob::Mob;
use crate::save::LoadSaveEvent;
use crate::state::AppState;
use crate::stats::Stats;
use crate::world::BLOCK_SIZE;
use bevy::{prelude::*, utils::HashMap};
use bevy_persistent::Persistent;
use bevy_rapier2d::prelude::*;
use serde::{Deserialize, Serialize};

pub const PLAYER_SPRITE_SHEETS_X_SIZE: u32 = 128;

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Player {
    pub class: PlayerClasses,
    pub inventory: Inventory,
    jump_timer: Timer,
    chain_attack_timer: Timer,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            jump_timer: Timer::from_seconds(0.12, TimerMode::Once),
            chain_attack_timer: Timer::from_seconds(0.5, TimerMode::Once),
            inventory: Inventory::default(),
            class: PlayerClasses::default(),
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (character_controller_update, player_setup).run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            OnEnter(AppState::MainMenu(
                crate::gui::main_menu::MainMenuState::Default,
            )),
            despawn_player,
        )
        .add_systems(Startup, spawn_camera)
        .add_plugins(InventoryPlugin);
    }
}

pub const PLAYER_TEXTURE: &str = "textures/player";

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    animated_sprite: AnimatedSpriteBundle,
    rigid_body: RigidBody,
    controller: KinematicCharacterController,
    collider: Collider,
    stats: Stats,
}

fn player_setup(
    mut commands: Commands,
    mut event: EventReader<LoadSaveEvent>,
    asset_server: Res<AssetServer>,
    mut assets_img: ResMut<Assets<Image>>,
    mut assets_texture_atlas: ResMut<Assets<TextureAtlas>>,
) {
    for ev in event.read() {
        let save = ev.read();

        let world = save.world.clone();
        world.spawn(&mut commands, &asset_server);

        let controller = KinematicCharacterController {
            autostep: Some(CharacterAutostep {
                min_width: CharacterLength::Relative(0.0),
                max_height: CharacterLength::Relative(0.3),
                include_dynamic_bodies: false,
            }),
            ..Default::default()
        };

        let player = save.player.player.clone();
        let transform = Transform::from_translation(save.player.pos.extend(0.0));

        let get_texture_path = |name: &str| -> PathBuf {
            let c: PlayerClasses = player.class.clone().into();
            Path::new(PLAYER_TEXTURE)
                .join(c.name())
                .join(format!("{}.png", name))
        };

        let player_animations = animation_maker!(&mut assets_img, &mut assets_texture_atlas, get_texture_path, PLAYER_SPRITE_SHEETS_X_SIZE, [
            "Idle" => (1., AnimationMode::Repeating, AnimationDirection::BackAndForth),
            // "Idle_2" => (3.0, 3, AnimationMode::Once, AnimationDirection::Forwards),
            "Walk" => (1., AnimationMode::Custom, AnimationDirection::Forwards),
            "Jump" => (0.3, AnimationMode::Once, AnimationDirection::Forwards),
            "Special_Attack_1" => (1., AnimationMode::Once, AnimationDirection::Forwards),
            "Special_Attack_2" => (1., AnimationMode::Once, AnimationDirection::Forwards),
            "Special_Attack_3" => (1., AnimationMode::Once, AnimationDirection::Forwards)
        ]);

        for i in 1..player.class.normal_attack_chain_count() {
            let name = format!("Attack_{}", i);
            player_animations.insert(
                &name,
                Animation::new(
                    name,
                    &mut assets_img,
                    &mut assets_texture_atlas,
                    Duration::from_seconds(0.5),
                    PLAYER_SPRITE_SHEETS_X_SIZE,
                    AnimationMode::Once,
                    AnimationDirection::Forwards,
                ),
            )
        }

        commands.spawn(PlayerBundle {
            player,
            animated_sprite: AnimatedSpriteBundle {
                sprite: SpriteSheetBundle {
                    transform,
                    sprite: TextureAtlasSprite {
                        anchor: bevy::sprite::Anchor::Custom(Vec2::new(0.0, -0.2)),
                        custom_size: Some(Vec2::splat(64.0)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                animation_controller: AnimationController::new(player_animations)
                    .with_default("Idle"),
            },
            collider: Collider::capsule_y(10.0, 10.0),
            controller,
            rigid_body: RigidBody::KinematicPositionBased,
            stats: Stats::default().with_health(20.0),
        });
    }
}

fn despawn_player(mut commands: Commands, query: Query<Entity, With<Player>>) {
    if let Ok(player) = query.get_single() {
        commands.entity(player).despawn();
    }
}

fn character_controller_update(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    output_query: Query<&KinematicCharacterControllerOutput, With<Player>>,
    mut query: Query<(
        Entity,
        &mut TextureAtlasSprite,
        &mut AnimationController,
        &Transform,
        &mut KinematicCharacterController,
        &Stats,
        &mut Player,
    )>,
    mut mob_query: Query<(&mut Stats, &mut Mob), Without<Player>>,
    rapier_context: Res<RapierContext>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    settings: Res<Persistent<Settings>>,
) {
    for (
        entity,
        mut sprite,
        mut animation_controller,
        transform,
        mut controller,
        stats,
        mut player,
    ) in query.iter_mut()
    {
        let mut direction = Vec2::default();

        if input.pressed(settings.keybinds.move_left.get()) {
            direction.x -= 1.0;
            animation_controller.tick(&time)
        }

        if input.pressed(settings.keybinds.move_right.get()) {
            direction.x += 1.0;
            animation_controller.tick(&time)
        }

        if let Ok(output) = output_query.get_single() {
            if output.grounded && input.just_pressed(settings.keybinds.jump.get()) {
                player.jump_timer.reset();
                player.jump_timer.unpause();
                animation_controller.play("Jump");
            } else if player.jump_timer.finished() || player.jump_timer.paused() {
                direction.y -= 1.0;
            }
        } else {
            direction.y -= 1.0;
        }

        if player.jump_timer.finished() {
            player.jump_timer.pause()
        }

        if !player.jump_timer.paused() {
            player.jump_timer.tick(time.delta());

            direction.y += 2.5 * ease_out_quad(player.jump_timer.percent());
        }

        direction.x *= stats.speed * time.delta_seconds();
        direction.y *= stats.mass * time.delta_seconds();

        controller.translation = Some(direction);

        let animating = animation_controller.current_animation.is_some();
        let moving_x = direction.x != 0.;

        if moving_x {
            sprite.flip_x = direction.x < 0.;

            if !animating {
                animation_controller.play("Walk");
            }
        } else if animation_controller.current_animation == Some("Walk") {
            animation_controller.stop();
        }

        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            camera_transform.translation = transform.translation
        }

        if input.just_pressed(settings.keybinds.attack.get()) {
            animation_controller.play("Attack_1");
        }

        if input.just_pressed(settings.keybinds.special_attack_1.get()) {
            animation_controller.play("Special_Attack_1");
        }

        if input.just_pressed(settings.keybinds.special_attack_2.get()) {
            animation_controller.play("Special_Attack_2");
        }

        if input.just_pressed(settings.keybinds.special_attack_3.get()) {
            animation_controller.play("Special_Attack_3");
        }

        if animation_controller.just_finished("Special_Attack_1") {
            player.class.special_attack_1(
                entity,
                &rapier_context,
                transform,
                sprite.flip_x,
                &mut mob_query,
            );
        }

        if animation_controller.just_finished("Special_Attack_2") {
            player.class.special_attack_2(
                entity,
                &rapier_context,
                transform,
                sprite.flip_x,
                &mut mob_query,
            );
        }

        if animation_controller.just_finished("Special_Attack_3") {
            player.class.special_attack_3(
                entity,
                &rapier_context,
                transform,
                sprite.flip_x,
                &mut mob_query,
            );
        }

        if animation_controller.just_finished("Attack_1") {
            let mut hitbox_translation = transform.translation.xy();
            hitbox_translation.x += (if sprite.flip_x { -1. } else { 1. }) * BLOCK_SIZE;

            rapier_context.intersections_with_shape(
                hitbox_translation,
                Rot::default(),
                &Collider::ball(BLOCK_SIZE * 0.8),
                QueryFilter {
                    exclude_rigid_body: Some(entity),
                    ..Default::default()
                },
                |hit_entity| {
                    if let Ok((mut stats, mut mob)) = mob_query.get_mut(hit_entity) {
                        mob.hit_animation();
                    }

                    true
                },
            );
        }
    }
}

fn spawn_camera(mut commands: Commands, settings: Res<Persistent<Settings>>) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            far: 1000.,
            near: -1000.,
            scale: settings.fov.get_value() * FOV_MULTIPLIER,
            ..Default::default()
        },
        ..Default::default()
    });
}
