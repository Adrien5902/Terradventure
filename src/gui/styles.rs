use bevy::prelude::*;

use super::misc::PIXEL_FONT;

pub fn scroll_button_style() -> Style {
    Style {
        width: Val::Px(270.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(10.0)),
        ..aligned_center()
    }
}

pub fn aligned_center() -> Style {
    Style {
        display: Display::Flex,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

pub fn text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load(PIXEL_FONT),
        font_size: 24.,
        ..Default::default()
    }
}
