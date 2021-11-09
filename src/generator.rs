use crate::config::Program;
use crate::runner::{CompileError, RunError, RunResult, Runner};
use std::path::PathBuf;

pub struct Generator(Runner);

impl Generator {
    pub fn build(program: Program, config_dir: &PathBuf) -> Result<Generator, CompileError> {
        Ok(Generator(Runner::build(program, config_dir)?))
    }

    pub fn run(&self, index: u32) -> Result<RunResult, RunError> {
        self.0.run_without_input(vec![index.to_string()])
    }
}
