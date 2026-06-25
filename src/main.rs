use bevy::prelude::*;

mod camera_controls;
use camera_controls::CameraControlsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraControlsPlugin)
        .run();
}


