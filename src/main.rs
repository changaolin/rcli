mod cli;
mod process;
mod utils;
use anyhow::Result;
use process::execute_cmd;
fn main() -> Result<()> {
    execute_cmd()
}
