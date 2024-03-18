use std::path::{Path, PathBuf};

use bevy::{prelude::*, utils::hashbrown::HashMap};
use bevy_rapier2d::{geometry::Collider, plugin::RapierContext};
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumCount, EnumIter};

use crate::{
    animation::{Animation, AnimationController},
    animation_maker,
    gui::settings::{keybinds::Keybind, Settings},
    lang::Lang,
    misc::read_img,
    mob::Mob,
    stats::Stats,
};

use self::{
    archer::Archer, enchantress::Enchantress, knight::Knight, musketeer::Musketeer,
    swordsman::Swordsman, wizard::Wizard,
};

use super::{
    cast_collider, sprite_vec, ChainAttack, Player, PLAYER_SPRITE_SHEETS_X_SIZE, PLAYER_TEXTURE,
};

pub mod archer;
pub mod enchantress;
pub mod knight;
pub mod musketeer;
pub mod swordsman;
pub mod wizard;

#[enum_dispatch]
pub trait PlayerClass: Sync + Send + Default + Into<PlayerClasses> {
    fn name(&self) -> &'static str;
    fn translated_name(&self, lang: &Res<Lang>) -> String {
        lang.get(&format!("player.class.{}", self.name()))
            .to_owned()
    }
    fn idle_texture(&self) -> Image {
        let path = Path::new(PLAYER_TEXTURE).join(self.name()).join("Idle.png");
        let img = read_img(path).crop(
            0,
            0,
            PLAYER_SPRITE_SHEETS_X_SIZE,
            PLAYER_SPRITE_SHEETS_X_SIZE,
        );
        Image::from_dynamic(img, true)
    }

    fn normal_attack_chain_count(&self) -> u8 {
        ChainAttack::DEFAULT
    }

    fn class_animations(&self, asset_server: &Res<AssetServer>) -> HashMap<String, Animation> {
        let get_texture = |name: &str| self.get_texture_path(name);

        animation_maker!(
            asset_server,
            get_texture,
            128,
            [
                "Special_Attack_1" => (1., AnimationMode::Once, AnimationDirection::Forwards),
                "Special_Attack_2" => (1., AnimationMode::Once, AnimationDirection::Forwards),
                "Special_Attack_3" => (1., AnimationMode::Once, AnimationDirection::Forwards)
            ]
        )
    }

    fn get_texture_path(&self, name: &str) -> PathBuf {
        Path::new(PLAYER_TEXTURE)
            .join(self.name())
            .join(format!("{}.png", name))
    }
}

#[derive(Serialize, Deserialize, EnumIter, EnumCount, Clone, PartialEq, Eq)]
#[enum_dispatch(PlayerClass)]
pub enum PlayerClasses {
    Archer(Archer),
    Enchantress(Enchantress),
    Knight(Knight),
    Musketeer(Musketeer),
    Swordsman(Swordsman),
    Wizard(Wizard),
}

pub struct PlayerClassesPlugin;
impl Plugin for PlayerClassesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((Archer, Enchantress, Knight, Musketeer, Swordsman, Wizard));
    }
}

impl Default for PlayerClasses {
    fn default() -> Self {
        Self::Swordsman(Swordsman::default())
    }
}

pub fn is_of_class<C: PlayerClass>(query: Query<&Player>) -> bool {
    if let Ok(player) = query.get_single() {
        player.class == C::default().into()
    } else {
        false
    }
}

pub fn can_attack(query: Query<&AnimationController, With<Player>>) -> bool {
    if let Ok(animation_controller) = query.get_single() {
        !animation_controller
            .current_animation
            .as_ref()
            .is_some_and(|anim| anim.contains("Attack"))
    } else {
        false
    }
}

pub fn swords_user_special_attacks(
    mut player_query: Query<(
        Entity,
        &mut Player,
        &Transform,
        &TextureAtlasSprite,
        &mut AnimationController,
    )>,
    mut mob_query: Query<(&mut Mob, &mut Stats)>,
    rapier_context: Res<RapierContext>,
    settings: Res<Settings>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
) {
    if let Ok((entity, mut player, transform, sprite, mut animation_controller)) =
        player_query.get_single_mut()
    {
        for i in 1..=3 {
            let anim_name = format!("Special_Attack_{i}");
            if settings
                .keybinds
                .get_field::<Keybind>(&anim_name.to_lowercase())
                .unwrap()
                .just_pressed(&keyboard_input, &mouse_input)
            {
                animation_controller.play(&anim_name)
            }

            if animation_controller.just_finished(&anim_name) {
                let sword_class = sword_user_class_from_player_classes(&player.class).unwrap();
                let required_mana = sword_class.special_attacks_mana()[i];
                let damage = sword_class.special_attacks_damage()[i];

                if player.mana.try_remove(
                    required_mana
                ) {
                    let mut shape_pos = transform.translation.xy();
                    shape_pos += Player::SIZE / 2. * sprite_vec(sprite);

                    cast_collider(
                        entity,
                        &Collider::ball(Player::SIZE / 2.),
                        shape_pos,
                        &rapier_context,
                        |hit_entity| {
                            if let Ok((mut mob, mut stats)) = mob_query.get_mut(hit_entity) {
                                mob.hit_animation();
                                stats.take_damage(damage);
                            }

                            true
                        },
                    );
                }
            }
        }
    }
}

pub trait SwordUserClass {
    fn special_attacks_damage(&self) -> [f32; 3] {
        [5., 6., 7.]
    }
    fn special_attacks_mana(&self) -> [f32; 3] {
        [20., 30., 40.]
    }
}

pub fn sword_user_class_from_player_classes<'a>(
    class: &'a PlayerClasses,
) -> Option<&'a dyn SwordUserClass> {
    match class {
        PlayerClasses::Knight(k) => Some(k),
        PlayerClasses::Swordsman(k) => Some(k),
        PlayerClasses::Musketeer(k) => Some(k),
        _ => None,
    }
}
