mod cli;
mod process;
use process::execute_cmd;
fn main() -> anyhow::Result<()> {
    execute_cmd()
}
