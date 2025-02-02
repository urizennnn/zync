use chrono::prelude::*;
use log::warn;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::fs::{create_dir_all, File};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use whoami::username;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    user: String,
    files: Vec<String>,
    last_active: String,
}

pub async fn init() -> Result<String, Box<dyn Error>> {
    if let Some(config_dir) = dirs::config_dir() {
        let tcp_client_dir = config_dir.join("tcp_client");
        create_dir_all(&tcp_client_dir).await?;

        let config_file_path = tcp_client_dir.join("config.json");

        if !config_file_path.exists() {
            let user = username();
            let current_date = Utc::now().to_rfc3339();
            let initial_config = Config {
                user,
                files: vec![],
                last_active: current_date,
            };

            let config_json = serde_json::to_string_pretty(&initial_config)?;
            let mut file = File::create(&config_file_path).await?;
            file.write_all(config_json.as_bytes()).await?;
        }

        Ok(tcp_client_dir.to_string_lossy().into_owned())
    } else {
        warn!("Config directory not found");
        Err("Config directory not found".into())
    }
}

pub async fn update_init(new_file: String) -> Result<(), Box<dyn Error>> {
    if let Some(config_dir) = dirs::config_dir() {
        let tcp_client_dir = config_dir.join("tcp_client");
        let config_file_path = tcp_client_dir.join("config.json");

        if config_file_path.exists() {
            let mut file = File::open(&config_file_path).await?;
            let mut contents = vec![];
            file.read_to_end(&mut contents).await?;
            let mut config: Config = serde_json::from_slice(&contents)?;

            config.files.push(new_file);
            config.last_active = Utc::now().to_rfc3339();

            let config_json = serde_json::to_string_pretty(&config)?;
            let mut file = File::create(&config_file_path).await?;
            file.write_all(config_json.as_bytes()).await?;
        } else {
            warn!("Config file not found");
            return Err("Config file not found".into());
        }

        Ok(())
    } else {
        warn!("Config directory not found");
        Err("Config directory not found".into())
    }
}
