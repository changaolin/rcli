mod csv_cli;
mod genpass_cli;
use clap::{Parser, Subcommand};
pub use csv_cli::{CsvOpts, OutputFormat};
pub use genpass_cli::GenPassOpts;
use std::fmt::{self, Display, Formatter};
mod base64_cli;
pub use base64_cli::{Base64Format, Base64SubCommand};
mod text_cli;
pub use text_cli::{TextSignFormat, TextSubCommand};
mod http_cli;
pub use http_cli::HttpSubCommand;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommands,
}

#[derive(Subcommand)]
pub enum Subcommands {
    #[command(name = "csv", about = "Convert CSV to JSON")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "Encode or decode base64")]
    Base64(Base64SubCommand),
    #[command(subcommand, about = "Sign or verify a text")]
    Text(TextSubCommand),
    #[command(subcommand, about = "HTTP server")]
    Http(HttpSubCommand),
}

impl Display for Subcommands {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Subcommands::Csv(_) => write!(f, "csv"),
            Subcommands::GenPass(_) => write!(f, "genpass"),
            Subcommands::Base64(_) => write!(f, "base64"),
            Subcommands::Text(_) => write!(f, "text"),
            Subcommands::Http(_) => write!(f, "http"),
        }
    }
}
