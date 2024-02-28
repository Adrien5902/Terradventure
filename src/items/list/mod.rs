use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use self::{sword::Sword, wool::Wool};
use super::item::{Item, ItemName, ItemTexture, StackSize};

pub mod sword;
pub mod wool;

#[derive(Clone, Deserialize, Serialize)]
#[enum_dispatch(Item)]
pub enum ItemObject {
    Sword(Sword),
    Wool(Wool),
}
