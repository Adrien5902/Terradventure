use bevy::prelude::*;

use crate::player::character_controller_update;

#[derive(Component)]
pub struct ParallaxBackground {
    pub layers: Vec<Handle<Image>>,
    pub speed_multiplier: f32,
    pub speed_offset: f32,
    pub z_offset: f32,
    pub camera: Entity,
    pub image_size: Vec2,
}

impl ParallaxBackground {
    const INSIDE_LAYER_COUNT: usize = 3;

    pub fn get_offset(&self, camera: &Camera, camera_transform: &GlobalTransform) -> Vec3 {
        let world_pos_top = camera
            .viewport_to_world(camera_transform, Vec2::new(0.0, 0.0))
            .map(|r| r.origin.truncate())
            .unwrap();

        Vec3::new(world_pos_top.x, world_pos_top.y, self.z_offset)
    }
}

#[derive(Component)]
pub struct ParallaxBackgroundLayerItem {
    pub layer_index: usize,
    pub inside_layer_index: usize,
}

#[derive(Bundle)]
pub struct ParallaxBackgroundLayerItemBundle {
    pub sprite: SpriteBundle,
    pub layer: ParallaxBackgroundLayerItem,
}

pub struct ParallaxBackgroundPlugin;
impl Plugin for ParallaxBackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_parallax_background.after(character_controller_update),
                spawn_background,
            ),
        );
    }
}

fn update_parallax_background(
    query: Query<(&ParallaxBackground, &Children)>,
    camera_query: Query<(&Transform, &Camera)>,
    mut bg_query: Query<
        (&mut Transform, &ParallaxBackgroundLayerItem, &mut Sprite),
        Without<Camera>,
    >,
) {
    for (bg, children) in query.iter() {
        if let Ok((cam_trans, camera)) = camera_query.get(bg.camera) {
            let cam_size = camera.physical_viewport_size().unwrap().as_vec2();
            for child in children {
                if let Ok((mut transform, bg_item, mut sprite)) = bg_query.get_mut(*child) {
                    let img_size = cam_size / 2.5;
                    let layer_speed =
                        bg_item.layer_index as f32 * bg.speed_multiplier + bg.speed_offset;

                    sprite.custom_size = Some(img_size);

                    let inside_layer_offset = (bg_item.inside_layer_index as f32 - 1.) * img_size.x;
                    let camera_offset = (cam_trans.translation.x * layer_speed) % img_size.x;

                    let x = inside_layer_offset - camera_offset;
                    let z = bg_item.layer_index as f32;
                    transform.translation = Vec3::new(x, 0.0, z)
                }
            }
        }
    }
}

fn spawn_background(
    mut commands: Commands,
    query: Query<(Entity, &ParallaxBackground), Changed<ParallaxBackground>>,
) {
    for (entity, bg) in query.iter() {
        commands.entity(entity).despawn_descendants().insert((
            TransformBundle::from_transform(Transform::from_xyz(0.0, 0.0, bg.z_offset)),
            InheritedVisibility::default(),
        ));

        for (layer_index, image) in bg.layers.iter().enumerate() {
            for inside_layer_index in 0..ParallaxBackground::INSIDE_LAYER_COUNT {
                commands.entity(entity).with_children(|builder| {
                    builder.spawn(ParallaxBackgroundLayerItemBundle {
                        layer: ParallaxBackgroundLayerItem {
                            inside_layer_index,
                            layer_index,
                        },
                        sprite: SpriteBundle {
                            texture: image.clone_weak(),
                            ..Default::default()
                        },
                    });
                });
            }
        }

        commands.entity(bg.camera).add_child(entity);
    }
}
