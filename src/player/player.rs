use bevy::math::IVec2;
use bevy::prelude::*;
use bevy_sprite3d::{Sprite3d, Sprite3dBuilder, Sprite3dBundle, Sprite3dParams};
use serde::{Deserialize, Serialize};

use crate::gen::location::WorldCatacomb;

#[derive(Component, Reflect, Debug)]
pub struct PlayerLocation {
    location: IVec2,
    forward: IVec2,
}

pub enum Turn {
    Right,
    Left,
}

impl PlayerLocation {
    pub fn new() -> Self {
        PlayerLocation {
            location: default(),
            forward: IVec2::Y,
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

#[derive(Component)]
pub struct OtherPlayer(pub u64);

#[derive(Component)]
pub struct OwnerId(pub u64);

#[derive(Component)]
#[require(PlayerLocation(setup_location))]
pub struct Player;

fn setup_location() -> PlayerLocation {
    PlayerLocation {
        forward: IVec2::Y,
        location: default(),
    }
}
