#![allow(unused)]

pub mod client;
pub mod common;
pub mod error;
pub mod handlers;
pub mod server;

use std::net::{SocketAddr, ToSocketAddrs as _};

use quinn::Endpoint;
use rustls::{Certificate, PrivateKey};

use self::error::NetworkError;

pub(crate) const ALPN_QUIC_HTTP: &[&[u8]] = &[b"hq-29"];

/// sent by h2 clients after negotiating over ALPN, or when doing h2c.
pub(crate) const PREFACE: &[u8] = b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n";

pub(crate) const CRLF: &str = "\r\n";

/// Get a self signed certificate for server_addr
pub fn get_self_signed_client_server(
    server_addr: SocketAddr,
) -> Result<(Endpoint, Endpoint), NetworkError> {
    let (cert, key) = common::generate_self_signed(vec!["127.0.0.1".to_string()])
        .expect("could not generate self signed cert");
    let server = server::default_endpoint(server_addr, cert.clone(), key)?;
    let mut roots = rustls::RootCertStore::empty();
    roots.add(&cert)?;
    let client = client::default_endpoint(roots)?;

    Ok((client, server))
}
