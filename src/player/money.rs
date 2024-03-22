use bevy::prelude::*;
use rand::random;
use serde::{Deserialize, Serialize};
use std::{ops::AddAssign, time::Duration};

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
pub struct Coin {
    pub timer: Timer,
}

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
    mut coin_query: Query<(Entity, &mut Transform, &mut Coin), Without<Player>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    for ev in event.read() {
        if let Ok((mut player, _, _)) = player_query.get_single_mut() {
            player.money += ev.amount;
        }

        for _i in 0..ev.amount {
            let mut transform = Transform::from_translation(ev.pos.extend(30.));
            let max_offset = BLOCK_SIZE * 2.;
            transform.translation.x += random::<f32>() * max_offset;
            transform.translation.y += random::<f32>() * max_offset;

            commands
                .spawn(Coin {
                    timer: Timer::new(Duration::from_secs_f32(1.0), TimerMode::Once),
                })
                .insert(SpriteBundle {
                    texture: asset_server.load("gui/coin.png"),
                    transform,
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(5.)),
                        ..Default::default()
                    },
                    ..Default::default()
                });
        }
    }

    if let Ok((_, player_transform, stats)) = player_query.get_single_mut() {
        for (entity, mut transform, mut coin) in coin_query.iter_mut() {
            if coin.timer.finished() {
                let current_pos = transform.translation.xy();

                let direction = player_transform.translation.xy() - current_pos;

                let sum = direction.x.abs() + direction.y.abs();
                if sum < Player::SIZE / 8. {
                    commands.entity(entity).despawn();
                    continue;
                }

                transform.translation +=
                    direction.normalize().extend(0.0) * stats.speed * 0.4 * time.delta_seconds();
            } else {
                coin.timer.tick(time.delta());
                transform.translation.y += time.delta_seconds() * 12.;
            }
        }
    }
}
