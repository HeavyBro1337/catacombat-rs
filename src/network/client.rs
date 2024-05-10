use std::net::UdpSocket;
use std::time::SystemTime;

use bevy::prelude::*;
use bevy::{app::App, ecs::schedule::SystemSet};
use bevy_renet::client_connected;
use renet::transport::{ClientAuthentication, NetcodeClientTransport};
use renet::{ConnectionConfig, DefaultChannel, RenetClient};

use crate::network::config::connection_config;
use crate::WorldCatacomb;

pub const PROTOCOL_ID: u64 = 1337;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Connected;

#[derive(Debug, Resource)]
struct CurrentClientId(u64);

fn new_renet_client(addr: &String) -> (RenetClient, NetcodeClientTransport) {
    let server_addr = addr.parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
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
    let client = RenetClient::new(ConnectionConfig::default());

    (client, transport)
}

pub fn init_client(commands: &mut Commands, addr: &String) {
    let (client, transport) = new_renet_client(addr);
    commands.insert_resource(client);
    commands.insert_resource(transport);
}

pub fn sync_world_catacomb(mut client: ResMut<RenetClient>, mut location: ResMut<WorldCatacomb>) {
    while let Some(bytes) = client.receive_message(DefaultChannel::ReliableOrdered) {}
}
