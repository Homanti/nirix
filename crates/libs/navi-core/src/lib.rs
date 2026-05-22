use std::{fs};
use std::path::PathBuf;

pub fn list_files(dir: &PathBuf) -> Vec<PathBuf> {
    match fs::read_dir(dir) {
        Ok(dir) => {dir.filter_map(|entry| entry.ok().map(|e| e.path())).collect()}
        _ => vec![]
    }
}