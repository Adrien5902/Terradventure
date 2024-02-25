use crate::{
    gui::{buttons::scroll::make_button, make_menu, misc::PIXEL_FONT},
    save::{LoadSaveEvent, Save, SaveMetaData},
    state::AppState,
};
use bevy::prelude::*;

use super::MainMenuState;

pub struct LoadWorldMenuPlugin;
impl Plugin for LoadWorldMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (back_button, load_button)
                .run_if(in_state(AppState::MainMenu(MainMenuState::LoadWorld))),
        )
        .add_systems(
            OnEnter(AppState::MainMenu(MainMenuState::LoadWorld)),
            spawn_load_world_menu,
        )
        .add_systems(
            OnExit(AppState::MainMenu(MainMenuState::LoadWorld)),
            despawn_load_world_menu,
        );
    }
}

#[derive(Component)]
pub struct LoadWorldMenu;

#[derive(Component)]
pub struct LoadWorldBackButton;

#[derive(Component)]
pub struct LoadWorldButton {
    pub save_name: String,
}

fn spawn_load_world_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    make_menu(
        &mut commands,
        Color::BLACK.into(),
        LoadWorldMenu,
        |builder| {
            let saves = Save::get_saves();

            if saves.len() > 0 {
                builder
                    .spawn(NodeBundle {
                        style: Style {
                            height: Val::Percent(70.),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|builder| {
                        saves
                            .into_iter()
                            .for_each(|data| world_save_item(builder, data, &asset_server));
                    });
            } else {
                builder.spawn(TextBundle::from_section(
                    "No worlds saved",
                    TextStyle {
                        font_size: 24.0,
                        font: asset_server.load(PIXEL_FONT),
                        ..Default::default()
                    },
                ));
            }

            make_button(builder, "Back", LoadWorldBackButton, &asset_server);
        },
        None,
        None,
    )
}

fn world_save_item(
    builder: &mut ChildBuilder,
    data: Result<(String, SaveMetaData), String>,
    asset_server: &Res<AssetServer>,
) {
    builder
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(600.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                height: Val::Percent(15.),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|builder| {
            let text_color = match data {
                Ok(_) => Color::WHITE,
                Err(_) => Color::RED,
            };

            let world_name = match &data {
                Ok((_, metadata)) => Ok(metadata.name.clone()),
                Err(e) => Err(e.clone()),
            };

            builder.spawn(TextBundle::from_section(
                match world_name {
                    Ok(name) => name,
                    Err(err) => format!("World data corrupted : {}", err),
                },
                TextStyle {
                    font: asset_server.load(PIXEL_FONT),
                    font_size: 24.0,
                    color: text_color,
                },
            ));

            if let Ok((save_name, _)) = data {
                make_button(builder, "Load", LoadWorldButton { save_name }, asset_server)
            }
        });
}

fn back_button(
    query: Query<&Interaction, With<LoadWorldBackButton>>,
    mut state_change: ResMut<NextState<AppState>>,
) {
    for interaction in query.iter() {
        if *interaction == Interaction::Pressed {
            state_change.set(AppState::MainMenu(MainMenuState::Default))
        }
    }
}

fn load_button(
    query: Query<(&Interaction, &LoadWorldButton)>,
    mut state_change: ResMut<NextState<AppState>>,
    mut load_save_event: EventWriter<LoadSaveEvent>,
) {
    for (interaction, button) in query.iter() {
        if *interaction == Interaction::Pressed {
            state_change.set(AppState::InGame);
            load_save_event.send(LoadSaveEvent::new(
                &button.save_name,
                Save::read(&button.save_name).unwrap(),
            ));
        }
    }
}

fn despawn_load_world_menu(mut commands: Commands, menu_query: Query<Entity, With<LoadWorldMenu>>) {
    for menu in menu_query.iter() {
        commands.entity(menu).despawn_recursive();
    }
}
