
任务：
- 编写 Rust 函数，接收 Arbitrum 测试网地址，使用`ethers-rs`查询该地址的 ETH 余额；
- 余额需转换为可读格式（从 wei 转换为 ETH）；


查询ethers-rs库得知接口为：

![image.png](https://vstral-1300032599.cos.ap-chengdu.myqcloud.com/blog/20260109193014.png)


```rust
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
```

![image.png](https://vstral-1300032599.cos.ap-chengdu.myqcloud.com/blog/20260109200836.png)

成功获取到余额
