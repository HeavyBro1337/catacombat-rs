use bevy::{diagnostic::*, prelude::*, window::WindowResolution};
use gen::{location::Location, walker::{destroy_walker_generators, setup_walkers, walk_walker_generators}};
use room::mesh::{check_walkers, setup_rooms};
use state::GameState;

mod gen;
mod room;
mod state;

fn main() {
    App::new()
    .add_plugins(LogDiagnosticsPlugin::default())
    .add_plugins(FrameTimeDiagnosticsPlugin::default())
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
    .init_state::<GameState>()
    .insert_resource(Location::default())
    .add_systems(Startup, setup_walkers)
    .add_systems(Update, (
        walk_walker_generators,
        destroy_walker_generators, 
        check_walkers).run_if(in_state(GameState::Generating)))
    .add_systems(OnEnter(GameState::Game), setup_rooms)
    .run();
}
