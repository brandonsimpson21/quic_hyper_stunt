# Quic Hyper Stunt

Simple library of Network utilities.

Quic and easy clients and servers.

```rust
use std::{net::ToSocketAddrs, time::Duration};

use quic_hyper_stunt::quic::{common::read_recv_stream, error::NetworkError, handlers::handle_accept};

 #[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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