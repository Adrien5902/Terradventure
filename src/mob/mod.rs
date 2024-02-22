pub mod list;

use crate::{items::loot_table::LootTable, state::AppState, stats::Stats, world::BLOCK_SIZE};
use bevy::{asset::AssetPath, prelude::*};
use bevy_rapier2d::prelude::*;
use rand::random;
use std::{path::Path, time::Duration};

use self::list::sheep::Sheep;

pub struct MobPlugin;
impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_ai.run_if(in_state(AppState::InGame)))
            .add_systems(OnEnter(AppState::InGame), spawn_sheep);
    }
}

fn spawn_sheep(commands: Commands, asset_server: Res<AssetServer>) {
    Sheep::new(commands, asset_server, Vec2 { x: 0.0, y: -6.0 });
}

fn update_ai(mut query: Query<(&mut Mob, &Stats, &mut Sprite, &mut Transform)>, time: Res<Time>) {
    for (mut mob, stats, mut sprite, mut transform) in query.iter_mut() {
        mob.ai.update(&mut transform, &mut sprite, stats, &time);
    }
}

#[derive(Component)]
pub struct Mob {
    pub typ: MobType,
    pub death_loot_table: Option<Handle<LootTable>>,
    pub ai: Box<dyn MobAi>,
}

impl Mob {
    pub fn new(typ: MobType, death_loot_table: Option<Handle<LootTable>>) -> Self {
        Self {
            ai: typ.clone().into(),
            typ,
            death_loot_table,
        }
    }
}

#[derive(Bundle)]
pub struct MobBundle {
    mob: Mob,
    stats: Stats,
    sprite: SpriteBundle,
    collider: Collider,
    rigid_body: RigidBody,
    mass: ColliderMassProperties,
}

#[derive(Clone)]
pub enum MobType {
    Passive,
    Neutral,
    Agressive,
}

impl Into<Box<dyn MobAi>> for MobType {
    fn into(self) -> Box<dyn MobAi> {
        Box::new(match self {
            Self::Passive => PassiveDefaultMobAi::default(),
            _ => PassiveDefaultMobAi::default(),
        })
    }
}

pub trait MobAi: Sync + Send {
    fn update(
        &mut self,
        transform: &mut Transform,
        sprite: &mut Sprite,
        stats: &Stats,
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
        transform: &mut Transform,
        sprite: &mut Sprite,
        stats: &Stats,
        time: &Res<Time>,
    ) {
        self.wander_timer.tick(time.delta());
        if let Some(destination) = self.wandering_destination {
            let destination_dist = destination - transform.translation.x;
            let destination_reached = destination_dist == 0.0;
            let timed_out = self.wander_timer.finished();

            if destination_reached || timed_out {
                self.wander_timer
                    .set_duration(Duration::from_secs_f32(10. * random::<f32>() + 5.));
                self.wander_timer.reset();
                self.wandering_destination = None
            } else {
                let movement = destination_dist.signum() * stats.speed * time.delta_seconds();

                transform.translation.x += if destination_dist.abs() > movement.abs() {
                    movement
                } else {
                    destination_dist
                };

                transform.translation.y += 1.0 * time.delta_seconds();
            }
        } else if self.wander_timer.finished() {
            let direction = (random::<f32>() - 0.5) * BLOCK_SIZE * 2. * Self::MAX_WANDER_DISTANCE;

            sprite.flip_x = direction > 0.;

            self.wandering_destination = Some(transform.translation.x + direction);

            self.wander_timer
                .set_duration(Duration::from_secs_f32(Self::WANDERING_TIMEOUT));
            self.wander_timer.reset();
        }
    }
}

pub struct MobTexture(pub &'static str);
impl<'a> Into<AssetPath<'a>> for MobTexture {
    fn into(self) -> AssetPath<'a> {
        let path = Path::new("textures/mobs").join(format!("{}.png", self.0));
        AssetPath::from(path)
    }
}

pub struct MobLootTable(pub &'static str);
impl<'a> Into<AssetPath<'a>> for MobLootTable {
    fn into(self) -> AssetPath<'a> {
        let path = Path::new("loot_tables/mobs").join(format!("{}.loot_table.json", self.0));
        AssetPath::from(path)
    }
}

pub struct MobName(&'static str);

pub trait MobTrait: Sized + Component + Default {
    fn name(&self) -> &'static str;
    fn texture(&self) -> MobTexture {
        MobTexture(self.name())
    }
    fn mob_obj(&self) -> Mob;
    fn default_stats(&self) -> Stats;
    fn collider(&self) -> Collider;
    fn bundle(&self, asset_server: Res<AssetServer>, position: Vec2) -> MobBundle {
        let stats = self.default_stats();
        MobBundle {
            collider: self.collider(),
            mob: self.mob_obj(),
            sprite: SpriteBundle {
                texture: asset_server.load(self.texture()),
                transform: Transform {
                    translation: position.extend(0.0) * BLOCK_SIZE,
                    ..Default::default()
                },
                ..Default::default()
            },
            rigid_body: RigidBody::Dynamic,
            mass: ColliderMassProperties::Mass(stats.mass),
            stats,
        }
    }

    fn spawn(
        self,
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        position: Vec2,
    ) -> Entity {
        let bundle = self.bundle(asset_server, position);
        commands.spawn(self).insert(bundle).id()
    }

    fn new(commands: Commands, asset_server: Res<AssetServer>, position: Vec2) {
        Self::spawn(Self::default(), commands, asset_server, position);
    }
}
