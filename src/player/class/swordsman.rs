use serde::{Deserialize, Serialize};

use super::PlayerClass;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Swordsman;

impl PlayerClass for Swordsman {
    fn name(&self) -> &'static str {
        "swordsman"
    }
}
