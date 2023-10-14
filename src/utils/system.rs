use std::env;

use crate::utils::{common, constants};

#[derive(PartialEq)]
pub enum OS {
    Linux,
    Windows,
    MacOS,
}

pub struct SystemDetails {
    pub os: OS,
    pub home: String,
    pub config_path: Option<String>,
}

impl SystemDetails {
    pub fn new() -> Self {
        let os = match env::consts::OS {
            "macos" => OS::Linux,
            "linux" => OS::Linux,
            "windows" => OS::Windows,
            _ => panic!("Unsupported OS"),
        };
        let home = Self::get_home(&os);
        let config_path = Self::get_config_path(&os, &home);

        Self {
            os,
            home,
            config_path,
        }
    }

    fn get_home(os: &OS) -> String {
        match os {
            OS::Windows => env::var("USERPROFILE").unwrap(),
            _ => env::var("HOME").unwrap(),
        }
    }

    fn get_config_path(os: &OS, home: &String) -> Option<String> {
        let config_suffix = common::create_path(&[constants::BRAND, constants::APP, "config.toml"]);
        match os {
            OS::Linux => {
                let xdg_config_path = common::create_path(&[
                    env::var("XDG_CONFIG_HOME").unwrap().as_str(),
                    &config_suffix,
                ]);

                let config_path = common::create_path(&[home, ".config", &config_suffix]);

                if common::path_exists(&xdg_config_path) {
                    Some(xdg_config_path)
                } else if common::path_exists(&config_path) {
                    Some(config_path)
                } else {
                    None
                }
            }
            OS::Windows => {
                let windows_config_path =
                    common::create_path(&[env::var("APPDATA").unwrap().as_str(), &config_suffix]);
                if common::path_exists(&windows_config_path) {
                    Some(windows_config_path)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl Default for SystemDetails {
    fn default() -> Self {
        Self::new()
    }
}
