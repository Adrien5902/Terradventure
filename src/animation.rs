use std::time::Duration;

use bevy::{asset::AssetPath, prelude::*, utils::HashMap};

use crate::{misc::read_img, state::AppState};

pub struct AnimationPlugin;
impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_animators.run_if(in_state(AppState::InGame)));
    }
}

#[derive(Bundle)]
pub struct AnimatedSpriteBundle {
    pub sprite: SpriteSheetBundle,
    pub animation_controller: AnimationController,
}

#[derive(Component)]
/// Composant qu'on ajoute aux entités pour controller les animations du sprite
pub struct AnimationController {
    /// Timer pour garder en mémoire l'avancement de l'animation
    pub timer: Timer,
    /// HashmMap (comme un dictionnaire python) contenant les animations de l'entité
    pub animations: HashMap<String, Animation>,
    /// Animation actuelle
    pub current_animation: Option<String>,
    /// Animation par défaut
    pub default_animation: Option<String>,
    pub just_finished: Option<String>,
    pub backwards: bool,
}

impl AnimationController {
    pub fn new(animations: HashMap<String, Animation>) -> Self {
        Self {
            timer: Timer::default(),
            animations,
            current_animation: None,
            default_animation: None,
            just_finished: None,
            backwards: true,
        }
    }

    pub fn with_default(mut self, name: &str) -> Self {
        if self.animations.get(name).is_some() {
            self.default_animation = Some(name.to_owned());
            self.play(name);
        }
        self
    }

    pub fn current_animation(&self) -> Option<&Animation> {
        self.current_animation
            .clone()
            .or(self.default_animation.clone())
            .map(|name| self.get_animation(&name))
    }

    pub fn get_animation(&self, name: &str) -> &Animation {
        self.animations
            .get(&name.to_owned())
            .ok_or(AnimationError::AnimationNotFound)
            .unwrap()
    }

    /// Jouer une animation avec son nom
    pub fn play(&mut self, name: &str) {
        let animation = self.get_animation(name).clone();

        if Some(name.to_owned()) != self.default_animation {
            self.current_animation = Some(name.to_owned());
        }

        self.timer.reset();
        self.timer.set_mode(animation.mode.into());
        self.timer.set_duration(animation.duration);
        self.timer.unpause();
        self.backwards = false;
    }

    pub fn stop(&mut self) {
        if let Some(default) = self.default_animation.clone() {
            self.play(&default);
        }
        self.current_animation = None;
    }

    /// Mise à jour de l'animation à chaque tick
    pub fn tick(&mut self, time: &Time) {
        self.timer.tick(time.delta());
        self.just_finished = None;

        if let Some(animation_name) = self.current_animation.clone() {
            let animation = self.get_animation(&animation_name).clone();
            if self.timer.just_finished() {
                if animation.mode == AnimationMode::Once || animation.mode == AnimationMode::Custom
                {
                    self.just_finished = self.current_animation.clone();
                    if animation.mode != AnimationMode::Custom {
                        self.stop();
                    }
                } else if animation.direction == AnimationDirection::BackAndForth {
                    let backwards = self.backwards;
                    self.backwards = !backwards;
                }
            }
        }
    }

    pub fn just_finished(&self, name: &str) -> bool {
        self.just_finished == Some(name.to_owned())
    }
}

#[derive(Clone, Debug)]
pub struct Animation {
    /// Image correspondant à l'animation
    pub texture: Handle<TextureAtlas>,
    /// Mode de l'animation répétée ou non ou controllée par un script externe
    pub mode: AnimationMode,
    pub direction: AnimationDirection,
    pub duration: Duration,
    /// Nombre d'image dans l'animation
    pub frames: usize,
}

impl Animation {
    pub fn new<'a>(
        path: impl Into<AssetPath<'a>>,
        asset_server: &AssetServer,
        duration: Duration,
        tile_size: u32,
        mode: AnimationMode,
        direction: AnimationDirection,
    ) -> Self {
        let img = read_img(path);
        let height = img.height();
        let frames = (img.width() / tile_size) as usize;
        Self {
            duration,
            frames,
            texture: asset_server.add(TextureAtlas::from_grid(
                asset_server.add(Image::from_dynamic(img, true)),
                Vec2::new(tile_size as f32, height as f32),
                frames,
                1,
                None,
                None,
            )),
            mode,
            direction,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AnimationMode {
    /// L'animation est joué qu'une seule fois pour les attaques par exemple
    Once,
    /// L'avancée de l'animation est controllée par un autre bout de code
    Custom,
    /// L'animation se répéte pour une idle par exemple
    Repeating,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum AnimationDirection {
    /// L'animation se joue dans l'ordre normale
    #[default]
    Forwards,
    /// L'animation se joue à l'endroit puis à l'envers
    BackAndForth,
}

impl From<AnimationMode> for TimerMode {
    fn from(val: AnimationMode) -> TimerMode {
        match val {
            AnimationMode::Once => TimerMode::Once,
            _ => TimerMode::Repeating,
        }
    }
}

fn update_animators(
    mut query: Query<(
        &mut AnimationController,
        &mut TextureAtlasSprite,
        &mut Handle<TextureAtlas>,
    )>,
    time: Res<Time>,
) {
    for (mut controller, mut sprite, mut texture) in query.iter_mut() {
        if let Some(animation) = controller.current_animation() {
            let animation = animation.clone();
            let backwards = controller.backwards;

            *texture = animation.texture.clone_weak();

            if !controller.timer.paused() {
                sprite.index = (match backwards {
                    false => controller.timer.percent(),
                    true => controller.timer.percent_left(),
                } * animation.frames as f32) as usize
                    % animation.frames;
            }

            if animation.mode != AnimationMode::Custom {
                controller.tick(&time);
            }
        }
    }
}

#[derive(Debug)]
pub enum AnimationError {
    AnimationNotFound,
}

#[macro_export]
macro_rules! animation_maker {
    ($assets_server:expr, $asset_type:ident, $tile_size:expr, [ $( $name:expr => ($duration:expr, $mode:expr, $direction:expr) ),* ]) => {{
        use std::time::Duration;
        use crate::animation::{AnimationMode, Animation, AnimationDirection};

        let mut map = HashMap::new();
        $(
            map.insert(
                $name.to_owned(),
                Animation::new($asset_type($name), $assets_server, Duration::from_secs_f32($duration), $tile_size, $mode, $direction),
            );
        )*
        map
    }};
}
