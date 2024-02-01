use bevy::prelude::*;

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
                    font: asset_server.load("fonts/Silkscreen-Bold.ttf"),
                },
            )
        })
        .collect()
}
