use bevy::{prelude::*, render::render_asset::RenderAssetUsages};

use crate::{gen::{location::Location, walker::Walker}, state::GameState};

fn generate_floor(assets: &mut ResMut<Assets<Mesh>>) -> Handle<Mesh> {
    use bevy::render::mesh::*;

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList, 
        RenderAssetUsages::default()
    );
    let mut indices: Vec<u32> = default();

    let mut positions: Vec<[f32; 3]> = default();
    let mut uvs: Vec<[f32; 2]> = default();
    let mut normals: Vec<[f32; 3]> = default();

    indices.push(0);
    indices.push(1);
    indices.push(2);
    indices.push(0);
    indices.push(2);
    indices.push(3);

    uvs.push([0.0, 0.0]);
    uvs.push([0.0, 1.0]);
    uvs.push([1.0, 1.0]);
    uvs.push([1.0, 0.0]);

    positions.push([0.0, 0.0, 0.0]);
    positions.push([0.0, 0.0, FROOM_SIZE]);
    positions.push([FROOM_SIZE, 0.0, FROOM_SIZE]);
    positions.push([FROOM_SIZE, 0.0, 0.0]);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_indices(Indices::U32(indices));

    assets.add(mesh)
}

const ROOM_SIZE: i32 = 64;
const FROOM_SIZE: f32 = ROOM_SIZE as f32;

pub fn setup_rooms(location: Res<Location>, mut command: Commands, mut assets: ResMut<Assets<Mesh>>) {
    for loc in location.0.iter() {
        let mesh = generate_floor(&mut assets);
        command.spawn(PbrBundle {
            mesh,
            transform: Transform {
                translation: convert_ivec2_to_vec3(*loc) * FROOM_SIZE,
                ..default()
            },
            ..default()
        });
    }
}

fn convert_ivec2_to_vec3(v: IVec2) -> Vec3 {
    IVec3 { x: v.x, y: 0, z: v.y}.as_vec3()
}

pub fn check_walkers(q_walkers: Query<Entity, With<Walker>>, mut state: ResMut<NextState<GameState>>) {
    if q_walkers.iter().len() > 0 {
        return;
    }

    state.set(GameState::Game);
}