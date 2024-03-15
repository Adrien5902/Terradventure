use crate::gui::main_menu::MainMenuState;
use crate::gui::misc::{ease_in_quad, ease_out_quad, PIXEL_FONT};
use crate::lang::Lang;
use crate::mob::list::rabbit::Rabbit;
use crate::mob::list::sheep::Sheep;
use crate::mob::list::MobObject;
use crate::mob::MobTrait;
use crate::random::{RandomWeightedRate, RandomWeightedTable};
use crate::state::AppState;
use crate::tiled::TiledMapBundle;
use bevy::{asset::AssetPath, prelude::*};
use bevy_parallax::{CreateParallaxEvent, LayerData, LayerSpeed, ParallaxPlugin};
use enum_dispatch::enum_dispatch;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};

pub const BLOCK_SIZE: f32 = 16.;

#[derive(Serialize, Deserialize, Component, Clone)]
pub enum World {
    Biome(Biome),
    Dungeon(Dungeon),
}

impl Default for World {
    fn default() -> Self {
        Self::Biome(Biome::default())
    }
}

impl From<Biome> for World {
    fn from(value: Biome) -> Self {
        Self::Biome(value)
    }
}

impl From<Dungeon> for World {
    fn from(value: Dungeon) -> Self {
        Self::Dungeon(value)
    }
}

impl World {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Biome(biome) => biome.name(),
            Self::Dungeon(dungeon) => dungeon.name(),
        }
    }

    pub fn get_type(&self) -> &'static str {
        match self {
            Self::Biome(_) => "biome",
            Self::Dungeon(_) => "dungeon",
        }
    }

    pub fn tile_set_path(&self) -> TileMapAsset {
        TileMapAsset(Path::new(self.get_type()).join(self.name()))
    }

    pub fn spawn(
        self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        lang: &Res<Lang>,
        create_parallax_event_writer: &mut EventWriter<CreateParallaxEvent>,
        camera: Entity,
    ) -> Entity {
        let tiled_map = asset_server.load(self.tile_set_path());

        commands.spawn((
            WorldEnterText {
                timer: Timer::from_seconds(5.0, TimerMode::Once),
            },
            TextBundle {
                text: Text::from_section(
                    lang.get(&format!("world.{}.{}", self.get_type(), self.name())),
                    TextStyle {
                        font: asset_server.load(PIXEL_FONT),
                        font_size: 64.,
                        color: Color::WHITE,
                    },
                )
                .with_alignment(TextAlignment::Center),
                style: Style {
                    margin: UiRect::axes(Val::Auto, Val::Percent(-5.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ));
        let background = match &self {
            Self::Biome(b) => b.background(),
            Self::Dungeon(d) => d.background(),
        };

        if let Some((count, image_size)) = background {
            let path = Path::new(&format!("textures/world"))
                .join(self.get_type())
                .join(self.name());

            create_parallax_event_writer.send(CreateParallaxEvent {
                layers_data: (1..=count)
                    .map(|i| LayerData {
                        speed: LayerSpeed::Horizontal(i as f32 / 3.),
                        path: path.join(format!("{i}.png")).to_string_lossy().to_string(),
                        tile_size: image_size,
                        cols: 1,
                        rows: 1,
                        scale: 1.,
                        z: i as f32,
                        ..Default::default()
                    })
                    .collect(),
                camera,
            });
        }

        let mobs = if let World::Biome(biome) = &self {
            let spawn_rates = biome.mob_spawn_rate();
            spawn_rates
                .get_random()
                .into_iter()
                .flat_map(|mob| {
                    let n = thread_rng().gen_range(mob.group);
                    (0..n).map(move |_| {
                        MobTrait::bundle(mob.mob.clone(), asset_server, Vec2::default())
                    })
                })
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };

        let entity = commands
            .spawn(self)
            .insert(TiledMapBundle {
                tiled_map,
                ..Default::default()
            })
            .insert(InheritedVisibility::VISIBLE)
            .id();

        mobs.into_iter().for_each(|bundle| {
            let mob = commands.spawn(bundle).id();
            commands.entity(entity).add_child(mob);
        });

        entity
    }
}

#[enum_dispatch(WorldTrait)]
#[enum_dispatch(BiomeTrait)]
#[derive(Serialize, Deserialize, Component, Clone)]
pub enum Biome {
    Plains(PlainsBiome),
    Forest(ForestBiome),
    Desert(DesertBiome),
}

impl Default for Biome {
    fn default() -> Self {
        PlainsBiome.into()
    }
}

#[enum_dispatch(WorldTrait)]
#[derive(Serialize, Deserialize, Component, Clone)]
pub enum Dungeon {
    Pyramid(PyramidDungeon),
}

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, world_text_update.run_if(in_state(AppState::InGame)))
            .add_systems(
                OnEnter(AppState::MainMenu(MainMenuState::Default)),
                despawn_world_text,
            )
            .add_plugins(ParallaxPlugin);
    }
}

fn world_text_update(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Style, &mut WorldEnterText)>,
) {
    for (entity, mut style, mut wet) in query.iter_mut() {
        wet.timer.tick(time.delta());

        let animation_percent = 0.3;
        if wet.timer.percent() < animation_percent {
            style.margin.top =
                Val::Percent(15. * ease_out_quad(wet.timer.percent() / animation_percent) - 5.);
        }

        if wet.timer.percent() > 1. - animation_percent {
            style.margin.top = Val::Percent(
                15. * ease_in_quad((1. - wet.timer.percent()) / animation_percent) - 5.,
            );
        }

        if wet.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn despawn_world_text(mut commands: Commands, query: Query<Entity, With<WorldEnterText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component)]
pub struct WorldEnterText {
    timer: Timer,
}

#[enum_dispatch]
pub trait WorldTrait: Sync + Send {
    fn name(&self) -> &'static str;

    fn background(&self) -> Option<(u32, Vec2)> {
        None
    }
}

#[enum_dispatch]
pub trait BiomeTrait: Sync + Send {
    fn mob_spawn_rate(&self) -> MobSpawnRates {
        MobSpawnRates::new_empty()
    }
}

#[derive(Serialize, Deserialize, Component, Clone)]
pub struct DesertBiome;
impl WorldTrait for DesertBiome {
    fn name(&self) -> &'static str {
        "desert"
    }
}
impl BiomeTrait for DesertBiome {}

#[derive(Serialize, Deserialize, Component, Clone)]
pub struct ForestBiome;
impl WorldTrait for ForestBiome {
    fn name(&self) -> &'static str {
        "forest"
    }
}
impl BiomeTrait for ForestBiome {}

#[derive(Serialize, Deserialize, Component, Clone)]
pub struct PlainsBiome;
impl WorldTrait for PlainsBiome {
    fn name(&self) -> &'static str {
        "plains"
    }

    fn background(&self) -> Option<(u32, Vec2)> {
        Some((4, Vec2::new(576., 324.)))
    }
}

impl BiomeTrait for PlainsBiome {
    fn mob_spawn_rate(&self) -> MobSpawnRates {
        MobSpawnRates::new(
            3,
            vec![
                RandomWeightedRate {
                    data: MobSpawnRate {
                        mob: Sheep::default().into(),
                        group: 1..=3,
                    },
                    weight: 5,
                },
                RandomWeightedRate {
                    data: MobSpawnRate {
                        mob: Rabbit::default().into(),
                        group: 1..=2,
                    },
                    weight: 1,
                },
            ],
        )
    }
}

#[derive(Clone)]
pub struct MobSpawnRate {
    pub mob: MobObject,
    pub group: RangeInclusive<u32>,
}

pub type MobSpawnRates = RandomWeightedTable<MobSpawnRate>;

#[derive(Serialize, Deserialize, Component, Clone)]
pub struct PyramidDungeon;
impl WorldTrait for PyramidDungeon {
    fn name(&self) -> &'static str {
        "pyramid"
    }
}

pub struct TileMapAsset(PathBuf);

impl<'a> From<TileMapAsset> for AssetPath<'a> {
    fn from(val: TileMapAsset) -> AssetPath<'a> {
        Path::new("tiled")
            .join(format!("{}.tmx", val.0.to_str().unwrap()))
            .into()
    }
}
