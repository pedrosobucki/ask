use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs::File;
use std::io::{Read, Write};

use crate::session::Session;

const SYSTEM_PROMPT: &str = "You are a helpful linux shell application which will have its output response printed in the terminal, so use only characters which are terminal friendly whe writing your answer. You also should write concise messages, as the user will prompt you again if more detailed information is needed.";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl From<Value> for Message {
    fn from(value: Value) -> Self {
        // Extract the fields from the JSON value
        let role = value.get("role").and_then(Value::as_str).unwrap_or_default().to_string();
        let content = value.get("content").and_then(Value::as_str).unwrap_or_default().to_string();

        Message { role, content }
    }
}

pub fn get_chat_history(current_chat: &str) -> Vec<Message> {
    let file_path: String = format!("chats/{}.json", current_chat);
    let error_msg: String = format!("Failed to read {} history file!", current_chat);

    // Check if file exists, if not return messages vector just with system prompt
    if !std::path::Path::new(&file_path).exists() {
        return vec![Message{role: String::from("system"), content: String::from(SYSTEM_PROMPT)}];
    }

    // otherwise, read file and retrieve chat history
    let mut file: File = File::open(&file_path).expect(&error_msg);
    let mut contents: String = String::new();
    file.read_to_string(&mut contents).expect(&error_msg);

    // extracys just messages array from JSON
    let messages: Vec<Message> = serde_json::from_str::<serde_json::Value>(&contents)
        .expect("Failed to parse JSON")
        .get("messages")
        .expect("Failed to get messages")
        .as_array()
        .expect("Messages is not an array")
        .iter()
        .map(|m| serde_json::from_value(m.clone()).expect("Failed to deserialize message"))
        .collect();   

    messages
}

// Serialize to JSON and write to a file
pub fn save_chat_history(session: &Session) {
    let file_path: String = format!("chats/{}.json", &session.current_chat);
    let error_msg: &str = "Failed to save chat history!";
    
    let mut file = File::create(&file_path).expect(error_msg);
    let messages_json = json!({ "messages": session.messages });
    file.write_all(messages_json.to_string().as_bytes()).expect(error_msg);
}
