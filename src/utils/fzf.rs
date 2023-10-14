use std::{
    io::Write,
    process::{Command, Stdio},
    str,
};

pub fn choose(options: Vec<String>) -> String {
    let mut fzf = Command::new("fzf")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn fzf output");

    let mut stdin = fzf.stdin.take().expect("Falied to take stdin");
    let mut option_stdin = String::new();
    for option in options {
        option_stdin.push_str(&option);
        option_stdin.push('\n');
    }

    stdin
        .write_all(option_stdin.as_bytes())
        .expect("Failed to write options as output");

    let output = fzf.wait_with_output().expect("Failed to read fzf output");

    String::from(str::from_utf8(&output.stdout).unwrap().trim())
}
