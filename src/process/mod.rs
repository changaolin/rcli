use std::fs;

use crate::{
    cli::{
        Base64Format, Base64SubCommand, HttpSubCommand, Opts, OutputFormat, Subcommands,
        TextSignFormat, TextSubCommand,
    },
    process::text_process::process_text_sign,
    utils::{get_content, get_reader},
};
use anyhow::{Ok, Result};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::Parser;
mod csv_process;
use csv_process::process_csv;
mod genpass_process;
use genpass_process::process_genpass;
mod base64_process;
use base64_process::{process_decode, process_encode};

use self::text_process::{process_text_key_generate, process_text_verify};
mod http_process;
mod text_process;
use http_process::process_http_serve;
/// csv --input <input> --output <output> --delimiter <delimiter> --format json --no-header
pub async fn execute_cmd() -> Result<()> {
    let cli = Opts::parse();
    match cli.cmd {
        Subcommands::Csv(csv) => process_csv(&csv.input, &csv.output, csv.format, csv.no_header),
        Subcommands::GenPass(genpass) => {
            let _ = process_genpass(
                genpass.length,
                genpass.uppercase,
                genpass.lowercase,
                genpass.number,
                genpass.symbol,
            );
            Ok(())
        }
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
        Subcommands::Text(cmd) => match cmd {
            TextSubCommand::Sign(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let sig = process_text_sign(&mut reader, &key, opts.format)?;
                // base64 output
                let encoded = URL_SAFE_NO_PAD.encode(sig);
                println!("{}", encoded);
                Ok(())
            }
            TextSubCommand::Verify(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let decoded = URL_SAFE_NO_PAD.decode(&opts.sig)?;
                let verified = process_text_verify(&mut reader, &key, &decoded, opts.format)?;
                if verified {
                    println!("✓ Signature verified");
                } else {
                    println!("⚠ Signature not verified");
                }
                Ok(())
            }
            TextSubCommand::Generate(opts) => {
                let key = process_text_key_generate(opts.format)?;
                for (k, v) in key {
                    fs::write(opts.output_path.join(k), v)?;
                }
                Ok(())
            }
        },
        Subcommands::Http(cmd) => match cmd {
            HttpSubCommand::Serve(opts) => {
                process_http_serve(opts.dir, opts.port).await?;
                Ok(())
            }
        },
    }
}
