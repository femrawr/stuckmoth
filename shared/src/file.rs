use walkdir::WalkDir;
use std::path::PathBuf;
use std::io::Write;
use std::fs::{self, File};

pub fn delete_file(file: &PathBuf) {
    let handle = File::create(file);

    let mut the_handle = match handle {
        Ok(res) => res,
        Err(_) => return
    };

    match the_handle.write_all("".as_bytes()) {
        Ok(_) => {},
        Err(_) => return
    }

    match fs::remove_file(file) {
        Ok(_) => {},
        Err(_) => return
    }
}

pub fn get_files(dir: &PathBuf) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|a| a.ok())
        .filter(|a| a.path().is_file())
        .map(|a| a.path().to_path_buf())
        .collect()
}