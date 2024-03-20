use bevy::prelude::*;
use bevy_simple_text_input::TextInput;
use strum::{EnumCount, IntoEnumIterator};

use crate::{
    gui::{buttons::scroll::make_button, make_menu, misc::PIXEL_FONT, styles::text_style},
    lang::Lang,
    player::class::{PlayerClass, PlayerClasses},
    save::{LoadSaveEvent, Save},
    state::AppState,
};

use super::MainMenuState;

pub struct NewWorldMenuPlugin;
impl Plugin for NewWorldMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (cancel_button, start_button, update_selected_class)
                .run_if(in_state(AppState::MainMenu(MainMenuState::NewWorld))),
        )
        .add_systems(
            OnEnter(AppState::MainMenu(MainMenuState::NewWorld)),
            spawn_new_world_menu,
        )
        .add_systems(
            OnExit(AppState::MainMenu(MainMenuState::NewWorld)),
            despawn_new_world_menu,
        )
        .init_resource::<CurrentSelectedClass>();
    }
}

#[derive(Resource, Debug, Default)]
pub struct CurrentSelectedClass {
    index: usize,
}

#[derive(Component)]
pub struct NewWorldMenu;

#[derive(Component)]
pub struct NewWorldCancelButton;

#[derive(Component)]
pub struct NewWorldStartButton;

#[derive(Component)]
pub struct WorldNameInput;

#[derive(Component)]
pub struct ArrowRight;

#[derive(Component)]
pub struct ArrowLeft;

#[derive(Component)]
pub struct ClassSelector;

fn spawn_new_world_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    selected_class: Res<CurrentSelectedClass>,
    lang: Res<Lang>,
) {
    make_menu(
        &mut commands,
        Color::BLACK.into(),
        NewWorldMenu,
        |builder| {
            builder.spawn((
                WorldNameInput,
                NodeBundle {
                    style: Style {
                        width: Val::Px(500.0),
                        border: UiRect::all(Val::Px(5.0)),
                        padding: UiRect::all(Val::Px(5.0)),
                        margin: UiRect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    border_color: BorderColor(Color::WHITE),
                    background_color: Color::GRAY.into(),
                    ..Default::default()
                },
                TextInput {
                    text_style: TextStyle {
                        font_size: 40.,
                        ..text_style(&asset_server)
                    },
                    ..Default::default()
                },
            ));

            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::vertical(Val::Percent(5.)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|builder| {
                    make_arrow_button(
                        builder,
                        &asset_server,
                        ArrowLeft,
                        false,
                        Some(UiRect::top(Val::Percent(10.))),
                    );

                    builder
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.),
                                height: Val::Percent(100.),
                                display: Display::Flex,
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|builder| {
                            for (i, class) in PlayerClasses::iter().enumerate() {
                                builder
                                    .spawn(ImageBundle {
                                        background_color: Color::WHITE.into(),
                                        style: calc_class_style(selected_class.index, i),
                                        image: UiImage {
                                            texture: asset_server.add(class.idle_texture()),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    })
                                    .insert(ClassSelector);
                            }
                        });

                    make_arrow_button(
                        builder,
                        &asset_server,
                        ArrowRight,
                        true,
                        Some(UiRect::top(Val::Percent(10.))),
                    );
                });

            builder
                .spawn(TextBundle {
                    text: text_from_class(&lang, &selected_class, &asset_server),
                    ..Default::default()
                })
                .insert(ClassSelectText);

            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|builder| {
                    make_button(
                        builder,
                        lang.get("ui.main_menu.new_world.cancel"),
                        NewWorldCancelButton,
                        &asset_server,
                    );
                    make_button(
                        builder,
                        lang.get("ui.main_menu.new_world.confirm"),
                        NewWorldStartButton,
                        &asset_server,
                    );
                });
        },
        None,
        None,
    )
}

fn text_from_class(
    lang: &Res<Lang>,
    selected_class: &CurrentSelectedClass,
    asset_server: &Res<AssetServer>,
) -> Text {
    Text::from_section(
        lang.get(&format!(
            "player.classes.{}",
            PlayerClasses::iter().collect::<Vec<_>>()[selected_class.index].name(),
        )),
        text_style(asset_server),
    )
}

#[derive(Component)]
pub struct ClassSelectText;

fn calc_class_style(current_index: usize, this_index: usize) -> Style {
    let diff = current_index.abs_diff(this_index);
    let circ_diff = std::cmp::min(diff, PlayerClasses::COUNT - diff);

    let display = if circ_diff <= 1 {
        Display::DEFAULT
    } else {
        Display::None
    };

    let height = Val::Percent(if circ_diff == 0 { 150. } else { 100. });

    Style {
        display,
        height,
        ..Default::default()
    }
}

fn make_arrow_button<T: Component>(
    builder: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    typ: T,
    right: bool,
    margin: Option<UiRect>,
) {
    builder
        .spawn((
            ButtonBundle {
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                style: Style {
                    margin: margin.unwrap_or_default(),
                    ..Default::default()
                },
                ..Default::default()
            },
            typ,
        ))
        .with_children(|button_builder| {
            button_builder.spawn(TextBundle {
                text: Text::from_section(
                    if right { ">" } else { "<" }, // o((>ω<))o
                    TextStyle {
                        font: asset_server.load(PIXEL_FONT),
                        font_size: 64.,
                        ..Default::default()
                    },
                ),
                ..Default::default()
            });
        });
}

fn update_selected_class(
    mut query: Query<&mut Style, With<ClassSelector>>,
    mut selected_class: ResMut<CurrentSelectedClass>,
    mut text_query: Query<&mut Text, With<ClassSelectText>>,
    button: Query<(Entity, &Interaction), Changed<Interaction>>,
    left_query: Query<&ArrowLeft>,
    right_query: Query<&ArrowRight>,
    lang: Res<Lang>,
    asset_server: Res<AssetServer>,
) {
    for (entity, interaction) in button.iter() {
        let left = left_query.get(entity).is_ok();
        let right = right_query.get(entity).is_ok();

        if *interaction == Interaction::Pressed {
            if right {
                *selected_class = CurrentSelectedClass {
                    index: (selected_class.index + 1) % PlayerClasses::COUNT,
                };
            } else if left {
                *selected_class = CurrentSelectedClass {
                    index: if selected_class.index == 0 {
                        PlayerClasses::COUNT - 1
                    } else {
                        (selected_class.index - 1) % PlayerClasses::COUNT
                    },
                };
            }

            if let Ok(mut text) = text_query.get_single_mut() {
                *text = text_from_class(&lang, &selected_class, &asset_server);
            }

            for (i, mut style) in query.iter_mut().enumerate() {
                *style = calc_class_style(selected_class.index, i);
            }
        }
    }
}

fn cancel_button(
    query: Query<&Interaction, With<NewWorldCancelButton>>,
    mut state_change: ResMut<NextState<AppState>>,
) {
    for interaction in query.iter() {
        if *interaction == Interaction::Pressed {
            state_change.set(AppState::MainMenu(MainMenuState::Default))
        }
    }
}

fn start_button(
    query: Query<&Interaction, (With<NewWorldStartButton>, Changed<Interaction>)>,
    mut input_query: Query<(&Children, &mut BorderColor), With<WorldNameInput>>,
    children_query: Query<&Children>,
    text_query: Query<&Text>,
    mut state_change: ResMut<NextState<AppState>>,
    mut save_event: EventWriter<LoadSaveEvent>,
    selected_class: Res<CurrentSelectedClass>,
) {
    for interaction in query.iter() {
        if *interaction == Interaction::Pressed {
            let Ok((children, mut border_color)) = input_query.get_single_mut() else {
                continue;
            };

            for child in children.iter() {
                let Ok(inner_children) = children_query.get(*child) else {
                    continue;
                };

                for inner_child in inner_children.iter() {
                    let Ok(text) = text_query.get(*inner_child) else {
                        continue;
                    };

                    let save_name = format!("{}{}", text.sections[0].value, text.sections[2].value);

                    let class =
                        PlayerClasses::iter().collect::<Vec<_>>()[selected_class.index].clone();

                    match Save::new(&save_name, class) {
                        Ok((save, meta)) => {
                            state_change.set(AppState::InGame);
                            save_event.send(LoadSaveEvent::new(&meta.name, save))
                        }
                        Err(_) => *border_color = BorderColor(Color::RED),
                    }
                }
            }
        }
    }
}

fn despawn_new_world_menu(mut commands: Commands, menu_query: Query<Entity, With<NewWorldMenu>>) {
    for menu in menu_query.iter() {
        commands.entity(menu).despawn_recursive();
    }
}
