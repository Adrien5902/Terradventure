use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::items::item::Item;

#[derive(Clone, Deserialize, Serialize, Reflect, PartialEq, Eq, Default)]
pub struct Porkchop;

impl Item for Porkchop {
    fn use_item(&self) -> bool {
        true
    }
}
