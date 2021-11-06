use crate::runner::program::Program;
use crate::utils::to_lines;
use std::io::Write;
use std::process::{Command, Stdio};

pub mod program {
    use std::path::PathBuf;

    pub struct BinaryProgram {
        pub path: PathBuf,
    }

    pub enum Program {
        Binary(BinaryProgram),
    }
}

pub struct ResultSuccess {
    pub output: Vec<String>,
}

pub enum RunResult {
    Success(ResultSuccess),
    NotFound,
}

pub fn run(program: Program, input: Vec<u8>) -> RunResult {
    let mut command = match program {
        Program::Binary(program) => Command::new(program.path),
    };
    let child = match command.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn() {
        Ok(child) => child,
        Err(_) => return RunResult::NotFound,
    };
    child.stdin.as_ref().unwrap().write_all(&input).unwrap(); // TODO: Remove write all unwrap
    let output = child.wait_with_output().unwrap(); // TODO: Remove unwrap
    return RunResult::Success(ResultSuccess {
        output: to_lines(&output.stdout).expect("Failed to split lines"),
    });
}
