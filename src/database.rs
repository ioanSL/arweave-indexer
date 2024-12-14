use mongodb::{
    bson::{doc, to_bson},
    options::ClientOptions,
    Client, Collection, Database,
};

use crate::transaction::Transaction;

#[derive(Debug, Clone)]
pub struct MongoDatabase {
    pub database: Database,
}

impl MongoDatabase {
    pub async fn new(mongo_url: &str) -> Self {
        let mongo_client_options = ClientOptions::parse(mongo_url).await.unwrap();
        let mongo_client = Client::with_options(mongo_client_options).unwrap();
        let database = mongo_client.database("arwave_indexer");

        MongoDatabase { database }
    }

    pub async fn store_transactions(&self, transactions: Vec<Transaction>) {
        for tx in transactions {
            let tx_data = to_bson(&tx).unwrap();
            let tx_collection = self.database.collection("transactions");
            tx_collection.insert_one(tx_data, None).await.unwrap();
        }
    }

    pub async fn get_transaction(&self, id: &str) -> mongodb::error::Result<Option<Transaction>> {
        let tx_collection: Collection<Transaction> = self.database.collection("transactions");
        let filter = doc! { "id": id };
        let transaction = tx_collection.find_one(filter, None).await?;
        Ok(transaction)
    }
}
