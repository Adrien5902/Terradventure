use bevy::{app::AppExit, prelude::*};

use crate::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(
                Update,
                (
                    play_button_interact,
                    pause_system,
                    quit_game_button_interact,
                ),
            )
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
struct PlayButton;

#[derive(Component)]
struct SettingsButton;

#[derive(Component)]
struct QuitGameButton;

fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            MainMenu,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    ..aligned_center()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..Default::default()
            },
        ))
        .with_children(|builder| {
            make_button(builder, "Play", PlayButton, &asset_server);
            make_button(builder, "Settings", SettingsButton, &asset_server);
            make_button(builder, "Quit Game", QuitGameButton, &asset_server);
        });
}

fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn button_style() -> Style {
    Style {
        width: Val::Px(200.0),
        height: Val::Px(50.0),
        margin: UiRect::all(Val::Px(10.0)),
        ..aligned_center()
    }
}

pub fn aligned_center() -> Style {
    Style {
        display: Display::Flex,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

fn into_text_sections(data: &[&'static str], asset_server: &Res<AssetServer>) -> Vec<TextSection> {
    data.iter()
        .map(|s| {
            TextSection::new(
                *s,
                TextStyle {
                    color: Color::WHITE,
                    font_size: 24.0,
                    font: asset_server.load("fonts/Silkscreen-Bold.ttf"),
                },
            )
        })
        .collect()
}

fn pause_system(mut app_state_next_state: ResMut<NextState<AppState>>, input: Res<Input<KeyCode>>) {
    if input.pressed(KeyCode::Escape) {
        app_state_next_state.set(AppState::MainMenu)
    }
}

fn play_button_interact(
    mut query: Query<&Interaction, With<PlayButton>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok(interaction) = query.get_single_mut() {
        if *interaction == Interaction::Pressed {
            app_state_next_state.set(AppState::InGame)
        }
    }
}

fn quit_game_button_interact(
    mut query: Query<&Interaction, With<QuitGameButton>>,
    mut exit: EventWriter<AppExit>,
) {
    if let Ok(interaction) = query.get_single_mut() {
        if *interaction == Interaction::Pressed {
            exit.send(AppExit);
        }
    }
}

fn make_button<T: Component>(
    builder: &mut ChildBuilder,
    text: &'static str,
    typ: T,
    asset_server: &Res<AssetServer>,
) {
    builder
        .spawn((
            typ,
            ButtonBundle {
                style: button_style(),
                image: UiImage::new(asset_server.load("scroll.png")),
                background_color: BackgroundColor(Color::WHITE),
                ..Default::default()
            },
        ))
        .with_children(|text_builder| {
            text_builder.spawn(TextBundle {
                text: Text {
                    sections: into_text_sections(&[text], asset_server),
                    alignment: TextAlignment::Center,
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}
