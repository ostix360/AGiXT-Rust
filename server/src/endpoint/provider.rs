use std::convert::Infallible;
use serde_json::json;
use hyper::{Request, Body, Response, Method};

use crate::{app::get_params, providers, Providers};


pub async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/api/provider") => {
            let providers = Providers::get_provider_list();
            let providers_json = json!({ "providers": providers }).to_string();
            let response = Response::new(providers_json.into());
            return Ok(response);
        },
        (&Method::GET, "/api/providers") => {
            return Ok(Response::new("Hello, Providers".into()));
        },
        (&Method::GET, _) => {
            let path = req.uri().path(); 
            if path.starts_with("/api/provider/") {
                let provider_name = path.replace("/api/provider/", "");
                if provider_name.contains("/") {
                    return Ok(Response::new("Invalide provider name".into()));
                }
                let settings = Providers::get_provider_settings(&provider_name).await;

                return Ok(Response::new(json!({"settings": settings}).to_string().into()));
            }
            return Ok(Response::new("This is impossible to access here".into()));
        },
        (_, _) => {
            return Ok(Response::new("Page not found".into()));
        }
    }
}