use serde_json::{Value, Map};
use super::get_sorted_map;

pub fn filter_null(value: Value) -> Value {
    match value {
        Value::Object(props) => {
            let props_iterator = props
                .into_iter()
                .filter(|(_key, value)| {
                    let is_retain = *value != Value::Null;
                    is_retain
                })
                .map(|(key, value)| (key, filter_null(value)))
            ;

            let mut new_map = Map::new();

            for (key, value) in get_sorted_map(props_iterator) {
                new_map.insert(key, value);
            }

            Value::Object(new_map)
        },
        Value::Array(list) => {
            let new_list = list
                .into_iter()
                .map(filter_null)
                .collect::<Vec<Value>>();

            Value::Array(new_list)
        },
        rest => rest,
    }
}