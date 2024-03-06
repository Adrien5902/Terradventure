use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    gui::{misc::PIXEL_FONT, styles::text_style},
    lang::Lang,
};

use super::Settings;

#[derive(Serialize, Deserialize, Reflect)]
pub struct Keybinds {
    pub move_left: Keybind,
    pub move_right: Keybind,
    pub interact: Keybind,
    pub jump: Keybind,
    pub inventory: Keybind,
    pub attack: Keybind,
    pub special_attack_1: Keybind,
    pub special_attack_2: Keybind,
    pub special_attack_3: Keybind,
    pub split_stack: Keybind,
}

impl Default for Keybinds {
    fn default() -> Self {
        Self {
            move_left: Keybind::Keyboard(KeyCode::Q),
            move_right: Keybind::Keyboard(KeyCode::D),
            interact: Keybind::Keyboard(KeyCode::E),
            jump: Keybind::Keyboard(KeyCode::Space),
            inventory: Keybind::Keyboard(KeyCode::A),
            attack: Keybind::Keyboard(KeyCode::J),
            special_attack_1: Keybind::Keyboard(KeyCode::K),
            special_attack_2: Keybind::Keyboard(KeyCode::L),
            special_attack_3: Keybind::Keyboard(KeyCode::M),
            split_stack: Keybind::Keyboard(KeyCode::ShiftLeft),
        }
    }
}

#[derive(Serialize, Deserialize, Reflect, Clone)]
pub enum Keybind {
    Keyboard(KeyCode),
    Mouse(MouseButton),
}

impl Keybind {
    pub fn pressed(
        &self,
        keyboard_input: &Res<Input<KeyCode>>,
        mouse_input: &Res<Input<MouseButton>>,
    ) -> bool {
        match *self {
            Self::Keyboard(key_code) => keyboard_input.pressed(key_code),
            Keybind::Mouse(button) => mouse_input.pressed(button),
        }
    }

    pub fn just_pressed(
        &self,
        keyboard_input: &Res<Input<KeyCode>>,
        mouse_input: &Res<Input<MouseButton>>,
    ) -> bool {
        match *self {
            Self::Keyboard(key_code) => keyboard_input.just_pressed(key_code),
            Keybind::Mouse(button) => mouse_input.just_pressed(button),
        }
    }

    pub fn to_string(&self) -> String {
        match *self {
            Self::Keyboard(key_code) => format!("{:?}", key_code),
            Self::Mouse(button) => format!("Mouse: {:?}", button),
        }
    }
}

pub fn keybinds_menu(
    builder: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    settings: &Res<Settings>,
    lang: &Res<Lang>,
) {
    builder
        //Node des keybinds
        .spawn(KeybindsMenu {
            editing_field: None,
        })
        .insert(NodeBundle {
            background_color: Color::NONE.into(),
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                margin: UiRect::all(Val::Px(4.)),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|builder| {
            //Texte titre
            builder.spawn(TextBundle::from_section(
                lang.get("ui.settings.keybinds.name"),
                TextStyle {
                    font: asset_server.load(PIXEL_FONT),
                    font_size: 50.,
                    ..Default::default()
                },
            ));

            //Boucles sur les keybinds
            for (index, _) in settings.keybinds.iter_fields().enumerate() {
                let field_name = settings.keybinds.name_at(index).unwrap();

                //Ligne pour le bind
                builder
                    .spawn(NodeBundle {
                        background_color: Color::NONE.into(),
                        style: Style {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            margin: UiRect::all(Val::Px(2.)),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|builder| {
                        //Description du bind
                        builder.spawn(TextBundle::from_section(
                            format!(
                                "{} :  ",
                                lang.get(&format!("ui.settings.keybinds.{}", field_name))
                            ),
                            text_style(asset_server),
                        ));

                        //Bouton pour modifier
                        builder
                            .spawn(KeybindEdit {
                                field: field_name.to_owned(),
                            })
                            .insert(ButtonBundle {
                                border_color: BorderColor(Color::WHITE),
                                background_color: Color::WHITE.into(),
                                style: Style {
                                    border: UiRect::all(Val::Px(2.)),
                                    display: Display::Flex,
                                    justify_content: JustifyContent::Center,
                                    width: Val::Px(100.),
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .with_children(|builder| {
                                //Nom de la touche à l'intérieur du bouton
                                builder.spawn(TextBundle::from_section(
                                    settings
                                        .keybinds
                                        .get_field::<Keybind>(field_name)
                                        .unwrap()
                                        .to_string(),
                                    TextStyle {
                                        font: asset_server.load(PIXEL_FONT),
                                        font_size: 24.,
                                        color: Color::BLACK,
                                    },
                                ));
                            });
                    });
            }
        });
}

#[derive(Component)]
pub struct KeybindsMenu {
    editing_field: Option<String>,
}

#[derive(Component)]
pub struct KeybindEdit {
    pub field: String,
}

pub fn keybinds_update(
    mut text_query: Query<&mut Text>,
    mut button_query: Query<(&Interaction, &KeybindEdit, &mut BackgroundColor, &Children)>,
    mut keybinds_menu_query: Query<&mut KeybindsMenu>,
    mut settings: ResMut<Settings>,
    keyboard: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
) {
    if let Ok(mut keybinds_menu) = keybinds_menu_query.get_single_mut() {
        'outer: {
            if let Some(field) = keybinds_menu.editing_field.clone() {
                let mut update = |bind: Keybind| {
                    keybinds_menu.editing_field = None;

                    settings.update(|s| {
                        s.keybinds
                            .field_mut(&field)
                            .unwrap()
                            .set(Box::new(bind.clone()))
                            .unwrap();
                    });

                    for (_, keybind_edit, _, children) in button_query.iter_mut() {
                        if keybind_edit.field == field {
                            for child in children.iter() {
                                if let Ok(mut text) = text_query.get_mut(*child) {
                                    text.sections[0].value = bind.to_string();
                                }
                            }
                        }
                    }
                };

                for key in keyboard.get_just_pressed() {
                    update(Keybind::Keyboard(key.clone()));
                    break 'outer;
                }

                for button in mouse.get_just_pressed() {
                    update(Keybind::Mouse(button.clone()));
                    break 'outer;
                }
            }
        }

        for (interaction, edit, mut background_color, children) in button_query.iter_mut() {
            if *interaction == Interaction::Pressed {
                keybinds_menu.editing_field = Some(edit.field.clone());
                *background_color = Color::BLACK.into();
                for child in children.iter() {
                    if let Ok(mut text) = text_query.get_mut(*child) {
                        text.sections[0].style.color = Color::WHITE
                    }
                }
            } else if keybinds_menu.editing_field.as_ref() != Some(&edit.field) {
                for child in children.iter() {
                    if let Ok(mut text) = text_query.get_mut(*child) {
                        text.sections[0].style.color = Color::BLACK;
                    }
                }
                *background_color = Color::WHITE.into()
            }
        }
    }
}
