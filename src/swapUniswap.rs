use dotenv::dotenv;
use ethers::abi::Abi;
use ethers::prelude::*;
use ethers::types::{Address, TransactionReceipt};
use std::env;
use std::sync::Arc;
use tokio::time::Duration;
//use std::time::{SystemTime, UNIX_EPOCH};

async fn swap_weth_to_eth(
    client: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
    weth_amount: i32,
) -> Result<TransactionReceipt, Box<dyn std::error::Error>> {
    dotenv().ok();

    let router_address = "0x3bFA4769FB09eefC5a80d6E87c3B9C650f7Ae48E".parse::<Address>()?;
    let weth_address = "0xfFf9976782d46CC05630D1f6eBAb18b2324d6B14".parse::<Address>()?;
    let usdc_address = "0x94a9D9AC8a22534E3FaCa9F4e7F2E2cf85d5E4C8".parse::<Address>()?;
    let abi = serde_json::from_slice::<Abi>(include_bytes!("./abiUniswapSwapRouter.json"))?;
    let router = Contract::new(router_address, abi, client.clone());

    let deadline = 1715999650;
    //SystemTime::now()        .duration_since(UNIX_EPOCH)?        .as_secs() + 1200;

    let swap_tx = router.method::<_, H256>(
        "exactInputSingle",
        (
            weth_address,
            usdc_address,
            3000,
            client.address(),
            deadline,
            weth_amount,
            1,
            0,
        ),
    )?;

    let pending_tx = swap_tx.send().await?;
    let receipt = client
        .get_transaction_receipt(pending_tx.tx_hash())
        .await?
        .ok_or("Transaction failed")?;

    Ok(receipt)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let rpc_url = env::var("SEPOLIA_RPC_URL").expect("RPC_URL not set in .env file");
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not set");
    let wallet: LocalWallet = private_key.parse()?;
    let provider = Provider::<Http>::try_from(rpc_url)?.interval(Duration::from_millis(500));
    let client = Arc::new(SignerMiddleware::new(provider, wallet.with_chain_id(11155111u64)));

    let weth_amount = 1;// U256::from_dec_str("1")?; // Amount of WETH to swap (1 WETH)

    match swap_weth_to_eth(client, weth_amount).await {
        Ok(receipt) => println!("Swap transaction successful: {:?}", receipt),
        Err(e) => eprintln!("Failed to swap WETH to ETH: {}", e),
    }

    Ok(())
}
