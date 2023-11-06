# Quic Hyper Stunt

Simple library of Network utilities.

Quic and easy clients and servers.

```rust

use anyhow::Result;
use bytes::Bytes;
use s2n_quic::stream::BidirectionalStream;
use std::{net::SocketAddr, path::Path, sync::Arc};
use quich_hyper_stunt::quic::{common, client, server};

async fn server_handle_request(stream: BidirectionalStream) -> Result<()> {
        let mut stream = stream;
        while let Ok(Some(data)) = stream.receive().await {
            stream.send(data).await.expect("stream should be open");
        }
        Ok(())
}

async fn server_handle_conn(conn: Connection) -> Result<()> {
        let mut conn = conn;
        let stream = conn.accept_bidirectional_stream().await?.expect("stream should be open");
        server_handle_request(stream).await?;
        Ok(())
    }


 #[tokio::test]
async fn main() -> Result<()> {
   common::generate_self_signed(vec!["localhost".to_string()])?;
    let addr: SocketAddr = "127.0.0.1:4433".parse()?;

    tokio::spawn(async move {
        let _ =
            // handler takes in connection
            // server::run_server(Path::new("cert.pem"), Path::new("key.pem"), addr, server_handle_conn).await;
            
            // handler takes in BidirectionalStream
            server::run_bidirectional_server(Path::new("cert.pem"), Path::new("key.pem"), addr, server_handle_request).await;
    });

    let (client, stream) =
        client::client_connect_bidirectional(addr, "localhost", Path::new("cert.pem"), true).await?;
    
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

```




The generated random connector will have randomized cipher suites, key exchange groups, and protocol versions
The generated random client will additionally randomly set pool timeout and http1 max buffer size
## get a random TLS Stream
```Rust
use quic_hyper_stunt::client::{get_random_tls_stream, get_random_tls_connector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "reddit.com";
    let port = 443u16;
    let mut stream = get_random_tls_stream(addr, port).await;
    assert!(stream.is_ok());
    Ok(())
}
```

## Random User-Agent Tower Layer

```Rust
use http::{header, HeaderValue};
use tower_http::set_header::SetRequestHeaderLayer;
use hyper::{Body, Request, Response};
use tower::{ServiceBuilder, ServiceExt, Service};
use quic_hyper_stunt::headers::get_random_user_agent_headval;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let http_client = tower::service_fn(|req: Request<Body>| async move {
        Ok::<_, std::convert::Infallible>(Response::new(Body::empty()))
  });
  let layer = SetRequestHeaderLayer::overriding(
              header::USER_AGENT,
              |_: &Request<Body>| {
                  Some(get_random_user_agent_headval())
              }
          );
  let mut svc = ServiceBuilder::new()
      .layer(layer)
      .service(http_client);
  let request = Request::new(Body::empty());
  let _ = svc.ready().await.unwrap().call(request).await.unwrap();
    Ok(())
}
```