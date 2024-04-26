mod cli;
mod process;
mod utils;
use anyhow::Result;
use process::execute_cmd;
use tracing::info;
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    info!("Starting the program");
    execute_cmd().await?;
    info!("Program finished");
    Ok(())
}
