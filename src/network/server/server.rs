use bevy::{
    prelude::*,
    utils::hashbrown::{HashMap, HashSet},
};
use renet::{Bytes, DefaultChannel, RenetServer, ServerEvent};

use crate::WorldCatacomb;

pub fn server_listen_event(
    mut server_events: EventReader<ServerEvent>,
    mut server: ResMut<RenetServer>,
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
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Client with id {} disconnected: {}", client_id, reason)
            }
        }
    }
}
