use std::error::Error;
use clap::Parser;
use mint_sniper::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::parse();
    let user = User::parse();

    let web3 = init_connection().unwrap();
    check_on_config(&config, &user.address, &web3).await?;

    Ok(())
}

// mockERC721 contract deployed on goerli: 0x2F7F6a9cE73529354fc62bd42632da7A5084CC27