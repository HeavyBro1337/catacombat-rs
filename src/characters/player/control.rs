use bevy::prelude::*;

use crate::{
    characters::location::{Turn, WorldLocation},
    gen::location::WorldCatacomb,
    tick::tick::TickEvent,
};

use super::player::Player;

pub fn move_player(
    world: Res<WorldCatacomb>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut ev_tick: EventWriter<TickEvent>,
    mut q_player: Query<&mut WorldLocation, With<Player>>,
) {
    let mut player_loc = q_player.single_mut();
    if keyboard.just_pressed(KeyCode::KeyA) {
        ev_tick.send(TickEvent);
        player_loc.turn(Turn::Left)
    }
    if keyboard.just_pressed(KeyCode::KeyD) {
        ev_tick.send(TickEvent);
        player_loc.turn(Turn::Right)
    }
    if mouse.just_pressed(MouseButton::Left) {
        ev_tick.send(TickEvent);
        player_loc.move_forward(&world);
    }
}
