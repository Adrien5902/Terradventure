use bevy::prelude::*;
use strum::IntoEnumIterator;

use crate::{
    gui::{misc::PIXEL_FONT, styles::text_style},
    lang::{Lang, LangIdentifier, Langs},
};

use super::Settings;

pub fn lang_chooser(
    builder: &mut ChildBuilder,
    current_lang: &LangIdentifier,
    asset_server: &Res<AssetServer>,
) {
    builder
        .spawn(NodeBundle {
            background_color: Color::NONE.into(),
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                "Langs",
                TextStyle {
                    font: asset_server.load(PIXEL_FONT),
                    font_size: 40.,
                    ..Default::default()
                },
            ));

            for lang in Langs::iter() {
                let lang: Lang = lang.into();
                builder
                    .spawn(LangChooseButton {
                        ident: lang.ident.clone(),
                    })
                    .insert(ButtonBundle {
                        border_color: BorderColor(Color::WHITE),
                        background_color: Color::NONE.into(),
                        style: Style {
                            border: UiRect::all(Val::Px(if lang.ident == *current_lang {
                                4.
                            } else {
                                0.
                            })),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|builder| {
                        builder.spawn(TextBundle::from_section(
                            lang.name,
                            text_style(asset_server),
                        ));
                    });
            }
        });
}

#[derive(Component)]
pub struct LangChooseButton {
    ident: LangIdentifier,
}

pub fn lang_choose_buttons_update(
    mut query: Query<(&Interaction, &LangChooseButton, &mut Style)>,
    mut settings: ResMut<Settings>,
    mut lang_res: ResMut<Lang>,
) {
    for (interaction, button, mut style) in query.iter_mut() {
        if *interaction == Interaction::Pressed {
            settings.update_lang(button.ident.clone(), &mut lang_res)
        }

        style.border = UiRect::all(Val::Px(if button.ident == settings.lang {
            4.
        } else {
            0.
        }));
    }
}
