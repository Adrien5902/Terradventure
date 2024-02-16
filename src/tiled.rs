//imported and modified from https://github.com/StarArawn/bevy_ecs_tilemap/blob/main/examples/helpers/tiled.rs

use bevy::asset::LoadContext;
use bevy::{
    asset::{io::Reader, AssetLoader, AssetPath, AsyncReadExt},
    log,
    prelude::*,
    utils::{BoxedFuture, HashMap},
};
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier_collider_gen::single_polyline_collider_translated;
use image::{DynamicImage, GenericImageView};
use std::io::{Cursor, ErrorKind};
use std::panic::catch_unwind;
use std::path::Path;
use std::sync::Arc;
use tiled::Tileset;

use thiserror::Error;

#[derive(Default)]
pub struct TiledMapPlugin;

impl Plugin for TiledMapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_asset::<TiledMap>()
            .register_asset_loader(TiledLoader)
            .add_systems(Update, process_loaded_maps);
    }
}

#[derive(TypePath, Asset)]
pub struct TiledMap {
    pub map: tiled::Map,

    pub tilesets: HashMap<usize, CollidingTileSet>,

    // The offset into the tileset_images for each tile id within each tileset.
    #[cfg(not(feature = "atlas"))]
    pub tile_image_offsets: HashMap<(usize, tiled::TileId), u32>,
}

pub struct CollidingTileSet {
    pub texture: TilemapTexture,
    pub colliders: HashMap<u32, Option<Collider>>,
}

impl CollidingTileSet {
    fn from_tileset(
        tileset: &Tileset,
        index: usize,
        load_context: &mut LoadContext,
        tile_image_offsets: &mut HashMap<(usize, u32), u32>,
    ) -> Self {
        let mut colliders = HashMap::new();

        match &tileset.image {
            None => {
                let mut tile_images: Vec<Handle<Image>> = Vec::new();
                for (tile_id, tile) in tileset.tiles() {
                    if let Some(img) = &tile.image {
                        let tmx_dir = load_context
                            .path()
                            .parent()
                            .expect("The asset load context was empty.");
                        let tile_path = tmx_dir.join(Path::new("..").join(&img.source));
                        let asset_path = AssetPath::from(tile_path);
                        let texture: Handle<Image> = load_context.load(asset_path.clone());
                        tile_image_offsets.insert((index, tile_id), tile_images.len() as u32);
                        tile_images.push(texture.clone());

                        let img = read_img(asset_path.clone().path());

                        let collider = collider_from_img(img);
                        colliders.insert(tile_id, collider);
                    }
                }

                Self {
                    texture: TilemapTexture::Vector(tile_images),
                    colliders,
                }
            }
            Some(img) => {
                let tmx_dir = load_context
                    .path()
                    .parent()
                    .expect("The asset load context was empty.");
                let tile_path = tmx_dir.join(Path::new("..").join(&img.source));
                let asset_path = AssetPath::from(tile_path);
                let handle: Handle<Image> = load_context.load(asset_path.clone());

                let img = read_img(asset_path.clone().path());
                let img_size = img.dimensions();

                for (i, _tile) in tileset.tiles() {
                    let col_count = img_size.0 / tileset.tile_width;

                    let x = i % col_count;
                    let y = i / col_count;

                    let new_img = img.clone().crop(
                        x * tileset.tile_width,
                        y * tileset.tile_height,
                        tileset.tile_width,
                        tileset.tile_height,
                    );

                    let collider = collider_from_img(new_img);

                    colliders.insert(i, collider);
                }

                Self {
                    texture: TilemapTexture::Single(handle.clone()),
                    colliders,
                }
            }
        }
    }
}

fn read_img(path: &Path) -> DynamicImage {
    image::io::Reader::open(Path::new("assets").join(path))
        .map_err(|e| e.to_string())
        .unwrap()
        .with_guessed_format()
        .map_err(|e| e.to_string())
        .unwrap()
        .decode()
        .unwrap()
}

fn slope_collider(half_size: f32, direction: SlopeLayerDirection) -> Collider {
    let points: [Vec2; 3] = [
        Vec2::new(
            match direction {
                SlopeLayerDirection::Left => 1.0,
                SlopeLayerDirection::Right => -1.0,
                SlopeLayerDirection::Full => panic!("please specify a side"),
            },
            1.0,
        ),
        Vec2::new(-1.0, -1.0),
        Vec2::new(1.0, -1.0),
    ];

    let scaled_points: [Vec2; 3] = points
        .into_iter()
        .map(|p| p * half_size)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let [a, b, c] = scaled_points;

    Collider::triangle(a, b, c)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SlopeLayerDirection {
    Full,
    Right,
    Left,
}

impl SlopeLayerDirection {
    fn compare(a: Self, b: Self) -> Result<Self, &'static str> {
        let err = "facing opposite direction";

        match a {
            Self::Full => Ok(b),
            Self::Left => match b {
                Self::Right => Err(err),
                _ => Ok(a),
            },
            Self::Right => match b {
                Self::Left => Err(err),
                _ => Ok(a),
            },
        }
    }
}

/// #Returns
///
/// (isSlope, isFlippedY)
fn img_is_slope(img: &DynamicImage) -> (bool, SlopeLayerDirection) {
    const ERROR: f32 = 0.1;

    let data = (0..img.height())
        .map(|y| {
            let pixels_a = (0..img.width())
                .map(|x| {
                    let pixel = img.get_pixel(x, y);
                    pixel[3] as f32
                })
                .collect::<Vec<_>>();

            let layer_ratio = (pixels_a.iter().sum::<f32>() / pixels_a.len() as f32) / 255.;
            let direction = if layer_ratio == 1. {
                SlopeLayerDirection::Full
            } else {
                let half = pixels_a.len() / 2;
                let left = &pixels_a[..half];
                let right = &pixels_a[half..];

                let left_ratio = left.iter().sum::<f32>() / left.len() as f32;
                let right_ratio = right.iter().sum::<f32>() / right.len() as f32;

                if left_ratio > right_ratio {
                    SlopeLayerDirection::Right
                } else if right_ratio > left_ratio {
                    SlopeLayerDirection::Left
                } else {
                    SlopeLayerDirection::Full
                }
            };

            (layer_ratio, direction)
        })
        .collect::<Vec<_>>();

    let mut is_slope = (1..data.len())
        .map(|i| {
            let (upper_ratio, _) = data[i - 1];
            let (down_ratio, _) = data[i];

            down_ratio >= upper_ratio - ERROR
        })
        .reduce(|a, b| (a && b))
        .unwrap();

    let direction = if is_slope {
        data.iter()
            .map(|(_, direction)| Ok(*direction))
            .reduce(|a, b| SlopeLayerDirection::compare(a.unwrap(), b.unwrap()))
            .unwrap()
    } else {
        Ok(SlopeLayerDirection::Full)
    };

    is_slope = is_slope && direction.is_ok() && direction.unwrap() != SlopeLayerDirection::Full;
    (is_slope, direction.unwrap_or(SlopeLayerDirection::Full))
}

fn collider_from_img(img: DynamicImage) -> Option<Collider> {
    let t = transparency_ratio(&img);

    if t >= 1. {
        Some(Collider::cuboid(
            img.width() as f32 / 2.,
            img.height() as f32 / 2.,
        ))
    } else if t > 0.5 {
        let (is_slope, direction) = img_is_slope(&img);

        if is_slope {
            Some(slope_collider(img.width() as f32 / 2., direction))
        } else {
            catch_unwind(|| {
                let bevy_img = Image::from_dynamic(img, true);

                single_polyline_collider_translated(&bevy_img)
            })
            .ok()
        }
    } else {
        None
    }
}

fn transparency_ratio(img: &DynamicImage) -> f32 {
    let vec = img
        .pixels()
        .map(|(_, _, pixel)| pixel[3] as f32)
        .collect::<Vec<_>>();

    vec.iter().sum::<f32>() / vec.len() as f32 / 255.0
}

// Stores a list of tiled layers.
#[derive(Component, Default)]
pub struct TiledLayersStorage {
    pub storage: HashMap<u32, Entity>,
}

#[derive(Default, Bundle)]
pub struct TiledMapBundle {
    pub tiled_map: Handle<TiledMap>,
    pub storage: TiledLayersStorage,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

struct BytesResourceReader {
    bytes: Arc<[u8]>,
}

impl BytesResourceReader {
    fn new(bytes: &[u8]) -> Self {
        Self {
            bytes: Arc::from(bytes),
        }
    }
}

impl tiled::ResourceReader for BytesResourceReader {
    type Resource = Cursor<Arc<[u8]>>;
    type Error = std::io::Error;

    fn read_from(&mut self, _path: &Path) -> std::result::Result<Self::Resource, Self::Error> {
        // In this case, the path is ignored because the byte data is already provided.
        Ok(Cursor::new(self.bytes.clone()))
    }
}

pub struct TiledLoader;

#[derive(Debug, Error)]
pub enum TiledAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load Tiled file: {0}")]
    Io(#[from] std::io::Error),
}

impl AssetLoader for TiledLoader {
    type Asset = TiledMap;
    type Settings = ();
    type Error = TiledAssetLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let mut loader = tiled::Loader::with_cache_and_reader(
                tiled::DefaultResourceCache::new(),
                BytesResourceReader::new(&bytes),
            );
            let map = loader.load_tmx_map(load_context.path()).map_err(|e| {
                std::io::Error::new(ErrorKind::Other, format!("Could not load TMX map: {e}"))
            })?;

            let mut tilesets = HashMap::default();
            let mut tile_image_offsets = HashMap::default();

            for (tileset_index, tileset) in map.tilesets().iter().enumerate() {
                tilesets.insert(
                    tileset_index,
                    CollidingTileSet::from_tileset(
                        tileset,
                        tileset_index,
                        load_context,
                        &mut tile_image_offsets,
                    ),
                );
            }

            let asset_map = TiledMap {
                map,
                tilesets,
                #[cfg(not(feature = "atlas"))]
                tile_image_offsets,
            };

            log::info!("Loaded map: {}", load_context.path().display());
            Ok(asset_map)
        })
    }

    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["tmx"];
        EXTENSIONS
    }
}

pub fn process_loaded_maps(
    mut commands: Commands,
    mut map_events: EventReader<AssetEvent<TiledMap>>,
    maps: Res<Assets<TiledMap>>,
    tile_storage_query: Query<(Entity, &TileStorage)>,
    mut map_query: Query<(&Handle<TiledMap>, &mut TiledLayersStorage)>,
    new_maps: Query<&Handle<TiledMap>, Added<Handle<TiledMap>>>,
) {
    let mut changed_maps = Vec::<AssetId<TiledMap>>::default();
    for event in map_events.read() {
        match event {
            AssetEvent::Added { id } => {
                log::info!("Map added!");
                changed_maps.push(*id);
            }
            AssetEvent::Modified { id } => {
                log::info!("Map changed!");
                changed_maps.push(*id);
            }
            AssetEvent::Removed { id } => {
                log::info!("Map removed!");
                // if mesh was modified and removed in the same update, ignore the modification
                // events are ordered so future modification events are ok
                changed_maps.retain(|changed_handle| changed_handle == id);
            }
            _ => continue,
        }
    }

    // If we have new map entities add them to the changed_maps list.
    for new_map_handle in new_maps.iter() {
        changed_maps.push(new_map_handle.id());
    }

    for changed_map in changed_maps.iter() {
        for (map_handle, mut layer_storage) in map_query.iter_mut() {
            // only deal with currently changed map
            if map_handle.id() != *changed_map {
                continue;
            }
            if let Some(tiled_map) = maps.get(map_handle) {
                // TODO: Create a RemoveMap component..
                for layer_entity in layer_storage.storage.values() {
                    if let Ok((_, layer_tile_storage)) = tile_storage_query.get(*layer_entity) {
                        for tile in layer_tile_storage.iter().flatten() {
                            commands.entity(*tile).despawn_recursive()
                        }
                    }
                    // commands.entity(*layer_entity).despawn_recursive();
                }

                // The TilemapBundle requires that all tile images come exclusively from a single
                // tiled texture or from a Vec of independent per-tile images. Furthermore, all of
                // the per-tile images must be the same size. Since Tiled allows tiles of mixed
                // tilesets on each layer and allows differently-sized tile images in each tileset,
                // this means we need to load each combination of tileset and layer separately.
                for (tileset_index, tileset) in tiled_map.map.tilesets().iter().enumerate() {
                    let Some(colliding_tileset) = tiled_map.tilesets.get(&tileset_index) else {
                        log::warn!("Skipped creating layer with missing tilemap textures.");
                        continue;
                    };

                    let tile_size = TilemapTileSize {
                        x: tileset.tile_width as f32,
                        y: tileset.tile_height as f32,
                    };

                    let tile_spacing = TilemapSpacing {
                        x: tileset.spacing as f32,
                        y: tileset.spacing as f32,
                    };

                    // Once materials have been created/added we need to then create the layers.
                    for (layer_index, layer) in tiled_map.map.layers().enumerate() {
                        let offset_x = layer.offset_x;
                        let offset_y = layer.offset_y;

                        let tiled::LayerType::Tiles(tile_layer) = layer.layer_type() else {
                            log::info!(
                                "Skipping layer {} because only tile layers are supported.",
                                layer.id()
                            );
                            continue;
                        };

                        let tiled::TileLayer::Finite(layer_data) = tile_layer else {
                            log::info!(
                                "Skipping layer {} because only finite layers are supported.",
                                layer.id()
                            );
                            continue;
                        };

                        let map_size = TilemapSize {
                            x: tiled_map.map.width,
                            y: tiled_map.map.height,
                        };

                        let grid_size = TilemapGridSize {
                            x: tiled_map.map.tile_width as f32,
                            y: tiled_map.map.tile_height as f32,
                        };

                        let map_type = match tiled_map.map.orientation {
                            tiled::Orientation::Hexagonal => {
                                TilemapType::Hexagon(HexCoordSystem::Row)
                            }
                            tiled::Orientation::Isometric => {
                                TilemapType::Isometric(IsoCoordSystem::Diamond)
                            }
                            tiled::Orientation::Staggered => {
                                TilemapType::Isometric(IsoCoordSystem::Staggered)
                            }
                            tiled::Orientation::Orthogonal => TilemapType::Square,
                        };

                        let mut tile_storage = TileStorage::empty(map_size);
                        let layer_entity = commands.spawn_empty().id();

                        let tile_map_offset = get_tilemap_center_transform(
                            &map_size,
                            &grid_size,
                            &map_type,
                            layer_index as f32,
                        ) * Transform::from_xyz(offset_x, -offset_y, 0.0);

                        for x in 0..map_size.x {
                            for y in 0..map_size.y {
                                // Transform TMX coords into bevy coords.
                                let mapped_y = tiled_map.map.height - 1 - y;

                                let mapped_x = x as i32;
                                let mapped_y = mapped_y as i32;

                                let layer_tile = match layer_data.get_tile(mapped_x, mapped_y) {
                                    Some(t) => t,
                                    None => {
                                        continue;
                                    }
                                };
                                if tileset_index != layer_tile.tileset_index() {
                                    continue;
                                }
                                let layer_tile_data =
                                    match layer_data.get_tile_data(mapped_x, mapped_y) {
                                        Some(d) => d,
                                        None => {
                                            continue;
                                        }
                                    };

                                let texture_index = match colliding_tileset.texture {
                                    TilemapTexture::Single(_) => layer_tile.id(),
                                    #[cfg(not(feature = "atlas"))]
                                    TilemapTexture::Vector(_) =>
                                        *tiled_map.tile_image_offsets.get(&(tileset_index, layer_tile.id()))
                                        .expect("The offset into to image vector should have been saved during the initial load."),
                                    #[cfg(not(feature = "atlas"))]
                                    _ => unreachable!()
                                };

                                let tile_pos = TilePos { x, y };
                                let tile_bundle = TileBundle {
                                    position: tile_pos,
                                    tilemap_id: TilemapId(layer_entity),
                                    texture_index: TileTextureIndex(texture_index),
                                    flip: TileFlip {
                                        x: layer_tile_data.flip_h,
                                        y: layer_tile_data.flip_v,
                                        d: layer_tile_data.flip_d,
                                    },
                                    ..Default::default()
                                };

                                let collider = colliding_tileset
                                    .colliders
                                    .get(&layer_tile.id())
                                    .expect("hitbox not found")
                                    .clone();

                                let mut tile_transform = tile_map_offset;

                                if layer_tile_data.flip_h {
                                    tile_transform.scale.x *= -1.;
                                }
                                if layer_tile_data.flip_d {
                                    tile_transform.scale.y *= -1.;
                                }

                                tile_transform.translation +=
                                    tile_pos.center_in_world(&grid_size, &map_type).extend(0.0);

                                let mut cmd = commands.spawn(tile_bundle);

                                if let Some(c) = collider {
                                    cmd.insert((c, TransformBundle::from(tile_transform)));
                                }

                                let tile_entity = cmd.id();
                                tile_storage.set(&tile_pos, tile_entity);
                            }
                        }

                        commands.entity(layer_entity).insert(TilemapBundle {
                            grid_size,
                            size: map_size,
                            storage: tile_storage,
                            texture: colliding_tileset.texture.clone(),
                            tile_size,
                            spacing: tile_spacing,
                            transform: tile_map_offset,
                            map_type,
                            ..Default::default()
                        });

                        layer_storage
                            .storage
                            .insert(layer_index as u32, layer_entity);
                    }
                }
            }
        }
    }
}
