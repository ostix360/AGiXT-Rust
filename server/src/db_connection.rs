// // Crates: diesel, diesel_migrations, dotenv, uuid, chrono, serde, serde_json, tokio, async-trait, url

// use diesel::pg::PgConnection;
// use diesel::prelude::*;
// use dotenv::dotenv;
// use std::env;
// use uuid::Uuid;
// use chrono::{DateTime, Utc};
// use serde::{Deserialize, Serialize};
// use async_trait::async_trait;
// use url::Url;


// #[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
// #[table_name = "user"]
// pub struct User {
//     id: Uuid,
//     email: String,
// }

// #[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
// #[table_name = "provider"]
// pub struct Provider {
//     id: Uuid,
//     name: String,
// }

// #[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
// #[table_name = "provider_setting"]
// pub struct ProviderSetting {
//     id: Uuid,
//     provider_id: Uuid,
//     name: String,
//     value: String,
// }

// #[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
// #[table_name = "agent_provider_setting"]
// pub struct AgentProviderSetting {
//     id: Uuid,
//     provider_setting_id: Uuid,
//     agent_provider_id: Uuid,
//     value: String,
// }

// #[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
// #[table_name = "agent_provider"]
// pub struct AgentProvider {
//     id: Uuid,
//     provider_id: Uuid,
//     agent_id: Uuid,
// }

// #[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
// #[table_name = "agent"]
// pub struct Agent {
//     id: Uuid,
//     name: String,
//     provider_id: Option<Uuid>,
//     user_id: Option<Uuid>,
// }

// #[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
// #[table_name = "command"]
// pub struct Command {
//     id: Uuid,
//     name: String,
//     extension_id: Option<Uuid>,
// }

// #[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
// #[table_name = "agent_command"]
// pub struct AgentCommand {
//     id: Uuid,
//     command_id: Uuid,
//     agent_id: Uuid,
//     state: bool,
// }

// #[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
// #[table_name = "conversation"]
// pub struct Conversation {
//     id: Uuid,
//     agent_id: Uuid,
//     name: String,
//     user_id: Option<Uuid>,
// }

// #[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
// #[table_name = "message"]
// pub struct Message {
//     id: Uuid,
//     role: String,
//     content: String,
//     timestamp: DateTime<Utc>,
//     conversation_id: Uuid,
// }

// #[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
// #[table_name = "setting"]
// pub struct Setting {
//     id: Uuid,
//     name: String,
//     extension_id: Option<Uuid>,
//     value: String,
// }

// #[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
// #[table_name = "chain"]
// pub struct Chain {
//     id: Uuid,
//     name: String,
//     description: Option<String>,
//     user_id: Option<Uuid>,
// }

// #[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
// #[table_name = "chain_step"]
// pub struct ChainStep {
//     id: Uuid,
//     chain_id: Uuid,
//     agent_id: Uuid,
//     prompt_type: Option<String>,
//     prompt: Option<String>,
//     target_chain_id: Option<Uuid>,
//     target_command_id: Option<Uuid>,
//     target_prompt_id: Option<Uuid>,
//     step_number: i32,
// }

// #[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
// #[table_name = "chain_step_argument"]
// pub struct ChainStepArgument {
//     id: Uuid,
//     argument_id: Uuid,
//     chain_step_id: Uuid,
//     value: String,
// }

// #[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
// #[table_name = "chain_step_response"]
// pub struct ChainStepResponse {
//     id: Uuid,
//     chain_step_id: Uuid,
//     timestamp: DateTime<Utc>,
//     content: String,
// }

// #[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
// #[table_name = "extension"]
// pub struct Extension {
//     id: Uuid,
//     name: String,
//     description: Option<String>,
// }

// #[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
// #[table_name = "argument"]
// pub struct Argument {
//     id: Uuid,
//     prompt_id: Option<Uuid>,
//     command_id: Option<Uuid>,
//     chain_id: Option<Uuid>,
//     name: String,
// }

// #[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
// #[table_name = "prompt_category"]
// pub struct PromptCategory {
//     id: Uuid,
//     name: String,
//     description: String,
//     user_id: Option<Uuid>,
// }

// #[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
// #[table_name = "prompt"]
// pub struct Prompt {
//     id: Uuid,
//     prompt_category_id: Uuid,
//     name: String,
//     description: String,
//     content: String,
//     user_id: Option<Uuid>,
// }

// pub fn establish_connection() -> PgConnection {
//     dotenv().ok();

//     let database_url = env::var("DATABASE_URL")
//         .expect("DATABASE_URL must be set");
//     PgConnection::establish(&database_url)
//         .expect(&format!("Error connecting to {}", database_url))
// }

// #[async_trait]
// pub trait DBConnection {
//     async fn get_session(&self) -> PgConnection;
// }

// pub struct DB;

// #[async_trait]
// impl DBConnection for DB {
//     async fn get_session(&self) -> PgConnection {
//         establish_connection()
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
//     async fn test_get_session() {
//         let db = DB {};
//         let session = db.get_session().await;
//         assert!(session.is_ok());
//     }
// }
