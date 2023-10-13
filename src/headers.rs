use super::utils::get_random_usr_agent;
use http::HeaderValue;
use hyper::{Body, Request};

#[inline(always)]
/// add a random user agent to the request
pub fn add_random_user_agent(req: &mut Request<Body>) {
    req.headers_mut()
        .insert("User-Agent", get_random_usr_agent().parse().unwrap());
}

#[inline(always)]
/// get a random user agent
/// this is most usefule for tower layers
/// EG;
/// ```
/// use http::{header, HeaderValue};
/// use tower_http::set_header::SetRequestHeaderLayer;
/// use hyper::{Body, Request, Response};
/// use tower::{ServiceBuilder, ServiceExt, Service};
/// use hyper_stunt::headers::get_random_user_agent_headval;
///
/// #[tokio::main]
///  async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let http_client = tower::service_fn(|req: Request<Body>| async move {
///         Ok::<_, std::convert::Infallible>(Response::new(Body::empty()))
/// });
/// let layer = SetRequestHeaderLayer::overriding(
///              header::USER_AGENT,
///              |_: &Request<Body>| {
///                  Some(get_random_user_agent_headval())
///              }
///          );
///  let mut svc = ServiceBuilder::new()
///      .layer(layer)
///      .service(http_client);

///  let request = Request::new(Body::empty());
///  let _ = svc.ready().await.unwrap().call(request).await.unwrap();
///    Ok(())
/// }
/// ```
pub fn get_random_user_agent_headval() -> HeaderValue {
    let agent = get_random_usr_agent();
    http::header::HeaderValue::from_str(agent).expect("invalid user agent")
}

#[cfg(test)]
mod test_headers {
    use super::*;
    use http::{header, Response};
    use tower::{Service, ServiceBuilder, ServiceExt};
    use tower_http::set_header::SetRequestHeaderLayer;

    #[test]
    fn test_add_random_user_agent() {
        let agent = get_random_usr_agent();
        let mut req = Request::new(Body::empty());
        let mut new_agent;

        // 1 in USER_AGENTS.len() chance of failing
        loop {
            add_random_user_agent(&mut req);
            let headers = req.headers();
            new_agent = headers.get("User-Agent").unwrap().to_str().unwrap();
            if new_agent != agent {
                break;
            }
        }
        assert_ne!(new_agent, agent);
    }

    #[tokio::test]
    async fn test_header_layer_in_tower_service() {
        let http_client = tower::service_fn(|req: Request<Body>| async move {
            let ua = req.headers().get("User-Agent");
            assert!(ua.is_some());
            let ua = ua.unwrap().to_str().unwrap().to_string();
            Ok::<_, std::convert::Infallible>(Response::new(Body::from(ua)))
        });
        let layer = SetRequestHeaderLayer::overriding(header::USER_AGENT, |_: &Request<Body>| {
            Some(get_random_user_agent_headval())
        });
        let mut svc = ServiceBuilder::new().layer(layer).service(http_client);

        let request = Request::new(Body::empty());
        assert!(request.headers().get("User-Agent").is_none());
        let resp = svc.ready().await.unwrap().call(request).await.unwrap();
        let ua1 = hyper::body::to_bytes(resp.into_body()).await;
        assert!(ua1.is_ok());
        let ua1 = ua1.unwrap();
        assert!(ua1.len() > 0);

        let request = Request::new(Body::empty());
        let resp = svc.ready().await.unwrap().call(request).await.unwrap();
        let ua2 = hyper::body::to_bytes(resp.into_body()).await;
        assert!(ua2.is_ok());
        let ua2 = ua2.unwrap();
        assert!(ua2.len() > 0);
        assert_ne!(ua1, ua2);
    }
}
