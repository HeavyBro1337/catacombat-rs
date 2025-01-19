use bevy::prelude::*;

use crate::audio::random::RandomAudio;

#[derive(Component, Default, DerefMut, Deref)]
pub struct PainSound(pub RandomAudio);
