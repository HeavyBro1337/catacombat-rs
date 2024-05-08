use bevy::{ecs::system::Resource, math::IVec2, utils::HashSet};

#[derive(Resource, Default)]
pub struct WorldCatacomb(pub HashSet<IVec2>);
