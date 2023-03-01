use crate::{timestamp_print, helpers::user_balance};
use colored::*;

use crate::helpers::wei_to_float;

use std::sync::Arc;
use ethers::prelude::{k256::ecdsa::SigningKey, *};

pub struct Transaction {
    pub receipt: TransactionReceipt,
    pub from: Address,
    pub fees: u64,
    pub status: bool,
    pub hash: H256,
}

impl Transaction {
    pub fn new(receipt: TransactionReceipt) -> Self {
        let from = receipt.from;

        let gas_used = receipt.gas_used.unwrap().as_u64();
        let gas_price = receipt.effective_gas_price.unwrap().as_u64();
        let fees = gas_used * gas_price;

        let status = receipt.status.unwrap().as_u64() == 1;

        let hash = receipt.transaction_hash;

        Self { 
            receipt, 
            from,
            fees,
            status,
            hash,
        }
    }

    pub fn print(&self, provider: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>) {
        timestamp_print!(Color::Blue, format!("Tx hash: {:#066x}", self.hash));
        timestamp_print!(Color::Blue, format!("You paid {} ETH of fees", wei_to_float(self.fees)));
        //timestamp_print!(Color::Blue, format!("Your balance: {} ETH", user_balance(provider)));
    }
}