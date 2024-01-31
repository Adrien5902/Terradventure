pub mod assets;
pub mod entity;
pub mod inventory;
pub mod items;
pub mod loot_table;
pub mod main_menu;
pub mod mob;
pub mod player;
pub mod settings;
pub mod stats;
pub mod world;

use bevy::prelude::*;
use bevy_rapier2d::{
    geometry::Collider,
    plugin::{NoUserData, RapierPhysicsPlugin},
};
// use main_menu::MainMenuPlugin;
use player::PlayerPlugin;
use settings::SettingsPlugin;

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
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from(GAME_NAME),
                ..Default::default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_systems(Startup, spawn_block)
        .add_systems(Startup, spawn_block)
        .add_plugins((SettingsPlugin, PlayerPlugin))
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
