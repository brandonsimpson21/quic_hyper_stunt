use anyhow::Result;
use std::{io::Cursor, path::Path};

use bytes::Bytes;
use rustls::{Certificate, PrivateKey};
use tracing::info;

pub fn generate_self_signed(
    subject_alt_names: Vec<String>,
    cert_path: Option<std::path::PathBuf>,
    key_path: Option<std::path::PathBuf>,
) -> Result<(Certificate, PrivateKey), rcgen::RcgenError> {
    tracing::info!("generating self-signed certificate");
    let cert = rcgen::generate_simple_self_signed(subject_alt_names)?;
    let key = cert.serialize_private_key_pem();
    let cert_bytes = cert.serialize_pem().expect("failed to serialize cert");
    if cert_path.is_some() {
        std::fs::write("cert.pem", &cert_bytes.as_bytes()).expect("failed to write cert");
    }
    if key_path.is_some() {
        std::fs::write("key.pem", &key).expect("failed to write key");
    }
    Ok((
        rustls::Certificate(cert.serialize_der()?),
        rustls::PrivateKey(cert.serialize_private_key_der()),
    ))
}

pub fn read_key(key_path: &Path) -> Result<rustls::PrivateKey> {
    let raw_bytes = std::fs::read(key_path)?;
    let mut cursor = Cursor::new(raw_bytes);

    for parser in [
        rustls_pemfile::rsa_private_keys,
        rustls_pemfile::pkcs8_private_keys,
        rustls_pemfile::ec_private_keys,
    ]
    .iter()
    {
        cursor.set_position(0);
        match parser(&mut cursor) {
            Ok(keys) => match keys.len() {
                0 => {
                    continue;
                }
                1 => {
                    let mut keys = keys;
                    return Ok(rustls::PrivateKey(keys.pop().unwrap()));
                }
                _ => return Err(anyhow::anyhow!("more than 1 key found")),
            },
            Err(_) => {
                continue;
            }
        }
    }
    Err(anyhow::anyhow!("no valid keys found"))
}

pub fn read_cert_chain(cert_pem_path: &Path) -> Result<Vec<Vec<u8>>> {
    let raw_bytes = std::fs::read(cert_pem_path)?;
    let mut cursor = Cursor::new(raw_bytes);
    rustls_pemfile::certs(&mut cursor)
        .map(|certs| certs.into_iter().collect())
        .map_err(|e| anyhow::anyhow!(e.to_string()))
}

#[allow(unused)]
pub async fn bytes_escape(req: Bytes) -> Bytes {
    let mut escaped = Vec::new();
    for &x in req.iter() {
        let part = std::ascii::escape_default(x).collect::<Vec<_>>();
        escaped.extend(part);
    }
    info!("content = {:?}", String::from_utf8_lossy(&escaped));
    Bytes::from(escaped)
}
