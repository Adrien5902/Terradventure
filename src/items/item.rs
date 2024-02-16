use std::num::NonZeroU8;

use serde::{Deserialize, Serialize};

use crate::assets::TextureAsset;

pub type StackSize = NonZeroU8;

pub trait Item: Sync {
    fn name(&self) -> ItemName;
    fn texture(&self) -> ItemTexture {
        ItemTexture::from(self.name())
    }

    fn stack_size(&self) -> StackSize {
        StackSize::MAX
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

impl From<ItemName> for ItemTexture {
    fn from(value: ItemName) -> Self {
        Self::from(value.get())
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
