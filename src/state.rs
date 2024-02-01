use bevy::prelude::*;

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    Paused,
}

pub struct AppStatePlugin;
impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_systems(Update, pause_system);
    }
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone)]
pub struct InGameState {
    pub paused: bool,
}

impl Default for InGameState {
    fn default() -> Self {
        Self { paused: false }
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
