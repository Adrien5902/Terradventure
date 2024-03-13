use bevy::{asset::AssetPath, prelude::*};

#[derive(Component)]
pub struct ParallaxBackground {
    images: Vec<Handle<Image>>,
    direction: f32,
    pub speed_multiplier: f32,
    pub speed_offset: f32,
    aspect_ratio: f32,
}

#[derive(Component)]
pub struct ParallaxBackgroundImage {
    pub speed: f32,
}

#[derive(Bundle)]
pub struct ParallaxBackgroundImageBundle {
    pub sprite: SpriteBundle,
    pub image: ParallaxBackgroundImage,
}

impl ParallaxBackground {
    pub fn move_bg(&mut self, direction: f32) {
        self.direction = direction
    }

    pub fn new<'a>(
        images: Vec<impl Into<AssetPath<'a>>>,
        asset_server: &AssetServer,
        speed_multiplier: f32,
        speed_offset: f32,
        image_size: Vec2,
    ) -> Self {
        Self {
            images: images
                .into_iter()
                .map(|img| asset_server.load(img))
                .collect(),
            direction: 0.0,
            speed_multiplier,
            speed_offset,
            aspect_ratio: image_size.x / image_size.y,
        }
    }

    pub fn spawn(self, commands: &mut Commands) -> Entity {
        commands
            .spawn_empty()
            .with_children(|builder| {
                for (i, image) in self.images.iter().enumerate() {
                    builder.spawn(ParallaxBackgroundImageBundle {
                        sprite: SpriteBundle {
                            texture: image.clone_weak(),
                            ..Default::default()
                        },
                        image: ParallaxBackgroundImage {
                            speed: i as f32 * self.speed_multiplier + self.speed_offset,
                        },
                    });
                }
            })
            .insert(self)
            .id()
    }
}

pub struct ParallaxBackgroundPlugin;
impl Plugin for ParallaxBackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_parallax_background);
    }
}

fn update_parallax_background(
    query: Query<(&ParallaxBackground, &Children)>,
    mut bg_query: Query<(&mut Transform, &ParallaxBackgroundImage, &mut Sprite)>,
    window_query: Query<&Window>,
) {
    let window = window_query.single();
    for (bg, children) in query.iter() {
        let direction = bg.direction;
        for child in children {
            if let Ok((mut transform, image, mut sprite)) = bg_query.get_mut(*child) {
                transform.translation.x += direction * image.speed;
                sprite.custom_size =
                    Some(Vec2::new(window.width(), window.width() / bg.aspect_ratio));
            }
        }
    }
}
