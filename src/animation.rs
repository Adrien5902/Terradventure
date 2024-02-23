use std::time::Duration;

use bevy::{prelude::*, utils::HashMap};

use crate::state::AppState;

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
    current_animation: Option<&'static str>,
    default_animation: Option<&'static str>,
    backwards: bool,
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
        self.play(name);
        self.default_animation = Some(name);
        self
    }

    pub fn current_animation(&self) -> Option<&Animation> {
        self.current_animation
            .or(self.default_animation)
            .and_then(|name| Some(self.animations.get(name).unwrap()))
    }

    pub fn play(&mut self, name: &'static str) {
        let animation = self
            .animations
            .get(name)
            .ok_or(AnimationError::AnimationNotFound)
            .unwrap();

        self.current_animation = Some(name);
        self.timer.reset();
        self.timer.set_mode(animation.mode.into());
        self.timer.set_duration(animation.duration);
        self.backwards = false;
    }

    pub fn stop(&mut self) {
        self.timer.reset();
        if let Some(default) = self.default_animation {
            self.play(default);
        }
    }

    pub fn tick(&mut self, time: &Res<Time>) {
        self.timer.tick(time.delta());
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

            if controller.timer.just_finished() {
                if animation.direction == AnimationDirection::BackAndForth {
                    controller.backwards = !backwards;
                }

                if animation.mode == AnimationMode::Once {
                    controller.stop();
                }
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
    ($asset_server:expr, $assets:expr, $asset_type:ident, $tile_size:expr, [ $( $name:expr => ($duration:expr, $frames:expr, $mode:expr, $direction:expr) ),* ]) => {{
        use std::time::Duration;
        use crate::animation::{AnimationMode, Animation, AnimationDirection};

        let mut map = HashMap::new();
        $(
            map.insert(
                $name,
                Animation {
                    duration: Duration::from_secs_f32($duration),
                    frames: $frames,
                    texture: $assets.add(TextureAtlas::from_grid(
                        $asset_server.load($asset_type($name)),
                        Vec2::splat($tile_size),
                        $frames,
                        1,
                        None,
                        None,
                    )),
                    mode: $mode,
                    direction: $direction,
                },
            );
        )*
        map
    }};
}
