use bevy::prelude::*;
use bevy::text::cosmic_text::ttf_parser::loca;
use bevy::{
    asset::Asset,
    prelude::{Component, Deref, DerefMut, Transform},
    reflect::TypePath,
    sprite::Sprite,
};

use bevy_sprite3d::{Sprite3d, Sprite3dBuilder, Sprite3dParams};
use serde::{Deserialize, Serialize};

use crate::WorldCatacomb;
use crate::{characters::location::WorldLocation, visuals::billboard::Billboard};

use super::db::ItemDB;

#[derive(Component, Deref, DerefMut)]
#[require(WorldLocation, Billboard)]
pub struct WorldItem(usize);

pub fn spawn_item(
    location: IVec2,
    world: &Res<WorldCatacomb>,
    item_db: &Res<ItemDB>,
    commands: &mut Commands,
    mut sprite_params: &mut Sprite3dParams,
) {
    if !world.contains(&location) {
        return;
    }

    let (id, item) = item_db.pick_random_item();

    commands.spawn((
        WorldItem(id),
        WorldLocation::new(location, default()),
        Sprite3dBuilder {
            image: item.sprite.clone(),
            unlit: true,
            pixels_per_metre: 64.0,
            pivot: Some(Vec2::new(0.5, 0.75)),
            ..default()
        }
        .bundle(sprite_params),
    ));
}
