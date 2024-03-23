use crate::{items::stack::ItemStack, random::RandomWeightedTable};
use bevy::prelude::*;
use rand::{seq::IteratorRandom, thread_rng};
use serde::Deserialize;
use std::{fs, ops::Range, path::Path};

#[derive(Deserialize)]
pub struct LootTable {
    #[serde(default)]
    pub money: Range<u64>,
    #[serde(default)]
    pub items: RandomWeightedTable<ItemStack>,
}

impl LootTable {
    pub fn read(path: &Path) -> Option<LootTable> {
        let data = fs::read(Path::new("assets/loot_tables").join(path)).unwrap();
        serde_json::from_slice(&data)
            .map_err(|e| error!("Failed to parse loot table {e}"))
            .ok()
    }

    /// # Returns the amount of earned money and the looted items
    pub fn get_random(&self) -> (u64, Vec<ItemStack>) {
        (
            self.money.clone().choose(&mut thread_rng()).unwrap_or(0),
            self.items.get_random(),
        )
    }
}
