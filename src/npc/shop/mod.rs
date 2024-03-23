use self::ui::ShopUiPlugin;
use crate::items::stack::ItemStack;
use bevy::prelude::*;
use serde::Deserialize;
use std::fs;

pub mod ui;

pub struct ShopPlugin;
impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ShopUiPlugin).init_resource::<CurrentShop>();
    }
}

#[derive(Resource, Default)]
pub struct CurrentShop {
    pub shop: Option<Shop>,
}

#[derive(Deserialize)]
pub struct Shop {
    pub sells: Vec<ShopItem>,
    pub buys: Vec<ShopItem>,
}

impl Shop {
    pub fn read(name: &str) -> Option<Self> {
        fs::read(format!("assets/shop/{name}.json"))
            .ok()
            .and_then(|shop_data| {
                serde_json::from_slice(&shop_data)
                    .map_err(|e| error!("Failed to parse shop {e}"))
                    .ok()
            })
    }
}

#[derive(Deserialize)]
pub struct ShopItem {
    pub stack: ItemStack,
    pub price: u64,
}
