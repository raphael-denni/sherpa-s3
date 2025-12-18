// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod ls;
use clap::Parser;

#[derive(Parser)]
pub enum Commands {
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
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}
