use libp2p::{identify, identity, ping, relay, swarm::NetworkBehaviour};

#[derive(NetworkBehaviour)]
pub struct RelayBehaviour {
    pub relay: relay::Behaviour,
    pub ping: ping::Behaviour,
    pub identify: identify::Behaviour,
}

impl RelayBehaviour {
    pub fn get_constructor(protocol_version: &str) -> impl FnOnce(&identity::Keypair) -> Self {
        let protocol_version = protocol_version.to_string();
        |key: &identity::Keypair| RelayBehaviour {
            relay: relay::Behaviour::new(key.public().to_peer_id(), Default::default()),
            ping: ping::Behaviour::new(ping::Config::new()),
            identify: identify::Behaviour::new(identify::Config::new(
                protocol_version,
                key.public(),
            )),
        }
    }
}

#[derive(NetworkBehaviour)]
pub struct RendezvousBehaviour {
    pub identify: identify::Behaviour,
    pub rendezvous: libp2p::rendezvous::server::Behaviour,
    pub ping: ping::Behaviour,
}

impl RendezvousBehaviour {
    pub fn get_constructor(protocol_version: &str) -> impl FnOnce(&identity::Keypair) -> Self {
        let protocol_version = protocol_version.to_string();
        |key: &identity::Keypair| RendezvousBehaviour {
            identify: identify::Behaviour::new(identify::Config::new(
                protocol_version,
                key.public(),
            )),
            rendezvous: libp2p::rendezvous::server::Behaviour::new(
                libp2p::rendezvous::server::Config::default(),
            ),
            ping: ping::Behaviour::new(
                ping::Config::new().with_interval(std::time::Duration::from_secs(10)),
            ),
        }
    }
}
