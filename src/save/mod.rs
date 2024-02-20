pub mod ui;

use crate::{mob::MobName, CONFIG_DIR};
use bevy::prelude::*;
use once_cell::sync::Lazy;
use std::path::PathBuf;

const SAVE_DIR: Lazy<PathBuf> = Lazy::new(|| CONFIG_DIR.join("saves"));
pub struct SavePlugin;
impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app;
    }
}

pub struct Save {
    pub mobs: Vec<(MobName, Vec2)>,
}
