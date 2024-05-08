use bevy::{ecs::{component::Component, system::{Command, Query, ResMut, Resource}}, math::IVec2, utils::HashSet};
use bevy::prelude::*;
use rand::{thread_rng, Rng};
use super::location::{Location};




#[derive(Component)]
pub struct Walker {
    pub current_location: IVec2,
    steps_to_live: u32
}


impl Walker {
    pub fn new(start: IVec2) -> Self {
        Walker { current_location: start, steps_to_live: thread_rng().gen_range(3..=10) }
    }

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

pub fn walk_walker_generators(mut q_walkers: Query<&mut Walker>, mut location: ResMut<Location>) {
    for mut walker in q_walkers.iter_mut() {
        walker.walk(&mut location);
    }
}

pub fn destroy_walker_generators(
    mut q_walkers: Query<(Entity, &Walker)>, 
    mut commands: Commands) {
    for (entity, walker) in q_walkers.iter() {
        if walker.steps_to_live > 0 { continue; }
        commands.entity(entity).despawn()
    }
}

pub fn setup_walkers(mut commands: Commands) {
    for _ in 0..4 {
        commands.spawn(Walker::new(default()));
    }
}