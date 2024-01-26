use std::{fs, path::Path};

use crate::items::stack::ItemStack;
use rand::random;

pub struct LootTable {
    loots: Vec<Loot>,
    rolls: u32,
}

struct Loot {
    itemstack: ItemStack,
    weight: u32,
}

impl LootTable {
    fn get_random_loots(&self) -> ItemStack {
        let mut res = Vec::new();

        for _ in self.rolls {
            let total_weight = self.loots.iter().map(|loot| loot.weight).sum();
            let mut r = random() * total_weight;
            for loot in self.loots.iter() {
                if r < loot.weight {
                    res.push(loot.itemstack);
                    break;
                } else {
                    r -= loot.weight
                }
            }
        }

        res
    }

    pub fn from_file(path: &Path) -> Result<Self, String> {
        let slice = fs::read(path)?;
    }
}
