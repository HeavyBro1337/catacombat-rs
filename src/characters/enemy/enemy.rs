use super::path::Path;
use crate::{
    audio::random::{get_audio_handles, RandomAudio},
    characters::player::player::Player,
    combat::{
        combat::{Combat, CombatReplica, CombatState, Health},
        death::DeathSound,
        pain::PainSound,
    },
    tick::tick::TickEvent,
    visuals::{
        animation::{AnimationInfo, AnimationTimer, Animations},
        billboard::Billboard,
    },
    WorldCatacomb, WorldLocation,
};
use bevy::prelude::*;
use bevy_sprite3d::{Sprite3dBuilder, Sprite3dParams};
use rand::seq::SliceRandom;

#[derive(Component)]
#[require(WorldLocation, Path, Health, Combat)]
pub struct Enemy;

pub fn setup_enemy_atlas(
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut animations: ResMut<Animations>,
) {
    let layout = texture_atlases.add(TextureAtlasLayout::from_grid(
        UVec2::new(161, 129),
        8,
        4,
        None,
        None,
    ));

    animations.new_animation(
        "Cultist".to_string(),
        "walk".to_string(),
        AnimationInfo {
            len: 6,
            row: 0,
            looped: true,
        },
        layout.clone(),
        8,
    );

    animations.new_animation(
        "Cultist".to_string(),
        "attack".to_string(),
        AnimationInfo {
            len: 2,
            row: 1,
            looped: true,
        },
        layout.clone(),
        8,
    );

    animations.new_animation(
        "Cultist".to_string(),
        "pain".to_string(),
        AnimationInfo {
            len: 2,
            row: 2,
            looped: true,
        },
        layout.clone(),
        8,
    );

    animations.new_animation(
        "Cultist".to_string(),
        "death".to_string(),
        AnimationInfo {
            len: 8,
            row: 3,
            looped: false,
        },
        layout.clone(),
        8,
    );
}

pub fn setup_enemies(
    world: Res<WorldCatacomb>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut sprite_params: Sprite3dParams,
    animations: Res<Animations>,
) {
    let dirs = vec![IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y];
    for room in world
        .iter()
        .collect::<Vec<_>>()
        .choose_multiple(&mut rand::thread_rng(), 4)
    {
        let room = **room;
        let face = dirs.choose(&mut rand::thread_rng()).unwrap();

        let (_, layout, _) = animations.atlases.get(&"Cultist".to_string()).unwrap();

        let texture_atlas = TextureAtlas {
            index: 0,
            layout: layout.clone(),
        };
        let mut replicas = Vec::<Handle<AudioSource>>::default();

        for n in 1..=3 {
            replicas.push(
                asset_server
                    .load::<AudioSource>(format!("sounds/characters/cultist/replica_{n}.wav")),
            );
        }

        commands.spawn((
            Enemy,
            CombatReplica(RandomAudio::new(replicas)),
            AnimationTimer {
                timer: Timer::from_seconds(0.3, TimerMode::Repeating),
                library: "Cultist".to_string(),
                current_animation: "walk".to_string(),
                current_frame: 0,
                ..default()
            },
            DeathSound(RandomAudio::new(get_audio_handles(
                &asset_server,
                vec![
                    "sounds/characters/cultist/die_1.wav".into(),
                    "sounds/characters/cultist/die_2.wav".into(),
                ],
            ))),
            PainSound(RandomAudio::new(get_audio_handles(
                &asset_server,
                vec![
                    "sounds/characters/cultist/pain_1.wav".into(),
                    "sounds/characters/cultist/pain_2.wav".into(),
                    "sounds/characters/cultist/pain_3.wav".into(),
                ],
            ))),
            Billboard,
            WorldLocation::new(room, *face),
            Sprite3dBuilder {
                image: asset_server.load("sprites/cultist.png"),
                pixels_per_metre: 64.0,
                pivot: Some(Vec2::new(0.5, 0.75)),
                unlit: true,
                ..default()
            }
            .bundle_with_atlas(&mut sprite_params, texture_atlas),
        ));
    }
}

pub fn enemies_find_player(
    mut q_enemies: Query<(&WorldLocation, &mut Path), With<Enemy>>,
    mut ev_tick: EventReader<TickEvent>,
    q_player: Query<&WorldLocation, With<Player>>,
    world: Res<WorldCatacomb>,
) {
    let player_location = q_player.single();
    for _ in ev_tick.read() {
        for (location, mut path) in q_enemies.iter_mut() {
            if path.has_path() {
                continue;
            }
            path.find_path(&location, &world, player_location.get_location());
        }
    }
}

pub fn move_enemies(
    mut q_enemies: Query<(&mut WorldLocation, &mut Path), With<Enemy>>,
    mut ev_tick: EventReader<TickEvent>,
    combat_state: Res<CombatState>,
    world: Res<WorldCatacomb>,
) {
    if combat_state.opponent.is_some() {
        return;
    }

    for _ in ev_tick.read() {
        for (mut location, mut path) in q_enemies.iter_mut() {
            path.move_location(&mut location, &world);
        }
    }
}
