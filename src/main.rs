mod commands;
mod config;
mod runner;
mod utils;

use clap::Parser;

#[derive(Parser)]
#[clap()]
struct Opts {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    #[clap()]
    Init(Init),
}

#[derive(Parser)]
struct Init;

fn main() {
    let opts: Opts = Opts::parse();
    match opts.subcommand {
        SubCommand::Init(_) => commands::init::init(),
    }
}
