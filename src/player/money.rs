use bevy::prelude::*;
use rand::random;
use serde::{Deserialize, Serialize};
use std::ops::AddAssign;

use crate::{stats::Stats, world::BLOCK_SIZE};

use super::Player;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Money {
    amount: u64,
}

impl Money {
    /// # Returns
    /// ```true``` if removing was successfull
    pub fn try_remove(&mut self, amount: u64) -> bool {
        let can_remove = self.amount >= amount;
        if can_remove {
            self.amount -= amount;
        }

        can_remove
    }

    pub fn get(&self) -> u64 {
        self.amount
    }
}

impl AddAssign<u64> for Money {
    fn add_assign(&mut self, rhs: u64) {
        self.amount += rhs
    }
}

#[derive(Event)]
pub struct DropMoneyEvent {
    pub amount: u64,
    pub pos: Vec2,
}

#[derive(Component)]
pub struct Coin;

pub struct MoneyPlugin;
impl Plugin for MoneyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DropMoneyEvent>()
            .add_systems(Update, money_drop);
    }
}

fn money_drop(
    mut commands: Commands,
    mut event: EventReader<DropMoneyEvent>,
    mut player_query: Query<(&mut Player, &Transform, &Stats)>,
    mut coin_query: Query<(Entity, &mut Transform), With<Coin>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    for ev in event.read() {
        if let Ok((mut player, _, _)) = player_query.get_single_mut() {
            player.money += ev.amount;
        }

        for _i in 0..ev.amount {
            let mut transform = Transform::from_translation(ev.pos.extend(30.));
            let max_offset = BLOCK_SIZE;
            transform.translation.x += random::<f32>() * max_offset;
            transform.translation.y += random::<f32>() * max_offset;

            commands.spawn(Coin).insert(SpriteBundle {
                texture: asset_server.load("gui/coin.png"),
                transform,
                ..Default::default()
            });
        }
    }

    if let Ok((_, player_transform, stats)) = player_query.get_single_mut() {
        for (entity, mut transform) in coin_query.iter_mut() {
            let current_pos = transform.translation.xy();
            let direction = player_transform.translation.xy() - current_pos;

            let sum = direction.x.abs() + direction.y.abs();
            if sum < Player::SIZE / 2. {
                commands.entity(entity).despawn();
                continue;
            }

            transform.translation +=
                direction.normalize().extend(0.0) * stats.speed * time.delta_seconds();
        }
    }
}
