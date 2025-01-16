use bevy::prelude::*;
use bevy_sprite3d::Sprite3dPlugin;

mod gen;
mod loading;
mod player;
mod room;
mod state;
mod utils;

use bevy::diagnostic::*;
use bevy::window::*;
use bevy_inspector_egui::quick::*;
use gen::location::*;
use gen::walker::*;
use loading::loading::*;
use player::camera::*;
use player::control::*;
use player::player::*;
use room::mesh::*;
use state::GameState;

fn main() {
    App::new()
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(PreStartup, setup_loading)
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
        .add_plugins(Sprite3dPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .register_type::<PlayerLocation>()
        .insert_resource(WorldCatacomb::default())
        .init_state::<GameState>()
        .add_systems(
            Update,
            check_assets_ready.run_if(in_state(GameState::Loading)),
        )
        .add_systems(OnEnter(GameState::Generating), setup_walkers)
        .add_systems(PostStartup, (setup_camera, spawn_fog).chain())
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
        .add_systems(OnExit(GameState::Generating), setup_rooms)
        .run();
}
