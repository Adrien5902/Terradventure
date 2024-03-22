use bevy::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::{
    gui::styles::{aligned_center, text_style},
    items::item::Item,
    lang::Lang,
    player::{
        inventory::ui::{spawn_inventory_ui, MoneyDisplay, UpdateSlotEvent},
        Player,
    },
    state::AppState,
};

use super::CurrentShop;

pub struct ShopUiPlugin;
impl Plugin for ShopUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, shop_ui_update.run_if(in_state(AppState::InGame)));
    }
}

#[derive(EnumIter, Display, Clone, Copy, PartialEq, Eq)]
pub enum ShopUiState {
    Buy,
    Sell,
}

#[derive(Component)]
pub struct ShopUi {
    pub state: ShopUiState,
}

#[derive(Component)]
pub struct ShopUiTabButton {
    pub to_state: ShopUiState,
}

#[derive(Component)]
pub struct ShopUiContainer {
    pub state: Option<ShopUiState>,
}

#[derive(Component)]
struct BuyButton {
    item_index: usize,
}

#[derive(Component)]
pub struct CloseShopButton;

fn shop_ui_update(
    mut commands: Commands,
    mut player_q: Query<(&mut Player, &Transform)>,
    mut current_shop: ResMut<CurrentShop>,
    mut shop_ui_q: Query<(Entity, &mut ShopUi)>,
    mut container_q: Query<(Entity, &mut ShopUiContainer)>,
    close_shop_q: Query<&Interaction, With<CloseShopButton>>,
    mut tab_button_q: Query<
        (&Interaction, &ShopUiTabButton, &mut BackgroundColor),
        Without<CloseShopButton>,
    >,
    buy_button_q: Query<(&Interaction, &BuyButton), Changed<Interaction>>,
    lang: Res<Lang>,
    asset_server: Res<AssetServer>,
    mut update_slot_event: EventWriter<UpdateSlotEvent>,
    mut money_display: Query<&mut Text, With<MoneyDisplay>>,
) {
    let Ok((mut player, transform)) = player_q.get_single_mut() else {
        return;
    };

    let Ok((entity, mut shop_ui)) = shop_ui_q.get_single_mut() else {
        //Spawn shop
        if let Some(_shop) = &mut current_shop.shop {
            commands
                .spawn(ShopUi {
                    state: ShopUiState::Buy,
                })
                .insert(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        ..aligned_center()
                    },
                    z_index: ZIndex::Global(13), // See [`DialogUi`] ZIndex = 12
                    background_color: Color::BLACK.into(),
                    ..Default::default()
                })
                .with_children(|builder| {
                    //Close button
                    builder
                        .spawn(ButtonBundle {
                            background_color: Color::NONE.into(),
                            style: Style {
                                position_type: PositionType::Absolute,
                                top: Val::Percent(4.),
                                right: Val::Percent(4.),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|builder| {
                            builder.spawn(TextBundle::from_section(
                                "X",
                                TextStyle {
                                    font_size: 40.,
                                    ..text_style(&asset_server)
                                },
                            ));
                        })
                        .insert(CloseShopButton);

                    //Shop Part
                    builder
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                margin: UiRect::all(Val::Px(20.)),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|builder| {
                            builder
                                .spawn(NodeBundle {
                                    style: Style {
                                        display: Display::Flex,
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::Center,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .with_children(|builder| {
                                    for tab in ShopUiState::iter() {
                                        builder
                                            .spawn(ButtonBundle {
                                                background_color: Color::GRAY.into(),
                                                style: Style {
                                                    margin: UiRect::horizontal(Val::Percent(1.)),
                                                    padding: UiRect::all(Val::Px(16.)),
                                                    ..Default::default()
                                                },
                                                ..Default::default()
                                            })
                                            .with_children(|builder| {
                                                builder.spawn(TextBundle::from_section(
                                                    lang.get(&format!(
                                                        "ui.shop.{}",
                                                        tab.to_string().to_lowercase()
                                                    )),
                                                    text_style(&asset_server),
                                                ));
                                            })
                                            .insert(ShopUiTabButton { to_state: tab });
                                    }
                                });

                            builder
                                .spawn(ShopUiContainer { state: None })
                                .insert(NodeBundle {
                                    style: Style {
                                        display: Display::Flex,
                                        flex_direction: FlexDirection::Column,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                });
                        });

                    //Player's inv
                    spawn_inventory_ui(builder, &asset_server, &player)
                });
        }
        return;
    };

    let Some(shop) = &mut current_shop.shop else {
        //Despawn shop
        commands.entity(entity).despawn_recursive();
        return;
    };

    //Update Shop
    match close_shop_q.single() {
        Interaction::Pressed => {
            current_shop.shop = None;
            return;
        }
        _ => {}
    }

    for (interaction, tab_button, mut background_color) in tab_button_q.iter_mut() {
        *background_color = if shop_ui.state == tab_button.to_state {
            Color::GRAY
        } else {
            Color::DARK_GRAY
        }
        .into();

        if *interaction == Interaction::Pressed {
            shop_ui.state = tab_button.to_state
        }
    }

    let (container_entity, mut container) = container_q.single_mut();

    if Some(shop_ui.state) != container.state {
        let mut container_commands = commands.entity(container_entity);
        container_commands.despawn_descendants();
        container.state = Some(shop_ui.state);

        match shop_ui.state {
            ShopUiState::Buy => {
                for (index, item) in shop.solds.iter().enumerate() {
                    container_commands.with_children(|builder| {
                        builder
                            .spawn(NodeBundle {
                                background_color: Color::DARK_GRAY.into(),
                                style: Style {
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Row,
                                    justify_content: JustifyContent::SpaceBetween,
                                    align_items: AlignItems::Center,
                                    margin: UiRect::all(Val::Percent(1.)),
                                    width: Val::Vw(40.),
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .with_children(|builder| {
                                builder
                                    .spawn(NodeBundle {
                                        style: Style {
                                            align_items: AlignItems::Center,
                                            margin: UiRect::bottom(Val::Px(4.)),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    })
                                    .with_children(|builder| {
                                        builder.spawn(ImageBundle {
                                            image: UiImage::new(
                                                asset_server.load(item.stack.item.texture()),
                                            ),
                                            style: Style {
                                                height: Val::Px(52.),
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        });

                                        builder
                                            .spawn(NodeBundle {
                                                style: Style {
                                                    display: Display::Flex,
                                                    flex_direction: FlexDirection::Column,
                                                    ..Default::default()
                                                },
                                                ..Default::default()
                                            })
                                            .with_children(|builder| {
                                                builder.spawn(TextBundle::from_section(
                                                    format!(
                                                        "{} x{}",
                                                        item.stack.item.name().get(),
                                                        item.stack.actual_count()
                                                    ),
                                                    text_style(&asset_server),
                                                ));

                                                builder
                                                    .spawn(NodeBundle {
                                                        ..Default::default()
                                                    })
                                                    .with_children(|builder| {
                                                        builder.spawn(ImageBundle {
                                                            image: UiImage::new(
                                                                asset_server.load("gui/coin.png"),
                                                            ),
                                                            ..Default::default()
                                                        });
                                                        builder.spawn(TextBundle::from_section(
                                                            item.price.to_string(),
                                                            text_style(&asset_server),
                                                        ));
                                                    });
                                            });
                                    });

                                builder
                                    .spawn(BuyButton { item_index: index })
                                    .insert(ButtonBundle {
                                        border_color: BorderColor(Color::WHITE),
                                        background_color: Color::DARK_GREEN.into(),
                                        style: Style {
                                            height: Val::Percent(100.),
                                            border: UiRect::all(Val::Px(1.)),
                                            ..aligned_center()
                                        },
                                        ..Default::default()
                                    })
                                    .with_children(|builder| {
                                        builder.spawn(TextBundle::from_section(
                                            lang.get("ui.shop.buy"),
                                            text_style(&asset_server),
                                        ));
                                    });
                            });
                    });
                }
            }

            ShopUiState::Sell => {}
        }
    }

    for (interaction, button) in buy_button_q.iter() {
        if *interaction == Interaction::Pressed {
            let item = &shop.solds[button.item_index];
            let mut item_stack = Some(item.stack.clone());

            if player.money.try_remove(item.price) {
                player
                    .inventory
                    .push_item_stack(&mut item_stack, &mut update_slot_event);

                if let Ok(mut text) = money_display.get_single_mut() {
                    text.sections[0].value = player.money.get().to_string();
                }

                if let Some(remaining_item_stack) = item_stack {
                    commands.spawn(
                        remaining_item_stack.bundle(&asset_server, transform.translation.xy()),
                    );
                }
            }
        }
    }
}
