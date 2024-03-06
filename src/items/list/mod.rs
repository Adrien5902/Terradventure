use bevy::reflect::Reflect;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::player::inventory::SlotType;

use self::wool::Wool;
use super::item::{Item, ItemName, StackSize};

pub mod wool;

#[derive(Clone, Deserialize, Serialize, Reflect, PartialEq, Eq)]
#[enum_dispatch(Item)]
pub enum ItemObject {
    Wool(Wool),
}
