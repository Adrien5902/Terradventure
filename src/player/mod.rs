pub mod class;
pub mod inventory;

use std::path::{Path, PathBuf};

use self::class::{PlayerClass, PlayerClasses};
use self::inventory::{Inventory, InventoryPlugin};
use crate::animation::{AnimatedSpriteBundle, AnimationController};
use crate::animation_maker;
use crate::gui::{
    misc::ease_out_quad,
    settings::{fov::FOV_MULTIPLIER, range::RangeSetting, Settings},
};
use crate::save::LoadSaveEvent;
use crate::state::AppState;
use crate::stats::Stats;
use crate::world::{PlainsBiome, World};
use bevy::{prelude::*, utils::HashMap};
use bevy_persistent::Persistent;
use bevy_rapier2d::prelude::*;
use serde::{Deserialize, Serialize};

const GRAVITY: f32 = 1.0;
const PLAYER_SPRITE_SHEETS_X_SIZE: u32 = 128;

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Player {
    pub class: PlayerClasses,
    jump_timer: Timer,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            jump_timer: Timer::from_seconds(0.12, TimerMode::Once),
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

fn player_setup(
    mut commands: Commands,
    mut event: EventReader<LoadSaveEvent>,
    asset_server: Res<AssetServer>,
    mut assets_img: ResMut<Assets<Image>>,
    mut assets_texture_atlas: ResMut<Assets<TextureAtlas>>,
) {
    for ev in event.read() {
        let save = ev.read();
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
            Path::new("textures/player")
                .join(c.name())
                .join(format!("{}.png", name))
        };

        let player_animations = animation_maker!(&mut assets_img, &mut assets_texture_atlas, get_texture_path, PLAYER_SPRITE_SHEETS_X_SIZE, [
            "Idle" => (1., 6, AnimationMode::Repeating, AnimationDirection::BackAndForth),
            // "Idle_2" => (3.0, 3, AnimationMode::Once, AnimationDirection::Forwards),
            "Walk" => (1., 8, AnimationMode::Custom, AnimationDirection::Forwards),
            "Jump" => (0.3, 8, AnimationMode::Once, AnimationDirection::Forwards)
        ]);

        commands
            .spawn(player)
            .insert(AnimatedSpriteBundle {
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
            })
            .insert(RigidBody::KinematicPositionBased)
            .insert(controller)
            .insert(Collider::capsule_y(10.0, 10.0))
            .insert(Inventory::default())
            .insert(Stats::default().with_health(20.0));

        PlainsBiome.spawn(&mut commands, &asset_server);
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
    output_query: Query<&KinematicCharacterControllerOutput>,
    mut query: Query<(
        &mut TextureAtlasSprite,
        &mut AnimationController,
        &Transform,
        &mut KinematicCharacterController,
        &Stats,
        &mut Player,
    )>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    settings: Res<Persistent<Settings>>,
) {
    for (mut sprite, mut animation_controller, transform, mut controller, stats, mut player) in
        query.iter_mut()
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
            if output.grounded {
                if input.just_pressed(settings.keybinds.jump.get()) {
                    player.jump_timer.reset();
                    player.jump_timer.unpause();
                    animation_controller.play("Jump");
                } else {
                    direction.y -= GRAVITY;
                }
            } else {
                if player.jump_timer.finished() || player.jump_timer.paused() {
                    direction.y -= GRAVITY;
                }
            }
        }

        if player.jump_timer.finished() {
            player.jump_timer.pause()
        }

        if !player.jump_timer.paused() {
            player.jump_timer.tick(time.delta());

            direction.y += 2.5 * ease_out_quad(player.jump_timer.percent());
        }

        direction *= stats.speed * time.delta_seconds();

        controller.translation = Some(direction);

        let walking = animation_controller.current_animation == Some("Walk");
        let moving_x = direction.x != 0.;

        if moving_x {
            sprite.flip_x = direction.x < 0.;

            if !walking {
                animation_controller.play("Walk");
            }
        } else if walking {
            animation_controller.stop();
        }

        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            camera_transform.translation = transform.translation
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
