use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
    pub port: u32,
    pub host: String,
}

impl Config {
    pub fn new() -> Self {
        let server = envy::prefixed("SERVER_").from_env().unwrap();

        Self { server }
    }
}
