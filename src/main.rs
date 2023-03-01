use mint_sniper::run;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    run().await;
}
