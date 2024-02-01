use bevy::prelude::*;

use crate::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, update_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        ..Default::default()
    });
}

fn update_camera(
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    transform_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
) {
    if let Ok(mut camera) = camera_query.get_single_mut() {
        if let Ok(player_pos) = transform_query.get_single() {
            camera.translation = player_pos.translation;
        }
    }
}
