use crate::batcher::{Batch, SmartContract};
use crate::error::Error;
use crate::oracle::{GetPriceResponse, OracleQuery, TokenPrice};
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use secret_rpc::Contract;
use std::time::SystemTime;

mod batcher;
mod error;
mod oracle;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();

    let client = secret_rpc::SecretRPC::new()
        .host(dotenv::var("LS_HOST").unwrap())
        .enclave_key(dotenv::var("LS_KEY").unwrap())
        .connect()?;

    let router = Contract::try_from_address_with_code_hash(
        &dotenv::var("QUERY_ROUTER_ADDRESS").unwrap(),
        &dotenv::var("QUERY_ROUTER_CODE_HASH").unwrap(),
    )?;

    let oracle = SmartContract {
        address: dotenv::var("ORACLE_ADDRESS").unwrap(),
        code_hash: dotenv::var("ORACLE_CODE_HASH").unwrap(),
    };
    let oracle_c = Contract::try_from_address_with_code_hash(
        &dotenv::var("ORACLE_ADDRESS").unwrap(),
        &dotenv::var("ORACLE_CODE_HASH").unwrap(),
    )?;

    let total_queries = 500;

    // Generate queries to make it easier to process
    let queries = (0..total_queries)
        .map(|_| TokenPrice::query_msg("SHD".to_string()))
        .collect::<Vec<OracleQuery>>();

    let batches = [5, 10, 25, 50, 100];
    for b in batches {
        let mut success = 0.0;
        let mut fail = 0.0;

        println!(
            "Running {} queries in batches of {}. {} total queries",
            total_queries,
            b,
            total_queries / b
        );

        let now = SystemTime::now();
        let mut tasks = FuturesUnordered::new();
        for chunk in queries.chunks(b) {
            let mut batch = Batch::new();
            for q in chunk {
                batch.push(0u8, oracle.clone(), q.clone())?;
            }
            tasks.push(batch.query_into::<u8, GetPriceResponse>(&client, &router));
        }
        while let Some(res) = tasks.next().await {
            match res {
                Ok(_) => success += 1.0,
                Err(_) => fail += 1.0,
            }
        }

        let time = SystemTime::now().duration_since(now).unwrap().as_millis() as f32 / 1000 as f32;
        println!(
            "Took {} seconds with {}% success",
            time,
            (success / (success + fail)) * 100.0
        );
    }

    // Traditional queries
    println!("Running {} queries", total_queries);
    let mut success = 0.0;
    let mut fail = 0.0;

    let now = SystemTime::now();
    let mut tasks = FuturesUnordered::new();
    for _ in 0..total_queries {
        tasks.push(TokenPrice::query(&client, &oracle_c, "SHD".to_string()));
    }
    while let Some(res) = tasks.next().await {
        match res {
            Ok(_) => success += 1.0,
            Err(_) => fail += 1.0,
        }
    }
    let time = SystemTime::now().duration_since(now).unwrap().as_millis() as f32 / 1000 as f32;
    println!(
        "Took {} seconds with {}% success",
        time,
        (success / (success + fail)) * 100.0
    );

    Ok(())
}
