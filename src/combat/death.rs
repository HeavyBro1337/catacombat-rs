pub use bevy::prelude::*;

use crate::audio::random::RandomAudio;

#[derive(Component, Default, Deref, DerefMut)]
pub struct DeathSound(pub RandomAudio);
