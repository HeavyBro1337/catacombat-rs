use bevy::prelude::*;
use bevy_sprite3d::{Sprite3d, Sprite3dBuilder, Sprite3dParams};
use rand::{rngs::ThreadRng, seq::SliceRandom};
use crate::{visuals::billboard::Billboard, Location, WorldCatacomb};

#[derive(Component)]
#[require(Location)]
pub struct Enemy;

pub fn setup_enemies(world: Res<WorldCatacomb>, mut commands: Commands, asset_server: Res<AssetServer>, mut sprite_params: Sprite3dParams) {
    let dirs = vec!(IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y);

    for room in world.0.iter().collect::<Vec<_>>().choose_multiple(&mut rand::thread_rng(), 5) {
        let room = **room;
        let face = dirs.choose(&mut rand::thread_rng()).unwrap();
        commands.spawn((Enemy, Billboard, Location::new(room, *face), Sprite3dBuilder {
            image: asset_server.load("sprites/cultist.png"),
            pixels_per_metre: 64.0,
            pivot: Some(Vec2::new(0.5, 0.75)),
            unlit: true,
            ..default()
        }.bundle(&mut sprite_params)));
    }
}