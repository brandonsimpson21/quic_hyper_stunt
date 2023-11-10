pub mod behavior;
pub mod transport;
pub mod utils;
use crate::error::NetworkError;



/// P2P
/// ```
/// use quic_hyper_stunt::p2p::{
///     swarm::build_swarm,
///     behavior::RelayBehaviour,
/// };
/// use quic_hyper_stunt::error::NetworkError;
/// fn main()->Result<(),NetworkError> {
///     let local_key = libp2p::identity::Keypair::generate_ed25519();
///     let behavior_constructor = RelayBehaviour::get_constructor("relay/1.0.0");
///     let swarm_config_constructor = |cfg: libp2p_swarm::Config| cfg;
///     let swarm = build_swarm(local_key, behavior_constructor, swarm_config_constructor);
///     assert!(swarm.is_ok());
///     Ok(())
///  }
/// 
/// ```
/// 
pub mod swarm;
