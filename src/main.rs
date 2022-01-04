use std::env;
use ethplorer::{GetAddressHistoryParams, GetTopParams};
use ethplorer_reqwest::{get_address_history, get_last_block, get_token_daily_price_history, get_top, get_top_tokens};

const BINANCE: &str = "0x3f5CE5FBFe3E9af3971dD833D26bA9b5C936f0bE";
fn main() {
    let key = env::var("ETHPLORER_KEY").unwrap();
    let data = get_token_daily_price_history(
        &key,
        "0xdac17f958d2ee523a2206206994597c13d831ec7", // tether erc20,
        5,
    );
    match data {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{:?}", e),
    }
}
