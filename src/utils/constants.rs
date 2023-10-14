use std::env;

pub const BRAND: &str = "clayien";
pub const APP: &str = "lyne";

#[derive(PartialEq)]
pub enum OS {
    Linux,
    Windows,
}

pub struct SystemDetails {
    pub os: OS,
    pub home: String,
}

impl SystemDetails {
    pub fn new() -> Self {
        let os = match env::consts::OS {
            "macos" => OS::Linux,
            "linux" => OS::Linux,
            "windows" => OS::Windows,
            _ => panic!("Unsupported OS"),
        };
        let home = env::var("HOME").unwrap();

        Self { os, home }
    }
}

impl Default for SystemDetails {
    fn default() -> Self {
        Self::new()
    }
}
