use serde::Deserialize;
use serde_json;
use std::fs;
use std::path::Path;
use toml;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub project_dir: Option<String>,
    pub language_seperated: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct LanguageConfig {
    pub language: Option<String>,
    pub env: Option<Env>,
    pub init: Option<Init>,
}

#[derive(Deserialize, Debug)]
pub struct Env {
    pub command: String,
    pub env: String,
}

#[derive(Deserialize, Debug)]
pub struct Init {
    pub command: String,
}

#[derive(Deserialize, Debug)]
pub struct CondaEnvs {
    pub envs: Option<Vec<String>>,
}

fn read_config_file(path: String) -> String {
    let file_path = Path::new(&path);

    fs::read_to_string(file_path).unwrap()
}

impl Config {
    pub fn new() -> Self {
        Self {
            project_dir: Some("/home/symph/Documents".to_string()),
            language_seperated: Some(true),
        }
    }

    pub fn parse_config(&mut self) {
        if let Ok(user_values) = toml::from_str::<Config>(&read_config_file(
            "/home/symph/.config/clayien/lyne/config.toml".to_string(),
        )) {
            if let Some(project_dir) = user_values.project_dir {
                self.project_dir = Some(project_dir);
            }
            if let Some(language_seperated) = user_values.language_seperated {
                self.language_seperated = Some(language_seperated);
            }
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageConfig {
    pub fn new() -> Self {
        Self {
            language: None,
            env: None,
            init: None,
        }
    }

    pub fn parse_config(&mut self, path: String) {
        if let Ok(user_values) = toml::from_str::<LanguageConfig>(&read_config_file(path)) {
            if let Some(language) = user_values.language {
                self.language = Some(language);
            }
            if let Some(env) = user_values.env {
                self.env = Some(env);
            }
            if let Some(init) = user_values.init {
                self.init = Some(init);
            }
        }
    }
}

impl Default for LanguageConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl CondaEnvs {
    pub fn new() -> Self {
        Self { envs: None }
    }

    pub fn parse_config(&mut self, data: String) {
        if let Ok(info) = serde_json::from_str::<CondaEnvs>(&data) {
            if let Some(envs) = info.envs {
                self.envs = Some(envs);
            }
        }
    }
}

impl Default for CondaEnvs {
    fn default() -> Self {
        Self::new()
    }
}
