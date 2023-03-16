pub mod contract;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {}

#[cw_serde]
pub struct Contract {
    pub address: String,
    pub code_hash: String,
}

#[cw_serde]
pub struct Query {
    pub id: Binary,
    pub contract: Contract,
    pub query: Binary,
}

#[cw_serde]
pub enum QueryMsg {
    Batch { queries: Vec<Query> },
}

#[cw_serde]
pub enum QueryAnswer {
    Batch { responses: Vec<QueryResponse> },
}

#[cw_serde]
pub struct QueryResponse {
    id: Binary,
    contract: Contract,
    response: Res,
}

#[cw_serde]
pub enum Res {
    SystemErr(String),
    ContractErr(String),
    Response(Binary),
}
