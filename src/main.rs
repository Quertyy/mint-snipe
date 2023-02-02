use web3_unit_converter::Unit;
use web3::types::Address;
use std::env;
use web3::api::Txpool;


use clap::{arg, command, value_parser, ArgAction, Command, Arg};

#[tokio::main]
async fn main() -> web3::Result<()> {
    let matches = Command::new("Snipe this mint!")
        .about("A simple CLI tool to mint NFTs on Ethereum")
        .version("0.1.0")
        .author("Querty for The Diggers DAO")
        .arg(
            Arg::new("Mint contract address")
                .short('c')
                .long("contract_address")
                .required(true),
        )
        .arg(
            Arg::new("Contract ABI")
                .short('a')
                .long("abi")
                .required(true),
        )
        .arg(
            Arg::new("Price in WEI")
                .short('p')
                .long("price")
                .required(true),
        )
        .arg(
            Arg::new("Amount of NFTs to mint")
                .short('n')
                .long("amount")
                .required(true),
        )
        .arg(
            Arg::new("Private key of the account")
                .short('k')
                .long("private-key")
                .required(true),
        )
        .arg(
            Arg::new("Timestamp of the mint in seconds")
                .short('t')
                .long("timestamp")
                .required(true),
        )
        .get_matches();
    
    let args: Vec<String> = env::args().collect();
    dotenv::dotenv().ok();

    let url = &env::var("ALCHEMY_URL").unwrap();
    let address = &env::var("ADDRESS").unwrap();

    get_user_balance(address, url).await?;
    Ok(())
}


// récupérer la balance d'un utilisateur
async fn get_user_balance(user: &str, url: &str) -> web3::Result<()> {
    let account: Address = user.parse().unwrap();
    

    let _ = env_logger::try_init();
    let transport = web3::transports::Http::new(url)?;
    let web3 = web3::Web3::new(transport);

    let wei_balance = web3.eth().balance(account, None).await?;
    let eth_balance = Unit::Wei(&wei_balance.to_string()).to_eth_str().unwrap();
    println!("Balance of {:?}: {} ETH", account, eth_balance);

    Ok(())
}

// TODO
// check les tx dans la mempool
//async fn get_mempool() -> web3::Result<()> {
//    let url = &env::var("ALCHEMY_URL").unwrap();
//
//    let _ = env_logger::try_init();
//    let transport = web3::transports::Http::new(url)?;
//    let web3 = web3::Web3::new(transport);
//
//    let mempool = web3.eth().pending_transactions().await?;
//    println!("Mempool: {:?}", mempool);
//
//    Ok(())
//}