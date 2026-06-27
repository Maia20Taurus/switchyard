use bevy::{
    prelude::*,
};

mod camera_controls;
mod rail_placement;
use camera_controls::CameraControlsPlugin;
use rail_placement::RailPlacementPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraControlsPlugin)
        .add_plugins(RailPlacementPlugin)
        .add_systems(Startup, create_objects)
        .run();
}


fn create_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50., 50.))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));

    // Cuboid
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default().mesh())),
        MeshMaterial3d(materials.add(Color::srgb(0.5, 0.3, 0.3))),
        Transform::from_xyz(3., 0.5, 0.),
    ));

    // directional 'sun' light
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(150.0, 200.0, 150.0),
            ..default()
        }.looking_at(Vec3::ZERO, Vec3::Y),
    ));



}
