use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;

use crate::{state::GameState, LoadingAssets};

use super::{
    db::ItemDB,
    item::{get_item_from_type, Item, ItemMetas},
};

#[derive(Default, Resource, Deref, DerefMut)]
pub struct Inventory(Vec<usize>);

impl Inventory {
    pub fn load_items<'a>(&self, db: &'a Res<ItemDB>) -> Vec<&'a ItemInstance> {
        self.iter()
            .map(|index| db.get(*index).unwrap())
            .collect::<Vec<_>>()
    }
}

pub struct ItemInstance {
    pub ui_name: String,
    pub ui_description: String,
    pub item: Box<dyn Item>,
    pub sprite: Handle<Image>,
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ItemDB::default())
            .insert_resource(Inventory::default())
            .add_plugins(JsonAssetPlugin::<ItemMetas>::new(&["items.json"]))
            .add_systems(Startup, setup_json_loading)
            .add_systems(OnExit(GameState::Loading), load_items_db_from_json);
    }
}

fn load_items_db_from_json(
    asset_server: Res<AssetServer>,
    item_metas: Res<Assets<ItemMetas>>,
    mut items_db: ResMut<ItemDB>,
) {
    let metas = asset_server.load::<ItemMetas>("default.items.json");
    let Some(metas) = item_metas.get(&metas) else {
        panic!("Could not load items.json. Is anything ok?");
    };
    for meta in metas.iter() {
        items_db.push(ItemInstance {
            sprite: asset_server.load(meta.sprite.clone()),
            ui_name: meta.name.clone(),
            ui_description: "unused for now".to_string(), // TODO: Add description later.
            item: get_item_from_type(&meta.item_type, &meta),
        });
    }
}

fn setup_json_loading(mut loading_assets: ResMut<LoadingAssets>, asset_server: Res<AssetServer>) {
    loading_assets.push(asset_server.load_untyped("default.items.json").untyped());
}
