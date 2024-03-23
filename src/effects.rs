use crate::state::AppState;
use bevy::{prelude::*, utils::hashbrown::HashMap};

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Effect {
    Levitation,
}

pub struct EffectData {
    pub timer: Timer,
    pub level: u8,
}

impl EffectData {
    pub fn new(duration: f32, level: u8) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Once),
            level,
        }
    }
}

#[derive(Component, Default)]
pub struct EffectsController {
    effects: HashMap<Effect, EffectData>,
}

impl EffectsController {
    pub fn add_new(&mut self, effect: Effect, seconds: f32, level: u8) {
        self.add(effect, EffectData::new(seconds, level))
    }

    pub fn get_effect(&self, effect: &Effect) -> Option<&EffectData> {
        self.effects.get(effect)
    }

    pub fn add(&mut self, effect: Effect, data: EffectData) {
        if let Some(current_data) = self.effects.get_mut(&effect) {
            if current_data.timer.remaining_secs() < data.timer.remaining_secs() {
                current_data.timer = data.timer;
            }

            if current_data.level < data.level {
                current_data.level = data.level;
            }
        } else {
            self.effects.insert(effect, data);
        }
    }

    pub fn clear_many(&mut self, effects: Vec<Effect>) {
        for effect in effects {
            self.effects.remove(&effect);
        }
    }

    pub fn clear_all(&mut self) {
        self.effects.clear();
    }
}

pub struct EffectsPlugin;
impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, effects_update.run_if(in_state(AppState::InGame)));
    }
}

fn effects_update(mut query: Query<&mut EffectsController>, time: Res<Time>) {
    let mut effects_to_remove = Vec::new();

    for mut controller in query.iter_mut() {
        for (effect, data) in controller.effects.iter_mut() {
            data.timer.tick(time.delta());

            if data.timer.finished() {
                effects_to_remove.push(effect.clone());
                continue;
            }

            // Each effect's implementation
            match effect {
                Effect::Levitation => {} //Implemented in [`character_controller_update`] ./player/
            }
        }

        for effect in &effects_to_remove {
            controller.effects.remove(effect);
        }

        effects_to_remove.clear();
    }
}
