pub mod python;

use serde::Deserialize;

use crate::utils::{common, constants};

#[derive(Deserialize, PartialEq, Debug)]
pub enum Language {
    Generic,
    Python,
    Rust,
}

#[derive(Deserialize, Debug)]
pub struct LanguageConfig {
    pub language: Language,
    pub init_command: Option<String>,
    pub environment: Option<Environment>,
    #[serde(skip)]
    pub project_path: String,
}

#[derive(Deserialize, Debug)]
pub struct Environment {
    pub name: String,
    pub command: String,
}

impl LanguageConfig {
    pub fn new(project_path: &String) -> Self {
        let config_path = Self::get_config_path(project_path);
        let config: Option<LanguageConfig> = if let Some(config_path) = config_path {
            common::read_toml(config_path)
        } else {
            None
        };

        if let Some(config) = config {
            Self {
                language: config.language,
                init_command: config.init_command,
                environment: config.environment,
                project_path: project_path.clone(),
            }
        } else {
            Self {
                language: Language::Generic,
                init_command: None,
                environment: None,
                project_path: project_path.clone(),
            }
        }
    }

    fn get_config_path(project_path: &String) -> Option<String> {
        let config_path = common::create_path(&[
            project_path,
            &format!(".{}", constants::BRAND),
            &constants::APP,
            "config.toml",
        ]);

        if common::path_exists(&config_path) {
            Some(config_path)
        } else {
            None
        }
    }

    pub fn get_actions(&self) -> String {
        let mut actions: Vec<String> = Vec::new();
        actions.push(format!("cd {}", self.project_path));
        let language_actions = if self.language == Language::Python {
            python::get_actions(self)
        } else {
            vec!["echo 'nothing to do'".to_string()]
        };

        for action in language_actions {
            actions.push(action);
        }

        actions.join("; ")
    }
}
