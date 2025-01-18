use bevy::{prelude::*, text::cosmic_text::ttf_parser::loca, utils::info};
use bevy_sprite3d::{Sprite3d, Sprite3dBuilder, Sprite3dParams};

use crate::{
    characters::{enemy::enemy::Enemy, location::WorldLocation, player::player::Player},
    tick::tick::TickEvent,
    visuals::{animation::{AnimationTimer, Animations}, billboard::Billboard},
};

#[derive(Component, Default, Debug)]
#[require(Health)]
pub struct Combat {
    pub done: bool,
    pub is_in_combat: bool,
}

#[derive(Component)]
pub struct Health(i32);

impl Default for Health {
    fn default() -> Self {
        Health(100)
    }
}

#[derive(Event)]
pub struct DamagedEvent(pub Entity);

pub fn update_combat(
    mut q_player: Query<(&mut WorldLocation, &Player, &mut Combat), Without<Enemy>>,
    mut q_enemies: Query<(&mut WorldLocation, &Enemy, &mut Combat)>,
    mut ev_tick: EventReader<TickEvent>,
) {
    for _ in ev_tick.read() {
        let (mut player_location, _, mut player_combat) = q_player.single_mut();
        if player_combat.is_in_combat {
            return;
        }

        for (mut enemy_location, _, mut enemy_combat) in q_enemies.iter_mut() {
            let distance = enemy_location
                .get_location()
                .distance_squared(player_location.get_location());
            if distance == 1 && enemy_location.can_move {
                player_location.face_towards(enemy_location.get_location());
                player_location.can_move = false;
                enemy_location.can_move = false;
                player_combat.is_in_combat = true;
                enemy_combat.is_in_combat = true;
                enemy_combat.done = true;
                break;
            }
        }
    }
}

pub fn damage_enemy(
    mut q_player: Query<(&Player, &mut Combat, &mut WorldLocation), Without<Enemy>>,
    mut q_enemies: Query<(&mut Health, &Enemy, &mut Combat, &mut AnimationTimer)>,
    mut ev_tick: EventReader<TickEvent>,
) {
    for _ in ev_tick.read() {
        let (_, mut player_combat, mut player_location) = q_player.single_mut();
        if player_combat.done || !player_combat.is_in_combat {
            return;
        }
        for (mut enemy_health, _, mut enemy_combat, mut enemy_animation) in q_enemies.iter_mut() {
            if !enemy_combat.is_in_combat {
                continue;
            }
            if !enemy_combat.done {
                continue;
            }
            enemy_health.0 -= 30;
            enemy_animation.play("pain".to_string(), Some("walk".to_string()));
            enemy_combat.done = false;
            player_combat.done = true;

            if enemy_health.0 <= 0 {
                player_combat.is_in_combat = false;
                player_location.can_move = true;
            }
        }
    }
}

pub fn damage_player(
    mut q_player: Query<(&mut Health, &Player, &mut Combat, Entity), Without<Enemy>>,
    mut q_enemies: Query<(&Enemy, &mut Combat, &mut AnimationTimer)>,
    mut ev_tick: EventReader<TickEvent>,
    mut ev_damaged: EventWriter<DamagedEvent>,
) {
    for _ in ev_tick.read() {
        let (mut player_health, _, mut player_combat, player_entity) = q_player.single_mut();
        if !player_combat.done || !player_combat.is_in_combat {
            return;
        }

        for (_, mut enemy_combat, mut enemy_animation) in q_enemies.iter_mut() {
            if !enemy_combat.is_in_combat {
                continue;
            }
            enemy_animation.play("attack".to_string(), Some("walk".to_string()));
            player_health.0 -= 30;
            ev_damaged.send(DamagedEvent(player_entity));
            enemy_combat.done = true;
            player_combat.done = false;
            break;
        }
    }
}

pub fn destroy_dead_enemies(
    mut commands: Commands,
    q_enemies: Query<(&Health, Entity, &AnimationTimer, &WorldLocation, &Transform), With<Enemy>>,
    asset_server: Res<AssetServer>,
    mut sprite_params: Sprite3dParams,
    animations: Res<Animations>,
) {
    for (health, entity, animation, location, transform) in q_enemies.iter() {
        if health.0 > 0 {
            continue;
        }

        let (_, layout, _) = animations.atlases.get(&animation.library.to_string()).unwrap();

        let texture_atlas = TextureAtlas {
            index: 0,
            layout: layout.clone(),
        };

        commands.entity(entity).despawn();
        commands.spawn((Sprite3dBuilder {
            image: asset_server.load("sprites/cultist.png"),
            unlit: true,
            pivot: Some(Vec2::new(0.5, 0.75)),
            ..default()
        }.bundle_with_atlas(&mut sprite_params, texture_atlas), Billboard, AnimationTimer {
            current_animation: "death".to_string(),
            library: animation.library.clone(),
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
            ..default()
        }, location.clone(), transform.clone()));
    }
}
