use bevy::{camera::ScalingMode, prelude::*};

pub struct CameraControlsPlugin;

#[derive(Component)]
pub struct PrimaryCamera;

fn add_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 6.0,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(2.0, 10.0, 40.0)
        .looking_at(Vec3::ZERO, Vec3::Y),
        PrimaryCamera,
    ));
}

fn controls(
    camera_query: Single<(&mut Camera, &mut Transform, &mut Projection)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Fixed>>,
) {
    let (mut camera, mut transform, mut projection) = camera_query.into_inner();

    let cam_speed = 1.5 * time.delta_secs();

    // Camera movement controls
    if input.pressed(KeyCode::KeyD) {
        transform.translation.x += cam_speed;
    }
    if input.pressed(KeyCode::KeyA) {
        transform.translation.x -= cam_speed;
    }
    if input.pressed(KeyCode::KeyW) {
        transform.translation.z -= cam_speed;
    }
    if input.pressed(KeyCode::KeyS) {
        transform.translation.z += cam_speed;
    }

}

impl Plugin for CameraControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_camera);
        app.add_systems(Update, controls);
    }
}