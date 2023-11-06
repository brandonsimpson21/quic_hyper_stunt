use anyhow::Result;
use s2n_quic::{stream::BidirectionalStream, Connection};

use std::{net::SocketAddr, path::Path, sync::Arc};

use futures::Future;

pub fn get_server(cert_path: &Path, key_path: &Path, addr: SocketAddr) -> Result<s2n_quic::Server> {
    let server = s2n_quic::Server::builder()
        .with_tls((cert_path, key_path))?
        .with_io(addr)?
        .start()?;
    Ok(server)
}

pub async fn run_server<F, Fut>(
    cert_path: &Path,
    key_path: &Path,
    addr: SocketAddr,
    handler: F,
) -> Result<()>
where
    F: Fn(Connection) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<()>> + Send + 'static,
{
    let handler = Arc::new(handler);
    tokio::pin!(handler);

    let mut server = get_server(cert_path, key_path, addr)?;
    while let Some(connection) = server.accept().await {
        let handler = handler.clone();
        tokio::spawn(async move {
            if let Err(e) = handler(connection).await {
                let msg = format!("connection task failed {:?}", e);
                tracing::error!("{}\n{:?}", msg, e);
            }
        });
    }
    Ok(())
}

pub async fn run_bidirectional_server<F, Fut>(
    cert_path: &Path,
    key_path: &Path,
    addr: SocketAddr,
    handler: F,
) -> Result<()>
where
    F: Fn(BidirectionalStream) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<()>> + Send + 'static,
{
    let handler = Arc::new(handler);
    tokio::pin!(handler);

    let mut server = get_server(cert_path, key_path, addr)?;
    while let Some(mut connection) = server.accept().await {
        let handler = handler.clone();
        // spawn a new task for the connection
        tokio::spawn(async move {
            while let Ok(Some(stream)) = connection.accept_bidirectional_stream().await {
                let handler = handler(stream);
                // spawn a new task for the stream
                let _ = tokio::spawn(async move {
                    if let Err(e) = handler.await {
                        let msg = format!("stream task failed {:?}", e);
                        tracing::error!("{}", msg);
                    }
                });
            }
        });
    }
    Ok(())
}
