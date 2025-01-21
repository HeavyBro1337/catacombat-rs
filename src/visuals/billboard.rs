use bevy::prelude::*;

use crate::MainCamera;

#[derive(Component, Default)]
#[require(Transform)]
pub struct Billboard;

pub fn update_billboards(
    q_cameras: Query<&Transform, With<MainCamera>>,
    mut q_billboards: Query<&mut Transform, With<Billboard>>,
) {
    let cam_transform = q_cameras.single();
    let forward = cam_transform.forward().as_vec3();

    for mut billboard_transform in q_billboards.iter_mut() {
        billboard_transform.look_to(forward, Vec3::Y);
    }
}
