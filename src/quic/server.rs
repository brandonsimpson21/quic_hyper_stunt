use std::{net::SocketAddr, sync::Arc};

use quinn::{Endpoint, ServerConfig};
use rustls::{Certificate, PrivateKey};

use super::error::NetworkError;

/// Constructs a QUIC endpoint configured to listen for incoming connections on a certain address
/// and port.
pub fn default_endpoint(
    listen_addr: SocketAddr,
    cert: Certificate,
    pk: PrivateKey,
) -> Result<Endpoint, NetworkError> {
    let server_config = default_config(cert, pk)?;
    let endpoint = quinn::Endpoint::server(server_config, listen_addr)?;
    Ok(endpoint)
}

pub fn default_crypto(
    cert: Certificate,
    pk: PrivateKey,
) -> Result<rustls::ServerConfig, rustls::Error> {
    let mut server_crypto = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(vec![cert], pk)?;
    server_crypto.alpn_protocols = super::ALPN_QUIC_HTTP.iter().map(|&x| x.into()).collect();
    server_crypto.key_log = Arc::new(rustls::KeyLogFile::new());
    Ok(server_crypto)
}
/// Returns default server configuration.
pub fn default_config(cert: Certificate, pk: PrivateKey) -> Result<ServerConfig, rustls::Error> {
    let server_crypto = default_crypto(cert, pk)?;
    let mut server_config = quinn::ServerConfig::with_crypto(Arc::new(server_crypto));
    let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
    transport_config.max_concurrent_uni_streams(0_u8.into());
    server_config.use_retry(true);
    Ok(server_config)
}
