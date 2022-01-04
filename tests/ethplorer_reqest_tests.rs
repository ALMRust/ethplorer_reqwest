#[cfg(test)]
mod tests {
    use std::env;
    use ethplorer::{GetAddressHistoryParams, GetAddressInfoParams, GetAddressTransactionsParams, GetTokenHistoryParams, GetTopParams};
    use ethplorer_reqwest::*;
    const TETHER: &str = "0xdac17f958d2ee523a2206206994597c13d831ec7";
    const BINANCE: &str = "0x3f5CE5FBFe3E9af3971dD833D26bA9b5C936f0bE";
    const ETHPLORER_KEY: &str = "ETHPLORER_KEY";

    #[test]
    fn get_last_block_works() {
        let key = env::var(ETHPLORER_KEY).unwrap();
        let data = get_last_block(&key).unwrap();
        assert!(data.last_block > 13935421);
    }

    #[test]
    fn get_address_info_works() {
        let key = env::var(ETHPLORER_KEY).unwrap();
        let data = get_address_info(
            &key,
            TETHER,
            &GetAddressInfoParams {
                token: "".to_string(),
                show_eth_totals: true,
            },
        ).unwrap();
        assert!(true);
    }

    #[test]
    fn get_top_token_holders_works() {
        let key = env::var(ETHPLORER_KEY).unwrap();
        let data = get_top_token_holders(
            &key,
            TETHER,
            5,
        ).unwrap();
        assert_eq!(data.holders.len(), 5);
    }

    #[test]
    fn get_top_tokens_works() {
        let key = env::var(ETHPLORER_KEY).unwrap();
        let data = get_top_tokens(&key).unwrap();
        assert!(data.tokens.len() > 0);
    }

    #[test]
    fn get_token_info_works() {
        let key = env::var(ETHPLORER_KEY).unwrap();
        let data = get_token_info(&key, TETHER).unwrap();
        assert_eq!(data.address, TETHER);
    }

    #[test]
    fn get_tokens_new_works() {
        let key = env::var(ETHPLORER_KEY).unwrap();
        let data = get_tokens_new(&key).unwrap();
        assert!(data.len() > 0);
    }

    #[test]
    fn get_token_daily_transaction_count_works() {
        let key = env::var(ETHPLORER_KEY).unwrap();
        let data = get_token_daily_transaction_count(
            &key,
            TETHER,
            5,
        ).unwrap();
        assert!(data.count_txs.len() > 0);
    }

    #[test]
    fn get_address_history_works() {
        let key = env::var(ETHPLORER_KEY).unwrap();
        let data = get_address_history(
            &key,
            BINANCE,
            &GetAddressHistoryParams {
                history_type: "".to_string(),
                limit: 5,
                timestamp: Default::default(),
                token: "".to_string()
            },
        ).unwrap();
        assert!(data.operations.len() > 0);
    }

    #[test]
    fn get_top_works() {
        let key = env::var(ETHPLORER_KEY).unwrap();
        let limit = 5;
        let data = get_top(&key, &GetTopParams {
            limit,
            criteria: "cap".to_string()
        }).unwrap();
        assert_eq!(data.tokens.len() as u64, limit + 1);
    }

    #[test]
    fn get_address_transactions_works() {
        let key = env::var(ETHPLORER_KEY).unwrap();
        let limit = 5;
        let data = get_address_transactions(
            &key,
            TETHER, // tether erc20,
            &GetAddressTransactionsParams {
                limit: 5,
                timestamp: Default::default(),
                show_zero_values: false,
            },
        ).unwrap();
        assert_eq!(data.len() as u64, limit);
    }

    #[test]
    fn get_token_history_works() {
        let key = env::var(ETHPLORER_KEY).unwrap();
        let limit = 5;
        let data = get_token_history(
            &key,
            "0xdac17f958d2ee523a2206206994597c13d831ec7", // tether erc20,
            &GetTokenHistoryParams {
                history_type: "".to_string(),
                limit: 5,
                timestamp: Default::default()
            },
        ).unwrap();
        assert_eq!(data.operations.len() as u64, limit);
    }

    #[test]
    fn get_token_daily_price_history_works() {
        let key = env::var(ETHPLORER_KEY).unwrap();
        let limit = 5;
        let data = get_token_daily_price_history(
            &key,
            TETHER, // tether erc20,
            5,
        ).unwrap();
        assert!(data.history.count_txs.len() > 0);
    }
}