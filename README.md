# Hyper Stunt

Simple library to cipher stunt [hyper](https://github.com/hyperium/hyper) clients.


The generated random connector will have randomized cipher suites, key exchange groups, and protocol versions
The generated random client will additionally randomly set pool timeout and http1 max buffer size
## get a random TLS Stream
```Rust
use hyper_stunt::client::{get_random_tls_stream, get_random_tls_connector};
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
use hyper_stunt::headers::get_random_user_agent_headval;

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