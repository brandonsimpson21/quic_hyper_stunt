use std::{net::SocketAddr, sync::Arc};

use bytes::Bytes;
use quinn::Endpoint;
use rustls::RootCertStore;

use super::error::NetworkError;

pub fn default_client_crypto(roots: RootCertStore) -> Arc<rustls::ClientConfig> {
    let mut client_crypto = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(roots)
        .with_no_client_auth();

    client_crypto.alpn_protocols = super::ALPN_QUIC_HTTP.iter().map(|&x| x.into()).collect();
    client_crypto.key_log = Arc::new(rustls::KeyLogFile::new());
    Arc::new(client_crypto)
}

pub fn default_endpoint(roots: RootCertStore) -> Result<Endpoint, NetworkError> {
    let client_crypto = default_client_crypto(roots);
    let client_config = quinn::ClientConfig::new(client_crypto);
    let mut endpoint = quinn::Endpoint::client("[::]:0".parse().unwrap())?;
    endpoint.set_default_client_config(client_config);

    Ok(endpoint)
}
