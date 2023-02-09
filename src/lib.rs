use clap::Parser;

use std::time::{SystemTime};
use std::fs::File;
use std::io::Read;
use std::env;
use std::error::Error;
use std::result::Result;

use ethabi::Contract;
use ethers::prelude::*;
use reqwest::header::{HeaderMap, HeaderValue};

use serde_json::{Result as JsonResult, Value};


use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Parser)]
#[command(name = "Snipe this Mint!")]
#[command(author = "Querty for The Diggers DAO")]
#[command(version = "0.1.0")]
#[command(about = "Does awesome things", long_about = None)]
pub struct Config {
    // The contract mint address
    #[arg(short, long)]
    contract_address: String,
    // The contract mint method
    #[arg(short, long)]
    mint_method: String,
    // The mint price
    #[arg(short, long)]
    price: u64,
    // Number of NFT you want to mint
    #[arg(short, long)]
    amount: u64,
    // The timestamp of the beginning of the mint
    #[arg(short, long)]
    timestamp: u64,
}

pub struct User {
    pub url: String,
    pub address: String,
}

impl User {
    pub fn parse() -> User {
        dotenv::dotenv().ok();

        let url = &env::var("ALCHEMY_URL").unwrap();
        let address = &env::var("ADDRESS").unwrap();

        User {
            url: url.to_string(),
            address: address.to_string(),
        }
    }
}

pub fn init_connection() ->  eyre::Result<Provider<Http>> {
    let user = User::parse();
    let _provider = Provider::<Http>::try_from(&user.url)?;

    Ok(_provider)
}

pub async fn check_on_config(
    config: &Config, 
    user: &str, 
    provider: &Provider<Http>
) -> Result<(), Box<dyn Error>> {
    check_timestamp_requirement(config).unwrap();
    check_balance_requirement(config, provider, user).await?;
    check_if_contract(config.contract_address.parse().unwrap(), provider).await?;
    check_abi_method(&config.mint_method)?;
    Ok(())
}

fn check_timestamp_requirement(config: &Config) -> Result<(), Box<dyn Error>> {
    let timestamp = get_unix_time();
    if timestamp > config.timestamp {
        panic!("The mint has already started!");
    } else if timestamp < config.timestamp - 300 { // must be run 5 minutes or less before the mint
        panic!("The mint is too far in the future!");
    }

    Ok(())
}

abigen!(Test, "src/abi/test.json", event_derives(serde::Deserialize, serde::Serialize));

async fn check_if_contract(contract: Address, provider: &Provider<Http>) -> Result<(), Box<dyn Error>>{
    let code = provider.get_code(contract, None).await?;
    if code.is_empty() {
        panic!("The contract address is not a contract!");
    }
    Ok(())
}

fn check_abi_method(method_name: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::open("src/abi/test.json")?;
    let mut abi = String::new();
    file.read_to_string(&mut abi)?;

    let contract = Contract::load(abi.as_bytes())?;

    if !contract.function(method_name).is_ok() {
        panic!("The mint method does not exist!");
    }

    Ok(())
}

async fn check_balance_requirement(
    config: &Config, 
    provider: &Provider<Http>, 
    user: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let account: Address = user.parse().unwrap();

    let balance = provider.get_balance(account, None).await?;
    let balance = balance.as_u64();
    if balance < config.price * config.amount {
        panic!("Not enough balance!");
    }

    println!("Balance: {}", balance);
    Ok(())
}

pub fn get_unix_time() -> u64 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
}

