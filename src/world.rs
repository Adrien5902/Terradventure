use crate::tiled::TiledMapBundle;
use bevy::{asset::AssetPath, prelude::*};
use std::path::{Path, PathBuf};

#[derive(Resource)]
pub struct CurrentWorld(&'static dyn World);

pub enum WorldType {
    Biome,
    Dungeon,
}

pub trait World: Sync {
    fn name(&self) -> &'static str;
    fn world_type(&self) -> WorldType;

    fn tile_set_path(&self) -> TileMapAsset {
        TileMapAsset(
            Path::new(match self.world_type() {
                WorldType::Biome => "biome",
                WorldType::Dungeon => "dungeon",
            })
            .join(self.name()),
        )
    }

    fn spawn(&self, mut commands: Commands, asset_server: &Res<AssetServer>) {
        let tiled_map = asset_server.load(self.tile_set_path());
        commands.spawn(TiledMapBundle {
            tiled_map,
            ..Default::default()
        });
    }
}

struct DesertBiome;
impl World for DesertBiome {
    fn name(&self) -> &'static str {
        "desert"
    }
    fn world_type(&self) -> WorldType {
        WorldType::Biome
    }
}

pub struct ForestBiome;
impl World for ForestBiome {
    fn name(&self) -> &'static str {
        "forest"
    }
    fn world_type(&self) -> WorldType {
        WorldType::Biome
    }
}

pub struct PlainsBiome;
impl World for PlainsBiome {
    fn name(&self) -> &'static str {
        "plains"
    }
    fn world_type(&self) -> WorldType {
        WorldType::Biome
    }
}

pub struct TileMapAsset(PathBuf);

impl<'a> Into<AssetPath<'a>> for TileMapAsset {
    fn into(self) -> AssetPath<'a> {
        Path::new("tiled")
            .join(format!("{}.tmx", self.0.to_str().unwrap()))
            .into()
    }
}
