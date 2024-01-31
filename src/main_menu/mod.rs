use bevy::prelude::*;

use crate::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(Update, (play_button_interact, pause_system))
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
struct PlayButton;

fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            MainMenu,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..aligned_center()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..Default::default()
            },
        ))
        .with_children(|builder| {
            builder
                .spawn((
                    PlayButton,
                    ButtonBundle {
                        style: button_style(),
                        background_color: BackgroundColor(Color::WHITE),
                        ..Default::default()
                    },
                ))
                .with_children(|text_builder| {
                    text_builder.spawn(TextBundle {
                        text: Text {
                            sections: into_text_sections(&["test"], &asset_server),
                            alignment: TextAlignment::Center,
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
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
                    color: Color::BLACK,
                    font_size: 24.0,
                    font: asset_server.load("fonts/Silkscreen-Bold.ttf"),
                },
            )
        })
        .collect()
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

fn pause_system(mut app_state_next_state: ResMut<NextState<AppState>>, input: Res<Input<KeyCode>>) {
    if input.pressed(KeyCode::Escape) {
        app_state_next_state.set(AppState::MainMenu)
    }
}
