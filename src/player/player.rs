use bevy::math::IVec2;
use bevy::prelude::*;
use bevy_sprite3d::{Sprite3d, Sprite3dBundle, Sprite3dParams};
use serde::{Deserialize, Serialize};

use crate::gen::location::WorldCatacomb;

#[derive(Component, Reflect)]
pub struct PlayerLocation {
    location: IVec2,
    forward: IVec2,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerLocationNetwork {
    location: [i32; 2],
    forward: [i32; 2],
}

impl Into<PlayerLocation> for PlayerLocationNetwork {
    fn into(self) -> PlayerLocation {
        PlayerLocation {
            location: IVec2::from_array(self.location),
            forward: IVec2::from_array(self.forward),
        }
    }
}

impl Into<PlayerLocationNetwork> for PlayerLocation {
    fn into(self) -> PlayerLocationNetwork {
        PlayerLocationNetwork {
            location: self.location.to_array(),
            forward: self.forward.to_array(),
        }
    }
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
pub struct OtherPlayer;

#[derive(Bundle)]
pub struct PlayerBundle {
    sprite: Sprite3dBundle,
    location: PlayerLocation,
    other_player: OtherPlayer,
}

impl PlayerBundle {
    pub fn new(image: &Handle<Image>, sprite_params: &mut Sprite3dParams) -> Self {
        Self {
            sprite: Sprite3d {
                image: image.clone(),
                alpha_mode: AlphaMode::Blend,
                pixels_per_metre: 400.,
                ..default()
            }
            .bundle(sprite_params),
            other_player: OtherPlayer,
            location: PlayerLocation::new(),
        }
    }
}
