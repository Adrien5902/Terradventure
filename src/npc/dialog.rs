use super::shop::{CurrentShop, Shop};
use crate::{gui::styles::text_style, state::AppState};
use bevy::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Dialog {
    pub lines: Vec<DialogLine>,
}

impl Dialog {
    const CHARACTER_SPAWN_SPEED: f32 = 50.;
}

#[derive(Deserialize, Debug)]
pub struct DialogLine {
    #[serde(default)]
    pub choices: Vec<DialogChoice>,
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct DialogChoice {
    pub message: String,
    #[serde(default)]
    pub action: DialogChoiceAction,
}

#[derive(Clone, Deserialize, Default, Debug, Component)]
pub enum DialogChoiceAction {
    #[default]
    NextLine,
    EndDialog(String),
    GotoLine(usize),
    OpenShop(String),
}

pub struct DialogPlugin;
impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, dialog_update.run_if(in_state(AppState::InGame)))
            .add_systems(OnExit(AppState::InGame), despawn_dialog_ui)
            .init_resource::<CurrentDialog>();
    }
}

#[derive(Resource, Default)]
pub struct CurrentDialog(pub Option<DialogResource>);

pub struct DialogResource {
    pub orator_image: Handle<Image>,
    pub orator_name: String,
    pub dialog: Dialog,
    pub line_index: isize,
}

#[derive(Component)]
pub struct DialogUi;

#[derive(Component)]
pub struct DialogUiText {
    full_text: String,
    current_index: f32,
}

#[derive(Component)]
pub struct DialogUiTextContainer;

#[derive(Component)]
pub struct DialogUiChoicesContainer;

fn dialog_update(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut current_dialog_res: ResMut<CurrentDialog>,
    dialog_ui_query: Query<Entity, With<DialogUi>>,
    mut text_query: Query<(&mut Text, &mut DialogUiText)>,
    text_container_query: Query<&Interaction, With<DialogUiTextContainer>>,
    choices_container_query: Query<Entity, With<DialogUiChoicesContainer>>,
    choices_query: Query<(&DialogChoiceAction, &Interaction)>,
    mut current_shop: ResMut<CurrentShop>,
    time: Res<Time>,
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
            .insert(DialogUi)
            .with_children(|builder| {
                //Choices
                builder
                    .spawn(NodeBundle {
                        style: Style {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::End,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(DialogUiChoicesContainer);

                //Text Container
                builder
                    .spawn(ButtonBundle {
                        background_color: Color::BLACK.with_a(0.7).into(),
                        style: Style {
                            width: Val::Percent(80.),
                            margin: UiRect::all(Val::Percent(2.)),
                            padding: UiRect::all(Val::Percent(2.)),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|builder| {
                        //NPC name
                        builder
                            .spawn(NodeBundle {
                                border_color: BorderColor(Color::WHITE),
                                style: Style {
                                    margin: UiRect::bottom(Val::Percent(2.)),
                                    border: UiRect::bottom(Val::Px(4.)),
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
                            });

                        //Dialog text
                        builder
                            .spawn(TextBundle::from_section("", text_style(&asset_server)))
                            .insert(DialogUiText {
                                current_index: 0.,
                                full_text: String::default(),
                            });
                    })
                    .insert(DialogUiTextContainer);

                //NPC Image
                builder.spawn(ImageBundle {
                    image: UiImage::new(current_dialog.orator_image.clone()),
                    style: Style {
                        position_type: PositionType::Absolute,
                        right: Val::Percent(-18.),
                        bottom: Val::Percent(-35.),
                        width: Val::Vw(50.),
                        height: Val::Vw(50.),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            });
    } else {
        //Update Ui
        let (mut text, mut dialog_text) = text_query.single_mut();
        let choices_container = choices_container_query.single();

        let text_container_interaction = text_container_query.single();

        let line_index = current_dialog.line_index as usize;
        let current_line_opt = current_dialog.dialog.lines.get(line_index);

        if let Some(current_line) = current_line_opt {
            let dialog_needs_to_be_updated = dialog_text.full_text != current_line.message;

            if dialog_needs_to_be_updated {
                //Update text
                dialog_text.full_text = current_line.message.clone();
                dialog_text.current_index = 0.;
                text.sections[0].value = String::default();

                //Update choices
                commands.entity(choices_container).despawn_descendants();
                for choice in &current_line.choices {
                    let choice_entity = commands
                        .spawn(ButtonBundle {
                            background_color: Color::BLACK.with_a(0.7).into(),
                            style: Style {
                                margin: UiRect::all(Val::Percent(1.)),
                                padding: UiRect::all(Val::Percent(1.)),
                                min_width: Val::Percent(30.),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|builder| {
                            builder.spawn(TextBundle::from_section(
                                &choice.message,
                                text_style(&asset_server),
                            ));
                        })
                        .insert(choice.action.clone())
                        .id();

                    commands.entity(choices_container).add_child(choice_entity);
                }
            }
        }

        if !current_line_opt.is_some_and(|c| !c.choices.is_empty()) {
            if *text_container_interaction == Interaction::Pressed {
                next_line(&mut current_dialog_res);
            }
        } else {
            for (choice_action, interaction) in choices_query.iter() {
                if *interaction == Interaction::Pressed {
                    match choice_action {
                        DialogChoiceAction::EndDialog(message) => {
                            current_dialog.line_index = -1;
                            dialog_text.full_text = message.clone();
                            commands.entity(choices_container).despawn_descendants();
                        }
                        DialogChoiceAction::OpenShop(shop_name) => {
                            let shop = Shop::read(shop_name);
                            *current_shop = CurrentShop { shop };

                            next_line(&mut current_dialog_res);

                            return;
                        }
                        DialogChoiceAction::GotoLine(index) => {
                            current_dialog.line_index = *index as isize;
                        }
                        DialogChoiceAction::NextLine => {
                            next_line(&mut current_dialog_res);
                            return;
                        }
                    }
                }
            }
        }

        let chars = dialog_text.full_text.chars().collect::<Vec<_>>();
        let current_char_index = dialog_text.current_index as usize;
        if current_char_index <= chars.len() {
            text.sections[0].value = chars[0..current_char_index].iter().collect();
            dialog_text.current_index += Dialog::CHARACTER_SPAWN_SPEED * time.delta_seconds();
        }
    }
}

fn despawn_dialog_ui(
    mut commands: Commands,
    dialog_ui_query: Query<Entity, With<DialogUi>>,
    mut current_dialog_res: ResMut<CurrentDialog>,
) {
    for entity in dialog_ui_query.iter() {
        commands.entity(entity).despawn_recursive();
        current_dialog_res.0 = None;
    }
}

fn next_line(current_dialog_res: &mut CurrentDialog) {
    let current_dialog = current_dialog_res.0.as_mut().unwrap();
    if current_dialog.line_index < 0 {
        current_dialog_res.0 = None;
        return;
    }

    if (current_dialog.line_index as usize) < current_dialog.dialog.lines.len() {
        //Goto next line
        current_dialog.line_index += 1;
    } else {
        //End dialog
        current_dialog_res.0 = None;
    }
}

pub fn in_dialog(current_dialog: Res<CurrentDialog>) -> bool {
    current_dialog.0.is_some()
}
