use serde::Deserialize;

use super::item::{Item, ItemName};

#[derive(Clone, Deserialize)]
pub struct ItemStack {
    pub count: u8,
    #[serde(deserialize_with = "deserialize_item")]
    pub item: &'static dyn Item,
}

fn deserialize_item<'de, D>(deserializer: D) -> Result<&'static dyn Item, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let item_name = Deserialize::deserialize(deserializer)?;
    Ok(ItemName::into_static_item(item_name))
}
