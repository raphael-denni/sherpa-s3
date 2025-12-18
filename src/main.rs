// SPDX-License-Identifier: MIT OR Apache-2.0

use aws_sdk_s3::Client;
use clap::Parser;
use tracing::info;

mod commands;
mod config;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let s3_config = config::handle_config().unwrap();
    let client = Client::from_conf(s3_config);

    let cli = commands::Cli::parse();

    match &cli.commands {
        commands::Commands::Ls { bucket } => {
            info!("'ls' command called");

            if let Err(e) = commands::ls::run_ls(&client, bucket.clone()).await {
                eprintln!("Error executing ls command: {}", e);
                std::process::exit(1);
            }
        }

        commands::Commands::Cp => {
            info!("'cp' command called");
        }

        commands::Commands::Rm => {
            info!("'rm' command called");
        }
    }
}
