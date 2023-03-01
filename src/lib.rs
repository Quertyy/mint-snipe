pub mod helpers;
pub mod mint;
pub mod receipt;

use clap::Parser;
use std::sync::Arc;

use ethers::prelude::{k256::ecdsa::SigningKey, *};

use crate::mint::Mint;
use crate::receipt::Transaction;
use crate::helpers::setup_signer;

#[macro_export]
macro_rules! timestamp_print {
    ($color: expr, $message: expr) => {
        println!(
            "{} {} {}",
            chrono::Local::now()
                .format("[%Y-%m-%d]")
                .to_string()
                .color($color),
            chrono::Local::now()
                .format("[%H:%M:%S]")
                .to_string()
                .color($color),
            $message.color($color)
        );
    };
}

pub struct Config {
    #[allow(dead_code)]
    pub http: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
}

impl Config {
    pub async fn new() -> Self {
        let network = std::env::var("NETWORK_RPC").expect("NETWORK_RPC missing");
        let provider = Provider::<Http>::try_from(network).unwrap();
        let middleware = Arc::new(setup_signer(provider.clone()).await);

        Self {
            http: middleware,
        }
    }
}

pub async fn run() {
    let config = Arc::new(Config::new().await);
    let mint = Mint::parse();
    mint.check_requirements(config.clone()).await.unwrap();
    mint.trigger_timestamp();
    let receipt = mint.sniping(config.clone()).await.unwrap();
    let mint_receipt = Transaction::new(receipt);
    //mint_receipt.print(config.clone());
}