// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Rm Command for sherpa-s3 CLI.
//! This module implements the 'rm' command for the sherpa-s3 CLI application.

use aws_sdk_s3::Client;
use std::io::{self, Write};

/// # Run 'rm' Command
/// This function removes an S3 bucket or an object within a bucket.
///
/// # Arguments
/// - `client`: An instance of the AWS S3 client.
/// - `bucket`: The name of the bucket to delete or from which to delete an object.
/// - `s3_object`: An optional object key. If provided, deletes the object; otherwise, deletes the bucket.
///
/// # Errors
/// Returns an error if there are issues during the deletion operation.
pub async fn run_rm(
    client: &Client,
    bucket: &str,
    s3_object: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(object_key) = s3_object {
        println!(
            "Deleting object '{}' from bucket '{}'...",
            object_key, bucket
        );

        client
            .delete_object()
            .bucket(bucket)
            .key(object_key)
            .send()
            .await?;

        println!("Object deleted successfully.");
    } else {
        println!(
            "Are you sure you want to delete the bucket '{}' and all its contents? (y/N): ",
            bucket
        );

        io::stdout().flush()?;

        let mut confirmation = String::new();
        io::stdin().read_line(&mut confirmation)?;

        if confirmation.trim().eq_ignore_ascii_case("y") {
            println!("Deleting bucket '{}'...", bucket);

            client.delete_bucket().bucket(bucket).send().await?;

            println!("Bucket deleted successfully.");
        } else {
            println!("Bucket deletion cancelled.");
        }
    }

    Ok(())
}
