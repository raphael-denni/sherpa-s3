// SPDX-License-Identifier: MIT OR Apache-2.0

use aws_sdk_s3::{Client, config::Credentials, config::Region};
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
    endpoint_url: Option<String>,
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
    Ls {
        /// The bucket to list objects from. If not provided, lists all buckets.
        bucket: Option<String>,
    },
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

#[tokio::main]
async fn main() {
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
                endpoint_url: None,
            };

            if let Err(save_err) = save_config(&new_config) {
                eprintln!(
                    "Fatal: Could not save the new configuration file: {}",
                    save_err
                );

                std::process::exit(1);
            }

            println!("Default configuration file created at the standard location.");
            println!("Please edit it with your S3 credentials before proceeding.");

            println!(
                "For S3-compatible services, add and uncomment the `endpoint_url` key in your config file. For example:"
            );
            println!("# endpoint_url = \"s3.us-east-001.compatible-service.com\"");

            new_config
        }

        Err(e) => {
            eprintln!("Fatal: Error loading configuration: {}", e);
            std::process::exit(1);
        }
    };

    info!("Current configuration: {:?}", config);

    let credentials = Credentials::new(
        &config.access_key_id,
        &config.secret_access_key,
        None,
        None,
        "Static",
    );

    let region = Region::new(config.region.clone());

    let mut s3_config_builder = aws_sdk_s3::Config::builder()
        .credentials_provider(credentials)
        .region(region);

    if let Some(endpoint_url) = &config.endpoint_url {
        s3_config_builder = s3_config_builder.endpoint_url(endpoint_url);
    }

    let s3_config = s3_config_builder.build();

    let client = Client::from_conf(s3_config);

    let cli = Cli::parse();

    match &cli.commands {
        Commands::Ls { bucket } => match bucket {
            Some(bucket_name) => {
                println!("Listing objects in bucket: {}", bucket_name);

                match client.list_objects_v2().bucket(bucket_name).send().await {
                    Ok(output) => {
                        if let Some(contents) = output.contents {
                            if contents.is_empty() {
                                println!("No objects found in bucket '{}'.", bucket_name);
                            } else {
                                println!("Objects in '{}':", bucket_name);

                                for object in contents {
                                    println!(
                                        "  - {}",
                                        object.key.unwrap_or_else(|| "N/A".to_string())
                                    );
                                }
                            }
                        } else {
                            println!("No objects found in bucket '{}'.", bucket_name);
                        }
                    }

                    Err(e) => {
                        eprintln!(
                            "Error: Could not list objects in bucket '{}': {}",
                            bucket_name, e
                        );

                        std::process::exit(1);
                    }
                }
            }

            None => {
                println!("Listing S3 buckets...");

                match client.list_buckets().send().await {
                    Ok(output) => {
                        if let Some(buckets) = output.buckets {
                            if buckets.is_empty() {
                                println!("No S3 buckets found.");
                            } else {
                                println!("S3 Buckets:");

                                for bucket in buckets {
                                    println!(
                                        "  - {}",
                                        bucket.name.unwrap_or_else(|| "N/A".to_string())
                                    );
                                }
                            }
                        } else {
                            println!("No S3 buckets found.");
                        }
                    }

                    Err(e) => {
                        eprintln!("Error: Could not list S3 buckets: {}", e);
                        std::process::exit(1);
                    }
                }
            }
        },

        Commands::Cp => {
            info!("'cp' command called");
        }

        Commands::Rm => {
            info!("'rm' command called");
        }
    }
}
