use crate::{
    items::item::{Item, ItemName},
    ore::{Ore, OreType},
    player::inventory::SlotType,
};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Reflect, PartialEq, Eq, Default)]
pub struct UnprocessedOre(pub Ore);

impl Item for UnprocessedOre {
    fn can_put_in(&self) -> SlotType {
        SlotType::Ressources
    }

    fn name(&self) -> ItemName {
        let mut ore_name = self.0.to_string();

        if self.0.ore_type() == OreType::Gem {
            ore_name += "Gem";
        }

        ore_name.into()
    }
}
