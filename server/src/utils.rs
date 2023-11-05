use std::collections::HashMap;


use hyper::{Body, Response};
use serde_json::json;

pub fn hashmap_to_response(hashmap: HashMap<&str, String>) -> Response<Body> {
    hashmap_to_response_with_status(hashmap, 200)
}

pub fn hashmap_to_response_with_status(hashmap: HashMap<&str, String>, status: u16) -> Response<Body> {
    let body = json!(hashmap);
    Response::builder()
        .status(status)
        .body(Body::from(body.to_string()))
        .unwrap_or_else(|_| string_to_response_with_status("Internal Server Error".to_string(), 500))
}

pub fn string_to_response(string: String) -> Response<Body> {
    string_to_response_with_status(string, 200)
}

pub fn string_to_response_with_status(string: String, status: u16) -> Response<Body> {
    Response::builder()
        .status(status)
        .body(Body::from(string))
        .unwrap_or_else(|e| Response::new(Body::from(e.to_string())))
}
