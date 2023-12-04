//! This example demonstrates the built-in 3d shapes in Bevy.
//! The scene includes a patterned texture and a rotation for visualizing the normals and UVs.
use rand::prelude::*;
use bevy::*;
use bevy::{
    prelude::*,
    render::render_resource::*,
};
use bevy::prelude::shape::*;
use bevy::render::mesh::Indices;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate)
        .run();
}

#[derive(Eq, Hash, PartialEq)]
struct PointI32 {
    x: i32,
    y: i32,
    z: i32,
}

impl PointI32 {
    /// Creates a new box centered at the origin with the supplied side lengths.
    pub fn new(x: i32, y: i32, z: i32) -> PointI32 {
        PointI32 { x, y, z }
    }
}

/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Shape;

fn setup(mut commands: Commands,
         mut meshes: ResMut<Assets<Mesh>>,
         mut materials: ResMut<Assets<StandardMaterial>>) {

    let points = get_random_points(400);

    let box_unit = 0.5;
    for point in points.iter()
    {
        let shape = SoftBox::new(box_unit, box_unit, box_unit, 0.10);
        let color = get_color(&point);
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(shape.into()),
                material: materials.add(StandardMaterial { base_color: color, metallic: 0.1, ..default() }),
                transform: Transform::from_xyz(point.x as f32 * box_unit, point.y as f32 * box_unit, point.z as f32 * box_unit), ..default()
            },
            Shape,
        ));
    }

    commands.spawn(PointLightBundle {
        point_light: PointLight { intensity: 9000.0, range: 100., shadows_enabled: true, ..default() },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    // ground plane
    /*    commands.spawn(PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(50.0).into()),
            material: materials.add(Color::SILVER.into()),
            ..default()
        });
    */
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 3., 20.0).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
        ..default()
    });
}

fn rotate(
    mut query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
    keycode: Res<Input<KeyCode>>) {
    let speed = time.delta().as_millis() as f32 / 1000.0;
    for mut transform in &mut query {
        transform.look_at(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 0.));
        if keycode.pressed(KeyCode::Right) {
            transform.rotate_around(Vec3::new(0., 0., 0.), Quat::from_rotation_y(speed));
        }
        if keycode.pressed(KeyCode::Left) {
            transform.rotate_around(Vec3::new(0., 0., 0.), Quat::from_rotation_y(-speed));
        }
        if keycode.pressed(KeyCode::Up) {
            transform.rotate_around(Vec3::new(0., 0., 0.), Quat::from_rotation_x(speed));
        }
        if keycode.pressed(KeyCode::Down) {
            transform.rotate_around(Vec3::new(0., 0., 0.), Quat::from_rotation_x(-speed));
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SoftBox {
    pub min_x: f32,
    pub max_x: f32,

    pub min_y: f32,
    pub max_y: f32,

    pub min_z: f32,
    pub max_z: f32,
    pub edge_radius: f32,
}

impl Default for SoftBox {
    fn default() -> Self {
        SoftBox::new(3.0, 3.0, 3.0, 0.5)
    }
}

impl SoftBox {
    /// Creates a new box centered at the origin with the supplied side lengths.
    pub fn new(x_length: f32, y_length: f32, z_length: f32, edge_radius: f32) -> SoftBox {
        SoftBox {
            max_x: x_length / 2.0,
            min_x: -x_length / 2.0,
            max_y: y_length / 2.0,
            min_y: -y_length / 2.0,
            max_z: z_length / 2.0,
            min_z: -z_length / 2.0,
            edge_radius: edge_radius,
        }
    }
}

impl From<SoftBox> for Mesh {
    fn from(sp: SoftBox) -> Self {
        let _r = sp.edge_radius;
        let vertices = &[
            // Front
            ([sp.min_x + _r, sp.min_y + _r, sp.max_z], [0., 0., 1.0], [0., 0.]),    // 0 - bottom, left
            ([sp.max_x - _r, sp.min_y + _r, sp.max_z], [0., 0., 1.0], [1.0, 0.]),   // 1 - bottom, right
            ([sp.max_x - _r, sp.max_y - _r, sp.max_z], [0., 0., 1.0], [1.0, 1.0]),  // 2 - top, right
            ([sp.min_x + _r, sp.max_y - _r, sp.max_z], [0., 0., 1.0], [0., 1.0]),   // 3 - top, left
            // Back
            ([sp.min_x + _r, sp.max_y - _r, sp.min_z], [0., 0., -1.0], [1.0, 0.]),  // 4 - top, left
            ([sp.max_x - _r, sp.max_y - _r, sp.min_z], [0., 0., -1.0], [0., 0.]),   // 5 - top, right
            ([sp.max_x - _r, sp.min_y + _r, sp.min_z], [0., 0., -1.0], [0., 1.0]),  // 6 - bottom, right
            ([sp.min_x + _r, sp.min_y + _r, sp.min_z], [0., 0., -1.0], [1.0, 1.0]), // 7 - bottom, left
            // Right
            ([sp.max_x, sp.min_y + _r, sp.min_z + _r], [1.0, 0., 0.], [0., 0.]),    // 8 - bottom, far
            ([sp.max_x, sp.max_y - _r, sp.min_z + _r], [1.0, 0., 0.], [1.0, 0.]),   // 9 - top, far
            ([sp.max_x, sp.max_y - _r, sp.max_z - _r], [1.0, 0., 0.], [1.0, 1.0]),  // 10 - top, near
            ([sp.max_x, sp.min_y + _r, sp.max_z - _r], [1.0, 0., 0.], [0., 1.0]),   // 11 - bottom, near
            // Left
            ([sp.min_x, sp.min_y + _r, sp.max_z - _r], [-1.0, 0., 0.], [1.0, 0.]),  // 12 - bottom, near
            ([sp.min_x, sp.max_y - _r, sp.max_z - _r], [-1.0, 0., 0.], [0., 0.]),   // 13 - top, near
            ([sp.min_x, sp.max_y - _r, sp.min_z + _r], [-1.0, 0., 0.], [0., 1.0]),  // 14 - top, far
            ([sp.min_x, sp.min_y + _r, sp.min_z + _r], [-1.0, 0., 0.], [1.0, 1.0]), // 15 - bottom, far
            // Top
            ([sp.max_x - _r, sp.max_y, sp.min_z + _r], [0., 1.0, 0.], [1.0, 0.]),   // 16 - right, far
            ([sp.min_x + _r, sp.max_y, sp.min_z + _r], [0., 1.0, 0.], [0., 0.]),    // 17 - left, far
            ([sp.min_x + _r, sp.max_y, sp.max_z - _r], [0., 1.0, 0.], [0., 1.0]),   // 18 - left, near
            ([sp.max_x - _r, sp.max_y, sp.max_z - _r], [0., 1.0, 0.], [1.0, 1.0]),  // 19 - right, near
            // Bottom
            ([sp.max_x - _r, sp.min_y, sp.max_z - _r], [0., -1.0, 0.], [0., 0.]),   // 20 - right, near
            ([sp.min_x + _r, sp.min_y, sp.max_z - _r], [0., -1.0, 0.], [1.0, 0.]),  // 21 - left, near
            ([sp.min_x + _r, sp.min_y, sp.min_z + _r], [0., -1.0, 0.], [1.0, 1.0]), // 22 - left, far
            ([sp.max_x - _r, sp.min_y, sp.min_z + _r], [0., -1.0, 0.], [0., 1.0]),  // 23 - right, far
        ];

        let positions: Vec<_> = vertices.iter().map(|(p, _, _)| *p).collect();
        let normals: Vec<_> = vertices.iter().map(|(_, n, _)| *n).collect();
        let uvs: Vec<_> = vertices.iter().map(|(_, _, uv)| *uv).collect();

        let indices = Indices::U32(vec![
            // faces
            0, 1, 2, 2, 3, 0, // front
            4, 5, 6, 6, 7, 4, // back
            8, 9, 10, 10, 11, 8, // right
            12, 13, 14, 14, 15, 12, // left
            16, 17, 18, 18, 19, 16, // top
            20, 21, 22, 22, 23, 20, // bottom

            // edges
            0,   3, 13, 13, 12, 0,    // front/left
            18, 17, 13, 14, 13, 17, // top/left
            4, 7, 14, 14, 7, 15,    // back/left
            22, 21, 15, 12, 15, 21, // bottom/left

            2, 1, 10, 1, 11, 10,    // front/right
            16, 19, 10, 10, 9, 16,  // top/right
            6, 5, 9, 9, 8, 6,       // back/right
            20, 23, 8, 8, 11, 20,   // bottom/right

            19, 18, 2, 3, 2, 18,    // top/front
            1, 0, 20, 21, 20, 0,    // front/bottom
            23, 22, 7, 7, 6, 23,    // bottom/back
            5, 4, 17, 17, 16, 5,    // back/top

            // corners
            18, 13, 3, // top/left/front
            17, 4, 14, // top/left/back
            19, 2, 10, // top/right/front
            16, 9, 5,  // top/right/back

            21, 0, 12, // bottom/left/front
            22, 15, 7, // bottom/left/back
            20, 11, 1, // bottom/right/front
            23, 6, 8,  // bottom/right/back
        ]);

        Mesh::new(PrimitiveTopology::TriangleList)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
            .with_indices(Some(indices))
    }
}

fn get_random_points(point_count: usize) -> Vec<PointI32> {
    let mut points: Vec<PointI32> = vec![];
    points.push(PointI32::new(0, 5, 0));
    while points.len() < point_count
    {
        let direction = get_random_direction();
        let prev = points.last().unwrap();
        let next = PointI32::new(prev.x + direction.x, prev.y + direction.y, prev.z + direction.z);
        points.push(next);
    }
    return points;
}

fn get_random_direction() -> PointI32 {
    let mut rng = rand::thread_rng();
    let position: usize = rng.gen_range(0..6);
    match position {
        0 => PointI32::new(1, 0, 0),
        1 => PointI32::new(-1, 0, 0),
        2 => PointI32::new(0, 1, 0),
        3 => PointI32::new(0, -1, 0),
        4 => PointI32::new(0, 0, 1),
        5 => PointI32::new(0, 0, -1),
        _ => panic!("Should not be hit!")
    }
}



fn get_color(point: &PointI32) -> Color {
    return Color::rgb(0.5 + 0.2 * point.x as f32, 0.5 + 0.1 * point.y as f32, 1. - (0.2 * point.z as f32));
}