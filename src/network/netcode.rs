use bevy::prelude::*;
use bevy::{app::App, ecs::schedule::SystemSet};
use bevy_renet::client_connected;
use renet::RenetClient;

use crate::network::config::connection_config;

pub const PROTOCOL_ID: u64 = 1337;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Connected;

#[derive(Debug, Resource)]
struct CurrentClientId(u64);

fn add_netcode_network(app: &mut App, port: Option<u16>) {
    use bevy_renet::renet::transport::{ClientAuthentication, NetcodeClientTransport};
    use std::{net::UdpSocket, time::SystemTime};

    app.add_plugins(bevy_renet::transport::NetcodeClientPlugin);

    app.configure_sets(Update, Connected.run_if(client_connected));

    let client = RenetClient::new(connection_config());

    let server_addr = "127.0.0.1:5000".parse().unwrap();
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

    app.insert_resource(client);
    app.insert_resource(transport);
    app.insert_resource(CurrentClientId(client_id));
}
