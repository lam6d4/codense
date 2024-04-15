
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename = "file")]
pub struct File {
    #[serde(rename = "$value")]
    content: String,
    #[serde(rename = "@name")]
    name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename = "directory")]
pub struct Directory {
    #[serde(rename = "file")]
    files: Vec<File>,
    #[serde(rename = "directory")]
    subdirectories: Vec<Directory>,
    #[serde(rename = "@name")]
    name: String,
}

pub fn read_directory(dir_path: &Path) -> Directory {
    let mut files = Vec::new();
    let mut subdirectories = Vec::new();

    for entry in WalkDir::new(dir_path) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            let content = fs::read_to_string(path).unwrap();
            let file_entry = File {
                content,
                name: path.file_name().unwrap().to_string_lossy().into(),
            };
            files.push(file_entry);
        } else if path.is_dir() && path != dir_path {
            let subdirectory_entry = read_directory(path);
            subdirectories.push(subdirectory_entry);
        }
    }

    Directory {
        files,
        subdirectories,
        name: dir_path.file_name().unwrap().to_string_lossy().into(),
    }
}
