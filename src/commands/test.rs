use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Params {
    pub tests: PathBuf,
    pub program: PathBuf,
}

// pub fn run(params: Params) {
//     let source_config_path = params.source.join("iobot.yaml");
//     let config =
//         parse_config(&fs::read(source_config_path).unwrap()).expect("Failed to parse config");
//     println!("{:#?}", config);
//     match config {
//         Config::OutputFiles(_) => {}
//         Config::ModelProgram(config) => {
//             let model_runner =
//                 make_runner(config.model_program.to_program().unwrap(), &params.source).unwrap();
//         }
//         Config::JustVerifier(_) => {}
//     }
// }
