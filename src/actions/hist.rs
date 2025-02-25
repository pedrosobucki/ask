// use crate::args::ClearArgs;
use crate::session::Session;
use std::{fs, process::exit};

pub fn clear(session: &Session) {
  //deletes file
  let file_path: String = session.current_chat_path();

  if !std::path::Path::new(&file_path).exists() {
    // print!("Chat \"{}\" history doesn't exist\n", &session.current_chat);
    exit(1);
  }

  let error_msg: String = format!("Failed to read \"{}\" history file!", &session.current_chat);
  fs::remove_file(&file_path).expect(&error_msg);

  print!("Chat \"{}\" history cleared!\n", &session.current_chat);
}