use web3_unit_converter::Unit;
use web3::types::Address;
use std::env;
use web3::api::Txpool;

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();

    let url = &env::var("ALCHEMY_URL").unwrap();
    let address = &env::var("ADDRESS").unwrap();

    get_user_balance("0x58F51aD645AAB9cEFc282F9630E2Ca0F15B3aF62").await?;
    Ok(())
}


// récupérer la balance d'un utilisateur
async fn get_user_balance(user: &str) -> web3::Result<()> {
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