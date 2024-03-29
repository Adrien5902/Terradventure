use super::{buttons::scroll, main_menu::MainMenuState, make_menu, settings::ui::settings_button};
use crate::{lang::Lang, state::AppState};
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

fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>, lang: Res<Lang>) {
    make_menu(
        &mut commands,
        Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
        PauseMenu,
        |builder| {
            scroll::make_button(
                builder,
                lang.get("ui.pause.resume"),
                ResumeButton,
                &asset_server,
            );
            settings_button(builder, &asset_server, &lang);
            scroll::make_button(
                builder,
                lang.get("ui.pause.quit"),
                LeaveButton,
                &asset_server,
            );
        },
        None,
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
            state_next_state.set(AppState::MainMenu(MainMenuState::Default))
        }
    }
}
