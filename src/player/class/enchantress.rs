use crate::{
    animation::AnimationController,
    animation_maker,
    gui::settings::{keybinds::Keybind, Settings},
    mob::Mob,
    player::{cast_collider, sprite_vec, Player},
    stats::Stats,
    world::BLOCK_SIZE,
};

use super::{is_of_class, PlayerClass};
use bevy::{prelude::*, sprite::Anchor, utils::hashbrown::HashMap};
use bevy_rapier2d::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Enchantress;

impl Enchantress {
    const SPECIAL_ATTACK_1_MANA: f32 = 25.;
    const SPECIAL_ATTACK_1_DAMAGE: f32 = 7.;
    const SHIELD_MANA_USE: f32 = 20.;
    const SPECIAL_ATTACK_2_MANA_USE: f32 = 30.;
    const SPECIAL_ATTACK_2_DAMAGE_PER_TICK: f32 = 18.;
}

impl PlayerClass for Enchantress {
    fn name(&self) -> &'static str {
        "enchantress"
    }

    fn normal_attack_chain_count(&self) -> u8 {
        4
    }

    fn class_animations(
        &self,
        asset_server: &Res<AssetServer>,
    ) -> HashMap<String, crate::animation::Animation> {
        let get_texture = |name: &str| self.get_texture_path(name);

        animation_maker!(
            asset_server,
            get_texture,
            128,
            [
                "Special_Attack_1" => (0.4, AnimationMode::Once, AnimationDirection::Forwards),
                "Special_Attack_2" => (1.5, AnimationMode::Once, AnimationDirection::Forwards),
                "Special_Attack_3" => (1., AnimationMode::Custom, AnimationDirection::Forwards)
            ]
        )
    }
}

impl Plugin for Enchantress {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, (special_attacks).run_if(is_of_class::<Self>));
    }
}

fn special_attacks(
    mut player_query: Query<(
        Entity,
        &mut Player,
        &Transform,
        &mut TextureAtlasSprite,
        &mut AnimationController,
    )>,
    settings: Res<Settings>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    rapier_context: Res<RapierContext>,
    time: Res<Time>,
    mut mob_query: Query<(&mut Stats, &mut Mob)>,
) {
    if let Ok((entity, mut player, transform, mut sprite, mut animation_controller)) =
        player_query.get_single_mut()
    {
        let mut hit = |damage: f32, collider: &Collider| {
            let mut hitbox_translation = transform.translation.xy();
            hitbox_translation += sprite_vec(&sprite) * BLOCK_SIZE;
            cast_collider(
                entity,
                collider,
                hitbox_translation,
                &rapier_context,
                |hit_entity| {
                    if let Ok((mut stats, mut mob)) = mob_query.get_mut(hit_entity) {
                        stats.take_damage(damage);
                        mob.hit_animation();
                    }
                    true
                },
            );
        };

        if animation_controller.just_finished("Special_Attack_1") {
            hit(
                Enchantress::SPECIAL_ATTACK_1_DAMAGE,
                &Collider::ball(BLOCK_SIZE),
            )
        }

        if animation_controller.current_animation == Some("Special_Attack_2".into()) {
            if animation_controller.timer.percent() > 0.3
                && animation_controller.timer.percent() < 0.8
            {
                if player
                    .mana
                    .try_remove(Enchantress::SPECIAL_ATTACK_2_MANA_USE * time.delta_seconds())
                {
                    hit(
                        Enchantress::SPECIAL_ATTACK_2_DAMAGE_PER_TICK * time.delta_seconds(),
                        &Collider::capsule_x(BLOCK_SIZE * 1.8, BLOCK_SIZE),
                    );
                } else {
                    animation_controller.stop()
                }
            }

            let mut default_anchor = Player::SPRITE_ANCHOR.as_vec();
            default_anchor.x -= 0.3 * sprite_vec(&sprite).x;
            sprite.anchor = Anchor::Custom(default_anchor);
        } else {
            sprite.anchor = Player::SPRITE_ANCHOR;
        }

        if !animation_controller
            .current_animation
            .as_ref()
            .is_some_and(|anim| anim.contains("Attack"))
        {
            for i in 1..=3 {
                let capital_name = format!("Special_Attack_{}", i);
                if settings
                    .keybinds
                    .get_field::<Keybind>(&format!("special_attack_{}", i))
                    .unwrap()
                    .just_pressed(&keyboard_input, &mouse_input)
                    && (i != 1 || player.mana.try_remove(Enchantress::SPECIAL_ATTACK_1_MANA))
                {
                    animation_controller.play(&capital_name);
                }
            }
        }

        let animation_passed_shield = animation_controller.timer.percent() > 0.6;

        if animation_controller
            .current_animation
            .as_ref()
            .is_some_and(|n| n == "Special_Attack_3")
        {
            if settings
                .keybinds
                .special_attack_3
                .pressed(&keyboard_input, &mouse_input)
            {
                if animation_passed_shield
                    && player
                        .mana
                        .try_remove(Enchantress::SHIELD_MANA_USE * time.delta_seconds())
                {
                    sprite.index = 4;
                } else {
                    animation_controller.tick(&time)
                }
            } else if animation_passed_shield {
                animation_controller.tick(&time);
                if animation_controller.timer.just_finished() {
                    animation_controller.stop()
                }
            } else {
                animation_controller.stop()
            }
        }
    }
}
