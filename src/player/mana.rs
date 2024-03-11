use std::ops::{AddAssign, SubAssign};

use bevy::time::Time;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Mana {
    value: f32,
    regen_rate: f32,
}

impl Mana {
    pub const MAX: f32 = 100.;

    pub fn get(&self) -> f32 {
        self.value
    }

    /// # Returns
    /// [`true`] if removing was successful
    pub fn try_remove(&mut self, amount: f32) -> bool {
        let can_remove = self.value >= amount;
        if can_remove {
            *self -= amount
        }
        can_remove
    }

    pub fn set_regen_rate(&mut self, regen_rate: f32) {
        self.regen_rate = regen_rate;
    }

    pub fn tick(&mut self, time: &Time) {
        if self.value < Self::MAX {
            self.value += time.delta_seconds() * self.regen_rate
        }
    }
}

impl Default for Mana {
    fn default() -> Self {
        Self {
            value: Self::MAX,
            regen_rate: 0.5,
        }
    }
}

impl AddAssign<f32> for Mana {
    fn add_assign(&mut self, rhs: f32) {
        self.value += rhs;
        if self.value > Self::MAX {
            self.value = Self::MAX
        }
    }
}

impl SubAssign<f32> for Mana {
    fn sub_assign(&mut self, rhs: f32) {
        self.value -= rhs;
        if self.value < 0. {
            self.value = 0.
        }
    }
}
