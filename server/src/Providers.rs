use std::{error::Error, sync::{Mutex, Arc}, collections::HashMap, cell::RefCell};
use lazy_static::lazy_static;
use tokio::sync::RwLock;

use crate::providers::{provider::Provider, gpt4free::Gpt4Free};

type ArcProvider = Arc<dyn Provider>;
type ProviderError = &'static str;

lazy_static! {
    static ref PROVIDER_LIST: Mutex<HashMap<String, ArcProvider>> = Mutex::new(HashMap::new());
    static ref PROVIDER_NAMES: Vec<&'static str> = vec!["gpt4free", "pipeline", "llamacppapi"];
}

pub fn init_providers() {
    let mut provider_list = PROVIDER_LIST.lock().expect("PROVIDER_LIST is stuck");
    provider_list.insert("gpt4free".to_string(), Arc::new(Gpt4Free::new()));
}


pub fn get_provider_list<'a >() -> &'a Vec<& 'a str> {
    &PROVIDER_NAMES
}

pub fn get_provider(provider_name: &str) -> Result<ArcProvider, ProviderError> {
    let needinit = {
         PROVIDER_LIST.lock().expect("PROVIDER_LIST is stuck").is_empty()
    };
    if needinit {
        init_providers();
    }
    match PROVIDER_LIST.lock().expect("PROVIDER_LIST is stuck").get(provider_name) {
        Some(provider) => Ok(provider.clone()),
        None => Err("Provider not found")
    }
    
}

pub async fn get_provider_settings(provider_name: &str) -> String {
    let provider = get_provider(provider_name);
    match provider {
        Ok(provider) => {
            let option = provider.get_params().await.unwrap();
            option
        },
        Err(err) => err.to_string()
    }
}

