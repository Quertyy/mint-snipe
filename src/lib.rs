pub struct Config {
    pub contract_address: String,
    pub contract_abi: String,
    pub price: u128,
    pub amount: u128,
    pub timestamp: u128,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let contract_address = args[1].clone();
        let contract_abi = args[2].clone();
        let price = args[3].parse().unwrap();
        let amount = args[4].parse().unwrap();
        let timestamp = args[5].parse().unwrap();

        Ok(Config {
            contract_address,
            contract_abi,
            price,
            amount,
            timestamp,
        })
    }
}