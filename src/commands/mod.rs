// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Command definitions for sherpa-s3 CLI.
//! This module defines the command-line interface (CLI) commands
//! for the sherpa-s3 application using the `clap` crate.
//! It includes commands for listing, copying, and removing S3 objects.

pub mod ls;
use clap::Parser;

/// # CLI Commands for sherpa-s3
/// Commands:
/// - `ls [bucket]`: List objects in a bucket or list all buckets if no bucket is specified.
/// - `cp`: Copy an object (not yet implemented).
/// - `rm`: Remove an object (not yet implemented).
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

/// # Command Line Interface for sherpa-s3
/// This struct defines the CLI for sherpa-s3 using clap.
/// It includes subcommands for listing, copying, and removing S3 objects.
///
/// Fields:
/// - `commands`: The subcommands available in the CLI.
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    /// The subcommands for the CLI.
    pub commands: Commands,
}
