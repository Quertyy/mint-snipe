use std::error::Error;
use std::thread;
use std::time::Duration;

use clap::Parser;
use mint_sniper::*;
use colored::*;
use colored::Colorize;
use mint_sniper::timestamp_print;

use ethers_middleware::SignerMiddleware;
use std::sync::Arc;
use ethers::contract::abigen;
use ethers::prelude::*;
use ethers::types::Address;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::parse();
    let user = User::parse()?;

    let provider = init_connection()?;
    let eth_balance_before = check_on_config(&config, &user.address, &provider).await?;
    trigger_timestamp(config.timestamp);
    sniping(user, &config, &provider, eth_balance_before).await?;
    timestamp_print!(Color::White, "Exiting...");

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
    timestamp_print!(Color::Blue, "Mint started!");
}

async fn sniping(user: User, config: &Config, provider: &Provider<Http>, balance_before: f64) -> Result<(), Box<dyn Error>> {
    abigen!(
        ERC721,
        r#"[
            function mint(uint256 amount) public payable
            function balanceOf(address account) public view returns (uint256)
        ]"#,
    );

    let client = SignerMiddleware::new_with_provider_chain(provider.clone(), user.skey).await.unwrap();
    let client = Arc::new(client);

    let address: Address = config.contract_address.parse().unwrap();
    let contract = ERC721::new(address, client.clone());

    let amount: U256 = U256::from(config.amount);
    let wei_amount = config.price * config.amount;
    let receipt = contract.mint(amount).value(wei_amount).send().await?.await?;
    println!("{:?}", receipt);

    timestamp_print!(Color::Green, "Minted successfully!");
    timestamp_print!(Color::Blue, "Checking your NFT balance...");
    let balance = contract.balance_of(user.address.parse().unwrap()).call().await?.as_u64();
    if balance == 0 {
        timestamp_print!(Color::Red, "Mint failed!");
    } else {
        timestamp_print!(Color::Green, format!("Minted {} NFTs!", balance));
    }
    timestamp_print!(Color::Blue, "Checking your ETH balance...");
    let u64_balance = client.get_balance(user.address, None).await?.as_u64();
    let balance_after = convert_wei_to_eth(u64_balance);

    timestamp_print!(Color::Green, format!("ETH balance: {} -> {}", balance_before, balance_after));
    

    Ok(())
}

// mockERC721 contract deployed on goerli: 0x30A8b049344DD9fb3d25C608816aC43A84317276
// 0.01 ETH = 10000000000000000 wei