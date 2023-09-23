use promptly::prompt;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::str::FromStr;

use ethers::prelude::*;

// Source of RPC URLs: https://rpc.info/sepolia
const SEPOLICA_RPCS: &[&str] = &[
    "https://rpc2.sepolia.org",
    "https://eth-sepolia.g.alchemy.com/v2/demo",
    "https://ethereum-sepolia.blockpi.network/v1/rpc/public",
    "https://rpc.sepolia.org",
    "https://endpoints.omniatech.io/v1/eth/sepolia/public",
];

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let provider = 'provider: {
        let rpcs = {
            let mut rpcs = Vec::from_iter(SEPOLICA_RPCS.into_iter().copied());
            rpcs.shuffle(&mut thread_rng());
            rpcs
        };

        for (url, p) in rpcs
            .iter()
            .map(|url| (url, Provider::<Http>::try_connect(url)))
        {
            println!("Connecting to Sepolia RPC at <{}>", url);
            match p.await {
                Ok(provider) => {
                    println!("Connected to Sepolia RPC at <{}>", url);
                    break 'provider provider;
                }
                Err(e) => {
                    println!("Failed to connect to Sepolia RPC at <{}>: {}", url, e);
                }
            }
        }
        eyre::bail!("Failed to connect to any Sepolia RPC");
    };
    let addr: String =
        prompt("Enter your address, e.g. \"0xEe52630b9e4629E47d0446c7a74cb17dd5D735EE\"")?;
    let addr = Address::from_str(addr.trim()).unwrap();
    let balance = provider.get_balance(addr, None).await?;

    // let block_number = provider.get_balance().await?;
    dbg!(balance);
    Ok(())
}
