use clap::Parser;
#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16, value_parser = parse_length)]
    pub length: u8,

    #[arg(long, default_value_t = true)]
    pub uppercase: bool,

    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    #[arg(long, default_value_t = true)]
    pub number: bool,

    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}
fn parse_length(s: &str) -> Result<u8, &'static str> {
    match s.parse() {
        Ok(n) if (4..=64).contains(&n) => Ok(n),
        _ => Err("Invalid length"),
    }
}
