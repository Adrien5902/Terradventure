use crate::gui::slider::*;
use bevy::prelude::*;

pub trait RangeSetting: Component + Sized {
    fn get_value(&self) -> f32;
    fn min(&self) -> f32;
    fn max(&self) -> f32;
    fn from_value(v: f32) -> Self;

    fn step(&self) -> f32 {
        1.
    }

    fn to_slider(self, builder: &mut ChildBuilder) {
        let slider = Slider::new(self.min(), self.max()).with_step(self.step());
        let value = self.get_value();
        builder
            .spawn((
                self,
                SliderBundle {
                    slider: slider.with_value(value).unwrap_or(slider),
                    style: Style {
                        width: Val::Px(200.),
                        height: Val::Px(20.),
                        margin: UiRect::bottom(Val::Px(15.)),
                        ..default()
                    },
                    background_color: Color::rgb(0.8, 0.8, 0.8).into(),
                    ..Default::default()
                },
            ))
            .with_children(|builder| {
                builder.spawn(SliderHandleBundle {
                    style: Style {
                        width: Val::Px(20.),
                        height: Val::Px(20.),
                        ..Default::default()
                    },
                    background_color: Color::rgb(0.9, 0.9, 0.9).into(),
                    slider_handle: SliderHandle,
                    ..Default::default()
                });
            });
    }
}
