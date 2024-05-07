use bevy::{ecs::{component::Component, system::{Query, ResMut, Resource}}, math::IVec2, utils::HashSet};


#[derive(Resource, Default)]
pub struct Location(pub HashSet<IVec2>);
