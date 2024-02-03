pub mod inventory;
pub mod model;

use self::{inventory::Inventory, model::PlayerModel};
use crate::{
    state::AppState,
    world::{ForestBiome, World},
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const GRAVITY: f32 = 20.0;

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
        .insert(Collider::ball(30.0));

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
    mut query: Query<(&mut KinematicCharacterController, &mut Transform), Without<Camera2d>>,
    mut sprite_query: Query<&mut Sprite, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    for (mut controller, mut transform) in query.iter_mut() {
        let mut direction = Vec2::default();

        if input.pressed(KeyCode::Q) {
            direction.x -= 1.0;
        }

        if input.pressed(KeyCode::D) {
            direction.x += 1.0;
        }

        if input.pressed(KeyCode::Space) {
            direction.y += 2.0;
        }

        direction.y -= GRAVITY * time.delta_seconds();

        controller.translation = Some(direction * 300.0 * time.delta_seconds());

        if let Some(translation) = controller.translation {
            transform.translation.x += translation.x;
            transform.translation.y += translation.y;

            if let Ok(mut sprite) = sprite_query.get_single_mut() {
                if input.just_pressed(KeyCode::Q) {
                    sprite.flip_x = true;
                }

                if input.just_pressed(KeyCode::D) {
                    sprite.flip_x = false;
                }
            }
        }

        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            camera_transform.translation = transform.translation
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        ..Default::default()
    });
}
