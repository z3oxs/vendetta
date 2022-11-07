use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub window: Window,
    pub commands: Commands,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Window {
    pub background_color: String,
    pub opacity: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Commands {
    pub shutdown: String,
    pub logout: String,
    pub reboot: String,
}
