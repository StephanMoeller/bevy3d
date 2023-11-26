//! This example demonstrates the built-in 3d shapes in Bevy.
//! The scene includes a patterned texture and a rotation for visualizing the normals and UVs.

use std::f32::consts::PI;
use bevy::*;
use bevy::{
    prelude::*,
    render::render_resource::*,
};
use bevy::render::mesh::Indices;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate)
        .run();
}

/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Shape;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    let radiuses:Vec<f32> = vec![0.15,0.13,0.1, 0.065, 0.05, 0.03, 0.0];
    for (idx, radius) in radiuses.iter().enumerate() {
        let shape = MyBox::new(1.0, 2.0, 1.0, *radius);
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(shape.into()),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(0.8, 1.0, 0.8),
                    metallic:0.5,
                    ..default()
                }),
                transform: Transform::from_xyz((idx as f32 - (radiuses.len() as f32 / 2.)) * 2.0, 2.0, 0.0),
                ..default()
            },
            Shape,
        ));

    }


    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(50.0).into()),
        material: materials.add(Color::SILVER.into()),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 3., 15.0).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
        ..default()
    });
}

fn rotate(
    mut query: Query<&mut Transform, With<Shape>>,
    time: Res<Time>,
    keycode: Res<Input<KeyCode>>) {
    let speed = 0.02;
    for mut transform in &mut query {
        if keycode.pressed(KeyCode::Right) {
            transform.rotate(Quat::from_rotation_y(speed));
        }
        if keycode.pressed(KeyCode::Left) {
            transform.rotate(Quat::from_rotation_y(-speed));
        }
        if keycode.pressed(KeyCode::Up) {
            transform.rotate(Quat::from_rotation_x(-speed));
        }
        if keycode.pressed(KeyCode::Down) {
            transform.rotate(Quat::from_rotation_x(speed));
        }
    }
}




/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
    )
}

#[derive(Debug, Copy, Clone)]
pub struct MyBox {
    pub min_x: f32,
    pub max_x: f32,

    pub min_y: f32,
    pub max_y: f32,

    pub min_z: f32,
    pub max_z: f32,
    pub edge_radius: f32,
}

impl Default for MyBox {
    fn default() -> Self {
        MyBox::new(3.0, 3.0, 3.0, 0.5)
    }
}

impl MyBox {
    /// Creates a new box centered at the origin with the supplied side lengths.
    pub fn new(x_length: f32, y_length: f32, z_length: f32, edge_radius: f32) -> MyBox {
        MyBox {
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

impl From<MyBox> for Mesh {
    fn from(sp: MyBox) -> Self {
        let _r = sp.edge_radius;
        // suppose Y-up right hand, and camera look from +z to -z

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
            0, 1, 2, 2, 3, 0, // front
            4, 5, 6, 6, 7, 4, // back
            8, 9, 10, 10, 11, 8, // right
            12, 13, 14, 14, 15, 12, // left
            16, 17, 18, 18, 19, 16, // top
            20, 21, 22, 22, 23, 20, // bottom

            // borders
            0, 3, 13, 13, 12, 0,    // front/left
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

            // corners top
            18, 13, 3, // top/left/front
            17, 4, 14, // top/left/back
            19, 2, 10, // top/right/front
            16, 9, 5,  // top/right/back

            21, 0, 12, // bottom/left/front
            22, 15, 7, // bottom/left/back
            20, 11, 1, // bottom/right/front
            23, 6, 8,   // bottom/right/back
        ]);

        Mesh::new(PrimitiveTopology::TriangleList)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
            .with_indices(Some(indices))
    }
}