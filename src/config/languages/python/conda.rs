use std::io::Error;

use serde::Deserialize;

use crate::config::languages::Environment;
use crate::utils::{cmd, common};

#[derive(Deserialize)]
pub struct CondaEnvs {
    envs: Vec<String>,
}

pub fn get_actions(env: &Environment, init_command: &Option<String>) -> Result<Vec<String>, Error> {
    let mut actions: Vec<String> = Vec::new();
    let conda_activate = format!("conda activate {}", env.name);
    let conda_envs_json = cmd::spawn_output("conda", &["info", "--json"])?;
    if let Some(conda_envs) = common::parse_json::<CondaEnvs>(conda_envs_json) {
        if conda_envs.envs.iter().any(|e| e.contains(&env.name)) {
            actions.push(conda_activate);
        } else if cmd::silent_spawn(
            "conda",
            &["create", &format!("--name={}", env.name), "python", "-y"],
        )? {
            actions.push(conda_activate);
            if let Some(init_command) = init_command {
                actions.push(init_command.to_string());
            }
        }
    }

    Ok(actions)
}
