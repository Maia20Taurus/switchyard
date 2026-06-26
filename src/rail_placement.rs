use bevy::prelude::*;

pub struct RailPlacementPlugin;
impl Plugin for RailPlacementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, controls);
        app.add_systems(Startup, setup);
    }
}

/// The algorithm that the rail generator uses to produce the rail mesh
pub enum CurveMode {
    Linear,
    Quadratic,
    Cubic,
}

#[derive(Component)]
pub struct Rail {
    starting_point: Transform,
    ending_point: Transform,
    curve_mode: CurveMode,
}

fn create_rail_mesh(rail: Rail) -> Vec<Vec3> {
    let control_point_offset: Vec3 = Vec3::Z;

    let inner_starting_point: Vec3 = rail.starting_point.rotation.mul_vec3(control_point_offset)
    + rail.starting_point.translation;
    let inner_ending_point: Vec3 = rail.ending_point.rotation.mul_vec3(control_point_offset)
    + rail.ending_point.translation;

    let points: [[Vec3; 4]; 1]  = [[
        rail.starting_point.translation,
        inner_starting_point,
        inner_ending_point,
        rail.ending_point.translation,
    ]];
    let cubic_bezier = CubicBezier::new(points).to_curve().unwrap();
    let interpoints: Vec<Vec3> = cubic_bezier.iter_positions(10).collect();
    interpoints
}

fn controls(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Fixed>>,
) {

}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let rail = Rail {
        starting_point: Transform::from_xyz(-2.5, 0., 2.5),
        ending_point: Transform::from_xyz(2.5, 0., 3.5),
        curve_mode: CurveMode::Cubic,
    };
    let interpoints = create_rail_mesh(rail);

    for point in interpoints {
        commands.spawn((
        Mesh3d(meshes.add(Cuboid{half_size: Vec3::new(0.1, 0.1, 0.1), ..default()}.mesh())),
        MeshMaterial3d(materials.add(Color::srgb(0.5, 0.3, 0.3))),
        Transform::from_translation(point)
    ));
    }
}
