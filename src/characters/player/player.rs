use bevy::math::IVec2;
use bevy::prelude::*;
use bevy_flycam::FlyCam;

use crate::characters::location::WorldLocation;

#[derive(Component)]
#[require(WorldLocation(new_player_location))]
pub struct Player;

fn new_player_location() -> WorldLocation {
    WorldLocation::new(default(), IVec2::Y)
}

pub fn setup_player(mut commands: Commands, q_fly_cam: Query<&FlyCam>) {
    if !q_fly_cam.is_empty() {
        return;
    }

    commands.spawn(Player);
}
