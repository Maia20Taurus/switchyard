use bevy::{camera::ScalingMode, prelude::*};

pub struct CameraControlsPlugin;

fn add_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 6.0,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(30.0, 30.0, 30.0)
        .looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

impl Plugin for CameraControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_camera);
    }
}