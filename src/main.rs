pub mod config;
pub mod model;
pub mod chat;
pub mod session;
pub mod args;
pub mod actions;

use std::error::Error;
use config::Config;
use session::Session;
use args::{Cli, CliCommand};
use actions::{hist, completion};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config: Config = config::get_config();
    let mut session: Session = session::get_session();
    let args = Cli::parse_and_dispatch(&config);
    
    // dbg!(&args);

    match args {
        CliCommand::Clear => hist::clear(&session),
        CliCommand::Completion(args) => completion::ask(args, &mut session, config).await,
    };

    Ok(())
}
