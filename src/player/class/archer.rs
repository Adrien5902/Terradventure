use serde::{Deserialize, Serialize};

use super::PlayerClass;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Archer;

impl PlayerClass for Archer {
    fn name(&self) -> &'static str {
        "archer"
    }
}
