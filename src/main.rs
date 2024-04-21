use anyhow::Result;
use clap::{Parser, Subcommand};
use csv::Reader;
use serde_json::Value;
use std::{
    fmt::{self, Display, Formatter},
    path::Path,
    str::FromStr,
};

/// csv --input <input> --output <output> --delimiter <delimiter> --format json --no-header
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    cmd: Subcommands,
}

#[derive(Subcommand)]
enum Subcommands {
    #[command(name = "csv", about = "Convert CSV to JSON")]
    Csv(CscOpts),
}
#[derive(Debug, Parser)]
struct CscOpts {
    #[arg(short, long, default_value = "-", value_parser=input_value_parser)]
    input: String,
    #[arg(short, long, default_value = "-")]
    output: String,
    #[arg(short, long, default_value_t = ',')]
    delimiter: char,
    #[arg(long, default_value_t = false)]
    no_header: bool,
    #[arg(short, long, default_value = "json", value_parser=format_value_parser)]
    format: Format,
}
fn main() -> anyhow::Result<()> {
    let cli = Opts::parse();
    match cli.cmd {
        Subcommands::Csv(csv) => process_csv(&csv.input, &csv.output, csv.format, csv.no_header)?,
    }
    Ok(())
}

fn process_csv(input: &str, output: &str, format: Format, no_header: bool) -> Result<()> {
    let mut buf = String::new();
    if input == "-" {
        std::io::stdin().read_line(&mut buf)?;
    } else {
        buf = std::fs::read_to_string(input)?;
    }
    let mut ret = vec![];
    match format {
        Format::Json => {
            let mut rdr = Reader::from_reader(buf.as_bytes());
            if no_header {
                rdr.records().enumerate().for_each(|(_, record)| {
                    let json_value = record.unwrap().iter().collect::<Value>();
                    ret.push(json_value);
                });
            } else {
                let header = rdr.headers()?.clone();
                rdr.records().enumerate().for_each(|(_, record)| {
                    let json_value = header.iter().zip(record.unwrap().iter()).collect::<Value>();
                    ret.push(json_value);
                });
            }
        }
        Format::Yaml => todo!("yaml format is not implemented"),
    }
    if output == "-" {
        print!("{}", serde_json::to_string_pretty(&ret)?);
    } else {
        std::fs::write(output, serde_json::to_string_pretty(&ret)?)?;
    }
    Ok(())
}
fn input_value_parser(input: &str) -> Result<String, &'static str> {
    if input == "-" || Path::new(input).exists() {
        Ok(input.to_string())
    } else {
        Err("Input file does not exist")
    }
}

#[derive(Debug, Clone)]
enum Format {
    Json,
    Yaml,
}
impl FromStr for Format {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(Format::Json),
            "toml" => Ok(Format::Yaml),
            _ => Err("Invalid format"),
        }
    }
}

fn format_value_parser(input: &str) -> Result<Format, &'static str> {
    match input {
        "json" => Ok(Format::Json),
        "toml" => Ok(Format::Yaml),
        _ => Err("Invalid format"),
    }
}

impl Display for Subcommands {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Subcommands::Csv(_) => write!(f, "csv"),
        }
    }
}
impl Display for Format {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Format::Json => write!(f, "json"),
            Format::Yaml => write!(f, "yaml"),
        }
    }
}
