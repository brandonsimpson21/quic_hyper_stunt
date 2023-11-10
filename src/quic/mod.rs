pub mod client;
pub mod common;
pub mod server;

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use bytes::Bytes;
    use s2n_quic::{stream::BidirectionalStream, Connection};
    use std::{net::SocketAddr, path::Path};

    async fn server_handle_request(stream: BidirectionalStream) -> Result<()> {
        let mut stream = stream;
        while let Ok(Some(data)) = stream.receive().await {
            stream.send(data).await.expect("stream should be open");
        }
        Ok(())
    }

    async fn server_handle_conn(conn: Connection) -> Result<()> {
        let mut conn = conn;
        let stream = conn
            .accept_bidirectional_stream()
            .await?
            .expect("stream should be open");
        server_handle_request(stream).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_client_server() -> anyhow::Result<()> {
        common::generate_self_signed(vec!["localhost".to_string()], None, None)?;
        let addr: SocketAddr = "127.0.0.1:4444".parse()?;

        tokio::spawn(async move {
            let _ = server::run_server(
                Path::new("cert.pem"),
                Path::new("key.pem"),
                addr,
                server_handle_conn,
            )
            .await;
        });

        let (_, stream) =
            client::client_connect_bidirectional(addr, "localhost", Path::new("cert.pem"), true)
                .await?;
        let (mut receive_stream, mut send_stream) = stream.split();
        let test_data = vec![
            "hello".to_string(),
            "world".to_string(),
            "foo".to_string(),
            "bar".to_string(),
        ];
        for data in test_data.iter().cloned() {
            let data = Bytes::from(data);
            send_stream.send(data.clone()).await?;
            let response = receive_stream.receive().await?;
            assert_eq!(response, Some(data));
        }
        Ok(())
    }
    #[tokio::test]
    async fn test_bidirectional_client_server() -> anyhow::Result<()> {
        common::generate_self_signed(vec!["localhost".to_string()], None, None)?;
        let addr: SocketAddr = "127.0.0.1:4433".parse()?;

        tokio::spawn(async move {
            let _ = server::run_bidirectional_server(
                Path::new("cert.pem"),
                Path::new("key.pem"),
                addr,
                server_handle_request,
            )
            .await;
        });

        let (_, stream) =
            client::client_connect_bidirectional(addr, "localhost", Path::new("cert.pem"), true)
                .await?;
        let (mut receive_stream, mut send_stream) = stream.split();
        let test_data = vec![
            "hello".to_string(),
            "world".to_string(),
            "foo".to_string(),
            "bar".to_string(),
        ];
        for data in test_data.iter().cloned() {
            let data = Bytes::from(data);
            send_stream.send(data.clone()).await?;
            let response = receive_stream.receive().await?;
            assert_eq!(response, Some(data));
        }
        Ok(())
    }
}
