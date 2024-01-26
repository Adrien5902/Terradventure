pub mod inventory;
mod items;
mod loot_table;
pub mod player;
pub mod entite;
pub mod mob;

use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
