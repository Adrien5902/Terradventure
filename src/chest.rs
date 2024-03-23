use crate::{
    animation::AnimationController, interactable::Interactable, items::loot_table::LootTable,
    player::money::DropMoneyEvent, state::AppState,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Chest {
    pub loot_table: LootTable,
    pub chest_type: i32,
    pub name: String,
}

pub struct ChestPlugin;
impl Plugin for ChestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, chest_update.run_if(in_state(AppState::InGame)));
    }
}

fn chest_update(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &Chest,
        &Interactable,
        &Transform,
        &mut AnimationController,
    )>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut money_event: EventWriter<DropMoneyEvent>,
) {
    for (entity, chest, interactable, transform, mut animation_controller) in query.iter_mut() {
        if animation_controller.timer.percent() == 0. {
            if interactable.just_pressed() {
                animation_controller.tick(&time);
            }
        } else {
            animation_controller.tick(&time);
        }

        if animation_controller.just_finished.is_some() {
            let (money, items) = chest.loot_table.get_random();

            money_event.send(DropMoneyEvent {
                amount: money,
                pos: transform.translation.xy(),
            });

            for loot in items {
                commands.spawn(loot.bundle(&asset_server, transform.translation.xy()));
            }

            commands.entity(entity).despawn_recursive();
        }
    }
}
