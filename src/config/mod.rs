use std::fs::read_to_string;
use std::env::var;

use serde_json;

use crate::utils::types::{
    Config,
    Window,
    Commands
};

pub fn parse() -> Config {
    let default_config = Config {
        window: Window {
            background_color: String::from("0, 0, 0"),
            opacity: 1.0,
        },
        commands: Commands {
            shutdown: String::from("shutdown"),
            logout: String::from("i3-msg exit"),
            reboot: String::from("reboot"),
        }
    };

    let path = match var("HOME") {
        Ok(path) => path,
        Err(_) => return default_config,
    };

    let content = match read_to_string(&format!("{path}/.config/vendetta/config.json")) {
        Ok(text) => text,
        Err(_) => return default_config,
    };

    match serde_json::from_str::<Config>(&content) {
        Ok(cfg) => cfg,
        Err(_) => {
            println!("\x1b[31m[X] Failed to load configuration file, using default config...\x1b[m");

            default_config
        }
    }
}
