use std::{collections::HashMap, convert::Infallible};
use hyper::{Body, Response};

use crate::{models::AgentConfig, utils};


pub async fn add_agent(agent_config: AgentConfig) -> Result<Response<Body>, Infallible> {
    Ok(utils::string_to_response_with_status("Not implemented".to_string(), 501))
}