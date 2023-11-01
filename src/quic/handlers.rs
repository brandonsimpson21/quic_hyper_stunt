use std::sync::Arc;

use quinn::{Connecting, Endpoint, RecvStream, SendStream};
use tracing::{info, instrument};

use super::error::NetworkError;

pub async fn run_server<F>(endpoint: Endpoint, handler: Arc<F>) -> Result<(), NetworkError>
where
    F: Fn(RecvStream, SendStream) -> Result<(), NetworkError> + Send + Sync + 'static,
{
    while let Some(conn) = endpoint.accept().await {
        let mut handler = handler.clone();
        tokio::spawn(async move {
            let connection = conn.await?;
            let (send, recv) = handle_bidirectional_connection(connection).await?;
            handler(recv, send)
        });
    }

    Ok(())
}

pub async fn handle_accept(endpoint: Endpoint) -> Result<(SendStream, RecvStream), NetworkError> {
    if let Some(conn) = endpoint.accept().await {
        let conn = conn.await?;
        handle_bidirectional_connection(conn).await
    } else {
        return Err(NetworkError::InternalError("no connection".to_string()));
    }
}

#[instrument(name = "handle connection")]
async fn handle_bidirectional_connection(
    conn: quinn::Connection,
) -> Result<(SendStream, RecvStream), NetworkError> {
    info!("established {:?}", conn);
    let stream = match conn.accept_bi().await {
        Err(quinn::ConnectionError::ApplicationClosed { .. }) => {
            let msg = format!("connection to {:?} closed by peer", conn.remote_address());
            return Err(NetworkError::ConnectionClosedError(msg));
        }
        Err(e) => {
            return Err(e.into());
        }
        Ok(s) => s,
    };
    Ok(stream)
}
