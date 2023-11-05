use anyhow::Result;
use s2n_quic::{client::Connect, stream::BidirectionalStream};

use std::{net::SocketAddr, path::Path};

pub fn get_client(cert_pem_path: &Path) -> Result<s2n_quic::Client> {
    let client = s2n_quic::Client::builder()
        .with_tls(cert_pem_path)?
        .with_io("0.0.0.0:0")?
        .start()?;
    Ok(client)
}
pub async fn client_connect(
    addr: SocketAddr,
    server_name: &str,
    cert_pem_path: &Path,
    keep_alive: bool,
) -> Result<(s2n_quic::Client, BidirectionalStream)> {
    let client = get_client(cert_pem_path)?;

    let connect = Connect::new(addr).with_server_name(server_name);
    let mut connection = client.connect(connect).await?;

    // ensure the connection doesn't time out with inactivity
    connection.keep_alive(keep_alive)?;

    // open a new stream
    let stream = connection.open_bidirectional_stream().await?;

    Ok::<_, anyhow::Error>((client, stream))
}
