use std::f32::consts::PI;

use bevy::{
    color::Color,
    ecs::{
        entity::Entity,
        system::{Commands, Query, Res},
    },
    image::ImageSampler,
    math::Quat,
    pbr::{DistanceFog, FogFalloff},
    prelude::*,
    render::{
        camera::Camera,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
    },
    time::Time,
    transform::components::Transform,
    utils::default,
};

use bevy::render::view::RenderLayers;

use bevy_flycam::FlyCam;

use crate::{characters::location::Location, room::mesh::F32_ROOM_SIZE, utils::utils::convert_ivec2_to_vec3_plane};

const CAMERA_HEIGHT: f32 = 1.5;

const RENDER_TEXTURE_WIDTH: u32 = 320;
const RENDER_TEXTURE_HEIGHT: u32 = 200;

pub fn setup_camera(
    mut commands: Commands,
    q_fly_cam: Query<&FlyCam>,
    mut images: ResMut<Assets<Image>>,
) {
    if !q_fly_cam.is_empty() {
        return;
    }

    let size = Extent3d {
        width: RENDER_TEXTURE_WIDTH,
        height: RENDER_TEXTURE_HEIGHT,
        ..default()
    };

    let mut render_texture: Image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        sampler: ImageSampler::nearest(), // For pixelated look
        ..default()
    };

    render_texture.resize(size);

    let render_texture_handle = images.add(render_texture);

    commands.spawn((
        Camera3d::default(),
        Camera {
            order: -1,
            target: render_texture_handle.clone().into(),
            clear_color: Color::WHITE.into(),
            ..default()
        },
        Location::new(default(), IVec2::Y),
    ));

    commands.spawn((
        Sprite {
            image: render_texture_handle,
            custom_size: Some(Vec2::new(
                RENDER_TEXTURE_WIDTH as f32,
                RENDER_TEXTURE_HEIGHT as f32
            )),
            ..default()
        },
        Transform::from_scale(Vec3::splat(4.0)),
        RenderLayers::layer(1),
    ));

    commands.spawn((Camera2d::default(), RenderLayers::layer(1)));
}

pub fn spawn_fog(mut commands: Commands, q_camera: Query<(Entity, &Camera), With<Location>>) {
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
    q_player: Query<&Location>,
    mut q_camera: Query<(&mut Transform, &Camera), With<Location>>,
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
    mut q_players: Query<(&Location, &mut Transform)>,
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
            .lerp(final_translation, time.delta_secs() * LERP_SPEED);
        transform.rotation = transform.rotation.lerp(
            Quat::from_rotation_y(angle - PI / 2.0),
            time.delta_secs() * LERP_SPEED,
        );
    }
}
