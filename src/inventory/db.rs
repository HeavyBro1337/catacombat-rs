use bevy::prelude::*;
use rand::prelude::*;

use super::inventory::ItemInstance;

#[derive(DerefMut, Deref, Resource, Default)]
pub struct ItemDB(Vec<ItemInstance>);

impl ItemDB {
    pub fn pick_random_item(&self) -> (usize, &ItemInstance) {
        let index = thread_rng().gen::<usize>() % self.len();

        (index, &self[index])
    }
}
