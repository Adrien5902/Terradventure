pub mod ui;

use crate::items::stack::ItemStack;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use self::ui::InventoryUiPlugin;

pub struct InventoryPlugin;
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InventoryUiPlugin);
    }
}

#[derive(Default, Component, Deserialize, Serialize, Clone, Reflect)]
pub struct Inventory {
    pub ressources: [Slot; Self::RESSOURCE_COUNT],
    pub armor: [Slot; Self::ARMOR_COUNT],
    pub pockets: [Slot; Self::POCKETS_COUNT],
    pub accessories: [Slot; Self::ACCESSORIES_COUNT],
}

impl Inventory {
    pub const RESSOURCE_COLUMNS: usize = 9;
    pub const RESSOURCES_ROWS: usize = 3;

    pub const RESSOURCE_COUNT: usize = 27;
    pub const ARMOR_COUNT: usize = 4;
    pub const POCKETS_COUNT: usize = 2;
    pub const ACCESSORIES_COUNT: usize = 2;

    fn get_slot_mut<'a>(&'a mut self, field: &str, index: usize) -> &'a mut Slot {
        match field {
            "accessories" => &mut self.accessories[index],
            "armor" => &mut self.armor[index],
            "pockets" => &mut self.pockets[index],
            "ressources" => &mut self.ressources[index],
            _ => panic!(),
        }
    }
}

bitflags::bitflags! {
    pub struct SlotType: u8 {
        const Ressources = 0b0001;
        const Armor = 0b0010;
        const Pockets = 0b0100;
        const Accessories = 0b1000;
    }
}

impl From<&str> for SlotType {
    fn from(value: &str) -> Self {
        match value {
            "accessories" => Self::Accessories,
            "armor" => Self::Armor,
            "pockets" => Self::Pockets,
            "ressources" => Self::Ressources,
            _ => panic!(),
        }
    }
}

impl Default for SlotType {
    fn default() -> Self {
        Self::Pockets | Self::Ressources
    }
}

#[derive(Default, Deserialize, Serialize, Clone, Reflect)]
pub struct Slot {
    pub item: Option<ItemStack>,
}
