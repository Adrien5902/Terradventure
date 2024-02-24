use serde::{Deserialize, Serialize};

use super::PlayerClass;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Enchantress;

impl PlayerClass for Enchantress {
    fn name(&self) -> &'static str {
        "enchantress"
    }
}
