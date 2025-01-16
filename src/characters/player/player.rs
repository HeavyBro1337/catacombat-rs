use bevy::math::IVec2;
use bevy::prelude::*;

use crate::characters::location::Location;


#[derive(Component)]
#[require(Location(setup_player))]
pub struct Player;

fn setup_player() -> Location {
    Location::new(default(), IVec2::Y)
}