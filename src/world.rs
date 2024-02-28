use crate::gui::main_menu::MainMenuState;
use crate::gui::misc::{ease_in_quad, ease_out_quad, PIXEL_FONT};
use crate::state::AppState;
use crate::tiled::TiledMapBundle;
use bevy::{asset::AssetPath, prelude::*};
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
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

    pub fn spawn(self, commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
        let tiled_map = asset_server.load(self.tile_set_path());
        commands.spawn((
            WorldEnterText {
                timer: Timer::from_seconds(5.0, TimerMode::Once),
            },
            TextBundle {
                text: Text::from_section(
                    self.name(),
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
        commands
            .spawn(self)
            .insert(TiledMapBundle {
                tiled_map,
                ..Default::default()
            })
            .id()
    }
}

#[enum_dispatch(WorldTrait)]
#[derive(Serialize, Deserialize, Component, Clone)]
pub enum Biome {
    Plains(PlainsBiome),
    Forest(ForestBiome),
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
            );
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
}

#[derive(Serialize, Deserialize, Component, Clone)]
struct DesertBiome;
impl WorldTrait for DesertBiome {
    fn name(&self) -> &'static str {
        "desert"
    }
}

#[derive(Serialize, Deserialize, Component, Clone)]
pub struct ForestBiome;
impl WorldTrait for ForestBiome {
    fn name(&self) -> &'static str {
        "forest"
    }
}

#[derive(Serialize, Deserialize, Component, Clone)]
pub struct PlainsBiome;
impl WorldTrait for PlainsBiome {
    fn name(&self) -> &'static str {
        "plains"
    }
}

#[derive(Serialize, Deserialize, Component, Clone)]
pub struct PyramidDungeon;
impl WorldTrait for PyramidDungeon {
    fn name(&self) -> &'static str {
        "pyramid"
    }
}

pub struct TileMapAsset(PathBuf);

impl<'a> Into<AssetPath<'a>> for TileMapAsset {
    fn into(self) -> AssetPath<'a> {
        Path::new("tiled")
            .join(format!("{}.tmx", self.0.to_str().unwrap()))
            .into()
    }
}
