// SPDX-License-Identifier: MIT OR Apache-2.0

use aws_sdk_s3::Client;
use clap::Parser;
use tracing::info;

mod config;

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

    let s3_config = config::handle_config().unwrap();
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
