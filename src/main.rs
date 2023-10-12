pub mod fzf;
pub mod parser;

use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    // let args = Args::parse();
    //
    // for _ in 0..args.count {
    //     println!("Hello {}!", args.name)
    // }
    let mut config = parser::Config::new();
    config.parse_config();

    let mut options: Vec<String> = Vec::new();
    let project_dirs = fs::read_dir(config.project_dir.unwrap()).unwrap();
    for dir in project_dirs {
        if config.language_seperated.unwrap() {
            for sub_dir in fs::read_dir(dir.unwrap().path().display().to_string()).unwrap() {
                options.push(sub_dir.unwrap().path().display().to_string());
            }
        } else {
            options.push(dir.unwrap().path().display().to_string());
        }
    }

    let mut shell_output = Vec::new();
    let chosen_dir = fzf::choose(options);
    let language_config_path = Path::new(&chosen_dir)
        .join(".clayien")
        .join("lyne")
        .join("config.toml");

    shell_output.push(format!("cd {chosen_dir}"));
    if language_config_path.exists() {
        let mut language_config = parser::LanguageConfig::new();
        language_config.parse_config(language_config_path.display().to_string());

        let mut conda_envs = parser::CondaEnvs::new();
        let conda_info_output = Command::new("conda")
            .args(["info", "--json"])
            .output()
            .unwrap();

        let output_string = if conda_info_output.status.success() {
            String::from_utf8(conda_info_output.stdout).unwrap()
        } else {
            "".to_string()
        };
        conda_envs.parse_config(output_string);
        if conda_envs
            .envs
            .unwrap()
            .iter()
            .any(|e| e.contains(&language_config.env.as_ref().unwrap().env))
        {
            shell_output.push(format!(
                "conda activate {}",
                language_config.env.unwrap().env
            ));
        } else {
            let mut conda_create = Command::new("conda")
                .args([
                    "create",
                    format!("--name={}", language_config.env.as_ref().unwrap().env).as_str(),
                    "python",
                    "-y",
                ])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .expect("Failed to spawn conda create output");
            if conda_create.wait().unwrap().success() {
                println!("Conda created successfully");

                shell_output.push(format!(
                    "conda activate {}",
                    language_config.env.unwrap().env
                ));
                shell_output.push("pip install -r requirements.txt".to_string());
            } else {
                println!("Conda creation failed");
            }
        }
    }

    for o in shell_output {
        print!("{o}; ");
    }
}
