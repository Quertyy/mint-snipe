use clap::Parser;

use std::time::{SystemTime};
use std::fs::File;
use std::io::Read;
use std::env;
use std::error::Error;
use std::result::Result;

use hex::encode;

use web3::{Web3, contract};
use web3::futures::Future;
use web3::contract::{Contract, Options};
use web3::transports::Http;
use web3::types::{Address, U256, H160};
use web3_unit_converter::Unit;
use serde_json::{Result as JsonResult, Value};

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

pub fn init_connection() -> web3::Result<Web3<Http>> {
    let user = User::parse();
    let url = &user.url;
    let _ = env_logger::try_init();

    let transport = Http::new(url)?;
    let web3 = Web3::new(transport);

    Ok(web3)
}

pub async fn check_on_config(config: &Config, user: &str, web3: &web3::Web3<Http>) -> Result<(), Box<dyn Error>> {
    check_timestamp_requirement(config).unwrap();
    check_balance_requirement(config, web3, user).await?;
    check_if_contract(config.contract_address.parse().unwrap(), web3).await?;
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

async fn check_if_contract(contract: Address, web3: &web3::Web3<Http>) -> Result<(), Box<dyn Error>>{
    let code = web3.eth().code(contract, None).await?;
    let serialized = format!("{}", serde_json::to_string(&code).unwrap());
    if serialized == String::from("\"0x\"") {
        panic!("The address is not a contract!");
    }
    Ok(())
}

async fn check_balance_requirement(config: &Config, web3: &web3::Web3<Http>, user: &str) -> web3::Result<()> {
    let account: Address = user.parse().unwrap();

    let wei_balance = web3.eth().balance(account, None).await?;
    let eth_balance = Unit::Wei(&wei_balance.to_string()).to_eth_str().unwrap();
    println!("Balance of {:?}: {} ETH", account, eth_balance);

    if wei_balance.as_u64() < config.price * config.amount {
        panic!("You don't have enough ETH to mint!");
    }

    Ok(())
}

pub fn get_unix_time() -> u64 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
}

pub async fn get_user_balance(web3: &web3::Web3<Http>, user: &str) -> web3::Result<u64> {
    let account: Address = user.parse().unwrap();

    let wei_balance = web3.eth().balance(account, None).await?;
    let eth_balance = Unit::Wei(&wei_balance.to_string()).to_eth_str().unwrap();
    println!("Balance of {:?}: {} ETH", account, eth_balance);
    Ok(wei_balance.as_u64())
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