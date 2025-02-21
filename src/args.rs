use clap::{Parser, Subcommand, ArgAction};
use crate::config::Config;
use crate::model::Provider;

// The main CLI parser - handles both subcommands and completions
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, name = "ask")]
pub struct Cli {
    /// Subcommand or prompt
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Maximum number of tokens to generate
    #[arg(short = 't', long = "tokens", value_name = "COUNT")]
    pub tokens: Option<u32>,

    /// Model to use for generation
    #[arg(short, long, value_name = "MODEL")]
    pub model: Option<String>,

    /// API provider to use
    #[arg(short, long, value_enum)]
    pub provider: Option<Provider>,

    /// Sampling temperature (0.0-2.0)
    #[arg(short = 'T', long, value_name = "TEMP")]
    pub temperature: Option<f32>,

    /// Outputs raw code instead of formatted text
    #[clap(short = 'r', long, action = ArgAction::SetTrue)]
    pub raw: bool,

    /// The prompt to send to the model (if no subcommand is specified)
    #[arg(trailing_var_arg = true)]
    pub prompt: Vec<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Clear the chat history
    Clr,
}

#[derive(Debug)]
pub struct CompletionArgs {
    pub model: String,
    pub tokens: u32,
    pub temperature: f32,
    pub provider: Provider,
    pub prompt: String,
    pub raw: bool,
}

#[derive(Debug)]
pub enum CliCommand {
    Completion(CompletionArgs),
    Clear,
}

impl Cli {
    pub fn parse_and_dispatch(config: &Config) -> CliCommand {
        let cli = Cli::parse();
        
        // First check if a subcommand was provided
        match cli.command {
            Some(Commands::Clr) => return CliCommand::Clear,
            None => {
                // No subcommand, so we'll use the prompt arguments
                if cli.prompt.is_empty() {
                    eprintln!("Error: No prompt provided");
                    std::process::exit(1);
                }
                
                // Convert to the final processed arguments with config defaults
                return CliCommand::Completion(CompletionArgs {
                    model: cli.model.unwrap_or(config.model.clone()),
                    tokens: cli.tokens.unwrap_or(config.max_tokens),
                    temperature: cli.temperature.unwrap_or(config.temperature),
                    provider: cli.provider.unwrap_or(config.provider.clone()),
                    prompt: cli.prompt.join(" "),
                    raw: cli.raw,
                });
            }
        }
    }
}