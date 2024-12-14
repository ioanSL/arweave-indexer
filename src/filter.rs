use crate::tag::Tag;
use serde_json::Value;

pub fn create_filter() -> fn(&Value) -> bool {
    fn filter_fn(tx_data: &Value) -> bool {
        let tags: Vec<Tag> = tx_data["tags"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(Tag::from_json)
            .collect();

        let bundle_format = tags
            .iter()
            .any(|tag| tag.name == "Bundle-Format" && tag.value == "binary");
        let bundle_version = tags
            .iter()
            .any(|tag| tag.name == "Bundle-Version" && tag.value == "2.0.0");

        bundle_format && bundle_version
    }
    filter_fn
}
