use bevy::prelude::*;

use crate::{
    gui::{settings::Settings, styles::text_style},
    lang::Lang,
    player::Player,
    state::AppState,
    world::BLOCK_SIZE,
};

pub struct InteractionPlugin;
impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, interactions.run_if(in_state(AppState::InGame)));
    }
}

#[derive(Component)]
pub struct Interactable {
    pub message: String,
    just_pressed: bool,
}

impl Default for Interactable {
    fn default() -> Self {
        Self {
            message: "player.actions.interact".into(),
            just_pressed: false,
        }
    }
}

impl Interactable {
    const MAX_DIST: f32 = BLOCK_SIZE * 3.;

    pub fn new(message: &str) -> Self {
        Self {
            message: message.into(),
            ..Default::default()
        }
    }

    pub fn just_pressed(&self) -> bool {
        self.just_pressed
    }
}

#[derive(Component)]
pub struct InteractionText;

fn interactions(
    mut commands: Commands,
    settings: Res<Settings>,
    player_query: Query<&Transform, With<Player>>,
    mut query: Query<(Entity, &mut Interactable, &Transform)>,
    children_query: Query<&Children>,
    text_query: Query<Entity, With<InteractionText>>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    asset_server: Res<AssetServer>,
    lang: Res<Lang>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_pos = player_transform.translation.xy();
        let mut close_interactable = query
            .iter_mut()
            .filter_map(|(entity, mut interaction, transform)| {
                interaction.just_pressed = false;
                let dist = transform.translation.xy().distance(player_pos);
                (dist < Interactable::MAX_DIST).then_some((dist, entity, interaction, transform))
            })
            .collect::<Vec<_>>();

        if !close_interactable.is_empty() {
            close_interactable.sort_by(|a, b| a.0.total_cmp(&b.0));

            let (_, closest_entity, closest, _) = &mut close_interactable[0];

            let mut spawn_text = || {
                let child = commands
                    .spawn(InteractionText)
                    .insert(Text2dBundle {
                        transform: Transform {
                            translation: Vec3::new(0.0, BLOCK_SIZE / 2., 1.0),
                            scale: Vec3::splat(0.2),
                            ..Default::default()
                        },
                        text: Text::from_section(
                            lang.get(&closest.message),
                            text_style(&asset_server),
                        ),
                        ..Default::default()
                    })
                    .id();
                commands.entity(*closest_entity).add_child(child);
            };

            if let Ok(closest_children) = children_query.get(*closest_entity) {
                let has_no_text = closest_children
                    .iter()
                    .map(|child| text_query.get(*child).is_err())
                    .reduce(|a, b| a && b)
                    .unwrap_or(true);

                if has_no_text {
                    spawn_text()
                }

                if !text_query.is_empty() {
                    for entity in text_query.iter() {
                        if !closest_children.contains(&entity) {
                            commands.entity(entity).despawn_recursive();
                        }
                    }
                }
            } else {
                spawn_text()
            }

            if settings
                .keybinds
                .interact
                .just_pressed(&keyboard_input, &mouse_input)
            {
                closest.just_pressed = true
            }
        } else {
            for entity in text_query.iter() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
