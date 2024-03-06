use std::{fs, path::Path};

use crate::{items::stack::ItemStack, random::RandomWeightedTable};

pub type LootTable = RandomWeightedTable<ItemStack>;

impl LootTable {
    pub fn read(path: &Path) -> Result<LootTable, String> {
        let data =
            fs::read(Path::new("assets/loot_tables").join(path)).map_err(|e| e.to_string())?;

        serde_json::from_slice(&data).map_err(|e| e.to_string())
    }
}
