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

pub fn ease_out_quad(t: f32) -> f32 {
    1.0 - (1.0 - t).powf(2.0)
}

pub fn ease_in_quad(t: f32) -> f32 {
    t.powf(2.0)
}
