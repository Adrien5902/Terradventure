use super::{range::RangeSetting, Settings};
use crate::gui::slider::Slider;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub const FOV_MULTIPLIER: f32 = 0.01;

#[derive(Component, Serialize, Deserialize, Copy, Clone)]
pub struct FovRange {
    value: f32,
}

impl RangeSetting for FovRange {
    fn min(&self) -> f32 {
        10.
    }
    fn max(&self) -> f32 {
        40.
    }
    fn get_value(&self) -> f32 {
        self.value
    }
    fn from_value(v: f32) -> Self {
        Self { value: v }
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
            settings.update(|s| s.fov = FovRange::from_value(new_fov));

            if let Ok(mut projection) = camera.get_single_mut() {
                projection.scale = new_fov * FOV_MULTIPLIER
            }
        }
    }
}
