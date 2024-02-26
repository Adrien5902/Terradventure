use bevy::prelude::*;
use bevy_simple_text_input::TextInput;
use strum::{EnumCount, IntoEnumIterator};

use crate::{
    gui::{buttons::scroll::make_button, make_menu, misc::PIXEL_FONT},
    player::{
        class::{PlayerClass, PlayerClasses},
        PLAYER_TEXTURE,
    },
    save::{LoadSaveEvent, Save},
    state::AppState,
};

use super::MainMenuState;

pub struct NewWorldMenuPlugin;
impl Plugin for NewWorldMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                cancel_button,
                start_button,
                update_selected_class,
                right_arrow,
                left_arrow,
            )
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

#[derive(Resource)]
pub struct CurrentSelectedClass(pub usize);
impl Default for CurrentSelectedClass {
    fn default() -> Self {
        Self(3)
    }
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
    mut assets: ResMut<Assets<Image>>,
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
                        font: asset_server.load(PIXEL_FONT),
                        color: Color::WHITE,
                        ..default()
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
                        position_type: PositionType::Relative,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|builder| {
                    let class_count = PlayerClasses::COUNT;
                    for (i, class) in PlayerClasses::iter().enumerate() {
                        builder
                            .spawn(ImageBundle {
                                style: Style {
                                    left: Val::Percent((i / class_count) as f32 * 100.),
                                    position_type: PositionType::Absolute,
                                    ..Default::default()
                                },
                                image: UiImage {
                                    texture: assets.add(class.idle_texture()),
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .insert(ClassSelector);
                    }

                    builder
                        .spawn(TextBundle {
                            text: Text::from_section(
                                ">",
                                TextStyle {
                                    font: asset_server.load(PIXEL_FONT),
                                    font_size: 32.,
                                    ..Default::default()
                                },
                            ),
                            style: Style {
                                position_type: PositionType::Absolute,
                                right: Val::Vw(10.),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(ArrowRight);

                    builder
                        .spawn(TextBundle {
                            text: Text::from_section(
                                "<",
                                TextStyle {
                                    font: asset_server.load(PIXEL_FONT),
                                    font_size: 32.,
                                    ..Default::default()
                                },
                            ),
                            style: Style {
                                left: Val::Vw(10.),
                                position_type: PositionType::Absolute,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(ArrowLeft);
                });

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
                    make_button(builder, "Cancel", NewWorldCancelButton, &asset_server);
                    make_button(builder, "Start", NewWorldStartButton, &asset_server);
                });
        },
        None,
        None,
    )
}

fn right_arrow(
    mut selected_class: ResMut<CurrentSelectedClass>,
    button: Query<&Interaction, With<ArrowLeft>>,
) {
    for interaction in button.iter() {
        if *interaction == Interaction::Pressed {
            *selected_class = CurrentSelectedClass(selected_class.0 + 1)
        }
    }
}

fn left_arrow(
    mut selected_class: ResMut<CurrentSelectedClass>,
    button: Query<&Interaction, With<ArrowLeft>>,
) {
    for interaction in button.iter() {
        if *interaction == Interaction::Pressed {
            *selected_class = CurrentSelectedClass(selected_class.0 - 1)
        }
    }
}

fn update_selected_class(
    mut query: Query<&mut Style, With<ClassSelector>>,
    selected_class: Res<CurrentSelectedClass>,
) {
    for (i, mut style) in query.iter_mut().enumerate() {
        style.left = Val::Percent((i + selected_class.0 / PlayerClasses::COUNT) as f32 * 100.);
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
    query: Query<&Interaction, With<NewWorldStartButton>>,
    input_query: Query<&Children, With<WorldNameInput>>,
    children_query: Query<&Children>,
    text_query: Query<&Text>,
    mut state_change: ResMut<NextState<AppState>>,
    mut save_event: EventWriter<LoadSaveEvent>,
    selected_class: Res<CurrentSelectedClass>,
) {
    for interaction in query.iter() {
        if *interaction == Interaction::Pressed {
            if let Ok(children) = input_query.get_single() {
                for child in children.iter() {
                    if let Ok(inner_children) = children_query.get(*child) {
                        for inner_child in inner_children.iter() {
                            if let Ok(text) = text_query.get(*inner_child) {
                                state_change.set(AppState::InGame);
                                let save_name =
                                    format!("{}{}", text.sections[0].value, text.sections[2].value);

                                let class = PlayerClasses::iter().collect::<Vec<_>>()
                                    [selected_class.0]
                                    .clone();

                                let (save, meta) = Save::new(&save_name, class).unwrap();
                                save_event.send(LoadSaveEvent::new(&meta.name, save))
                            }
                        }
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
