use std::fs;

use crate::items::stack::ItemStack;
use rand::random;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LootTable {
    loots: Vec<Loot>,
    rolls: u32,
}

#[derive(Deserialize)]
struct Loot {
    itemstack: ItemStack,
    weight: u32,
}

impl LootTable {
    fn get_random_loots(&self) -> Vec<ItemStack> {
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

    pub fn load(ident: &str) -> Self {
        let data = fs::read(format!("loot_tables/{}.json", ident)).unwrap();
        serde_json::from_slice(&data).unwrap()
    }
}
