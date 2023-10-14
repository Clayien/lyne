pub mod conda;

use super::LanguageConfig;

pub fn get_actions(config: &LanguageConfig) -> Vec<String> {
    let mut output = Vec::new();
    if let Some(env) = &config.environment {
        if env.command == "conda" {
            if let Ok(actions) = conda::get_actions(env, &config.init_command) {
                for action in actions {
                    output.push(action);
                }
            }
        }
    }

    output
}
