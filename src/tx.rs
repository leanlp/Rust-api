use ethers::prelude::*;
use std::sync::Arc;
use dotenv::dotenv;
use std::env;
use log::info;
use simple_logger::SimpleLogger;
use log::warn;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    SimpleLogger::new().with_utc_timestamps().init().unwrap();

    dotenv().ok();

    let rpc_url = env::var("RPC_URL").unwrap_or_else(|_| "Missing RPC URL".to_string());
    let private_key = env::var("PRIVATE_KEY").unwrap_or_else(|_| "Missing Private Key".to_string());


    let wallet: LocalWallet = private_key.parse()?;

    // Connect the wallet to the Ethereum network
    let provider = Provider::<Http>::try_from(rpc_url)?;
    info!("provider: {}", provider.get_chainid().await?.as_u64());
    let chain_id = provider.get_chainid().await?.as_u64();
    let wallet = wallet.with_chain_id(chain_id);

    let client = Arc::new(SignerMiddleware::new(provider, wallet));

    // Specify the recipient address and the amount to send
    let to_address: Address = "0xD20baecCd9F77fAA9E2C2B185F33483D7911f9C8".parse()?;
    let amount = 1;//ethers::utils::parse_ether(1)?; // Sending 1 ETH

    // Create and send the transaction
    let tx = TransactionRequest::new()
        .to(to_address)
        .value(amount)
        .chain_id(chain_id);
    info!("Sending transaction...");
    let pending_tx = client.send_transaction(tx, None).await?;

    // Await the transaction receipt
    let receipt = pending_tx.await?;
    info!("Transaction successful with receipt: {:?}", receipt);

    Ok(())
}
