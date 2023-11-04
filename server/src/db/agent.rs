use std::{collections::HashMap, convert::Infallible};
use hyper::{Body, Response};

use crate::db_connection::establish_connection;


pub async fn add_agent(agent_name: String,
    settings: HashMap<String, serde_json::Value>,
    commands: HashMap<String, serde_json::Value>
    ) -> Result<Response<Body>, Infallible> {
    let session = establish_connection();
    let user_data = session.query::<User>().filter(User::email.eq(user)).first();

    Ok(Response::new(Body::from("")))
}