use bevy::prelude::*;

use crate::{
    utils::utils::convert_ivec2_to_vec3_plane, WorldCatacomb, CAMERA_HEIGHT, F32_ROOM_SIZE,
};

#[derive(Component, Reflect, Debug, Default, Clone)]
pub struct WorldLocation {
    location: IVec2,
    forward: IVec2,
    pub can_move: bool,
}

pub enum Turn {
    Right,
    Left,
}

impl WorldLocation {
    pub const fn new(start: IVec2, face: IVec2) -> Self {
        WorldLocation {
            location: start,
            forward: face,
            can_move: true,
        }
    }

    pub fn get_forward(&self) -> IVec2 {
        self.forward
    }

    pub fn get_location(&self) -> IVec2 {
        self.location
    }

    pub fn face_towards(&mut self, position: IVec2) {
        self.forward = position - self.location;
    }

    pub fn turn(&mut self, dir: Turn) {
        if !self.can_move {
            return;
        }

        let forward = self.forward;
        match dir {
            Turn::Right => {
                let rot = IVec2 {
                    x: forward.y,
                    y: -forward.x,
                };
                self.forward = rot;
            }
            Turn::Left => {
                let rot = IVec2 {
                    x: -forward.y,
                    y: forward.x,
                };
                self.forward = rot;
            }
        }
    }
    pub fn move_forward(&mut self, world: &Res<WorldCatacomb>) {
        if !self.can_move {
            return;
        }

        let forward_location = self.location + self.forward;

        if !world.contains(&forward_location) {
            return;
        }

        self.location += self.forward;
    }
}

pub fn update_character_sprite_positions(
    mut q_characters: Query<(&WorldLocation, &mut Transform)>,
    time: Res<Time>,
) {
    const LERP_SPEED: f32 = 10.0;

    for (loc, mut transform) in q_characters.iter_mut() {
        let location = loc.get_location();

        let mut final_translation = convert_ivec2_to_vec3_plane(location) * F32_ROOM_SIZE;
        final_translation.y = CAMERA_HEIGHT;

        transform.translation = transform
            .translation
            .lerp(final_translation, time.delta_secs() * LERP_SPEED);
    }
}
