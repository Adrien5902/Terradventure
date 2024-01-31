use crate::items::stack::ItemStack;

#[derive(Default)]
pub struct Inventory {
    pub ressources: [Slot; 27],
    pub armor: ArmorSlots,
    pub pockets: LRSlots,
    pub back: LRSlots,
    pub accessories: AccessoriesSlots,
}

#[derive(Default)]
pub struct Slot {
    pub item: Option<ItemStack>,
}

#[derive(Default)]
pub struct ArmorSlots {
    pub head: Slot,
    pub torso: Slot,
    pub legs: Slot,
    pub feet: Slot,
}

#[derive(Default)]
pub struct LRSlots {
    pub left: Slot,
    pub right: Slot,
}

#[derive(Default)]
pub struct AccessoriesSlots {
    pub neck: Slot,
    pub bracelet: Slot,
}
