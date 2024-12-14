use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::tag::Tag;

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
pub struct Transaction {
    format: i32,
    id: String,
    last_tx: String,
    owner: String,
    tags: Vec<Tag>,
    target: String,
    quantity: String,
    data: String,
    data_size: String,
    data_tree: Vec<String>,
    data_root: String,
    reward: String,
    signature: String,
}

impl Transaction {
    pub fn from_json(value: &Value) -> Option<Self> {
        let format = value["format"].as_u64()? as i32;
        let id = value["id"].as_str()?.to_string();
        let last_tx = value["last_tx"].as_str()?.to_string();
        let owner = value["owner"].as_str()?.to_string();
        let tags = value["tags"]
            .as_array()?
            .iter()
            .map(|v| Tag::from_json(v).unwrap())
            .collect();
        let target = value["target"].as_str()?.to_string();
        let quantity = value["quantity"].as_str()?.to_string();
        let data = value["data"].as_str()?.to_string();
        let data_size = value["data_size"].as_str()?.to_string();
        let data_tree = value["data_tree"]
            .as_array()?
            .iter()
            .map(|v| v.as_str().unwrap_or("").to_string())
            .collect();
        let data_root = value["data_root"].as_str()?.to_string();
        let reward = value["reward"].as_str()?.to_string();
        let signature = value["signature"].as_str()?.to_string();

        Some(Transaction {
            format,
            id,
            last_tx,
            owner,
            tags,
            target,
            quantity,
            data,
            data_size,
            data_tree,
            data_root,
            reward,
            signature,
        })
    }
}
