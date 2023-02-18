use anyhow::{Context, Result};
use clap::Parser;
use std::{env, os::unix::process::CommandExt, path::PathBuf, process::Command};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    directory: PathBuf,
    command: String,
    args: Vec<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    println!("{:?}", &cli);
    env::set_current_dir(&cli.directory)
        .with_context(|| format!("Failed to change directory to {}", &cli.directory.display()))?;

    let error = Command::new(&cli.command).args(&cli.args).exec();
    Err(error).with_context(|| {
        format!(
            "Failed to execute {} with arguments {:?} in directory {}",
            &cli.command,
            &cli.args,
            &cli.directory.display()
        )
    })
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
