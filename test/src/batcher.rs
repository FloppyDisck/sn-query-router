use crate::error::Error;
use base64::{engine::general_purpose, Engine as _};
use secret_rpc::{Client, Contract};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum QueryMsg {
    Batch {
        // Type T since we have two variations
        queries: Vec<RawQuery>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) struct RawQuery {
    pub id: String,
    pub contract: SmartContract,
    pub query: String,
}

impl<I, Q> TryFrom<Query<I, Q>> for RawQuery
where
    I: Serialize + DeserializeOwned,
    Q: Serialize + DeserializeOwned,
{
    type Error = Error;

    fn try_from(value: Query<I, Q>) -> Result<Self, Self::Error> {
        Ok(Self {
            id: general_purpose::STANDARD.encode(serde_json::to_string(&value.id)?),
            contract: value.contract,
            query: general_purpose::STANDARD.encode(serde_json::to_string(&value.query)?),
        })
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum QueryAnswer {
    Batch { responses: Vec<QueryAnswerResponse> },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) struct QueryAnswerResponse {
    pub id: String,
    pub contract: SmartContract,
    pub response: QueryResponseType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub(crate) enum QueryResponseType {
    SystemErr(String),
    ContractErr(String),
    Response(String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SmartContract {
    pub address: String,
    pub code_hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Batch(Vec<RawQuery>);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmptyBatchKey;

impl Batch {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn push<I, Q>(&mut self, id: I, contract: SmartContract, query: Q) -> Result<(), Error>
    where
        I: Serialize + DeserializeOwned,
        Q: Serialize + DeserializeOwned,
    {
        let q = Query {
            id,
            contract,
            query,
        };
        self.0.push(q.try_into()?);

        Ok(())
    }

    async fn query_raw(
        self,
        client: &Client,
        contract: &Contract,
    ) -> Result<Vec<QueryAnswerResponse>, Error> {
        let res: QueryAnswer = client
            .query_contract(
                &QueryMsg::Batch { queries: self.0 },
                contract,
                &secret_rpc::a(),
            )
            .await?;

        Ok(match res {
            QueryAnswer::Batch { responses } => responses,
        })
    }

    pub async fn query_into<I, Q>(
        self,
        client: &Client,
        contract: &Contract,
    ) -> Result<Vec<Response<I, Q>>, Error>
    where
        I: Serialize + DeserializeOwned,
        Q: Serialize + DeserializeOwned,
    {
        let mut results = vec![];
        for res in self.query_raw(client, contract).await? {
            results.push(res.try_into()?);
        }
        Ok(results)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Query<I, Q> {
    pub id: I,
    pub contract: SmartContract,
    pub query: Q,
}

#[derive(Debug)]
pub struct RawResponse {
    id: String,
    contract: SmartContract,
    response: Result<String, Error>,
}

impl From<QueryAnswerResponse> for RawResponse {
    fn from(value: QueryAnswerResponse) -> Self {
        Self {
            id: value.id,
            contract: value.contract,
            response: match value.response {
                QueryResponseType::SystemErr(e) => Err(Error::BatchError(e)),
                QueryResponseType::ContractErr(e) => Err(Error::BatchError(e)),
                QueryResponseType::Response(r) => Ok(r),
            },
        }
    }
}

#[derive(Debug)]
pub struct Response<I, Q> {
    pub id: I,
    pub contract: SmartContract,
    pub response: Result<Q, Error>,
}

impl<I, Q> TryFrom<QueryAnswerResponse> for Response<I, Q>
where
    I: Serialize + DeserializeOwned,
    Q: Serialize + DeserializeOwned,
{
    type Error = Error;

    fn try_from(value: QueryAnswerResponse) -> Result<Self, Self::Error> {
        let raw: RawResponse = value.into();
        Ok(Self {
            id: serde_json::from_slice(&general_purpose::STANDARD.decode(&raw.id)?)?,
            contract: raw.contract,
            response: raw.response.and_then(|r| {
                serde_json::from_slice(&general_purpose::STANDARD.decode(&r).unwrap())
                    .map_err(|err| err.into())
            }),
        })
    }
}
