// SPDX-License-Identifier: MIT OR Apache-2.0

use tracing::info;

fn main() {
    tracing_subscriber::fmt::init();
    info!("Hello, sherpa-s3!");
}
