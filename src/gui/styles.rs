use bevy::prelude::*;

pub fn scroll_button_style() -> Style {
    Style {
        width: Val::Px(200.0),
        height: Val::Px(50.0),
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
