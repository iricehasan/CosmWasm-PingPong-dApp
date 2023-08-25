use cosmwasm_std::entry_point;

use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, StdResult, Response,
};

use crate::error::ContractError;
use crate::msg::{InstantiateMsg, ExecuteMsg, Query};
use crate::state::{PING_COUNT};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response, ContractError> {

    PING_COUNT.save(deps.storage, &Uint64::zero())?;
    let res = Response::new();
    Ok(res)

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

    let count = PING_COUNT.load(deps.storage)?;
    to_binary(&count)
}


