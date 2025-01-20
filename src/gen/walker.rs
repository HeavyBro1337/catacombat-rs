use crate::state::GameState;
use bevy::prelude::*;
use bevy::{
    ecs::{
        component::Component,
        system::{Query, ResMut},
    },
    math::IVec2,
};
use rand::{thread_rng, Rng};

use super::location::WorldCatacomb;

#[derive(Component)]
pub struct Walker {
    pub current_location: IVec2,
    steps_to_live: u32,
}

impl Walker {
    pub fn new(start: IVec2) -> Self {
        Walker {
            current_location: start,
            steps_to_live: thread_rng().gen_range(3..=70),
        }
    }

    pub fn walk(&mut self, location: &mut ResMut<WorldCatacomb>) {
        if self.steps_to_live == 0 {
            return;
        }
        let is_x = thread_rng().gen_bool(0.5);
        use rand::prelude::*;

        let offset = IVec2 {
            x: if is_x {
                rand::thread_rng().gen_range(-1..=1)
            } else {
                0
            },
            y: if !is_x {
                rand::thread_rng().gen_range(-1..=1)
            } else {
                0
            },
        };
        self.current_location += offset;

        if !location.insert(self.current_location) {
            self.steps_to_live -= 1;
        }
    }
}

pub fn walk_walker_generators(
    mut q_walkers: Query<&mut Walker>,
    mut location: ResMut<WorldCatacomb>,
) {
    for mut walker in q_walkers.iter_mut() {
        walker.walk(&mut location);
    }
}

pub fn destroy_walker_generators(q_walkers: Query<(Entity, &Walker)>, mut commands: Commands) {
    for (entity, walker) in q_walkers.iter() {
        if walker.steps_to_live > 0 {
            continue;
        }
        commands.entity(entity).despawn()
    }
}

pub fn setup_walkers(mut commands: Commands) {
    print!("Setting up random walkers...");
    for _ in 0..4 {
        commands.spawn(Walker::new(default()));
    }
    println!(" Done!");
}

pub fn check_walkers(
    q_walkers: Query<Entity, With<Walker>>,
    mut state: ResMut<NextState<GameState>>,
) {
    if !q_walkers.is_empty() {
        return;
    }
    info!("Done generating...");
    state.set(GameState::Game);
}
