mod commands;
mod config;
mod init_common;
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
    InitPrototype(InitPrototype),
}

#[derive(Parser)]
struct Init;

#[derive(Parser)]
struct InitPrototype;

fn main() {
    let opts: Opts = Opts::parse();
    match opts.subcommand {
        SubCommand::Init(_) => commands::init::run(),
        SubCommand::InitPrototype(_) => commands::init_prototype::run(),
    }
}
