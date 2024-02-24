pub mod ui;

use crate::{
    mob::{MobBundle, MobObject, MobTrait},
    player::Player,
    CONFIG_DIR,
};
use bevy::prelude::*;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const SAVE_DIR: Lazy<PathBuf> = Lazy::new(|| CONFIG_DIR.join("saves"));
pub struct SavePlugin;
impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app;
    }
}

#[derive(Serialize, Deserialize)]
pub struct MobSave {
    data: MobObject,
    pos: Vec2,
}

impl MobSave {
    pub fn into_bundle(&self, asset_server: &Res<AssetServer>) -> MobBundle {
        self.data.into_bundle(asset_server, self.pos)
    }
}

#[derive(Serialize, Deserialize)]
pub struct PlayerSave {
    pub player: Player,
    pub pos: Vec2,
}

#[derive(Serialize, Deserialize)]
pub struct Save {
    pub player: PlayerSave,
    pub mobs: Vec<MobSave>,
}

impl Save {
    pub fn load(&self, mut commands: Commands, asset_server: &Res<AssetServer>) {
        self.mobs.iter().for_each(|mob| {
            commands.spawn(mob.into_bundle(asset_server));
        });
    }
}
