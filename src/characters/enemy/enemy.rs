use super::path::Path;
use crate::{
    characters::player::player::Player, tick::tick::TickEvent, visuals::billboard::Billboard,
    WorldCatacomb, WorldLocation,
};
use bevy::prelude::*;
use bevy_sprite3d::{Sprite3dBuilder, Sprite3dParams};
use rand::seq::SliceRandom;

#[derive(Component)]
#[require(WorldLocation, Path)]
pub struct Enemy;

pub fn setup_enemies(
    world: Res<WorldCatacomb>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut sprite_params: Sprite3dParams,
) {
    let dirs = vec![IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y];

    for room in world
        .0
        .iter()
        .collect::<Vec<_>>()
        .choose_multiple(&mut rand::thread_rng(), 1)
    {
        let room = **room;
        let face = dirs.choose(&mut rand::thread_rng()).unwrap();
        commands.spawn((
            Enemy,
            Billboard,
            WorldLocation::new(room, *face),
            Sprite3dBuilder {
                image: asset_server.load("sprites/cultist.png"),
                pixels_per_metre: 64.0,
                pivot: Some(Vec2::new(0.5, 0.75)),
                unlit: true,
                ..default()
            }
            .bundle(&mut sprite_params),
        ));
    }
}

pub fn enemies_find_player(
    mut q_enemies: Query<(&WorldLocation, &mut Path), With<Enemy>>,
    mut ev_tick: EventReader<TickEvent>,
    q_player: Query<&WorldLocation, With<Player>>,
    world: Res<WorldCatacomb>,
) {
    let player_location = q_player.single();
    for _ in ev_tick.read() {
        for (location, mut path) in q_enemies.iter_mut() {
            if path.has_path() {
                continue;
            }
            path.find_path(&location, &world, player_location.get_location());
        }
    }
}

pub fn move_enemies(
    mut q_enemies: Query<(&mut WorldLocation, &mut Path), With<Enemy>>,
    mut ev_tick: EventReader<TickEvent>,
    world: Res<WorldCatacomb>,
) {
    for _ in ev_tick.read() {
        info!("Moving enemies...");
        for (mut location, mut path) in q_enemies.iter_mut() {
            path.move_location(&mut location, &world);
        }
    }
}
