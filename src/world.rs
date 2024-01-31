use std::path::Path;

use crate::assets::Asset;

pub enum WorldType {
    Dungeon(&'static dyn Dungeon),
    World(&'static dyn Biome),
}

pub trait Biome {
    fn name(&self) -> &'static str;

    fn tile_set(&self) -> TileMapAsset;
}

pub trait Dungeon {}

struct DesertBiome;
impl Biome for DesertBiome {
    fn name(&self) -> &'static str {
        "desert"
    }

    fn tile_set(&self) -> TileMapAsset {
        "desert.tsx"
    }
}

pub struct TileMapAsset(&'static str);

impl Asset for TileMapAsset {
    fn path(&self) -> std::path::PathBuf {
        Path::new("tile_sets").join(self.0)
    }
}
