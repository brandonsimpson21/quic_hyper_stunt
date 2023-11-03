pub mod client;
pub mod headers;
pub mod quic;
mod utils;

#[cfg(test)]
mod test {
    use std::{net::ToSocketAddrs, time::Duration};

    use crate::quic::{common::read_recv_stream, error::NetworkError, handlers::handle_accept};

    use super::*;

    #[tokio::test]
    async fn test_quic_client_server() -> Result<(), NetworkError> {
        let server_addr = "127.0.0.1:4000".to_socket_addrs()?.next().unwrap();
        let (client, server) = quic::get_self_signed_client_server(server_addr)?;
        let server_handle = tokio::spawn(async move {
            let (mut send, recv) = handle_accept(server).await?;
            let buf = read_recv_stream(recv, None).await?;
            assert_eq!(buf, b"hello".to_vec());
            send.write_all(&*b"world".to_vec()).await?;
            send.finish().await?;
            Ok::<_, NetworkError>(())
        });
        let client_conn = client
            .connect(server_addr, "127.0.0.1")
            .map_err(|e| NetworkError::InternalError(e.to_string()))?
            .await?;
        let (mut client_send, client_recv) = client_conn.open_bi().await?;
        client_send.write_all(&*b"hello".to_vec()).await?;
        client_send.finish().await?;
        let buf = read_recv_stream(client_recv, None).await?;

        client.close(0u32.into(), b"done");
        assert!(buf == b"world".to_vec());
        let _ = tokio::time::timeout(Duration::from_secs(60), client.wait_idle()).await;

        if let Err(e) = server_handle.await {
            return Err(NetworkError::InternalError(e.to_string()));
        }

        Ok(())
    }
}
