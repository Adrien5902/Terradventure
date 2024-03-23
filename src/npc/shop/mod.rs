use bevy::prelude::*;
use serde::Deserialize;

use crate::items::stack::ItemStack;

use self::ui::ShopUiPlugin;

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

#[derive(Deserialize)]
pub struct ShopItem {
    pub stack: ItemStack,
    pub price: u64,
}
