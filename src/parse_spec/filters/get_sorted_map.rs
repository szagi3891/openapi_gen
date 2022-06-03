use std::cmp::Ordering;
use serde_json::Value;

pub fn get_sorted_map(mapping: impl Iterator<Item = (String, Value)>) -> Vec<(String, Value)> {
    let mut list = Vec::new();

    for (name, value) in mapping {
        list.push((name, value));
    }

    list.sort_by(|(akey, _), (bkey, _)| -> Ordering {
        akey.cmp(bkey)
    });

    list
}
