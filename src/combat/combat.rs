use bevy::prelude::*;

#[derive(Component)]
#[require(Health)]
pub struct Combat;

#[derive(Component)]
pub struct Health(i32);

impl Default for Health {
    fn default() -> Self {
        Health(100)
    }
}
