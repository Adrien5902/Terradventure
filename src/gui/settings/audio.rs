use super::{range::RangeSetting, Settings};
use crate::{
    gui::{misc::PIXEL_FONT, slider::Slider, styles::text_style},
    lang::Lang,
    music::BackgroundAudio,
};
use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, Clone, Reflect)]
pub struct AudioChannelVolumeRange {
    pub value: f32,
    pub field: String,
}

impl RangeSetting for AudioChannelVolumeRange {
    fn name(&self) -> Option<&'static str> {
        None
    }

    fn min(&self) -> f32 {
        0.
    }
    fn max(&self) -> f32 {
        100.
    }
    fn get_value(&self) -> f32 {
        self.value
    }

    fn set_value(&mut self, v: f32) {
        self.value = v
    }
}

pub fn audio_channel_volume_range_update(
    query: Query<(&Slider, &AudioChannelVolumeRange)>,
    mut settings: ResMut<Settings>,
    background_channel: Res<AudioChannel<BackgroundAudio>>,
) {
    if let Ok((slider, audio_source)) = query.get_single() {
        let new_volume = slider.value();
        if new_volume != settings.fov.get_value() {
            settings.update(|s| {
                let field = s
                    .audio
                    .get_field_mut::<AudioChannelVolumeRange>(&audio_source.field)
                    .unwrap();

                field.set_value(new_volume);
            });

            match audio_source.field.as_str() {
                "background_music" => background_channel,
                _ => panic!(),
            }
            .set_volume((new_volume / 100.) as f64);
        }
    }
}

#[derive(Reflect, Serialize, Deserialize)]
pub struct AudioChannelsVolumeRanges {
    pub background_music: AudioChannelVolumeRange,
}

impl Default for AudioChannelsVolumeRanges {
    fn default() -> Self {
        AudioChannelsVolumeRanges {
            background_music: AudioChannelVolumeRange {
                value: 100.,
                field: "background_music".into(),
            },
        }
    }
}

pub fn audio_volume_display(
    builder: &mut ChildBuilder,
    channels: &AudioChannelsVolumeRanges,
    asset_server: &Res<AssetServer>,
    lang: &Res<Lang>,
) {
    builder.spawn(TextBundle::from_section(
        lang.get("ui.settings.audio.title"),
        TextStyle {
            font: asset_server.load(PIXEL_FONT),
            font_size: 50.,
            ..Default::default()
        },
    ));

    channels.iter_fields().enumerate().for_each(|(index, _)| {
        let field_name = channels.name_at(index).unwrap();
        let source: &AudioChannelVolumeRange = channels.get_field(field_name).unwrap();

        builder
            .spawn(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    justify_content: JustifyContent::SpaceBetween,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|builder| {
                builder.spawn(TextBundle::from_section(
                    format!(
                        "{} : ",
                        lang.get(&format!("ui.settings.audio.{}", source.field))
                    ),
                    text_style(&asset_server),
                ));
                source.clone().to_slider(builder, asset_server, lang);
            });
    });
}
