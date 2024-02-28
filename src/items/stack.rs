use serde::{Deserialize, Serialize};

use super::list::ItemObject;

#[derive(Clone, Deserialize, Serialize)]
pub struct ItemStack {
    pub count: u8,
    pub item: ItemObject,
}
