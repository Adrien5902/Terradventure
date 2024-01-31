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
