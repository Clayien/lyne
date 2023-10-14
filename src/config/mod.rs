pub mod languages;

use serde::Deserialize;
use std::env;

use crate::utils::constants::SystemDetails;

use super::utils::{common, constants};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub dir: String,
    #[serde(default = "language_separated")]
    pub language_separated: bool,
}

fn language_separated() -> bool {
    false
}

impl Config {
    pub fn new(sys_details: &SystemDetails) -> Self {
        let config_file_path: Option<String> = Self::get_config_path(sys_details);
        let main: Option<Self> = if let Some(config_file_path) = config_file_path {
            common::read_toml(config_file_path)
        } else {
            None
        };

        if let Some(main) = main {
            main
        } else {
            Self {
                dir: common::create_path(&[&sys_details.home, "Documents"]),
                language_separated: language_separated(),
            }
        }
    }

    fn get_config_path(sys_details: &SystemDetails) -> Option<String> {
        if sys_details.os == constants::OS::Linux {
            let xdg_config_path = common::create_path(&[
                env::var("XDG_CONFIG_HOME").unwrap().as_str(),
                &constants::BRAND,
                &constants::APP,
                "config.toml",
            ]);

            let config_path = common::create_path(&[
                &sys_details.home,
                ".config",
                &constants::BRAND,
                &constants::APP,
                "config.toml",
            ]);

            if common::path_exists(&xdg_config_path) {
                Some(xdg_config_path)
            } else if common::path_exists(&config_path) {
                Some(config_path)
            } else {
                None
            }
        } else {
            None
        }
    }
}
