use std::collections::HashMap;


#[derive(Debug, serde::Deserialize)]
pub struct AgentName {
    agent_name: String,
    
}

#[derive(Debug, serde::Deserialize)]
pub struct AgentNewName {
    new_name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct AgentPrompt {
    prompt_name: String,
    prompt_args: HashMap<String, String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct AgentMemoryQuery {
    user_input: String,
    limit: Option<i32>,
    min_relevance_score: Option<f32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Objective {
    objective: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Prompt {
    prompt: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct PromptName {
    prompt_name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct PromptList {
    prompts: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct PromptCategoryList {
    prompt_categories: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Completions {
    model: Option<String>,
    prompt: Option<String>,
    suffix: Option<String>,
    max_tokens: Option<i32>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    n: Option<i32>,
    stream: Option<bool>,
    logprobs: Option<i32>,
    echo: Option<bool>,
    stop: Option<Vec<String>>,
    presence_penalty: Option<f32>,
    frequency_penalty: Option<f32>,
    best_of: Option<i32>,
    logit_bias: Option<HashMap<String, f32>>,
    user: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ChatCompletions {
    model: Option<String>,
    messages: Option<Vec<HashMap<String, String>>>,
    functions: Option<Vec<HashMap<String, String>>>,
    function_call: Option<String>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    n: Option<i32>,
    stream: Option<bool>,
    stop: Option<Vec<String>>,
    max_tokens: Option<i32>,
    presence_penalty: Option<f32>,
    frequency_penalty: Option<f32>,
    logit_bias: Option<HashMap<String, f32>>,
    user: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct EmbeddingModel {
    input: String,
    model: String,
    user: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ChainNewName {
    new_name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct ChainName {
    chain_name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct ChainData {
    chain_name: String,
    steps: HashMap<String, serde_json::Value>,
}

#[derive(Debug, serde::Deserialize)]
pub struct RunChain {
    prompt: String,
    agent_override: Option<String>,
    all_responses: Option<bool>,
    from_step: Option<i32>,
    chain_args: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct RunChainStep {
    prompt: String,
    agent_override: Option<String>,
    chain_args: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct StepInfo {
    step_number: i32,
    agent_name: String,
    prompt_type: String,
    prompt: HashMap<String, serde_json::Value>,
}

#[derive(Debug, serde::Deserialize)]
pub struct RunChainResponse {
    response: String,
    agent_name: String,
    prompt: HashMap<String, serde_json::Value>,
    prompt_type: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct ChainStep {
    step_number: i32,
    agent_name: String,
    prompt_type: String,
    prompt: HashMap<String, serde_json::Value>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ChainStepNewInfo {
    old_step_number: i32,
    new_step_number: i32,
}

#[derive(Debug, serde::Deserialize)]
pub struct ResponseMessage {
    message: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct UrlInput {
    url: String,
    collection_number: Option<i32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct FileInput {
    file_name: String,
    file_content: String,
    collection_number: Option<i32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct TextMemoryInput {
    user_input: String,
    text: String,
    collection_number: Option<i32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct TaskOutput {
    output: String,
    message: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ToggleCommandPayload {
    command_name: String,
    enable: bool,
}

#[derive(Debug, serde::Deserialize)]
pub struct CustomPromptModel {
    prompt_name: String,
    prompt: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct AgentSettings {
    agent_name: String,
    settings: HashMap<String, serde_json::Value>,
}

#[derive(Debug, serde::Deserialize)]
pub struct AgentConfig {
    pub agent_name: String,
    pub settings: Option<HashMap<String, serde_json::Value>>,
    pub commands: HashMap<String, serde_json::Value>,
}

#[derive(Debug, serde::Deserialize)]
pub struct AgentCommands {
    agent_name: String,
    commands: HashMap<String, serde_json::Value>,
}

#[derive(Debug, serde::Deserialize)]
pub struct HistoryModel {
    agent_name: String,
    conversation_name: String,
    limit: Option<i32>,
    page: Option<i32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ConversationHistoryModel {
    agent_name: String,
    conversation_name: String,
    conversation_content: Vec<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ConversationHistoryMessageModel {
    agent_name: String,
    conversation_name: String,
    message: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct GitHubInput {
    github_repo: String,
    github_user: Option<String>,
    github_token: Option<String>,
    github_branch: Option<String>,
    use_agent_settings: Option<bool>,
    collection_number: Option<i32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ArxivInput {
    query: Option<String>,
    article_ids: Option<String>,
    max_results: Option<i32>,
    collection_number: Option<i32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CommandExecution {
    command_name: String,
    command_args: HashMap<String, serde_json::Value>,
    conversation_name: Option<String>,
}
