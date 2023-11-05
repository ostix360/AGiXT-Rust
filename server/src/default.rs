use lazy_static::lazy_static;
use maplit::hashmap;
use std::collections::HashMap;
use serde_json::Value;


lazy_static! {
    pub static ref DEFAULT_SETTINGS: HashMap<String, Value> = hashmap! {
        "provider".to_string() => Value::String("gpt4free".to_string()),
        "embedder".to_string() => Value::String("default".to_string()),
        "AI_MODEL".to_string() => Value::String("gpt-3.5-turbo".to_string()),
        "AI_TEMPERATURE".to_string() => Value::String("0.7".to_string()),
        "AI_TOP_P".to_string() => Value::String("1".to_string()),
        "MAX_TOKENS".to_string() => Value::String("4096".to_string()),
        "helper_agent_name".to_string() => Value::String("gpt4free".to_string()),
        "WEBSEARCH_TIMEOUT".to_string() => Value::String("0".to_string()),
        "WAIT_BETWEEN_REQUESTS".to_string() => Value::String("1".to_string()),
        "WAIT_AFTER_FAILURE".to_string() => Value::String("3".to_string()),
        "stream".to_string() => Value::Bool(false),
        "WORKING_DIRECTORY".to_string() => Value::String("./WORKSPACE".to_string()),
        "WORKING_DIRECTORY_RESTRICTED".to_string() => Value::Bool(true),
        "AUTONOMOUS_EXECUTION".to_string() => Value::Bool(true),
        "PERSONA".to_string() => Value::String("".to_string()),
    };
}
