pub mod assets;
pub mod entity;
pub mod gui;
pub mod items;
pub mod mob;
pub mod player;
pub mod settings;
pub mod state;
pub mod stats;
pub mod tiled;
pub mod world;

use bevy::prelude::*;
use bevy_rapier2d::{
    geometry::Collider,
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use gui::GuiPlugin;
use player::PlayerPlugin;
use settings::SettingsPlugin;
use state::AppStatePlugin;

pub const GAME_NAME: &'static str = "Terradventure";

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
        .add_systems(Startup, spawn_block)
        .add_systems(Startup, spawn_block)
        .add_plugins((SettingsPlugin, PlayerPlugin, AppStatePlugin, GuiPlugin))
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
