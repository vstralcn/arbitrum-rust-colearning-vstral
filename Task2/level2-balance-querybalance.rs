use ethers::prelude::*;
use std::convert::TryFrom;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rpc_url = "https://arbitrum-sepolia.drpc.org";
    let provider = Provider::<Http>::try_from(rpc_url)?;

    let my_address = "0x038405665520AC945D26eB7936ffe0115B2f2BBd".parse::<Address>()?;

    println!("开始连接测试网");
    let balance = provider.get_balance(my_address, None).await?;
    let ether_balance = ethers::utils::format_ether(balance);

    println!("余额为：{} ETH", ether_balance);

    Ok(())
}
