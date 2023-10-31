use std::{error::Error, sync::{Mutex, Arc}, collections::HashMap, cell::RefCell};
use lazy_static::lazy_static;
use tokio::sync::RwLock;

use crate::providers::{provider::Provider, gpt4free::Gpt4Free};

type ArcProvider = Arc<RwLock<dyn Provider>>;

lazy_static! {
    static ref PROVIDER_LIST: Mutex<HashMap<String, ArcProvider>> = Mutex::new(HashMap::new());
}


pub fn get_provider_list<'a >() -> Vec<& 'a str> {
    vec!["gpt4free", "pipeline", "llamacppapi"]
}

pub fn get_provider(provider_name: &str) -> Result<Arc<RwLock<dyn Provider>>, String> {
    match PROVIDER_LIST.lock().unwrap().get(provider_name) {
        Some(provider) => Ok(provider.clone()),
        None => Err("Provider not found".to_string())
    }
    
}

pub async fn get_provider_option(provider_name: &str) -> String {
    let provider = get_provider(provider_name);
    match provider {
        Ok(provider) => {
            let option = provider.read().await.get_params().await.unwrap();
            option
        },
        Err(err) => err
    }
}

