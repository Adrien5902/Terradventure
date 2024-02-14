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

impl<'a> From<&'static dyn Asset> for AssetPath<'a> {
    fn from(val: &'static dyn Asset) -> Self {
        let path = val.path().clone();
        AssetPath::from(path)
    }
}
