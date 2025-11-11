use thiserror::Error;

#[derive(Debug, Error)]
pub enum BungeeError {
    #[error("Error sending a request to Bungee")]
    RequestError(#[from] reqwest::Error),
    #[error("Invalid chain ID")]
    BadChainId(String),
}
