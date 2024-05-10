use std::time::SystemTime;

use bevy_renet::RenetServerPlugin;
use catacombat_rs::network::server::server::server_listen_event;
use catacombat_rs::transport::ServerConfig;
use catacombat_rs::*;
use renet::transport::NetcodeServerTransport;

fn main() {
    let port = get_connection_port();

    println!("Server port: {}", port);

    let mut app = App::new();

    app.add_plugins(MinimalPlugins);
    app.add_plugins(LogPlugin::default());

    app.add_plugins(RenetServerPlugin);

    let server = RenetServer::new(ConnectionConfig::default());
    app.insert_resource(server);

    // Transport layer setup
    app.add_plugins(NetcodeServerPlugin);
    let server_addr = ("0.0.0.0:".to_owned() + port.to_string().as_str())
        .parse()
        .unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let server_config = ServerConfig {
        current_time: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap(),
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        public_addresses: vec![server_addr],
        authentication: renet::transport::ServerAuthentication::Unsecure,
    };
    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
    app.insert_resource(transport);
    app.add_systems(Update, server_listen_event);
    app.add_systems(Startup, setup_walkers).add_systems(
        Update,
        (
            walk_walker_generators,
            destroy_walker_generators,
            check_walkers,
        ),
    );
    app.run();
}
