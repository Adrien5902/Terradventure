use self::sword::Sword;
use super::item::{Item, ItemName};

pub mod sword;

impl ItemName {
    pub fn into_static_item(name: &str) -> &'static dyn Item {
        &match name {
            "sword" => Sword,
            _ => panic!("Unknown item name"),
        }
    }
}
