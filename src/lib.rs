use clap::Parser;
use ethers_core::k256::ecdsa::SigningKey;

use std::time::SystemTime;
use std::fs::File;
use std::io::Read;
use std::env;
use std::error::Error;
use std::result::Result;

use colored::*;
use colored::Colorize;

use ethabi::Contract;
use ethers::prelude::*;
use ethers_signers::LocalWallet;

use std::sync::Arc;

#[macro_export]
macro_rules! timestamp_print {
    ($color: expr, $message: expr) => {
        println!(
            "{} {} {}",
            chrono::Local::now().format("[%Y-%m-%d]").to_string().color($color),
            chrono::Local::now().format("[%H:%M:%S]").to_string().color($color),
            $message.color($color)
        );
    };
}

#[derive(Parser)]
#[command(name = "Snipe this Mint!")]
#[command(author = "Querty for The Diggers DAO")]
#[command(version = "0.1.0")]
#[command(about = "Let's compete with other bots", long_about = None)]
pub struct Config {
    /// The contract mint address
    #[arg(short, long)]
    pub contract_address: String,
    /// The contract mint method
    #[arg(short, long)]
    pub mint_method: String,
    /// The mint price in wei
    #[arg(short, long)]
    pub price: u64,
    /// Number of NFT you want to mint
    #[arg(short, long)]
    pub amount: u64,
    /// The timestamp of the beginning of the mint
    #[arg(short, long)]
    pub timestamp: u64,
}

pub struct User {
    pub skey: Wallet<SigningKey>,
    pub address: String,
    pub endpoint: String,
}


impl User {
    pub fn parse() -> Result<User, Box<dyn Error>> {
        dotenv::dotenv().ok();
        let pkey = env::var("PRIVATE_KEY").unwrap();
        let alchemy_endpoint: &str = &env::var("ALCHEMY_URL").unwrap().to_string();

        let skey = pkey.parse::<LocalWallet>()?;
        //let address = secret_key_to_address(&skey).to_string();
        //let signer = SignerMiddleware::new(provider, skey);
        let address = env::var("ADDRESS").unwrap();

        Ok(
            User {
                skey,
                address: address.to_string(),
                endpoint: alchemy_endpoint.to_string(),
            }
        )
    }
}

pub fn init_connection() ->  eyre::Result<Arc<Provider<Http>>> {
    let user = User::parse().unwrap();
    let _provider = Provider::<Http>::try_from(&user.endpoint)?;
    

    Ok(Arc::new(_provider))
}

pub async fn check_on_config(
    config: &Config, 
    user: &str, 
    provider: &Provider<Http>
) -> Result<f64, Box<dyn Error>> {
    timestamp_print!(Color::White, "Checking the mint requirements!");
    check_timestamp_requirement(config).unwrap();
    let eth_balance_before = check_balance_requirement(config, provider, user).await?;
    check_if_contract(config.contract_address.parse().unwrap(), provider).await?;
    check_abi_method(&config.mint_method)?;
    timestamp_print!(Color::Green, "All checks passed!");
    Ok(eth_balance_before)
}

fn check_timestamp_requirement(config: &Config) -> Result<(), Box<dyn Error>> {
    let timestamp = get_unix_time();
    if timestamp > config.timestamp {
        panic!("The mint has already started!");
    } else if timestamp < config.timestamp - 300 { // must be run 5 minutes or less before the mint
        panic!("The mint is too far in the future!");
    }
    timestamp_print!(Color::Green, "Timestamp check passed!");
    Ok(())
}

async fn check_if_contract(contract: Address, provider: &Provider<Http>) -> Result<(), Box<dyn Error>>{
    timestamp_print!(Color::White, "Checking if the contract address is a contract!");
    let code = provider.get_code(contract, None).await?;
    if code.is_empty() {
        panic!("The contract address is not a contract!");
    }

    timestamp_print!(Color::Green, "Contract check passed!");
    Ok(())
}

fn check_abi_method(method_name: &str) -> Result<(), Box<dyn Error>> {
    timestamp_print!(Color::White, "Checking the ABI method!");
    let mut file = File::open("src/abi/abi.json")?;
    let mut abi = String::new();
    file.read_to_string(&mut abi)?;

    let contract = Contract::load(abi.as_bytes())?;

    if !contract.function(method_name).is_ok() {
        panic!("The mint method does not exist!");
    }
    timestamp_print!(Color::Green, "ABI method check passed!");
    Ok(())
}

async fn check_balance_requirement(
    config: &Config, 
    provider: &Provider<Http>, 
    address: &str
) -> Result<f64, Box<dyn std::error::Error>> {
    timestamp_print!(Color::Blue, "Checking user's balance!");
    timestamp_print!(
        Color::Blue, 
        format!(
            "Balance required is : {} ETH", 
            convert_wei_to_eth(config.price * config.amount)
        )
    );
    let balance = get_wei_balance(address, provider).await?;
    let eth_balance = convert_wei_to_eth(balance);
    if balance < config.price * config.amount {
        panic!("Not enough balance!");
    }

    timestamp_print!(Color::Green, format!("Balance check passed! Balance before: {} ETH", eth_balance));
    Ok(eth_balance)
}

pub fn get_unix_time() -> u64 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
}

pub async fn get_wei_balance(address: &str, provider: &Provider<Http>) -> Result<u64, Box<dyn Error>> {
    let account: Address = address.parse().unwrap();
    let balance = provider.get_balance(account, None).await?;
    let balance = balance.as_u64();
    Ok(balance)
}

pub fn convert_wei_to_eth(wei: u64) -> f64 {
    wei as f64 / 1_000_000_000_000_000_000.0
}


