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

#[derive(Default, Component, Deserialize, Serialize, Clone)]
pub struct Inventory {
    pub ressources: [Slot; 27],
    pub armor: ArmorSlots,
    pub pockets: LRSlots,
    pub back: LRSlots,
    pub accessories: AccessoriesSlots,
}

#[derive(Default, Deserialize, Serialize, Clone)]
pub struct Slot {
    pub item: Option<ItemStack>,
}

#[derive(Default, Deserialize, Serialize, Clone)]
pub struct ArmorSlots {
    pub head: Slot,
    pub torso: Slot,
    pub legs: Slot,
    pub feet: Slot,
}

#[derive(Default, Component, Deserialize, Serialize, Clone)]
pub struct LRSlots {
    pub left: Slot,
    pub right: Slot,
}

#[derive(Default, Component, Deserialize, Serialize, Clone)]
pub struct AccessoriesSlots {
    pub neck: Slot,
    pub bracelet: Slot,
}
