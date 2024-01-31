use serde::{Deserialize, Serialize};

use crate::assets::TextureAsset;

pub type StackSize = u8;
const MAX_STACK_SIZE: StackSize = StackSize::MAX;

pub trait Item: Sync {
    fn texture(&self) -> ItemTexture;
    fn name(&self) -> ItemName;
    fn stack_size(&self) -> StackSize {
        MAX_STACK_SIZE
    }
    // fn get_use(&self) -> Option<fn() -> ()> {
    //     None
    // }
}

pub struct ItemTexture(&'static str);

impl From<&'static str> for ItemTexture {
    fn from(value: &'static str) -> Self {
        Self(value)
    }
}

impl TextureAsset for ItemTexture {
    fn name(&self) -> String {
        format!("item/{}", self.0)
    }
}

#[derive(Deserialize, Serialize)]
pub struct ItemName(&'static str);

impl From<&'static str> for ItemName {
    fn from(value: &'static str) -> Self {
        Self(value)
    }
}

impl ItemName {
    pub fn get(&self) -> &'static str {
        self.0
    }
}
