use crate::items::stack::ItemStack;
use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use rand::random;
use serde::Deserialize;

struct LootTablePlugin;
impl Plugin for LootTablePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(JsonAssetPlugin::<LootTable>::new(&["loot_table"]));
    }
}

#[derive(Deserialize, TypePath, Asset)]
pub struct LootTable {
    pub loots: Vec<Loot>,
    pub rolls: u32,
}

#[derive(Deserialize)]
pub struct Loot {
    pub itemstack: ItemStack,
    pub weight: u32,
}

impl LootTable {
    pub fn get_random_loots(&self) -> Vec<ItemStack> {
        let mut res = Vec::new();

        for _ in 0..self.rolls {
            let total_weight: u32 = self.loots.iter().map(|loot| loot.weight).sum();
            let mut r = random::<u32>() * total_weight;
            for loot in self.loots.iter() {
                if r < loot.weight {
                    res.push(loot.itemstack.clone());
                    break;
                } else {
                    r -= loot.weight
                }
            }
        }

        res
    }
}
