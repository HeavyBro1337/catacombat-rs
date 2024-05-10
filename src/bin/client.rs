use catacombat_rs::*;

pub use bevy::diagnostic::*;
pub use bevy::window::*;
pub use bevy_inspector_egui::quick::*;
pub use bevy_renet::RenetClientPlugin;

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
        .add_plugins(RenetClientPlugin)
        .add_plugins(NetcodeClientPlugin)
        // .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(MainMenuPlugin)
        .register_type::<PlayerLocation>()
        .init_state::<GameState>()
        .init_state::<NetworkState>()
        .insert_resource(WorldCatacomb::default())
        .add_systems(
            OnEnter(GameState::Generating),
            setup_walkers.run_if(in_state(NetworkState::Offline)),
        )
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
        .add_systems(
            OnEnter(GameState::Game),
            setup_rooms.run_if(in_state(NetworkState::Offline)),
        )
        .run();
}
