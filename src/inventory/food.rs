use bevy::prelude::*;

use super::item::Item;

#[derive(Deref, DerefMut)]
pub struct Food(u32);

impl Item for Food {
    fn use_item(&mut self) -> bool {
        false
    }
}
