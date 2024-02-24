use serde::{Deserialize, Serialize};

use super::PlayerClass;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Knight;

impl PlayerClass for Knight {
    fn name(&self) -> &'static str {
        "knight"
    }
}
