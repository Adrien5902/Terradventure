use crate::{assets::Asset, tiled::TiledMapBundle};
use bevy::prelude::*;
use std::path::{Path, PathBuf};

pub enum WorldType {
    Dungeon(&'static dyn Dungeon),
    World(&'static dyn Biome),
}

pub trait Biome {
    fn name(&self) -> &'static str;
    fn tile_set(&self) -> &'static str;
}

impl World for dyn Biome {
    fn tile_set_path(&self) -> TileMapAsset {
        TileMapAsset(Path::new("biomes").to_owned())
    }
}

pub trait World {
    fn tile_set_path(&self) -> TileMapAsset;

    fn spawn(&self, mut commands: Commands, asset_server: &Res<AssetServer>) {
        let tiled_map = asset_server.load("tiled/test.tmx");
        commands.spawn(TiledMapBundle {
            tiled_map,
            ..Default::default()
        });
    }
}

pub trait Dungeon {}

struct DesertBiome;
impl Biome for DesertBiome {
    fn name(&self) -> &'static str {
        "desert"
    }
    fn tile_set(&self) -> &'static str {
        "desert"
    }
}

pub struct ForestBiome;
impl Biome for ForestBiome {
    fn name(&self) -> &'static str {
        "forest"
    }
    fn tile_set(&self) -> &'static str {
        "forest"
    }
}
impl World for ForestBiome {
    fn tile_set_path(&self) -> TileMapAsset {
        TileMapAsset(Path::new("tiled/test.tmx").to_owned())
    }
}

pub struct TileMapAsset(PathBuf);

impl Asset for TileMapAsset {
    fn path(&self) -> PathBuf {
        Path::new("tile_sets").join(&self.0)
    }
}
