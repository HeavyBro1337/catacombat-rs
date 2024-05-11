use bevy::ecs::component::Component;

use bevy::ecs::schedule::OnEnter;
use bevy::prelude::DespawnRecursiveExt;
use bevy::ui::{BorderColor, UiRect};
use bevy::{
    app::{Plugin, Update},
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
        Interaction, Node, Style, Val,
    },
    utils::default,
};
use bevy_simple_text_input::{TextInputBundle, TextInputPlugin, TextInputSettings, TextInputValue};

use crate::{init_client, GameState, NetworkState};
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(TextInputPlugin);
        app.add_systems(OnEnter(GameState::Menu), setup_menu);
        app.add_systems(OnExit(GameState::Menu), despawn_menu);
        app.add_systems(
            Update,
            (singleplayer_button, connect_button).run_if(in_state(GameState::Menu)),
        );
    }
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/PublicPixel.ttf");
    let text_style = TextStyle {
        font_size: 40.,
        color: Color::WHITE,
        ..default()
    };
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
            commands.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(10.0),
                        ..default()
                    },
                    background_color: Color::BLUE.into(),
                    button: Button,
                    ..default()
                },
                SinglePlayerButton,
            ));

            commands.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(10.0),
                        ..default()
                    },
                    background_color: Color::GREEN.into(),
                    button: Button,
                    ..default()
                },
                ConnectButton,
            ));
            commands.spawn((
                InputIpAddress,
                NodeBundle {
                    style: Style {
                        width: Val::Percent(25.0),
                        border: UiRect::all(Val::Px(5.0)),
                        padding: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    border_color: BorderColor(Color::WHITE),
                    ..default()
                },
                TextInputBundle::default()
                    .with_text_style(text_style)
                    .with_value("127.0.0.1")
                    .with_settings(TextInputSettings {
                        retain_on_submit: true,
                        ..default()
                    }),
            ));
        });
}

fn despawn_menu(mut commands: Commands, q_nodes: Query<Entity, With<Node>>) {
    for ent in q_nodes.iter() {
        commands.entity(ent).despawn_recursive();
    }
}

fn singleplayer_button(
    q_button: Query<&Interaction, (Changed<Interaction>, With<SinglePlayerButton>)>,
    mut state: ResMut<NextState<GameState>>,
) {
    for interaction in &q_button {
        match *interaction {
            Interaction::Pressed => {
                state.set(GameState::Generating);
            }
            _ => (),
        }
    }
}

fn connect_button(
    q_button: Query<&Interaction, (Changed<Interaction>, With<ConnectButton>)>,
    q_ip_addr_input: Query<&TextInputValue, With<InputIpAddress>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut multiplayer_state: ResMut<NextState<NetworkState>>,
    mut commands: Commands,
) {
    let ip_addr = q_ip_addr_input.single();

    for interaction in &q_button {
        match *interaction {
            Interaction::Pressed => {
                game_state.set(GameState::Generating);
                multiplayer_state.set(NetworkState::Online);
                init_client(&mut commands, &ip_addr.0);
            }
            _ => (),
        }
    }
}
#[derive(Component)]
struct SinglePlayerButton;

#[derive(Component)]
struct InputIpAddress;

#[derive(Component)]
struct ConnectButton;
