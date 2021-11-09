mod commands;
mod config;
mod generator;
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
    Init(commands::init::Params),
    #[clap()]
    Generate(commands::generate::Params),
    // #[clap()]
    // Test(commands::test::Params),
}

fn main() {
    let opts: Opts = Opts::parse();
    match opts.subcommand {
        SubCommand::Init(_) => commands::init::run(),
        SubCommand::Generate(params) => commands::generate::run(params),
        // SubCommand::Test(params) => commands::test::run(params),
    }
}
