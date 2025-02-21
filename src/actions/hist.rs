// use crate::args::ClearArgs;
use crate::session::Session;
use std::fs;

pub fn clear(session: &Session) {
  //deletes file
  let file_path: String = format!("chats/{}.json", &session.current_chat);
  let error_msg: String = format!("Failed to read {} history file!", &session.current_chat);
  fs::remove_file(&file_path).expect(&error_msg);

  print!("Chat \"{}\" history cleared!\n", &session.current_chat);
}