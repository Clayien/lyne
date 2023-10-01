use serde::Deserialize;
use std::fs;
use std::path::Path;
use toml;

#[derive(Deserialize)]
pub struct Config {
    pub main: Main,
}

#[derive(Deserialize)]
pub struct Main {
    pub project_dir: String,
    pub language_seperated: bool,
    pub random: Option<bool>,
}

fn read_config_file() -> String {
    let file_path = Path::new("/home/symph/.config/clayien/lyne/config.toml");

    fs::read_to_string(file_path).unwrap()
}

pub fn parse() -> Config {
    toml::from_str(read_config_file().as_str()).unwrap()
}
