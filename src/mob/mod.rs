pub mod list;

use crate::{
    animation::{AnimatedSpriteBundle, Animation, AnimationController},
    items::{loot_table::LootTable, stack::ItemStack},
    save::SaveData,
    state::AppState,
    stats::Stats,
    world::{is_loading, BLOCK_SIZE},
};
use bevy::{prelude::*, sprite::Anchor, utils::hashbrown::HashMap};
use bevy_rapier2d::prelude::*;
use enum_dispatch::enum_dispatch;
use rand::random;
use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use self::list::{pig::Pig, rabbit::Rabbit, MobObject};

pub struct MobPlugin;
impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_ai.run_if(not(is_loading)), mob_hit).run_if(in_state(AppState::InGame)),
        )
        .add_systems(OnEnter(AppState::InGame), load_mobs);
    }
}

fn update_ai(
    mut query: Query<(
        &mut Mob,
        &mut KinematicCharacterController,
        &Transform,
        &mut Stats,
        &mut TextureAtlasSprite,
        &mut AnimationController,
    )>,
    time: Res<Time>,
) {
    for (mut mob, mut controller, transform, mut stats, mut sprite, mut animation_controller) in
        query.iter_mut()
    {
        mob.ai.update(
            transform,
            &mut controller,
            &mut animation_controller,
            &mut sprite,
            &mut stats,
            &time,
        );
    }
}

#[derive(Component)]
pub struct Mob {
    pub hit_timer: Timer,
    pub typ: MobType,
    pub death_loot_table: MobLootTable,
    pub ai: Box<dyn MobAi>,
}

impl Mob {
    pub fn new(typ: MobType, death_loot_table: MobLootTable) -> Self {
        let mut hit_timer = Timer::from_seconds(0.3, TimerMode::Once);
        hit_timer.pause();
        Self {
            hit_timer,
            ai: typ.clone().into(),
            typ,
            death_loot_table,
        }
    }

    pub fn hit_animation(&mut self) {
        self.hit_timer.reset();
        self.hit_timer.unpause();
    }

    pub fn get_loot(&self) -> (u64, Vec<ItemStack>) {
        let path: PathBuf = self.death_loot_table.into();
        let loot_table_res = LootTable::read(&path);
        if let Some(loot_table) = loot_table_res {
            loot_table.get_random()
        } else {
            (0, vec![])
        }
    }
}

#[derive(Bundle)]
pub struct MobBundle {
    object: MobObject,
    mob: Mob,
    stats: Stats,
    sprite: AnimatedSpriteBundle,
    collider: Collider,
    rigid_body: RigidBody,
    controller: KinematicCharacterController,
    mass: ColliderMassProperties,
}

#[derive(Clone)]
pub enum MobType {
    Passive,
    Neutral,
    Aggressive,
}

impl From<MobType> for Box<dyn MobAi> {
    fn from(value: MobType) -> Self {
        Box::new(match value {
            MobType::Passive => PassiveDefaultMobAi::default(),
            _ => PassiveDefaultMobAi::default(),
        })
    }
}

pub trait MobAi: Sync + Send {
    fn update(
        &mut self,
        transform: &Transform,
        controller: &mut KinematicCharacterController,
        animation_controller: &mut AnimationController,
        sprite: &mut TextureAtlasSprite,
        stats: &mut Stats,
        time: &Res<Time>,
    );
}

#[derive(Default)]
pub struct PassiveDefaultMobAi {
    /// time until next wander (if wandering => timeout)
    pub wander_timer: Timer,

    /// x coord of the destination, None if not wandering
    pub wandering_destination: Option<f32>,
}

impl PassiveDefaultMobAi {
    const MAX_WANDER_DISTANCE: f32 = 15.;
    const WANDERING_TIMEOUT: f32 = 10.;
}

impl MobAi for PassiveDefaultMobAi {
    fn update(
        &mut self,
        transform: &Transform,
        controller: &mut KinematicCharacterController,
        animation_controller: &mut AnimationController,
        sprite: &mut TextureAtlasSprite,
        stats: &mut Stats,
        time: &Res<Time>,
    ) {
        let mut moving_direction = Vec2::ZERO;
        self.wander_timer.tick(time.delta());

        if let Some(destination) = self.wandering_destination {
            let destination_dist = destination - transform.translation.x;
            let destination_reached = destination_dist == 0.0;
            let timed_out = self.wander_timer.finished();

            if destination_reached || timed_out {
                self.wander_timer
                    .set_duration(Duration::from_secs_f32(10. * random::<f32>() + 5.));
                self.wander_timer.reset();
                self.wandering_destination = None;
                animation_controller.stop();
            } else {
                let movement = destination_dist.signum() * stats.speed * time.delta_seconds();

                if animation_controller.animations.get("Walk").is_some()
                    && animation_controller.current_animation != Some("Walk".into())
                {
                    animation_controller.play("Walk");
                }

                moving_direction.x += if destination_dist.abs() > movement.abs() {
                    movement
                } else {
                    destination_dist
                };
            }
        } else if self.wander_timer.finished() {
            let direction = (random::<f32>() - 0.5) * BLOCK_SIZE * 2. * Self::MAX_WANDER_DISTANCE;

            sprite.flip_x = direction < 0.;

            self.wandering_destination = Some(transform.translation.x + direction);

            self.wander_timer
                .set_duration(Duration::from_secs_f32(Self::WANDERING_TIMEOUT));
            self.wander_timer.reset();
        }

        moving_direction.y -= stats.mass * time.delta_seconds();
        controller.translation = Some(moving_direction)
    }
}

#[derive(Clone, Copy)]
pub struct MobLootTable(pub &'static str);
impl From<MobLootTable> for PathBuf {
    fn from(val: MobLootTable) -> PathBuf {
        Path::new("mobs").join(format!("{}.json", val.0))
    }
}

#[enum_dispatch]
pub trait MobTrait: Component + Sized
where
    MobObject: From<Self>,
{
    fn name(&self) -> &'static str;

    fn texture(&self, animation: &str) -> PathBuf {
        Path::new("textures/mobs")
            .join(self.name())
            .join(format!("{}.png", animation))
    }

    fn animations(&self, asset_server: &Res<AssetServer>) -> HashMap<String, Animation>;

    fn typ(&self) -> MobType;

    fn death_loot_table(&self) -> MobLootTable {
        MobLootTable(self.name())
    }

    fn mob_obj(&self) -> Mob {
        Mob::new(self.typ(), self.death_loot_table())
    }

    fn sprite_custom_size_and_anchor(&self) -> (Option<Vec2>, Option<Anchor>) {
        (None, None)
    }

    fn default_stats(&self) -> Stats;
    fn collider(&self) -> Collider;
    fn bundle(self, asset_server: &Res<AssetServer>, position: Vec2) -> MobBundle {
        let (custom_size, anchor) = self.sprite_custom_size_and_anchor();
        let stats = self.default_stats();
        MobBundle {
            collider: self.collider(),
            mob: self.mob_obj(),
            sprite: AnimatedSpriteBundle {
                sprite: SpriteSheetBundle {
                    transform: Transform {
                        translation: position.extend(2.0),
                        ..Default::default()
                    },
                    sprite: TextureAtlasSprite {
                        anchor: anchor.unwrap_or_default(),
                        custom_size,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                animation_controller: AnimationController::new(self.animations(asset_server))
                    .with_default("Idle"),
            },
            rigid_body: RigidBody::KinematicPositionBased,
            mass: ColliderMassProperties::Mass(stats.mass),
            stats,
            controller: KinematicCharacterController::default(),
            object: self.into(),
        }
    }

    fn spawn(
        self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        position: Vec2,
    ) -> Entity {
        let bundle = self.bundle(asset_server, position);
        commands.spawn(bundle).id()
    }
}

fn mob_hit(time: Res<Time>, mut query: Query<(&mut Mob, &mut TextureAtlasSprite)>) {
    for (mut mob, mut sprite) in query.iter_mut() {
        mob.hit_timer.tick(time.delta());
        if !mob.hit_timer.paused() {
            sprite.color = Color::RED
                .with_g(mob.hit_timer.percent())
                .with_b(mob.hit_timer.percent());
        }
    }
}

fn load_mobs(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event: EventReader<SaveData>,
) {
    for ev in event.read() {
        let data = ev.read();
        if let Some(world) = data.worlds.get(&data.current_world) {
            world.mobs.iter().for_each(|mob| {
                mob.data
                    .clone()
                    .spawn(&mut commands, &asset_server, mob.pos);
            });
        }
    }
}
