use clap::Parser;
use std::sync::Arc;
use ethers::prelude::{k256::ecdsa::SigningKey, *};
use crate::helpers::{
    user_balance, 
    timestamp, 
    wei_to_float,
};
use crate::Config;
use ethabi::Contract;
use std::error::Error;
use std::io::Read;

use colored::*;

use crate::timestamp_print;


#[derive(Parser)]
#[command(name = "Backrunnor", version = "0.1.0", author = "Querty for The Diggers DAO")]
pub struct Mint {
    /// Contract address
    #[arg(short, long)]
    pub contract: Address,
    /// Quantity to mint
    #[arg(short, long)]
    pub quantity: u64,
    /// Mint price in wei
    #[arg(short, long)]
    pub price: u64,
    /// Mint method
    #[arg(short, long)]
    pub method: String,
    /// Mint timestamp
    #[arg(short, long)]
    pub timestamp: u64,
    // Max retry
    #[arg(short, long)]
    pub retry_max: u64,
    // Max fees paid
    #[arg(short, long)]
    pub fees_max: u64,
}

impl Mint {
    pub async fn check_requirements(&self, config: Arc<Config>) -> Result<(), Box<dyn Error>> {
        timestamp_print!(Color::Blue, "Checking infos provided.".to_string());
    
        self.check_timestamp()?;    
        self.check_balance(config.http.clone()).await?;
        self.check_contract(config.http.clone()).await?;
        self.check_method(&self.method)?;
        Ok(())
    }

    fn check_timestamp(&self) -> Result<(), Box<dyn Error>> {
        let now = timestamp();

        if self.timestamp < now {
            let message = format!(
                    "Timestamp: Timestamp is in the past! (now={}, timestamp={})",
                    now, self.timestamp
            );
            timestamp_print!(Color::Red, message.clone());
            return Err(message.into());
        } else {
            timestamp_print!(Color::Green, "Timestamp is valid.".to_string());
            Ok(())
        } 
    }

    async fn check_balance(
        &self, 
        provider: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>
    ) -> Result<(), Box<dyn Error>> {
        timestamp_print!(Color::Blue, "Checking balance.".to_string());
        let required = self.quantity * self.price;
        timestamp_print!(
            Color::Blue, 
            format!(
                "Balance required: {} ETH", 
                wei_to_float(required)
            )
        );
        let balance = user_balance(provider).await?.as_u64();
        let required = self.quantity * self.price;

        if balance < required {
            let message = format!(
                "Balance: Not enough balance to mint! Expected: {} ETH, Balance: {} ETH", 
                wei_to_float(required), 
                wei_to_float(balance)
            );
            timestamp_print!(Color::Red, message.clone());
            return Err(message.into());
        }

        timestamp_print!(Color::Green, format!("Balance is valid: {} ETH", wei_to_float(balance)));
        Ok(())
    }

    async fn check_contract(
        &self, 
        provider: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>
    ) -> Result<(), Box<dyn Error>> {
        timestamp_print!(Color::Blue, format!("Checking contract address {:#066x}", self.contract));
        let code = provider.get_code(self.contract, None).await?;
        if code.is_empty() {
            return Err(format!(
                "Contract: Address is not a contract!"
            )
            .into());
        }
        timestamp_print!(Color::Green, "Contract address is valid.".to_string());
        Ok(())
    }

    fn check_method(&self, method: &str) -> Result<(), Box<dyn Error>> {
        timestamp_print!(Color::Blue, "Checking mint method.".to_string());
        let mut file = std::fs::File::open("src/abi/abi.json")?;
        let mut abi = String::new();
        file.read_to_string(&mut abi)?;
        
        let contract = Contract::load(abi.as_bytes())?;

        if !contract.function(method).is_ok() {
            return Err(format!(
                "ABI: The mint method does not exist!"
            )
            .into());
        }
        timestamp_print!(Color::Green, "Mint method is valid.".to_string());
        Ok(())
    }

    pub fn trigger_timestamp(&self) {
        timestamp_print!(Color::White, "Waiting for the mint to start...");
        loop {
            let current_time = timestamp();
            if current_time < self.timestamp {
                std::thread::sleep(std::time::Duration::from_millis(500));
            } else {
                break;
            }
        }
        timestamp_print!(Color::Blue, "Mint started!");
    }

    pub async fn sniping(&self, config: Arc<Config>) -> Result<TransactionReceipt, Box<dyn Error>> {
        abigen!(
            ERC721,
            r#"[
                function mint(uint256 amount) public payable
                function balanceOf(address account) public view returns (uint256)
            ]"#,
        );
        
        let provider = config.http.clone();
        let contract = ERC721::new(self.contract, provider);
        
        let qty = U256::from(self.quantity);
        

        timestamp_print!(Color::Blue, "SNIPIIIIIIIIING.".to_string());
        let mut success = false;
        let mut i = 0;
        let mut final_receipt = TransactionReceipt::default();

        while !success && i < self.retry_max {
            let call = contract.method::<_, U256>(self.method.as_str(), qty)?;
            let receipt = if self.price > 0 {
                call.value(self.price * self.quantity).send().await?.await?
            } else {
                call.send().await?.await?
            };

            match receipt {
                Some(receipt) => {
                    if receipt.status.unwrap().as_u64() == 1 {
                        timestamp_print!(Color::Green, "Transaction succeeded!".to_string());
                        timestamp_print!(Color::Green, format!("Tx hash: {:#066x}", receipt.transaction_hash));
                        success = true;
                        final_receipt = receipt;
                    } else {
                        timestamp_print!(Color::Red, "Transaction failed!".to_string());
                        timestamp_print!(Color::Blue, "Retrying...".to_string());
                        i += 1;

                    }
                },
                None => {
                    timestamp_print!(Color::Red, "Transaction submission failed!".to_string());
                    return Err("Transaction submission failed!".into());
                }
            }
        }

        if !success {
            timestamp_print!(Color::Red, "Transaction failed after maximum retries.".to_string());
            return Err("Transaction failed after maximum retries.".into());
        }

        timestamp_print!(Color::Green, "Minted successfully!".to_string());
    
        Ok(final_receipt)
    }
}