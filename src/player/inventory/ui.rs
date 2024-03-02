use bevy::prelude::*;

use crate::{
    gui::{make_menu, settings::Settings, styles::text_style},
    items::{item::Item, stack::ItemStack},
    player::{class::PlayerClass, Player},
    state::AppState,
    world::BLOCK_SIZE,
};

use super::{Inventory, Slot};

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
            .add_systems(
                Update,
                slot_interaction.run_if(in_state(InventoryUiState::Opened)),
            )
            .insert_resource(MovingStack(None))
            .add_systems(Update, inventory_toggle.run_if(in_state(AppState::InGame)));
    }
}

#[derive(Component)]
struct InventoryUi;

#[derive(Resource)]
struct MovingStack(pub Option<ItemStack>);

fn spawn_inventory(
    mut commands: Commands,
    player_query: Query<&Player>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(player) = player_query.get_single() {
        let inventory = &player.inventory;

        //Overlay darken bg
        make_menu(
            &mut commands,
            Color::BLACK.with_a(0.5).into(),
            InventoryUi,
            |builder| {
                // Inventory menu
                builder
                    .spawn(NodeBundle {
                        background_color: Color::DARK_GRAY.into(),
                        style: Style {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            margin: UiRect::all(Val::Px(8.)),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|builder| {
                        //Upper part
                        builder
                            .spawn(NodeBundle {
                                style: Style {
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Row,
                                    justify_content: JustifyContent::SpaceBetween,
                                    align_items: AlignItems::Center,
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .with_children(|builder| {
                                //Accessories
                                slots::<2>(
                                    FlexDirection::Column,
                                    builder,
                                    "accessories",
                                    &asset_server,
                                    inventory,
                                    None,
                                );

                                //Player preview
                                builder.spawn(ImageBundle {
                                    image: UiImage::from(
                                        asset_server.add(player.class.idle_texture()),
                                    ),
                                    style: Style {
                                        height: Val::Px(160.),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                });

                                //Armor
                                slots::<4>(
                                    FlexDirection::Column,
                                    builder,
                                    "armor",
                                    &asset_server,
                                    inventory,
                                    None,
                                );

                                //Slots
                                slots::<2>(
                                    FlexDirection::Column,
                                    builder,
                                    "pockets",
                                    &asset_server,
                                    inventory,
                                    None,
                                );
                            });

                        //Down part : Ressources slots
                        slots::<27>(
                            FlexDirection::Row,
                            builder,
                            "ressources",
                            &asset_server,
                            inventory,
                            Some((FlexDirection::Column, 9)),
                        )
                    });
            },
            None,
            Some(FlexDirection::Column),
        );
    }
}

const SLOT_BG_COLOR: Color = Color::GRAY;

fn slot<const COUNT: usize>(
    builder: &mut ChildBuilder,
    slot_index: usize,
    typ: &str,
    asset_server: &Res<AssetServer>,
    inventory: &Inventory,
) {
    builder
        .spawn(InventorySlot {
            slot_index,
            typ: typ.into(),
        })
        .insert(ButtonBundle {
            border_color: BorderColor(Color::WHITE),
            background_color: SLOT_BG_COLOR.into(),
            style: Style {
                width: Val::Px(InventorySlot::SIZE),
                height: Val::Px(InventorySlot::SIZE),
                border: UiRect::all(Val::Px(2.)),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|builder| {
            let slot = &inventory.get_field::<[Slot; COUNT]>(typ).unwrap()[slot_index];

            if let Some(item_stack) = &slot.item {
                display_item_stack(builder, item_stack, asset_server);
            }
        });
}

fn display_item_stack(
    builder: &mut ChildBuilder,
    item_stack: &ItemStack,
    asset_server: &Res<AssetServer>,
) {
    let texture = item_stack.item.texture();
    builder.spawn(ImageBundle {
        image: asset_server.load(texture).into(),
        ..Default::default()
    });

    if item_stack.count > 0 {
        builder.spawn(TextBundle {
            text: Text::from_section(
                (item_stack.count as u16 + 1).to_string(),
                text_style(&asset_server),
            ),
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(4.),
                bottom: Val::Px(4.),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}

fn slots<const COUNT: usize>(
    direction: FlexDirection,
    builder: &mut ChildBuilder,
    field: &str,
    asset_server: &Res<AssetServer>,
    inventory: &Inventory,
    split: Option<(FlexDirection, usize)>,
) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: split.map(|s| s.0).unwrap_or(direction),
                margin: UiRect::all(Val::Px(12.)),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|builder| {
            if let Some((_, by_row_count)) = split {
                for y in (0..(COUNT / by_row_count)).rev() {
                    builder
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                flex_direction: direction,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|builder| {
                            for i in 0..by_row_count {
                                slot::<COUNT>(
                                    builder,
                                    y * by_row_count + i,
                                    field,
                                    &asset_server,
                                    inventory,
                                );
                            }
                        });
                }
            } else {
                for i in 0..COUNT {
                    slot::<COUNT>(builder, i, field, &asset_server, inventory);
                }
            }
        });
}

#[derive(Component)]
pub struct InventorySlot {
    pub typ: String,
    pub slot_index: usize,
}

impl InventorySlot {
    const SIZE: f32 = 40.;
}

fn inventory_toggle(
    keyboard: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    settings: Res<Settings>,
    state: Res<State<InventoryUiState>>,
    mut set_state: ResMut<NextState<InventoryUiState>>,
) {
    if settings.keybinds.inventory.just_pressed(&keyboard, &mouse) {
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
    mut moving_stack_res: ResMut<MovingStack>,
    mouse_moving_stack_query: Query<Entity, With<MouseMovingStack>>,
    mut set_state: ResMut<NextState<InventoryUiState>>,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    for ui in inventory_ui_query.iter() {
        commands.entity(ui).despawn_recursive();
        set_state.set(InventoryUiState::Closed)
    }

    if let Ok(transform) = player_query.get_single() {
        let taken = std::mem::take(&mut moving_stack_res.0);
        if let Some(item_stack) = taken {
            commands.spawn(item_stack.bundle(&asset_server, transform.translation.xy()));
        }
    }

    for mouse_moving_stack in mouse_moving_stack_query.iter() {
        commands.entity(mouse_moving_stack).despawn_recursive();
    }
}

#[derive(Component)]
pub struct MouseMovingStack;

fn slot_interaction(
    mut commands: Commands,
    mut mouse_moving_stack_query: Query<(Entity, &mut Style), With<MouseMovingStack>>,
    mut player_query: Query<&mut Player>,
    mut moving_stack_res: ResMut<MovingStack>,
    mut query: Query<
        (Entity, &InventorySlot, &Interaction, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    asset_server: Res<AssetServer>,
    windows: Query<&Window>,
) {
    if let Some(moving_stack) = &moving_stack_res.0 {
        if let Ok((_, mut style)) = mouse_moving_stack_query.get_single_mut() {
            if let Some(position) = windows.single().cursor_position() {
                style.left = Val::Px(position.x - (InventorySlot::SIZE / 2.));
                style.top = Val::Px(position.y - (InventorySlot::SIZE / 2.));
            }
        } else if let Some(position) = windows.single().cursor_position() {
            let left = Val::Px(position.x - (InventorySlot::SIZE / 2.));
            let top = Val::Px(position.y - (InventorySlot::SIZE / 2.));

            commands
                .spawn(MouseMovingStack)
                .insert(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Val::Px(InventorySlot::SIZE),
                        height: Val::Px(InventorySlot::SIZE),
                        left,
                        top,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|builder| display_item_stack(builder, &moving_stack, &asset_server));
        }
    } else {
        if let Ok((entity, _)) = mouse_moving_stack_query.get_single() {
            commands.entity(entity).despawn_recursive();
        }
    }

    if let Ok(mut player) = player_query.get_single_mut() {
        let inventory = &mut player.inventory;
        for (entity, slot, interaction, mut bg_color) in query.iter_mut() {
            match *interaction {
                Interaction::Pressed => {
                    let item = &mut inventory.get_slot_mut(&slot.typ, slot.slot_index).item;

                    std::mem::swap::<Option<ItemStack>>(item, &mut moving_stack_res.0);

                    commands
                        .entity(entity)
                        .despawn_descendants()
                        .with_children(|builder| {
                            if let Some(item_stack) = &item {
                                display_item_stack(builder, item_stack, &asset_server)
                            }
                        });
                }
                Interaction::Hovered => {
                    *bg_color = Color::WHITE.with_a(0.6).into();
                }
                Interaction::None => {
                    *bg_color = SLOT_BG_COLOR.into();
                }
            }
        }
    }
}
