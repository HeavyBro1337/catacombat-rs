use bevy::{prelude::*, utils::hashbrown::{HashMap, HashSet}};
use renet::{ClientId, DefaultChannel, RenetServer, ServerEvent};

use crate::{OtherPlayer, PlayerLocation, PlayerLocationNetwork, ServerMessages, WorldCatacomb};

pub fn server_listen_event(
    mut server_events: EventReader<ServerEvent>,
    mut server: ResMut<RenetServer>,
    mut commands: Commands,
    location: Res<WorldCatacomb>,
) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Client with id {} connected", client_id);
                println!("Sending world info...");
                let channel_id = DefaultChannel::ReliableOrdered;
                let array_locs = location
                    .0
                    .iter()
                    .map(|x| x.to_array())
                    .collect::<HashSet<[i32; 2]>>();
                let message = bincode::serialize(&array_locs).unwrap();
                server.send_message(*client_id, channel_id, message);
                let server_event_message = bincode::serialize(&ServerMessages::PlayerConnected(client_id.raw())).unwrap();
                
                commands.spawn((OtherPlayer(client_id.raw()), PlayerLocation::new()));

                server.broadcast_message_except(
                    *client_id, 
                    DefaultChannel::ReliableOrdered, 
                    server_event_message)
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Client with id {} disconnected: {}", client_id, reason)
            }
        }
    }
}

fn server_sync_players(mut server: ResMut<RenetServer>, query: Query<(&PlayerLocation, &OtherPlayer)>) {
    let mut players: HashMap<u64, PlayerLocationNetwork> = HashMap::new();

    for (loc, id) in query.iter() {
        players.insert(id.0, loc.as_remote());
    }
    let message = bincode::serialize(&players).unwrap();
    server.broadcast_message(DefaultChannel::ReliableOrdered, message);
}