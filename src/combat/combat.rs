use bevy::{prelude::*, utils::info};

use crate::{
    characters::{enemy::enemy::Enemy, location::WorldLocation, player::player::Player},
    tick::tick::TickEvent,
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
    mut q_enemies: Query<(&mut Health, &Enemy, &mut Combat)>,
    mut ev_tick: EventReader<TickEvent>,
) {
    for _ in ev_tick.read() {
        let (_, mut player_combat, mut player_location) = q_player.single_mut();
        if player_combat.done || !player_combat.is_in_combat {
            return;
        }
        for (mut enemy_health, _, mut enemy_combat) in q_enemies.iter_mut() {
            dbg!(enemy_combat.is_in_combat);
            dbg!(enemy_combat.done);
            if !enemy_combat.is_in_combat {
                continue;
            }
            if !enemy_combat.done {
                continue;
            }
            enemy_health.0 -= 30;
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
    mut q_enemies: Query<(&Enemy, &mut Combat)>,
    mut ev_tick: EventReader<TickEvent>,
    mut ev_damaged: EventWriter<DamagedEvent>
) {
    for _ in ev_tick.read() {
        let (mut health, _, mut player_combat, player_entity) = q_player.single_mut();
        info!("Trying to damage player");
        if !player_combat.done || !player_combat.is_in_combat {
            return;
        }
        info!("Player is fightable");

        for (_, mut enemy_combat) in q_enemies.iter_mut() {
            if !enemy_combat.is_in_combat {
                continue;
            }
        info!("Enemy is in combat");

            health.0 -= 30;
            ev_damaged.send(DamagedEvent(player_entity));
            enemy_combat.done = true;
            player_combat.done = false;
            break
        }
    }
}

pub fn destroy_dead_enemies(
    mut commands: Commands,
    q_enemies: Query<(&Health, Entity), With<Enemy>>,
) {
    for (health, entity) in q_enemies.iter() {
        if health.0 > 0 {
            continue;
        }

        commands.entity(entity).despawn();
    }
}
