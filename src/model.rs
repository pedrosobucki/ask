use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::config::Config;
use crate::chat::Message;
use std::error::Error;
use clap::ValueEnum;

#[derive(Serialize, Deserialize, Debug, Clone, ValueEnum)]
#[clap(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    Gpt,
    Claude,
    Grok,
}

#[derive(Debug)]
pub struct ModelData {
    pub name: String,
    pub provider: Provider,
    pub max_tokens: u32,
    pub temperature: f32,
    pub api_key: String,
    pub request_uri: String
}

#[derive(Debug)]
pub enum Model {
    Gpt {data: ModelData},
    Claude {data: ModelData},
    Grok {data: ModelData},
}

impl Model {
    pub async fn request(&self, messages: &mut Vec<Message>) -> String {
        let completion = match self {
            Model::Gpt {data} => {
                let response = generic_chat_completion_request(data, messages).await.expect("Failed to get response from OpenAI API!");
                let new_message: Message = response["choices"][0]["message"].clone().into();
                let answer: String = new_message.content.clone();

                // under user message, add assistant message
                messages.push(new_message);

                answer
            },
            Model::Claude {data: _} => String::from("mock claude response"),
            Model::Grok {data: _} => String::from("mock grok response"),
            // Model::Claude {data} => generic_chat_completion_request(data).await.expect("Failed to get response from Anthropic API!"),
            // Model::Grok {data} => generic_chat_completion_request(data).await.expect("Failed to get response from xAI API!"),
        };

        completion
    }
}

pub fn build_model(config: &Config) -> Model {
    let (api_key,request_uri): (String, String) = match config.provider {
        Provider::Gpt => (config.openai_api_key.clone(), String::from("https://api.openai.com/v1/chat/completions")),
        Provider::Claude => (config.xai_api_key.clone(), String::from("")),
        Provider::Grok => (config.anthropic_api_key.clone(), String::from("")),
    };

    if api_key.is_empty() {
        panic!("API key for {:?} is empty!", config.provider);
    }

    let data: ModelData = ModelData {
        name: config.model.clone(),
        provider: config.provider.clone(),
        max_tokens: config.max_tokens,
        temperature: config.temperature,
        api_key,
        request_uri,
    };

    match config.provider {
        Provider::Gpt => Model::Gpt {data},
        Provider::Claude => Model::Claude {data},
        Provider::Grok => Model::Grok {data},
    }
}

async fn generic_chat_completion_request(model: &ModelData, messages: &mut Vec<Message>) -> Result<Value, Box<dyn Error>> {
    // Create a client
    let client = reqwest::Client::new();

    // Define the request payload
    let request_body = json!({
        "model": model.name,
        "messages": messages,
        "temperature": model.temperature,
        "max_tokens": model.max_tokens,
    });

    // Send the request
    let response = client
        .post(&model.request_uri)
        .header("Authorization", format!("Bearer {}", model.api_key))
        .json(&request_body)
        .send()
        .await?;

    // Parse the response
    let response_body: Value = response.json().await?;


    Ok(response_body)
}