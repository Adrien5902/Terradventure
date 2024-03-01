use self::{
    buttons::scroll::button_interact, main_menu::MainMenuPlugin, misc::Background,
    pause::PausePlugin, slider::SliderPlugin, styles::aligned_center,
};
use bevy::prelude::*;
use bevy_simple_text_input::TextInputPlugin;

pub mod buttons;
pub mod main_menu;
pub mod misc;
pub mod pause;
pub mod settings;
pub mod slider;
pub mod styles;

pub struct GuiPlugin;
impl Plugin for GuiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((PausePlugin, MainMenuPlugin, SliderPlugin))
            .add_systems(Update, button_interact)
            .add_plugins(TextInputPlugin);
    }
}

pub fn make_menu<T: Component>(
    commands: &mut Commands,
    bg: Background,
    typ: T,
    builder: impl FnOnce(&mut ChildBuilder),
    z_index: Option<ZIndex>,
    flex_direction: Option<FlexDirection>,
) {
    let bg_color: Option<BackgroundColor> = bg.clone().into();

    commands
        .spawn((
            typ,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: flex_direction.unwrap_or(FlexDirection::Column),
                    ..aligned_center()
                },
                z_index: z_index.unwrap_or_default(),
                background_color: bg_color.unwrap_or_default(),
                ..Default::default()
            },
        ))
        .with_children(builder);
}
