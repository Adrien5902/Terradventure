pub mod class;
pub mod inventory;
pub mod mana;
pub mod money;

use std::path::PathBuf;
use std::time::Duration;

use self::class::{PlayerClass, PlayerClasses, PlayerClassesPlugin};
use self::inventory::{Inventory, InventoryPlugin};
use self::mana::Mana;
use self::money::{Money, MoneyPlugin};
use crate::animation::{
    AnimatedSpriteBundle, Animation, AnimationController, AnimationDirection, AnimationMode,
};
use crate::animation_maker;
use crate::effects::{Effect, EffectsController, EffectsPlugin};
use crate::gui::{
    misc::ease_out_quad,
    settings::{fov::FOV_MULTIPLIER, range::RangeSetting, Settings},
};
use crate::items::item::ItemPlugin;
use crate::lang::Lang;
use crate::mob::Mob;
use crate::npc::dialog::in_dialog;
use crate::save::LoadSaveEvent;
use crate::state::AppState;
use crate::stats::Stats;
use crate::world::{is_loading, BLOCK_SIZE};
use bevy::sprite::Anchor;
use bevy::{prelude::*, utils::HashMap};
use bevy_rapier2d::prelude::*;
use serde::{Deserialize, Serialize};

pub const PLAYER_SPRITE_SHEETS_X_SIZE: u32 = 128;

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct Player {
    pub class: PlayerClasses,
    pub inventory: Inventory,
    jump_timer: Timer,
    pub money: Money,
    #[serde(skip)]
    chain_attack: ChainAttack,
    pub mana: Mana,
}

impl Player {
    pub const SPRITE_ANCHOR: Anchor = Anchor::Custom(Vec2::new(0.0, -0.2));
    pub const EXTEND: f32 = 10.0;
    pub const SIZE: f32 = 96.0;
}

#[derive(Component, Clone)]
pub struct ChainAttack {
    end_at: u8,
    pub timer: Timer,
    count: u8,
    pub registered_next: bool,
}

impl Default for ChainAttack {
    fn default() -> Self {
        Self {
            end_at: Self::DEFAULT,
            timer: Timer::from_seconds(1., TimerMode::Once),
            count: 0,
            registered_next: false,
        }
    }
}

impl ChainAttack {
    const DEFAULT: u8 = 1;

    fn with_max(mut self, max: u8) -> Self {
        self.end_at = max;
        self
    }

    fn get(&mut self) -> u8 {
        let count = if self.timer.finished() {
            Self::DEFAULT
        } else {
            self.count = if self.count >= self.end_at {
                Self::DEFAULT
            } else {
                self.count + 1
            };
            self.count
        };
        self.timer.reset();
        count
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            jump_timer: Timer::from_seconds(0.12, TimerMode::Once),
            chain_attack: ChainAttack::default(),
            inventory: Inventory::default(),
            class: PlayerClasses::default(),
            money: Money::default(),
            mana: Mana::default(),
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                character_controller_update.run_if(not(is_loading).and_then(not(in_dialog))),
                player_setup,
            )
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            OnEnter(AppState::MainMenu(
                crate::gui::main_menu::MainMenuState::Default,
            )),
            despawn_player,
        )
        .add_systems(Startup, spawn_camera)
        .add_plugins((
            InventoryPlugin,
            ItemPlugin,
            PlayerClassesPlugin,
            MoneyPlugin,
            EffectsPlugin,
        ));
    }
}

pub const PLAYER_TEXTURE: &str = "textures/player";

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    animated_sprite: AnimatedSpriteBundle,
    rigid_body: RigidBody,
    controller: KinematicCharacterController,
    collider: Collider,
    stats: Stats,
    effects_controller: EffectsController,
}

fn player_setup(
    mut commands: Commands,
    mut event: EventReader<LoadSaveEvent>,
    lang: Res<Lang>,
    asset_server: Res<AssetServer>,
    camera_query: Query<Entity, With<Camera>>,
) {
    for ev in event.read() {
        let save = ev.read();

        let world = save.world.clone();
        world.spawn(&mut commands, &asset_server, &lang, camera_query.single());

        let controller: KinematicCharacterController = KinematicCharacterController {
            autostep: Some(CharacterAutostep {
                min_width: CharacterLength::Relative(0.0),
                max_height: CharacterLength::Relative(0.3),
                include_dynamic_bodies: false,
            }),
            ..Default::default()
        };

        let mut player = save.player.player.clone();
        let mut transform = Transform::from_translation(save.player.pos.extend(Player::EXTEND));
        transform.translation.y += BLOCK_SIZE;

        let get_texture_path = |name: &str| -> PathBuf { player.class.get_texture_path(name) };

        let mut player_animations = animation_maker!(&asset_server, get_texture_path, PLAYER_SPRITE_SHEETS_X_SIZE, [
            "Idle" => (1., AnimationMode::Repeating, AnimationDirection::BackAndForth),
            // "Idle_2" => (3.0, 3, AnimationMode::Once, AnimationDirection::Forwards),
            "Take" => (0.4, AnimationMode::Once, AnimationDirection::Forwards),
            "Elixir" => (1.0, AnimationMode::Once, AnimationDirection::Forwards),
            "Walk" => (1., AnimationMode::Custom, AnimationDirection::Forwards),
            "Jump" => (0.3, AnimationMode::Once, AnimationDirection::Forwards),
            "Dead" => (0.3, AnimationMode::Once, AnimationDirection::Forwards)
        ]);

        player_animations.extend(player.class.class_animations(&asset_server));

        for i in 1..=player.class.normal_attack_chain_count().into() {
            let name = format!("Attack_{}", i);
            player_animations.insert(
                name.clone(),
                Animation::new(
                    get_texture_path(&name),
                    &asset_server,
                    Duration::from_secs_f32(0.5),
                    PLAYER_SPRITE_SHEETS_X_SIZE,
                    AnimationMode::Once,
                    AnimationDirection::Forwards,
                ),
            );
        }

        player.chain_attack = player
            .chain_attack
            .with_max(player.class.normal_attack_chain_count());

        commands.spawn(PlayerBundle {
            player,
            animated_sprite: AnimatedSpriteBundle {
                sprite: SpriteSheetBundle {
                    transform,
                    sprite: TextureAtlasSprite {
                        anchor: Player::SPRITE_ANCHOR,
                        custom_size: Some(Vec2::splat(Player::SIZE)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                animation_controller: AnimationController::new(player_animations)
                    .with_default("Idle"),
            },
            collider: Collider::capsule_y(13.5, 13.5),
            controller,
            rigid_body: RigidBody::KinematicPositionBased,
            stats: Stats::default().with_health(20.0),
            effects_controller: EffectsController::default(),
        });
    }
}

fn despawn_player(mut commands: Commands, query: Query<Entity, With<Player>>) {
    if let Ok(player) = query.get_single() {
        commands.entity(player).despawn();
    }
}

pub fn character_controller_update(
    keyboard: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    time: Res<Time>,
    output_query: Query<&KinematicCharacterControllerOutput, With<Player>>,
    mut query: Query<(
        Entity,
        &mut TextureAtlasSprite,
        &mut AnimationController,
        &mut Transform,
        &mut KinematicCharacterController,
        &Stats,
        &mut Player,
        &EffectsController,
    )>,
    mut mob_query: Query<(&mut Stats, &mut Mob), Without<Player>>,
    rapier_context: Res<RapierContext>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    settings: Res<Settings>,
) {
    for (
        entity,
        mut sprite,
        mut animation_controller,
        mut transform,
        mut controller,
        stats,
        mut player,
        effects,
    ) in query.iter_mut()
    {
        player.mana.tick(&time);

        let mut direction = Vec2::default();
        let keybinds = &settings.keybinds;

        if keybinds.move_left.pressed(&keyboard, &mouse) {
            direction.x -= 1.0;
            animation_controller.tick(&time)
        }

        if keybinds.move_right.pressed(&keyboard, &mouse) {
            direction.x += 1.0;
            animation_controller.tick(&time)
        }

        if let Ok(output) = output_query.get_single() {
            if output.grounded && keybinds.jump.just_pressed(&keyboard, &mouse) {
                player.jump_timer.reset();
                player.jump_timer.unpause();
                animation_controller.play("Jump");
            } else if player.jump_timer.finished() || player.jump_timer.paused() {
                direction.y -= 1.0;
            }
        } else {
            direction.y -= 1.0;
        }

        if let Some(data) = effects.get_effect(&Effect::Levitation) {
            direction.y = data.level as f32 + 1.;
        }

        //Prevent from dropping in the void
        if transform.translation.y < BLOCK_SIZE * -30. {
            transform.translation.y = BLOCK_SIZE * 30.
        }

        if player.jump_timer.finished() {
            player.jump_timer.pause()
        }

        if !player.jump_timer.paused() {
            player.jump_timer.tick(time.delta());

            //Jump impulsion
            direction.y += 2.5 * ease_out_quad(player.jump_timer.percent());
        }

        direction.x *= stats.speed * time.delta_seconds();
        direction.y *= stats.mass * time.delta_seconds();

        controller.translation = Some(direction);

        let animating = animation_controller.current_animation.is_some();
        let moving_x = direction.x != 0.;

        if moving_x {
            sprite.flip_x = direction.x < 0.;

            if !animating {
                animation_controller.play("Walk");
            }
        } else if animation_controller.current_animation == Some("Walk".to_owned()) {
            animation_controller.stop();
        }

        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            camera_transform.translation = transform.translation;
        }

        player.chain_attack.timer.tick(time.delta());

        if !animation_controller
            .current_animation
            .as_ref()
            .is_some_and(|anim| anim.contains("Attack"))
        {
            if keybinds.attack.just_pressed(&keyboard, &mouse)
                || player.chain_attack.registered_next
            {
                let count = player.chain_attack.get();
                player.chain_attack.registered_next = false;
                animation_controller.play(&format!("Attack_{}", count));
            }
        } else if keybinds.attack.just_pressed(&keyboard, &mouse) {
            player.chain_attack.registered_next = true;
        }

        if let Some(name) = &animation_controller.just_finished {
            if name.starts_with("Attack") {
                player.chain_attack.timer.reset();

                let mut hitbox_translation = transform.translation.xy();
                hitbox_translation += sprite_vec(&sprite) * BLOCK_SIZE;
                cast_collider(
                    entity,
                    &Collider::ball(BLOCK_SIZE),
                    hitbox_translation,
                    &rapier_context,
                    |hit_entity| {
                        if let Ok((mut stats, mut mob)) = mob_query.get_mut(hit_entity) {
                            stats.take_damage(3.);
                            mob.hit_animation();
                        }
                        true
                    },
                );
            }
        }
    }
}

pub fn flip_direction(flipped: bool) -> f32 {
    if flipped {
        -1.
    } else {
        1.
    }
}

pub fn sprite_vec(sprite: &TextureAtlasSprite) -> Vec2 {
    Vec2::new(flip_direction(sprite.flip_x), 0.0)
}

fn spawn_camera(mut commands: Commands, settings: Res<Settings>) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            far: 2000.,
            near: -2000.,
            scale: settings.fov.get_value() * FOV_MULTIPLIER,
            ..Default::default()
        },
        ..Default::default()
    });
}

pub fn cast_collider(
    player_entity: Entity,
    collider: &Collider,
    shape_pos: Vec2,
    rapier_context: &Res<RapierContext>,
    callback: impl FnMut(Entity) -> bool,
) {
    rapier_context.intersections_with_shape(
        shape_pos,
        0.0,
        collider,
        QueryFilter {
            exclude_rigid_body: Some(player_entity),
            ..Default::default()
        },
        callback,
    );
}
