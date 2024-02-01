use bevy::asset::AssetPath;
use std::path::{Path, PathBuf};

pub trait Asset {
    fn path(&self) -> PathBuf;
}

pub trait TextureAsset {
    fn name(&self) -> String;
}

impl Asset for dyn TextureAsset {
    fn path(&self) -> PathBuf {
        Path::new("textures").join(self.name())
    }
}

impl<'a> Into<AssetPath<'a>> for &'static dyn Asset {
    fn into(self) -> AssetPath<'a> {
        let path = self.path().clone();
        AssetPath::from(path)
    }
}
