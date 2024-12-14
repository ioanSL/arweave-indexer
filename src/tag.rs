use core::str;

use base64::decode;
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, GraphQLObject)]
pub struct Tag {
    pub name: String,
    pub value: String,
}

impl Tag {
    pub fn from_json(value: &Value) -> Option<Self> {
        let name = value["name"].as_str()?;
        let value = value["value"].as_str()?;
        let decoded_name = decode(name).ok()?;
        let decoded_value = decode(value).ok()?;

        Some(Tag {
            name: str::from_utf8(&decoded_name).unwrap().to_string(),
            value: str::from_utf8(&decoded_value).unwrap().to_string(),
        })
    }
}
