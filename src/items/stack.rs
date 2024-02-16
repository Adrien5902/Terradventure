use serde::Deserialize;

use super::item::{deserialize_item, Item};

#[derive(Clone, Deserialize)]
pub struct ItemStack {
    pub count: u8,
    #[serde(deserialize_with = "deserialize_item")]
    pub item: &'static dyn Item,
}
