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

use crate::{
    characters::location::WorldLocation, room::mesh::F32_ROOM_SIZE,
    utils::utils::convert_ivec2_to_vec3_plane,
};

use super::player::Player;

pub(crate) const CAMERA_HEIGHT: f32 = 1.5;

pub const RENDER_TEXTURE_WIDTH: u32 = 320;
pub const RENDER_TEXTURE_HEIGHT: u32 = 200;

#[derive(Component)]
pub struct MainCamera;

pub fn setup_camera(
    mut commands: Commands,
    q_fly_cam: Query<&FlyCam>,
    mut images: ResMut<Assets<Image>>,
    q_player: Query<(Entity, &Player)>,
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

    let mut weapon_render_texture: Image = Image {
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
    let weapon_render_texture_handle = images.add(weapon_render_texture);

    let (player, _) = q_player.single();

    commands.entity(player).insert((
        Camera3d::default(),
        Camera {
            order: -1,
            target: render_texture_handle.clone().into(),
            clear_color: Color::WHITE.into(),
            ..default()
        },
        Projection::Perspective(PerspectiveProjection {
            fov: 90.0_f32.to_radians(),
            ..default()
        }),
        MainCamera,
    ));

    commands.entity(player).insert((
        Camera3d::default(),
        Camera {
            order: -1,
            target: weapon_render_texture_handle.clone().into(),
            clear_color: Color::WHITE.into(),
            ..default()
        },
        Projection::Perspective(PerspectiveProjection {
            fov: 90.0_f32.to_radians(),
            ..default()
        }),
    ));

    commands.spawn((
        Sprite {
            image: render_texture_handle,
            custom_size: Some(Vec2::new(
                RENDER_TEXTURE_WIDTH as f32,
                RENDER_TEXTURE_HEIGHT as f32,
            )),
            ..default()
        },
        Transform::from_scale(Vec3::splat(4.0)).with_translation((0.0, 0.0, -3.0).into()),
        RenderLayers::layer(1),
    ));

    // Weapon arms
    commands.spawn((
        Sprite {
            image: weapon_render_texture_handle,
            custom_size: Some(Vec2::new(
                RENDER_TEXTURE_WIDTH as f32,
                RENDER_TEXTURE_HEIGHT as f32,
            )),
            ..default()
        },
        Transform::from_scale(Vec3::splat(4.0)).with_translation((0.0, 0.0, -2.0).into()),
        RenderLayers::layer(1),
    ));

    commands.spawn((Camera2d::default(), RenderLayers::layer(1)));
}

pub fn spawn_fog(mut commands: Commands, q_camera: Query<Entity, With<MainCamera>>) {
    let entity = q_camera.single();
    commands.entity(entity).insert(DistanceFog {
        color: Color::BLACK,
        falloff: FogFalloff::Linear {
            start: 2.0,
            end: 5.0,
        },
        ..default()
    });
}

pub fn sync_camera(
    q_player: Query<&WorldLocation, With<Player>>,
    mut q_camera: Query<&mut Transform, With<MainCamera>>,
    time: Res<Time>,
) {
    const LERP_SPEED: f32 = 10.0;

    let player_location = q_player.single();
    let forward = player_location.get_forward().as_vec2();
    let location = player_location.get_location();
    let angle = forward.to_angle();
    let (mut transform) = q_camera.single_mut();

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
