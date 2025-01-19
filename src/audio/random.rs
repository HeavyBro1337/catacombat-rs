use std::path::PathBuf;

use bevy::prelude::*;
use rand::prelude::*;

#[derive(Default)]
pub struct RandomAudio(Vec<Handle<AudioSource>>);

impl RandomAudio {
    pub fn new(audios: Vec<Handle<AudioSource>>) -> Self {
        Self(audios)
    }

    pub fn pick(&self) -> Option<&Handle<AudioSource>> {
        self.0.choose(&mut thread_rng())
    }
}

pub fn get_audio_handles(
    asset_server: &Res<AssetServer>,
    paths: Vec<String>,
) -> Vec<Handle<AudioSource>> {
    paths
        .iter()
        .map(|path| asset_server.load::<AudioSource>(path))
        .collect::<Vec<_>>()
}
