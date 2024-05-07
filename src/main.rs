use bevy::{diagnostic::*, prelude::*, window::WindowResolution};

mod gen;

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
    .run();
}
