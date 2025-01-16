use std::{env, str::FromStr};

use bevy::math::*;

pub fn convert_ivec2_to_vec3_plane(v: IVec2) -> Vec3 {
    IVec3 {
        x: v.x,
        y: 0,
        z: -v.y,
    }
    .as_vec3()
}

pub fn cli_parameter_arg<T>(args: &Vec<String>, options: Vec<String>) -> Option<T>
where
    T: FromStr,
{
    let Some(index) = args.iter().position(|r| options.contains(r)) else {
        return None;
    };

    if index + 1 >= args.iter().len() {
        return None;
    }
    let cmd_str = &args[index + 1];

    match cmd_str.parse::<T>() {
        Ok(v) => Some(v),
        Err(_) => None,
    }
}
