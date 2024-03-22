use crate::{
    animation::AnimatedSpriteBundle, interactable::Interactable, lang::Lang, misc::read_img,
    state::AppState,
};
use bevy::prelude::*;
use enum_dispatch::enum_dispatch;
use std::{
    fs,
    path::{Path, PathBuf},
};
use strum_macros::{Display, EnumString};

use self::dialog::{
    CurrentDialog, Dialog, DialogChoice, DialogChoiceAction, DialogLine, DialogPlugin,
    DialogResource,
};

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
        let json_data = fs::read(format!("assets/dialogs/{}.json", self.to_string())).ok()?;
        Some(serde_json::from_slice(&json_data).unwrap())
    }

    pub fn translated_dialog(&self, lang: &Lang) -> Option<Dialog> {
        let lines = self
            .dialog()?
            .lines
            .iter()
            .map(|line| self.translate_dialog_line(line, lang))
            .collect();

        Some(Dialog { lines })
    }

    pub fn translate_dialog_line(&self, line: &DialogLine, lang: &Lang) -> DialogLine {
        let npc_name = self.to_string();
        let npc_dialog_path = format!("npc.{npc_name}.dialog");
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
                    action: match &choice.action {
                        DialogChoiceAction::EndDialog(message) => DialogChoiceAction::EndDialog(
                            lang.get(&format!("{npc_dialog_path}.line.{}", message))
                                .into(),
                        ),
                        _ => choice.action.clone(),
                    },
                })
                .collect(),
        }
    }

    pub fn get_texture(&self) -> PathBuf {
        Path::new("textures/npc").join(&format!("{}.png", self.to_string()))
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

fn npc_update_system(
    mut query: Query<(&Npc, &Interactable)>,
    lang: Res<Lang>,
    mut current_dialog: ResMut<CurrentDialog>,
    asset_server: Res<AssetServer>,
) {
    for (npc, interactable) in query.iter_mut() {
        if interactable.just_pressed() {
            if let Some(dialog) = npc.translated_dialog(&lang) {
                let image = asset_server.add(Image::from_dynamic(
                    read_img(npc.get_texture()).crop(0, 0, npc.texture_size(), npc.texture_size()),
                    true,
                ));

                current_dialog.0 = Some(DialogResource {
                    orator_image: image,
                    dialog,
                    line_index: 0,
                    orator_name: lang.get(&format!("npc.{}.name", npc.to_string())).into(),
                })
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
