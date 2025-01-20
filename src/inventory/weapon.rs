use serde::{Deserialize, Serialize};

use super::item::Item;


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WeaponCategory {
    Shotgun,
    Gun,
    Blunt,
    Sharp
}


#[derive(Debug)]
pub struct Weapon {
    pub durability: i32,
    pub damage: i32,
    pub category: WeaponCategory
}

impl Item for Weapon {
    fn use_item(&mut self) -> bool {
        true
    }
}

#[derive(Debug)]
pub struct Ammo {
    pub category: WeaponCategory
}

impl Item for Ammo {
    fn use_item(&mut self) -> bool {
        false
    }
}