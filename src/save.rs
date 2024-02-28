use crate::{
    gui::main_menu::MainMenuState,
    mob::{MobBundle, MobObject, MobTrait},
    player::{class::PlayerClasses, Player},
    state::AppState,
    world::World,
    CONFIG_DIR,
};
use bevy::prelude::*;
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
        app.add_event::<LoadSaveEvent>()
            .init_resource::<CurrentSaveName>()
            .add_systems(Update, set_current_save_name)
            .add_systems(
                OnTransition {
                    from: AppState::Paused,
                    to: AppState::MainMenu(MainMenuState::Default),
                },
                save_world,
            );
    }
}

fn set_current_save_name(
    mut event: EventReader<LoadSaveEvent>,
    mut current: ResMut<CurrentSaveName>,
) {
    for ev in event.read() {
        *current = CurrentSaveName(Some(ev.ident.clone()))
    }
}

fn save_world(
    mut commands: Commands,
    current_save_name: Res<CurrentSaveName>,
    player_query: Query<(&Player, &Transform)>,
    mobs: Query<(Entity, &MobObject, &Transform)>,
    world_query: Query<(Entity, &World)>,
) {
    if let Some(save_name) = current_save_name.0.clone() {
        info!("Saving world : {}", save_name);

        let (player, player_transform) = player_query.get_single().unwrap();

        let (entity, world) = world_query.get_single().unwrap();
        commands.entity(entity).despawn_recursive();

        let save = Save {
            player: PlayerSave {
                player: player.clone(),
                pos: player_transform.translation.xy(),
            },
            mobs: mobs
                .iter()
                .map(|(entity, mob, transform)| {
                    commands.entity(entity).despawn_recursive();
                    MobSave {
                        data: mob.clone().into(),
                        pos: transform.translation.xy(),
                    }
                })
                .collect(),
            world: world.clone(),
        };

        save.save_world(&save_name);
    }
}

#[derive(Resource, Default)]
pub struct CurrentSaveName(pub Option<String>);

#[derive(Event)]
pub struct LoadSaveEvent {
    pub ident: String,
    data: Save,
}

impl LoadSaveEvent {
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

#[derive(Serialize, Deserialize)]
pub struct MobSave {
    pub data: MobObject,
    pub pos: Vec2,
}

impl MobSave {
    pub fn into_bundle(&self, asset_server: &Res<AssetServer>) -> MobBundle {
        self.data.clone().bundle(asset_server, self.pos)
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct PlayerSave {
    pub player: Player,
    pub pos: Vec2,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Save {
    pub player: PlayerSave,
    pub mobs: Vec<MobSave>,
    pub world: World,
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
        fs::write(save_path.join(SaveMetaData::FILE_NAME), &meta_str).unwrap();
    }
}

impl Save {
    pub const DIR: Lazy<PathBuf> = Lazy::new(|| CONFIG_DIR.join("saves"));
    const FILE_NAME: &'static str = "world";

    pub fn load(&self, mut commands: Commands, asset_server: &Res<AssetServer>) {
        self.mobs.iter().for_each(|mob| {
            commands.spawn_empty().insert(mob.into_bundle(asset_server));
        });
    }

    pub fn read(name: &str) -> Result<Self, String> {
        let path = Self::DIR.join(name).join(Self::FILE_NAME);
        let data = fs::read(&path).map_err(|e| e.to_string())?;
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

        fs::write(&world_path, data).unwrap();

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
                        return Err("No world save found".into());
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
}
