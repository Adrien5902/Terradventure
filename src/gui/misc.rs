use bevy::prelude::*;

pub const PIXEL_FONT: &str = "fonts/Silkscreen-Bold.ttf";

pub fn into_text_sections(
    data: &[&'static str],
    asset_server: &Res<AssetServer>,
) -> Vec<TextSection> {
    data.iter()
        .map(|s| {
            TextSection::new(
                *s,
                TextStyle {
                    color: Color::WHITE,
                    font_size: 24.0,
                    font: asset_server.load(PIXEL_FONT),
                },
            )
        })
        .collect()
}

#[derive(Clone)]
pub enum Background {
    Color(BackgroundColor),
    Image(Handle<Image>),
}

impl Into<Option<BackgroundColor>> for Background {
    fn into(self) -> Option<BackgroundColor> {
        match self {
            Self::Color(bg) => Some(bg),
            _ => None,
        }
    }
}

impl Into<Option<Handle<Image>>> for Background {
    fn into(self) -> Option<Handle<Image>> {
        match self {
            Self::Image(bg) => Some(bg),
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
