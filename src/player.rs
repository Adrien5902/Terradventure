use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::{control::KinematicCharacterController, geometry::Collider};

use crate::inventory::Inventory;

#[derive(Component)]
pub struct Player {
    pub inventory: Inventory,
    pub speed: f32,
    pub pos: Vec3,
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let player = Player {
        inventory: Inventory::default(),
        speed: 500.0,
        pos: Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0),
    };

    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_translation(player.pos),
                texture: asset_server.load("player.png"),
                ..default()
            },
            player,
        ))
        .insert(Collider::ball(30.0))
        .insert(KinematicCharacterController::default());
}
