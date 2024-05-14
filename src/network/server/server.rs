use bevy::{
    prelude::*,
    utils::hashbrown::{HashMap, HashSet},
};
use renet::{ClientId, RenetServer, ServerEvent};

use crate::{
    ClientChannel, OtherPlayer, PlayerLocation, PlayerLocationNetwork, ServerChannel,
    ServerMessages, WorldCatacomb,
};

pub fn server_listen_event(
    mut server_events: EventReader<ServerEvent>,
    mut server: ResMut<RenetServer>,
    mut commands: Commands,
    location: Res<WorldCatacomb>,
    q_player_entities: Query<(Entity, &OtherPlayer)>,
) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Client with id {} connected", client_id);
                println!("Sending world info...");
                let channel_id = ServerChannel::GenerationMessage;
                let player_ids = q_player_entities
                    .iter()
                    .map(|(_, id)| id.0)
                    .collect::<Vec<_>>();
                let array_locs = location
                    .0
                    .iter()
                    .map(|x| x.to_array())
                    .collect::<HashSet<[i32; 2]>>();
                let message = bincode::serialize(&(array_locs, player_ids)).unwrap();
                server.send_message(*client_id, channel_id, message);

                commands.spawn((OtherPlayer(client_id.raw()), PlayerLocation::new()));

                let player_connected = ServerMessages::PlayerConnected(client_id.raw());
                let server_event_message = bincode::serialize(&player_connected).unwrap();
                server.broadcast_message_except(
                    *client_id,
                    ServerChannel::ServerMessages,
                    server_event_message,
                )
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Client with id {} disconnected: {}", client_id, reason);
                for (entity, id) in q_player_entities.iter() {
                    if id.0 == client_id.raw() {
                        commands.entity(entity).despawn();
                        break;
                    }
                }

                let player_disconnected = ServerMessages::PlayerDisconnected(client_id.raw());
                let server_event_message = bincode::serialize(&player_disconnected).unwrap();
                server.broadcast_message_except(
                    *client_id,
                    ServerChannel::ServerMessages,
                    server_event_message,
                );
            }
        }
    }
}

pub fn server_sync_positions(
    mut server: ResMut<RenetServer>,
    query: Query<(&PlayerLocation, &OtherPlayer)>,
) {
    let mut players: HashMap<u64, PlayerLocationNetwork> = HashMap::new();

    for (loc, id) in query.iter() {
        players.insert(id.0, loc.as_remote());
    }
    let message = bincode::serialize(&players).unwrap();
    server.broadcast_message(ServerChannel::SyncPositions, message);
}

pub fn server_receive_position(
    mut server: ResMut<RenetServer>,
    mut query: Query<(&mut PlayerLocation, &OtherPlayer)>,
) {
    for (mut loc, id) in query.iter_mut() {
        while let Some(bytes) =
            server.receive_message(ClientId::from_raw(id.0), ClientChannel::SyncPositions)
        {
            let remote_loc = bincode::deserialize(&bytes).unwrap();
            loc.sync(&remote_loc);
        }
    }
}
