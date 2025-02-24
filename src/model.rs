use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::chat::Message;
use std::{error::Error, process::exit};
use clap::ValueEnum;
use crate::args::CompletionArgs;
use crate::config::Config;
use reqwest::{Response, StatusCode};

#[derive(Serialize, Deserialize, Debug, Clone, ValueEnum)]
#[clap(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    OpenAI,
    Anthropic,
    XAI,
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
    OpenAI {data: ModelData},
    Anthropic {data: ModelData},
    XAI {data: ModelData},
}

impl Model {
    pub async fn request(&self, messages: &mut Vec<Message>) -> String {
        let completion = match self {
            Model::OpenAI {data} => {
                let response: Response = openai_chat_completion_request(data, messages).await.expect("Failed to get response from OpenAI API!");
                let status: StatusCode = response.status();
                // dbg!(&response);
                let json: Value = response.json().await.expect("Failed to decode response JSON");
                // dbg!(&json);

                if !status.is_success() {
                    let pretty_json: String = serde_json::to_string_pretty(&json).expect("failed to pretty print JSON");
                    return format!("\x1b[31mX\x1b[0m Request error code {}:\n{}", status, pretty_json);
                }

                let new_message: Message = json["choices"][0]["message"].clone().into();
                let answer: String = new_message.content.clone();

                // under user message, add assistant message
                messages.push(new_message);

                answer
            },
            Model::Anthropic {data} => {
                let response: Response = anthropic_chat_completion_request(data, messages).await.expect("Failed to get response from OpenAI API!");
                let status: StatusCode = response.status();
                // dbg!(&response);
                let json: Value = response.json().await.expect("Failed to decode response JSON");
                // dbg!(&json);

                if !status.is_success() {
                    let pretty_json: String = serde_json::to_string_pretty(&json).expect("failed to pretty print JSON");
                    return format!("\x1b[31mX\x1b[0m Request error code {}:\n{}", status, pretty_json);
                }

                json.as_object().expect("Failed to convert JSON to object").get("content");
                let content = json.get("content").expect("failed")[0].get("text").and_then(|v| v.as_str()).expect("failed").to_string();

                let new_message: Message = Message {
                    role: "assistant".to_string(),
                    content,
                };

                let answer: String = new_message.content.clone();

                // under user message, add assistant message
                messages.push(new_message);

                answer
            },
            Model::XAI {data: _} => String::from("mock XAI response"),
            // Model::Anthropic {data} => generic_chat_completion_request(data).await.expect("Failed to get response from Anthropic API!"),
            // Model::XAI {data} => generic_chat_completion_request(data).await.expect("Failed to get response from xAI API!"),
        };

        completion
    }
}

pub fn build_model(config: &Config, args: &CompletionArgs) -> Model {
    let (api_key,request_uri): (String, String) = match args.provider {
        Provider::OpenAI => (config.openai_api_key.clone(), String::from("https://api.openai.com/v1/chat/completions")),
        Provider::Anthropic => (config.anthropic_api_key.clone(), String::from("https://api.anthropic.com/v1/messages")),
        Provider::XAI => (config.xai_api_key.clone(), String::from("")),
    };

    if api_key.is_empty() {
        panic!("API key for {:?} is empty!", args.provider);
    }

    let data: ModelData = ModelData {
        name: args.model.clone(),
        provider: args.provider.clone(),
        max_tokens: args.tokens,
        temperature: args.temperature,
        api_key,
        request_uri,
    };

    match args.provider {
        Provider::OpenAI => Model::OpenAI {data},
        Provider::Anthropic => Model::Anthropic {data},
        Provider::XAI => Model::XAI {data},
    }
}

async fn openai_chat_completion_request(model: &ModelData, messages: &mut Vec<Message>) -> Result<Response, Box<dyn Error>> {
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

    Ok(response)
}


async fn anthropic_chat_completion_request(model: &ModelData, messages: &mut Vec<Message>) -> Result<Response, Box<dyn Error>> {
    // Create a client
    let client = reqwest::Client::new();

    let mut msgs_clone: Vec<Message> = messages.clone();
    let system:String = msgs_clone[0].content.clone();
    msgs_clone.remove(0);

    // Define the request payload
    let request_body = json!({
        "model": model.name,
        "messages": msgs_clone,
        "temperature": model.temperature,
        "max_tokens": model.max_tokens,
        "system": system,
    });

    // Send the request
    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", &model.api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    Ok(response)
}