use bevy::{mesh::{PrimitiveTopology, Indices}, asset::RenderAssetUsages, prelude::*};

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

/// Creates the four end points of a rail based on the given transform
fn create_rail_end_points(transform: &Transform) -> Vec<Vec3> {
    let rail_width: f32 = 0.25;
    let rail_height: f32 = 0.5;

    let lower_left: Vec3 = transform.transform_point(Vec3::new(-rail_width, -rail_height, 0.0));
    let lower_right: Vec3 = transform.transform_point(Vec3::new(rail_width, -rail_height, 0.0));
    let upper_left: Vec3 = transform.transform_point(Vec3::new(-rail_width, rail_height, 0.0));
    let upper_right: Vec3 = transform.transform_point(Vec3::new(rail_width, rail_height, 0.0));
    vec![lower_left, lower_right, upper_left, upper_right]
}

fn create_rail_segment_mesh(starting_transform: &Transform, ending_transform: &Transform) -> Mesh {
    let mut starting_points = create_rail_end_points(&starting_transform);
    let mut ending_points = create_rail_end_points(&ending_transform);
    starting_points.append(&mut ending_points);
    starting_points.append(&mut vec![
        starting_points[2].clone(),
        starting_points[3].clone(),
        starting_points[6].clone(),
        starting_points[7].clone(),
    ]);

    let segment = Mesh::new(PrimitiveTopology::TriangleList,
        RenderAssetUsages::default()
    )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, starting_points)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vec![
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::Y,
            Vec3::Y,
            Vec3::Y,
            Vec3::Y,
        ])
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(1.0, 1.0),
        ])
        .with_inserted_indices(Indices::U32(vec![
            // -X
            4, 2, 0, 4, 6, 2,
            // +X
            1, 5, 3, 3, 5, 7,
            // +Y
            9, 8, 10, 9, 10, 11,
        ]));
        segment
}

fn create_rail_mesh(rail: &Rail) -> Vec<Vec3> {
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
    let interpoints: Vec<Vec3> = cubic_bezier.iter_positions(25).collect();
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
        starting_point: Transform::from_xyz(-3., 0., -4.5),
        ending_point: Transform::from_xyz(2.5, 0., 3.5),
        curve_mode: CurveMode::Cubic,
    };
    let interpoints = create_rail_mesh(&rail);
    for point in interpoints {
        commands.spawn((
        Mesh3d(meshes.add(Cuboid{half_size: Vec3::new(0.1, 0.1, 0.1), ..default()}.mesh())),
        MeshMaterial3d(materials.add(Color::srgb(0.5, 0.3, 0.3))),
        Transform::from_translation(point)
    ));
    }

     let rail_segment_handle: Handle<Mesh> = meshes.add(create_rail_segment_mesh(
        &rail.starting_point, &rail.ending_point
    ));
    commands.spawn((
        Mesh3d(rail_segment_handle),
        MeshMaterial3d(materials.add(Color::srgb(0.5, 0.3, 0.3))),
        Transform::from_xyz(1., 0.5, 0.),
    ));
}
