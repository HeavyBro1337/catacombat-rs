use bevy::{diagnostic::*, prelude::*, window::WindowResolution};

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use gen::{
    location::WorldCatacomb,
    walker::{check_walkers, destroy_walker_generators, setup_walkers, walk_walker_generators},
};
use player::{
    camera::{setup_camera, spawn_fog, sync_camera},
    control::move_player,
    player::PlayerLocation,
};
use room::mesh::setup_rooms;
use state::GameState;

mod gen;
mod player;
mod room;
mod state;
mod utils;

fn main() {
    App::new()
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        // .add_plugins(PlayerPlugin)
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Catacombat".to_string(),
                        resolution: WindowResolution::new(1024.0, 600.0),
                        present_mode: bevy::window::PresentMode::Immediate,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(WorldInspectorPlugin::new())
        .register_type::<PlayerLocation>()
        .init_state::<GameState>()
        .insert_resource(WorldCatacomb::default())
        .add_systems(PostStartup, (setup_camera, spawn_fog).chain())
        .add_systems(Startup, setup_walkers)
        .add_systems(
            Update,
            (
                walk_walker_generators,
                destroy_walker_generators,
                check_walkers,
            )
                .run_if(in_state(GameState::Generating)),
        )
        .add_systems(
            Update,
            (sync_camera, move_player).run_if(in_state(GameState::Game)),
        )
        .add_systems(OnEnter(GameState::Game), setup_rooms)
        .run();
}
