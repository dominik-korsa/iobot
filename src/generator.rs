use crate::config;
use crate::config::{Files, UnknownExtensionError};
use crate::runner::{CompileError, RunError, RunResult, Runner};
use crate::utils::{get_thread_count, list_config_files, FilesType};
use indicatif::ProgressBar;
use std::path::{Path, PathBuf, StripPrefixError};
use std::sync::{Arc, Mutex};
use std::{fs, io};
use threadpool::ThreadPool;

pub struct Generator(Runner);

impl Generator {
    pub fn build(program: &config::Program, config_dir: &Path) -> Result<Generator, CompileError> {
        Ok(Generator(Runner::build(program, config_dir)?))
    }

    pub fn run(&self, index: u64) -> Result<RunResult, RunError> {
        self.0.run_without_input(vec![index.to_string()])
    }
}

fn copy_inputs(
    files: &config::Files,
    source: &Path,
    generated: &Path,
) -> io::Result<config::Files> {
    let file_paths: Vec<PathBuf> = list_config_files(files, source, FilesType::Input)?;
    let bar = ProgressBar::new(file_paths.len() as u64);
    bar.tick();
    for path in file_paths {
        let target_path = generated.join(path.strip_prefix(source).unwrap()); // TODO: Handle error
        fs::create_dir_all(&target_path.parent().unwrap())?;
        fs::copy(path, &target_path)?;
        bar.inc(1);
    }
    bar.finish();
    Ok(files.clone())
}

#[derive(Debug)]
pub enum GenerateInputsError {
    IO(io::Error),
    Run { index: u64, error: RunError },
    GeneratorCompile(CompileError),
    GeneratorUnknownExtension(UnknownExtensionError),
}

impl From<io::Error> for GenerateInputsError {
    fn from(error: io::Error) -> Self {
        GenerateInputsError::IO(error)
    }
}

impl From<CompileError> for GenerateInputsError {
    fn from(error: CompileError) -> Self {
        GenerateInputsError::GeneratorCompile(error)
    }
}

impl From<UnknownExtensionError> for GenerateInputsError {
    fn from(error: UnknownExtensionError) -> Self {
        GenerateInputsError::GeneratorUnknownExtension(error)
    }
}

fn generate_input(
    generator: &Generator,
    input_path: &Path,
    bar: &ProgressBar,
    ext: &str,
    i: u64,
) -> Result<(), GenerateInputsError> {
    let result = generator
        .run(i)
        .map_err(|error| GenerateInputsError::Run { error, index: i })?;
    fs::write(input_path.join(i.to_string() + ext), result.output)?;
    bar.inc(1);
    Ok(())
}

fn generate_inputs(
    program: &config::Program,
    source: &Path,
    generated: &Path,
    ext: &str,
) -> Result<config::Files, GenerateInputsError> {
    let generator = Arc::new(Generator::build(program, &source)?);

    let count = 100;
    let bar = Arc::new(ProgressBar::new(count));
    bar.tick();
    let input_path_relative = PathBuf::from("in/");
    let input_path = generated.join(&input_path_relative);
    fs::create_dir_all(input_path.as_path())?;

    let error = Arc::new(Mutex::new(None));
    let pool = ThreadPool::new(get_thread_count());
    for i in 0..count {
        let bar = bar.clone();
        let generator = generator.clone();
        let input_path = input_path.clone();
        let error = error.clone();
        let ext = ext.to_string();
        pool.execute(move || {
            if error.lock().unwrap().is_some() {
                return;
            }
            if let Err(err) = generate_input(&generator, &input_path, &bar, &ext, i) {
                *error.lock().unwrap() = Some(err);
            }
        })
    }
    pool.join();
    if pool.panic_count() > 0 {
        panic!()
    }
    if let Some(error) = Arc::try_unwrap(error).unwrap().into_inner().unwrap() {
        return Err(error);
    }
    bar.finish();

    Ok(config::Files {
        path: input_path_relative,
        extensions: Some(vec![ext.to_string()]),
    })
}

pub fn copy_or_generate_input(
    input: &config::InputRef,
    source: &Path,
    generated: &Path,
) -> Result<config::Files, GenerateInputsError> {
    Ok(match input {
        config::InputRef::Files(files) => copy_inputs(files, source, generated)?,
        config::InputRef::Generator { program } => {
            generate_inputs(&program.to_program()?, source, generated, ".in")?
        }
    })
}

#[derive(Debug)]
pub enum GenerateOutputsError {
    IO(io::Error),
    Run(RunError),
    GeneratorCompile(CompileError),
    StripPrefix(StripPrefixError),
}

impl From<io::Error> for GenerateOutputsError {
    fn from(error: io::Error) -> Self {
        GenerateOutputsError::IO(error)
    }
}

impl From<CompileError> for GenerateOutputsError {
    fn from(error: CompileError) -> Self {
        GenerateOutputsError::GeneratorCompile(error)
    }
}

impl From<StripPrefixError> for GenerateOutputsError {
    fn from(error: StripPrefixError) -> Self {
        GenerateOutputsError::StripPrefix(error)
    }
}

impl From<RunError> for GenerateOutputsError {
    fn from(error: RunError) -> Self {
        GenerateOutputsError::Run(error)
    }
}

pub fn generate_output(
    model_runner: &Runner,
    input_path: &Path,
    output_path: &Path,
    bar: &ProgressBar,
) -> Result<(), GenerateOutputsError> {
    let result = model_runner.run(fs::read(input_path)?, vec![])?;
    fs::create_dir_all(output_path.parent().unwrap())?;
    fs::write(output_path, result.output)?;
    bar.inc(1);
    Ok(())
}

pub fn generate_outputs(
    model_runner_program: &config::Program,
    input_config: &config::Files,
    source: &Path,
    generated: &Path,
    ext: &str,
) -> Result<config::Files, GenerateOutputsError> {
    let model_runner = Arc::from(Runner::build(model_runner_program, source)?);
    let input_files = list_config_files(input_config, generated, FilesType::Input)?;
    let bar = ProgressBar::new(input_files.len() as u64);
    bar.tick();
    let error = Arc::new(Mutex::new(None));
    let pool = ThreadPool::new(get_thread_count());
    let output_path_relative = PathBuf::from("out/");
    let output_path = generated.join(&output_path_relative);
    for input_file in input_files {
        let output_file = output_path.join(
            input_file
                .strip_prefix(generated.join(&input_config.path))?
                .with_extension(ext.strip_prefix(".").unwrap()),
        );
        let model_runner = model_runner.clone();
        let error = error.clone();
        let bar = bar.clone();
        pool.execute(move || {
            if error.lock().unwrap().is_some() {
                return;
            }
            if let Err(err) = generate_output(&model_runner, &input_file, &output_file, &bar) {
                *error.lock().unwrap() = Some(err);
            }
        })
    }
    pool.join();
    if pool.panic_count() > 0 {
        panic!()
    }
    if let Some(error) = Arc::try_unwrap(error).unwrap().into_inner().unwrap() {
        return Err(error);
    }
    bar.finish();
    Ok(Files {
        path: output_path_relative,
        extensions: Some(vec![ext.to_string()]),
    })
}
