use crate::error::Error;
use secret_rpc::{Client, Contract};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TokenPrice {
    pub rate: String,
    pub last_updated_base: u128,
    pub last_updated_quote: u128,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct GetPriceResponse {
    pub key: String,
    pub data: TokenPrice,
}

impl TokenPrice {
    pub fn query_msg(token: String) -> OracleQuery {
        OracleQuery::get_price(token)
    }

    pub async fn query(client: &Client, contract: &Contract, token: String) -> Result<Self, Error> {
        let res: GetPriceResponse = client
            .query_contract(&OracleQuery::get_price(token), &contract, &secret_rpc::a())
            .await?;

        Ok(res.data)
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum OracleQuery {
    GetPrice { key: String },
}

impl OracleQuery {
    pub fn get_price(token: String) -> Self {
        OracleQuery::GetPrice { key: token }
    }
}
