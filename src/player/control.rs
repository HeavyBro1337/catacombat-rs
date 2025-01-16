use bevy::prelude::*;

use crate::gen::location::WorldCatacomb;

use super::player::{PlayerLocation, Turn};

pub fn move_player(
    world: Res<WorldCatacomb>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut q_player: Query<&mut PlayerLocation>,
) {
    let mut player_loc = q_player.single_mut();
    if keyboard.just_pressed(KeyCode::KeyA) {
        player_loc.turn(Turn::Left)
    }
    if keyboard.just_pressed(KeyCode::KeyD) {
        player_loc.turn(Turn::Right)
    }
    if mouse.just_pressed(MouseButton::Left) {
        player_loc.move_forward(&world);
    }
}
