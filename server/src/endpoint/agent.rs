use std::collections::HashMap;
use hyper::{Body, Request, Response, Server, Method, body};
use hyper::service::{make_service_fn, service_fn};
use hyper::header::{HeaderMap, HeaderValue};
use hyper::http::StatusCode;
use tokio::sync::Mutex;
use url::Url;
use serde_json::{json, Value};

use log::{info, error};
use std::convert::Infallible;
use crate::api_client;
use crate::models::{AgentConfig, AgentSettings, AgentNewName, AgentCommands, AgentPrompt, ToggleCommandPayload};

async fn add_agent(agent: AgentConfig, user: String) -> Result<Response<Body>, Infallible> {
    api_client::add_agent(agent).await
}

async fn import_agent(agent: AgentConfig, user: String) -> Result<Response<Body>, Infallible> {
    // TODO: Implement import_agent function
    Ok(Response::new(Body::from("")))
}

async fn rename_agent(agent_name: String, new_name: AgentNewName, user: String) -> Result<Response<Body>, Infallible> {
    // TODO: Implement rename_agent function
    Ok(Response::new(Body::from("")))
}

async fn update_agent_settings(agent_name: String, settings: AgentSettings, user: String, authorization: HeaderValue) -> Result<Response<Body>, Infallible> {
    // TODO: Implement update_agent_settings function
    Ok(Response::new(Body::from("")))
}

async fn update_agent_commands(agent_name: String, commands: AgentCommands, user: String, authorization: HeaderValue) -> Result<Response<Body>, Infallible> {
    // TODO: Implement update_agent_commands function
    Ok(Response::new(Body::from("")))
}

async fn delete_agent(agent_name: String, user: String) -> Result<Response<Body>, Infallible> {
    // TODO: Implement delete_agent function
    Ok(Response::new(Body::from("")))
}

async fn get_agents(user: String) -> Result<Response<Body>, Infallible> {
    // TODO: Implement get_agents function
    Ok(Response::new(Body::from("")))
}

async fn get_agent_config(agent_name: String, user: String, authorization: HeaderValue) -> Result<Response<Body>, Infallible> {
    // TODO: Implement get_agent_config function
    Ok(Response::new(Body::from("")))
}

async fn prompt_agent(agent_name: String, agent_prompt: AgentPrompt, user: String, authorization: HeaderValue) -> Result<Response<Body>, Infallible> {
    // TODO: Implement prompt_agent function
    Ok(Response::new(Body::from("")))
}

async fn get_commands(agent_name: String, user: String, authorization: HeaderValue) -> Result<Response<Body>, Infallible> {
    // TODO: Implement get_commands function
    Ok(Response::new(Body::from("")))
}

async fn toggle_command(agent_name: String, payload: ToggleCommandPayload, user: String, authorization: HeaderValue) -> Result<Response<Body>, Infallible> {
    // TODO: Implement toggle_command function
    Ok(Response::new(Body::from("")))
}

pub async fn handle_request(mut req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let body = hyper::body::to_bytes(req.body_mut()).await.unwrap();
    let uri = req.uri();
    let path = uri.path();
    let method = req.method();
    let headers = req.headers();
    let authorization = headers.get("Authorization").unwrap();
    let user = String::from("user"); // TODO: Implement user authent
    

    match (method, path) {
        (&Method::POST, "/api/agent") => {
            match parse_body::<AgentConfig>(&body).await {
                Ok(agent) => add_agent(agent, user).await,
                Err(e) => Ok(e)
            }
        },
        (&Method::POST, "/api/agent/import") => {
            match parse_body::<AgentConfig>(&body).await {
                Ok(agent) => import_agent(agent, user).await,
                Err(e) => Ok(e)
            }
        },
        (&Method::PATCH, path) if path.starts_with("/api/agent/") => {
            let agent_name = path.trim_start_matches("/api/agent/").to_string();
            match parse_body::<AgentNewName>(&body).await {
                Ok(new_name) => rename_agent(agent_name, new_name, user).await,
                Err(e) => Ok(e)
            }
        },
        (&Method::PUT, path) if path.starts_with("/api/agent/") && path.ends_with("/settings") => {
            let agent_name = path.trim_start_matches("/api/agent/").trim_end_matches("/settings").to_string();
            match parse_body::<AgentSettings>(&body).await {
                Ok(settings) => update_agent_settings(agent_name, settings, user, authorization.clone()).await,
                Err(e) => Ok(e)
            }
        },
        (&Method::PUT, path) if path.starts_with("/api/agent/") && path.ends_with("/commands") => {
            let agent_name = path.trim_start_matches("/api/agent/").trim_end_matches("/commands").to_string();
            match parse_body::<AgentCommands>(&body).await {
                Ok(commands) => update_agent_commands(agent_name, commands, user, authorization.clone()).await,
                Err(e) => Ok(e)
            }
        },
        (&Method::DELETE, path) if path.starts_with("/api/agent/") => {
            let agent_name = path.trim_start_matches("/api/agent/").to_string();
            delete_agent(agent_name, user).await
        },
        (&Method::GET, "/api/agent") => {
            get_agents(user).await
        },
        (&Method::GET, path) if path.starts_with("/api/agent/") => {
            let agent_name = path.trim_start_matches("/api/agent/").to_string();
            get_agent_config(agent_name, user, authorization.clone()).await
        },
        (&Method::POST, path) if path.starts_with("/api/agent/") && path.ends_with("/prompt") => {
            let agent_name = path.trim_start_matches("/api/agent/").trim_end_matches("/prompt").to_string();
            match parse_body::<AgentPrompt>(&body).await {
                Ok(agent_prompt) => prompt_agent(agent_name, agent_prompt, user, authorization.clone()).await,
                Err(e) => Ok(e)
            }
        },
        (&Method::GET, path) if path.starts_with("/api/agent/") && path.ends_with("/command") => {
            let agent_name = path.trim_start_matches("/api/agent/").trim_end_matches("/command").to_string();
            get_commands(agent_name, user, authorization.clone()).await
        },
        (&Method::PATCH, path) if path.starts_with("/api/agent/") && path.ends_with("/command") => {
            let agent_name = path.trim_start_matches("/api/agent/").trim_end_matches("/command").to_string();
            match parse_body::<ToggleCommandPayload>(&body).await {
                Ok(payload) => toggle_command(agent_name, payload, user, authorization.clone()).await,
                Err(e) => Ok(e)
            }
        },
        _ => {
            let mut res = Response::new(Body::from("Not Found"));
            *res.status_mut() = StatusCode::NOT_FOUND;
            Ok(res)
        }
    }
}

async fn parse_body<T: serde::de::DeserializeOwned>(body: &[u8]) -> Result<T, Response<Body>>{
    match serde_json::from_slice(body) {
        Ok(t) => Ok(t),
        Err(e) => {
            Err(
            Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(format!("Invalid JSON: {}", e)))
                .map_err(|e| {
                    Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::from(format!("Something went wrong: {}", e)))
                        .unwrap()
                }).unwrap()
            )
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use hyper::header::HeaderValue;
    use hyper::Body;
    use maplit::hashmap;
    use std::convert::Infallible;

    #[tokio::test]
    async fn test_add_agent() {
        let agent = AgentConfig {
            agent_name: "test_agent".to_string(),
            commands: hashmap! {},
            settings: Some(hashmap!["setting1".to_string() => Value::String("value1".to_string())]),
        };
        let user = String::from("test_user");

        let result = add_agent(agent, user).await;

        match result {
            Ok(response) => {
                assert_eq!(response.status(), StatusCode::OK);
                // Vous pouvez ajouter d'autres assertions ici pour vérifier le corps de la réponse ou d'autres aspects de la réponse.
            }
            Err(e) => panic!("Erreur lors de l'appel à add_agent: {:?}", e),
        }
    }
}