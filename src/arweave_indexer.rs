use core::str;

use reqwest::Client;
use serde_json::Value;

use crate::{database::MongoDatabase, transaction::Transaction};

const RPC_BASE_URL: &str = "https://arweave.net";

pub struct ArweaveIndexer {
    client: Client,
    database: MongoDatabase,
    interval: tokio::time::Interval,
    indexer_options: ArweaveIndexerOptions,
}

#[derive(Debug, Clone)]
pub struct ArweaveIndexerOptions {
    pub start_block: u64,
    pub checkpoints: Vec<u64>,
    pub filter: Option<fn(&Value) -> bool>,
}

impl ArweaveIndexer {
    pub async fn new(db: MongoDatabase, config: ArweaveIndexerOptions) -> Self {
        let client = Client::new();

        ArweaveIndexer {
            client,
            database: db,
            interval: tokio::time::interval(tokio::time::Duration::from_secs(5)),
            indexer_options: config,
        }
    }

    pub async fn fetch_last_block(&self) -> u64 {
        let endpoint = format!("{}/info", RPC_BASE_URL);
        match self.fetch_data(&endpoint).await {
            Ok(block) => block["height"].as_u64().unwrap(),
            Err(_) => panic!("Error fetching last block"),
        }
    }

    pub async fn start(&mut self) {
        let mut block_height = self.indexer_options.start_block;
        let checkpoints_list = self.indexer_options.checkpoints.clone();
        let custom_filter = self.indexer_options.filter.unwrap_or(|_| true);

        for checkpoint in checkpoints_list {
            let block_data = self.fetch_block_data(checkpoint).await.unwrap();
            self.fetch_transactions(&block_data, custom_filter)
                .await
                .unwrap();
        }

        let mut last_onchain_block = self.fetch_last_block().await;
        loop {
            if block_height < last_onchain_block {
                let block_data = self.fetch_block_data(block_height).await.unwrap();

                match self.fetch_transactions(&block_data, custom_filter).await {
                    Ok(_) => {
                        println!(
                            "Transactions fetched successfully at block {}",
                            block_data["height"]
                        );
                    }
                    Err(e) => eprintln!("Error fetching transactions: {:?}", e),
                }

                block_height += 1;
            } else {
                self.interval.tick().await;
                last_onchain_block = self.fetch_last_block().await;
            }
        }
    }

    pub async fn fetch_data(&self, endpoint: &str) -> Result<Value, reqwest::Error> {
        let response = self.client.get(endpoint).send().await?;
        let data = response.json::<Value>().await?;
        Ok(data)
    }

    pub async fn fetch_block_data(&self, block_height: u64) -> Result<Value, reqwest::Error> {
        let endpoint = format!("{}/block/height/{}", RPC_BASE_URL, block_height);
        self.fetch_data(&endpoint).await
    }

    pub async fn fetch_transactions<F>(
        &mut self,
        block_data: &Value,
        filter: F,
    ) -> Result<Vec<Value>, reqwest::Error>
    where
        F: Fn(&Value) -> bool,
    {
        let tx_ids = block_data["txs"].as_array();
        match tx_ids {
            Some(tx_ids) => {
                let mut transactions: Vec<Transaction> = vec![];
                for tx_id in tx_ids {
                    let endpoint = format!("{}/tx/{}", RPC_BASE_URL, tx_id.as_str().unwrap());
                    let tx_data = self.fetch_data(&endpoint).await?;

                    if filter(&tx_data) {
                        // println!("Transaction: {:?}", tx_id);
                        transactions.push(Transaction::from_json(&tx_data).unwrap());
                    }
                }

                self.database.store_transactions(transactions).await;
            }
            None => println!("No transactions found in block"),
        }
        return Ok(vec![]);
    }
}
