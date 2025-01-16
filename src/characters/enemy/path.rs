use std::collections::VecDeque;
use std::default;

use bevy::{prelude::*, text::cosmic_text::ttf_parser::loca};

use crate::characters::location::{self, Turn};
use crate::{characters::location::WorldLocation, WorldCatacomb};
use pathfinding::prelude::*;

#[derive(Component, Default)]
pub struct Path(Vec<IVec2>);

impl Path {
    pub fn has_path(&self) -> bool {
        !self.0.is_empty()
    }

    pub fn move_location(&mut self, location: &mut WorldLocation, world: &Res<WorldCatacomb>) {
        if !self.has_path() {
            error!("Path is empty. Returning!");
            return;
        }

        let next_position = self.0[0];

        if next_position == location.get_location() {
            self.0.remove(0);
            self.move_location(location, world);
            return;
        }

        let delta = next_position - location.get_location();
        dbg!((
            next_position,
            location.get_location(),
            delta,
            location.get_forward()
        ));
        if location.get_forward() == delta {
            info!("Moved forward!");
            location.move_forward(world);
        } else {
            location.turn(Turn::Left);
        }
    }

    pub fn find_path(
        &mut self,
        location: &WorldLocation,
        world: &Res<WorldCatacomb>,
        target: IVec2,
    ) {
        if !world.0.contains(&target) {
            error!("Target is outside the map.");
            return;
        }

        let start = location.get_location();

        match astar(
            &start,
            &|p: &IVec2| {
                let mut successors = vec![
                    p + IVec2::new(1, 0),
                    p + IVec2::new(-1, 0),
                    p + IVec2::new(0, 1),
                    p + IVec2::new(0, -1),
                ];

                successors
                    .iter()
                    .filter(|p| world.0.contains(*p))
                    .map(|p| (*p, 1))
                    .collect::<Vec<_>>()
            },
            |pos| (target - pos).abs().dot(IVec2::new(1, 1)) as u32,
            |pos| *pos == target,
        ) {
            None => error!("Couldn't find path"),
            Some((p, _)) => self.0 = p,
        }
    }
}
