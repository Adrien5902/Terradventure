use bevy::{
    app::{Plugin, Update},
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs},
    reflect::Reflect,
};
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::{player::inventory::SlotType, state::AppState};

use self::{
    mana_potion::{use_mana_potion, ManaPotion},
    wool::Wool,
};
use super::item::{Item, ItemName, StackSize};

pub mod mana_potion;
pub mod wool;

pub struct ItemsPlugin;
impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, (use_mana_potion).run_if(in_state(AppState::InGame)));
    }
}

#[derive(Clone, Deserialize, Serialize, Reflect, PartialEq, Eq)]
#[enum_dispatch(Item)]
pub enum ItemObject {
    Wool(Wool),
    ManaPotion(ManaPotion),
}
