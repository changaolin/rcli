mod cli;
mod process;
mod utils;
use anyhow::Result;
use process::execute_cmd;
use tracing::info;
fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    info!("Starting the program");
    let ret = execute_cmd();
    info!("Program finished");
    ret
}
