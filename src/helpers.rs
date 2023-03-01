use std::sync::Arc;
use ethers::prelude::{k256::ecdsa::SigningKey, *};

pub fn address(address: &str) -> Address {
    address.parse::<Address>().unwrap()
}

pub fn to_1e18(input: u64) -> U256 {
    let ether: U256 = U256::exp10(18);
    let parsed: U256 = input.into();
    parsed * ether
}

pub fn wei_to_float(input: u64) -> f64 {
    input as f64 / 1_000_000_000_000_000_000.0
}

pub fn timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

pub async fn user_balance(
    provider: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
) -> Result<U256, Box<dyn std::error::Error>> {
    let pkey = std::env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
    let skey = pkey.parse::<LocalWallet>().unwrap();
    let user_address = skey.address();

    let balance = provider.get_balance(user_address, None).await?;
    Ok(balance)
}

pub async fn setup_signer(
    provider: Provider<Http>,
) -> SignerMiddleware<Provider<Http>, Wallet<SigningKey>> {
    let chain_id = provider
        .get_chainid()
        .await
        .expect("Failed to get chain id");

    let pkey = std::env::var("PRIVATE_KEY").expect("PRIVATE_KEY missing");
    let wallet = pkey
        .parse::<LocalWallet>()
        .expect("Failed to parse wallet")
        .with_chain_id(chain_id.as_u64());

    SignerMiddleware::new(provider, wallet)
}