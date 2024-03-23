use crate::{
    chest::Chest,
    effects::EffectsController,
    gui::main_menu::MainMenuState,
    items::stack::ItemStack,
    mob::{list::MobObject, MobBundle, MobTrait},
    player::{class::PlayerClasses, Player},
    state::AppState,
    stats::Stats,
    world::World,
    CONFIG_DIR,
};
use bevy::{prelude::*, utils::hashbrown::HashMap};
use bincode;
use chrono::DateTime;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    fs,
    path::{Path, PathBuf},
};

pub struct SavePlugin;
impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SaveData>()
            .init_resource::<CurrentSave>()
            .add_systems(Update, set_current_save)
            .add_systems(
                OnTransition {
                    from: AppState::Paused,
                    to: AppState::MainMenu(MainMenuState::Default),
                },
                save_world,
            );
    }
}

fn set_current_save(mut event: EventReader<SaveData>, mut current: ResMut<CurrentSave>) {
    for ev in event.read() {
        *current = CurrentSave(Some(ev.clone()))
    }
}

fn save_world(
    mut commands: Commands,
    player_query: Query<(&Player, &Transform, &Stats, &EffectsController)>,
    mobs: Query<(Entity, &MobObject, &Transform, &Stats)>,
    items: Query<(Entity, &ItemStack, &Transform)>,
    chests: Query<&Chest>,
    world_query: Query<(Entity, &World)>,
    mut current_save: ResMut<CurrentSave>,
) {
    if let Some(save_data) = &mut current_save.0.clone() {
        info!("Saving world : {}", save_data.ident);

        let (player, player_transform, stats, effects) = player_query.get_single().unwrap();

        let (world_entity, world) = world_query.get_single().unwrap();

        let mut worlds = save_data.data.worlds.clone();

        worlds.insert(
            world.clone(),
            WorldSave {
                items: items
                    .iter()
                    .map(|(entity, stack, transform)| {
                        commands.entity(entity).despawn_recursive();
                        ItemSave {
                            stack: stack.clone(),
                            pos: transform.translation.xy(),
                        }
                    })
                    .collect(),

                mobs: mobs
                    .iter()
                    .map(|(entity, mob, transform, stats)| {
                        commands.entity(entity).despawn_recursive();
                        MobSave {
                            data: mob.clone(),
                            stats: stats.clone(),
                            pos: transform.translation.xy(),
                        }
                    })
                    .collect(),

                available_chests: Some(chests.iter().map(|chest| chest.name.clone()).collect()),
            },
        );

        let save = Save {
            player: PlayerSave {
                player: player.clone(),
                stats: stats.clone(),
                pos: player_transform.translation.xy(),
                effects: effects.clone(),
            },
            worlds,
            current_world: world.clone(),
        };

        save.save_world(&save_data.ident);
        current_save.0 = None;
        commands.entity(world_entity).despawn_recursive();
    }
}

#[derive(Resource, Default)]
pub struct CurrentSave(pub Option<SaveData>);

#[derive(Event, Clone)]
pub struct SaveData {
    pub ident: String,
    pub data: Save,
}

impl SaveData {
    pub fn read(&self) -> &Save {
        &self.data
    }

    pub fn new(ident: &str, data: Save) -> Self {
        Self {
            ident: ident.to_owned(),
            data,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MobSave {
    pub data: MobObject,
    pub stats: Stats,
    pub pos: Vec2,
}

impl MobSave {
    pub fn into_bundle(&self, asset_server: &Res<AssetServer>) -> MobBundle {
        self.data.clone().bundle(asset_server, self.pos)
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct PlayerSave {
    pub player: Player,
    pub stats: Stats,
    pub pos: Vec2,
    pub effects: EffectsController,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ItemSave {
    pub stack: ItemStack,
    pub pos: Vec2,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WorldSave {
    pub mobs: Vec<MobSave>,
    pub items: Vec<ItemSave>,
    pub available_chests: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Default, Resource, Clone)]
pub struct Save {
    pub player: PlayerSave,
    pub worlds: HashMap<World, WorldSave>,
    pub current_world: World,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveMetaData {
    pub name: String,
    pub creation_date: DateTime<chrono::Local>,
    pub last_played: DateTime<chrono::Local>,
}

impl SaveMetaData {
    const FILE_NAME: &'static str = "metadata.json";

    pub fn from_save_path(path: &Path) -> Result<Self, String> {
        let data = fs::read(path.join(Self::FILE_NAME)).map_err(|e| e.to_string())?;
        serde_json::from_slice(&data).map_err(|e| e.to_string())
    }

    pub fn new_now(name: &str) -> Self {
        let date = chrono::offset::Local::now();

        Self {
            name: name.to_owned(),
            creation_date: date,
            last_played: date,
        }
    }

    pub fn save(&self, save_path: &Path) {
        let meta_str = serde_json::to_string(&self).unwrap();
        fs::write(save_path.join(SaveMetaData::FILE_NAME), meta_str).unwrap();
    }
}

impl Save {
    pub const DIR: Lazy<PathBuf> = Lazy::new(|| CONFIG_DIR.join("saves"));
    const FILE_NAME: &'static str = "world";

    pub fn read(name: &str) -> Result<Self, String> {
        let path = Self::DIR.join(name).join(Self::FILE_NAME);
        let data = fs::read(path).map_err(|e| e.to_string())?;
        bincode::deserialize(&data).map_err(|e| e.to_string())
    }

    pub fn new(name: &str, class: PlayerClasses) -> Result<(Self, SaveMetaData), String> {
        let path = Self::DIR.join(name);
        if path.exists() {
            return Err("Save already exists".to_owned());
        }

        fs::create_dir_all(&path).map_err(|e| e.to_string())?;

        let meta = SaveMetaData::new_now(name);
        meta.save(&path);

        let mut save = Self::default();
        save.player.player.class = class;

        Ok((save, meta))
    }

    pub fn save_world(&self, name: &str) {
        let path = Self::DIR.join(name);
        let world_path = path.join(Self::FILE_NAME);
        let data = bincode::serialize(&self).unwrap();

        let mut meta = SaveMetaData::from_save_path(&path).unwrap();
        meta.last_played = chrono::offset::Local::now();
        meta.save(&path);

        fs::write(world_path, data).unwrap();

        info!("Saved world {} successfully", name)
    }

    pub fn get_saves() -> Vec<Result<(String, SaveMetaData), String>> {
        let dir_res = Self::DIR.read_dir().map_err(|e| e.to_string());
        if let Ok(dir) = dir_res {
            let mut data = dir
                .into_iter()
                .map(|entry| {
                    let folder = entry.map_err(|e| e.to_string())?;
                    let path = folder.path();
                    let file_name = folder.file_name().to_string_lossy().to_string();

                    let metadata = SaveMetaData::from_save_path(&path)?;
                    let world_save_path = path.join(Save::FILE_NAME);

                    if !world_save_path.exists() {
                        return Err("ui.main_menu.load_save.err.no_world".into());
                    }

                    Ok((file_name, metadata))
                })
                .collect::<Vec<Result<_, _>>>();

            data.sort_by(|a, b| {
                if let Ok((_, meta_a)) = a {
                    if let Ok((_, meta_b)) = b {
                        meta_b
                            .last_played
                            .timestamp()
                            .partial_cmp(&meta_a.last_played.timestamp())
                            .unwrap()
                    } else {
                        Ordering::Equal
                    }
                } else {
                    Ordering::Equal
                }
            });

            data
        } else {
            fs::create_dir_all(&*Self::DIR).unwrap();
            Vec::new()
        }
    }

    pub fn delete(name: &str) -> Result<(), String> {
        fs::remove_dir_all(Self::DIR.join(name)).map_err(|e| e.to_string())
    }
}
