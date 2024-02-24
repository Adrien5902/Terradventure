use bevy::asset::AssetPath;
use image::DynamicImage;
use std::path::Path;

pub fn read_img<'a>(path: impl Into<AssetPath<'a>>) -> DynamicImage {
    image::io::Reader::open(Path::new("assets").join(path.into().path()))
        .map_err(|e| e.to_string())
        .unwrap()
        .with_guessed_format()
        .map_err(|e| e.to_string())
        .unwrap()
        .decode()
        .unwrap()
}
