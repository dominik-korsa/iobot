use crate::config;
use std::path::{Path, PathBuf};
use std::str::{from_utf8, Utf8Error};
use std::{cmp, io};

pub fn to_lines(output: &Vec<u8>) -> Result<Vec<String>, Utf8Error> {
    let mut lines: Vec<String> = from_utf8(output)?
        .lines()
        .map(|x| x.trim_end().to_string())
        .collect();
    while lines.ends_with(&["".to_string()]) {
        lines.pop();
    }
    Ok(lines)
}

pub fn list_files(path: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = vec![];
    for entry in path.read_dir()? {
        let entry_path = entry?.path();
        if entry_path.is_dir() {
            files.extend(list_files(entry_path.as_path())?)
        } else {
            files.push(entry_path);
        }
    }
    Ok(files)
}

#[derive(Debug, Clone, Copy)]
pub enum FilesType {
    Input,
    Output,
}

pub fn list_config_files(
    files_config: &config::Files,
    base: &Path,
    files_type: FilesType,
) -> io::Result<Vec<PathBuf>> {
    Ok(list_files(&base.join(&files_config.path))?
        .into_iter()
        .filter(|file| {
            let ext = match file.extension().and_then(|x| x.to_str()) {
                None => "".to_string(),
                Some(ext) => ".".to_string() + ext,
            };
            files_config
                .extensions
                .clone()
                .unwrap_or(match files_type {
                    FilesType::Input => vec![".in".to_string()],
                    FilesType::Output => vec![".out".to_string()],
                })
                .contains(&ext.to_string())
        })
        .collect())
}

pub fn get_thread_count() -> usize {
    cmp::max(num_cpus::get() - 1, 2)
}
