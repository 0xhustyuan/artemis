use anyhow::Result;
use clap::Parser;
use ethers::prelude::*;
use ethers::types::H160;
use ethers::providers::Provider;

use tracing::{info, Level};
use tracing_subscriber::{filter, prelude::*};
use std::str::FromStr;
use std::sync::Arc;

/// CLI Options.
#[derive(Parser, Debug)]
pub struct Args {
    /// Ethereum node WS endpoint.
    #[arg(long)]
    pub ipc: String,

    /// Private key for sending txs.
    #[arg(long)]
    pub private_key: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let filter = filter::Targets::new()
        .with_target("artemis_core", Level::INFO);
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();

    let args = Args::parse();

    // Set up ethers provider.
    let provider = Provider::connect_ipc(args.ipc).await?;
    let provider = Arc::new(provider);

    let block = provider.get_block_number().await?;

    info!("Current block: {block}");

    Ok(())
}
