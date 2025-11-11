use crate::BungeeError;
use bigdecimal::BigDecimal;
use core::str::FromStr;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::cast::FromPrimitive;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashSet;

/// Params for /api/v1/bungee/quote
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BungeeQuoteParams {
    pub user_address: String,
    pub origin_chain_id: BungeeSupportedChain,
    pub destination_chain_id: BungeeSupportedChain,
    pub input_token: String,
    pub input_amount: BigDecimal,
    pub receiver_address: String,
    pub output_token: String,
    #[serde(flatten)]
    pub optional_params: BungeeOptionalParams,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BungeeOptionalParams {
    pub slippage: Option<BigDecimal>,
    pub delegate_address: Option<String>,
    pub refuel: Option<bool>,
    #[serde(flatten)]
    pub destination: Option<BungeeDestinationPayload>,
    pub fee_bps: Option<BigDecimal>,
    pub fee_taker_address: Option<String>,
    pub enable_manual: Option<bool>,
    pub disable_swapping: Option<bool>,
    pub disable_auto: Option<bool>,
    pub exclude_bridges: Option<HashSet<BungeeSupportedBridge>>,
    pub include_bridges: Option<HashSet<BungeeSupportedBridge>>,
    pub exclude_dexes: Option<HashSet<BungeeSupportedDex>>,
    pub include_dexes: Option<HashSet<BungeeSupportedDex>>,
    pub exclusive_transmitter: Option<String>,
    pub use_inbox: Option<bool>,
}

/// Optional Destination Payload and Destination Gas Limit params
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BungeeDestinationPayload {
    pub destination_payload: String,
    pub destination_gas_limit: BigDecimal,
}

/// Bungee supported chains with Bungee chain IDs
/// List from https://public-backend.bungee.exchange/api/v1/supported-chains
#[derive(
    Serialize_repr,
    Deserialize_repr,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Hash,
    ToPrimitive,
    FromPrimitive,
)]
#[repr(u32)]
pub enum BungeeSupportedChain {
    #[serde(rename = "Ethereum")]
    Ethereum = 1,
    #[serde(rename = "Optimism")]
    Optimism = 10,
    #[serde(rename = "BNB")]
    Bnb = 56,
    #[serde(rename = "Gnosis")]
    Gnosis = 100,
    #[serde(rename = "Unichain")]
    Unichain = 130,
    #[serde(rename = "Polygon")]
    Polygon = 137,
    #[serde(rename = "Sonic")]
    Sonic = 146,
    #[serde(rename = "zkSync Era")]
    ZksyncEra = 324,
    #[serde(rename = "World Chain")]
    WorldChain = 480,
    #[serde(rename = "HyperEVM")]
    HyperEvm = 999,
    #[serde(rename = "Polygon zkEVM")]
    PolygonZkevm = 1101,
    #[serde(rename = "Sei")]
    Sei = 1329,
    #[serde(rename = "Soneium")]
    Soneium = 1868,
    #[serde(rename = "Abstract")]
    Abstract = 2741,
    #[serde(rename = "Mantle")]
    Mantle = 5000,
    #[serde(rename = "Base")]
    Base = 8453,
    #[serde(rename = "Plasma")]
    Plasma = 9745,
    #[serde(rename = "Mode")]
    Mode = 34443,
    #[serde(rename = "Arbitrum")]
    Arbitrum = 42161,
    #[serde(rename = "Avalanche")]
    Avalanche = 43114,
    #[serde(rename = "Ink")]
    Ink = 57073,
    #[serde(rename = "Linea")]
    Linea = 59144,
    #[serde(rename = "Berachain")]
    Berachain = 80094,
    #[serde(rename = "Blast")]
    Blast = 81457,
    #[serde(rename = "Solana")]
    Solana = 89999,
    #[serde(rename = "Plume")]
    Plume = 98866,
    #[serde(rename = "Scroll")]
    Scroll = 534352,
    #[serde(rename = "Katana")]
    Katana = 747474,
    #[serde(rename = "Tron")]
    Tron = 728126428,
}

impl TryFrom<String> for BungeeSupportedChain {
    type Error = BungeeError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        let id = u32::from_str(&s)
            .map_err(|err| BungeeError::BadChainId(format!("Unable to parse string: {err:?}")))?;
        let chain = Self::from_u32(id).ok_or(BungeeError::BadChainId(s))?;
        Ok(chain)
    }
}

impl From<BungeeSupportedChain> for String {
    fn from(chain: BungeeSupportedChain) -> Self {
        (chain as u32).to_string()
    }
}

/// Bungee supported bridges
/// List from https://public-backend.bungee.exchange/api/v1/supported-chains
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum BungeeSupportedBridge {
    #[serde(rename = "optimism-bridge")]
    OptimismBridge,
    #[serde(rename = "cctp-v2")]
    CctpV2,
    #[serde(rename = "across")]
    Across,
    #[serde(rename = "symbiosis")]
    Symbiosis,
    #[serde(rename = "polygon-bridge")]
    PolygonBridge,
    #[serde(rename = "zksync-native")]
    ZksyncNative,
    #[serde(rename = "stargate-v2")]
    StargateV2,
    #[serde(rename = "arbitrum-bridge")]
    ArbitrumBridge,
    #[serde(rename = "celer")]
    Celer,
    #[serde(rename = "base-bridge")]
    BaseBridge,
    #[serde(rename = "synapse")]
    Synapse,
    #[serde(rename = "gnosis-native-bridge")]
    GnosisNativeBridge,
    #[serde(rename = "mantle-native-bridge")]
    MantleNativeBridge,
    #[serde(rename = "scroll-native-bridge")]
    ScrollNativeBridge,
    #[serde(rename = "mode-native-bridge")]
    ModeNativeBridge,
    #[serde(rename = "ink-native-bridge")]
    InkNativeBridge,
    #[serde(rename = "cctp-v2-fast")]
    CctpV2Fast,
    #[serde(rename = "cctp")]
    Cctp,
    #[serde(rename = "zora-bridge")]
    ZoraBridge,
    #[serde(rename = "mayan")]
    Mayan,
    #[serde(rename = "spectral-signal")]
    SpectralSignal,
    #[serde(rename = "aavegotchi-mainnet")]
    AavegotchiMainnet,
    #[serde(rename = "b3-mainnet")]
    B3Mainnet,
    #[serde(rename = "b3-native-bridge")]
    B3NativeBridge,
}

/// Bungee supported dexes
/// List from https://public-backend.bungee.exchange/api/v1/supported-chains
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum BungeeSupportedDex {
    #[serde(rename = "rainbow")]
    Rainbow,
    #[serde(rename = "zeroxv2")]
    ZeroXv2,
    #[serde(rename = "oneinch")]
    OneInch,
    #[serde(rename = "openocean")]
    OpenOcean,
    #[serde(rename = "kyberswap")]
    KyberSwap,
    #[serde(rename = "magpie")]
    Magpie,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BungeeApiResponse {
    pub success: bool,
    pub status_code: u32,
    pub result: Option<BungeeQuoteResult>,
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BungeeQuoteResult {
    pub origin_chain_id: BungeeSupportedChain,
    pub destination_chain_id: BungeeSupportedChain,
    pub receiver_address: String,
    pub user_address: String,
    pub input: BungeeQuoteResultInput,
    pub destination_exec: Option<BungeeDestinationPayload>,
    pub auto_route: Option<AutoRoute>,
    pub manual_routes: Vec<ManualRoute>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BungeeQuoteResultInput {
    pub token: BungeeToken,
    pub amount: BigDecimal,
    pub price_in_usd: BigDecimal,
    pub value_in_usd: BigDecimal,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BungeeToken {
    pub chain_id: BungeeSupportedChain,
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u32,
    #[serde(rename = "logoURI")]
    pub logo_uri: String,
    pub icon: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AutoRoute {
    pub user_op: String,
    pub request_hash: String,
    pub output: BungeeRouteOutput,
    pub request_type: BungeeQuoteRequestType,
    pub approval_data: Option<BungeeApprovalData>,
    pub affiliate_fee: Option<BungeeAffiliateFee>,
    pub sign_typed_data: Option<BungeeSignTypedData>,
    pub gas_fee: Option<BungeeGasFee>,
    pub slippage: BigDecimal,
    pub suggested_client_slippage: BigDecimal,
    pub tx_data: Option<BungeeTxData>,
    pub estimated_time: BigDecimal,
    pub route_details: BungeeRouteDetails,
    pub refuel: Option<BungeeRefuel>,
    pub quote_id: String,
    pub quote_expiry: u64,
    pub rewards: BungeeRewards,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ManualRoute {
    pub quote_id: String,
    pub output: BungeeRouteOutput,
    pub affiliate_fee: Option<BungeeAffiliateFee>,
    pub approval_data: Option<BungeeApprovalData>,
    pub gas_fee: BungeeGasFee,
    pub slippage: BigDecimal,
    pub estimated_time: BigDecimal,
    pub route_details: BungeeRouteDetails,
    pub refuel: Option<BungeeRefuel>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BungeeQuoteRequestType {
    SingleOutputRequest,
    SwapRequest,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BungeeRouteOutput {
    pub token: BungeeToken,
    pub amount: BigDecimal,
    pub price_in_usd: BigDecimal,
    pub value_in_usd: BigDecimal,
    pub min_amount_out: BigDecimal,
    pub effective_received_in_usd: BigDecimal,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BungeeApprovalData {
    pub spender_address: String,
    pub amount: BigDecimal,
    pub token_address: String,
    pub user_address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BungeeAffiliateFee {
    pub token: BungeeToken,
    pub amount: BigDecimal,
    pub fee_taker_address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BungeeSignTypedData {
    pub domain: BungeeSignTypedDataDomain,
    pub types: serde_json::Value,
    pub values: serde_json::Value,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BungeeSignTypedDataDomain {
    pub name: String,
    pub chain_id: BungeeSupportedChain,
    pub verifying_contract: String,
    pub version: String,
    pub salt: serde_json::Value,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BungeeGasFee {
    pub gas_token: BungeeToken,
    pub gas_limit: BigDecimal,
    pub gas_price: BigDecimal,
    pub estimated_fee: BigDecimal,
    pub fee_in_usd: BigDecimal,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BungeeTxData {
    pub data: String,
    pub to: String,
    pub chain_id: BungeeSupportedChain,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BungeeRouteDetails {
    pub name: String,
    #[serde(rename = "logoURI")]
    pub logo_uri: String,
    pub route_fee: BungeeRouteFee,
    pub dex_details: BungeeDexDetails,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BungeeRouteFee {
    pub token: BungeeToken,
    pub amount: BigDecimal,
    pub fee_in_usd: BigDecimal,
    pub price_in_usd: BigDecimal,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BungeeDexDetails {
    pub protocol: BungeeDexProtocol,
    pub min_amount_out: BigDecimal,
    pub output_token_address: String,
    pub input_token_address: String,
    pub amount_out: BigDecimal,
    pub slippage: BigDecimal,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BungeeDexProtocol {
    pub name: String,
    pub display_name: String,
    pub icon: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BungeeRefuel {
    pub input: BungeeRefuelAmount,
    pub output: BungeeRefuelAmount,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BungeeRefuelAmount {
    pub token: BungeeToken,
    pub amount: BigDecimal,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BungeeRewards {
    pub rebate_amount: BigDecimal,
    pub reward_amount: BigDecimal,
    pub total_reward_amount: BigDecimal,
    pub total_reward_amount_in_usd: BigDecimal,
    pub token: BungeeToken,
    pub is_reward_enabled: bool,
}
