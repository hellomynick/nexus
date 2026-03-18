use serde::Deserialize;
use tokio::fs;

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub hls: HlsConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub struct HlsConfig {
    pub ram_disk_path: String,
    pub chunk_duration: u8,
    pub buffer_size: u8,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    pub async fn load() -> Self {
        let config = fs::read_to_string("../config.toml")
            .await
            .expect("Can't read file");

        return toml::from_str(&config).expect("Can't read config");
    }
}
