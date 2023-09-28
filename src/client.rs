use hyper_rustls::HttpsConnector;
use rustls::ClientConfig;

use crate::utils;

fn get_root_store() -> rustls::RootCertStore {
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

fn get_random_tls_config(nciphers: usize) -> ClientConfig {
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

    client_config
        .unwrap()
        .with_root_certificates(root_certs)
        .with_no_client_auth()
}

/// get a random tls connector
/// # Eg
/// ```Rust
///     let config = get_random_https_connector(5);
/// ```
pub fn get_random_https_connector() -> HttpsConnector<hyper::client::HttpConnector> {
    let nciphers = utils::get_random_int(3, 6);
    let config = get_random_tls_config(nciphers);

    hyper_rustls::HttpsConnectorBuilder::new()
        .with_tls_config(config)
        .https_or_http()
        .enable_http1()
        .enable_http2()
        .build()
}
/// get random hyper client
/// # Eg
/// ```Rust
///    let client = get_random_hyper_client(5).await;
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
