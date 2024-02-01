use super::{buttons::scroll, make_menu, settings::settings_button, styles::aligned_center};
use crate::state::AppState;
use bevy::prelude::*;

pub struct PausePlugin;

#[derive(Component)]
pub struct PauseMenu;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Paused), spawn_pause_menu)
            .add_systems(OnExit(AppState::Paused), despawn_pause_menu)
            .add_systems(
                Update,
                (resume_button_interact, leave_button_interact).run_if(in_state(AppState::Paused)),
            );
    }
}

#[derive(Component)]
struct ResumeButton;

#[derive(Component)]
struct LeaveButton;

fn spawn_pause_menu(commands: Commands, asset_server: Res<AssetServer>) {
    make_menu(
        commands,
        BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.5)),
        PauseMenu,
        |builder| {
            scroll::make_button(builder, "Resume", ResumeButton, &asset_server);
            settings_button(builder, &asset_server);
            scroll::make_button(builder, "Leave", LeaveButton, &asset_server);
        },
        None,
    );
}

fn despawn_pause_menu(mut commands: Commands, query: Query<Entity, With<PauseMenu>>) {
    if let Ok(menu) = query.get_single() {
        commands.entity(menu).despawn_recursive();
    }
}

fn resume_button_interact(
    query: Query<&Interaction, With<ResumeButton>>,
    mut state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok(interaction) = query.get_single() {
        if *interaction == Interaction::Pressed {
            state_next_state.set(AppState::InGame)
        }
    }
}

fn leave_button_interact(
    query: Query<&Interaction, With<LeaveButton>>,
    mut state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok(interaction) = query.get_single() {
        if *interaction == Interaction::Pressed {
            state_next_state.set(AppState::MainMenu)
        }
    }
}
