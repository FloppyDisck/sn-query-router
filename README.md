# Secret Network Query Router
A smart contract that allows for batching multiple queries.

Running multiple queries in a short time frame can be a challenge, particularly if the API or web service you are using enforces strict rate limits. These rate limits are designed to prevent excessive requests that could potentially harm the service, such as overloading their servers or impacting their performance.

A smart contract that batches queries for you can be an effective solution to this problem. Essentially, this smart contract acts as an intermediary between you and the API, receiving your individual queries, grouping them together into batches, and then sending them to the API as a single request.

By batching your queries in this way, you can reduce the total number of requests sent to the API, which can help you stay within the rate limits and avoid being rate limited. Moreover, batching can also reduce the latency of the overall system, as sending fewer requests results in fewer round trips between the client and the server.

## Results
All of the following results are run with 500 total queries

| Batch Size | RPC Queries | Time  | Success |
|------------|-------------|-------|---------|
| N/A        | 500         | 5.56s | 26.6%   |
| 5          | 100         | 6.51s | 100%    |
| 10         | 50          | 9.24s | 100%    |
| 25         | 20          | 7.27s | 100%    |
| 50         | 10          | 8.78s | 100%    |
| 100        | 5           | 7.72s | 100%    |


## Building
To build the contract simply run `cargo make contract` and use the generated `./contract/query_router_contract.wasm.gz`

## Running
To run the tests used here run `cargo make run`, you will need to create a `.env` file in the root dir with the following parameters.
```dotenv
LS_HOST=http://rpc.testnet.secretsaturn.net
LS_KEY=4fdbe595050032f5f3458eb665f3bf70d83ae04b14f3e6c02a90c6a3260b3a7c

ORACLE_ADDRESS=secret1egcud2xf22lvpja6wwvccgv6tdtplfhfvx0ppt
ORACLE_CODE_HASH=05dd283b9bb5e3c113c3c67f84d2163cb8190373badfda65976964c05e95d6f3

QUERY_ROUTER_ADDRESS=secret1g0d2d367majpx2kwn3dmhk0ff6x5mcnn7q8e6y
QUERY_ROUTER_CODE_HASH=72a09535b77b76862f7b568baf1ddbe158a2e4bbd0f0879c69ada9b398e31c1f
```

