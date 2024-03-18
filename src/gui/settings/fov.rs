use super::{range::RangeSetting, Settings};
use crate::gui::slider::Slider;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub const FOV_MULTIPLIER: f32 = 0.01;

#[derive(Component, Serialize, Deserialize, Copy, Clone)]
pub struct FovRange {
    pub value: f32,
}

impl RangeSetting for FovRange {
    fn name(&self) -> Option<&'static str> {
        Some("fov")
    }

    fn min(&self) -> f32 {
        10.
    }
    fn max(&self) -> f32 {
        70.
    }
    fn get_value(&self) -> f32 {
        self.value
    }
    fn set_value(&mut self, v: f32) {
        self.value = v;
    }
}

pub fn fov_update(
    query: Query<&Slider, With<FovRange>>,
    mut camera: Query<&mut OrthographicProjection, With<Camera2d>>,
    mut settings: ResMut<Settings>,
) {
    if let Ok(slider) = query.get_single() {
        let new_fov = slider.value();
        if new_fov != settings.fov.get_value() {
            settings.update(|s| s.fov = FovRange { value: new_fov });

            if let Ok(mut projection) = camera.get_single_mut() {
                projection.scale = new_fov * FOV_MULTIPLIER
            }
        }
    }
}
