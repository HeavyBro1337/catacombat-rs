use bevy::prelude::*;

use crate::state::GameState;

use super::item::Item;

#[derive(Default, Resource, Deref, DerefMut)]
pub struct Inventory(Vec<usize>);

pub struct ItemInstance {
    pub ui_name: String,
    pub ui_description: String,
    pub item: Box<dyn Item>,
}

#[derive(DerefMut, Deref, Resource, Default)]
pub struct ItemDB(Vec<ItemInstance>);

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ItemDB::default())
            .insert_resource(Inventory::default())
            .add_systems(Startup, load_items_db_from_json);
    }
}

fn load_items_db_from_json(asset_server: Res<AssetServer>) {
    
}
