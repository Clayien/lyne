pub mod fzf;
pub mod parser;

use std::fs;

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
    let config = parser::parse();

    let project_dirs = fs::read_dir(config.main.project_dir).unwrap();
    let mut options: Vec<String> = Vec::new();
    for project_dir in project_dirs {
        options.push(project_dir.unwrap().path().display().to_string());
    }

    let output = fzf::choose(options);
    print!("{output}");
}
