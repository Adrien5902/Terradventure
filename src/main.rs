#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod assets;
pub mod entity;
pub mod gui;
pub mod items;
pub mod mob;
pub mod player;
pub mod state;
pub mod stats;
pub mod tiled;
pub mod world;

use bevy::prelude::*;
use bevy_rapier2d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use gui::{settings::SettingsPlugin, GuiPlugin};
use once_cell::sync::Lazy;
use player::PlayerPlugin;
use state::AppStatePlugin;

pub const GAME_NAME: &'static str = "Terradventure";

use dirs;
use std::path::PathBuf;
const CONFIG_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let dir = dirs::config_dir().unwrap();
    dir.join(GAME_NAME)
});

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from(GAME_NAME),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins((bevy_ecs_tilemap::TilemapPlugin, tiled::TiledMapPlugin))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins((SettingsPlugin, PlayerPlugin, AppStatePlugin, GuiPlugin))
        .run();
}
