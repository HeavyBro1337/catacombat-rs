use bevy::prelude::*;
use bevy_rustysynth::MidiAudio;

pub fn setup_background_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    let midi_handle = asset_server.load::<MidiAudio>("music/catacombs.mid");

    commands.spawn(AudioPlayer(midi_handle));
}
