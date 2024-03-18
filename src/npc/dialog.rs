use bevy::prelude::*;
use serde::Deserialize;

use crate::gui::styles::text_style;

#[derive(Deserialize)]
pub struct Dialog {
    pub lines: Vec<DialogLine>,
}

#[derive(Deserialize, Debug)]
pub struct DialogLine {
    pub choices: Vec<DialogChoice>,
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct DialogChoice {
    pub message: String,
    #[serde(default)]
    pub action: DialogChoiceAction,
}

#[derive(Clone, Deserialize, Default, Debug)]
pub enum DialogChoiceAction {
    #[default]
    NextLine,
    EndDialog(usize),
    GotoLine(usize),
    OpenShop(String),
}

pub struct DialogPlugin;
impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, dialog_update)
            .init_resource::<CurrentDialog>();
    }
}

#[derive(Resource, Default)]
pub struct CurrentDialog(pub Option<DialogResource>);

pub struct DialogResource {
    pub orator_name: String,
    pub dialog: Dialog,
    pub line_index: usize,
}

#[derive(Component)]
pub struct DialogUi;

#[derive(Component)]
pub struct DialogUiText;

#[derive(Component)]
pub struct DialogUiTextContainer;

fn dialog_update(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut current_dialog_res: ResMut<CurrentDialog>,
    dialog_ui_query: Query<Entity, With<DialogUi>>,
    mut text_query: Query<&mut Text, With<DialogUiText>>,
    text_container_query: Query<&Interaction, With<DialogUiTextContainer>>,
) {
    let Some(current_dialog) = &mut current_dialog_res.0 else {
        if let Ok(dialog_ui_entity) = dialog_ui_query.get_single() {
            commands.entity(dialog_ui_entity).despawn_recursive();
        }
        return;
    };

    if dialog_ui_query.get_single().is_err() {
        //Spawn ui
        commands
            //Global
            .spawn(NodeBundle {
                background_color: Color::NONE.into(),
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::End,
                    width: Val::Vw(100.),
                    height: Val::Vh(100.),
                    ..Default::default()
                },
                z_index: ZIndex::Global(12),
                ..Default::default()
            })
            .with_children(|builder| {
                //Choices
                builder.spawn(NodeBundle {
                    style: Style {
                        margin: UiRect::all(Val::Percent(5.)),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::End,
                        ..Default::default()
                    },
                    ..Default::default()
                });

                //Text
                builder
                    .spawn(ButtonBundle {
                        background_color: Color::BLACK.with_a(0.2).into(),
                        style: Style {
                            width: Val::Percent(80.),
                            margin: UiRect::all(Val::Percent(5.)),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|builder| {
                        builder.spawn(TextBundle::from_section(
                            current_dialog.orator_name.clone(),
                            TextStyle {
                                font_size: 40.,
                                ..text_style(&asset_server)
                            },
                        ));

                        builder
                            .spawn(TextBundle::from_section("", text_style(&asset_server)))
                            .insert(DialogUiText);
                    })
                    .insert(DialogUiTextContainer);
            });
    }

    //Update Ui
    let current_line = &current_dialog.dialog.lines[current_dialog.line_index];

    let mut text = text_query.single_mut();
    let dialog_needs_to_be_updated = text.sections[0].value != current_line.message;

    if dialog_needs_to_be_updated {
        //Update text
        text.sections[0].value = current_line.message.clone();

        //Update choices
    }

    let text_container_interaction = text_container_query.single();
    if current_line.choices.is_empty() {
        if *text_container_interaction == Interaction::Pressed {
            if current_dialog.line_index >= current_dialog.dialog.lines.len() {
                //End dialog
                current_dialog_res.0 = None;
            } else {
                //Goto next line
                current_dialog.line_index += 1;
            }
        }
    } else {
    }
}
