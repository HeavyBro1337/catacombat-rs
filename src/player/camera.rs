use std::f32::consts::PI;

use bevy::{
    core_pipeline::core_3d::Camera3dBundle, ecs::{
        entity::Entity,
        query::{With, Without},
        system::{Commands, Query, Res},
    }, math::{Quat, Vec3}, pbr::{DistanceFog, FogFalloff}, prelude::Camera3d, render::{camera::Camera, color::Color}, time::Time, transform::components::Transform, utils::default
};
use bevy_flycam::FlyCam;

use crate::{room::mesh::F32_ROOM_SIZE, utils::utils::convert_ivec2_to_vec3_plane, OtherPlayer};

use super::player::PlayerLocation;

const CAMERA_HEIGHT: f32 = 1.5;

pub fn setup_camera(mut commands: Commands, q_fly_cam: Query<&FlyCam>) {
    if !q_fly_cam.is_empty() {
        return;
    }

    commands.spawn((
        Camera3d::default(),
        PlayerLocation::new(),
    ));
}

pub fn spawn_fog(mut commands: Commands, q_camera: Query<(Entity, &Camera)>) {
    let (entity, _) = q_camera.single();
    commands.entity(entity).insert(DistanceFog {
        color: Color::BLACK,
        falloff: FogFalloff::Linear {
            start: 5.0,
            end: 10.0,
        },
        ..default()
    });
}

pub fn sync_camera(
    q_player: Query<&PlayerLocation, Without<OtherPlayer>>,
    mut q_camera: Query<(&mut Transform, &Camera)>,
    time: Res<Time>,
) {
    const LERP_SPEED: f32 = 10.0;

    let player_location = q_player.single();
    let forward = player_location.get_forward().as_vec2();
    let location = player_location.get_location();
    let angle = forward.to_angle();
    let (mut transform, _) = q_camera.single_mut();

    let mut final_translation = convert_ivec2_to_vec3_plane(location) * F32_ROOM_SIZE;
    final_translation.y = CAMERA_HEIGHT;

    transform.translation = transform
        .translation
        .lerp(final_translation, time.delta_secs() * LERP_SPEED);
    transform.rotation = transform.rotation.lerp(
        Quat::from_rotation_y(angle - PI / 2.0),
        time.delta_secs() * LERP_SPEED,
    )
}

pub fn set_player_sprite_positions(
    mut q_players: Query<(&PlayerLocation, &mut Transform), With<OtherPlayer>>,
    time: Res<Time>,
) {
    const LERP_SPEED: f32 = 10.0;

    for (loc, mut transform) in q_players.iter_mut() {
        let forward = loc.get_forward().as_vec2();
        let location = loc.get_location();
        let angle = forward.to_angle();

        let mut final_translation = convert_ivec2_to_vec3_plane(location) * F32_ROOM_SIZE;
        final_translation.y = CAMERA_HEIGHT;

        transform.translation = transform
            .translation
            .lerp(final_translation, time.delta_seconds() * LERP_SPEED);
        transform.rotation = transform.rotation.lerp(
            Quat::from_rotation_y(angle - PI / 2.0),
            time.delta_seconds() * LERP_SPEED,
        );
    }
}
