use std::error::Error;
use std::thread;
use std::time::Duration;

use clap::Parser;
use mint_sniper::*;
use colored::*;
use colored::Colorize;
use mint_sniper::timestamp_print;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::parse();
    let user = User::parse();

    let provider = init_connection()?;
    check_on_config(&config, &user.address, &provider).await?;
    trigger_timestamp(config.timestamp);
    Ok(())
}

fn trigger_timestamp(timestamp: u64) {
    timestamp_print!(Color::White, "Waiting for the mint to start...");
    loop {
        let current_time = get_unix_time();
        if current_time < timestamp {
            thread::sleep(Duration::from_secs(1));
        } else {
            break;
        }
    }
    timestamp_print!(Color::Yellow, "Mint started!");
}

// mockERC721 contract deployed on goerli: 0x2F7F6a9cE73529354fc62bd42632da7A5084CC27