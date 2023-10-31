use std::{error::Error, sync::Mutex, collections::HashMap};
use lazy_static::lazy_static;

use crate::providers::provider::Provider;

lazy_static! {
    static ref PROVIDER_LIST: Mutex<HashMap<String, dyn Provider>> = Mutex::new(HashMap::new());
}

pub fn get_provider_list<'a >() -> Vec<& 'a str> {
    vec!["gpt4free", "pipeline", "llamacppapi"]
}

pub fn get_provider(provider_name: &str) -> dyn Provider {
    PROVIDER_LIST.lock().unwrap().get(provider_name).unwrap().clone()
}

pub async fn get_provider_option(provider_name: &str) -> Result<String, dyn Error> {
    let provider = get_provider(provider_name);
    let option = provider.get_option().await?;
    Ok(option)   
}

