use serde::{Deserialize, Serialize};

use super::PlayerClass;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Musketeer;

impl PlayerClass for Musketeer {
    fn name(&self) -> &'static str {
        "musketeer"
    }
}
