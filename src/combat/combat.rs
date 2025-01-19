use bevy::prelude::*;
use bevy_sprite3d::{Sprite3dBuilder, Sprite3dParams};

use crate::{
    characters::{enemy::enemy::Enemy, location::WorldLocation, player::player::Player},
    tick::tick::TickEvent,
    visuals::{
        animation::{AnimationTimer, Animations},
        billboard::Billboard,
    },
};

#[derive(Resource)]
pub struct CombatState {
    pub cooldown: Timer,
    pub opponent: Option<Entity>,
    pub is_player_turn: bool,
}

#[derive(Event)]
pub struct CombatEvent(Entity);

#[derive(Component, Default, Debug)]
#[require(Health)]
pub struct Combat {
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
    mut q_enemies: Query<(&mut WorldLocation, &Enemy, Entity)>,
    mut ev_tick: EventReader<TickEvent>,
    mut combat_state: ResMut<CombatState>,
) {
    let (mut player_location, _, mut player_combat) = q_player.single_mut();
    player_combat.is_in_combat = combat_state.opponent.is_some();
    player_location.can_move = combat_state.opponent.is_none();
    if player_combat.is_in_combat {
        return;
    }
    for _ in ev_tick.read() {
        for (mut enemy_location, _, enemy_entity) in q_enemies.iter_mut() {
            let distance = enemy_location
                .get_location()
                .distance_squared(player_location.get_location());
            if distance == 1 && enemy_location.can_move {
                player_location.face_towards(enemy_location.get_location());
                combat_state.opponent = Some(enemy_entity);
                enemy_location.can_move = false;
                break;
            }
        }
    }
}

pub fn damage_enemy(
    mut q_player: Query<(&Player, &mut Combat, &mut WorldLocation), Without<Enemy>>,
    mut q_enemies: Query<(
        &mut Health,
        &Enemy,
        &mut Combat,
        &mut AnimationTimer,
        Entity,
    )>,
    mut ev_combat: EventReader<CombatEvent>,
    mut ev_damaged: EventWriter<DamagedEvent>,
    combat_state: ResMut<CombatState>,
) {
    if !combat_state.is_player_turn {
        return;
    }

    for _ in ev_combat.read() {
        let (_, mut player_combat, mut player_location) = q_player.single_mut();
        if !player_combat.is_in_combat {
            return;
        }

        let Ok((mut enemy_health, _, enemy_combat, mut enemy_animation, enemy_entity)) =
            q_enemies.get_mut(combat_state.opponent.unwrap())
        else {
            return;
        };

        enemy_health.0 -= 30;
        enemy_animation.play("pain".to_string(), Some("walk".to_string()));
        ev_damaged.send(DamagedEvent(enemy_entity));
        if enemy_health.0 <= 0 {
            player_combat.is_in_combat = false;
            player_location.can_move = true;
        }

        break;
    }
}

pub fn check_player_combat(
    mut q_player: Query<(&mut Combat, Entity), With<Player>>,
    mut ev_tick: EventReader<TickEvent>,
    mut ev_combat: EventWriter<CombatEvent>,
    mut combat_state: ResMut<CombatState>,
    time: Res<Time>,
) {
    combat_state.cooldown.tick(time.delta());

    if !combat_state.cooldown.finished() {
        return;
    }

    if !combat_state.is_player_turn {
        return;
    }

    for _ in ev_tick.read() {
        let (player_combat, player_entity) = q_player.single_mut();
        if !player_combat.is_in_combat {
            return;
        }
        combat_state.is_player_turn = false;
        combat_state.cooldown.reset();

        ev_combat.send(CombatEvent(player_entity));
    }
}

pub fn check_enemy_combat(
    mut q_enemies: Query<Entity, With<Enemy>>,
    mut ev_tick: EventReader<TickEvent>,
    mut ev_combat: EventWriter<CombatEvent>,
    mut combat_state: ResMut<CombatState>,
    time: Res<Time>,
) {
    combat_state.cooldown.tick(time.delta());

    if !combat_state.cooldown.finished() {
        return;
    }

    if combat_state.opponent.is_none() {
        return;
    }

    if combat_state.is_player_turn {
        return;
    }

    for _ in ev_tick.read() {
        let Ok(enemy_entity) = q_enemies.get_mut(combat_state.opponent.unwrap()) else {
            return;
        };
        combat_state.cooldown.reset();
        combat_state.is_player_turn = true;
        ev_combat.send(CombatEvent(enemy_entity));
    }
}

pub fn damage_player(
    mut q_player: Query<(&mut Health, &Player, &Combat, Entity), Without<Enemy>>,
    mut q_enemies: Query<(&Enemy, &mut AnimationTimer)>,
    mut ev_combat: EventReader<CombatEvent>,
    mut ev_damaged: EventWriter<DamagedEvent>,
    combat_state: ResMut<CombatState>,
) {
    if combat_state.is_player_turn {
        return;
    }

    if combat_state.opponent.is_none() {
        return;
    }

    for _ in ev_combat.read() {
        let (mut player_health, _, player_combat, player_entity) = q_player.single_mut();
        if !player_combat.is_in_combat {
            return;
        }

        let Ok((_, mut enemy_animation)) = q_enemies.get_mut(combat_state.opponent.unwrap()) else {
            return;
        };
        enemy_animation.play("attack".to_string(), Some("walk".to_string()));
        player_health.0 -= 30;
        ev_damaged.send(DamagedEvent(player_entity));
    }
}

pub fn despawn_dead_enemies(
    mut commands: Commands,
    q_enemies: Query<(&Health, Entity, &AnimationTimer, &WorldLocation, &Transform), With<Enemy>>,
    asset_server: Res<AssetServer>,
    mut sprite_params: Sprite3dParams,
    animations: Res<Animations>,
    mut combat_state: ResMut<CombatState>,
) {
    if combat_state.opponent.is_some() {
        combat_state.opponent = if q_enemies.get(combat_state.opponent.unwrap()).is_err() {
            None
        } else {
            combat_state.opponent
        }
    }

    for (health, entity, animation, location, transform) in q_enemies.iter() {
        if health.0 > 0 {
            continue;
        }

        let (_, layout, _) = animations
            .atlases
            .get(&animation.library.to_string())
            .unwrap();

        let texture_atlas = TextureAtlas {
            index: 0,
            layout: layout.clone(),
        };

        commands.entity(entity).despawn();
        commands.spawn((
            Sprite3dBuilder {
                image: asset_server.load("sprites/cultist.png"),
                unlit: true,
                pixels_per_metre: 64.0,
                pivot: Some(Vec2::new(0.5, 0.75)),
                ..default()
            }
            .bundle_with_atlas(&mut sprite_params, texture_atlas),
            Billboard,
            AnimationTimer {
                current_animation: "death".to_string(),
                library: animation.library.clone(),
                timer: Timer::from_seconds(0.2, TimerMode::Repeating),
                ..default()
            },
            location.clone(),
            transform.clone(),
        ));
    }
}
