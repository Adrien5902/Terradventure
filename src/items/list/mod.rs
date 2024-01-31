use self::sword::Sword;
use super::item::{Item, ItemName};

pub mod sword;

impl ItemName {
    pub fn into_static_item(&self) -> &'static dyn Item {
        &match self.get().as_str() {
            "sword" => Sword,
            _ => panic!("Unknown item name"),
        }
    }
}
