use bevy::{asset::RenderAssetUsages, prelude::*, render::mesh::Indices};

use crate::{gen::location::WorldCatacomb, utils::utils::convert_ivec2_to_vec3_plane};

fn generate_floor_mesh(assets: &mut ResMut<Assets<Mesh>>) -> Handle<Mesh> {
    use bevy::render::mesh::*;

    assets.add(Cuboid::new(F32_ROOM_SIZE, 0.0, F32_ROOM_SIZE).mesh())
}

fn generate_wall_mesh(assets: &mut ResMut<Assets<Mesh>>) -> Handle<Mesh> {
    let size = F32_ROOM_SIZE * 0.5;

    let mesh = Mesh::new(
        bevy::render::mesh::PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            // top (facing towards +y)
            [-size, size, -size], // vertex with index 0
            [size, size, -size],  // vertex with index 1
            [size, size, size],   // etc. until 23
            [-size, size, size],
            // bottom   (-y)
            [-size, -size, -size],
            [size, -size, -size],
            [size, -size, size],
            [-size, -size, size],
            // right    (+x)
            [size, -size, -size],
            [size, -size, size],
            [size, size, size], // This vertex is at the same position as vertex with index 2, but they'll have different UV and normal
            [size, size, -size],
            // left     (-x)
            [-size, -size, -size],
            [-size, -size, size],
            [-size, size, size],
            [-size, size, -size],
            // back     (+z)
            [-size, -size, size],
            [-size, size, size],
            [size, size, size],
            [size, -size, size],
            // forward  (-z)
            [-size, -size, -size],
            [-size, size, -size],
            [size, size, -size],
            [size, -size, -size],
        ],
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            // Assigning the UV coords for the top side.
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0],
            // Assigning the UV coords for the bottom side.
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0],
            ////////
            // Assigning the UV coords for the right side.
            [1.0, 1.0],
            [0.0, 1.0],
            [0.0, 0.0],
            [1.0, 0.0], // Done
            // Assigning the UV coords for the left side.
            [1.0, 1.0],
            [0.0, 1.0],
            [0.0, 0.0],
            [1.0, 0.0], // Done
            // Assigning the UV coords for the back side.
            [0.0, 1.0],
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0], // Done
            // Assigning the UV coords for the forward side.
            [0.0, 1.0],
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
        ],
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![
            // Normals for the top side (towards +y)
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            // Normals for the bottom side (towards -y)
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            // Normals for the right side (towards +x)
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            // Normals for the left side (towards -x)
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            // Normals for the back side (towards +z)
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            // Normals for the forward side (towards -z)
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
        ],
    )
    .with_inserted_indices(Indices::U32(vec![
        0, 3, 1, 1, 3, 2, // triangles making up the top (+y) facing side.
        4, 5, 7, 5, 6, 7, // bottom (-y)
        8, 11, 9, 9, 11, 10, // right (+x)
        12, 13, 15, 13, 14, 15, // left (-x)
        16, 19, 17, 17, 19, 18, // back (+z)
        20, 21, 23, 21, 22, 23, // forward (-z)
    ]));

    assets.add(mesh)
}

fn new_material(texture: Handle<Image>, emission: Option<Handle<Image>>) -> StandardMaterial {
    StandardMaterial {
        base_color_texture: Some(texture),
        emissive: if emission.is_some() {
            LinearRgba::WHITE
        } else {
            LinearRgba::BLACK
        },
        emissive_texture: emission,
        reflectance: 0.0,
        ..default()
    }
}

const ROOM_SIZE: i32 = 2;
pub const F32_ROOM_SIZE: f32 = ROOM_SIZE as f32;

pub fn setup_rooms(
    location: Res<WorldCatacomb>,
    mut commands: Commands,
    mut assets: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    textures: Res<AssetServer>,
) {
    println!("Setting up rooms");
    for loc in location.0.iter() {
        // Floor
        let mesh = generate_floor_mesh(&mut assets);
        commands.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(materials.add(new_material(textures.load("textures/floor.png"), None))),
            Transform {
                translation: convert_ivec2_to_vec3_plane(*loc) * F32_ROOM_SIZE,
                ..default()
            },
        ));
        // Ceiling
        let mesh = generate_floor_mesh(&mut assets);
        let mut translation = convert_ivec2_to_vec3_plane(*loc) * F32_ROOM_SIZE;
        translation.y = F32_ROOM_SIZE;
        commands.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(materials.add(new_material(textures.load("textures/floor.png"), None))),
            Transform {
                translation,
                ..default()
            },
        ));
    }
}

pub fn setup_walls(
    location: Res<WorldCatacomb>,
    mut commands: Commands,
    mut assets: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    textures: Res<AssetServer>,
) {
    println!("Setting up walls");

    for loc in location.0.iter() {
        for i in -1..2 {
            for j in -1..2 {
                let loc = *loc + IVec2::from_array([i, j]);
                if !location.0.contains(&loc) {
                    let mesh = generate_wall_mesh(&mut assets);
                    let mut translation = convert_ivec2_to_vec3_plane(loc) * F32_ROOM_SIZE;
                    translation.y = F32_ROOM_SIZE / 2.0;
                    commands.spawn((
                        Mesh3d(mesh),
                        MeshMaterial3d(materials.add(new_material(
                            textures.load("textures/wall.png"),
                            Some(textures.load("textures/wall_emission.png")),
                        ))),
                        Transform {
                            translation,
                            ..default()
                        },
                    ));
                }
            }
        }
    }
}
