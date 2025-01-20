use bevy::{
    asset::Asset,
    prelude::{Deref, DerefMut},
    reflect::TypePath,
};

use serde::{Deserialize, Serialize};

use super::{food::Food, weapon::{Ammo, Weapon, WeaponCategory}};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum ItemType {
    Weapon,
    Common,
    AmmoSupply,
    Food,
    Armor
}

pub trait Item: Send + Sync {
    fn use_item(&mut self) -> bool;
}

#[derive(serde::Deserialize, Asset, TypePath, Debug, Deref, DerefMut)]
#[serde(transparent)]
pub struct ItemMetas(Vec<ItemMeta>);

#[derive(serde::Deserialize, Asset, TypePath, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemMeta {
    pub name: String,
    #[serde(rename(deserialize = "type"))]
    pub item_type: ItemType,
    pub sprite: String,
    pub weapon_properties: Option<WeaponMeta>,
    pub ammo_properties: Option<AmmoMeta>,
}

#[derive(serde::Deserialize, Asset, TypePath, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WeaponMeta {
    pub durability: i32,
    pub damage: i32,
    pub category: WeaponCategory,
}

#[derive(serde::Deserialize, Asset, TypePath, Debug, Clone)]
pub struct AmmoMeta {
    pub category: WeaponCategory,
}

pub fn get_item_from_type(t: &ItemType, meta: &ItemMeta) -> Box<dyn Item> {
    match t {
        ItemType::Weapon => Box::new(Weapon {
            damage: meta.clone().weapon_properties.unwrap().damage,
            durability: meta.clone().weapon_properties.unwrap().durability,
            category: meta.clone().weapon_properties.unwrap().category,
        }),
        ItemType::Common => todo!(),
        ItemType::AmmoSupply => Box::new(Ammo {
            category: meta.clone().ammo_properties.unwrap().category,
        }),
        ItemType::Food => Box::new(Food(10)), // TODO: Make Food customizable in JSON.
        ItemType::Armor => todo!(),
    }
}