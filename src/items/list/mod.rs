use bevy::{
    app::{Plugin, Update},
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs},
    reflect::Reflect,
};
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use strum_macros::EnumString;

use crate::{player::inventory::SlotType, state::AppState};

use self::{
    levitation_potion::{use_levitation_potion, LevitationPotion},
    mana_potion::{use_mana_potion, ManaPotion},
    porkchop::Porkchop,
    processed_ore::ProcessedOre,
    unprocessed_ore::UnprocessedOre,
};
use super::item::{ItemName, ItemTrait, StackSize};

pub mod levitation_potion;
pub mod mana_potion;
pub mod porkchop;
pub mod processed_ore;
pub mod unprocessed_ore;

pub struct ItemsPlugin;
impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (use_mana_potion, use_levitation_potion).run_if(in_state(AppState::InGame)),
        );
    }
}

#[derive(Clone, Deserialize, Serialize, Reflect, PartialEq, Eq, EnumString)]
#[enum_dispatch(ItemTrait)]
pub enum Item {
    ManaPotion(ManaPotion),
    LevitationPotion(LevitationPotion),
    Porkchop(Porkchop),
    UnprocessedOre(UnprocessedOre),
    ProcessedOre(ProcessedOre),
}
