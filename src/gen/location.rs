use bevy::{
    ecs::system::Resource,
    math::IVec2,
    prelude::{Deref, DerefMut},
    utils::HashSet,
};
use rand::prelude::*;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct WorldCatacomb(HashSet<IVec2>);

impl WorldCatacomb {
    pub fn pick_location(&self) -> IVec2 {
        **self
            .iter()
            .collect::<Vec<_>>()
            .choose(&mut thread_rng())
            .unwrap()
    }
}
