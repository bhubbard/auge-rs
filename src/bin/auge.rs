use auge_rs::cli::{run_cli, Cli};
use clap::Parser;
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli = Cli::parse();
    if let Err(e) = run_cli(cli) {
        eprintln!("auge: {e}");
        return ExitCode::from(e.exit_code() as u8);
    }
    ExitCode::SUCCESS
}
