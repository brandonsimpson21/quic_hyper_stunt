# Hyper Stunt

Simple library to cipher stunt [hyper](https://github.com/hyperium/hyper) clients.

It exposes 4 main functions:
  1. `get_random_https_connector() -> HttpsConnector<HttpConnector>`
  2. `async fn get_random_hyper_client()-> hyper::Client<HttpsConnector<HttpConnector>>)`
  3. `fn add_random_user_agent(req: &mut Request<Body>)->()`
  4. `get_random_user_agent_headval() -> HeaderValue`

The generated random connector will have randomized cipher suites, key exchange groups, and protocol versions
The generated random client will additionally randomly set pool timeout and http1 max buffer size

## User-Agent Tower Layer

```Rust
use http::HeaderValue;
use tower_http::set_header::SetRequestHeaderLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let http_client = tower::service_fn(|req: Request<Body>| async move {

      // here the requests user agent header is a random agent
    
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

  // here the requests user agent header is empty

  let _ = svc.ready().await.unwrap().call(request).await.unwrap();
  Ok(())
}
```