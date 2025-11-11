use crate::*;

pub const BUNGEE_QUOTE_URL: &str = "https://public-backend.bungee.exchange/api/v1/bungee/quote";

/// Bungee REST client
#[derive(Clone, Debug, Default)]
pub struct BungeeClient {
    client: reqwest::Client,
}

impl BungeeClient {
    pub async fn get_quote(
        &self,
        params: &BungeeQuoteParams,
    ) -> Result<BungeeApiResponse, BungeeError> {
        let parsed = self
            .client
            .get(BUNGEE_QUOTE_URL)
            .query(params)
            .send()
            .await?
            .json()
            .await?;
        Ok(parsed)
    }
}
