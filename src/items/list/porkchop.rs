use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::items::item::ItemTrait;

#[derive(Clone, Deserialize, Serialize, Reflect, PartialEq, Eq, Default)]
pub struct Porkchop;

impl ItemTrait for Porkchop {
    fn use_item(&self) -> bool {
        true
    }
}
