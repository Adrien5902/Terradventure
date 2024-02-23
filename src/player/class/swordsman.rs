use super::PlayerClass;

pub struct Swordsman;

impl PlayerClass for Swordsman {
    fn name(&self) -> &'static str {
        "swordsman"
    }
}
