use std::time::Duration;

use super::{
    behavior::{RelayBehaviour, RendezvousBehaviour},
    NetworkError,
};
use libp2p::{identity, noise, swarm::NetworkBehaviour, tcp, yamux, Swarm};

pub fn build_swarm<B, BF, CF>(
    local_key: identity::Keypair,
    behavior_constructor: BF,
    swarm_config_constructor: CF,
) -> Result<Swarm<B>, NetworkError>
where
    B: NetworkBehaviour,
    BF: FnOnce(&identity::Keypair) -> B,
    CF: FnOnce(libp2p_swarm::Config) -> libp2p_swarm::Config,
{
    let swarm = libp2p::SwarmBuilder::with_existing_identity(local_key)
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_quic()
        .with_behaviour(|key| behavior_constructor(key))?
        .with_swarm_config(|cfg| swarm_config_constructor(cfg))
        .build();
    Ok(swarm)
}
pub fn get_swarm_relay_default(
    local_key: identity::Keypair,
    protocol_version: &str,
) -> Result<Swarm<RelayBehaviour>, NetworkError> {
    Ok(build_swarm(
        local_key,
        RelayBehaviour::get_constructor(protocol_version),
         |cfg: libp2p_swarm::Config| cfg,
    )?)
}

pub fn get_swarm_rendezvous_default(
    local_key: identity::Keypair,
    protocol_version: &str,
) -> Result<Swarm<RendezvousBehaviour>, NetworkError> {
    let swarm_config_constructor =
        |cfg: libp2p_swarm::Config| cfg.with_idle_connection_timeout(Duration::from_secs(5));
    let swarm = build_swarm(
        local_key,
        RendezvousBehaviour::get_constructor(protocol_version),
        swarm_config_constructor,
    )?;
    Ok(swarm)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_relay_default_swarm() -> Result<(), NetworkError> {
        let local_key = identity::Keypair::generate_ed25519();
        let swarm = get_swarm_relay_default(local_key, "relay/1.0.0");
        assert!(swarm.is_ok());
        Ok(())
    }

    #[test]
    fn test_rendezvous_default_swarm() -> Result<(), NetworkError> {
        let local_key = identity::Keypair::generate_ed25519();
        let swarm = get_swarm_rendezvous_default(local_key, "rendezvous/1.0.0");
        assert!(swarm.is_ok());
        Ok(())
    }
}
