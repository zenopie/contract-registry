use cosmwasm_std::Addr;
use schemars::JsonSchema;
use secret_toolkit::storage::{Item, Keymap};
use serde::{Deserialize, Serialize};

/// Global configuration
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GlobalConfig {
    pub owner: Addr,
}

/// Contract entry with address and code hash
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractInfo {
    pub address: Addr,
    pub code_hash: String,
}

// Storage keys
pub const GLOBAL_CONFIG: Item<GlobalConfig> = Item::new(b"global_config");

// Contract registry - stores all contract addresses and hashes
// Key format: contract name (e.g., "erth_token", "anml_token", "sscrt_token", "airdrop", "staking", "exchange", "allocation", "registration")
pub const CONTRACTS: Keymap<String, ContractInfo> = Keymap::new(b"contracts");
