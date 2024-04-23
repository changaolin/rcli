use std::{
    fmt::{self, Display, Formatter},
    path::Path,
    str::FromStr,
};

use clap::Parser;

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, default_value = "-", value_parser=input_value_parser)]
    pub input: String,
    #[arg(short, long, default_value = "-")]
    pub output: String,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = false)]
    pub no_header: bool,
    #[arg(short, long, default_value = "json", value_parser=format_value_parser)]
    pub format: OutputFormat,
}

#[derive(Debug, Clone)]
pub enum OutputFormat {
    Json,
    Yaml,
}
impl FromStr for OutputFormat {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err("Invalid format"),
        }
    }
}

fn format_value_parser(input: &str) -> Result<OutputFormat, &'static str> {
    match input {
        "json" => Ok(OutputFormat::Json),
        "yaml" => Ok(OutputFormat::Yaml),
        _ => Err("Invalid format"),
    }
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Yaml => write!(f, "yaml"),
        }
    }
}

fn input_value_parser(input: &str) -> Result<String, &'static str> {
    if input == "-" || Path::new(input).exists() {
        Ok(input.to_string())
    } else {
        Err("Input file does not exist")
    }
}
