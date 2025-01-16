use bevy::prelude::*;

use crate::WorldCatacomb;

#[derive(Component, Reflect, Debug)]
pub struct Location {
    location: IVec2,
    forward: IVec2,
}

pub enum Turn {
    Right,
    Left,
}

impl Location {
    pub const fn new(start: IVec2, face: IVec2) -> Self {
        Location {
            location: start,
            forward: face,
        }
    }

    pub fn get_forward(&self) -> IVec2 {
        self.forward
    }

    pub fn get_location(&self) -> IVec2 {
        self.location
    }

    pub fn turn(&mut self, dir: Turn) {
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
        let forward_location = self.location + self.forward;

        if !world.0.contains(&forward_location) {
            return;
        }

        self.location += self.forward;
    }
}