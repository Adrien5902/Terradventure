use crate::{
    animation::AnimatedSpriteBundle, interactable::Interactable, lang::Lang, state::AppState,
};
use bevy::prelude::*;
use enum_dispatch::enum_dispatch;
use std::fs;
use strum_macros::{Display, EnumString};

use self::dialog::{Dialog, DialogChoice, DialogLine, DialogPlugin};

pub mod dialog;

#[enum_dispatch]
pub trait NpcTrait {
    fn texture_size(&self) -> u32;
}

#[derive(EnumString, Component, Display)]
#[enum_dispatch(NpcTrait)]
pub enum Npc {
    Blacksmith(Blacksmith),
    Witch(Witch),
    Alchemist(Alchemist),
}

impl Npc {
    pub fn dialog(&self) -> Option<Dialog> {
        let json_data = fs::read(format!("dialogs/{}", self.to_string())).ok()?;
        serde_json::from_slice(&json_data).ok()
    }

    pub fn translate_dialog_line(&self, line: &DialogLine, lang: &Lang) -> DialogLine {
        let npc_name = self.to_string();
        let npc_dialog_path = format!("npc.dialog.{npc_name}");
        DialogLine {
            message: lang
                .get(&format!("{npc_dialog_path}.line.{}", line.message))
                .into(),
            choices: line
                .choices
                .iter()
                .map(|choice| DialogChoice {
                    message: lang
                        .get(&format!("{npc_dialog_path}.choices.{}", choice.message))
                        .into(),
                    action: choice.action.clone(),
                })
                .collect(),
        }
    }
}

#[derive(Bundle)]
pub struct NpcBundle {
    pub npc: Npc,
    pub interactable: Interactable,
    pub sprite: AnimatedSpriteBundle,
}

pub struct NpcPlugin;
impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, npc_update_system.run_if(in_state(AppState::InGame)))
            .add_plugins(DialogPlugin);
    }
}

fn npc_update_system(mut query: Query<(&Npc, &Interactable)>, lang: Res<Lang>) {
    for (npc, interactable) in query.iter_mut() {
        if interactable.just_pressed() {
            if let Some(dialog) = npc.dialog() {
                println!("{:?}", npc.translate_dialog_line(&dialog.lines[0], &lang));
            }
        }
    }
}

#[derive(Default)]
pub struct Blacksmith;
impl NpcTrait for Blacksmith {
    fn texture_size(&self) -> u32 {
        80
    }
}

#[derive(Default)]
pub struct Alchemist;
impl NpcTrait for Alchemist {
    fn texture_size(&self) -> u32 {
        85
    }
}

#[derive(Default)]
pub struct Witch;
impl NpcTrait for Witch {
    fn texture_size(&self) -> u32 {
        128
    }
}
