use bevy::math::*;

pub fn convert_ivec2_to_vec3_plane(v: IVec2) -> Vec3 {
    IVec3 { x: v.x, y: 0, z: -v.y}.as_vec3()
}