mod core;
mod api;
mod cli;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    cli::execute().await
}
