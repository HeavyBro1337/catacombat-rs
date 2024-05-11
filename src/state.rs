use bevy::prelude::*;

#[derive(Resource, States, Default, Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum GameState {
    #[default]
    Loading = 0,
    Game = 1,
    Generating = 2,
    Menu = 3,
}

#[derive(Resource, States, Default, Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum NetworkState {
    #[default]
    Offline = 0,
    Online = 1,
}
