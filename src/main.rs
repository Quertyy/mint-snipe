use web3_unit_converter::Unit;
use web3::transports::Http;
use web3::types::{Address, U256};
use std::env;
use std::time::{SystemTime};
use std::error::Error;

// abi read
use serde_json::{Result, Value};
use std::fs::File;
use std::io::Read;

// args parser
use clap::{arg, command, value_parser, ArgAction, Command, Arg};

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();
    helper();

    let url = &env::var("ALCHEMY_URL").unwrap();
    let address = &env::var("ADDRESS").unwrap();

    let _ = env_logger::try_init();
    let transport = web3::transports::Http::new(url)?;
    let web3 = web3::Web3::new(transport);
    
    
    let args: Vec<String> = env::args().collect();
    

    get_user_balance(&web3, address).await?;
    let timestamp = get_unix_time();
    println!("Current timestamp: {}", timestamp);

    let abi = read_abi().unwrap();
    Ok(())
}

fn get_unix_time() -> u64 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
}

fn helper() {
    Command::new("Snipe this mint!")
        .about("A simple CLI tool to mint NFTs on Ethereum")
        .version("0.1.0")
        .author("Querty for The Diggers DAO")
        .arg(
            Arg::new("Mint contract address")
                .short('c')
                .long("contract_address")
                .required(false),
        )
        .arg(
            Arg::new("Contract ABI")
                .short('a')
                .long("abi")
                .required(false),
        )
        .arg(
            Arg::new("Price in WEI")
                .short('p')
                .long("price")
                .required(false),
        )
        .arg(
            Arg::new("Amount of NFTs to mint")
                .short('n')
                .long("amount")
                .required(false),
        )
        .arg(
            Arg::new("Timestamp of the mint in seconds")
                .short('t')
                .long("timestamp")
                .required(false),
        )
        .get_matches();
}

async fn get_user_balance(web3: &web3::Web3<Http>, user: &str) -> web3::Result<()> {
    let account: Address = user.parse().unwrap();

    let wei_balance = web3.eth().balance(account, None).await?;
    let eth_balance = Unit::Wei(&wei_balance.to_string()).to_eth_str().unwrap();
    println!("Balance of {:?}: {} ETH", account, eth_balance);
    Ok(())
}

fn read_abi() -> Result<Value>{
    let mut file = File::open("abi/test.json").unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    

    let abi_json: Value = serde_json::from_str(&contents).unwrap();
    let abi_string = abi_json["result"].as_str().unwrap();
    let abi: Value = serde_json::from_str(abi_string).unwrap();
    Ok(abi)
}