use bevy::app::Plugin;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};

use crate::{
    animation::AnimationPlugin, background::ParallaxBackgroundPlugin, chest::ChestPlugin,
    commands::CommandsPlugin, gui::GuiPlugin, interactable::InteractionPlugin, mob::MobPlugin,
    music::MusicPlugin, npc::NpcPlugin, ore::OrePlugin, player::PlayerPlugin, save::SavePlugin,
    state::AppStatePlugin, stats::StatsPlugin, tiled, world::WorldPlugin,
};

pub struct TerradventurePlugin;
impl Plugin for TerradventurePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugins((bevy_ecs_tilemap::TilemapPlugin, tiled::TiledMapPlugin))
            .add_plugins((
                PlayerPlugin,
                AppStatePlugin,
                GuiPlugin,
                WorldPlugin,
                MobPlugin,
                AnimationPlugin,
                SavePlugin,
                StatsPlugin,
                InteractionPlugin,
                MusicPlugin,
                CommandsPlugin,
                ChestPlugin,
                ParallaxBackgroundPlugin,
                NpcPlugin,
                OrePlugin,
            ));
    }
}
