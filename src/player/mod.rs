pub mod inventory;
pub mod model;

use self::inventory::InventoryPlugin;
use self::{inventory::Inventory, model::PlayerModel};
use crate::gui::misc::ease_out_quad;
use crate::gui::settings::fov::FOV_MULTIPLIER;
use crate::gui::settings::range::RangeSetting;
use crate::gui::settings::Settings;
use crate::state::AppState;
use crate::stats::Stats;
use crate::world::{PlainsBiome, World};
use bevy::prelude::*;
use bevy_persistent::Persistent;
use bevy_rapier2d::prelude::*;

const GRAVITY: f32 = 1.0;

#[derive(Component, Default)]
pub struct Player {
    pub model: PlayerModel,
    jump_timer: Timer,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::MainMenu), player_setup)
            .add_systems(
                Update,
                character_controller_update.run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnEnter(AppState::MainMenu), despawn_player)
            .add_systems(Startup, spawn_camera)
            .add_plugins(InventoryPlugin);
    }
}

fn player_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let controller = KinematicCharacterController {
        autostep: Some(CharacterAutostep {
            min_width: CharacterLength::Relative(0.0),
            max_height: CharacterLength::Relative(0.3),
            include_dynamic_bodies: false,
        }),
        ..Default::default()
    };
    let mut transform = Transform::default();
    transform.translation.y -= 20.0;

    let mut jump_timer = Timer::from_seconds(0.12, TimerMode::Once);
    jump_timer.pause();

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("player.png"),
            transform,
            ..Default::default()
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(controller)
        .insert(Player {
            jump_timer,
            ..Default::default()
        })
        .insert(Collider::capsule_y(8.0, 8.0))
        .insert(Inventory::default())
        .insert(Stats::default().with_health(20.0));

    PlainsBiome.spawn(commands, &asset_server);
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
        &mut Sprite,
        &Transform,
        &mut KinematicCharacterController,
        &Stats,
        &mut Player,
    )>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    settings: Res<Persistent<Settings>>,
) {
    for (mut sprite, transform, mut controller, stats, mut player) in query.iter_mut() {
        let mut direction = Vec2::default();

        if input.pressed(settings.keybinds.move_left.get()) {
            direction.x -= 1.0;
        }

        if input.pressed(settings.keybinds.move_right.get()) {
            direction.x += 1.0;
        }

        if let Ok(output) = output_query.get_single() {
            if output.grounded {
                if input.just_pressed(settings.keybinds.jump.get()) {
                    player.jump_timer.reset();
                    player.jump_timer.unpause()
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

        if input.just_pressed(settings.keybinds.move_left.get()) {
            sprite.flip_x = true;
        }

        if input.just_pressed(settings.keybinds.move_right.get()) {
            sprite.flip_x = false;
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
