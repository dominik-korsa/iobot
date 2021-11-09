use crate::config::Program;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};
use std::{env, fs, io};
use uuid::Uuid;

struct Compiled {
    pub target: PathBuf,
}

impl Compiled {
    fn delete_file(&self) -> Result<(), trash::Error> {
        if self.target.exists() {
            fs::remove_file(&self.target);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum CompileError {
    IO(io::Error),
    Unsuccessful,
    Extension,
}

impl From<io::Error> for CompileError {
    fn from(item: io::Error) -> Self {
        CompileError::IO(item)
    }
}

fn replace_target(template: &str, target: &PathBuf) -> String {
    template.replace("{target}", target.to_str().unwrap())
}

fn compile(
    command: &str,
    args: &[String],
    config_dir: &PathBuf,
    ext: &str,
) -> Result<Compiled, CompileError> {
    if !ext.is_empty() && !ext.starts_with(".") {
        return Err(CompileError::Extension);
    }
    let build_folder = env::temp_dir().join("iobot/build");
    fs::create_dir_all(&build_folder).unwrap();
    let target = build_folder.join(Uuid::new_v4().to_string() + ext);
    let mut compile_command = Command::new(replace_target(command, &target));
    compile_command.args(args.into_iter().map(|x| replace_target(x, &target)));
    compile_command.current_dir(config_dir);
    if !compile_command.spawn()?.wait()?.success() {
        return Err(CompileError::Unsuccessful);
    }
    Ok(Compiled { target })
}

pub struct RunResult {
    pub output: Vec<u8>,
}

impl RunResult {
    pub fn from_output(output: Output) -> Result<RunResult, RunError> {
        if !output.status.success() {
            return Err(RunError(output.status.code()));
        }
        return Ok(RunResult {
            output: output.stdout,
        });
    }
}

pub struct RunError(pub Option<i32>);

pub struct Runner {
    config_dir: PathBuf,
    command: String,
    args: Vec<String>,
    compiled: Option<Compiled>,
}

impl Runner {
    pub fn build(program: Program, config_dir: &PathBuf) -> Result<Runner, CompileError> {
        let (command, args, compiled): (String, Vec<String>, Option<Compiled>) = match program {
            Program::GPP {
                path,
                compiler_args,
            } => {
                let mut args = compiler_args.unwrap_or(vec![]);
                args.extend([
                    path.into_os_string().into_string().unwrap(),
                    "-o".to_string(),
                    "{target}".to_string(),
                ]);
                // TODO: Add config for compiler
                let compiled = compile("g++".as_ref(), &args, config_dir, ".exe")?;
                (
                    compiled.target.to_str().unwrap().to_string(),
                    vec![],
                    Some(compiled),
                )
            }
            Program::Python { path } => {
                // TODO: Add config for python command
                (
                    "python3".to_string(),
                    vec![path.to_str().unwrap().to_string()],
                    None,
                )
            }
            Program::Command { run } => (run.command, run.args.unwrap_or(vec![]), None),
            Program::Compiled {
                compile: compile_config,
                extension,
                run,
            } => {
                let compiled = compile(
                    &compile_config.command,
                    &compile_config.args.unwrap_or(vec![]),
                    config_dir,
                    &extension,
                )?;
                (
                    replace_target(&run.command, &compiled.target),
                    run.args
                        .unwrap_or(vec![])
                        .into_iter()
                        .map(|x| replace_target(&x, &compiled.target))
                        .collect(),
                    Some(compiled),
                )
            }
        };
        Ok(Runner {
            compiled,
            command,
            args,
            config_dir: config_dir.clone(),
        })
    }

    fn get_command(&self) -> Command {
        let mut command = Command::new(&self.command);
        command.args(&self.args);
        command.current_dir(&self.config_dir);
        command
    }

    pub fn run(&self, input: Vec<u8>, args: Vec<String>) -> Result<RunResult, RunError> {
        let mut command = self.get_command();
        command.args(args);
        let child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|error| RunError(None))?;
        child
            .stdin
            .as_ref()
            .unwrap()
            .write_all(&input)
            .map_err(|error| RunError(None))?;
        let output = child.wait_with_output().map_err(|error| RunError(None))?;
        return RunResult::from_output(output);
    }

    pub fn run_without_input(&self, args: Vec<String>) -> Result<RunResult, RunError> {
        let mut command = self.get_command();
        command.args(args);
        let output = command.output().map_err(|error| RunError(None))?;
        return RunResult::from_output(output);
    }
}

impl Drop for Runner {
    fn drop(&mut self) {
        if let Some(compiled) = &self.compiled {
            compiled.delete_file().unwrap();
        }
    }
}
