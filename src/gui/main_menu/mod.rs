use self::{load_world::LoadWorldMenuPlugin, new_world::NewWorldMenuPlugin};

use super::{buttons::scroll::make_button, make_menu, settings::ui::settings_button};
use crate::state::AppState;
use bevy::{app::AppExit, prelude::*};
use rand::{seq::SliceRandom, thread_rng};
use std::path::Path;

pub mod load_world;
pub mod new_world;

const BACKGROUNDS: [&str; 2] = ["montagnes.png", "plaines.png"];
pub struct MainMenuPlugin;

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum MainMenuState {
    #[default]
    Default,
    NewWorld,
    LoadWorld,
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::MainMenu(MainMenuState::Default)),
            spawn_main_menu,
        )
        .add_systems(
            Update,
            (
                quit_game_button_interact,
                new_world_menu_spawn_button,
                load_world_menu_spawn_button,
            )
                .run_if(not(in_state(AppState::InGame))),
        )
        .add_systems(
            OnExit(AppState::MainMenu(MainMenuState::Default)),
            despawn_main_menu,
        )
        .add_plugins((NewWorldMenuPlugin, LoadWorldMenuPlugin));
    }
}

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
struct LoadWorldButton;

#[derive(Component)]
struct NewWorldButton;

#[derive(Component)]
struct QuitGameButton;

fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    make_menu(
        &mut commands,
        Color::rgb_u8(167, 213, 235).into(),
        MainMenu,
        |builder| {
            let mut rng = thread_rng();
            let random_bg = BACKGROUNDS.choose(&mut rng).unwrap();
            builder.spawn(ImageBundle {
                image: UiImage::new(asset_server.load(Path::new("gui/main_menu").join(random_bg))),
                style: Style {
                    width: Val::Vw(100.0),
                    height: Val::Vh(100.0),
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    ..default()
                },
                ..default()
            });

            builder.spawn(ImageBundle {
                image: UiImage::new(asset_server.load("gui/main_menu/logo.png")),
                style: Style {
                    width: Val::Px(590.0),
                    height: Val::Px(316.5),
                    ..Default::default()
                },
                ..Default::default()
            });

            make_button(builder, "Continue", LoadWorldButton, &asset_server);
            make_button(builder, "New world", NewWorldButton, &asset_server);
            settings_button(builder, &asset_server);
            make_button(builder, "Quit Game", QuitGameButton, &asset_server);
        },
        None,
        None,
    );
}

fn new_world_menu_spawn_button(
    query: Query<&Interaction, With<NewWorldButton>>,
    mut state_change: ResMut<NextState<AppState>>,
) {
    if let Ok(interaction) = query.get_single() {
        if *interaction == Interaction::Pressed {
            state_change.set(AppState::MainMenu(MainMenuState::NewWorld))
        }
    }
}

fn load_world_menu_spawn_button(
    query: Query<&Interaction, With<LoadWorldButton>>,
    mut state_change: ResMut<NextState<AppState>>,
) {
    if let Ok(interaction) = query.get_single() {
        if *interaction == Interaction::Pressed {
            state_change.set(AppState::MainMenu(MainMenuState::LoadWorld))
        }
    }
}

fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

fn quit_game_button_interact(
    query: Query<&Interaction, With<QuitGameButton>>,
    mut exit: EventWriter<AppExit>,
) {
    if let Ok(interaction) = query.get_single() {
        if *interaction == Interaction::Pressed {
            exit.send(AppExit);
        }
    }
}
