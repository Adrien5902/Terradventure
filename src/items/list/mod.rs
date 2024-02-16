use self::{sword::Sword, wool::Wool};
use super::item::{Item, ItemName};

pub mod sword;
pub mod wool;

impl ItemName {
    pub fn into_static_item(name: &str) -> &'static dyn Item {
        match name {
            "sword" => &Sword,
            "wool" => &Wool,
            _ => panic!("Unknown item name"),
        }
    }
}
