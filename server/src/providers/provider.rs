use std::error::Error;

use async_trait::async_trait;

#[async_trait]
pub trait Provider: Send + Sync {
    async fn instruct(&self) -> Result<String, Box<dyn Error>>;

    async fn get_params(&self) -> Result<String, Box<dyn Error>>;
}