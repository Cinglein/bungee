pub mod client;
pub mod error;
pub mod types;

pub use client::*;
pub use error::*;
pub use types::*;

#[cfg(test)]
pub mod tests {
    use crate::*;
    use bigdecimal::BigDecimal;
    use core::str::FromStr;
    use num_traits::cast::*;

    #[tokio::test]
    async fn test_request() {
        let params = BungeeQuoteParams {
            user_address: "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045".to_string(),
            origin_chain_id: BungeeSupportedChain::from_u32(42161).expect("invalid chain id"),
            destination_chain_id: BungeeSupportedChain::from_u32(10).expect("invalid chain id"),
            input_token: "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE".to_string(),
            input_amount: BigDecimal::from_str("100000000000000").unwrap(),
            receiver_address: "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045".to_string(),
            output_token: "0x0b2c639c533813f4aa9d7837caf62653d097ff85".to_string(),
            optional_params: Default::default(),
        };
        let client = BungeeClient::default();
        let response = client
            .get_quote(&params)
            .await
            .expect("unable to get bungee response");
        assert!(response.success);
    }
}
