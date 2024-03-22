use bevy::prelude::*;

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

pub struct Shop {
    pub solds: Vec<ShopItem>,
    pub buys: Vec<ShopItem>,
}

pub struct ShopItem {
    pub stack: ItemStack,
    pub price: u64,
}
