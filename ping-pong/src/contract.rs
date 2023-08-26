use cosmwasm_std::entry_point;

use cosmwasm_std::{
   attr, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, StdResult, Response, Uint64, StdError
};

use crate::error::ContractError;
use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg};
use crate::state::PING_COUNT;
use std::ops::Add;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    PING_COUNT.save(deps.storage, &Uint64::zero())?;
    let res = Response::new();
    Ok(res)

}


#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {

    match msg {
        ExecuteMsg::Ping {} => ping(deps, env, info),
        }
}

pub fn ping(deps: DepsMut, _env: Env, _info: MessageInfo) -> Result<Response, ContractError> {
    let mut count = Uint64::zero();
    let res: Result<Uint64, StdError> = PING_COUNT.update(deps.storage, |exists| {
        count = exists.add(Uint64::from(1u8));
        Ok(count)
    });
    res?;

    let mut res = Response::new();
    res.attributes.push(attr("ping_count", count));
    res.data = Some(to_binary("pong")?);
    Ok(res)
}

#[entry_point]
pub fn query(
    deps: Deps,
    _env: Env,
    _msg: QueryMsg,
) -> StdResult<Binary> {

    let count = PING_COUNT.load(deps.storage)?;
    to_binary(&count)
}



#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::from_binary;

    #[test]
    fn proper_instantiation() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {};

        let info = mock_info("creator", &[]);

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: Uint64 = from_binary(&res).unwrap();
        assert_eq!(Uint64::zero(), value) 
    }

    #[test]
    fn test_ping() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &[]);

        // Instantiate
        instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::Ping {};
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.attributes, vec![attr("ping_count", 1.to_string())]);
        let data: String = from_binary(&res.data.unwrap()).unwrap();
        assert_eq!(data, "pong");

        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.attributes, vec![attr("ping_count", 2.to_string())]);
        let data: String = from_binary(&res.data.unwrap()).unwrap();
        assert_eq!(data, "pong");
    }
}