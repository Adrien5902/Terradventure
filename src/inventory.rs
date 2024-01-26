use crate::items::item::Item;

#[derive(Default)]
pub struct Inventory {
    pub resources: [Slot; 27],
    pub armor: [Slot; 4],
    pub pockets: [Slot; 2],
    pub back: [Slot; 2],
    pub accessories: [Slot; 4],
}

#[derive(Default)]
pub struct Slot {
    pub item: Option<ItemStack>,
}
