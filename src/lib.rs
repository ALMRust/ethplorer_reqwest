use ethplorer::*;
use reqwest::Error;
use serde::de::DeserializeOwned;

pub fn handle_request<T: DeserializeOwned>(config: RequestConfig) -> Result<T, Error> {
    let url = config.to_string();
    let client = reqwest::blocking::Client::new();
    println!("{}", &url);
    println!("{:?}", &config.params);
    let res = client.get(url).query(&config.params).send()?;
    res.json::<T>()
}

pub fn get_last_block(api_key: &str) -> Result<LastBlock, Error> {
    let config = get_last_block_config(api_key);
    handle_request(config)
}

pub fn get_address_info(
    api_key: &str,
    address: &str,
    params: &GetAddressInfoParams,
) -> Result<AddressInfo, Error> {
    let config = get_address_info_config(api_key, address, params);
    handle_request(config)
}

pub fn get_token_info(api_key: &str, address: &str) -> Result<TokenInfo, Error> {
    let config = get_token_info_config(api_key, address);
    handle_request(config)
}

pub fn get_top_token_holders(
    api_key: &str,
    address: &str,
    limit: u64,
) -> Result<TopTokenHolders, Error> {
    let config = get_top_token_holders_config(api_key, address, limit);
    handle_request(config)
}

pub fn get_tokens_new(api_key: &str) -> Result<Vec<TokenInfo>, Error> {
    let config = get_tokens_new_config(api_key);
    handle_request(config)
}

pub fn get_token_daily_transaction_count(
    api_key: &str,
    address: &str,
    period: u64,
) -> Result<TokenDailyTransactionCounts, Error> {
    let config = get_token_daily_transaction_count_config(api_key, address, period);
    handle_request(config)
}

pub fn get_token_history(
    api_key: &str,
    address: &str,
    params: &GetTokenHistoryParams,
) -> Result<TokenHistory, Error> {
    let config = get_token_history_config(api_key, address, params);
    handle_request(config)
}

pub fn get_address_history(
    api_key: &str,
    address: &str,
    params: &GetAddressHistoryParams,
) -> Result<TokenHistory, Error> {
    let config = get_address_history_config(api_key, address, params);
    handle_request(config)
}

pub fn get_address_transactions(
    api_key: &str,
    address: &str,
    params: &GetAddressTransactionsParams,
) -> Result<Vec<AddressTransaction>, Error> {
    let config = get_address_transactions_config(api_key, address, params);
    handle_request(config)
}

pub fn get_top_tokens(api_key: &str) -> Result<TopTokens, Error> {
    let config = get_top_tokens_config(api_key);
    handle_request(config)
}

pub fn get_top(api_key: &str, params: &GetTopParams) -> Result<TopTokens, Error> {
    let config = get_top_config(api_key, params);
    handle_request(config)
}

pub fn get_token_daily_price_history(
    api_key: &str,
    address: &str,
    period: u64,
) -> Result<TokenDailyPriceHistory, Error> {
    let config = get_token_daily_price_history_config(api_key, address, period);
    handle_request(config)
}
