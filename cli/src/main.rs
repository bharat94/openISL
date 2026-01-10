use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(name = "openisl")]
#[command(about = "Interactive Smart Log - Smart git operations")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Help,
}

fn main() -> Result<()> {
    let _cli = Cli::parse();
    todo!("Implement CLI commands")
}
