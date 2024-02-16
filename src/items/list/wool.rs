use crate::items::item::Item;

pub struct Wool;

impl Item for Wool {
    fn name(&self) -> crate::items::item::ItemName {
        "wool".into()
    }
}
