pub mod config;
pub mod utils;

use std::fs;

fn main() {
    let sys_details = utils::system::SystemDetails::new();
    let config = config::Config::new(&sys_details);

    let mut options: Vec<String> = Vec::new();
    let project_dirs = fs::read_dir(&config.dir).unwrap();
    for dir in project_dirs {
        if config.language_separated {
            for sub_dir in fs::read_dir(dir.unwrap().path().display().to_string()).unwrap() {
                options.push(sub_dir.unwrap().path().display().to_string());
            }
        } else {
            options.push(dir.unwrap().path().display().to_string());
        }
    }

    let chosen_dir = utils::fzf::choose(options);
    let language_config = config::languages::LanguageConfig::new(&chosen_dir);
    let actions = language_config.get_actions();
    println!("{}", actions);
}
