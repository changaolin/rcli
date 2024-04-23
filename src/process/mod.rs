use crate::cli::{Opts, OutputFormat, Subcommands};
use anyhow::Result;
use clap::Parser;
mod csv_process;
use csv_process::process_csv;
mod genpass_process;
use genpass_process::process_genpass;
/// csv --input <input> --output <output> --delimiter <delimiter> --format json --no-header
pub fn execute_cmd() -> Result<()> {
    let cli = Opts::parse();
    match cli.cmd {
        Subcommands::Csv(csv) => process_csv(&csv.input, &csv.output, csv.format, csv.no_header),
        Subcommands::GenPass(genpass) => process_genpass(
            genpass.length,
            genpass.uppercase,
            genpass.lowercase,
            genpass.number,
            genpass.symbol,
        ),
    }
}
