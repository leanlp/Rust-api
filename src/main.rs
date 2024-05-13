use ethers::prelude::*;
use ethers::providers::{Provider, Http};
use std::sync::Arc;
use std::env;
use dotenv::dotenv;
//use log::{info, error};
use serde_json::Value;
use tokio::time::Duration;
use reqwest::Response;

async fn fetch_eth_usdt_price() -> Result<f64, Box<dyn std::error::Error>> {
    let url = "https://api.binance.com/api/v3/ticker/price?symbol=ETHUSDT";
    let response = reqwest::get(url).await?;
    parse_eth_usdt_price(response).await
}

async fn parse_eth_usdt_price(response: Response) -> Result<f64, Box<dyn std::error::Error>> {
    let text = response.text().await?;
    let json: Value = serde_json::from_str(&text)?;

    if let Some(price) = json["price"].as_str() {
        if let Ok(price_float) = price.parse::<f64>() {
            return Ok(price_float);
        }
    }

    Err("Failed to parse response".into())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    //env_logger::init();

    let contract_address = "0x3D18C60D612Dd0c0096aA83007dBa0B7F66a2418".parse::<Address>()?;
    abigen!(IERC721, "./src/abi.json");

    let rpc_url = env::var("SEPOLIA_RPC_URL").expect("RPC_URL not set in .env file");
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let provider = Arc::new(provider);
    let contract = IERC721::new(contract_address, provider.clone());

    let result: String = contract.method("symbol", ())?.call().await?;
    println!("Contract symbol: {}", result);

    loop {
        match fetch_eth_usdt_price().await {
            Ok(price) => println!("ETH/USDT Price: {}", price),
            Err(e) => eprintln!("Error: {}", e),
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}