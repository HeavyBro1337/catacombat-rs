use std::f32::consts::PI;

use bevy::{
    math::Affine2,
    prelude::*,
    render::render_resource::Texture,
    text::cosmic_text::{rustybuzz::script::JAVANESE, ttf_parser::loca},
};

use crate::{gen::location::WorldCatacomb, utils::utils::convert_ivec2_to_vec3_plane};

fn generate_floor_mesh(assets: &mut ResMut<Assets<Mesh>>) -> Handle<Mesh> {
    use bevy::render::mesh::*;

    assets.add(Cuboid::new(F32_ROOM_SIZE, 0.0, F32_ROOM_SIZE).mesh())
}

fn generate_wall_mesh(assets: &mut ResMut<Assets<Mesh>>) -> Handle<Mesh> {
    use bevy::render::mesh::*;

    assets.add(Cuboid::new(F32_ROOM_SIZE, F32_ROOM_SIZE, F32_ROOM_SIZE).mesh())
}

fn new_material(texture: Handle<Image>, emission: Option<Handle<Image>>) -> StandardMaterial {
    StandardMaterial {
        base_color_texture: Some(texture),
        emissive: if emission.is_some() { LinearRgba::WHITE } else { LinearRgba::BLACK },
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
