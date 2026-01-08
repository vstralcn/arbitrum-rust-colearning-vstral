use ethers::prelude::*;
use std::convert::TryFrom;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rpc_url = "https://arbitrum-sepolia-testnet.api.pocket.network";

    let provider = Provider::<Http>::try_from(rpc_url)?;

    println!("正在尝试连接到 Arbitrum Sepolia...");

    let block_number = provider.get_block_number().await?;
    let chain_id = provider.get_chainid().await?;

    println!("-------------------------------------------");
    println!("Hello Web3!");
    println!("链 ID (Chain ID): {}", chain_id);
    println!("当前区块高度: {}", block_number);
    println!("-------------------------------------------");

    Ok(())
}