use audio::music::setup_background_music;
use bevy::prelude::*;
use bevy_rustysynth::RustySynthPlugin;
use bevy_sprite3d::Sprite3dPlugin;

mod audio;
mod characters;
mod combat;
mod gen;
mod loading;
mod room;
mod state;
mod tick;
mod ui;
mod utils;
mod visuals;

use bevy::diagnostic::*;
use bevy::window::*;
use bevy_inspector_egui::quick::*;
use characters::enemy::enemy::enemies_find_player;
use characters::enemy::enemy::move_enemies;
use characters::enemy::enemy::setup_enemies;
use characters::enemy::enemy::setup_enemy_atlas;
use characters::location::update_character_sprite_positions;
use characters::location::WorldLocation;
use characters::player::camera::*;
use characters::player::control::*;
use characters::player::player::setup_player;
use combat::combat::damage_enemy;
use combat::combat::damage_player;
use combat::combat::destroy_dead_enemies;
use combat::combat::update_combat;
use combat::combat::DamagedEvent;
use gen::location::*;
use gen::walker::*;
use loading::loading::*;
use room::mesh::*;
use state::GameState;
use tick::tick::TickEvent;
use ui::tint::damage_screen;
use ui::tint::destroy_tints;
use visuals::animation::animate_sprite;
use visuals::animation::Animations;
use visuals::billboard::update_billboards;

fn main() {
    App::new()
        .add_plugins(LogDiagnosticsPlugin::default())
        // .add_plugins(FrameTimeDiagnosticsPlugin::default())
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
        .register_type::<WorldLocation>()
        .add_event::<TickEvent>()
        .add_event::<DamagedEvent>()
        .insert_resource(WorldCatacomb::default())
        .insert_resource(Animations::default())
        .init_state::<GameState>()
        .add_systems(
            Update,
            check_assets_ready.run_if(in_state(GameState::Loading)),
        )
        .add_systems(OnExit(GameState::Loading), setup_enemy_atlas)
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
            (
                (
                    sync_camera,
                    move_player,
                    move_enemies,
                    enemies_find_player,
                    update_combat,
                    (damage_enemy, damage_player).chain(),
                    destroy_tints,
                    destroy_dead_enemies,
                    damage_screen,
                    animate_sprite,
                )
                    .run_if(in_state(GameState::Game)),
                (update_character_sprite_positions, update_billboards),
            ),
        )
        .add_systems(
            OnExit(GameState::Generating),
            ((
                setup_rooms,
                setup_walls,
                setup_background_music,
                setup_enemies,
            ),),
        )
        .run();
}
