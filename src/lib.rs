use clap::Parser;

use std::time::{SystemTime};
use std::fs::File;
use std::io::Read;
use std::env;
use std::error::Error;
use std::result::Result;

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
    //check_if_contract(config.contract_address.parse().unwrap(), provider).await?;
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

//async fn check_if_contract(contract: Address, provider: &Provider<Http>) -> Result<(), Box<dyn Error>>{
//    let code = web3.eth().code(contract, None).await?;
//    let serialized = format!("{}", serde_json::to_string(&code).unwrap());
//    if serialized == String::from("\"0x\"") {
//        panic!("The address is not a contract!");
//    }
//    Ok(())
//}

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

pub fn read_abi() -> JsonResult<Value>{
    let mut file = File::open("abi/test.json").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    

    let abi_json: Value = serde_json::from_str(&contents).unwrap();
    let abi_string = abi_json["result"].as_str().unwrap();
    let abi: Value = serde_json::from_str(abi_string).unwrap();
    Ok(abi)
}

/*
async fn get_contract_abi(web3: &web3::Web3<Http>, contract_address: H160) -> bool {
    let code = web3.eth().code(contract_address, None).await.unwrap();
    if code.is_empty() {
        return false;
    }
    let abi = web3.eth().get_abi(contract_address, None).await.unwrap();
    if abi.is_none() {
        return false;
    }
    let contract = Contract::from_abi(web3.eth(), contract_address, abi.unwrap(), Options::default()).unwrap();
    let symbol: H256 = contract.function("symbol").call().await.unwrap();
    if symbol.is_zero() {
        return false;
    }
    true
}
*/