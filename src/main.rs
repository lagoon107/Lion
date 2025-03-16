mod args;
mod lion_config;
mod lion;
mod prelude;

use clap::Parser;
use args::{Args, RootCommand};
use lion::Lion;
use lion_config::Config;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Handle parsed args
    match args.command {
        RootCommand::New {name} => {
            Lion::create_new_project(name)?;
        }
    }

    Ok(())
}
