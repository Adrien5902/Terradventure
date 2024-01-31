pub mod model;

use bevy::prelude::*;
use bevy_rapier2d::{control::KinematicCharacterController, dynamics::RigidBody};

use self::model::PlayerModel;
use crate::{inventory::Inventory, AppState};

const GRAVITY: f32 = 20.0;

#[derive(Component, Default)]
pub struct Player {
    pub inventory: Inventory,
    pub model: PlayerModel,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_setup)
            .add_systems(Update, character_controller_update);
    }
}

fn player_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let controller = KinematicCharacterController::default();
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("player.png"),
            ..Default::default()
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(controller)
        .insert(Player::default())
        .insert(Camera2dBundle {
            ..Default::default()
        });
}

fn character_controller_update(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut KinematicCharacterController, &mut Transform)>,
    mut sprite_query: Query<&mut Sprite, With<Player>>,
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
    }
}
