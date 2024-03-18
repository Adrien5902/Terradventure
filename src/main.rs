pub mod animation;
pub mod background;
pub mod chest;
pub mod commands;
pub mod gui;
pub mod interactable;
pub mod items;
pub mod lang;
pub mod misc;
pub mod mob;
pub mod music;
pub mod player;
pub mod plugin;
pub mod random;
pub mod save;
pub mod state;
pub mod stats;
pub mod tiled;
pub mod world;

use bevy::prelude::*;
use bevy_rapier2d::render::RapierDebugRenderPlugin;
use once_cell::sync::Lazy;
use plugin::TerradventurePlugin;
use std::{env::args, path::PathBuf};

pub const GAME_NAME: &str = "Terradventure";

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
        .add_plugins(TerradventurePlugin)
        .add_plugins(RapierDebugRenderPlugin {
            enabled: args().collect::<Vec<_>>().contains(&String::from("debug")),
            ..Default::default()
        })
        .run();
}
