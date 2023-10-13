use hyper_rustls::HttpsConnector;
use rustls::ClientConfig;
use tokio::net::TcpStream;

use crate::utils;

pub fn get_root_store() -> rustls::RootCertStore {
    let mut root_store = rustls::RootCertStore::empty();
    root_store.add_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.iter().map(|ta| {
        rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject,
            ta.spki,
            ta.name_constraints,
        )
    }));
    root_store
}

/// get a random tls config
/// the returned config has randomized supported ciphers, kx_groups and protocol versions
/// and respects the SSLKEYLOGFILE env var
/// /// # Eg
/// ```
/// use hyper_stunt::client::get_random_tls_config;
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = get_random_tls_config(5);
///     Ok(())
/// }
/// ```
pub fn get_random_tls_config(nciphers: usize) -> ClientConfig {
    let root_certs = get_root_store();
    let mut ciphers = utils::get_random_ciphersuites(nciphers);
    let kx_groups = utils::get_random_kx_group();
    let versions = utils::get_random_protocols();
    let mut client_config = rustls::ClientConfig::builder()
        .with_cipher_suites(&ciphers)
        .with_kx_groups(&kx_groups)
        .with_protocol_versions(&versions);

    // occassionally unsupported ciphers are generated
    while client_config.is_err() {
        ciphers = utils::get_random_ciphersuites(nciphers);
        client_config = rustls::ClientConfig::builder()
            .with_cipher_suites(&ciphers)
            .with_kx_groups(&kx_groups)
            .with_protocol_versions(&versions);
    }

    let mut client_config = client_config
        .expect("unreachable after loop")
        .with_root_certificates(root_certs)
        .with_no_client_auth();

    client_config.key_log = std::sync::Arc::new(rustls::KeyLogFile::new());
    client_config
}

/// get a random tls connector
/// # Eg
/// ```
/// use hyper_stunt::client::get_random_https_connector;
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let connector = get_random_https_connector();
///     Ok(())
/// }
/// ```
pub fn get_random_https_connector() -> HttpsConnector<hyper::client::HttpConnector> {
    let nciphers = utils::get_random_int(3, rustls::ALL_CIPHER_SUITES.len());
    let config = get_random_tls_config(nciphers);

    hyper_rustls::HttpsConnectorBuilder::new()
        .with_tls_config(config)
        .https_or_http()
        .enable_http1()
        .enable_http2()
        .build()
}

/// get a random tls connector
/// /// Eg
/// ```
/// use hyper_stunt::client::{get_random_tls_stream, get_random_tls_connector};
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let connector = get_random_tls_connector();
///     Ok(())
/// }
/// ```
pub fn get_random_tls_connector() -> tokio_rustls::TlsConnector {
    let nciphers = utils::get_random_int(3, rustls::ALL_CIPHER_SUITES.len());
    let config = get_random_tls_config(nciphers);
    tokio_rustls::TlsConnector::from(std::sync::Arc::new(config))
}

/// get a random tls stream
/// Eg
/// ```
/// use hyper_stunt::client::{get_random_tls_stream, get_random_tls_connector};
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let addr = "reddit.com";
///     let port = 443u16;
///     let mut stream = get_random_tls_stream(addr, port).await;
///     assert!(stream.is_ok());
///     Ok(())
/// }
/// ```
pub async fn get_random_tls_stream(
    addr: &str,
    port: u16,
) -> Result<tokio_rustls::client::TlsStream<tokio::net::TcpStream>, Box<dyn std::error::Error>> {
    let tls_connector = get_random_tls_connector();
    let ip_addr = format!("{}:{}", addr, port);
    let stream = TcpStream::connect(ip_addr).await?;
    let server_name = rustls::ServerName::try_from(addr)?;
    let stream = tls_connector.connect(server_name, stream).await?;
    Ok(stream)
}

/// get random hyper client
/// Eg
/// ```
/// use hyper_stunt::client::get_random_hyper_client;
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = get_random_hyper_client().await;
///     Ok(())
/// }
/// ```
pub async fn get_random_hyper_client() -> hyper::Client<HttpsConnector<hyper::client::HttpConnector>>
{
    let connector = get_random_https_connector();
    let pool_timeout =
        std::time::Duration::from_secs(utils::get_random_int(60 * 5, 60 * 60) as u64);

    let mut client = hyper::Client::builder();

    client.pool_idle_timeout(pool_timeout);

    let http1_max_buf_size = utils::get_random_int(400 * 1024, 1024 * 1024);
    client.http1_max_buf_size(http1_max_buf_size);
    client.http1_read_buf_exact_size(http1_max_buf_size);

    client.build::<_, hyper::Body>(connector)
}

#[cfg(test)]
mod test_client {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        for _ in 1..100 {
            let _ = get_random_hyper_client().await;
        }
    }
}
