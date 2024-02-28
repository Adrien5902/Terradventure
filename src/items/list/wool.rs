use serde::{Deserialize, Serialize};

use crate::items::item::Item;

#[derive(Clone, Deserialize, Serialize)]
pub struct Wool;

impl Item for Wool {
    fn name(&self) -> crate::items::item::ItemName {
        "wool".into()
    }
}
