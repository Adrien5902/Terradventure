use bevy_rapier_collider_gen::*;
use image;
use std::env;
use std::path::Path;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let path_str = args[1].as_str();
    let xsize = args[2].as_str();
    let ysize = args[3].as_str();

    let path = Path::new(&path_str);
}
