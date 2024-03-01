use bevy::prelude::*;

pub const PIXEL_FONT: &str = "fonts/Silkscreen-Bold.ttf";

#[derive(Clone)]
pub enum Background {
    Color(BackgroundColor),
    Image(Handle<Image>),
}

impl From<Background> for Option<BackgroundColor> {
    fn from(val: Background) -> Option<BackgroundColor> {
        match val {
            Background::Color(bg) => Some(bg),
            _ => None,
        }
    }
}

impl From<Background> for Option<Handle<Image>> {
    fn from(val: Background) -> Option<Handle<Image>> {
        match val {
            Background::Image(bg) => Some(bg),
            _ => None,
        }
    }
}

impl From<Handle<Image>> for Background {
    fn from(value: Handle<Image>) -> Self {
        Self::Image(value)
    }
}

impl From<Color> for Background {
    fn from(value: Color) -> Self {
        Self::Color(value.into())
    }
}

pub fn ease_out_quad(t: f32) -> f32 {
    1.0 - (1.0 - t).powf(2.0)
}

pub fn ease_in_quad(t: f32) -> f32 {
    t.powf(2.0)
}
