use crate::commands::get_theme;
use crate::config::{Config, FilesInput, GenerableConfig, Input, JustVerifier, OutputFiles};
use crate::generator::{copy_or_generate_input, generate_outputs};
use clap::Parser;
use console::style;
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
    let config = Config::parse_bytes(&fs::read(source_config_path).unwrap())
        .expect("Failed to parse config");
    let config = config.to_generable().expect("Already generated");
    let input_files_config =
        copy_or_generate_input(&config.get_input(), &params.source, &params.generated).unwrap();
    let generated_config = match config {
        GenerableConfig::ModelProgram(config) => {
            let output_files_config = generate_outputs(
                &config.model_program.to_program().unwrap(),
                &input_files_config,
                &params.source,
                &params.generated,
                ".out",
            )
            .unwrap();
            Config::OutputFiles(OutputFiles {
                input: FilesInput::Files(input_files_config),
                output_files: output_files_config,
                verifier: config.verifier,
            })
        }
        GenerableConfig::JustVerifier(config) => Config::JustVerifier(JustVerifier {
            input: Input::Files(input_files_config),
            verifier: config.verifier,
        }),
    };
    let generated_config_path = params.generated.join("./iobot.yaml");
    let yaml = serde_yaml::to_string(&generated_config).unwrap();
    fs::write(&generated_config_path, &yaml).unwrap();
    println!("{}", style(format!("Finished generating")).green());
    println!("{}", yaml);
}
