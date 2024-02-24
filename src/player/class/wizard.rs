use serde::{Deserialize, Serialize};

use super::PlayerClass;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Wizard;

impl PlayerClass for Wizard {
    fn name(&self) -> &'static str {
        "wizard"
    }
}
