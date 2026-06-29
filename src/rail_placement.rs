use bevy::{asset::RenderAssetUsages, mesh::{Indices, PrimitiveTopology}, prelude::*, render::render_asset::RenderAsset};

pub struct RailPlacementPlugin;
impl Plugin for RailPlacementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

#[derive(Component)]
pub struct Rail {
    starting_point: Transform,
    ending_point: Transform,
}

/// Creates the four end points of a rail based on the given transform
fn create_rail_end_vertices(transform: &Transform) -> Vec<Vec3> {
    const RAIL_WIDTH: f32 = 0.1;
    const RAIL_HEIGHT: f32 = 0.15;

    let lower_left: Vec3 = transform.transform_point(Vec3::new(-RAIL_WIDTH, -RAIL_HEIGHT, 0.0));
    let lower_right: Vec3 = transform.transform_point(Vec3::new(RAIL_WIDTH, -RAIL_HEIGHT, 0.0));
    let upper_left: Vec3 = transform.transform_point(Vec3::new(-RAIL_WIDTH, RAIL_HEIGHT, 0.0));
    let upper_right: Vec3 = transform.transform_point(Vec3::new(RAIL_WIDTH, RAIL_HEIGHT, 0.0));
    vec![lower_left, lower_right, upper_left, upper_right]
}

/// Create a mesh for a rail segment between two transforms
fn create_rail_segment_mesh(starting_transform: &Transform, ending_transform: &Transform) -> Mesh {
    let mut starting_points = create_rail_end_vertices(&starting_transform);
    let mut ending_points = create_rail_end_vertices(&ending_transform);
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
            3, 5, 1, 7, 5, 3,
            // +Y
            9, 8, 10, 9, 10, 11,
        ]));
        segment
}

/// Create a rail made of rail segments between the starting and ending points of the given rail
/// Uses a cubic bezier curve
fn create_rail_mesh(rail: &Rail) -> Vec<Mesh> {
    const RAIL_STANDARD_GAUGE: f32 = 1.435;
    const SAMPLES: usize = 30;
    // This value is temporary as ultimately the player will be able to manipulate the control points
    // using drag handles; this is here as a placeholder
    let control_point_offset: Vec3 = Vec3::new(0., 0., 20.);

    let inner_starting_point: Vec3 = rail.starting_point.transform_point(control_point_offset);
    let inner_ending_point: Vec3 = rail.ending_point.transform_point(control_point_offset);

    let control_points: [[Vec3; 4]; 1]  = [[
        rail.starting_point.translation,
        inner_starting_point,
        inner_ending_point,
        rail.ending_point.translation,
    ]];
    let cubic_bezier = CubicBezier::new(control_points).to_curve().unwrap();
    let inter_positions: Vec<Vec3> = cubic_bezier.iter_positions(SAMPLES).collect();
    let inter_velocities: Vec<Vec3> = cubic_bezier.iter_velocities(SAMPLES).collect();
    let mut inter_transforms: Vec<Transform> = inter_positions
    .iter()
    .zip(inter_velocities.iter())
    .map(|(pos, vel)| {
        let rotation = Quat::from_rotation_arc(Vec3::Z, (*vel).normalize());
        Transform {
            translation: *pos,
            rotation,
            ..default()
        }
    }).collect();
    // Set the start and end to the originally specified Rail to ensure that this mesh aligns with those ends
    inter_transforms[0] = rail.starting_point.clone();

    // Every rail end needs to face the same direction to generate the mesh properly
    let last_point = inter_transforms.last_mut().unwrap();
    *last_point = rail.ending_point.clone();
    last_point.rotate_local_y(std::f32::consts::PI);

    let tangential_offset = Vec3::new(RAIL_STANDARD_GAUGE / 2.0, 0., 0.);
    let mut segment_meshes: Vec<Mesh> = Vec::new();
    for window in inter_transforms.windows(2) {

        let start_transform: Transform = window[0];
        let end_transform: Transform = window[1];

        let start_tangent = start_transform.rotation * tangential_offset;
        let end_tangent = end_transform.rotation * tangential_offset;

        // Right track
        segment_meshes.push(create_rail_segment_mesh(
            &Transform {
                translation: start_transform.translation + start_tangent,
                rotation: start_transform.rotation,
                ..default()
            }, 
            &Transform {
                translation: end_transform.translation + end_tangent,
                rotation: end_transform.rotation,
                ..default()
            }
        ));
        // Left track
        segment_meshes.push(create_rail_segment_mesh(
            &Transform {
                translation: start_transform.translation - start_tangent,
                rotation: start_transform.rotation,
                ..default()
            }, 
            &Transform {
                translation: end_transform.translation - end_tangent,
                rotation: end_transform.rotation,
                ..default()
            }
        ));
    }
    
    segment_meshes
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let rail = Rail {
        starting_point: Transform::from_xyz(10., 0., -7.).looking_to(Vec3::new(1.,0.,0.), Vec3::Y),
        ending_point: Transform::from_xyz(-10., 0., 15.).looking_to(Vec3::new(0.,0.,1.), Vec3::Y),
    };
    let rail_section = create_rail_mesh(&rail);
    for section in rail_section {
        let rail_segment_handle: Handle<Mesh> = meshes.add(section);
        commands.spawn((
            Mesh3d(rail_segment_handle),
            MeshMaterial3d(materials.add(Color::srgb(0.3, 0.3, 0.3))),
            Transform::from_xyz(0., 0., 0.)
        ));
    }

    
}
