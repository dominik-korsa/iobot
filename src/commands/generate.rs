use crate::commands::get_theme;
use crate::config::{parse_config, Config};
use crate::runner::Runner;
use clap::Parser;
use dialoguer::Confirm;
use std::fs::DirEntry;
use std::path::PathBuf;
use std::{fs, io};

#[derive(Parser)]
pub struct Params {
    pub source: PathBuf,
    pub generated: PathBuf,
}

pub fn run(params: Params) {
    let theme = get_theme();

    if params.source.is_file() {
        panic!("Source path should be a directory")
    }
    if params.generated.is_file() {
        panic!("Generated path should be a directory")
    }
    fs::create_dir_all(&params.generated).unwrap();
    let dir_contents: Vec<io::Result<DirEntry>> = params.generated.read_dir().unwrap().collect();
    if !dir_contents.is_empty() {
        let empty_dir = Confirm::with_theme(&theme)
            .with_prompt("Generated directory is not empty. Do you want to remove its contents?")
            .interact()
            .unwrap();
        if !empty_dir {
            return;
        }
        trash::delete_all(
            dir_contents
                .into_iter()
                .map(|result| result.unwrap().path()),
        )
        .unwrap();
    }
    let source_config_path = params.source.join("iobot.yaml");
    let config =
        parse_config(&fs::read(source_config_path).unwrap()).expect("Failed to parse config");
    println!("{:#?}", config);
    match config {
        Config::OutputFiles(_) => {}
        Config::ModelProgram(config) => {
            let model_runner =
                Runner::build(config.model_program.to_program().unwrap(), &params.source).unwrap();
        }
        Config::JustVerifier(_) => {}
    }
}
