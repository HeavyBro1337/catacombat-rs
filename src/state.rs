use bevy::prelude::*;

#[derive(Resource, States, Default, Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum GameState {
    Loading = 0,
    Game = 1,
    #[default]
    Generating = 2,
}