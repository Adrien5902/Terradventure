pub mod assets;
pub mod entity;
pub mod inventory;
pub mod items;
pub mod loot_table;
pub mod mob;
pub mod player;
pub mod settings;
pub mod stats;
pub mod world;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::{
    geometry::Collider,
    plugin::{NoUserData, RapierPhysicsPlugin},
};
use player::PlayerPlugin;
use settings::{Settings, SettingsPlugin};

pub const GAME_NAME: &'static str = "Terradventure";

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
        .add_state::<Settings>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from(GAME_NAME),
                ..Default::default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, spawn_block)
        .add_plugins(SettingsPlugin)
        .add_plugins(TilemapPlugin)
        .run();
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
