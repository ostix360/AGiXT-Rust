use std::{env, convert::Infallible};

use hyper::{Response, Body};
use lazy_static::lazy_static;

use crate::{models::*, db, fb};


lazy_static!(
    pub static ref DB_CONNECTED:bool = env::var("DB_CONNECTED").unwrap_or_else(|_| "false".to_string()) == "true";
);

pub fn is_db_connected() -> bool {
    *DB_CONNECTED
}

pub async fn add_agent(agent_config: AgentConfig) -> Result<Response<Body>, Infallible> {
    if *DB_CONNECTED {
        db::agent::add_agent(agent_config).await
    }else {
        fb::agent::add_agent(agent_config).await
    }
}