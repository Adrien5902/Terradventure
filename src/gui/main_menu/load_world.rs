use crate::{
    gui::{buttons::scroll::make_button, make_menu, styles::text_style},
    lang::Lang,
    save::{Save, SaveData, SaveMetaData},
    state::AppState,
};
use bevy::prelude::*;

use super::MainMenuState;

pub struct LoadWorldMenuPlugin;
impl Plugin for LoadWorldMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (back_button, load_button, delete_button)
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

#[derive(Component)]
pub struct DeleteWorldButton {
    pub save_name: String,
}

fn spawn_load_world_menu(mut commands: Commands, asset_server: Res<AssetServer>, lang: Res<Lang>) {
    make_menu(
        &mut commands,
        Color::BLACK.into(),
        LoadWorldMenu,
        |builder| {
            let saves = Save::get_saves();

            if !saves.is_empty() {
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
                            .for_each(|data| world_save_item(builder, data, &asset_server, &lang));
                    });
            } else {
                builder.spawn(TextBundle::from_section(
                    lang.get("ui.main_menu.load_save.none"),
                    text_style(&asset_server),
                ));
            }

            make_button(
                builder,
                lang.get("ui.main_menu.load_save.back"),
                LoadWorldBackButton,
                &asset_server,
            );
        },
        None,
        None,
    )
}

fn world_save_item(
    builder: &mut ChildBuilder,
    data: Result<(String, SaveMetaData), String>,
    asset_server: &Res<AssetServer>,
    lang: &Res<Lang>,
) {
    builder
        .spawn(NodeBundle {
            style: Style {
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
                    Err(err) => format!(
                        "{} : {}",
                        lang.get("ui.main_menu.load_save.err.corrupted"),
                        lang.get(&err)
                    ),
                },
                TextStyle {
                    color: text_color,
                    ..text_style(asset_server)
                },
            ));

            if let Ok((save_name, _)) = data {
                make_button(
                    builder,
                    lang.get("ui.main_menu.load_save.load"),
                    LoadWorldButton {
                        save_name: save_name.clone(),
                    },
                    asset_server,
                );

                make_button(
                    builder,
                    lang.get("ui.main_menu.load_save.delete"),
                    DeleteWorldButton { save_name },
                    asset_server,
                );
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
    query: Query<(&Interaction, &LoadWorldButton, &Children)>,
    mut text_query: Query<&mut Text>,
    mut state_change: ResMut<NextState<AppState>>,
    mut load_save_event: EventWriter<SaveData>,
) {
    for (interaction, button, children) in query.iter() {
        if *interaction == Interaction::Pressed {
            match Save::read(&button.save_name) {
                Ok(save) => {
                    state_change.set(AppState::InGame);
                    load_save_event.send(SaveData::new(&button.save_name, save));
                }
                Err(e) => {
                    for child in children {
                        if let Ok(mut text) = text_query.get_mut(*child) {
                            text.sections[0].style.color = Color::RED;
                        }
                    }
                    error!(e)
                }
            }
        }
    }
}

fn delete_button(
    mut commands: Commands,
    query: Query<(&Parent, &Interaction, &DeleteWorldButton)>,
) {
    for (parent, interaction, button) in query.iter() {
        if *interaction == Interaction::Pressed && Save::delete(&button.save_name).is_ok() {
            commands.entity(parent.get()).despawn_recursive();
        }
    }
}

fn despawn_load_world_menu(mut commands: Commands, menu_query: Query<Entity, With<LoadWorldMenu>>) {
    for menu in menu_query.iter() {
        commands.entity(menu).despawn_recursive();
    }
}
