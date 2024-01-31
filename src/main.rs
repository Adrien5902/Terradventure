pub mod inventory;
pub mod items;
pub mod loot_table;
pub mod mob;
pub mod player;
pub mod stats;
pub mod world;

use bevy::{prelude::*, transform::components::Transform};
use bevy_rapier2d::{
    geometry::Collider,
    plugin::{NoUserData, RapierPhysicsPlugin},
};
use player::{spawn_player, Player};

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    Paused,
}

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_systems(PreStartup, spawn_player)
        .add_systems(Startup, (spawn_block, spawn_camera))
        .add_systems(Update, player_movement)
        .run();
}

pub fn spawn_camera(mut commands: Commands, player_query: Query<&Player>) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(player_query.single().pos),
        ..Default::default()
    });
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut transform_query: Query<&mut Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera2d>)>,
    mut player_query: Query<&mut Player>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = transform_query.get_single_mut() {
        if let Ok(mut player) = player_query.get_single_mut() {
            if let Ok(mut camera) = camera_query.get_single_mut() {
                let mut direction = Vec3::ZERO;

                if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::Q) {
                    direction += Vec3::new(-1.0, 0.0, 0.0);
                }
                if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
                    direction += Vec3::new(1.0, 0.0, 0.0);
                }
                if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::Z) {
                    direction += Vec3::new(0.0, 1.0, 0.0);
                }
                if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
                    direction += Vec3::new(0.0, -1.0, 0.0);
                }

                if direction.length() > 0.0 {
                    direction = direction.normalize();
                }

                let speed = player.speed;

                player.pos += direction * speed * time.delta_seconds();
                transform.translation = player.pos;
                camera.translation = player.pos;
            }
        }
    }
}

pub fn spawn_block(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("block.png"),
            ..Default::default()
        },
        Collider::cuboid(20.0, 20.0),
    ));
}
