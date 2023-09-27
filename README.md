# Hyper Stunt

Simple library to cipher stunt [hyper](https://github.com/hyperium/hyper) clients.

It exposes three main functions:
  1. `get_random_https_connector() -> HttpsConnector<HttpConnector>`
  2. `async fn get_random_hyper_client()-> hyper::Client<HttpsConnector<HttpConnector>>)`
  3. `fn add_random_user_agent(req: &mut Request<Body>)->()`

The generated random connector will have randomized cipher suites, key exchange groups, and protocol versions
The generated random client will additionally randomly set pool timeout and http1 max buffer size
