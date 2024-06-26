use crate::{
    items::item::{ItemName, ItemTrait},
    ore::{Ore, OreType},
    player::inventory::SlotType,
};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Reflect, PartialEq, Eq, Default)]
pub struct ProcessedOre(pub Ore);

impl ItemTrait for ProcessedOre {
    fn can_put_in(&self) -> SlotType {
        SlotType::Ressources
    }

    fn name(&self) -> ItemName {
        let mut ore_name = self.0.to_string();

        if self.0.ore_type() == OreType::Ingot {
            ore_name += "Ingot";
        }

        ore_name.into()
    }
}
