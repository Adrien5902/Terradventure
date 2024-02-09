pub mod inventory;
pub mod model;

use self::{inventory::Inventory, model::PlayerModel};
use crate::gui::settings::Settings;
use crate::{
    state::AppState,
    world::{ForestBiome, World},
};
use bevy::prelude::*;
use bevy_persistent::Persistent;
use bevy_rapier2d::prelude::*;

const GRAVITY: f32 = 1.0;
const FOV: f32 = 0.2;

#[derive(Component, Default)]
pub struct Player {
    pub inventory: Inventory,
    pub model: PlayerModel,
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
            .add_systems(Startup, spawn_camera);
    }
}

fn player_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let controller = KinematicCharacterController::default();
    let mut transform = Transform::default();
    transform.translation.y -= 20.0;

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("player.png"),
            transform,
            ..Default::default()
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(controller)
        .insert(Player::default())
        .insert(Collider::cuboid(8.0, 8.0));

    ForestBiome.spawn(commands, &asset_server);
}

fn despawn_player(mut commands: Commands, query: Query<Entity, With<Player>>) {
    if let Ok(player) = query.get_single() {
        commands.entity(player).despawn();
    }
}

fn character_controller_update(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut KinematicCharacterController, Without<Camera2d>>,
    output_query: Query<&KinematicCharacterControllerOutput>,
    mut sprite_query: Query<(&mut Sprite, &Transform), With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    settings: Res<Persistent<Settings>>,
) {
    for mut controller in query.iter_mut() {
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
                    direction.y += 20.0;
                } else {
                    direction.y -= GRAVITY;
                }
            } else {
                if !input.just_released(settings.keybinds.jump.get()) {
                    direction.y -= GRAVITY;
                }
            }
        }

        direction *= 300.0 * time.delta_seconds();

        controller.translation = Some(direction);

        if let Ok((mut sprite, transform)) = sprite_query.get_single_mut() {
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
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            far: 1000.,
            near: -1000.,
            scale: FOV,
            ..Default::default()
        },
        ..Default::default()
    });
}
