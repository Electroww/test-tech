use serde_json::Value;
use std::collections::HashMap;

pub fn count(record: &Vec<Value>) -> HashMap<String, i32> {
    let mut result: HashMap<String, i32> = HashMap::new();

    record.iter().for_each(| company | {
        let ape = company["fields"]["activiteprincipaleetablissement"].as_str();
        if let Some(ape) = ape {
            *result.entry(ape.to_string()).or_insert(0) += 1;
        } else {
            *result.entry("NA".to_string()).or_insert(0) += 1;
        }
    });

    return result;
}
