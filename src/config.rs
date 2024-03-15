use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct CheckpointConfig {
    host: String,  // what if this is an IP?
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub(crate) struct Config {
    CheckpointConfig,
}

