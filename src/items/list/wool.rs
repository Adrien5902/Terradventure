use bevy::reflect::Reflect;
use serde::{Deserialize, Serialize};

use crate::items::item::Item;

#[derive(Clone, Deserialize, Serialize, Reflect, PartialEq, Eq)]
pub struct Wool;

impl Item for Wool {}
