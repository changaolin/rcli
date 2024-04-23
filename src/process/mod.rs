use crate::{
    cli::{Base64Format, Base64SubCommand, Opts, OutputFormat, Subcommands},
    utils::get_reader,
};
use anyhow::Result;
use clap::Parser;
mod csv_process;
use csv_process::process_csv;
mod genpass_process;
use genpass_process::process_genpass;
mod base64_process;
use base64_process::{process_decode, process_encode};
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
        Subcommands::Base64(base64) => match base64 {
            Base64SubCommand::Encode(opts) => {
                let mut reader = get_reader(&opts.input)?;
                process_encode(&mut reader, opts.format)
            }
            Base64SubCommand::Decode(opts) => {
                let mut reader = get_reader(&opts.input)?;
                process_decode(&mut reader, opts.format)
            }
        },
    }
}
