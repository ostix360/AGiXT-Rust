use std::convert::Infallible;
use std::fs::File;
use std::io::Write;
use hyper::{Response, Body};
use log::warn;
use serde_json::json;
use maplit::hashmap;

use crate::default::DEFAULT_SETTINGS;
use crate::models::AgentConfig;
use crate::utils;
use std::path::PathBuf;
use std::fs;

pub async fn add_agent(agent_config: AgentConfig) -> Result<Response<Body>, Infallible> {
    let agent_name = agent_config.agent_name;
    let commands = agent_config.commands;
    let provider_settings = agent_config.settings;

    if agent_name.is_empty() {
        warn!("Agent name is empty");
        return Ok(utils::string_to_response_with_status("Agent name is empty".to_string(), 501))
    }
    let provider_settings = match provider_settings {
        Some(settings) if !settings.is_empty() => settings,
        _ => DEFAULT_SETTINGS.to_owned(),
    };
    let (config_path, _folder_path) = match get_agent_file_paths(agent_name.clone()) {
        Ok(paths) => paths,
        Err(e) => {
            warn!("Error getting agent file paths: {}", e);
            return Ok(utils::string_to_response_with_status(e, 500));
        }
    };
    let settings = json!({
        "commands": commands,
        "settings": provider_settings,
    });

    let settings_str = match serde_json::to_string(&settings).map_err(|e| e.to_string()) {
        Ok(s) => s,
        Err(e) => {
            warn!("Error serializing settings to string: {}", e);
            return Ok(utils::string_to_response_with_status(e, 500));
        }
    };

    let mut file = match File::create(config_path).map_err(|e| e.to_string()) {
        Ok(f) => f,
        Err(e) => {
            warn!("Error creating file: {}", e);
            return Ok(utils::string_to_response_with_status(e, 500));
        }
    };

    match file.write_all(settings_str.as_bytes()).map_err(|e| e.to_string()) {
        Ok(_) => Ok(utils::hashmap_to_response(hashmap!["message" => format!("Agent {} created.", agent_name)])),
        Err(e) => {
            warn!("Error writing to file: {}", e);
            return Ok(utils::string_to_response_with_status(e, 500));
        }
    }
}


fn get_agent_file_paths(agent_name: String) -> Result<(PathBuf, PathBuf), String> {
    let base_path = std::env::current_dir().map_err(|e| e.to_string())?;
    let agents_path = base_path.join("agents");
    
    if !agents_path.exists() {
        warn!("Failed to find agents directory");
        fs::create_dir_all(&agents_path).map_err(|e| format!("Failed to create directory: {}", e))?;
    }
    let folder_path = agents_path.join(agent_name);
    if !folder_path.exists() {
        warn!("Failed to find agent folder");
        fs::create_dir_all(&folder_path).map_err(|e| format!("Failed to create directory: {}", e))?;
    }
    let config_path = folder_path.join("config.json");
    if !config_path.exists() {
        warn!("Failed to find agent config file");
        fs::File::create(&config_path).map_err(|e| format!("Failed to create file: {}", e))?;
    }

    if !config_path.starts_with(&agents_path) || !folder_path.starts_with(&agents_path) {
        return Err("Invalid path, agent name must not contain slashes.".to_string());
    }
    Ok((config_path, folder_path))
}


#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::*;
    use std::fs;
    use std::path::Path;

    #[tokio::test]
    async fn test_add_agent() {
        let agent_config = AgentConfig {
            agent_name: "test_agent".to_string(),
            commands: hashmap! {},
            settings: Some(hashmap!["setting1".to_string() => Value::String("value1".to_string())]),
        };

        let response = add_agent(agent_config).await.unwrap();
        assert_eq!(response.status(), 200);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body.get("message").unwrap(), "Agent test_agent created.");

        let (config_path, folder_path) = get_agent_file_paths("test_agent".to_string()).unwrap();
        assert!(config_path.exists());
        assert!(folder_path.exists());

        // Clean up
        fs::remove_dir_all(folder_path).unwrap();
    }

    #[test]
    fn test_get_agent_file_paths() {
        let (config_path, folder_path) = get_agent_file_paths("test_agent".to_string()).unwrap();
        let base_path = std::env::current_dir().map_err(|e| e.to_string()).unwrap();
        assert_eq!(config_path, base_path.join(Path::new("agents/test_agent/config.json")));
        assert_eq!(folder_path, base_path.join(Path::new("agents/test_agent")));

        // Clean up
        fs::remove_dir_all(folder_path).unwrap();
    }
}
