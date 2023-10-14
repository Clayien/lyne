use serde_json;
use std::fs;
use std::path::{Path, PathBuf};
use toml;

pub fn read_file(path: String) -> Result<String, std::io::Error> {
    let file_path = Path::new(&path);

    fs::read_to_string(file_path)
}

pub fn create_path(args: &[&str]) -> String {
    let mut path = PathBuf::new();
    for arg in args {
        path = path.join(arg);
    }

    path.to_string_lossy().to_string()
}

pub fn path_exists(path: &String) -> bool {
    Path::new(path).exists()
}

pub fn parse_toml<T>(data: String) -> Option<T>
where
    T: serde::de::DeserializeOwned,
{
    if let Ok(main) = toml::from_str::<T>(&data) {
        Some(main)
    } else {
        None
    }
}

pub fn read_toml<T>(file_path: String) -> Option<T>
where
    T: serde::de::DeserializeOwned + std::fmt::Debug,
{
    if let Ok(file_data) = read_file(file_path) {
        if let Some(main) = parse_toml(file_data) {
            return Some(main);
        } else {
            return None;
        }
    }
    None
}

pub fn parse_json<T>(data: String) -> Option<T>
where
    T: serde::de::DeserializeOwned,
{
    if let Ok(main) = serde_json::from_str::<T>(&data) {
        Some(main)
    } else {
        None
    }
}
