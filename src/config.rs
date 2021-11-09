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
    Shorthand(PathBuf),
    Value(Program),
}

impl ProgramOrShorthand {
    pub fn to_program(self) -> Result<Program, UnknownExtensionError> {
        match self {
            ProgramOrShorthand::Value(program) => Ok(program),
            ProgramOrShorthand::Shorthand(shorthand) => {
                let ext = shorthand.extension().and_then(|x| x.to_str());
                match ext {
                    None => Err(UnknownExtensionError),
                    Some(ext) => match ext {
                        "cpp" => Ok(Program::GPP {
                            path: shorthand,
                            compiler_args: None,
                        }),
                        "py" => Ok(Program::Python { path: shorthand }),
                        _ => Err(UnknownExtensionError),
                    },
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnknownExtensionError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    pub command: String,
    pub args: Option<Vec<String>>,
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
        run: Command,
    },
    Compiled {
        compile: Command,
        extension: String,
        run: Command,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Input {
    Files(Files),
    Generator { program: ProgramOrShorthand },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModelProgram {
    pub input: Input,
    pub model_program: ProgramOrShorthand,
    pub verifier: Option<ProgramOrShorthand>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OutputFiles {
    pub input: Files,
    pub output_files: Files,
    pub verifier: Option<ProgramOrShorthand>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JustVerifier {
    pub input: Input,
    pub verifier: ProgramOrShorthand,
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
