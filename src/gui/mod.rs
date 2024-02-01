use self::{
    buttons::scroll::button_interact, main_menu::MainMenuPlugin, pause::PausePlugin,
    styles::aligned_center,
};
use bevy::prelude::*;

pub mod buttons;
pub mod main_menu;
pub mod misc;
pub mod pause;
pub mod settings;
pub mod styles;

pub struct GuiPlugin;
impl Plugin for GuiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((PausePlugin, MainMenuPlugin))
            .add_systems(Update, button_interact);
    }
}

pub fn make_menu<T: Component>(
    mut commands: Commands,
    bg_color: BackgroundColor,
    typ: T,
    builder: impl FnOnce(&mut ChildBuilder),
    z_index: Option<ZIndex>,
) {
    commands
        .spawn((
            typ,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    ..aligned_center()
                },
                z_index: z_index.unwrap_or_default(),
                background_color: bg_color,
                ..Default::default()
            },
        ))
        .with_children(builder);
}
