use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::state::ContractInfo;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MigrateMsg {
    Migrate {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Update the global owner
    UpdateOwner {
        new_owner: Addr,
    },

    /// Register or update a contract's address and code hash
    UpdateContract {
        name: String,
        address: Addr,
        code_hash: String,
    },

    /// Remove a contract from registry
    RemoveContract {
        name: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Get the global owner
    GetOwner {},

    /// Get a specific contract's info by name
    GetContract {
        name: String,
    },

    /// Get multiple contracts by name
    GetContracts {
        names: Vec<String>,
    },

    /// Get all registered contracts
    GetAllContracts {},
}

// Query response types
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OwnerResponse {
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractResponse {
    pub name: String,
    pub info: ContractInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AllContractsResponse {
    pub contracts: Vec<ContractResponse>,
}
