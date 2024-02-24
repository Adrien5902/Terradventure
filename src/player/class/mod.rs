use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use self::{
    archer::Archer, enchantress::Enchantress, knight::Knight, musketeer::Musketeer,
    swordsman::Swordsman, wizard::Wizard,
};

pub mod archer;
pub mod enchantress;
pub mod knight;
pub mod musketeer;
pub mod swordsman;
pub mod wizard;

#[enum_dispatch]
pub trait PlayerClass: Sync + Send {
    fn name(&self) -> &'static str;
}

#[derive(Serialize, Deserialize, Clone)]
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
