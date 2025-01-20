use bevy::prelude::*;

use super::item::Item;

#[derive(Deref, DerefMut)]
pub struct Food(pub u32);

impl Item for Food {
    fn use_item(&mut self) -> bool {
        false
    }
}
