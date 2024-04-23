mod cli;
mod process;
use anyhow::Result;
use process::execute_cmd;
fn main() -> Result<()> {
    execute_cmd()
}
