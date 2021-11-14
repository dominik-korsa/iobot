use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Files {
    pub path: PathBuf,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ProgramOrShorthand {
    Shorthand(PathBuf),
    Value(Program),
}

impl ProgramOrShorthand {
    pub fn to_program(&self) -> Result<Program, UnknownExtensionError> {
        match self {
            ProgramOrShorthand::Value(program) => Ok(program.clone()),
            ProgramOrShorthand::Shorthand(shorthand) => {
                let ext = shorthand.extension().and_then(|x| x.to_str());
                match ext {
                    None => Err(UnknownExtensionError),
                    Some(ext) => match ext {
                        "cpp" => Ok(Program::GPP {
                            path: shorthand.clone(),
                            compiler_args: None,
                        }),
                        "py" => Ok(Program::Python {
                            path: shorthand.clone(),
                        }),
                        _ => Err(UnknownExtensionError),
                    },
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnknownExtensionError;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Command {
    pub command: String,
    pub args: Option<Vec<String>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
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

impl Input {
    pub fn as_input_ref(&self) -> InputRef {
        match self {
            Input::Files(files) => InputRef::Files(files),
            Input::Generator { program } => InputRef::Generator { program },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum FilesInput {
    Files(Files),
}

impl FilesInput {
    pub fn as_files(&self) -> &Files {
        match self {
            FilesInput::Files(files) => files,
        }
    }

    pub fn as_input_ref(&self) -> InputRef {
        match self {
            FilesInput::Files(files) => InputRef::Files(files),
        }
    }
}

pub enum InputRef<'a> {
    Files(&'a Files),
    Generator { program: &'a ProgramOrShorthand },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModelProgram {
    pub input: Input,
    pub model_program: ProgramOrShorthand,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verifier: Option<ProgramOrShorthand>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OutputFiles {
    pub input: FilesInput,
    pub output_files: Files,
    #[serde(skip_serializing_if = "Option::is_none")]
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

pub struct JustVerifierGenerator {
    pub input: ProgramOrShorthand,
    pub verifier: ProgramOrShorthand,
}

pub enum GenerableConfig {
    ModelProgram(ModelProgram),
    JustVerifier(JustVerifierGenerator),
}

impl Config {
    pub fn parse_bytes(file: &Vec<u8>) -> serde_yaml::Result<Config> {
        serde_yaml::from_slice(&file)
    }

    pub fn get_input(&self) -> InputRef {
        match self {
            Config::ModelProgram(config) => config.input.as_input_ref(),
            Config::OutputFiles(config) => config.input.as_input_ref(),
            Config::JustVerifier(config) => config.input.as_input_ref(),
        }
    }

    pub fn to_generable(self) -> Option<GenerableConfig> {
        match self {
            Config::ModelProgram(config) => Some(GenerableConfig::ModelProgram(config)),
            Config::OutputFiles(_) => None,
            Config::JustVerifier(config) => match config.input {
                Input::Files(_) => None,
                Input::Generator { program } => {
                    Some(GenerableConfig::JustVerifier(JustVerifierGenerator {
                        input: program,
                        verifier: config.verifier,
                    }))
                }
            },
        }
    }
}

impl GenerableConfig {
    pub fn get_input(&self) -> InputRef {
        match self {
            GenerableConfig::ModelProgram(config) => config.input.as_input_ref(),
            GenerableConfig::JustVerifier(config) => InputRef::Generator {
                program: &config.input,
            },
        }
    }
}
