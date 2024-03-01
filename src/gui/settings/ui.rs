use bevy::prelude::*;

use crate::{
    gui::{buttons::scroll::make_button, make_menu},
    lang::Lang,
};

use super::{
    fov::fov_update,
    lang::{lang_choose_buttons_update, lang_chooser},
    range::RangeSetting,
    Settings,
};

pub struct SettingsUiPlugin;
impl Plugin for SettingsUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (settings_button_interact).run_if(in_state(SettingsPageOpened::Closed)),
        )
        .add_systems(
            Update,
            (
                close_settings_button_interact,
                fov_update,
                lang_choose_buttons_update,
            )
                .run_if(not(in_state(SettingsPageOpened::Closed))),
        )
        .add_systems(OnEnter(SettingsPageOpened::Main), spawn_settings_menu)
        .add_systems(OnEnter(SettingsPageOpened::Closed), despawn_settings_menu)
        .add_state::<SettingsPageOpened>();
    }
}

#[derive(Component)]
struct SettingsButton;

#[derive(Component)]
struct CloseSettingsButton;

#[derive(Component)]
struct SettingsMenu;

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
enum SettingsPageOpened {
    Main,
    #[default]
    Closed,
}

pub fn settings_button(
    builder: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    lang: &Res<Lang>,
) {
    make_button(
        builder,
        lang.get("ui.settings.name"),
        SettingsButton,
        asset_server,
    )
}

fn settings_button_interact(
    query: Query<&Interaction, With<SettingsButton>>,
    mut settings_page_opened_state_set_next_state: ResMut<NextState<SettingsPageOpened>>,
) {
    if let Ok(interaction) = query.get_single() {
        if *interaction == Interaction::Pressed {
            settings_page_opened_state_set_next_state.set(SettingsPageOpened::Main)
        }
    }
}

fn close_settings_button_interact(
    query: Query<&Interaction, With<CloseSettingsButton>>,
    mut settings_page_opened_state_set_next_state: ResMut<NextState<SettingsPageOpened>>,
) {
    if let Ok(interaction) = query.get_single() {
        if *interaction == Interaction::Pressed {
            settings_page_opened_state_set_next_state.set(SettingsPageOpened::Closed)
        }
    }
}

fn spawn_settings_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<Settings>,
    lang: Res<Lang>,
) {
    make_menu(
        &mut commands,
        Color::BLACK.into(),
        SettingsMenu,
        |builder| {
            settings.fov.to_slider(builder);
            lang_chooser(builder, &settings.lang, &asset_server);
            make_button(
                builder,
                lang.get("ui.settings.close"),
                CloseSettingsButton,
                &asset_server,
            );
        },
        Some(ZIndex::Global(1)),
        None,
    );
}

fn despawn_settings_menu(mut commands: Commands, query: Query<Entity, With<SettingsMenu>>) {
    if let Ok(menu) = query.get_single() {
        commands.entity(menu).despawn_recursive();
    }
}
