use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult,
};

use crate::msg::{
    AllContractsResponse, ContractResponse, ExecuteMsg, InstantiateMsg, OwnerResponse, QueryMsg,
};
use crate::state::{GlobalConfig, CONTRACTS, GLOBAL_CONFIG};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let config = GlobalConfig { owner: msg.owner };
    GLOBAL_CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("owner", config.owner.to_string()))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::UpdateOwner { new_owner } => execute_update_owner(deps, info, new_owner),
        ExecuteMsg::UpdateContract {
            name,
            address,
            code_hash,
        } => execute_update_contract(deps, info, name, address, code_hash),
        ExecuteMsg::RemoveContract { name } => execute_remove_contract(deps, info, name),
    }
}

fn execute_update_owner(
    deps: DepsMut,
    info: MessageInfo,
    new_owner: cosmwasm_std::Addr,
) -> StdResult<Response> {
    let mut config = GLOBAL_CONFIG.load(deps.storage)?;

    if info.sender != config.owner {
        return Err(StdError::generic_err("Unauthorized: only owner can update"));
    }

    config.owner = new_owner.clone();
    GLOBAL_CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "update_owner")
        .add_attribute("new_owner", new_owner.to_string()))
}

fn execute_update_contract(
    deps: DepsMut,
    info: MessageInfo,
    name: String,
    address: cosmwasm_std::Addr,
    code_hash: String,
) -> StdResult<Response> {
    let config = GLOBAL_CONFIG.load(deps.storage)?;

    if info.sender != config.owner {
        return Err(StdError::generic_err("Unauthorized: only owner can update"));
    }

    let contract_info = crate::state::ContractInfo { address, code_hash };
    CONTRACTS.insert(deps.storage, &name, &contract_info)?;

    Ok(Response::new()
        .add_attribute("action", "update_contract")
        .add_attribute("name", name))
}

fn execute_remove_contract(
    deps: DepsMut,
    info: MessageInfo,
    name: String,
) -> StdResult<Response> {
    let config = GLOBAL_CONFIG.load(deps.storage)?;

    if info.sender != config.owner {
        return Err(StdError::generic_err("Unauthorized: only owner can update"));
    }

    CONTRACTS.remove(deps.storage, &name)?;

    Ok(Response::new()
        .add_attribute("action", "remove_contract")
        .add_attribute("name", name))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOwner {} => to_binary(&query_owner(deps)?),
        QueryMsg::GetContract { name } => to_binary(&query_contract(deps, name)?),
        QueryMsg::GetAllContracts {} => to_binary(&query_all_contracts(deps)?),
    }
}

fn query_owner(deps: Deps) -> StdResult<OwnerResponse> {
    let config = GLOBAL_CONFIG.load(deps.storage)?;
    Ok(OwnerResponse {
        owner: config.owner,
    })
}

fn query_contract(deps: Deps, name: String) -> StdResult<ContractResponse> {
    let info = CONTRACTS
        .get(deps.storage, &name)
        .ok_or_else(|| StdError::generic_err(format!("Contract '{}' not found", name)))?;

    Ok(ContractResponse { name, info })
}

fn query_all_contracts(deps: Deps) -> StdResult<AllContractsResponse> {
    let contracts: Vec<ContractResponse> = CONTRACTS
        .iter(deps.storage)?
        .map(|item| {
            let (name, info) = item?;
            Ok(ContractResponse { name, info })
        })
        .collect::<StdResult<Vec<_>>>()?;

    Ok(AllContractsResponse { contracts })
}
