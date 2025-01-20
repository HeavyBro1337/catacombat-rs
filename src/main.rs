use std::fs::File;
use std::io::Cursor;
use std::path::PathBuf;

use audio::music::setup_background_music;
use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_rustysynth::RustySynthPlugin;
use bevy_sprite3d::Sprite3dPlugin;

mod audio;
mod characters;
mod combat;
mod gen;
mod inventory;
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
use clap::arg;
use clap::command;
use clap::value_parser;
use clap::ArgMatches;
use combat::combat::check_enemy_combat;
use combat::combat::check_player_combat;
use combat::combat::damage_enemy;
use combat::combat::damage_player;
use combat::combat::despawn_dead_enemies;
use combat::combat::update_combat;
use combat::combat::CombatEvent;
use combat::combat::CombatState;
use combat::combat::DamagedEvent;
use gen::location::*;
use gen::walker::*;
use inventory::inventory::InventoryPlugin;
use inventory::item::ItemMeta;
use loading::loading::*;
use room::mesh::*;
use state::GameState;
use std::io::Read;
use tick::tick::TickEvent;
use ui::tint::damage_screen;
use ui::tint::destroy_tints;
use visuals::animation::animate_sprite;
use visuals::animation::Animations;
use visuals::billboard::update_billboards;

fn cli() -> ArgMatches {
    command!()
        .arg(
            arg!([soundfont]
                -s --soundfont <FILE> "Sets custom soundfont before startup"
            )
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .get_matches()
}

fn try_open_soundfont(path: Option<PathBuf>) -> Vec<u8> {
    let hl4mgm: Vec<u8> = include_bytes!("./embedded/soundfonts/hl4mgm.sf2").to_vec();

    if path.is_none() {
        return hl4mgm;
    }

    let path = path.unwrap();

    match File::open(path.clone()) {
        Ok(mut file) => {
            info!("Opened custom soundfont.");
            let mut buffer = Vec::new();
            match file.read_to_end(&mut buffer) {
                Ok(_) => {
                    info!("Successfully loaded custom soundfont!");
                    buffer
                }
                Err(err) => {
                    warn!(
                        "Failed to open soundfont at path \"{}\": {}.\nUsing hl4mgm.sf2",
                        path.display(),
                        err
                    );
                    hl4mgm
                }
            }
        }
        Err(err) => {
            warn!(
                "Failed to open soundfont at path \"{}\": {}.\nUsing hl4mgm.sf2",
                path.display(),
                err
            );
            hl4mgm
        }
    }
}

fn main() {
    let matches = cli();

    let soundfont_path = matches.get_one::<PathBuf>("soundfont").cloned();
    dbg!(soundfont_path.clone());
    let sf2_vec = try_open_soundfont(soundfont_path).clone();

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
        .add_plugins(RustySynthPlugin {
            soundfont: Cursor::new(sf2_vec),
        })
        .add_plugins(Sprite3dPlugin)
        .add_plugins(InventoryPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .register_type::<WorldLocation>()
        .add_event::<TickEvent>()
        .add_event::<CombatEvent>()
        .add_event::<DamagedEvent>()
        .insert_resource(WorldCatacomb::default())
        .insert_resource(Animations::default())
        .insert_resource(CombatState {
            cooldown: Timer::from_seconds(0.5, TimerMode::Once),
            opponent: None,
            is_player_turn: false,
        })
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
                    despawn_dead_enemies,
                    check_player_combat,
                    check_enemy_combat,
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
