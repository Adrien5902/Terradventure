use std::{fs, ops::Range, path::Path};

use rand::{seq::IteratorRandom, thread_rng};
use serde::Deserialize;

use crate::{items::stack::ItemStack, random::RandomWeightedTable};

#[derive(Deserialize)]
pub struct LootTable {
    #[serde(default)]
    pub money: Range<u64>,
    #[serde(default)]
    pub items: RandomWeightedTable<ItemStack>,
}

impl LootTable {
    pub fn read(path: &Path) -> Result<LootTable, String> {
        let data =
            fs::read(Path::new("assets/loot_tables").join(path)).map_err(|e| e.to_string())?;

        serde_json::from_slice(&data).map_err(|e| e.to_string())
    }

    /// # Returns the amount of earned money and the looted items
    pub fn get_random(&self) -> (u64, Vec<ItemStack>) {
        (
            self.money.clone().choose(&mut thread_rng()).unwrap_or(0),
            self.items.get_random(),
        )
    }
}
