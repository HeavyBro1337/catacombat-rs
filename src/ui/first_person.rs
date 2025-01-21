use bevy::{prelude::*, render::view::RenderLayers};
use bevy_sprite3d::{Sprite3dBuilder, Sprite3dParams};

use crate::{
    characters::player::player::Player,
    visuals::{
        animation::{AnimationInfo, AnimationTimer, Animations},
        billboard::Billboard,
    },
    MainCamera,
};

#[derive(Component)]
pub struct FirstPersonArms;

pub fn spawn_first_person_weapon(
    mut commands: Commands,
    animations: Res<Animations>,
    asset_server: Res<AssetServer>,
    q_camera: Query<Entity, With<MainCamera>>,
    mut sprite_params: Sprite3dParams,
) {
    let entity = q_camera.single();

    let child = commands
        .spawn((
            FirstPersonArms,
            Sprite3dBuilder {
                unlit: true,
                image: asset_server.load("sprites/ui/sawn_off.png"),
                pivot: Some((0.5, 1.0).into()),
                pixels_per_metre: 64.0,
                ..default()
            }
            .bundle_with_atlas(
                &mut sprite_params,
                TextureAtlas {
                    layout: animations
                        .atlases
                        .get(&"Shotgun".to_string())
                        .unwrap()
                        .1
                        .clone(),
                    ..default()
                },
            ),
            AnimationTimer {
                timer: Timer::from_seconds(0.3333, TimerMode::Repeating),
                library: "Shotgun".into(),
                current_animation: "fire".into(),
                ..default()
            },
            Name::new("First Person Arm"),
            Transform::from_translation((0.0, 0.0, -1.5).into()),
        ))
        .id();

    commands.entity(entity).add_child(child);
}

pub fn setup_weapon_atlas(
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut animations: ResMut<Animations>,
) {
    let layout = texture_atlases.add(TextureAtlasLayout::from_grid(
        (279, 133).into(),
        6,
        1,
        None,
        None,
    ));

    animations.new_animation(
        "Shotgun".to_string(),
        "fire".to_string(),
        AnimationInfo {
            len: 6,
            row: 0,
            looped: true,
        },
        layout.clone(),
        6,
    );
}
