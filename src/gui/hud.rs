use bevy::prelude::*;

use crate::{
    player::{
        inventory::ui::{display_item_stack, display_slots, InventorySlot, UpdateSlotEvent},
        Player,
    },
    save::LoadSaveEvent,
    state::AppState,
    stats::Stats,
};

use super::main_menu::MainMenuState;

#[derive(Component)]
pub struct Hud;

#[derive(Component)]
pub struct HudHeart {
    pub index: u32,
}

#[derive(Component)]
pub struct HudSlot;

pub struct HudPlugin;
impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::MainMenu(MainMenuState::Default)),
            despawn_hud,
        )
        .add_systems(OnEnter(AppState::InGame), spawn_hud)
        .add_systems(Update, update_hud.run_if(in_state(AppState::InGame)));
    }
}

fn spawn_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event: EventReader<LoadSaveEvent>,
) {
    for ev in event.read() {
        let player_data = &ev.read().player;
        let mut slots = Vec::new();

        commands
            .spawn(Hud)
            .insert(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexEnd,
                    ..Default::default()
                },
                background_color: Color::NONE.into(),
                ..Default::default()
            })
            .with_children(|builder| {
                for i in 0..(player_data.stats.max_health as u32 / 2) {
                    builder
                        .spawn(NodeBundle {
                            style: Style {
                                margin: UiRect::vertical(Val::Px(12.)),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|builder| {
                            builder.spawn(ImageBundle {
                                image: UiImage::new(
                                    asset_server.load("gui/hud/heart/container.png"),
                                ),
                                style: Style {
                                    width: Val::Px(InventorySlot::SIZE),
                                    ..Default::default()
                                },
                                ..Default::default()
                            });

                            builder
                                .spawn(ImageBundle {
                                    image: UiImage::new(
                                        asset_server.load("gui/hud/heart/full.png"),
                                    ),
                                    visibility: if player_data.stats.health > (i * 2) as f32 {
                                        Visibility::Visible
                                    } else {
                                        Visibility::Hidden
                                    },
                                    style: Style {
                                        width: Val::Px(InventorySlot::SIZE),
                                        left: Val::Px(0.),
                                        top: Val::Px(0.),
                                        position_type: PositionType::Absolute,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .insert(HudHeart { index: i });
                        });
                }

                slots = display_slots::<2>(
                    FlexDirection::Row,
                    builder,
                    "pockets",
                    &asset_server,
                    &player_data.player.inventory,
                    None,
                )
                .to_vec()
            });

        for slot in slots {
            commands.entity(slot).insert(HudSlot);
            // .add_child(commands.spawn(todo!()).id());
        }
    }
}

fn update_hud(
    mut commands: Commands,
    player_query: Query<&Stats, With<Player>>,
    mut update_slot_event: EventReader<UpdateSlotEvent>,
    slots_query: Query<(Entity, &InventorySlot), With<HudSlot>>,
    mut heart_query: Query<(&HudHeart, &mut Visibility, &mut UiImage)>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(stats) = player_query.get_single() {
        for ev in update_slot_event.read() {
            for (entity, slot) in slots_query.iter() {
                if ev.slot == *slot {
                    commands
                        .entity(entity)
                        .despawn_descendants()
                        .with_children(|builder| {
                            if let Some(item_stack) = &ev.new_item {
                                display_item_stack(builder, item_stack, &asset_server)
                            }
                        });
                }
            }
        }

        for (heart, mut visibility, mut image) in heart_query.iter_mut() {
            let cap = (heart.index * 2) as f32;
            *visibility = if stats.health > cap - 1. {
                image.texture = asset_server.load(format!(
                    "gui/hud/heart/{}.png",
                    if stats.health > cap { "full" } else { "half" }
                ));
                Visibility::Visible
            } else {
                Visibility::Hidden
            }
        }
    }
}

fn despawn_hud(mut commands: Commands, query: Query<Entity, With<Hud>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
