use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Files {
    pub path: PathBuf,
    pub extensions: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ProgramOrShorthand {
    Shorthand(String),
    Value(Program),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "mode", rename_all = "camelCase")]
pub enum Program {
    #[serde(rename = "g++")]
    GPP {
        path: PathBuf,
        #[serde(rename = "compilerArgs")]
        compiler_args: Option<Vec<String>>,
    },
    Python {
        path: PathBuf,
    },
    Command {
        command: String,
        args: Option<Vec<String>>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Input {
    Files(Files),
    Generator(ProgramOrShorthand),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModelProgram {
    input: Files,
    model_program: ProgramOrShorthand,
    verifier: Option<ProgramOrShorthand>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OutputFiles {
    input: Input,
    output_files: Files,
    verifier: Option<ProgramOrShorthand>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JustVerifier {
    input: Input,
    verifier: ProgramOrShorthand,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Config {
    ModelProgram(ModelProgram),
    OutputFiles(OutputFiles),
    JustVerifier(JustVerifier),
}

pub fn parse_config(file: &Vec<u8>) -> serde_yaml::Result<Config> {
    serde_yaml::from_slice(&file)
}
