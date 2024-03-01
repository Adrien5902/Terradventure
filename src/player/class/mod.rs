use std::path::Path;

use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierContext;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumCount, EnumIter};

use crate::{lang::Lang, misc::read_img, mob::Mob, stats::Stats};

use self::{
    archer::Archer, enchantress::Enchantress, knight::Knight, musketeer::Musketeer,
    swordsman::Swordsman, wizard::Wizard,
};

use super::{ChainAttack, Player, PLAYER_SPRITE_SHEETS_X_SIZE, PLAYER_TEXTURE};

pub mod archer;
pub mod enchantress;
pub mod knight;
pub mod musketeer;
pub mod swordsman;
pub mod wizard;

#[enum_dispatch]
pub trait PlayerClass: Sync + Send {
    fn name(&self) -> &'static str;
    fn translated_name(&self, lang: &Res<Lang>) -> String {
        lang.get(&format!("player.class.{}", self.name()))
            .to_owned()
    }
    fn idle_texture(&self) -> Image {
        let path = Path::new(PLAYER_TEXTURE).join(self.name()).join("Idle.png");
        let img = read_img(path).crop(
            0,
            0,
            PLAYER_SPRITE_SHEETS_X_SIZE,
            PLAYER_SPRITE_SHEETS_X_SIZE,
        );
        Image::from_dynamic(img, true)
    }

    fn normal_attack_chain_count(&self) -> u8 {
        ChainAttack::DEFAULT
    }

    fn special_attack_1(
        &self,
        player: Entity,
        rapier_context: &Res<RapierContext>,
        transform: &Transform,
        flipped: bool,
        mob_query: &mut Query<(&mut Stats, &mut Mob), Without<Player>>,
    );

    fn special_attack_2(
        &self,
        player: Entity,
        rapier_context: &Res<RapierContext>,
        transform: &Transform,
        flipped: bool,
        mob_query: &mut Query<(&mut Stats, &mut Mob), Without<Player>>,
    );

    fn special_attack_3(
        &self,
        player: Entity,
        rapier_context: &Res<RapierContext>,
        transform: &Transform,
        flipped: bool,
        mob_query: &mut Query<(&mut Stats, &mut Mob), Without<Player>>,
    );
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
