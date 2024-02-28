use std::time::Duration;

use bevy::{animation, asset::AssetPath, prelude::*, utils::HashMap};

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
pub struct AnimationController {
    pub timer: Timer,
    pub animations: HashMap<&'static str, Animation>,
    pub current_animation: Option<&'static str>,
    pub default_animation: Option<&'static str>,
    pub backwards: bool,
}

impl AnimationController {
    pub fn new(animations: HashMap<&'static str, Animation>) -> Self {
        Self {
            timer: Timer::default(),
            animations,
            current_animation: None,
            default_animation: None,
            backwards: true,
        }
    }

    pub fn with_default(mut self, name: &'static str) -> Self {
        self.default_animation = Some(name);
        self.play(name);
        self
    }

    pub fn current_animation(&self) -> Option<&Animation> {
        self.current_animation
            .or(self.default_animation)
            .and_then(|name| Some(self.animations.get(name).unwrap()))
    }

    pub fn get_animation(&self, name: &'static str) -> &Animation {
        self.animations
            .get(name)
            .ok_or(AnimationError::AnimationNotFound)
            .unwrap()
    }

    pub fn play(&mut self, name: &'static str) {
        let animation = self.get_animation(name).clone();

        if Some(name) != self.default_animation {
            self.current_animation = Some(name);
        }

        self.timer.reset();
        self.timer.set_mode(animation.mode.into());
        self.timer.set_duration(animation.duration);
        self.timer.unpause();
        self.backwards = false;
    }

    pub fn stop(&mut self) {
        if let Some(default) = self.default_animation {
            self.play(default);
        }
        self.current_animation = None;
    }

    pub fn tick(&mut self, time: &Res<Time>) {
        self.timer.tick(time.delta());

        if let Some(animation_name) = self.current_animation {
            let animation = self.get_animation(animation_name);
            if self.timer.just_finished() {
                if animation.mode == AnimationMode::Once {
                    self.stop();
                } else if animation.direction == AnimationDirection::BackAndForth {
                    let backwards = self.backwards;
                    self.backwards = !backwards;
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct Animation {
    pub texture: Handle<TextureAtlas>,
    pub mode: AnimationMode,
    pub direction: AnimationDirection,
    pub duration: Duration,
    pub frames: usize,
}

impl Animation {
    pub fn new<'a>(
        path: impl Into<AssetPath<'a>>,
        assets_img: &mut ResMut<Assets<Image>>,
        assets_texture_atlas: &mut ResMut<Assets<TextureAtlas>>,
        duration: Duration,
        tile_size: u32,
        mode: AnimationMode,
        direction: AnimationDirection,
    ) -> Self {
        let img = read_img(path);
        let frames = (img.width() / tile_size) as usize;
        Self {
            duration,
            frames,
            texture: assets_texture_atlas.add(TextureAtlas::from_grid(
                assets_img.add(Image::from_dynamic(img, true)),
                Vec2::splat(tile_size as f32),
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AnimationMode {
    Once,
    Custom,
    Repeating,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum AnimationDirection {
    #[default]
    Forwards,
    BackAndForth,
}

impl Into<TimerMode> for AnimationMode {
    fn into(self) -> TimerMode {
        match self {
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

            sprite.index = (match backwards {
                false => controller.timer.percent(),
                true => controller.timer.percent_left(),
            } * animation.frames as f32) as usize
                % animation.frames;

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
    ($assets_img:expr, $assets_texture_atlas:expr, $asset_type:ident, $tile_size:expr, [ $( $name:expr => ($duration:expr, $frames:expr, $mode:expr, $direction:expr) ),* ]) => {{
        use std::time::Duration;
        use crate::animation::{AnimationMode, Animation, AnimationDirection};

        let mut map = HashMap::new();
        $(
            map.insert(
                $name,
                Animation::new($asset_type($name), $assets_img, $assets_texture_atlas, Duration::from_secs_f32($duration), $tile_size, $mode, $direction),
            );
        )*
        map
    }};
}
