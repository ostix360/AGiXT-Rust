use std::error::Error;

use async_trait::async_trait;
use serde_json::json;

use super::provider::Provider;

pub struct Gpt4Free {
    pub option: String,
}

#[async_trait]
impl Provider for Gpt4Free {
    async fn instruct(&self) -> Result<String, Box<dyn Error>> {
        Ok("instruct".to_string())
    }

    async fn get_params(&self) -> Result<String, Box<dyn Error>> {
        Ok(json!({"option": self.option}).to_string())
    }
}