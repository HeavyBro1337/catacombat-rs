use bevy::prelude::*;

use crate::characters::player::player::Player;

#[derive(Component)]
#[require(Transform)]
pub struct Billboard;

pub fn update_billboards(
    q_cameras: Query<(&Transform, &Camera3d), Without<Billboard>>,
    mut q_billboards: Query<&mut Transform, With<Billboard>>,
) {
    let (cam, _) = q_cameras.single();
    let forward = cam.forward().as_vec3();

    for mut billboard_transform in q_billboards.iter_mut() {
        let rot = Quat::from_rotation_arc(Vec3::Y, forward);
        billboard_transform.look_to(forward, Vec3::Y);
    }
}
