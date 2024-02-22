use crate::gui::misc::{ease_in_quad, ease_out_quad, PIXEL_FONT};
use crate::tiled::TiledMapBundle;
use bevy::{asset::AssetPath, prelude::*};
use std::path::{Path, PathBuf};

pub const BLOCK_SIZE: f32 = 16.;

#[derive(Resource)]
pub struct CurrentWorld(&'static dyn World);

pub enum WorldType {
    Biome,
    Dungeon,
}

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, world_text_update);
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

#[derive(Component)]
pub struct WorldEnterText {
    timer: Timer,
}

pub trait World: Sync {
    fn name(&self) -> &'static str;
    fn world_type(&self) -> WorldType;

    fn tile_set_path(&self) -> TileMapAsset {
        TileMapAsset(
            Path::new(match self.world_type() {
                WorldType::Biome => "biome",
                WorldType::Dungeon => "dungeon",
            })
            .join(self.name()),
        )
    }

    fn spawn(&self, mut commands: Commands, asset_server: &Res<AssetServer>) {
        let tiled_map = asset_server.load(self.tile_set_path());
        commands.spawn(TiledMapBundle {
            tiled_map,
            ..Default::default()
        });
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
    }
}

struct DesertBiome;
impl World for DesertBiome {
    fn name(&self) -> &'static str {
        "desert"
    }
    fn world_type(&self) -> WorldType {
        WorldType::Biome
    }
}

pub struct ForestBiome;
impl World for ForestBiome {
    fn name(&self) -> &'static str {
        "forest"
    }
    fn world_type(&self) -> WorldType {
        WorldType::Biome
    }
}

pub struct PlainsBiome;
impl World for PlainsBiome {
    fn name(&self) -> &'static str {
        "plains"
    }
    fn world_type(&self) -> WorldType {
        WorldType::Biome
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
