use bevy::prelude::*;
use bevy_persistent::Persistent;

use crate::{
    gui::{make_menu, settings::Settings},
    state::AppState,
};

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
pub enum InventoryUiState {
    Opened,
    #[default]
    Closed,
}

pub struct InventoryUiPlugin;
impl Plugin for InventoryUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<InventoryUiState>()
            .add_systems(OnEnter(InventoryUiState::Opened), spawn_inventory)
            .add_systems(OnEnter(InventoryUiState::Closed), despawn_inventory)
            .add_systems(OnExit(AppState::InGame), despawn_inventory)
            .add_systems(Update, settings_toggle.run_if(in_state(AppState::InGame)));
    }
}

#[derive(Component)]
struct InventoryUi;

fn spawn_inventory(commands: Commands) {
    make_menu(
        commands,
        Color::/* rgba_u8(0, 0, 0, 0) */RED.into(),
        InventoryUi,
        |builder| {},
        None,
    );
}

fn settings_toggle(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    settings: Res<Persistent<Settings>>,
    state: Res<State<InventoryUiState>>,
    mut set_state: ResMut<NextState<InventoryUiState>>,
) {
    if input.just_pressed(settings.keybinds.inventory.get()) {
        set_state.set(if *state == InventoryUiState::Opened {
            InventoryUiState::Closed
        } else {
            InventoryUiState::Opened
        });
    }
}

fn despawn_inventory(
    mut commands: Commands,
    inventory_ui_query: Query<Entity, With<InventoryUi>>,
    mut set_state: ResMut<NextState<InventoryUiState>>,
) {
    for ui in inventory_ui_query.iter() {
        commands.entity(ui).despawn_recursive();
        set_state.set(InventoryUiState::Closed)
    }
}
