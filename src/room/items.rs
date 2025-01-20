use bevy::prelude::*;
use bevy_sprite3d::Sprite3dParams;

use crate::{
    inventory::{db::ItemDB, world::spawn_item},
    WorldCatacomb,
};

pub fn place_items(
    item_db: Res<ItemDB>,
    world: Res<WorldCatacomb>,
    mut commands: Commands,
    mut sprite_params: Sprite3dParams,
) {
    for _ in 0..=10 {
        spawn_item(
            world.pick_location(),
            &world,
            &item_db,
            &mut commands,
            &mut sprite_params,
        );
    }
}
