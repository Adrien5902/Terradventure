pub mod ui;

use crate::items::{item::StackSize, stack::ItemStack};
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

    /// # Returns
    /// true if there's any item left false otherwise
    pub fn push_item_stack(&mut self, item_stack: &mut ItemStack) -> bool {
        let same_slot = self.ressources.iter_mut().filter_map(|slot| {
            slot.item.as_mut().and_then(|stack| {
                (stack.count < StackSize::MAX && stack.item == item_stack.item).then(|| stack)
            })
        });

        for slot_item_stack in same_slot {
            let remaining_space = StackSize::MAX - slot_item_stack.count;

            if item_stack.count > remaining_space {
                slot_item_stack.count = StackSize::MAX;
                item_stack.count -= remaining_space;
            } else {
                let new_count = slot_item_stack.count as u16 + item_stack.actual_count();
                slot_item_stack.count = new_count as u8;
                return false;
            }
        }

        let found_slot = self.ressources.iter_mut().find(|slot| slot.item.is_none());

        if let Some(slot) = found_slot {
            slot.item = Some(item_stack.clone());
            false
        } else {
            true
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
