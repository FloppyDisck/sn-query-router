use crate::{ExecuteMsg, InstantiateMsg, QueryAnswer, QueryMsg, QueryResponse, Res};
use cosmwasm_std::{
    entry_point, to_binary, to_vec, Binary, ContractResult, Deps, DepsMut, Empty, Env, MessageInfo,
    QuerierResult, QueryRequest, Response, StdError, StdResult, SystemResult, WasmQuery,
};

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::new())
}

#[entry_point]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> StdResult<Response> {
    Ok(Response::new())
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    to_binary(&match msg {
        QueryMsg::Batch { queries } => {
            let mut responses = vec![];
            for query in queries {
                let raw = to_vec::<QueryRequest<Empty>>(
                    &WasmQuery::Smart {
                        contract_addr: query.contract.address.clone(),
                        code_hash: query.contract.code_hash.clone(),
                        msg: query.query,
                    }
                    .into(),
                )
                .map_err(|serialize_err| {
                    StdError::generic_err(format!("Serializing QueryRequest: {}", serialize_err))
                })?;

                responses.push(QueryResponse {
                    id: query.id,
                    contract: query.contract,
                    response: match deps.querier.raw_query(&raw) {
                        QuerierResult::Err(system_err) => Res::SystemErr(system_err.to_string()),
                        SystemResult::Ok(ContractResult::Err(contract_err)) => {
                            Res::ContractErr(contract_err.to_string())
                        }
                        QuerierResult::Ok(ContractResult::Ok(res)) => Res::Response(res),
                    },
                });
            }
            QueryAnswer::Batch { responses }
        }
    })
}
