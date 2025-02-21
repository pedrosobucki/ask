use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use crate::chat;
use crate::chat::Message;

#[derive(Serialize, Deserialize, Debug)]
struct SessionJson {
    pub current_chat: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub current_chat: String,
    pub messages: Vec<Message>
}

impl From<SessionJson> for Session {
    fn from(session_json: SessionJson) -> Session {
        Session {
            current_chat: session_json.current_chat.clone(),
            messages: chat::get_chat_history(&session_json.current_chat)
        }
    }
}

impl Session {
    pub fn save(&self) {
        chat::save_chat_history(&self);
    }
}

pub fn get_session() -> Session {
    let error_msg: &str = "Failed to read session.json file!";
    let mut file: File = File::open("session.json").expect(error_msg);
    let mut contents: String = String::new();
    file.read_to_string(&mut contents).expect(error_msg);
    let json: SessionJson = serde_json::from_str(&contents).expect(error_msg);

    Session::from(json)
}