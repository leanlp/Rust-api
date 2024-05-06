use serde_json::Value;
use reqwest::{Error as ReqwestError, Response};
use std::time::Duration;

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
async fn main() {
    loop {
        match fetch_eth_usdt_price().await {
            Ok(price) => println!("ETH/USDT Price: {}", price),
            Err(e) => eprintln!("Error: {}", e),
        }
        // Wait for 5 seconds before fetching again
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
