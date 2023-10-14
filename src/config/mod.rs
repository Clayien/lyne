pub mod languages;

use serde::Deserialize;

use super::utils::{common, system::SystemDetails};

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
        let main: Option<Self> = if let Some(config_file_path) = &sys_details.config_path {
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
}
