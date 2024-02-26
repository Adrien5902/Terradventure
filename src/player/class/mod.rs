use std::{fs, path::Path};

use bevy::{asset::Handle, render::texture::Image};
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumCount, EnumIter};

use crate::misc::read_img;

use self::{
    archer::Archer, enchantress::Enchantress, knight::Knight, musketeer::Musketeer,
    swordsman::Swordsman, wizard::Wizard,
};

use super::{PLAYER_SPRITE_SHEETS_X_SIZE, PLAYER_TEXTURE};

pub mod archer;
pub mod enchantress;
pub mod knight;
pub mod musketeer;
pub mod swordsman;
pub mod wizard;

#[enum_dispatch]
pub trait PlayerClass: Sync + Send {
    fn name(&self) -> &'static str;
    fn idle_texture(&self) -> Image {
        let path = Path::new(PLAYER_TEXTURE).join(self.name()).join("Idle.png");
        let mut img = read_img(path);
        img.crop(
            0,
            0,
            PLAYER_SPRITE_SHEETS_X_SIZE,
            PLAYER_SPRITE_SHEETS_X_SIZE,
        );
        let bevy_img = Image::from_dynamic(img, true);
        bevy_img
    }
}

#[derive(Serialize, Deserialize, Clone, EnumIter, EnumCount)]
#[enum_dispatch(PlayerClass)]
pub enum PlayerClasses {
    Archer(Archer),
    Enchantress(Enchantress),
    Knight(Knight),
    Musketeer(Musketeer),
    Swordsman(Swordsman),
    Wizard(Wizard),
}

impl Default for PlayerClasses {
    fn default() -> Self {
        Self::Swordsman(Swordsman::default())
    }
}
