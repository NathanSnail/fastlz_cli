use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Input compressed file
    name: PathBuf,

    /// Sets a custom config file
    #[arg(short, long, default_value_t = 0)]
    skip: usize,

    #[arg(short, long)]
    output: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    println!(
        "name: {:?}, skip: {:?}, output: {:?}",
        cli.name, cli.skip, cli.output
    );
}
