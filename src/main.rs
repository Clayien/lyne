pub mod parser;
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
    let output = parser::parse();
    println!(
        "{} {}",
        output.main.project_dir, output.main.language_seperated
    );
}
