use super::{buttons::scroll::make_button, make_menu, settings::ui::settings_button};
use crate::state::AppState;
use bevy::{app::AppExit, prelude::*};
use rand::{seq::SliceRandom, thread_rng};
use std::path::Path;

const BACKGROUNDS: [&str; 2] = ["montagnes.png", "plaines.png"];
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(
                Update,
                (play_button_interact, quit_game_button_interact)
                    .run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
struct PlayButton;

#[derive(Component)]
struct QuitGameButton;

fn spawn_main_menu(commands: Commands, asset_server: Res<AssetServer>) {
    make_menu(
        commands,
        BackgroundColor(Color::rgb_u8(167, 213, 235)),
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
                    height: Val::Px(410.0),
                    ..Default::default()
                },
                ..Default::default()
            });

            make_button(builder, "Play", PlayButton, &asset_server);
            settings_button(builder, &asset_server);
            make_button(builder, "Quit Game", QuitGameButton, &asset_server);
        },
        None,
    );
}

fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
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
