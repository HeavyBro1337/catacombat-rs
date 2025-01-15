use std::net::UdpSocket;

use std::time::SystemTime;

use bevy::ecs::schedule::SystemSet;
use bevy::prelude::*;
use bevy::utils::hashbrown::{HashMap, HashSet};

use bevy_sprite3d::Sprite3dParams;
use renet::RenetClient;

use crate::{
    connection_config, ClientChannel, GameState, OtherPlayer, PlayerBundle, PlayerLocation,
    PlayerLocationNetwork, ServerChannel, ServerMessages, WorldCatacomb,
};

pub const PROTOCOL_ID: u64 = 1337;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Connected;

#[derive(Debug, Resource)]
struct CurrentClientId(u64);

fn new_renet_client(addr: &String) -> (RenetClient, NetcodeClientTransport) {
    let server_addr = addr.parse().unwrap();
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
    let client = RenetClient::new(connection_config());

    (client, transport)
}

pub fn init_client(commands: &mut Commands, addr: &String) {
    let (client, transport) = new_renet_client(addr);
    commands.insert_resource(client);
    commands.insert_resource(transport);
}

pub fn sync_world_catacomb_from_server(
    mut sprite_params: Sprite3dParams,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    mut location: ResMut<WorldCatacomb>,
    mut state: ResMut<NextState<GameState>>,
) {
    let image: &Handle<Image> = &asset_server.load("sprites/doomguy.png");
    while let Some(message) = client.receive_message(ServerChannel::GenerationMessage) {
        let world_info: (HashSet<[i32; 2]>, Vec<u64>) = bincode::deserialize(&message).unwrap();
        location.0 = world_info.0.iter().map(|x| IVec2::from_array(*x)).collect();
        state.set(GameState::Game);

        for id in world_info.1.iter() {
            commands.spawn(PlayerBundle::new(image, &mut sprite_params, *id));
        }
    }
}

pub fn client_listen_event(
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    mut sprite_params: Sprite3dParams,
    asset_server: Res<AssetServer>,
    q_player_entities: Query<(Entity, &OtherPlayer)>,
) {
    let image: &Handle<Image> = &asset_server.load("sprites/doomguy.png");
    while let Some(message) = client.receive_message(ServerChannel::ServerMessages) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessages::PlayerConnected(id) => {
                commands.spawn(PlayerBundle::new(image, &mut sprite_params, id));
            }
            ServerMessages::PlayerDisconnected(id) => {
                println!("Client with id {} disconnected :DDD", id);

                for (entity, other_id) in q_player_entities.iter() {
                    if other_id.0 == id {
                        commands.entity(entity).despawn();
                        break;
                    }
                }
            }
        }
    }
}

pub fn sync_other_player_positions(
    mut client: ResMut<RenetClient>,
    mut q_other_players: Query<(&mut PlayerLocation, &OtherPlayer)>,
) {
    while let Some(message) = client.receive_message(ServerChannel::SyncPositions) {
        let player_map: HashMap<u64, PlayerLocationNetwork> =
            bincode::deserialize(&message).unwrap();

        for (mut player_loc, id) in q_other_players.iter_mut() {
            let Some(remote) = player_map.get(&id.0) else {
                continue;
            };

            player_loc.sync(&remote);
        }
    }

    for (mut loc, id) in q_other_players.iter_mut() {
        while let Some(message) = client.receive_message(ServerChannel::SyncPositions) {
            let (remote_loc, remote_id): (PlayerLocationNetwork, u64) =
                bincode::deserialize(&message).unwrap();

            if id.0 == remote_id {
                loc.sync(&remote_loc);
            }
        }
    }
}

pub fn sync_own_player_position(
    mut client: ResMut<RenetClient>,
    q_player: Query<&PlayerLocation, Without<OtherPlayer>>,
) {
    let loc = q_player.single();
    let loc_network = loc.as_remote();
    let message = bincode::serialize(&loc_network).unwrap();

    client.send_message(ClientChannel::SyncPositions, message)
}
