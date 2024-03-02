use std::{fs, path::Path};

use crate::items::stack::ItemStack;
use bevy::prelude::*;
use rand::random;
use serde::Deserialize;

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

        let total_weight: u32 = self.loots.iter().map(|loot| loot.weight).sum();
        for _ in 0..self.rolls {
            let mut r = (random::<f32>() * total_weight as f32).floor() as u32;
            for loot in self.loots.iter() {
                if r < loot.weight {
                    res.push(loot.itemstack.clone());
                    break;
                }

                r -= loot.weight
            }
        }

        res
    }

    pub fn read(path: &Path) -> Self {
        let loot_table = (|| {
            let data =
                fs::read(Path::new("assets/loot_tables").join(path)).map_err(|e| e.to_string())?;
            serde_json::from_slice(&data).map_err(|e| e.to_string())
        })();

        loot_table.unwrap()
    }
}
