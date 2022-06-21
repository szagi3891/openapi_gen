use serde_json::{Value, Map};

use crate::utils::ErrorProcess;

use super::get_sorted_map::get_sorted_map;

fn deref_from_path(all: &Value, ref_path: &str) -> Value {
    let target_value = go_to_spec(all, ref_path);

    let target_value = match target_value {
        Ok(value) => value,
        Err(err) => {
            println!("{err:?}");
            panic!("Problem with finding ref: {ref_path}");
        }
    };

    target_value.clone()
}

fn deref_ref_attribute(all: &Value, value: &Value) -> Option<Value> {
    if let Value::Object(props) = value {
        if let Some(Value::String(path)) = props.get("$ref") {
            let target_value = deref_from_path(all, path.as_str());
            return Some(deref(target_value, all));
        }
    }

    None
}

fn deref_ref_in_discriminator(all: &Value, value: &Value) -> Option<Value> {
    if let Value::Object(props) = value {
        if let Some(Value::Object(props_mapping)) = props.get("mapping") {
            let mut result_mapping = Map::new();

            for (key, ref_path) in props_mapping.into_iter() {
                let ref_path = if let Value::String(inner) = ref_path {
                    inner.clone()
                } else {
                    panic!("Expected string");
                };
                
                let value_deref = deref_from_path(all, ref_path.as_str());
                result_mapping.insert(key.clone(), value_deref);
            }

            let mut new_props = serde_json::Map::new();
            new_props.insert(String::from("mapping"), Value::Object(result_mapping));
            return Some(Value::Object(new_props));
        }
    }

    None
}

pub fn deref(value: Value, all: &Value) -> Value {
    if let Some(new_value) = deref_ref_attribute(all, &value) {
        return new_value;
    }

    if let Some(new_value) = deref_ref_in_discriminator(all, &value) {
        return new_value;
    }

    match value.clone() {
        Value::Object(props) => {
            let mut new_map = Map::new();

            for (key, value) in get_sorted_map(props.into_iter()) {
                new_map.insert(key, deref(value, all));
            }

            Value::Object(new_map)
        },
        Value::Array(list) => {
            let mut new_list = Vec::new();

            for item in list.into_iter() {
                new_list.push(deref(item, all));
            }

            Value::Array(new_list)
        },
        rest => rest
    }
}

pub fn filter_deref(value: Value) -> Value {
    let all = value.clone();
    deref(value, &all)
}


fn parse_ref_path(path: &str) -> Result<Vec<String>, ErrorProcess> {
    if let Some((_, path)) = path.split_once("#/") {
        Ok(path.split("/").map(|item| item.to_string()).collect())        
    } else {
        Err(ErrorProcess::message(format!("invalid ref {path}")))
    }
}

fn go_to_spec_property<'a>(spec: &'a Value, property: &str, ref_path: &str) -> Result<&'a Value, ErrorProcess> {
    match spec {
        Value::Object(data) => {
            match data.get(property) {
                Some(value) => {
                    Ok(value)
                },
                None => {
                    let message = format!("invalid ref {ref_path}, Error with reference to property = {property}, no value");
                    Err(ErrorProcess::message(message))
                }
            }
        },
        _ => {
            let message = format!("invalid ref {ref_path}, Error with reference to property = {property}, no map");
            Err(ErrorProcess::message(message))
        }
    }
}

fn go_to_spec<'a>(all_spec: &'a Value, ref_path: &str) -> Result<&'a Value, ErrorProcess> {
    let ref_path_list = parse_ref_path(&ref_path)?;

    let mut current = all_spec;

    for property in ref_path_list {
        current = go_to_spec_property(current, &property, ref_path)?;
    }

    Ok(current)
}


#[test]
fn test_parse_ref() {
    let result: Vec<String> = vec!("components".into(), "schemas".into(), "WithdrawalViewForAccount".into());

    assert_eq!(
        parse_ref_path("#/components/schemas/WithdrawalViewForAccount").unwrap(),
        result
    );
}
