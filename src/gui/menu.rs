use bevy::prelude::DespawnRecursiveExt;
use bevy::{
    app::{Plugin, Startup, Update},
    asset::{AssetServer, Handle},
    ecs::{
        entity::Entity,
        query::{Changed, With},
        schedule::{common_conditions::in_state, IntoSystemConfigs, NextState, OnExit},
        system::{Commands, Query, Res, ResMut},
    },
    hierarchy::BuildChildren,
    render::color::Color,
    text::{Font, Text, TextStyle},
    ui::{
        node_bundles::{ButtonBundle, NodeBundle, TextBundle},
        widget::Button,
        BackgroundColor, Interaction, Node, Style, Val,
    },
    utils::default,
};

use crate::GameState;
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup_menu);
        app.add_systems(OnExit(GameState::Menu), despawn_menu);
        app.add_systems(Update, button_system.run_if(in_state(GameState::Menu)));
    }
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/PublicPixel.ttf");

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: bevy::ui::AlignItems::Center,
                flex_direction: bevy::ui::FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|commands| {
            commands.spawn(TextBundle {
                text: Text::from_section("Main Menu", TextStyle { font, ..default() }),
                ..default()
            });
            commands.spawn(ButtonBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(10.0),
                    ..default()
                },
                background_color: Color::BLUE.into(),
                button: Button,
                ..default()
            });
        });
}

fn despawn_menu(mut commands: Commands, button_query: Query<Entity, With<Node>>) {
    for ent in button_query.iter() {
        commands.entity(ent).despawn_recursive();
    }
}

fn button_system(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<NextState<GameState>>,
) {
    for interaction in &interaction_query {
        match *interaction {
            Interaction::Pressed => {
                state.set(GameState::Generating);
            }
            _ => (),
        }
    }
}
