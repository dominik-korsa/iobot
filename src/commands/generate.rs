use crate::config::parse_config;
use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Params {
    pub source: PathBuf,
    pub generated: PathBuf,
}

pub fn run(params: Params) {
    let source_config_path = params.source.join("iobot.yaml");
    let config =
        parse_config(&fs::read(source_config_path).unwrap()).expect("Failed to parse config");
    println!("{:#?}", config)
}
