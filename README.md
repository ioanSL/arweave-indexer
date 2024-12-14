# Arweave Indexer

Arweave Indexer is a Rust-based application that indexes transactions from the Arweave blockchain and stores them in a MongoDB database. It also provides a GraphQL API to query the indexed data.

## Features

- Fetches the latest block number from the Arweave blockchain.
- Compares the latest block number with the last indexed block in the database.
- Synchronizes transactions from the blockchain to the database
- Provides a GraphQL API to query the indexed data.

## Prerequisites

- Rust (latest stable version)
- MongoDB
- Docker (optional)

## Install dependencies

```sh
cargo build --release
```

## How to use

### Configuration

Use `src/config/config.json` to configure the indexer. Example:

```json
{
    "start_block": 1567000,
    "checkpoints": [900, 845, 733],
    "mongo_url": "mongodb://localhost:27017"
}
```

### Database

Launch a MongoDB docker container

```sh
docker compose -f docker-compose-dev.yaml up
```

### Run

Run the indexer

```sh
cargo run --release
```

## Query

In order to fetch indexed transaction access the GraphQL Playground at `http://localhost:8000/graphql`. Example:

```graphql
{
  transaction (id: "tx id") {
    format
    id
    lastTx
    owner
    tags {
      name
      value
    }
    target
    quantity
    data
    dataSize
    dataTree
    dataRoot
    reward
    signature
  }
}
```