use std::collections::HashMap;
use std::convert::Infallible;


use hyper::{Body, Request, Response};


use url::form_urlencoded;

use crate::endpoint::provider;

pub async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path = req.uri().path();
    if path.starts_with("/api/provider") {
        return provider::handle_request(req).await;
    }else if path.starts_with("/api/agent") {
        return Ok(Response::new("Hello, Agent".into()));
    }else if path.starts_with("/api/chain") {
        return Ok(Response::new("Hello, Chain".into()));
    }else {
        return Ok(Response::new("Page not found".into()));
    }
}

pub async fn get_params(req: Request<Body>) -> HashMap<String, String> {
    let b= hyper::body::to_bytes(req.into_body()).await.unwrap();
    let params = form_urlencoded::parse(b.as_ref())
                .into_owned()
                .collect::<HashMap<String, String>>();
    return params;
}
