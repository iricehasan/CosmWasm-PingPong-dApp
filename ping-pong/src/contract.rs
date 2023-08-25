use cosmwasm_std::entry_point;

use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, StdResult, Response,
};

use crate::msg::{InstantiateMsg, ExecuteMsg, Query};
use crate::state::{State, STATE};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response, ContractError> {
    PING_COUNT.save(deps.storage, &0u64)?;
}


#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> StdResult<Response, ContractError> {

    unimplemented!()
}



#[entry_point]
pub fn query(
    deps: Deps,
    _env: Env,
    _msg: QueryMsg,
) -> StdResult<Binary, ContractError> {

    unimplemented!()
}
