use audio::music::setup_background_music;
use bevy::prelude::*;
use bevy_rustysynth::RustySynthPlugin;
use bevy_sprite3d::Sprite3dPlugin;

mod audio;
mod gen;
mod loading;
mod room;
mod state;
mod utils;
mod characters;
mod visuals;

use bevy::diagnostic::*;
use bevy::window::*;
use bevy_inspector_egui::quick::*;
use characters::enemy::enemy::setup_enemies;
use characters::location::update_character_sprite_positions;
use characters::location::Location;
use characters::player::player::setup_player;
use gen::location::*;
use gen::walker::*;
use loading::loading::*;
use characters::player::camera::*;
use characters::player::control::*;
use room::mesh::*;
use state::GameState;
use visuals::billboard::update_billboards;

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
        .add_plugins(RustySynthPlugin::default())
        .add_plugins(Sprite3dPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .register_type::<Location>()
        .insert_resource(WorldCatacomb::default())
        .init_state::<GameState>()
        .add_systems(
            Update,
            check_assets_ready.run_if(in_state(GameState::Loading)),
        )
        .add_systems(OnEnter(GameState::Generating), setup_walkers)
        .add_systems(PostStartup, (setup_player, setup_camera, spawn_fog).chain())
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
            ((sync_camera, move_player).run_if(in_state(GameState::Game)), (update_character_sprite_positions, update_billboards))
        )
        .add_systems(
            OnExit(GameState::Generating),
            (setup_rooms, setup_walls, setup_background_music, setup_enemies),
        )
        .run();
}
