use bevy::prelude::*;

use crate::gui::main_menu::MainMenuState;

#[derive(States, Debug, Hash, PartialEq, Eq, Clone)]
pub enum AppState {
    MainMenu(MainMenuState),
    InGame,
    Paused,
}

impl Default for AppState {
    fn default() -> Self {
        AppState::MainMenu(MainMenuState::Default)
    }
}

pub struct AppStatePlugin;
impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_systems(Update, pause_system);
    }
}

fn pause_system(
    mut app_state_next_state: ResMut<NextState<AppState>>,
    input: Res<Input<KeyCode>>,
    current_state: Res<State<AppState>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        if *current_state == AppState::InGame {
            app_state_next_state.set(AppState::Paused)
        } else if *current_state == AppState::Paused {
            app_state_next_state.set(AppState::InGame);
        }
    }
}
