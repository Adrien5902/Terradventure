use bevy::prelude::*;

use crate::gui::styles::{scroll_button_style, text_style};

#[derive(Component)]
pub struct ScrollButton;

pub fn make_button<T: Component>(
    builder: &mut ChildBuilder,
    text: &str,
    typ: T,
    asset_server: &Res<AssetServer>,
) {
    builder
        .spawn((
            typ,
            ScrollButton,
            ButtonBundle {
                style: scroll_button_style(),
                image: UiImage::new(asset_server.load("scroll.png")),
                background_color: BackgroundColor(Color::WHITE),
                ..Default::default()
            },
        ))
        .with_children(|text_builder| {
            text_builder.spawn(TextBundle::from_section(text, text_style(asset_server)));
        });
}

pub fn button_interact(
    query: Query<(&Children, &Interaction), With<ScrollButton>>,
    mut text_query: Query<&mut Text>,
) {
    for (children, interaction) in query.iter() {
        for child in children.iter() {
            if let Ok(mut text) = text_query.get_mut(*child) {
                if text.sections[0].style.color != Color::RED {
                    text.sections[0].style.color = match *interaction {
                        Interaction::Hovered => Color::BLACK,
                        Interaction::None => Color::WHITE,
                        Interaction::Pressed => Color::WHITE,
                    }
                }
            }
        }
    }
}
