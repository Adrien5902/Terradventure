use serde::{Deserialize, Serialize};

use super::PlayerClass;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Knight;

impl PlayerClass for Knight {
    fn name(&self) -> &'static str {
        "knight"
    }
    fn special_attack_1(
        &self,
        player: bevy::prelude::Entity,
        rapier_context: &bevy::prelude::Res<bevy_rapier2d::prelude::RapierContext>,
        transform: &bevy::prelude::Transform,
        flipped: bool,
        mob_query: &mut bevy::prelude::Query<
            (&mut crate::stats::Stats, &mut crate::mob::Mob),
            bevy::prelude::Without<crate::player::Player>,
        >,
    ) {
    }
    fn special_attack_2(
        &self,
        player: bevy::prelude::Entity,
        rapier_context: &bevy::prelude::Res<bevy_rapier2d::prelude::RapierContext>,
        transform: &bevy::prelude::Transform,
        flipped: bool,
        mob_query: &mut bevy::prelude::Query<
            (&mut crate::stats::Stats, &mut crate::mob::Mob),
            bevy::prelude::Without<crate::player::Player>,
        >,
    ) {
    }
    fn special_attack_3(
        &self,
        player: bevy::prelude::Entity,
        rapier_context: &bevy::prelude::Res<bevy_rapier2d::prelude::RapierContext>,
        transform: &bevy::prelude::Transform,
        flipped: bool,
        mob_query: &mut bevy::prelude::Query<
            (&mut crate::stats::Stats, &mut crate::mob::Mob),
            bevy::prelude::Without<crate::player::Player>,
        >,
    ) {
    }
    fn normal_attack_chain_count(&self) -> u8 {
        4
    }
}
