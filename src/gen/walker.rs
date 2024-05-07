use bevy::{ecs::{component::Component, system::{Query, ResMut, Resource}}, math::IVec2, utils::HashSet};

use super::location::{Location};




#[derive(Component)]
pub struct Walker {
    pub current_location: IVec2,
}


impl Walker {
    pub fn walk(&mut self, location: &mut ResMut<Location>) {
        use rand::prelude::*;

        let offset = IVec2 { 
            x: rand::thread_rng().gen_range(-1..=1), 
            y: rand::thread_rng().gen_range(-1..=1)
        };

        self.current_location += offset;

        location.0.insert(self.current_location);
    }
}

pub fn walk_walkers(mut q_walkers: Query<&mut Walker>, mut location: ResMut<Location>) {
    for mut walker in q_walkers.iter_mut() {
        walker.walk(&mut location);
    }
}