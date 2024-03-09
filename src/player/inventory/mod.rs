pub mod ui;

use std::cmp::Ordering;

use crate::items::{item::StackSize, list::ItemObject, stack::ItemStack};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use self::ui::{InventorySlot, InventoryUiPlugin, UpdateSlotEvent};

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

    /// to check if all the stack was consumed use [`optional_item_stack.is_none()`]
    pub fn push_item_stack(
        &mut self,
        optional_item_stack: &mut Option<ItemStack>,
        update_slot_event: &mut EventWriter<UpdateSlotEvent>,
    ) {
        let mut slots = [
            self.pockets
                .iter_mut()
                .enumerate()
                .map(|(i, s)| (i, "pockets", s))
                .collect::<Vec<_>>(),
            self.ressources
                .iter_mut()
                .enumerate()
                .map(|(i, s)| (i, "ressources", s))
                .collect::<Vec<_>>(),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

        if let Some(item_stack) = optional_item_stack {
            slots.sort_by(|(_, _, a), (_, _, b)| {
                let a = a.item_is(&item_stack.item);
                let b = b.item_is(&item_stack.item);

                if a == b {
                    Ordering::Equal
                } else if a {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
        }

        for (i, field, slot) in slots {
            if optional_item_stack.is_none() {
                return;
            }

            slot.push_item_stack(optional_item_stack);
            update_slot_event.send(UpdateSlotEvent {
                slot: InventorySlot {
                    typ: field.into(),
                    slot_index: i,
                },
                new_item: slot.item.clone(),
            });
        }
    }
}

bitflags::bitflags! {
    pub struct SlotType: u8 {
        const Ressources = 0b00000001;
        const Pockets = 0b00000100;
        const Accessories = 0b00001000;
        const Helmet = 0b10000000;
        const ChestPlate = 0b01000000;
        const Leggings = 0b00100000;
        const Boots = 0b00010000;
        const Armor = 0b11110000;
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

impl From<&InventorySlot> for SlotType {
    fn from(value: &InventorySlot) -> Self {
        let field: SlotType = value.typ.as_str().into();
        if field.contains(Self::Armor) {
            field
                & match value.slot_index {
                    0 => Self::Helmet,
                    1 => Self::ChestPlate,
                    2 => Self::Leggings,
                    3 => Self::Boots,
                    _ => panic!(),
                }
        } else {
            field
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

impl Slot {
    /// to check if all the stack was consumed use [`optional_item_stack.is_none()`]
    /// # Returns
    /// true if the items were the same kind
    pub fn push_item_stack(&mut self, optional_item_stack: &mut Option<ItemStack>) -> bool {
        if let Some(slot_item_stack) = &mut self.item {
            // Slot contains item

            //Remaining space in slot's itemstack
            let remaining_space = StackSize::MAX - slot_item_stack.count;

            if let Some(item_stack) = optional_item_stack {
                // Given stack isn't empty
                let same_items = slot_item_stack.item == item_stack.item;

                if same_items {
                    if item_stack.count > remaining_space {
                        slot_item_stack.count = StackSize::MAX;
                        item_stack.count -= remaining_space;
                    } else {
                        let new_count = slot_item_stack.count as u16 + item_stack.actual_count();
                        slot_item_stack.count = new_count as u8;
                        *optional_item_stack = None;
                    }
                }

                same_items
            } else {
                // Given item stack is empty
                false
            }
        } else {
            // Slot is empty
            self.item = optional_item_stack.take();
            optional_item_stack.is_none() //Both are none
        }
    }

    pub fn item_is(&self, item: &ItemObject) -> bool {
        self.item.as_ref().is_some_and(|stack| stack.item == *item)
    }
}
