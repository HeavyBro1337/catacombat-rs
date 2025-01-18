use bevy::prelude::*;

use crate::{characters::player::player::Player, combat::combat::DamagedEvent};

#[derive(Component)]
pub struct ScreenTint(f32);

pub fn damage_screen(
    mut commands: Commands,
    mut ev_damaged: EventReader<DamagedEvent>,
    q_players: Query<&Player>,
) {
    for damaged in ev_damaged.read() {
        if !q_players.get(damaged.0).is_ok() {
            return;
        }
        info!("Damaged player!");
        commands.spawn((
            Node {
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(Color::LinearRgba(LinearRgba::RED.with_alpha(0.2))),
            ScreenTint(0.1),
        ));
    }
}

pub fn destroy_tints(
    mut commands: Commands,
    mut q_tints: Query<(&mut ScreenTint, Entity)>,
    time: Res<Time>,
) {
    for (mut tint, entity) in q_tints.iter_mut() {
        tint.0 -= time.delta_secs();
        if tint.0 <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
