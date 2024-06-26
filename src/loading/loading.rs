use bevy::prelude::*;

use crate::GameState;

#[derive(Resource, Default)]
pub struct LoadingAssets(Vec<UntypedHandle>);

pub fn setup_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut loading_assets = LoadingAssets::default();

    let doomguy_sprite: Handle<Image> = asset_server.load("sprites/doomguy.png");
    loading_assets.0.push(doomguy_sprite.clone().untyped());

    commands.insert_resource(loading_assets);
}

pub fn check_assets_ready(
    server: Res<AssetServer>,
    loading: Res<LoadingAssets>,
    mut state: ResMut<NextState<GameState>>,
) {
    use bevy::asset::LoadState;

    if loading
        .0
        .iter()
        .all(|asset| match server.get_load_state(asset.id()).unwrap() {
            LoadState::Loaded => true,
            _ => false,
        })
    {
        println!("loaded");
        state.set(GameState::Menu);
    }
}
