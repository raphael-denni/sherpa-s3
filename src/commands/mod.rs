// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Command definitions for sherpa-s3 CLI.
//! This module defines the command-line interface (CLI) commands
//! for the sherpa-s3 application using the `clap` crate.
//! It includes commands for listing, copying, and removing S3 objects.

pub mod cp;
pub mod ls;
pub mod rm;
use clap::Parser;

/// # CLI Commands for sherpa-s3
///
/// # Commands
/// - `ls [bucket]`: List objects in a bucket or list all buckets if no bucket is specified.
/// - `cp` `<source>` `<destination>`: Copy an object from source to destination.
/// - `rm`: Remove an object (not yet implemented).
#[derive(Parser)]
pub enum Commands {
    /// List of objects or buckets from S3 storage
    Ls {
        /// The bucket to list objects from. If not provided, lists all buckets.
        bucket: Option<String>,
    },
    /// Copy an S3 object from source to destination
    /// Format: sherpa cp s3://source-bucket/source-key s3://dest-bucket/dest-key
    Cp {
        /// The source S3 object URI
        source: String,
        /// The destination S3 object URI
        destination: String,
    },
    /// Remove a bucket or object from S3 storage
    Rm {
        /// The bucket name
        bucket: String,
        /// The object key (optional)
        s3_object: Option<String>,
    },
}

/// # Command Line Interface for sherpa-s3
/// This struct defines the CLI for sherpa-s3 using clap.
/// It includes subcommands for listing, copying, and removing S3 objects.
///
/// # Fields
/// - `commands`: The subcommands available in the CLI.
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    /// The subcommands for the CLI.
    pub commands: Commands,
}
