use std::{
    io::{Error, ErrorKind},
    process::{Command, Stdio},
};

fn spawn(cmd: &str, args: &[&str], show_output: bool) -> Result<Option<String>, Error> {
    let stdout = if show_output {
        Stdio::piped()
    } else {
        Stdio::null()
    };

    let command = Command::new(cmd)
        .args(args)
        .stdout(stdout)
        .stderr(Stdio::null())
        .spawn()?;

    let output = command.wait_with_output()?;
    if output.status.success() {
        if show_output {
            let output_msg = String::from_utf8(output.stdout).unwrap();
            Ok(Some(output_msg))
        } else {
            Ok(None)
        }
    } else {
        Err(std::io::Error::new(
            ErrorKind::Interrupted,
            match output.status.code() {
                Some(code) => format!("process exited with code {}", code),
                None => format!("process failed with status {}", output.status),
            },
        ))
    }
}

pub fn silent_spawn(cmd: &str, args: &[&str]) -> Result<bool, Error> {
    match spawn(cmd, args, false) {
        Ok(_) => Ok(true),
        Err(err) => Err(err),
    }
}

pub fn spawn_output(cmd: &str, args: &[&str]) -> Result<String, Error> {
    match spawn(cmd, args, true) {
        Ok(msg) => Ok(msg.unwrap()),
        Err(err) => Err(err),
    }
}
