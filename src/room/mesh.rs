use bevy::prelude::*;

use crate::{gen::location::WorldCatacomb, utils::utils::convert_ivec2_to_vec3_plane};

fn generate_floor(assets: &mut ResMut<Assets<Mesh>>) -> Handle<Mesh> {
    use bevy::render::mesh::*;

    assets.add(Cuboid::new(F32_ROOM_SIZE, 0.0, F32_ROOM_SIZE).mesh())
}

const ROOM_SIZE: i32 = 4;
pub const F32_ROOM_SIZE: f32 = ROOM_SIZE as f32;

pub fn setup_rooms(
    location: Res<WorldCatacomb>,
    mut commands: Commands,
    mut assets: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("Setting up rooms");
    for loc in location.0.iter() {
        let mesh = generate_floor(&mut assets);
        // commands.spawn(Mesh3d {
        //     mesh,
        //     material: materials.add(StandardMaterial {
        //         base_color: Color::GOLD,
        //         ..default()
        //     }),
        //     transform: Transform {
        //         translation: convert_ivec2_to_vec3_plane(*loc) * F32_ROOM_SIZE,
        //         ..default()
        //     },
        //     ..default()
        // });
        commands.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::linear_rgba(0.58, 0.576, 0.259, 1.0),
                ..default()
            })),
            Transform {
                translation: convert_ivec2_to_vec3_plane(*loc) * F32_ROOM_SIZE,
                ..default()
            },
        ));
    }
}
