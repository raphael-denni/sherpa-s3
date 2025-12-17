// SPDX-License-Identifier: MIT OR Apache-2.0

use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tracing::info;

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    access_key_id: String,
    secret_access_key: String,
    region: String,
}

fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let config_dir = dirs::config_dir().ok_or("Could not find config directory")?;
    let app_config_dir = config_dir.join("sherpa-s3");

    fs::create_dir_all(&app_config_dir)?;

    Ok(app_config_dir.join("config.toml"))
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = get_config_path()?;

    if !config_path.exists() {
        return Err("Configuration file does not exist. Please create one".into());
    }

    let content = fs::read_to_string(&config_path)?;
    let config: Config = toml::from_str(&content)?;

    Ok(config)
}

fn save_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = get_config_path()?;
    let content = toml::to_string_pretty(config)?;

    fs::write(&config_path, content)?;

    Ok(())
}

#[derive(Parser)]
enum Commands {
    /// List of objects or buckets
    Ls,
    /// Copy object
    Cp,
    /// Remove object
    Rm,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

fn main() {
    tracing_subscriber::fmt::init();

    let config = match load_config() {
        Ok(config) => {
            info!("Successfully loaded configuration.");
            config
        }

        Err(e) if e.to_string().contains("does not exist") => {
            info!("No configuration file found. Creating a default one.");

            let new_config = Config {
                access_key_id: "YOUR_ACCESS_KEY_ID".to_string(),
                secret_access_key: "YOUR_SECRET_ACCESS_KEY".to_string(),
                region: "us-east-1".to_string(),
            };

            if let Err(save_err) = save_config(&new_config) {
                eprintln!(
                    "Fatal: Could not save the new configuration file: {}",
                    save_err
                );
                std::process::exit(1);
            }

            info!("Default configuration file created at the standard location.");
            info!("Please edit it with your S3 credentials before proceeding.");
            new_config
        }

        Err(e) => {
            eprintln!("Fatal: Error loading configuration: {}", e);
            std::process::exit(1);
        }
    };

    info!("Current configuration: {:?}", config);

    let cli = Cli::parse();

    match &cli.commands {
        Commands::Ls => {
            info!("'ls' command called");
        }
        Commands::Cp => {
            info!("'cp' command called");
        }
        Commands::Rm => {
            info!("'rm' command called");
        }
    }
}
