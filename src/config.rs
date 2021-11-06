use std::path::PathBuf;

pub struct Files {
    pub path: PathBuf,
    pub extensions: Vec<String>,
}

pub enum Program {
    GPP {
        path: PathBuf,
        compiler_args: Vec<String>,
    },
    Python {
        path: PathBuf,
    },
    Command {
        command: String,
        args: Vec<String>,
    },
}

pub enum Output {
    Files(Files),
    Generator(Program),
}

pub enum Config {
    FilesInput {
        input: Files,
        output: Output,
    },
    FilesInputVerifier {
        input: Files,
        output: Option<Output>,
    },
}
