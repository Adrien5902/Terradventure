#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod animation;
pub mod assets;
pub mod gui;
pub mod items;
pub mod lang;
pub mod misc;
pub mod mob;
pub mod player;
pub mod save;
pub mod state;
pub mod stats;
pub mod tiled;
pub mod world;

use animation::AnimationPlugin;
use bevy::prelude::*;
use bevy_rapier2d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use gui::{settings::SettingsPlugin, GuiPlugin};
use items::loot_table::LootTablePlugin;
use mob::MobPlugin;
use once_cell::sync::Lazy;
use player::PlayerPlugin;
use save::SavePlugin;
use state::AppStatePlugin;
use world::WorldPlugin;

pub const GAME_NAME: &str = "Terradventure";

use std::path::PathBuf;
static CONFIG_DIR: Lazy<PathBuf> = Lazy::new(|| {
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
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins((
            SettingsPlugin,
            PlayerPlugin,
            AppStatePlugin,
            GuiPlugin,
            LootTablePlugin,
            WorldPlugin,
            MobPlugin,
            AnimationPlugin,
            SavePlugin,
        ))
        .run();
}
